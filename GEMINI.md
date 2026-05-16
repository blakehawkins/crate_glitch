# crate_glitch

## Project Overview
`crate_glitch` is a Matrix bot written in Rust. It monitors messages in a specified Matrix channel and provides links based on simple pattern matching (e.g., matching `!crate serde` and replying with `https://crates.io/crates/serde`).

**Key Technologies:**
- **Language:** Rust (Edition 2024)
- **Async Runtime:** `tokio`
- **Matrix Integration:** `matrix-sdk`
- **Configuration:** `serde` and `serde_yaml_ng` for parsing a YAML configuration file.

## Building and Running

The project relies on Cargo for dependency management and building, and uses `just` as a command runner for common development tasks.

- **Check everything:** `just check` (runs clippy, cargo check, fmt, test, and outdated dependencies)
- **Format code:** `just fmt`
- **Lint code:** `just clippy`
- **Run tests:** `just test`
- **Build/Run:** `cargo run -- <config_file>` (defaults to `config.yaml` if no file is provided)

*Note: The project currently pins the Rust toolchain to `1.93.1` via `rust-toolchain.toml` to work around a compiler recursion limit regression in `matrix-sdk`.*

## Development Conventions

- **Error Handling:** Uses the `anyhow` crate for flexible and context-aware error handling.
- **Configuration:** A template is provided in `config.yaml.template`. Before running the bot locally, copy this template to `config.yaml` and populate it with valid Matrix credentials and room information.
- **Formatting:** Code formatting is enforced via `cargo fmt`. Make sure to run `just fmt` or `cargo fmt` before committing changes.
