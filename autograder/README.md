# sample-rust-submission
This repository implements an autograder for Rust, intended to interface with the Gradescope autograding infrastructure. 
At a high level, the autograder
1. runs `cargo test` on the student's submission.
2. scrapes the output and parses into `rust` `TestResult` structs.
3. creates a `TestReport` struct and writes the associated json object to the location where Gradescope looks for it.

For details on how Gradescope works with this, read https://github.com/ethanabrooks/autograder/blob/master/README.md.

# TODO
1. Run tests designed by the _instructor_
