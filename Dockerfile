# Use the official Rust image as the build environment
FROM rust:1.89 as builder

# Set the working directory
WORKDIR /usr/src/expenses_tracker

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy database migrations
COPY migrations ./migrations

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image to run the application
FROM debian:buster-slim

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/expenses_tracker/target/release/expenses_tracker /usr/local/bin/expenses_tracker

# Copy statics
COPY statics /usr/local/bin/expenses_tracker/statics

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["expenses_tracker"]
