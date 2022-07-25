# Builder stage.
# Latest Rust stable release.
FROM rust:1.62.1-slim AS builder

# Switch the working directory to `/app`.
WORKDIR /app
# Update and then install LLD.
RUN apt update && apt install lld clang -y
# Copy all files from the working environment to the Docker container.
COPY . .
# Set the `SQLX_OFFLINE` environment variable to true,
# to perform an offline build using a metadata file.
ENV SQLX_OFFLINE true
# Build the binary in the release profile to make it faster.
RUN cargo build --release

# Runtime stage.
# Latest debian release.
FROM debian:bullseye-slim AS runtime

# Switch the working directory to `/app`.
WORKDIR /app
# Update.
RUN apt-get update -y \
    # Install OpenSSL.
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up.
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the `builder` environment to the `runtime` environment.
COPY --from=builder /app/target/release/tessera tessera
# Copy the configuration files from the `builder` environment to the `runtime` environment.
COPY configuration configuration
# Set the `APP_ENVIRONMENT` environment variable to `production`.
ENV APP_ENVIRONMENT production

# When asked to run, launch the binary.
ENTRYPOINT ["./tessera"]
