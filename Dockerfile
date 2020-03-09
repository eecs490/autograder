FROM ethanabrooks/rust-autograder-base

COPY autograder .
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN cargo build --manifest-path /autograder/assignment/Cargo.toml
COPY scores.yaml .
COPY run_autograder.base .
COPY run_autograder .
#CMD ./run_autograder
