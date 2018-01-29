.PHONY: test-all commit release doc peer hole_puncher

default:
	cargo build

bench:
	rustup run nightly cargo bench

test:
	rustup run stable cargo test
	rustup run nightly cargo test

# check that everything works before commiting
commit:
	make bench
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