###############
### BUILD IMAGE
###############

### Ensure the application is runnable in prduction env
#FROM dasinlsb/rust:nightly-2019-11-13 as build
### Use the latest nightly in development env
FROM rustlang/rust:nightly as build

# Install dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app
COPY Cargo.* ./
COPY rust-toolchain ./

ENV ROCKET_ENV=production
RUN cargo build --release
RUN find . -not -path "./target*" -delete

# Build server
COPY . .
RUN cargo build --release

#################
### RUNTIME IMAGE
#################
FROM debian:buster-slim

COPY --from=build /app/target/release/news-backend /

EXPOSE 8000
CMD ["/news-backend"]

