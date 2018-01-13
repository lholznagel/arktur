.PHONY: debug doc release peer

default:
	cargo build

test-all:
	rustup run stable cargo test
	rustup run nightly cargo test

commit:
	make test-all
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
	cd peer; cargo run -- 0.0.0.0 $(NAME)

connection:
	clear
	cd connection_manager; cargo run