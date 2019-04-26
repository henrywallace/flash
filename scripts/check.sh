#!/bin/sh

set -ex

cargo fmt -- --check
cargo clippy --all-targets
cargo test

