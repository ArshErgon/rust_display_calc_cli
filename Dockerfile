# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/myapp

# Copy the Cargo.toml file to the container
COPY Cargo.toml .

# Copy the src directory to the container
COPY src src

# Build the application
RUN cargo build --release

# Expose the application's port
EXPOSE 8000

# Run the application
CMD ["cargo", "run", "--release"]