cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/roller_hero.wasm
start http://localhost:8000/
python -m http.server