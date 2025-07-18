# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/

################################################################################
# Base image as the foundation for the other build stages in this file.
FROM lukemathwalker/cargo-chef:latest-rust-1-slim-bookworm AS chef
WORKDIR /app

################################################################################
# Create a stage for cargo chef prepare recipe.
FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

################################################################################
# Create a stage for building/compiling the application.
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application.
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.

COPY . .
RUN cargo build --release --bin stackclass-server

################################################################################
# Create a final stage for running your application.
#
# The following commands copy the output from the "build" stage above and tell
# the container runtime to execute it when the image is run. Ideally this stage
# contains the minimal runtime dependencies for the application as to produce
# the smallest image possible. This often means using a different and smaller
# image than the one used for building the application, but for illustrative
# purposes the "base" image is used here.
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN adduser --uid 1001 --home /home/axum axum \
    && chown axum:axum /home/axum
USER axum

RUN git config --global user.name "StackClass" && \
    git config --global user.email "hello@stackclass.dev"

# Copy the executable from the "building" stage.
COPY --from=builder \
    /app/target/release/stackclass-server \
    /usr/local/bin/

EXPOSE 8080

# What the container should run when it is started
ENTRYPOINT ["/usr/local/bin/stackclass-server"]
