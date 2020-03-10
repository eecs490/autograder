#! /usr/bin/env bash

ccov=${1:-/tmp/ccov}
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo build
cargo test
mkdir $ccov
zip -0 $ccov/ccov.zip `find . \( -name "${PWD##*/}*.gc*" \) -print`
grcov $ccov/ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing -o $ccov/lcov.info
genhtml -o $ccov/ --show-details --highlight --ignore-errors source --legend $ccov/lcov.info
