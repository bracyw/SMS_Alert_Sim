# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app

# accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/sms_backend .

CMD ["./sms_backend"]
