#! /usr/bin/env bash
RUSTUP_HOME=/usr/local/rustup
CARGO_HOME=/usr/local/cargo
PATH=/usr/local/cargo/bin:$PATH
 
# file structure
AUTOGRADER=/autograder/autograder/Cargo.toml
ASSIGNMENT=/autograder/assignment/Cargo.toml
SUBMISSION=/autograder/submission/Cargo.toml
OUR_TEST_RESULTS=/autograder/our_test_results
THEIR_TEST_RESULTS=/autograder/their_test_results
OUTPUT=/autograder/results/results.json
LCOV=/autograder/submission/ccov/lcov.info
SCORES=/autograder/scores.yaml
OUR_SOLUTION=/autograder/assignment/src/solution.rs
THEIR_SOLUTION=/autograder/submission/src/solution.rs

# these flags are necessary for grcov
CARGO_INCREMENTAL=0
RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
