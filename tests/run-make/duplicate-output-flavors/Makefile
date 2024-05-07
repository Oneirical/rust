# When a rust file contained #[crate_type = "rlib"] and rlib was also specified inside the rustc flags, this duplication caused unexpected errors.
# This test checks that this bug is not brought back.
# See https://github.com/rust-lang/rust/issues/11346

include ../tools.mk

all:
	$(RUSTC) --crate-type=rlib foo.rs
	$(RUSTC) --crate-type=rlib,rlib foo.rs
