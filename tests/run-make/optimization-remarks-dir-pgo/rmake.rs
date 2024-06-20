// This test checks the -Zremark-dir flag, which writes LLVM
// optimization remarks to the YAML format. When using PGO (Profile
// Guided Optimization), the Hotness attribute should be included in
// the output remark files.
// See https://github.com/rust-lang/rust/pull/114439

//@ needs-profiler-support
//@ ignore-cross-compile

use run_make_support::{
    find_files_by_prefix_and_extension, invalid_utf8_contains, llvm_profdata, run, rustc,
};

fn main() {
    rustc().profile_generate("profdata").opt().input("foo.rs").output("foo").run();
    run("foo");
    llvm_profdata()
        .merge()
        .output("merged.profdata")
        .input(find_files_by_prefix_and_extension("profdata", "default", "profraw").get(0).unwrap())
        .run();
    rustc()
        .profile_use("merged.profdata")
        .opt()
        .input("foo.rs")
        .arg("-Cremark=all")
        .arg("-Zremark-dir=profiles")
        .run();
    // Check that PGO hotness is included in the remark files
    for file in find_files_by_prefix_and_extension("profiles", "foo", "yaml") {
        if !file.to_str().unwrap().contains("codegen") {
            invalid_utf8_contains(file, "Hotness")
        };
    }
}
