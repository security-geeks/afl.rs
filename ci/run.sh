#!/bin/sh -e

export LLVM_CONFIG=$(which llvm-config-3.8)

cd afl-plugin
cargo build --verbose
cd ..
cargo build --verbose
cargo build --example hello
cargo build --example deferred-init
cargo build --example integer-overflow
cargo build --example panic
cd afl-sys
cargo build
cd ..
cargo install --path .
timeout 10s cargo afl-fuzz -i . -o out target/debug/examples/hello
