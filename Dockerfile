# Use the official Rust image as a base
FROM rust:latest AS builder

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the backend code
COPY frc-service .

# Build the backend
RUN cargo build --release

# Use a minimal image for the final stage
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/frc-service /usr/local/bin/frc-service

# Set the environment variables
ENV OPENAI_API_KEY=${OPENAI_API_KEY}

# Expose the port the app runs on
EXPOSE 9086

# Run the binary
CMD ["frc-service"]