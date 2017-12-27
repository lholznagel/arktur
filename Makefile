.PHONY: debug doc release

default:
	cargo build

commit:
	cargo test
	git add -A
	git commit

release:
	rm -rf target/release
	cargo build --release
	exec ./release.sh

doc:
	rm -rf target/doc
	cargo doc --all --frozen