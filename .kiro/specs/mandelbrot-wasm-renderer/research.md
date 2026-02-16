# Research & Design Decisions

---
**Purpose**: mandelbrot-wasm-renderer の技術設計に先立つ調査記録
---

## Summary
- **Feature**: `mandelbrot-wasm-renderer`
- **Discovery Scope**: New Feature（グリーンフィールド）
- **Key Findings**:
  - wasm-bindgen 0.2.108 + wasm-pack 0.14.0 が最新安定版。公式Julia Setサンプルの `&mut [u8]` 直接書き込みパターンを採用
  - Rust Edition 2024 では `static mut` 参照がdeny-by-defaultとなり、ピクセルバッファは構造体で管理する設計が必須
  - Cargo.toml に `[lib] crate-type = ["cdylib", "rlib"]` が必須（Context7検証で確認）
  - 非ブロッキング描画は `requestAnimationFrame` パターンで実現。Web Worker + OffscreenCanvasはスコープ外とする

## Research Log

### wasm-bindgen / web-sys のCanvas 2D連携
- **Context**: Rust Wasm からCanvas 2Dにピクセルデータを描画する最適な方法の調査
- **Sources Consulted**: wasm-bindgen公式examples、WasmByExample、docs.rs/web-sys
- **Findings**:
  - `ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), w, h)` で一括描画が可能
  - `put_image_data` でCanvasに転送。部分更新は `put_image_data_with_dirty_*` で対応可能
  - web-sys feature flags: `CanvasRenderingContext2d`, `Document`, `Element`, `HtmlCanvasElement`, `Window`, `ImageData` が必要
  - `JsCast::dyn_into::<T>()` による型キャストが必須
- **Implications**: 初期調査ではRust側ImageData生成を想定していたが、Context7検証で公式Julia Setパターンに変更（後述）

### Context7検証: wasm-bindgen公式Julia Setサンプル
- **Context**: Context7 MCPで取得したwasm-bindgen公式Julia Setフラクタルサンプルとの設計整合性検証
- **Sources Consulted**: Context7 `/rustwasm/wasm-bindgen` (736 snippets, High reputation)
- **Findings**:
  - 公式Julia Setサンプルでは **JS側がImageDataを作成し、`&mut [u8]` としてRustに渡す**方式を採用
  - Rust側は `pub fn render(data: &mut [u8]) -> usize` で直接バッファに書き込み
  - JS側: `ctx.createImageData()` → `Uint8ClampedArray` ビュー作成 → Rust呼び出し → `ctx.putImageData()`
  - wasm-bindgenが `&mut [u8]` と `Uint8ClampedArray` の自動変換をサポート
  - ただし公式サンプルは `static mut WIDTH/HEIGHT` を使用しており、Edition 2024では非互換
- **Implications**: JS-creates-buffer パターンを採用。`static mut` 部分は構造体ベースに置き換え

### Context7検証: Cargo.toml構成
- **Context**: wasm-packで正常にビルドするために必要なCargo.toml設定の確認
- **Sources Consulted**: Context7 `/drager/wasm-pack` (93 snippets, High reputation)
- **Findings**:
  - `[lib] crate-type = ["cdylib"]` が**必須**。これがないとwasm-packがWasmバイナリを生成できない
  - `rlib` を追加すると `cargo test` でのユニットテストも動作する
  - `console_error_panic_hook` が標準的なオプション依存として推奨
  - `wee_alloc`（サイズ最適化アロケータ）は現在非推奨傾向のため不採用
  - エントリーポイントは `src/lib.rs` であることが前提（`src/main.rs` ではない）
- **Implications**: Cargo.tomlに `[lib]` セクション追加、`src/main.rs` → `src/lib.rs` への変更がタスクに必要

### Context7検証: 構造体エクスポートパターン
- **Context**: `#[wasm_bindgen]` による構造体とメソッドのJSエクスポートの正確なパターン確認
- **Sources Consulted**: Context7 `/rustwasm/wasm-bindgen`
- **Findings**:
  - `#[wasm_bindgen]` を struct と impl ブロック両方に付与
  - `#[wasm_bindgen(constructor)]` でJS側の `new` 演算子対応
  - private フィールドはJSから不可視（設計通り）
  - `&mut self` メソッドは正常に動作（Rendererのrender/zoom/panに適用可能）
  - `pub` フィールドは自動的にgetter/setterが生成される
- **Implications**: 設計のRenderer構造体パターンはContext7で確認済み。変更不要

### Rust Edition 2024 と Wasm
- **Context**: Edition 2024固有の変更がWasmビルドに与える影響の確認
- **Sources Consulted**: Rust 1.85.0リリースノート、Edition Guide
- **Findings**:
  - `static mut` への参照がdeny-by-defaultエラー → グローバルバッファパターンは使用不可
  - `unsafe_op_in_unsafe_fn` がデフォルト警告 → unsafe操作は明示的なunsafeブロックが必要
  - `unsafe extern` ブロックが必須
  - LLVM 19で `multi-value` と `reference-types` がデフォルト有効
- **Implications**: ピクセルバッファは構造体フィールドとして管理。static mutは避ける

### 非ブロッキング描画パターン
- **Context**: UIスレッドをブロックしないレンダリング方式の調査
- **Sources Consulted**: wasm-bindgen requestAnimationFrame example、MDN OffscreenCanvas
- **Findings**:
  - `requestAnimationFrame` + `Rc<RefCell<Option<Closure>>>` パターンが標準的
  - Web Worker + OffscreenCanvasで完全なオフスレッド描画も可能だが複雑度が高い
  - マンデルブロ集合の1フレーム計算はWasmで十分高速（< 16ms目標）
- **Implications**: 初期実装は requestAnimationFrame パターンで十分。Worker化は将来の最適化オプションとする

### wasm-pack ビルドワークフロー
- **Context**: ビルドツールチェーンの最新状況確認
- **Sources Consulted**: wasm-pack 0.14.0 リリースノート
- **Findings**:
  - `wasm-pack build --target web` でES module対応のpkgを生成
  - 出力: `.js`（ラッパー）、`_bg.wasm`（バイナリ）、`.d.ts`（型定義）
  - `--dev` / `--release` プロファイル切替対応
  - HTML側は `<script type="module">` で `import init from './pkg/...'` する
- **Implications**: `--target web` を標準ビルドターゲットとする

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| レイヤード（採用） | 計算コア → Wasmバインディング → JS描画ブリッジ の3層 | 計算ロジックの独立テスト可能、責務が明確 | 層間のデータ受け渡しにコピーコスト | steering structure.mdの方針と一致 |
| モノリシック | 全ロジックをlib.rsに集約 | シンプル、初期実装が速い | 成長時にメンテナンス困難 | 小規模プロジェクトには十分だが拡張性に欠ける |
| Worker分離 | 計算をWeb Workerに分離 | UIスレッド完全非ブロッキング | SharedArrayBuffer/OffscreenCanvas依存、複雑度大 | 将来の最適化として検討 |

## Design Decisions

### Decision: ピクセルバッファ受け渡し方式
- **Context**: Wasm側で計算したピクセルデータをCanvasに転送する方式の選択
- **Alternatives Considered**:
  1. `static mut` グローバルバッファ + ポインタ公開 — WasmByExampleの古いパターン
  2. Rust側で `ImageData` を生成し `put_image_data` を呼ぶ — web-sys依存パターン
  3. JS側が `ImageData` を作成し `&mut [u8]` としてRustに渡す — wasm-bindgen公式Julia Setパターン
- **Selected Approach**: Option 3（JS-creates-buffer パターン）
- **Rationale**:
  - wasm-bindgen公式フラクタル描画サンプル（Julia Set）と同一パターンで信頼性が高い（Context7検証済み）
  - Rust側がweb-sysの `ImageData` APIに依存しなくなり、コードがシンプルになる
  - `&mut [u8]` はwasm-bindgenが `Uint8ClampedArray` と自動変換するため、追加コード不要
  - Rust 2024で `static mut` 参照がdeny-by-defaultのため Option 1 は不可
  - Option 2 はweb-sys依存が増え、Rust側のテストが複雑化する
- **Trade-offs**: JS側に `createImageData` / `putImageData` の責務が増えるが、数行のコードで済む
- **Follow-up**: 大きなキャンバスサイズでのパフォーマンス測定

### Decision: Cargo.toml構成
- **Context**: wasm-packで正常にビルドするための必須設定
- **Selected Approach**: `[lib] crate-type = ["cdylib", "rlib"]` を設定し、エントリーポイントを `src/lib.rs` に変更
- **Rationale**: `cdylib` はwasm-packによるWasmバイナリ生成に必須（Context7検証済み）。`rlib` は `cargo test` でのユニットテスト実行に必要
- **Follow-up**: `src/main.rs` を `src/lib.rs` に置き換えるタスクが必要

### Decision: フロントエンドアーキテクチャ
- **Context**: フロントエンドフレームワーク使用の要否
- **Alternatives Considered**:
  1. React/Vue等のSPAフレームワーク
  2. Vanilla JS + ES modules
- **Selected Approach**: Vanilla JS + ES modules
- **Rationale**: UIはCanvas1つとイベントリスナーのみ。フレームワークのオーバーヘッドは不要。steering tech.mdの方針と一致
- **Trade-offs**: 状態管理はJS側で手動管理が必要だが、状態が少ないため問題なし

### Decision: 非ブロッキング描画方式
- **Context**: 重い計算中にUIがフリーズしない設計
- **Alternatives Considered**:
  1. requestAnimationFrame + 同期計算（1フレーム全計算）
  2. Web Worker + OffscreenCanvas
  3. チャンク分割計算（数行ずつ計算、rAFで分割）
- **Selected Approach**: Option 1（rAF + 同期計算）をベースとし、必要に応じてOption 3を検討
- **Rationale**: Wasm計算は十分高速で、800x600程度なら16ms以内に収まる見込み。Worker化は複雑度が大きい
- **Trade-offs**: 非常に高解像度や深いズームでは一時的にフレームドロップの可能性
- **Follow-up**: プロファイリングで16ms超過が確認された場合、チャンク分割を導入

### Decision: デバッグ支援
- **Context**: Wasmパニック時のエラー出力改善
- **Selected Approach**: `console_error_panic_hook` を optional dependency として追加
- **Rationale**: wasm-packテンプレートで標準的に含まれており、パニック時にブラウザコンソールに詳細なスタックトレースを出力する。開発効率が大幅に向上する（Context7検証済み）

## Risks & Mitigations
- **深いズームでの精度不足（f64）** — f64の精度限界（約1e-15）を超えるズームでは描画がブロック化する。初期スコープでは対応不要、将来的に任意精度ライブラリを検討
- **ブラウザ互換性** — wasm-bindgen + web-sys は主要ブラウザ対応済み。IE非対応だが対象外
- **大キャンバスでのパフォーマンス** — 4K解像度等ではフレーム時間が増大。初期はウィンドウサイズ依存で対応
- **Cargo.toml構成ミス** — `[lib] crate-type` の設定漏れでwasm-packビルドが失敗するリスク。タスク化して確実に対応する

## References
- [wasm-bindgen 0.2.108 Release](https://github.com/wasm-bindgen/wasm-bindgen/releases) — 最新バージョン確認
- [wasm-pack 0.14.0 Release](https://github.com/rustwasm/wasm-pack/releases) — ビルドツール最新版
- [web-sys Canvas Example](https://rustwasm.github.io/docs/wasm-bindgen/examples/2d-canvas.html) — Canvas 2D連携パターン
- [wasm-bindgen Julia Set Example](https://github.com/rustwasm/wasm-bindgen/blob/main/guide/src/examples/julia.md) — 公式フラクタル描画サンプル（Context7で取得）
- [Rust 1.85.0 / Edition 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/) — Edition 2024の変更点
- [requestAnimationFrame Example](https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html) — rAFパターン
- [MDN OffscreenCanvas](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas) — 将来の最適化参考
- Context7 MCP `/rustwasm/wasm-bindgen` — 構造体エクスポート、Closureパターン、Canvas描画の検証
- Context7 MCP `/drager/wasm-pack` — Cargo.toml構成、ビルドコマンドの検証
