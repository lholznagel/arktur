.PHONY: build build-relase

build:
	cargo build

build-release:
	# use musk
	rm -rf target/release
	cargo build --release
	du -h target/release/blockchain_* --exclude=*.d
	strip target/release/blockchain_connection_manager
	strip target/release/blockchain_peer
	du -h target/release/blockchain_* --exclude=*.d

test:
	rm -rf target/release
	cargo build --release
	./test.sh