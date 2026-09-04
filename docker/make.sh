#!/bin/bash

TARGET_ARCH="aarch64-unknown-linux-gnu"
SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# build
cd "$DIR/.."
cargo build --release --target "$TARGET_ARCH"
cp "$DIR/../target/$TARGET_ARCH/release/kosync" "$DIR/kosync"

# build docker
cd "$DIR"
docker build . -t kosync:$VERSION

# clean
rm "$DIR/kosync"
