.PHONY: test-all commit release doc peer hole_puncher

default:
	cargo build

bench:
	rustup run nightly cargo bench

test:
	rustup run stable cargo test
	rustup run nightly cargo test

docker_clean:
	docker ps -a -q -f label=peer | xargs --no-run-if-empty docker rm -f 

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

peer_docker_run:
	make peer_docker_build
	docker run -it --net="host" blockchain_peer:latest

peer_docker_run_multi:
	make peer_docker_build
	./docker/start.sh

peer_docker_build:
	clear
	cd peer; cargo build
	cp target/debug/blockchain_peer docker/blockchain_peer
	cd docker; docker build -t blockchain_peer .

hole_puncher:
	clear
	cd hole_puncher; cargo run