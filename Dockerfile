FROM rust:1.76 as builder

WORKDIR /usr/src

RUN rustup toolchain install nightly-2024-06-07 \
	&& rustup default nightly-2024-06-07


RUN apt-get update \
	&& apt-get install -y ca-certificates tzdata libcurl4 libpq-dev ca-certificates netcat \
	&& rm -rf /var/lib/apt/lists/*

# 1. Create a new empty shell project
RUN USER=root cargo new --bin anita

# 2. Copy our manifests
WORKDIR /usr/src/anita
COPY Cargo.toml Cargo.lock ./

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm -rf ./src

# 4. Now that the dependency is built, copy your source code
COPY . .

# 5. Build for release.
RUN rm ./target/release/deps/anita*
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
	&& apt-get install -y ca-certificates tzdata libcurl4 libpq-dev ca-certificates netcat \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/anita/target/release/anita /usr/local/bin/anita

CMD ["anita"]