# GitHub Pages デプロイメントガイド

このドキュメントは、N-SupをGitHub Pagesに自動デプロイする方法を説明します。

## 🚀 GitHub Pages設定

### 1. リポジトリ設定
1. GitHubリポジトリの「Settings」タブに移動
2. 左サイドバーの「Pages」を選択
3. 「Source」で「GitHub Actions」を選択

### 2. 自動デプロイメント
- `main`ブランチにプッシュすると自動的にデプロイされます
- GitHub Actionsワークフロー（`.github/workflows/deploy.yml`）が実行されます
- ビルド完了後、サイトがhttps://dsk-code.github.io/n-sup/ で公開されます

## 🛠️ ビルドプロセス

### 依存関係
- **Node.js 18**: TailwindCSSのビルド
- **Rust stable**: WebAssemblyコンパイル
- **Trunk**: Leptos/Wasmビルドツール

### ビルドステップ
1. Node.js依存関係のインストール
2. Rust toolchainとwasm32ターゲットのセットアップ
3. TailwindCSSの本番ビルド (`npm run build-css-prod`)
4. Trunkでのリリースビルド (`trunk build --release --public-url /n-sup/`)
5. 生成されたファイルをGitHub Pagesにデプロイ

## 📁 ファイル構成

### 設定ファイル
- `.github/workflows/deploy.yml`: GitHub Actionsワークフロー
- `public/404.html`: SPAルーティング用404ハンドラー
- `index.html`: SPAサポートスクリプトを含むメインHTML
- `Trunk.toml`: Trunkビルド設定

### 出力ファイル
- `dist/`: ビルド成果物（GitHub Pagesで配信）
- `dist/n_sup.js`: コンパイル済みWebAssembly
- `dist/tailwind.css`: 最適化されたCSS
- `dist/index.html`: 最終HTML

## 🔧 ローカル本番ビルド

```bash
# TailwindCSS本番ビルド
npm run build-css-prod

# Trunk本番ビルド
trunk build --release

# ローカル確認
cd dist && python -m http.server 8000
```

## 🌐 URL構造

- **ホーム**: https://dsk-code.github.io/n-sup/
- **ダッシュボード**: https://dsk-code.github.io/n-sup/dashboard
- **工具管理**: https://dsk-code.github.io/n-sup/tools
- **従業員管理**: https://dsk-code.github.io/n-sup/employees
- **NCプログラム**: https://dsk-code.github.io/n-sup/nc-programs
- **AI支援**: https://dsk-code.github.io/n-sup/nc-support
- **AI提案**: https://dsk-code.github.io/n-sup/ai-suggestions
- **チャット**: https://dsk-code.github.io/n-sup/chat

## 📊 デプロイメント監視

### GitHub Actionsタブで確認できる項目
- ビルド実行時間
- 依存関係キャッシュ状況
- TailwindCSS最適化結果
- WebAssemblyコンパイル状況
- デプロイメント成功/失敗

### デバッグ
ビルド失敗時は、GitHub Actionsのログを確認してください：
1. リポジトリの「Actions」タブ
2. 失敗したワークフロー実行をクリック
3. 「build」ジョブのログを展開して詳細確認

## 🔒 セキュリティ

- GitHub Pages設定でHTTPS強制を有効化推奨
- Dependabotによる依存関係の自動更新設定
- セキュリティアラートの監視

## 📈 パフォーマンス最適化

### 自動最適化
- WebAssemblyの最小化（`data-wasm-opt="z"`）
- TailwindCSSの未使用クラス削除
- 静的ファイルの圧縮
- ブラウザキャッシュ最適化

### 手動最適化オプション
```bash
# より積極的な最適化
trunk build --release --minify

# ファイルサイズ確認
ls -la dist/
```

## 🚀 継続的デプロイメント

mainブランチへのマージで自動デプロイされるため：
1. プルリクエストでの変更確認
2. マージ後、数分でライブサイト更新
3. デプロイメント通知（Actionsページで確認可能）

---

**Note**: 初回デプロイ時は、GitHub Pagesの設定でSourceを「GitHub Actions」に変更する必要があります。