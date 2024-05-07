# This test verifies the functionality of ICE backtrace dumping in the case of a panic. It tries multiple alternatives, such as verifying that the dump exists on the disk.
# It also checks if no file is produced in the case of specifying that no ICE backtrace is requested.
# See https://github.com/rust-lang/rust/pull/108714

include ../tools.mk

# ignore-windows

export RUSTC := $(RUSTC_ORIGINAL)
export LD_LIBRARY_PATH := $(HOST_RPATH_DIR)
export TMPDIR := $(TMPDIR)

all:
	bash check.sh
