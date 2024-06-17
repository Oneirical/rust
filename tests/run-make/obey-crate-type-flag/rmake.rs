// test.rs should produce both an rlib and a dylib
// by default. When the crate_type flag is passed and
// forced to dylib, no rlibs should be produced.
// See https://github.com/rust-lang/rust/issues/11573

//@ ignore-cross-compile

use run_make_support::{
    cwd, dynamic_lib_name, find_files, fs_wrapper, has_extension, rust_lib_name, rustc,
};

fn main() {
    rustc().input("test.rs").run();
    fs_wrapper::remove_file(dynamic_lib_name("test"));
    fs_wrapper::remove_file(rust_lib_name("test"));
    rustc().crate_type("dylib").input("test.rs").run();
    assert_eq!(find_files(cwd(), |path| { has_extension(path, "rlib") }).len(), 0);
}
