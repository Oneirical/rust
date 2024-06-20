use run_make_support::{cwd, is_darwin, llvm_dwarfdump, rust_lib_name, rustc};

fn main() {
    compile("./src/quux.rs", None);
    llvm_common_check();
    compile("./src/quux.rs", Some("object"));
    llvm_common_check();
    //FIXME(Oneirical): This is where the absolute outside dir check would happen
    compile("src/quux.rs", None);
    llvm_common_check();
    compile("src/quux.rs", Some("object"));
    llvm_common_check();
    compile("src/quux.rs", Some("diagnostics"));
    llvm_common_check();

    rustc()
        .input("srd/quux.rs")
        .output(rust_lib_name("output_of_test"))
        .arg("-Cdebuginfo=2")
        .remap_path_prefix(cwd().join("src"), cwd().join("REMAPPED"));
    let out = llvm_dwarfdump().input("output_of_test").run();
    out.assert_stdout_contains("REMAPPED/quux.rs");
    out.assert_stdout_not_contains("REMAPPED/REMAPPED");
    out.assert_stdout_not_contains("src");

    rustc()
        .input("srd/quux.rs")
        .output(rust_lib_name("output_of_test"))
        .arg("-Cdebuginfo=2")
        .remap_path_prefix(cwd().parent().unwrap(), &cwd().join("REMAPPED"));
    let out = llvm_dwarfdump().input("output_of_test").run();
    out.assert_stdout_contains("REMAPPED/remap-path-prefix-dwarf/src/quux.rs");
    out.assert_stdout_not_contains("REMAPPED/REMAPPED");
    out.assert_stdout_not_contains("run-make");
}

fn compile(path: &str, scope: Option<&str>) {
    let mut compiler = rustc();
    compiler
        .input(path)
        .output(rust_lib_name("output_of_test"))
        .arg("-Cdebuginfo=2")
        .remap_path_prefix(cwd(), cwd().join("REMAPPED"));
    if let Some(scope) = scope {
        compiler.arg(format!("-Zremap-path-scope={scope}"));
        if is_darwin() {
            compiler.arg("-Csplit-debuginfo=off");
        }
    }
    compiler.run();
}

fn llvm_common_check() {
    let out = llvm_dwarfdump().input(rust_lib_name("output_of_test")).run();
    out.assert_stdout_contains("REMAPPED/src/quux.rs");
    out.assert_stdout_not_contains("REMAPPED/REMAPPED");
}
