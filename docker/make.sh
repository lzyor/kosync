#!/bin/bash

SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# build
cd "$DIR/.."
cargo build --release --target x86_64-unknown-linux-musl
cp "$DIR/../target/x86_64-unknown-linux-musl/release/kosync" "$DIR/kosync"

# build docker
cd "$DIR"
docker build . -t kosync:$VERSION

# clean
rm "$DIR/kosync"
