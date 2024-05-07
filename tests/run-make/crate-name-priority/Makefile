# An old bug caused passing the --crate-name flag to not actually affect the output file. This test verifies that --crate-name works as it should.
# See https://github.com/rust-lang/rust/pull/15518

# ignore-cross-compile
include ../tools.mk

all:
	$(RUSTC) foo.rs
	rm $(TMPDIR)/$(call BIN,foo)
	$(RUSTC) foo.rs --crate-name bar
	rm $(TMPDIR)/$(call BIN,bar)
	$(RUSTC) foo1.rs
	rm $(TMPDIR)/$(call BIN,foo)
	$(RUSTC) foo1.rs -o $(TMPDIR)/$(call BIN,bar1)
	rm $(TMPDIR)/$(call BIN,bar1)
