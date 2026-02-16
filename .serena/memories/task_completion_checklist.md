# Task Completion Checklist

When a task is completed, run the following checks:

1. **Format code**: `cargo fmt`
2. **Run linter**: `cargo clippy -- -D warnings`
3. **Run tests**: `cargo test`
4. **Build check**: `cargo build`

All four commands must pass without errors before considering a task complete.
