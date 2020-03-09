FROM ethanabrooks/rust-autograder-base

COPY autograder/ /autograder/autograder
COPY assignment/ /autograder/assignment
COPY submission/ /autograder/submission
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH="/usr/local/cargo/bin:$PATH" \
    CARGO_INCREMENTAL=0 \
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
CMD /bin/bash
RUN cargo build --manifest-path autograder/Cargo.toml
RUN cargo build --manifest-path assignment/Cargo.toml
RUN cargo build --manifest-path submission/Cargo.toml
COPY scores.yaml .
COPY config.sh .
COPY run_autograder .
CMD ./run_autograder
