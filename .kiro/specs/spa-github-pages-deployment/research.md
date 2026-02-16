# Research & Design Decisions

## Summary
- **Feature**: `spa-github-pages-deployment`
- **Discovery Scope**: Extension（既存WASMアプリにビルドパイプラインとデプロイを追加）
- **Key Findings**:
  - Viteは公式ドキュメントでGitHub Pagesデプロイのワークフローを提供しており、`base` 設定でサブディレクトリパスに対応可能
  - `vite-plugin-wasm` はwasm-pack生成モジュールを明示的にサポートしており、`vite-plugin-top-level-await` と組み合わせて使用する
  - wasm-pack `--target bundler` はバンドラー統合に最適だが、`--target web` でもViteの `?url` インポートで対応可能

## Research Log

### ViteのWASMサポート方式
- **Context**: gap-analysisで `vite-plugin-wasm` vs Vite組み込みWASMサポートの比較が必要と判明
- **Sources Consulted**: Context7 MCP — `/vitejs/vite` docs, `/menci/vite-plugin-wasm` docs
- **Findings**:
  - Vite本体にはWASM ESMインポートの組み込みサポートがない（`.wasm` ファイルはアセットとして扱われる）
  - `vite-plugin-wasm` がWASM ESMインテグレーション（Webpackの `asyncWebAssembly` 相当）を提供
  - `vite-plugin-top-level-await` が必須の併用プラグイン（WASMの非同期初期化をトップレベルawaitで処理）
  - 設定は `plugins: [wasm(), topLevelAwait()]` のみでシンプル
- **Implications**: vite-plugin-wasmが最適。Vite組み込み機能だけでは不十分

### wasm-pack `--target web` vs `--target bundler`
- **Context**: 既存ビルドは `--target web` を使用。Viteとの互換性を検証する必要あり
- **Sources Consulted**: Context7 MCP — `/drager/wasm-pack` docs, `/menci/vite-plugin-wasm` docs
- **Findings**:
  - `--target web`: ES Modules形式で出力。`init()` が内部でfetch()を使ってwasmをロード。バンドラー不要
  - `--target bundler`: ES Modules形式で出力。`import * as wasm from './xxx_bg.wasm'` のESMインポートを使用。バンドラーがWASMハンドリングを担当
  - `vite-plugin-wasm` は `--target bundler` の出力形式（ESM WASMインポート）を直接サポート
  - `--target web` の場合、Viteバンドル後にfetch()の相対パスが壊れるリスクがある
- **Implications**: `--target bundler` への変更を推奨。main.jsのインポートパターンは `init()` 呼び出しを維持可能（wasm-bindgenがdefault exportとして`init`を生成）

### Vite GitHub Pagesデプロイ設定
- **Context**: GitHub Pagesデプロイの公式推奨パターンを調査
- **Sources Consulted**: Context7 MCP — Vite公式 static-deploy.md
- **Findings**:
  - Vite公式がGitHub Actionsワークフローのテンプレートを提供
  - `base: '/<REPO>/'` 設定が必須（プロジェクトページの場合）
  - `actions/upload-pages-artifact` + `actions/deploy-pages` の組み合わせを使用
  - `import.meta.env.BASE_URL` でランタイムにベースパスを参照可能
- **Implications**: 公式テンプレートをベースにRustツールチェーンのセットアップステップを追加する

### vite-plugin-wasm設定詳細
- **Context**: プラグインの具体的な設定方法と制約を確認
- **Sources Consulted**: Context7 MCP — `/menci/vite-plugin-wasm` README
- **Findings**:
  - 基本設定: `import wasm from 'vite-plugin-wasm'; import topLevelAwait from 'vite-plugin-top-level-await';`
  - Web Worker使用時はworkerオプションにもプラグインを追加する必要あり（本プロジェクトでは不要）
  - `vite-plugin-top-level-await` >= 1.3.0 ではworkerの `format: "es"` 設定は不要
  - wasm-pack生成モジュールのインポートを直接サポート
- **Implications**: 設定は最小限。Worker対応は将来の拡張ポイントとして記録

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Vite + vite-plugin-wasm + `--target bundler` | バンドラーがWASMロードを完全管理 | パス解決が確実、最小化可能、公式サポート | wasm-pack targetの変更が必要（`web` → `bundler`） | 推奨 |
| Vite + `--target web` + `?url` import | fetch()ベースのWASMロードを維持 | Rust側変更ゼロ | パス解決がバンドル後に壊れるリスク、追加ハック必要 | 代替案 |
| バンドラーなし + build.sh | シェルスクリプトでファイルコピー | 依存追加なし | JS最小化なし、dev server機能なし、要件未達 | 不適 |

## Design Decisions

### Decision: バンドラーとしてViteを採用
- **Context**: SPAビルドパイプラインにバンドラーが必要（Req 1）
- **Alternatives Considered**:
  1. Vite — 高速なdev server、ESMネイティブ、シンプルな設定
  2. Webpack — 成熟したWASMエコシステム、設定が複雑
  3. バンドラーなし — 要件（JS最小化、dev server）を満たせない
- **Selected Approach**: Vite
- **Rationale**: 設定がシンプル、ビルド速度が速い、公式にGitHub Pagesデプロイガイドあり。tech.mdの「フレームワーク不使用」方針と整合（Viteはフレームワークではなくビルドツール）
- **Trade-offs**: Node.js/npm依存が新規追加されるが、CI/CDで自動化されるため運用負荷は低い
- **Follow-up**: なし

### Decision: wasm-pack `--target bundler` への変更
- **Context**: `--target web` のfetch()パス解決がバンドル後に壊れるリスク
- **Alternatives Considered**:
  1. `--target bundler` — バンドラーがWASMパスを管理、vite-plugin-wasmが直接サポート
  2. `--target web` 維持 + `?url` import — Rust側変更ゼロだが、init()へのURL手動渡しが必要
- **Selected Approach**: `--target bundler`
- **Rationale**: vite-plugin-wasmが明示的にサポート。パス解決問題を根本解決。main.jsのインポートパターンは同等を維持可能
- **Trade-offs**: npm scriptsのwasm-packコマンドを変更する必要あり（`--target web` → `--target bundler`）。tech.mdのCommon Commandsも更新が必要
- **Follow-up**: main.jsでの`init()`呼び出しパターンの確認（`--target bundler`でもdefault exportとしてinit関数が生成される）

### Decision: フロントエンドソースをルート直下に維持
- **Context**: index.htmlとmain.jsの配置場所の決定
- **Alternatives Considered**:
  1. ルート直下維持 — Viteの規約（index.htmlがルートに必要）に合致、既存構成からの変更最小
  2. `web/` サブディレクトリ移動 — Rust/Webの分離が明確だが、Viteの`root`設定が必要
- **Selected Approach**: ルート直下維持
- **Rationale**: Viteはデフォルトでプロジェクトルートの`index.html`をエントリーポイントとして扱う。既存構成との差分最小化
- **Trade-offs**: Rustソースとフロントエンドソースが同じディレクトリレベルに共存するが、ファイル数が少ないため問題なし
- **Follow-up**: なし

## Risks & Mitigations
- `--target bundler` 変更後にWASMの初期化パターンが変わる可能性 — main.jsでの`init()`呼び出しを検証し、必要に応じてインポートパターンを調整
- GitHub ActionsでのRust/wasm-packビルド時間 — Rustツールチェーンとwasm-packのキャッシュを検討（ただし初回実装では不要）
- GitHub Pagesのファイルサイズ制限 — WASMバイナリは通常数百KB程度で問題なし

## References
- [Vite Static Deploy Guide](https://vite.dev/guide/static-deploy.html) — GitHub Pages デプロイの公式ガイド
- [vite-plugin-wasm README](https://github.com/menci/vite-plugin-wasm) — WASM ESM integration plugin
- [wasm-pack Build Command](https://rustwasm.github.io/wasm-pack/book/commands/build.html) — target オプションの仕様
- [GitHub Actions: deploy-pages](https://github.com/actions/deploy-pages) — GitHub Pages デプロイアクション
