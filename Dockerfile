# Stage 1: Build Tailwind CSS
FROM node:18-slim AS frontend-builder

WORKDIR /app/frontend

# Copy package.json and package-lock.json (or yarn.lock)
COPY package.json ./
# If you're using yarn, copy yarn.lock instead and use yarn install
# COPY yarn.lock ./
COPY package-lock.json ./

# Install dependencies for Tailwind CSS
RUN npm install

# Copy the rest of your frontend assets and Tailwind config
# Ensure your tailwind.config.js and your main input CSS file (e.g., styles/tailwind.css) are present
# COPY tailwind.config.js ./
COPY tailwind.css ./
 # If your Rust source files influence Tailwind's content scanning (e.g. for class names)
COPY src ./src

# Run the Tailwind CSS build command
# Adjust the input and output paths as per your project structure
# The output CSS will be copied to the final stage
# RUN npm run build:css # Assuming you have a script like: "tailwindcss -i ./styles/tailwind.css -o ./static/css/main.css --minify" in your package.json
                     # Or, you can run the npx command directly:
                     # RUN npx tailwindcss -i ./styles/tailwind.css -o ./static/css/main.css --minify
RUN npx @tailwindcss/cli -i tailwind.css -o ./static/styles.css --minify

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
