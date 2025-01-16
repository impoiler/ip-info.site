# Use the official Rust image as the base image
FROM rust:1.82 as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Use distroless with compatible glibc
FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/app/target/release/ip_lookup_service /

# Run the binary
CMD ["/ip_lookup_service"]


