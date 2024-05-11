# This test checks that Rustc can still compile correctly when the unstable no_global_oom_handling feature is turned on, which disables global error handling.
# Unlike alloc-no-oom-handling, this test does not only build alloc.
# See https://github.com/rust-lang/rust/pull/84266 and https://github.com/rust-lang/rust/pull/110652

include ../tools.mk

FAKEROOT=$(TMPDIR)/fakeroot

all:
	$(RUSTC) --edition=2021 -Dwarnings --crate-type=rlib ../../../library/core/src/lib.rs --sysroot=$(FAKEROOT) --cfg no_global_oom_handling
