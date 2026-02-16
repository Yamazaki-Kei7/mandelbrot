# Mandelbrot Set Explorer

Rust と WebAssembly を使用したインタラクティブなマンデルブロ集合のビジュアライザー。

## 概要

このプロジェクトは、Rust で実装されたマンデルブロ集合の計算エンジンを WebAssembly にコンパイルし、ブラウザ上で高速に描画・操作できるようにしたものです。

## 機能

- **リアルタイム描画**: WebAssembly による高速な計算
- **インタラクティブな操作**: マウス操作による拡大・縮小・移動
- **カラーグラデーション**: 美しい色付けによる集合の可視化

## プロジェクト構成

```
mandelbrot/
├── src/
│   ├── lib.rs          # メインライブラリ
│   ├── core.rs         # マンデルブロ集合の計算ロジック
│   ├── color.rs        # カラーマッピング
│   ├── renderer.rs     # Canvas描画ロジック
│   └── viewport.rs     # ビューポート管理
├── index.html          # Webインターフェース
├── main.js             # JavaScript エントリーポイント
├── Cargo.toml          # Rust プロジェクト設定
└── pkg/                # ビルドされた WASM パッケージ
```

## 必要な環境

- Rust (2024 edition)
- wasm-pack
- Node.js (開発サーバー用)

## セットアップ

1. **Rust のインストール**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack のインストール**
   ```bash
   cargo install wasm-pack
   ```

3. **プロジェクトのビルド**
   ```bash
   wasm-pack build --target web
   ```

## 実行方法

ローカルサーバーを起動してブラウザで開きます：

```bash
# Python を使用する場合
python3 -m http.server 8080

# Node.js の http-server を使用する場合
npx http-server -p 8080
```

ブラウザで `http://localhost:8080` を開いてください。

## 使い方

- **マウスドラッグ**: 表示領域を移動
- **スクロール/ピンチ**: 拡大・縮小
- **クリック**: その点を中心に表示

## 技術スタック

- **Rust**: 計算エンジンの実装
- **WebAssembly**: ブラウザでの高速実行
- **wasm-bindgen**: Rust と JavaScript の連携
- **web-sys**: Web API へのアクセス
- **Canvas API**: 描画

## 開発

このプロジェクトは Kiro-style Spec Driven Development に従って開発されています。開発に関する詳細は `.kiro/` ディレクトリと `CLAUDE.md` を参照してください。

## ライセンス

MIT License

## 作者

Keiyama
