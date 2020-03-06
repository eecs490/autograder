FROM rustlang/rust:nightly
 
RUN apt-get update && apt-get install -y zip
RUN cargo install grcov
ENV CARGO_INCREMENTAL=0
ENV RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
WORKDIR /autograder
COPY autograder .
RUN cargo build --manifest-path /autograder/assignment/Cargo.toml
COPY run_autograder .
#CMD ./run_autograder
