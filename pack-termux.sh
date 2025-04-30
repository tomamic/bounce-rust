clear
pkg i rust rust-std-wasm32-wasi rust-std-wasm32-unknown-unknown 
cargo install wasm-pack
cargo update --package wasm-bindgen
~/.cargo/bin/wasm-pack build --target web
