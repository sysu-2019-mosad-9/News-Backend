###############
### BUILD IMAGE
###############
FROM dasinlsb/rust:nightly-2019-11-13 as build

# Install dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app
COPY Cargo.* ./
COPY rust-toolchain ./

RUN cargo build --release
RUN find . -not -path "./target*" -delete

# Build server
COPY . .
RUN cargo build --release

#################
### RUNTIME IMAGE
#################
FROM debian:buster-slim

COPY --from=build /app/target/release/fake /

EXPOSE 8000
CMD ["/fake"]

