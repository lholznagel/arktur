# just build the project
default:
	cargo build --features dev

# run the benchmarks
bench:
	rustup run nightly cargo bench

# run test with stable and nightly
test:
	rustup run stable cargo test
	rustup run nightly cargo test

# check that everything works before commiting
commit: bench test
	git add -A
	git commit

# build a release for all binaries
release:
	rm -rf target/release
	cargo build --release
	exec ./scripts/release.sh
