#!/bin/bash
set -e

export RUSTC_BOOTSTRAP=1
export RUSTFLAGS="-Zinstrument-coverage"
export LLVM_PROFILE_FILE="coverage/tests-%p-%m.profraw"

rm -rf coverage/*
cargo clean
cargo build
cargo test
~/.cargo/bin/grcov . --binary-path ./target/debug/ -s . -t lcov --ignore-not-existing --keep-only "src/*.rs" --ignore "src/main.rs" -o ./coverage/coverage.lcov --excl-line "^(#\[derive\(Debug\))" -p src
genhtml coverage/coverage.lcov -o coverage --show-details --highlight --ignore-errors source --legend --no-function-coverage --no-branch-coverage