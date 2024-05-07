# The linked PR changed crate-name, file-names and such flags to be processed before the entire crate is parsed. This test ensures that this functionality is preserved.
# See https://github.com/rust-lang/rust/pull/11077

include ../tools.mk

all:
	[ `$(RUSTC) --print crate-name crate.rs` = "foo" ]
	[ `$(RUSTC) --print file-names crate.rs` = "$(call BIN,foo)" ]
	[ `$(RUSTC) --print file-names --crate-type=lib \
		--test crate.rs` = "$(call BIN,foo)" ]
	[ `$(RUSTC) --print file-names --test lib.rs` = "$(call BIN,mylib)" ]
	$(RUSTC) --print file-names lib.rs
	$(RUSTC) --print file-names rlib.rs
