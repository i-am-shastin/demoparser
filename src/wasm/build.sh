OUT_DIR=www/pkg
THREADS_DIR=$OUT_DIR/threads

# Compile multithreaded version using nightly Rust
RUSTFLAGS='-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Clink-arg=--max-memory=4294967296' wasm-pack build --out-dir $THREADS_DIR --target web -- --features threads -Z build-std=panic_abort,std
# Copy wasm-bindgen-rayon snippet file (SEE: https://github.com/rustwasm/wasm-bindgen/issues/3330)
cp -R ./snippets/* $(ls -d -1 $THREADS_DIR/snippets/* | sed -n '1p')/src
# Compile singlethreaded version using stable Rust
RUSTUP_TOOLCHAIN=stable wasm-pack build --out-dir $OUT_DIR --target web