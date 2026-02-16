# ギャップ分析: spa-github-pages-deployment

## 1. 現状調査

### 既存アセット

| カテゴリ | アセット | 状態 |
|---------|---------|------|
| WASM計算コア | `src/lib.rs`, `core.rs`, `color.rs`, `viewport.rs`, `renderer.rs` | 完成済み・変更不要 |
| フロントエンド | `index.html`, `main.js`（プロジェクトルート直下） | 動作済み・要移行 |
| WASMビルド出力 | `pkg/`（wasm-pack生成） | 自動生成・gitignore済み |
| ビルド設定 | `Cargo.toml`（`wasm-pack build --target web`） | 完成済み・変更不要 |
| Git Remote | `https://github.com/Yamazaki-Kei7/mandelbrot.git` | 設定済み |

### 現在のアーキテクチャパターン

- **バンドラー不使用**: vanilla JS + ES Modulesで直接 `./pkg/mandelbrot.js` をインポート
- **開発サーバー**: 任意のHTTPサーバー（`python3 -m http.server`等）で手動配信
- **ビルドフロー**: `wasm-pack build --target web` のみ、フロントエンドのビルドステップなし
- **wasm-pack target**: `--target web`（ES Modules形式、バンドラー不要）

### 統合サーフェス

- `main.js` → `./pkg/mandelbrot.js` の `init()` と `Renderer` をインポート
- `init()` は同ディレクトリの `.wasm` ファイルを自動フェッチ（相対パス解決）
- Canvas API のみ使用、外部API依存なし

---

## 2. 要件フィージビリティ分析

### 要件→アセット マッピング

| 要件 | 既存アセット | ギャップ |
|------|-------------|---------|
| Req 1: SPAビルドパイプライン | `Cargo.toml`, `wasm-pack` | **Missing**: バンドラー、`package.json`、統合ビルドスクリプト |
| Req 2: 開発サーバー | なし（手動HTTPサーバー） | **Missing**: WASM対応の開発サーバー、ウォッチモード |
| Req 3: GitHub Pagesデプロイ | Git remote設定済み | **Missing**: `.github/workflows/`, GitHub Actions設定全体 |
| Req 4: SPA構成 | `index.html`, `main.js` | **Missing**: ローディングUI、サブディレクトリパス対応 |
| Req 5: リポジトリ整理 | `.gitignore`（/target, /pkg） | **Missing**: `dist/` の除外、ルート `package.json`、フロントエンドソースの整理 |

### 技術的ニーズ

- **バンドラー**: WASM対応のモダンバンドラー（JS最小化、アセット管理、dev server）
- **ビルドオーケストレーション**: wasm-pack → バンドラーの2段階ビルドを統合するnpmスクリプト
- **CI/CD**: GitHub Actions ワークフロー（Rust + wasm-pack + Node.js 環境）
- **ベースパス設定**: GitHub Pages の `/<repo>/` サブパスに対応するバンドラー設定

### 制約事項

- **Constraint**: tech.md にて「フレームワーク不使用」を明記 → フロントエンドフレームワーク（React等）は導入しない
- **Constraint**: `--target web` は ES Modules形式を出力。バンドラーと組み合わせる場合 `--target bundler` への変更が必要になる可能性あり
- **Research Needed**: `--target web` vs `--target bundler` のViteとの互換性検証

---

## 3. 実装アプローチオプション

### Option A: Vite + vite-plugin-wasm

**概要**: Viteをバンドラーとして採用し、WASM対応プラグインで統合

- **変更対象**:
  - `index.html` → ルートに維持（Viteのエントリーポイント規約に合致）
  - `main.js` → `./pkg/` からのインポートパスをバンドラー解決に変更
  - 新規: `package.json`, `vite.config.js`
  - 新規: `.github/workflows/deploy.yml`
  - wasm-pack target: `--target web` を維持可能（Viteは ES Modules を直接処理可能）

- **ビルドフロー**:
  1. `wasm-pack build --target web` → `pkg/` 生成
  2. `vite build` → `pkg/` を含めて `dist/` に出力

**トレードオフ**:
- ✅ 高速な開発サーバー（ESビルド + HMR）
- ✅ WASM ファイルのネイティブサポート
- ✅ `base` 設定でGitHub Pagesサブパスに簡単対応
- ✅ 既存の `--target web` 出力と互換
- ✅ 設定がシンプル（最小限の `vite.config.js`）
- ❌ Node.js/npm 依存が新規追加

### Option B: Webpack + @aspect-build/rules-wasm 等

**概要**: Webpackをバンドラーとして採用し、WASM対応ローダーで統合

- **変更対象**:
  - `main.js` → webpack entry pointとして再構成
  - `index.html` → HTMLWebpackPluginで管理
  - 新規: `package.json`, `webpack.config.js`
  - wasm-pack target: `--target bundler` への変更が必要

**トレードオフ**:
- ✅ 成熟したWASMエコシステム（wasm-pack-plugin等）
- ✅ `--target bundler` との統合実績が豊富
- ❌ 設定の複雑さ（loader, plugin設定が多い）
- ❌ ビルド速度がViteに劣る
- ❌ dev serverのHMR設定がWASMと相性が悪い場合あり

### Option C: バンドラー不使用（静的ファイル + ビルドスクリプト）

**概要**: バンドラーを導入せず、シェルスクリプトでwasm-packビルドとファイルコピーを行う

- **変更対象**:
  - 新規: `build.sh`（wasm-pack + cp で dist/ 構成）
  - `index.html`, `main.js` はほぼ現状維持
  - 新規: `.github/workflows/deploy.yml`

**トレードオフ**:
- ✅ 依存追加なし（Node.js不要）
- ✅ 現在のアーキテクチャを最大限維持
- ❌ JS最小化なし（Req 1-5 未達）
- ❌ 開発サーバーのウォッチモードなし（Req 2 未達）
- ❌ WASMのパス解決がGitHub Pagesサブディレクトリで問題になりやすい

---

## 4. 実装複雑度とリスク

| 項目 | 評価 | 根拠 |
|------|------|------|
| **工数** | **S（1〜3日）** | 既存コア（Rust/WASM）は変更不要。バンドラー設定 + CI/CDワークフロー追加のみ |
| **リスク** | **Low** | 確立された技術（Vite + wasm-pack + GitHub Actions）の組み合わせ。アーキテクチャ変更なし |

---

## 5. 設計フェーズへの推奨事項

### 推奨アプローチ: Option A（Vite）

- Viteはモダンフロントエンドの標準的ビルドツールであり、WASM対応も成熟している
- `--target web` のまま使用可能であれば、Rust側の変更ゼロで済む
- GitHub Pagesへのデプロイワークフローは `actions/deploy-pages` で定型化されている

### 設計フェーズでの調査項目

1. **Research Needed**: `vite-plugin-wasm` vs `vite-plugin-wasm-pack` vs Vite組み込みWASMサポートの比較
2. **Research Needed**: `wasm-pack --target web` の出力をViteがそのまま解決できるか、`--target bundler` への変更が必要かの検証
3. **Key Decision**: フロントエンドソースの配置（ルート直下維持 vs `web/` サブディレクトリ移動）
4. **Key Decision**: GitHub Actions で使用する `actions/deploy-pages` の具体的な設定
