#!/bin/bash

set -e

echo "Building target=x86_64-unknown-linux-musl in release mode. This will take some time!"

TARGET_CC=x86_64-linux-musl-gcc \
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
cargo build --release --target=x86_64-unknown-linux-musl

docker build -t scav/colour-service:latest -f Dockerfile.linux .