cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/rgraphview.wasm ./site
python -m http.server -d ./site
