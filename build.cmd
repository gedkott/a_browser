del *.dll *.pyd
cargo build --release --lib
copy target\release\layout_engine.dll layout_engine.pyd