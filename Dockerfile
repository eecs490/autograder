FROM rust

WORKDIR /autograder
RUN mkdir results
COPY autograder/assignment assignment
COPY autograder/submission submission
COPY autograder/tarpaulin tarpaulin
RUN cargo build --manifest-path '/autograder/assignment/Cargo.toml'
COPY run_autograder run_autograder
