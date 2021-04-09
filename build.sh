rm *.so *.pyd 
cargo build --release --lib
cp target/release/liblayout_engine.so layout_engine.so
cp target/release/liblayout_engine.so liblayout_engine.so

cp target/release/liblayout_engine.so layout_engine.pyd
cp target/release/liblayout_engine.so liblayout_engine.pyd