default:
    just --list

check: clippy cargo-check fmt test

clippy:
    cargo clippy

cargo-check:
    cargo check

fmt:
    cargo fmt

test:
    cargo test
