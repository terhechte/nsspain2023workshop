#!/bin/sh
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
cd ./fetch_dependencies
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
cargo install cargo-swift
cargo check
