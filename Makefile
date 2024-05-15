.PHONY: build
build:
	cargo build --release

.PHONY: run
run:
	cargo run

.PHONY: install
install:
	cargo install --path .

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features

.PHONY: clean
clean:
	cargo clean
