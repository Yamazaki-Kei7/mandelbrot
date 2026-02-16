# Requirements Document

## Introduction
本ドキュメントは、Rustで実装するマンデルブロ集合描画プログラムの要件を定義する。計算コアをWebAssembly (Wasm) にコンパイルし、ブラウザ上のHTML Canvas要素を通じてフラクタル画像をインタラクティブに描画・操作できるWebアプリケーションを構築する。

## Requirements

### Requirement 1: マンデルブロ集合の計算
**Objective:** ユーザーとして、複素平面上の任意の領域に対するマンデルブロ集合の帰属判定を正確に行いたい。これにより、フラクタル画像を忠実に描画できるようにする。

#### Acceptance Criteria
1. The Renderer shall 複素平面上の各ピクセルに対応する座標についてマンデルブロ集合の反復計算 (z = z² + c) を実行する
2. The Renderer shall 設定可能な最大反復回数に基づいて各ピクセルの発散判定を行う
3. The Renderer shall 発散までの反復回数に基づいて各ピクセルの色を決定する
4. When ピクセルが最大反復回数に達しても発散しない場合, the Renderer shall そのピクセルを集合内（黒色）として扱う

### Requirement 2: WebAssemblyビルドとブラウザ統合
**Objective:** ユーザーとして、ブラウザだけでマンデルブロ集合を閲覧したい。追加のソフトウェアインストールなしで利用できるようにする。

#### Acceptance Criteria
1. The Renderer shall Rustソースコードから `wasm32-unknown-unknown` ターゲットにコンパイルできる
2. The Renderer shall wasm-bindgenを使用してJavaScriptとのバインディングを生成する
3. The Renderer shall HTMLファイルからWasmモジュールをロードし、Canvas上に描画結果を表示する
4. The Renderer shall ビルドツール（wasm-pack）を使用して、Wasmバイナリとグルーコードを生成する

### Requirement 3: Canvas描画
**Objective:** ユーザーとして、マンデルブロ集合の計算結果をブラウザ画面上で視覚的に確認したい。鮮明なフラクタル画像として表示されることを期待する。

#### Acceptance Criteria
1. The Renderer shall 計算結果をRGBAピクセルデータとしてHTML Canvas 2Dコンテキストに描画する
2. The Renderer shall Canvas全体を1フレームとして一括描画する
3. The Renderer shall 画面のCanvasサイズに合わせた解像度で描画する

### Requirement 4: ズームとパン操作
**Objective:** ユーザーとして、マンデルブロ集合の特定の領域を拡大したり、表示位置を移動したりして、フラクタルの詳細な構造を探索したい。

#### Acceptance Criteria
1. When ユーザーがマウスホイールでスクロールした場合, the Renderer shall カーソル位置を中心にズームイン・ズームアウトする
2. When ユーザーがCanvasをドラッグした場合, the Renderer shall 表示領域をドラッグ方向にパンする
3. While ズーム操作中, the Renderer shall 現在のズームレベルに応じて複素平面の表示範囲を再計算する
4. The Renderer shall 初期表示として複素平面上の標準的な領域（実部 -2.0〜1.0、虚部 -1.5〜1.5 付近）を表示する

### Requirement 5: カラーマッピング
**Objective:** ユーザーとして、フラクタルの境界部分が視覚的に美しく区別できるカラーリングで表示されることを期待する。

#### Acceptance Criteria
1. The Renderer shall 反復回数に基づくグラデーションカラーマッピングを適用する
2. The Renderer shall 集合内部のピクセルを黒色で描画する
3. The Renderer shall スムーズカラーリング（連続的な色遷移）を実装し、バンディングを軽減する

### Requirement 6: パフォーマンス
**Objective:** ユーザーとして、ズームやパン操作時にストレスなくスムーズにフラクタルが再描画されることを期待する。

#### Acceptance Criteria
1. The Renderer shall Wasm内で計算処理を行い、JavaScriptよりも高速にピクセル計算を実行する
2. The Renderer shall 描画領域のサイズに応じた効率的なメモリ割り当てを行う
3. While 再描画中, the Renderer shall UIスレッドをブロックしない設計とする
