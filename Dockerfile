FROM gradescope/auto-builds:ubuntu-18.04

# FROM rustlang/rust:nightly
RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		autoconf \
		automake \
		bzip2 \
		dpkg-dev \
		file \
		g++ \
		gcc \
		imagemagick \
		libbz2-dev \
		libc6-dev \
		libcurl4-openssl-dev \
		libdb-dev \
		libevent-dev \
		libffi-dev \
		libgdbm-dev \
		libglib2.0-dev \
		libgmp-dev \
		libjpeg-dev \
		libkrb5-dev \
		liblzma-dev \
		libmagickcore-dev \
		libmagickwand-dev \
		libmaxminddb-dev \
		libncurses5-dev \
		libncursesw5-dev \
		libpng-dev \
		libpq-dev \
		libreadline-dev \
		libsqlite3-dev \
		libssl-dev \
		libtool \
		libwebp-dev \
		libxml2-dev \
		libxslt-dev \
		libyaml-dev \
		make \
		patch \
    zip \
		unzip \
		xz-utils \
		zlib1g-dev \
		\
# https://lists.debian.org/debian-devel-announce/2016/09/msg00000.html
		$( \
# if we use just "apt-cache show" here, it returns zero because "Can't select versions from package 'libmysqlclient-dev' as it is purely virtual", hence the pipe to grep
			if apt-cache show 'default-libmysqlclient-dev' 2>/dev/null | grep -q '^Version:'; then \
				echo 'default-libmysqlclient-dev'; \
			else \
				echo 'libmysqlclient-dev'; \
			fi \
		) \
	; \
	rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# end FROM rustlang/rust:nightly

RUN cargo install grcov

ENV CARGO_INCREMENTAL=0 \
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"

WORKDIR /autograder

# dummy build autograder for caching purposes
COPY autograder/Cargo.toml /autograder/autograder/Cargo.toml
COPY autograder/Cargo.lock /autograder/autograder/Cargo.lock
RUN mkdir /autograder/autograder/src && touch /autograder/autograder/src/lib.rs
RUN cargo build --manifest-path autograder/Cargo.toml
RUN rm -rf /autograder/autograder/src

COPY submission/ /autograder/submission

# dummy build autograder for caching purposes
COPY assignment/Cargo.toml /autograder/assignment/Cargo.toml
COPY assignment/Cargo.lock /autograder/assignment/Cargo.lock
RUN mkdir /autograder/assignment/src && touch /autograder/assignment/src/lib.rs
RUN cargo build --manifest-path assignment/Cargo.toml
RUN rm -rf /autograder/assignment/src

COPY autograder/src /autograder/autograder/src
COPY assignment/ /autograder/assignment

RUN cargo build --manifest-path autograder/Cargo.toml
RUN cargo build --manifest-path assignment/Cargo.toml
RUN cargo build --manifest-path submission/Cargo.toml
COPY scores.yaml .
COPY config.sh .
COPY run_autograder .
#CMD ./run_autograder
