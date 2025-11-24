# Stage 1: Build Tailwind CSS
FROM debian:12-slim AS frontend-builder
WORKDIR /app

# Install curl for downloading Tailwind CLI
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Download standalone Tailwind CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64

# Copy Tailwind config and CSS files
COPY tailwind.css .
COPY src/views/ ./src/views/

# Generate CSS with standalone Tailwind CLI
RUN ./tailwindcss-linux-x64 -i tailwind.css -o ./static/styles.css --minify

# ---

# Stage 2: Build Rust application
FROM rust:latest AS rust-builder

WORKDIR /app

# Install build essentials (like a C linker) if your Rust app has C dependencies.
# For basic Axum apps, this might not be strictly necessary with the slim-buster image.
# RUN apt-get update && apt-get install -y build-essential

# Create a new empty project to cache dependencies
RUN USER=root cargo new --bin app_binary
WORKDIR /app/app_binary

# Copy over your manifests
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build dependencies only to leverage Docker cache
# This will only re-run if Cargo.toml or Cargo.lock changes
# RUN cargo build --release
# RUN rm src/*.rs target/release/deps/app_binary*

# Copy your actual source code
COPY src ./src
# If you have other files/dirs needed for compilation (e.g. a `config` directory)
# COPY config ./config

# Build the application
RUN cargo build --release
RUN ls -la ./target/release/
# ---

# Stage 3: Create the final lightweight image
FROM debian:bookworm-slim AS final

WORKDIR /app

# Create a non-root user for security
# RUN groupadd -r appuser && useradd -r -g appuser -s /sbin/nologin -c "Docker image user" appuser

# Copy static assets (including Tailwind CSS output) from the frontend-builder stage
# Adjust the source path if your Tailwind output is different
COPY --from=frontend-builder /app/frontend/static ./static

# Copy HTML templates
# COPY templates ./templates

# Copy other static assets if you have them (e.g., images, JavaScript files not handled by node)
# COPY public ./public

# Copy the compiled Rust binary from the rust-builder stage
COPY --from=rust-builder /app/app_binary/target/release/jake-personal-site ./jake-personal-site
COPY static ./static
# Make sure to replace 'your_rust_app_name' with the actual name of your binary (defined in Cargo.toml)

# Ensure the binary is executable and owned by the appuser
RUN chmod +x ./jake-personal-site
# Add other directories here if needed: chown -R appuser:appuser ./public

# Switch to the non-root user
# USER appuser

# Expose the port your Axum application listens on
EXPOSE 3000

# Set the entrypoint for the container
CMD ["./jake-personal-site"]
