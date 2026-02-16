# Product Overview

Rustで実装されたマンデルブロ集合の描画プログラム。WebAssembly (Wasm) にコンパイルし、ブラウザ上でインタラクティブにフラクタル画像を表示・操作できるWebアプリケーション。

## Core Capabilities

- **フラクタル計算エンジン**: マンデルブロ集合の反復計算 (z = z² + c) をRustで高速に実行
- **WebAssembly描画**: Wasmを通じてブラウザのHTML Canvas上にリアルタイム描画
- **インタラクティブ操作**: ズーム・パンによるフラクタルの詳細探索

## Target Use Cases

- フラクタル幾何学の視覚的な探索・学習
- Rust + Wasm技術スタックの実践的なデモンストレーション
- 数学的に美しいフラクタル画像のインタラクティブ生成

## Value Proposition

RustのパフォーマンスをWasmを通じてブラウザに持ち込むことで、インストール不要で高速なフラクタル描画体験を提供する。計算集約的な処理をWasmで行い、JavaScriptでは実現しにくい滑らかな描画性能を実現する。

---
_Focus on patterns and purpose, not exhaustive feature lists_
