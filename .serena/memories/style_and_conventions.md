# Code Style and Conventions

## Rust Conventions
- **Edition**: Rust 2024
- **Naming**: Follow standard Rust naming conventions
  - `snake_case` for functions, methods, variables, modules
  - `CamelCase` (PascalCase) for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants and statics
- **Formatting**: Use `cargo fmt` (default rustfmt settings — no rustfmt.toml found)
- **Linting**: Use `cargo clippy` for additional linting

## No Custom Configuration Files
- No `rustfmt.toml` — uses default rustfmt settings
- No `.clippy.toml` — uses default Clippy settings
- No `rust-toolchain.toml` — uses system default toolchain

## General Guidelines (from CLAUDE.md)
- Avoid hard-coding values unless absolutely necessary
- Keep solutions simple and focused — avoid over-engineering
- Prefer editing existing files over creating new ones
