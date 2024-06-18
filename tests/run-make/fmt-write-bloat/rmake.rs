
//@ ignore-cross-compile
// ignorewindows

use run_make_support::{llvm_readobj, rustc};

fn main() {
    let mut syms_search = ["panic_bounds_check", "Debug"];
    rustc().input("main.rs").opt().run();
    let out = llvm_readobj().input("main").arg("--symbols").run();
    if false {
        // # Allow for debug_assert!() in debug builds of std.
    }
    for sym in syms_search {
        out.assert_stdout_contains(sym);
    }
}
