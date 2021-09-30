run: build
	rlwrap ./target/debug/ak
build:
	cargo +nightly build
release:
	cargo +nightly build --release
fmt:
	cargo +nightly fmt
clean:
	cargo +nightly clean
