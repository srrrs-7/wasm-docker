.PHONY: run build test check fmt clippy benchmark docker-build
run:
	wasmedge ./target/wasm32-wasi/release/wasm.wasm
build:
	cargo build --target wasm32-wasi --release
test:
	cargo test
check:
	cargo check
fmt:
	cargo fmt
clippy:
	cargo clippy
benchmark:
	cargo bench
tree:
	cargo tree
lib:
	cargo new --lib ${arg}
bin:
	cargo new --bin ${arg}
pkg:
	cargo new ${arg}

docker-build:
	docker buildx build --platform wasi/wasm32 -t app.wasm ../.
req:
	curl -d 'Bob Morane' -X POST http://127.0.0.1:8080/hello