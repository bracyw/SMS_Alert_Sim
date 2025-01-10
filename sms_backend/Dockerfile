# Build stage
FROM rust:latest AS builder

WORKDIR /usr/src/sms_backend

# Copy source code and the .env file
COPY . .
RUN cargo install --path . --locked --profile release

# Production stage
FROM debian:stable-slim

WORKDIR /usr/local/bin

# Install required dependencies
RUN apt-get update && \
    apt-get install -y openssl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and the .env file
COPY --from=builder /usr/local/cargo/bin/sms_backend /usr/local/bin/sms_backend
COPY .env /usr/local/bin/.env

# Run the application
CMD ["sh", "-c", "sleep 5 && /usr/local/bin/sms_backend"]
