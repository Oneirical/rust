// In this test, the function `bar` has #[inline(never)] and the function `foo`
// does not. This test outputs LLVM optimization remarks twice - first for all
// functions (including `bar`, and the `inline` mention), and then for only `foo`
// (should not have the `inline` mention).
// See https://github.com/rust-lang/rust/pull/113040

use run_make_support::{
    find_files_by_prefix_and_extension, invalid_utf8_contains, invalid_utf8_not_contains, rustc,
};

fn main() {
    rustc()
        .opt()
        .input("foo.rs")
        .crate_type("lib")
        .arg("-Cremark=all")
        .arg("-Zremark-dir=profiles_all")
        .run();
    for file in find_files_by_prefix_and_extension("profiles_all", "foo", "yaml") {
        if !file.to_str().unwrap().contains("codegen") {
            invalid_utf8_contains(file, "inline")
        };
    }
    rustc()
        .opt()
        .input("foo.rs")
        .crate_type("lib")
        .arg("-Cremark=foo")
        .arg("-Zremark-dir=profiles_foo")
        .run();
    for file in find_files_by_prefix_and_extension("profiles_foo", "foo", "yaml") {
        invalid_utf8_not_contains(file, "inline")
    }
}
