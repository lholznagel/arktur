.PHONY: test-all commit release doc peer hole_puncher

default:
	cargo build

test:
	rustup run stable cargo test
	rustup run nightly cargo test

commit:
	make test
	git add -A
	git commit

release:
	rm -rf target/release
	cargo build --release
	exec ./release.sh

doc:
	rm -rf target/doc
	cargo doc --all --frozen

peer:
	clear
	cd peer; cargo run -- --name $(NAME)

hole_puncher:
	clear
	cd hole_puncher; cargo run