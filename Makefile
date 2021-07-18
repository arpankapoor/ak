run: build
	rlwrap ./target/debug/ak
build:
	cargo +nightly build
fmt:
	cargo +nightly fmt
