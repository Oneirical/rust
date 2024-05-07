# At any opt-level higher than 0, the flag debug_assertions should be turned off. This test checks that this rule remains true.
# See https://github.com/rust-lang/rust/pull/22980

# ignore-cross-compile
# needs-unwind
include ../tools.mk

all:
	$(RUSTC) debug.rs -C debug-assertions=no
	$(call RUN,debug) good
	$(RUSTC) debug.rs -C opt-level=0
	$(call RUN,debug) bad
	$(RUSTC) debug.rs -C opt-level=1
	$(call RUN,debug) good
	$(RUSTC) debug.rs -C opt-level=2
	$(call RUN,debug) good
	$(RUSTC) debug.rs -C opt-level=3
	$(call RUN,debug) good
	$(RUSTC) debug.rs -C opt-level=s
	$(call RUN,debug) good
	$(RUSTC) debug.rs -C opt-level=z
	$(call RUN,debug) good
	$(RUSTC) debug.rs -O
	$(call RUN,debug) good
	$(RUSTC) debug.rs
	$(call RUN,debug) bad
	$(RUSTC) debug.rs -C debug-assertions=yes -O
	$(call RUN,debug) bad
	$(RUSTC) debug.rs -C debug-assertions=yes -C opt-level=1
	$(call RUN,debug) bad
