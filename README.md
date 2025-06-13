# N-Sup - 製造効率プラットフォーム

<div align="center">

![N-Sup Logo](https://img.shields.io/badge/N--Sup-製造効率プラットフォーム-blue?style=for-the-badge)

**AIを活用した次世代製造業務管理システム**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.8-green?style=flat-square)](https://leptos.dev/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-purple?style=flat-square&logo=webassembly)](https://webassembly.org/)
[![TailwindCSS](https://img.shields.io/badge/TailwindCSS-3.x-blue?style=flat-square&logo=tailwindcss)](https://tailwindcss.com/)

[デモ](#) | [ドキュメント](#開発ガイド) | [インストール](#セットアップ)

</div>

---

## 🚀 概要

N-Supは、製造現場の効率性を革新するAI駆動の包括的管理プラットフォームです。工具管理から従業員管理、NCプログラム最適化まで、製造業務のあらゆる側面をカバーします。

### ✨ 主要特徴

- 🔧 **スマート工具管理** - リアルタイム在庫追跡と自動アラート
- 👥 **統合従業員管理** - 個別管理と効率化
- 🤖 **AI NCプログラム支援** - 最適化提案と自動コード生成
- 💬 **リアルタイムチャット** - チーム内コミュニケーション
- 📊 **AI工具提案** - 機械学習による改善提案
- 📱 **フルレスポンシブ** - あらゆるデバイスで利用可能

---

## 🏗️ 技術スタック

- **フロントエンド**: [Leptos 0.8](https://leptos.dev/) (Rust Web Framework)
- **コンパイル**: [WebAssembly (WASM)](https://webassembly.org/)
- **ビルドツール**: [Trunk](https://trunkrs.dev/)
- **スタイリング**: [TailwindCSS](https://tailwindcss.com/)
- **アーキテクチャ**: Client-Side Rendering (CSR)

---

## 📋 機能一覧

### 🔧 工具管理 (`/tools`)
- **CRUD操作**: 工具の追加・編集・削除
- **ステータス管理**: 利用可能・使用中・メンテナンス中・故障
- **在庫追跡**: リアルタイム在庫状況の監視
- **メンテナンス管理**: 前回・次回メンテナンス日の追跡

### 👥 従業員管理 (`/employees`)
- **従業員データベース**: 包括的な従業員情報管理
- **ステータス追跡**: 在職・休職中・退職
- **編集機能**: 従業員情報の更新と管理
- **部署・役職管理**: 組織構造の管理

### 📊 NCプログラム管理 (`/nc-programs`)
- **バージョン管理**: NCプログラムのバージョン追跡
- **ステータス管理**: 運用中・テスト中・アーカイブ・廃止
- **詳細情報**: 作成者・更新履歴・説明の管理
- **ファイル管理**: プログラムサイズと形式の追跡

### 🤖 NCプログラム支援 (`/nc-support`)
- **AI最適化提案**: 既存プログラムの改善提案
- **コードテンプレート**: 再利用可能なプログラムテンプレート
- **AIコード生成**: 自然言語からの自動プログラム生成
- **性能分析**: 実行時間・工具寿命・表面品質の改善測定

### 🎯 AI工具提案 (`/ai-suggestions`)
- **予知保全**: 工具摩耗予測システム
- **品質改善**: 自動異常検出
- **プロセス最適化**: 生産スケジュール最適化
- **コスト削減**: エネルギー消費最適化
- **安全性向上**: 作業者安全監視システム

### 💬 チャット機能 (`/chat`)
- **リアルタイムメッセージング**: 即座のコミュニケーション
- **チャットルーム管理**: プロジェクト別ルーム作成
- **メッセージタイプ**: ユーザー・システム・アラートメッセージ
- **チーム管理**: メンバー数と活動状況の追跡

---

## 🛠️ セットアップ

### 前提条件

以下のツールが必要です：

```bash
# Rust toolchain (nightly)
rustup toolchain install nightly --allow-downgrade

# WASM target
rustup target add wasm32-unknown-unknown

# Build tools
cargo install trunk

# Node.js (TailwindCSS用)
npm install
```

### インストール

1. **リポジトリのクローン**
```bash
git clone https://github.com/dsk-code/n-sup.git
cd n-sup
```

2. **依存関係のインストール**
```bash
# TailwindCSS dependencies
npm install

# Rust dependencies (自動)
```

3. **開発サーバーの起動**
```bash
# Development server with hot reload
trunk serve --port 3000 --open
```

4. **TailwindCSS のビルド** (別ターミナル)
```bash
# Watch mode for development
npm run build-css
```

### 本番ビルド

```bash
# Production CSS build
npm run build-css-prod

# Production application build
trunk build --release
```

---

## 🚦 開発ガイド

### 開発コマンド

| コマンド | 説明 |
|---------|------|
| `trunk serve --port 3000 --open` | 開発サーバー起動 (ホットリロード付き) |
| `trunk build --release` | 本番ビルド |
| `npm run build-css` | TailwindCSS開発ビルド (ウォッチモード) |
| `npm run build-css-prod` | TailwindCSS本番ビルド (最小化) |
| `cargo fmt` | コード整形 |
| `cargo clippy` | コード解析 |
| `cargo check` | コンパイルチェック |
| `cargo test` | テスト実行 |

### プロジェクト構造

```
src/
├── lib.rs              # メインアプリ・ルーター設定
├── main.rs             # エントリーポイント
├── components/         # 再利用可能コンポーネント
│   ├── features.rs     # 機能カードセクション
│   ├── cta_section.rs  # CTA セクション
│   └── ui.rs          # UI共通コンポーネント
├── pages/              # ページコンポーネント
│   ├── home.rs                    # ホームページ
│   ├── tool_management.rs         # 工具管理
│   ├── employee_management.rs     # 従業員管理
│   ├── nc_program_management.rs   # NCプログラム管理
│   ├── nc_program_support.rs      # NCプログラム支援
│   ├── ai_tool_suggestions.rs     # AI工具提案
│   ├── chat.rs                    # チャット機能
│   └── not_found.rs               # 404ページ
└── utils.rs            # ユーティリティ関数
```

### ルーティング

| パス | ページ | 説明 |
|------|--------|------|
| `/` | Home | ランディングページ |
| `/tools` | ToolManagement | 工具管理システム |
| `/employees` | EmployeeManagement | 従業員管理システム |
| `/nc-programs` | NcProgramManagement | NCプログラム管理 |
| `/nc-support` | NcProgramSupport | NCプログラム支援 |
| `/ai-suggestions` | AiToolSuggestions | AI工具提案 |
| `/chat` | Chat | チャット機能 |

---

## 🎨 デザインシステム

### カラーパレット
- **プライマリ**: Slate (900-800) グラデーション背景
- **アクセント**: Blue to Purple グラデーション
- **セカンダリ**: Green, Yellow, Cyan, Orange (機能別カラーリング)

### レスポンシブデザイン
- **デスクトップ**: テーブルレイアウト
- **モバイル**: カードレイアウト
- **ブレークポイント**: Tailwind CSS標準

### アニメーション
- **ホバー効果**: Transform scale, color transitions
- **トランジション**: 300ms duration
- **バックドロップ**: Blur effects

---

## 🧪 テスト

```bash
# 全テスト実行
cargo test

# 特定テスト実行
cargo test <test_name>

# WASM テスト実行
wasm-pack test --headless --firefox
```

---

## 📦 デプロイ

### 静的サイトホスティング

1. **ビルド**
```bash
npm run build-css-prod
trunk build --release
```

2. **デプロイ**
`dist/` フォルダーの内容を任意の静的サイトホスティングサービスにアップロード

### 対応プラットフォーム
- Netlify
- Vercel
- GitHub Pages
- AWS S3 + CloudFront
- その他の静的サイトホスティング

---

## 🤝 コントリビューション

プルリクエストやイシューを歓迎します！

1. フォークを作成
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

---

## 📄 ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルを参照してください。

---

## 🌟 謝辞

- [Leptos](https://leptos.dev/) - 素晴らしいRust Web Framework
- [Trunk](https://trunkrs.dev/) - シンプルなWASMビルドツール
- [TailwindCSS](https://tailwindcss.com/) - ユーティリティファーストCSS
- [WebAssembly](https://webassembly.org/) - 高性能Web技術

---

<div align="center">

**🚀 N-Sup で製造業務を次のレベルへ**

Made with ❤️ by the N-Sup Team | Powered by Rust & WebAssembly

</div>