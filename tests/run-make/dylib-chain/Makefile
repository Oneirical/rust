# m1 to m4.rs are dynamically linked to one another. This test checks that the chain succeeds inititally, then fails after the removal of dynamic libraries.
# See https://github.com/rust-lang/rust/issues/10434

# ignore-cross-compile
include ../tools.mk

all:
	$(RUSTC) m1.rs -C prefer-dynamic
	$(RUSTC) m2.rs -C prefer-dynamic
	$(RUSTC) m3.rs -C prefer-dynamic
	$(RUSTC) m4.rs
	$(call RUN,m4)
	$(call REMOVE_DYLIBS,m1)
	$(call REMOVE_DYLIBS,m2)
	$(call REMOVE_DYLIBS,m3)
	$(call FAIL,m4)
