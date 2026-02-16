# Technology Stack

## Architecture

クライアントサイド完結型のWebアプリケーション。Rustで計算コアを実装し、wasm-packでWasmバイナリにコンパイル。HTML/JavaScript側でWasmモジュールをロードし、Canvas 2Dコンテキストに描画する。

## Core Technologies

- **Language**: Rust (Edition 2024)
- **Compile Target**: WebAssembly (`wasm32-unknown-unknown`)
- **Binding**: wasm-bindgen (Rust ↔ JavaScript相互運用)
- **Build Tool**: wasm-pack (Wasmバイナリ + グルーコード生成)
- **Frontend**: HTML Canvas 2D API, vanilla JavaScript

## Key Libraries

- `wasm-bindgen` — JavaScript/DOM APIへのバインディング
- `web-sys` — Web API (Canvas, Document等) へのアクセス

## Development Standards

### Code Quality
- `cargo fmt` — デフォルト設定でのコードフォーマット
- `cargo clippy -- -D warnings` — 警告をエラーとして扱うリンティング

### Testing
- `cargo test` — ユニットテスト
- `wasm-pack test --headless --chrome` — Wasmブラウザテスト（将来的に）

## Development Environment

### Required Tools
- Rust toolchain (Edition 2024対応)
- wasm-pack
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

### Common Commands
```bash
# Build (native): cargo build
# Build (wasm): wasm-pack build --target web
# Test: cargo test
# Format: cargo fmt
# Lint: cargo clippy -- -D warnings
# Run dev server: 任意のHTTPサーバーで配信（例: python3 -m http.server）
```

## Key Technical Decisions

- **Rust Edition 2024**: 最新のRust機能を活用
- **wasm-pack + wasm-bindgen**: Wasmエコシステムの標準的なツールチェーンを採用
- **Canvas 2D API**: WebGL不要でシンプルなピクセル描画を実現
- **フレームワーク不使用**: フロントエンドフレームワークを使わず、vanilla JS + Wasmで軽量に構成

---
_Document standards and patterns, not every dependency_
