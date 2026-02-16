# Project Structure

## Organization Philosophy

機能レイヤー分離型。Rust側は計算ロジックとWasmバインディングに集中し、フロントエンド側（HTML/JS）は描画とユーザーインタラクションを担当する。

## Directory Patterns

### Rust Source (`src/`)
**Purpose**: マンデルブロ集合の計算コアとWasmエクスポート関数
**Pattern**: `lib.rs` にWasmエントリーポイント、モジュール分割は機能単位

### Web Frontend (`www/` or project root)
**Purpose**: HTMLページ、JavaScript グルーコード、スタイル
**Pattern**: Wasmモジュールのロードと Canvas描画のブリッジ

### Wasm Build Output (`pkg/`)
**Purpose**: wasm-packが生成するWasmバイナリとJSバインディング
**Pattern**: 自動生成、バージョン管理対象外

## Naming Conventions

- **Rustファイル**: snake_case (`mandelbrot.rs`, `color_map.rs`)
- **Rust関数/変数**: snake_case
- **Rust型/トレイト**: PascalCase
- **Rust定数**: SCREAMING_SNAKE_CASE
- **JSファイル**: kebab-case (`index.js`)
- **HTMLファイル**: kebab-case (`index.html`)

## Module Organization

```rust
// Rust module pattern
mod core;       // マンデルブロ集合の計算ロジック (compute_pixel, compute_region)
mod color;      // カラーマッピング (HSL→RGB, iteration→color変換)
mod viewport;   // ビューポート管理 (ズーム・パン・座標変換)
mod renderer;   // Wasmエクスポート境界 (#[wasm_bindgen] struct Renderer)

// lib.rs は pub use でモジュールを再エクスポート
pub use core::*;
pub use color::*;
pub use viewport::*;
pub use renderer::*;
```

## Code Organization Principles

- 計算ロジックはWasm依存から分離し、純粋なRustとしてテスト可能にする
- `#[wasm_bindgen]` はエクスポート境界のみに使用し、内部関数には付与しない
- フロントエンドのJSは最小限に保ち、重い処理はすべてWasm側で行う

---
_Document patterns, not file trees. New files following patterns shouldn't require updates_
_updated_at: 2026-02-16 — モジュール構成を実装に合わせて更新 (4モジュール構成)_
