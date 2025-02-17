# Use the official Rust image as the base image
FROM rust:1.82 AS builder

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock (if it exists)
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm src/main.rs

# Now copy the actual source code
COPY src src/

# Build the application
RUN touch src/main.rs && cargo build --release

# Use distroless as minimal runtime
FROM gcr.io/distroless/cc-debian12

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/ip-info-site /app/ip-info-site

# Copy data and static directories
COPY data /data
COPY static /static

EXPOSE 8080

# Set the startup command
CMD ["/app/ip-info-site"]
