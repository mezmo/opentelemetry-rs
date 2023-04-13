-include .config.mk

# Provide standard defaults - set overrides in .config.mk
SHELL=/bin/bash -o pipefail

# Source in repository specific environment variables
MAKEFILE_LIB=.makefiles
MAKEFILE_INCLUDES=$(wildcard $(MAKEFILE_LIB)/*.mk)
include $(MAKEFILE_INCLUDES)

# This helper function makes debugging much easier.
.PHONY:debug-%
debug-%:              ## Debug a variable by calling `make debug-VARIABLE`
	@echo $(*) = $($(*))

.PHONY:help
.SILENT:help
help:                 ## Show this help, includes list of all actions.
	@awk 'BEGIN {FS = ":.*?## "}; /^.+: .*?## / && !/awk/ {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' ${MAKEFILE_LIST}

.PHONY:build-release
build-release:
	cargo build --release

.PHONY:build-debug
build-debug:
	cargo build

.PHONY:build
build:: build-release

.PHONY:clean
clean:: ## Cleanup the local checkout
	-rm -rf *.backup tmp/ output/ target/
	cargo clean

.PHONY:clean-all
clean-all:: clean      ## Full cleanup of all artifacts
	-git clean -Xdf

.PHONY:lint
lint: lint-fmt lint-clippy lint-audit

.PHONY:lint-fmt
lint-fmt:
	cargo fmt --all

.PHONY:lint-clippy
lint-clippy:
	cargo clippy

.PHONY:lint-audit
lint-audit:
	cargo audit

.PHONY:publish
publish::

.PHONY:test
test::
	cargo test

.PHONY:version
version::
