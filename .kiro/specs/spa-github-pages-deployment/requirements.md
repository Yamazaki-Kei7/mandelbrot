# Requirements Document

## Introduction

既存のマンデルブロ集合WASMレンダラーを、バンドラーを用いたSPA（Single Page Application）として構成し直し、GitHub Pages（github.io）で一般公開する。現在はローカルHTTPサーバーで静的ファイルを配信する構成だが、ビルドパイプラインの整備とCI/CDによる自動デプロイを実現する。

## Requirements

### Requirement 1: SPAビルドパイプライン

**Objective:** 開発者として、WASMバイナリとフロントエンドアセットを単一のビルドコマンドで生成したい。これにより、手動でのwasm-packビルドと静的ファイル管理の手間を排除する。

#### Acceptance Criteria

1. The SPA Build System shall wasm-packによるWASMコンパイルとフロントエンドアセットのバンドルを単一コマンドで実行する
2. The SPA Build System shall ビルド成果物を `dist/` ディレクトリに出力する（HTML、JS、WASM、CSS等すべてのアセットを含む）
3. The SPA Build System shall WASMファイルを適切にロード可能な形式でバンドルする（Content-Type、パス解決を含む）
4. When ソースコード（Rust/JS/HTML/CSS）が変更された場合, the SPA Build System shall 変更を検知して自動的にリビルドする（開発モード）
5. The SPA Build System shall 本番ビルドにおいてJavaScriptの最小化とWASMの最適化を行う

### Requirement 2: 開発サーバー

**Objective:** 開発者として、ローカルで変更を即座にプレビューできる開発サーバーが欲しい。これにより、開発サイクルを高速化する。

#### Acceptance Criteria

1. The Development Server shall ローカルHTTPサーバーを起動し、ビルド成果物をブラウザからアクセス可能にする
2. The Development Server shall WASMファイルに対して正しいMIMEタイプ（`application/wasm`）を返す
3. When ソースファイルが変更された場合, the Development Server shall 自動リビルドを実行する

### Requirement 3: GitHub Pages デプロイメント

**Objective:** 開発者として、mainブランチへのプッシュ時にGitHub Pagesへ自動デプロイしたい。これにより、手動デプロイの手間とミスを排除する。

#### Acceptance Criteria

1. When mainブランチにコードがプッシュされた場合, the CI/CD Pipeline shall WASMビルドを含むフルビルドを自動実行する
2. When ビルドが成功した場合, the CI/CD Pipeline shall ビルド成果物をGitHub Pagesに自動デプロイする
3. The CI/CD Pipeline shall GitHub Actions ワークフローとして構成する
4. The CI/CD Pipeline shall Rustツールチェーン、wasm-pack、Node.js環境のセットアップを自動化する
5. If ビルドが失敗した場合, the CI/CD Pipeline shall デプロイを中止しエラーログを残す

### Requirement 4: SPA構成とアセット管理

**Objective:** ユーザーとして、github.ioのURLからアプリケーションにアクセスし、マンデルブロ集合をインタラクティブに操作したい。これにより、インストールなしでブラウザから利用可能になる。

#### Acceptance Criteria

1. The SPA shall 単一のHTMLエントリーポイントからアプリケーション全体をロードする
2. The SPA shall GitHub Pages のサブディレクトリパス（`https://<user>.github.io/<repo>/`）で正しく動作する
3. The SPA shall WASMモジュールの初期化完了まで適切なローディング表示を行う
4. The SPA shall 既存のマンデルブロ集合描画機能（ズーム、パン、リアルタイム描画）をすべて維持する
5. While WASMモジュールがロード中の場合, the SPA shall ユーザーにローディング状態を視覚的に伝える

### Requirement 5: リポジトリ構成の整理

**Objective:** 開発者として、ソースコードとビルド成果物が明確に分離された構成にしたい。これにより、リポジトリの管理性と可読性を向上させる。

#### Acceptance Criteria

1. The Repository shall `pkg/` および `dist/` ディレクトリをバージョン管理から除外する（`.gitignore`に追加）
2. The Repository shall フロントエンドの依存関係を `package.json` で管理する
3. The Repository shall ビルド手順とデプロイ手順をREADMEまたはドキュメントに記載する
