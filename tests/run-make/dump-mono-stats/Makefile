# When compiling with the flag -Z dump-mono-stats (which generates a file with a list of the monomorphized items in the current crate), a json output should be produced.
# This test checks that the contents of this output are as expected.
# See https://github.com/rust-lang/rust/pull/106274

include ../tools.mk

all:
	$(RUSTC) --crate-type lib foo.rs -Z dump-mono-stats=$(TMPDIR) -Zdump-mono-stats-format=json
	cat $(TMPDIR)/foo.mono_items.json | $(CGREP) '"name":"bar"'
