#! /usr/bin/env bash
root=$HOME/autograder/autograder
assignment=$root/assignment/Cargo.toml
results=${6:-/autograder/results/results.json}

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo build --manifest-path $assignment
cd $root/submission
rm -rf ccov
mkdir ccov
cargo test 
zip -0 ccov/ccov.zip $(find . \( -name "submission*.gc*" \) -print)
grcov ccov/ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing  -o ccov/lcov.info
#cargo run --manifest-path $assignment $assignment /tmp/results.json /ccov/lcov.info
genhtml -o ccov --show-details --highlight --ignore-errors source --legend ccov/lcov.info
#cargo run --manifest-path $assignment $assignment $results ccov/lcov.info

open ccov/src/lib.rs.gcov.html
