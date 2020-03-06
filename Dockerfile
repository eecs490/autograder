FROM rustlang/rust:nightly
 
RUN cargo install grcov
COPY autograder /project
ENV CARGO_INCREMENTAL=0
ENV RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
RUN cargo build --manifest-path /project/assignment/Cargo.toml
RUN cargo test --manifest-path /project/assignment/Cargo.toml
RUN apt-get update && apt-get install -y zip
RUN zip -0 /ccov.zip `find /project \( -name "*.gc*" \) -print`
CMD grcov "/ccov.zip" -s /project -t lcov --llvm --branch --ignore-not-existing 
