# Use the official Rust image as the base image for building the app
FROM rust:1.82-alpine as builder

# Install dependencies needed for building (openssl and musl)
RUN apk add --no-cache openssl-dev musl-dev

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Create a new empty directory for the actual code
RUN mkdir src

# Copy the actual source code into the container
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create a new smaller Alpine image to run the app
FROM alpine:latest

# Install the required runtime dependencies (openssl)
RUN apk add --no-cache openssl

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/rust-file-transfer-api .

# Set the environment variable (optional: use docker-compose to configure it dynamically)
ENV FILE_DIR=/path/to/your/directory

# Expose the port the app will be running on
EXPOSE 3000

# Run the app
CMD ["./rust-file-transfer-api"]
