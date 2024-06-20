
// # needs-force-clang-based-tests

// # FIXME(#126180): This test doesn't actually run anywhere, because the only
// # CI job that sets RUSTBUILD_FORCE_CLANG_BASED_TESTS runs very few tests.

// # This test makes sure the embed bitcode in elf created with
// # lto-embed-bitcode=optimized is valid llvm BC module.

use run_make_support::{rustc, llvm_objcopy, llvm_dis, env_var};

fn main() {
    rustc().input("test.rs").link_arg("-fuse-ld=lld").arg("-Clinker-plugin-lto").linker(&env_var("CLANG")).link_arg("-Wl,--plugin-opt=-lto-embed-bitcode=optimized").arg("-Zemit-thin-lto=no").run();
    llvm_objcopy().dump_section(".llvmbc", "test.bc").input("test").run();
    llvm_dis().input("test.bc").run();
}
