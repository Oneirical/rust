include ../tools.mk

# ignore-cross-compile

# When dep-graph flags are passed, this test checks that the requested dep-graph files are outputted.
# See https://github.com/rust-lang/rust/pull/89343

all:
	RUST_DEP_GRAPH=$(TMPDIR)/dep-graph $(RUSTC) \
        -Cincremental=$(TMPDIR)/incr \
        -Zquery-dep-graph -Zdump-dep-graph foo.rs
	test -f $(TMPDIR)/dep-graph.txt
	test -f $(TMPDIR)/dep-graph.dot
