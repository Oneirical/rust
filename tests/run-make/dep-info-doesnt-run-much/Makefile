# This test checks that rustc compiles when the --emit dep-info flag is passed.
# See https://github.com/rust-lang/rust/pull/40336

include ../tools.mk

all:
	$(RUSTC) foo.rs --emit dep-info
