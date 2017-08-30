rm -rf target/release
cargo build --release
mv target/release/libnyan.dylib target/release/nyan.dylib 