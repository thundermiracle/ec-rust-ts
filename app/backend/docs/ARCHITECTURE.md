# Architecture Documentation

## プロジェクト概要

このプロジェクトは **Uncle Bob's Clean Architecture** に完全準拠したRust/Axumアプリケーションです。
4層構造による明確な責任分離と依存関係の管理を実現しています。

## Clean Architecture とは

Uncle Bob (Robert C. Martin) が提唱した Clean Architecture は、以下の原則に基づいています：

1. **依存関係の方向**: 内側に向かって依存する
2. **フレームワーク独立**: ビジネスロジックがフレームワークに依存しない
3. **テスタブル**: UIやデータベースなしでビジネスロジックをテスト可能
4. **UI独立**: UIの変更がビジネスロジックに影響しない
5. **データベース独立**: データベースの変更がビジネスロジックに影響しない

## フォルダ構成

```
src/
├── domain/                          # Entities (Enterprise Business Rules)
│   ├── models/                      # ドメインエンティティ
│   └── error.rs                     # ドメインエラー定義
├── application/                     # Use Cases (Application Business Rules)
│   ├── use_cases/                   # ユースケース実装
│   ├── repositories/                # リポジトリインターフェース
│   ├── commands/                    # コマンドオブジェクト
│   ├── queries/                     # クエリオブジェクト
│   └── error.rs                     # アプリケーションエラー定義
├── interface_adapters/              # Interface Adapters
│   └── products/                    # Resource-based grouping
│       ├── controllers/             # HTTPリクエストハンドラー
│       ├── requests/                # リクエストDTO
│       └── presenters/              # レスポンスフォーマッター
├── frameworks_and_drivers/          # Frameworks & Drivers (最外層)
│   ├── database/                    # データベース接続・マイグレーション
│   ├── persistence/                 # データ永続化実装
│   │   ├── entities/               # データベースエンティティ
│   │   └── repositories_impl/      # リポジトリ実装
│   └── di/                         # 依存性注入
├── error.rs                         # グローバルエラーハンドリング
└── main.rs                          # アプリケーションエントリーポイント
```

## 各層の詳細

### 1. Domain Layer (最内層)

**責任**: エンタープライズビジネスルールの実装

- **Entity**: ビジネスエンティティとその振る舞い
- **Domain Error**: ドメイン固有のエラー
- **依存関係**: なし（完全独立）

```rust
// 例: Product エンティティ
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl Product {
    pub fn sell(&mut self, quantity: u32) -> Result<(), DomainError> {
        // ビジネスロジック
    }
}
```

### 2. Application Layer

**責任**: アプリケーションビジネスルールの実装、ユースケースの調整

- **Use Cases**: 特定のユースケースの実装
- **Repository Interfaces**: データアクセスの抽象化
- **Commands/Queries**: CQRS パターンの実装
- **Application Error**: アプリケーション固有のエラー
- **依存関係**: Domain層のみ

```rust
// 例: Get Product Use Case
pub struct GetProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductUseCase {
    pub async fn get_by_id(&self, id: u32) -> Result<GetProductQuery, ApplicationError> {
        // ユースケースロジック
    }
}
```

### 3. Interface Adapters Layer

**責任**: 外部とのインターフェース、データ変換

- **Controllers**: HTTPリクエストの受付とルーティング
- **Presenters**: レスポンスの整形とフォーマット
- **Request DTOs**: 入力データの受け取り
- **依存関係**: Application層

#### Resource-based Grouping

関連するController、Request、Presenterを同じディレクトリにグループ化：

```
products/
├── controllers/     # 商品関連のコントローラー
├── requests/        # 商品関連のリクエストDTO
└── presenters/      # 商品関連のプレゼンター
```

**メリット**:
- 高凝集：関連性の高いコンポーネントが近い場所
- 発見性：使用関係が明確
- 単一責任：1つのController = 1つのユースケース

### 4. Frameworks & Drivers Layer (最外層)

**責任**: 技術的詳細の実装

- **Database**: データベース接続と設定
- **Persistence**: データ永続化の実装
- **DI Container**: 依存性注入の設定
- **依存関係**: 内側の全ての層にアクセス可能

## 依存関係の流れ

```
Frameworks & Drivers → Interface Adapters → Application → Domain
```

### 依存関係の原則

1. **内側への依存**: 外側の層は内側の層に依存可能
2. **外側への非依存**: 内側の層は外側の層に依存しない
3. **抽象化への依存**: 具象ではなく抽象（トレイト）に依存

## エラーハンドリング

各層に適切なエラータイプを定義し、層間でエラーをマッピング：

```rust
// Domain層
pub enum DomainError {
    InsufficientQuantity { requested: u32, available: u32 },
    InvalidProductData(String),
}

// Application層
pub enum ApplicationError {
    Domain(DomainError),           // ドメインエラーのラップ
    Repository(RepositoryError),   // リポジトリエラーのラップ
    ProductNotFound(u32),          // アプリケーション固有エラー
    Validation(String),
}

// Repository層
pub enum RepositoryError {
    DatabaseConnection(String),
    QueryExecution(String),
    NotFound,
    Unknown(String),
}
```

## 命名規則

### Uncle Bob準拠の命名

- **Use Cases**: `{Action}UseCase` (例: `GetProductUseCase`)
- **Controllers**: `{Action}Controller` (例: `GetProductController`)
- **Repositories**: `{Entity}Repository` (例: `ProductRepository`)
- **Entities**: `{Entity}` (例: `Product`)

### ファイル命名

- **Use Cases**: `{action}_{entity}_use_case.rs`
- **Controllers**: `{action}_{entity}_controller.rs`
- **snake_case**: Rustの標準に従う

## 開発ガイドライン

### 新機能追加時の手順

1. **Domain層**: 必要に応じてエンティティやドメインサービスを追加
2. **Application層**: ユースケースとリポジトリインターフェースを定義
3. **Interface Adapters層**: Controller、Request、Presenterを実装
4. **Frameworks & Drivers層**: リポジトリ実装やDI設定を追加

### テスト戦略

- **Domain層**: 純粋なユニットテスト
- **Application層**: モックリポジトリを使ったユニットテスト
- **Interface Adapters層**: 統合テスト
- **Frameworks & Drivers層**: インフラストラクチャテスト

### 設計原則

1. **Single Responsibility Principle**: 1つのクラスは1つの責任
2. **Open/Closed Principle**: 拡張に開き、修正に閉じる
3. **Liskov Substitution Principle**: 基底型を派生型で置換可能
4. **Interface Segregation Principle**: 使わないメソッドに依存しない
5. **Dependency Inversion Principle**: 抽象に依存し、具象に依存しない

## 技術スタック

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: SQLite
- **Async Runtime**: Tokio
- **Architecture**: Uncle Bob's Clean Architecture

## まとめ

このプロジェクトは Clean Architecture の原則に完全準拠し、以下の特徴を持ちます：

- **保守性**: 変更の影響範囲が限定的
- **テスタブル性**: 各層が独立してテスト可能
- **拡張性**: 新機能の追加が容易
- **理解しやすさ**: 明確な責任分離
- **フレームワーク独立**: ビジネスロジックがフレームワークに依存しない

この構造により、長期的に保守可能で拡張性の高いアプリケーションを実現しています。
