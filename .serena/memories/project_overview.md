# Project Overview: mandelbrot

## Purpose
A Rust project for Mandelbrot set computation/rendering. Currently in initial stage with only a "Hello, world!" main function.

## Tech Stack
- **Language**: Rust (Edition 2024)
- **Build System**: Cargo
- **Dependencies**: None (as of initial setup)
- **Version**: 0.1.0

## Project Structure
```
mandelbrot/
├── Cargo.toml          # Project manifest
├── Cargo.lock          # Dependency lock file
├── CLAUDE.md           # AI assistant instructions
├── .gitignore          # Git ignore rules (/target)
├── .serena/            # Serena configuration
│   ├── project.yml     # Project settings
│   └── memories/       # Onboarding memories
├── .claude/            # Claude Code configuration
│   ├── settings.local.json
│   └── commands/kiro/  # Kiro spec-driven development commands
├── .kiro/              # Kiro settings and templates
│   └── settings/
│       ├── rules/      # Design and development rules
│       └── templates/  # Spec and steering templates
└── src/
    └── main.rs         # Entry point (currently Hello World)
```

## Development Methodology
This project uses **Spec-Driven Development** with Kiro-style workflows:
1. Requirements → Design → Tasks → Implementation
2. Steering documents in `.kiro/steering/`
3. Specifications in `.kiro/specs/`

## Key Notes
- All responses should be in Japanese (per CLAUDE.md)
- Design documents go in `.tmp/` directory
- File encoding: UTF-8
