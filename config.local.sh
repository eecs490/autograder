#! /usr/bin/env bash

# file structure
AUTOGRADER=autograder/Cargo.toml
ASSIGNMENT=assignment/Cargo.toml
SUBMISSION=submission/Cargo.toml
OUR_TEST_RESULTS=/tmp/our_test_results
THEIR_TEST_RESULTS=/tmp/their_test_results
OUTPUT=/tmp/results.json
LCOV=/tmp/lcov.info
SCORES=scores.yaml
LABELS=labels.yaml
OUR_SOLUTION=assignment/src/solution.rs
THEIR_SOLUTION=submission/src/solution.rs

