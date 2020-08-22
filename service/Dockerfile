FROM debian:buster-slim AS package-cache

WORKDIR /mire
COPY Cargo.lock Cargo.toml docker/package.sh /mire/
COPY ./crates /mire/crates/
RUN ./package.sh

# Build the code first
FROM rust:slim-buster AS builder

RUN rustc --version --verbose
RUN apt update -y && apt install -y pkg-config libssl-dev

WORKDIR /mire

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY --from=package-cache /mire /mire
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /mire/target -name "*mire*" | xargs rm -rf

# Finally, we do a full build of our code
RUN rm -rf crates
COPY crates /mire/crates/
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM debian:buster-slim

RUN apt update -y && apt install -y openssl

WORKDIR /mire

COPY --from=builder /mire/target/release/mire /mire/mire

CMD ["/mire/mire"]
