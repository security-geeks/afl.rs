#!/bin/bash -e

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

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then sudo sh -c 'echo core > /proc/sys/kernel/core_pattern'; fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then SL=/System/Library; PL=com.apple.ReportCrash; fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then launchctl unload -w ${SL}/LaunchAgents/${PL}.plist; fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then sudo launchctl unload -w ${SL}/LaunchDaemons/${PL}.Root.plist; fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then gtimeout 10s cargo afl-fuzz -i . -o out target/debug/examples/hello; fi
if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then timeout 10s cargo afl-fuzz -i . -o out target/debug/examples/hello; fi
