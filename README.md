# Rust Asset Server

VRChat などの用途向けに、ファイルのアップロードと静的配信を行います。(本番環境 非推奨)

## 機能

- **ファイルアップロード**: マルチパート形式でのファイルアップロードを受け付け、ファイル内容のSHA-256ハッシュに基づいて保存します（自動重複排除）。
- **静的ファイル配信**: アップロードされたファイルを HTTP 経由で公開します。
- **CORS 対応**: Unity WebGL や他のオリジンからのアクセスを許可します。

## 必要要件

- Rust (Cargo)

## セットアップと実行

1. リポジトリをクローンまたはダウンロードします。
2. ディレクトリに移動します。

   ```bash
   cd Rust-Asset-Server
   ```

3. サーバーを起動します。

   ```bash
   cargo run
   ```

   初回起動時に依存クレートのダウンロードとコンパイルが行われます。
   サーバーはポート `3000` で待機します。

## API エンドポイント

### 1. ファイルアップロード

**POST** `/upload`

- **Content-Type**: `multipart/form-data`
- **Body**:
  - `file`: アップロードするファイル本体

**レスポンス例**:

```json
{
  "url": "http://localhost:3000/uploads/a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e.png"
}
```

### 2. ファイルアクセス

**GET** `/uploads/<filename>`

アップロードされたファイルにアクセスします。

例: `http://localhost:3000/uploads/a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e.png`

## ディレクトリ構成

サーバー起動時、カレントディレクトリに `public/uploads` フォルダが自動的に作成され、そこにファイルが保存されます。

## 技術スタック

- **言語**: Rust
- **Webフレームワーク**: Axum
- **非同期ランタイム**: Tokio
- **HTTPユーティリティ**: Tower-http (CORS, ServeDir)
