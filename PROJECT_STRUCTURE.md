# N-Sup プロジェクト構成とページ遷移

## プロジェクト概要
N-Supは製造業向けの効率化プラットフォームです。工具管理、従業員管理、NCプログラム管理などの機能を提供するLeptos CSRアプリケーションです。

## 技術スタック
- **フレームワーク**: Leptos 0.8 (Client-Side Rendering)
- **言語**: Rust
- **ビルドツール**: Trunk
- **スタイル**: TailwindCSS
- **ターゲット**: WebAssembly (WASM)

## ディレクトリ構成

```
src/
├── lib.rs                  # アプリケーションのルートコンポーネント、ルーティング設定
├── main.rs                 # エントリーポイント
├── components/             # 再利用可能なUIコンポーネント
│   ├── mod.rs             # コンポーネントモジュール定義
│   ├── benefits.rs        # ベネフィットセクション
│   ├── counter_btn.rs     # カウンターボタン（未使用）
│   ├── cta_section.rs     # CTA（Call To Action）セクション
│   ├── features.rs        # 機能紹介セクション
│   ├── footer.rs          # フッター
│   ├── header.rs          # ヘッダー
│   ├── hero_section.rs    # ヒーローセクション
│   ├── navigation.rs      # ナビゲーションコンポーネント
│   └── ui.rs              # 基本UIコンポーネント
├── pages/                  # ページコンポーネント
│   ├── mod.rs             # ページモジュール定義
│   ├── home.rs            # ホームページ（ランディングページ）
│   ├── not_found.rs       # 404エラーページ
│   ├── tool_management.rs # 工具管理ページ
│   └── employee_management.rs # 従業員管理ページ
└── utils/                  # ユーティリティ関数
    └── ...                # アニメーション、3D効果等
```

## ページ構成とルーティング

### 1. ホームページ (`/`)
- **ファイル**: `src/pages/home.rs`
- **説明**: ランディングページ、会社紹介、機能説明
- **コンポーネント構成**:
  - Header: ナビゲーションバー
  - HeroSection: メインビジュアル
  - FeaturesSection: 機能紹介（工具管理、従業員管理など）
  - BenefitsSection: 導入メリット
  - CtaSection: 行動喚起（管理画面へのリンク）
  - Footer: フッター情報

### 2. 工具管理ページ (`/tools`)
- **ファイル**: `src/pages/tool_management.rs`
- **説明**: 工具の在庫管理、追跡、メンテナンス管理
- **機能**:
  - 工具一覧表示
  - 新規工具追加
  - 工具削除
  - ステータス管理（利用可能、使用中、メンテナンス中、故障）
  - 保管場所管理
  - メンテナンス日程管理

### 3. 従業員管理ページ (`/employees`)
- **ファイル**: `src/pages/employee_management.rs`
- **説明**: 従業員情報の管理、所属部署、役職管理
- **機能**:
  - 従業員一覧表示
  - 新規従業員追加
  - 従業員情報編集
  - 従業員削除
  - ステータス管理（在職、休職中、退職）
  - 部署・役職管理
  - 連絡先管理

### 4. 404エラーページ (`/not-found`)
- **ファイル**: `src/pages/not_found.rs`
- **説明**: 存在しないページアクセス時の表示

## ページ遷移フロー

```
ホームページ (/)
    ↓
    ├── 機能カード「工具管理」クリック → 工具管理ページ (/tools)
    ├── 機能カード「従業員別管理」クリック → 従業員管理ページ (/employees)
    ├── CTAボタン「工具管理」クリック → 工具管理ページ (/tools)
    └── CTAボタン「従業員管理」クリック → 従業員管理ページ (/employees)

工具管理ページ (/tools)
    ├── 「ホームに戻る」リンク → ホームページ (/)
    ├── 新規工具追加モーダル
    └── 工具削除機能

従業員管理ページ (/employees)
    ├── 「ホームに戻る」リンク → ホームページ (/)
    ├── 新規従業員追加モーダル
    ├── 従業員編集モーダル
    └── 従業員削除機能
```

## ナビゲーションコンポーネント

### BackToHome
- **用途**: 管理ページから ホームページへの戻りリンク
- **配置**: 各管理ページの上部
- **スタイル**: 矢印アイコン付きリンク

### FeatureCard Links
- **用途**: ホームページの機能カードから各管理ページへのリンク
- **対象機能**:
  - 「工具管理」→ `/tools`
  - 「従業員別管理」→ `/employees`

### CTA Section Links
- **用途**: ホームページのCTAセクションから各管理ページへのリンク
- **ボタン**:
  - 「工具管理」→ `/tools`
  - 「従業員管理」→ `/employees`

## データ構造

### Tool (工具)
```rust
pub struct Tool {
    pub id: u32,                    // 工具ID
    pub name: String,               // 工具名
    pub tool_type: String,          // 工具種類
    pub status: ToolStatus,         // ステータス
    pub location: String,           // 保管場所
    pub last_maintenance: String,   // 前回メンテナンス日
    pub next_maintenance: String,   // 次回メンテナンス日
}

pub enum ToolStatus {
    Available,      // 利用可能
    InUse,         // 使用中
    Maintenance,   // メンテナンス中
    Damaged,       // 故障
}
```

### Employee (従業員)
```rust
pub struct Employee {
    pub id: u32,                    // 従業員ID
    pub name: String,               // 氏名
    pub employee_id: String,        // 従業員番号
    pub department: String,         // 部署
    pub position: String,           // 役職
    pub email: String,              // メールアドレス
    pub phone: String,              // 電話番号
    pub hire_date: String,          // 入社日
    pub status: EmployeeStatus,     // ステータス
}

pub enum EmployeeStatus {
    Active,     // 在職
    OnLeave,    // 休職中
    Inactive,   // 退職
}
```

## 開発コマンド

### 開発サーバー起動
```bash
trunk serve --port 3000 --open
```

### ビルド
```bash
trunk build --release
```

### CSS処理
```bash
npm run build-css        # 開発用
npm run build-css-prod   # 本番用
```

### コード品質チェック
```bash
cargo check     # コンパイルチェック
cargo clippy    # リンター
cargo fmt       # フォーマッター
```

## 今後の拡張計画

1. **NCプログラム管理機能** (`/nc-programs`)
2. **チャット機能** (`/chat`)
3. **AI工具提案機能**
4. **ダッシュボード** (`/dashboard`)
5. **ユーザー認証・権限管理**
6. **レポート機能**
7. **バックアップ・復元機能**

## アクセス方法

- **ホームページ**: http://localhost:3000/
- **工具管理**: http://localhost:3000/tools
- **従業員管理**: http://localhost:3000/employees