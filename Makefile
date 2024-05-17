.PHONY: run run-release build build-release dev-upgrade-rs-edition

run:
	cargo run

run-release:
	cargo run --release

build:
	cargo build

build-release:
	cargo build --release

dev-upgrade-rs-edition:
	cargo fix --edition
