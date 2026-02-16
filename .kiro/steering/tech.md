# Technology Stack

## Architecture

クライアントサイド完結型のWebアプリケーション。Rustで計算コアを実装し、wasm-packでWasmバイナリにコンパイル。HTML/JavaScript側でWasmモジュールをロードし、Canvas 2Dコンテキストに描画する。

## Core Technologies

- **Language**: Rust (Edition 2024)
- **Compile Target**: WebAssembly (`wasm32-unknown-unknown`)
- **Binding**: wasm-bindgen (Rust ↔ JavaScript相互運用)
- **Build Tool**: wasm-pack (Wasmバイナリ + グルーコード生成), Vite (バンドル + 開発サーバー)
- **Frontend**: HTML Canvas 2D API, vanilla JavaScript (フレームワーク不使用)

## Key Libraries

- `wasm-bindgen` — JavaScript/DOM APIへのバインディング
- `web-sys` — Web API (Canvas, Document等) へのアクセス
- `console_error_panic_hook` — Wasmパニック時のコンソールエラー出力
- `vite-plugin-wasm` / `vite-plugin-top-level-await` — ViteでのWasmバンドリングサポート

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
- Node.js (v20+) + npm
- cargo-watch (ホットリロード開発時)

### Common Commands
```bash
# Build (native): cargo build
# Build (wasm + web): npm run build
# Build (wasm only): npm run build:wasm  (wasm-pack build --target bundler)
# Dev server: npm run dev  (wasm-watchとVite dev serverを並行実行)
# Test: cargo test
# Format: cargo fmt
# Lint: cargo clippy -- -D warnings
# Preview: npm run preview  (Viteプレビューサーバー)
```

## Key Technical Decisions

- **Rust Edition 2024**: 最新のRust機能を活用
- **wasm-pack + wasm-bindgen**: Wasmエコシステムの標準的なツールチェーンを採用
- **Canvas 2D API**: WebGL不要でシンプルなピクセル描画を実現
- **フレームワーク不使用**: UIフレームワーク (React/Vue等) を使わず、vanilla JS + Wasmで軽量に構成
- **Vite**: バンドラー兼開発サーバーとして採用。`--target bundler` でwasm-packの出力をViteに統合
- **GitHub Pages デプロイ**: GitHub Actionsで `master` pushをトリガーに自動ビルド・デプロイ (`base: '/mandelbrot/'`)

---
_Document standards and patterns, not every dependency_
_updated_at: 2026-02-16 — Vite統合、GitHub Pagesデプロイ、開発ワークフローを反映_
