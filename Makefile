.PHONY: serve
serve:
	trunk serve

.PHONY: build
build:
	trunk build --release

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

.PHONY: check
check:
	cargo check

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: all
all: check test fmt clippy build