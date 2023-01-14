ROOT_DIR=docs/

server-run:
	basic-http-server $(ROOT_DIR)

# ローカル環境で確認を行う場合
build-local:
	cargo build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/invader-macroquad.wasm $(ROOT_DIR)

build-web:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/invader-macroquad.wasm $(ROOT_DIR)