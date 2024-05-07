# This test emits a mir dump from compilation of the main.rs file, and then tests that dump against a sample already provided in the test directory. 
# This is to check that --emit=mir emits both mir_for_ctfe and optimized_mir for const fn.
# See https://github.com/rust-lang/rust/pull/78407

# needs-unwind -Cpanic=abort gives different MIR output
include ../tools.mk

all:
	$(RUSTC) main.rs --emit=mir -o "$(TMPDIR)"/dump.mir
	$(RUSTC_TEST_OP) "$(TMPDIR)"/dump.mir dump.mir
