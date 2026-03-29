default:
    just --list

check: update clippy cargo-check fmt test outdated

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
