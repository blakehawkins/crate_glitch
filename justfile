default:
    just --list

check: clippy cargo-check fmt test update outdated

clippy:
    cargo clippy

cargo-check:
    cargo check

fmt:
    cargo fmt

test:
    cargo test

update:
    cargo update

outdated:
    cargo outdated -R
