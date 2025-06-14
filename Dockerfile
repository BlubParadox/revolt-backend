# Build Stage
FROM --platform="${BUILDPLATFORM}" rust:1.86.0-slim-bookworm
USER 0:0
WORKDIR /home/rust/src

ARG TARGETARCH

# Install build requirements
RUN apt-get update && \
    apt-get install -y \
    make \
    pkg-config \
    libssl-dev
COPY scripts/build-image-layer.sh /tmp/
RUN sh /tmp/build-image-layer.sh tools

# Build all dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates/bonfire/Cargo.toml ./crates/bonfire/
COPY crates/delta/Cargo.toml ./crates/delta/
COPY crates/core/config/Cargo.toml ./crates/core/config/
COPY crates/core/database/Cargo.toml ./crates/core/database/
COPY crates/core/files/Cargo.toml ./crates/core/files/
COPY crates/core/models/Cargo.toml ./crates/core/models/
COPY crates/core/parser/Cargo.toml ./crates/core/parser/
COPY crates/core/permissions/Cargo.toml ./crates/core/permissions/
COPY crates/core/presence/Cargo.toml ./crates/core/presence/
COPY crates/core/result/Cargo.toml ./crates/core/result/
COPY crates/services/autumn/Cargo.toml ./crates/services/autumn/
COPY crates/services/january/Cargo.toml ./crates/services/january/
COPY crates/daemons/crond/Cargo.toml ./crates/daemons/crond/
COPY crates/daemons/pushd/Cargo.toml ./crates/daemons/pushd/
RUN sh /tmp/build-image-layer.sh deps

# Build all apps
COPY crates ./crates
RUN sh /tmp/build-image-layer.sh apps

# Copy the start.sh script into the container
COPY scripts/start.sh ./start.sh
RUN chmod +x ./start.sh

# === Runtime Stage ===
FROM debian:bookworm-slim

WORKDIR /app

# Copy only the built binaries and start.sh script from builder stage
COPY --from=builder /home/rust/src/target/release/revolt-delta ./revolt-delta
COPY --from=builder /home/rust/src/target/release/revolt-bonfire ./revolt-bonfire
COPY --from=builder /home/rust/src/target/release/revolt-autumn ./revolt-autumn
COPY --from=builder /home/rust/src/target/release/revolt-january ./revolt-january
COPY --from=builder /home/rust/src/target/release/revolt-pushd ./revolt-pushd

COPY --from=builder /home/rust/src/start.sh ./start.sh
RUN chmod +x ./start.sh

# Install any runtime dependencies your binaries need (if any)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 14704 14705

CMD ["./start.sh"]