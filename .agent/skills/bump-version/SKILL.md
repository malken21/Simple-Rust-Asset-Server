---
name: Bump Version
description: Bumps the project version in Cargo.toml, updates Cargo.lock, and creates a git tag.
---

# Bump Version Skill

このスキルは、Rustプロジェクトのバージョンをインクリメントし、関連するメタデータとGitタグを更新するための手順を定義する。

## 前提条件

- プロジェクトのルートディレクトリに `Cargo.toml` が存在すること。
- Gitが初期化されており、リモートリポジトリ（origin）が設定されていること。

## 手順

1. **現在のバージョン確認**
   - `Cargo.toml` の `[package]` セクションにある `version` を確認する。
   - 必要に応じて `git tag` を確認し、最新のタグと整合性を取る。

2. **バージョン更新 (Cargo.toml)**
   - `Cargo.toml` の `version` を次の番号（例: パッチバージョンの繰り上げ）に更新する。

3. **Cargo.lock の更新**
   - 以下のコマンドを実行して依存関係を再計算し、`Cargo.lock` を更新する。
     ```bash
     cargo check
     ```

4. **Git 操作**
   - 変更をステージングし、コミットメッセージを設定してコミットする。
     ```bash
     git add Cargo.toml Cargo.lock
     git commit -m "chore: bump version to <VERSION>"
     ```
   - 新しいバージョンタグを作成する。
     ```bash
     git tag v<VERSION>
     ```

5. **リモートへのプッシュ**
   - 変更とタグをリモートリポジトリにプッシュする。
     ```bash
     git push origin main
     git push origin v<VERSION>
     ```

## 注意点

- バージョン番号は Semantic Versioning (SemVer) に従うこと。
- リリース用ワークフロー（GitHub Actions等）がタグによってトリガーされる場合、プッシュ後に動作を確認すること。
