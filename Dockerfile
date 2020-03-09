FROM ethanabrooks/rust-autograder-base

COPY autograder .
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH="/usr/local/cargo/bin:$PATH" \
    CARGO_INCREMENTAL=0 \
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
RUN cargo build --manifest-path assignment/Cargo.toml
COPY scores.yaml .
COPY file_structure .
COPY run_autograder .
#CMD ./run_autograder
