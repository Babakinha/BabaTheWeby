#!/bin/sh
rm -rf docs
dx build --features web --release
cargo run --features ssr --release
