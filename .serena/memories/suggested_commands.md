# Suggested Commands

## Build & Run
- `cargo build` — Build the project
- `cargo build --release` — Build with optimizations
- `cargo run` — Build and run the project
- `cargo run --release` — Build and run with optimizations

## Testing
- `cargo test` — Run all tests
- `cargo test <test_name>` — Run a specific test

## Code Quality
- `cargo fmt` — Format code with rustfmt
- `cargo fmt --check` — Check formatting without modifying files
- `cargo clippy` — Run Clippy linter
- `cargo clippy -- -D warnings` — Run Clippy and treat warnings as errors

## Dependency Management
- `cargo add <crate>` — Add a dependency
- `cargo update` — Update dependencies

## Documentation
- `cargo doc --open` — Generate and open documentation

## System Utilities (macOS / Darwin)
- `git status` — Check git status
- `git diff` — View changes
- `git log --oneline` — View recent commits
- `ls -la` — List files with details
- `find . -name "*.rs"` — Find Rust source files
- `grep -r "pattern" src/` — Search in source files
