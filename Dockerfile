FROM ethanabrooks/rust-autograder-base

COPY autograder .
RUN cargo build --manifest-path /autograder/assignment/Cargo.toml
COPY run_autograder.base .
COPY run_autograder .
#CMD ./run_autograder
