#! /usr/bin/env bash
root=$HOME/autograder/autograder
assignment=$root/assignment/Cargo.toml

rm -rf ccov
mkdir ccov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo build --manifest-path $assignment
cargo test --manifest-path $root/submission/Cargo.toml
zip -0 ccov/ccov.zip $(find $root/submission \( -name "submission*.gc*" \) -print)
grcov ccov/ccov.zip -s $root/submission -t lcov --llvm --branch --ignore-not-existing  -o $root/submission/ccov/lcov.info
cargo run --manifest-path $assignment $assignment /tmp/results.json $root/submission/ccov/lcov.info
genhtml -o $root/submission/ccov --show-details --highlight --ignore-errors source --legend $root/submission/ccov/lcov.info
open ccov/src/lib.rs.gcov.html
