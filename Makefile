SHELL := /bin/bash
CARGO_TOOLCHAIN ?= nightly
CARGO_ARGS ?=

export DEBUG ?= 0

.ONESHELL:
default: build

release: DEBUG=0
release: clean build

debug: DEBUG=1
debug: build

dev: DEBUG=1
dev: run

clean:
	@$(call cargo,clean)	

build: 
	@$(call cargo,build)

run: 
	@$(call cargo,run)

check: 
	@$(call cargo,check)

publish:
	@set -e
	cargo +$(CARGO_TOOLCHAIN) fmt
	cargo +$(CARGO_TOOLCHAIN) clippy
	cargo +$(CARGO_TOOLCHAIN) publish

.PHONY: default build debug clean check dev run publish

define cargo =
set -e;
cargo_args="$(CARGO_ARGS)";
if [ "$(DEBUG)" != "1" ]; then cargo_args="--release $(CARGO_ARGS)"; fi;
cargo +$(CARGO_TOOLCHAIN) $(1) $${cargo_args}
endef