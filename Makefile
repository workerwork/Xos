# Makefile for top level of Xos

.PHONY: help clean test

# print top level help
help:
	cargo xtask

# clean targets
clean:
	cargo clean

test:
	cargo git-proxy