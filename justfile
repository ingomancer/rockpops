run:
    cargo run

wasm:
    cargo build --release --target wasm32-unknown-unknown

zip: wasm
    zip -j target/rockpops.zip src/index.html target/wasm32-unknown-unknown/release/rockpops.wasm