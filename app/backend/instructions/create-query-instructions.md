---
description: 
globs: 
alwaysApply: false
---
# クエリエンドポイント作成手順書

このドキュメントでは、Clean Architectureパターンに従って新しいクエリエンドポイントを作成する手順を説明します。

## アーキテクチャ概要

本プロジェクトは以下の層構造に従っています：

```
src/
├── domain/              # ドメイン層：ビジネスルールと企業ロジック
├── application/         # アプリケーション層：ユースケースとビジネスロジック
├── infrastructure/      # インフラストラクチャ層：外部との接続
└── presentation/        # プレゼンテーション層：HTTPインターフェース
```

## 実装パターン

### シンプルなリスト取得エンドポイント（例：/colors）

- パラメータなし
- リスト形式のデータ返却
- ページネーションなし

### 複雑なリスト取得エンドポイント（例：/products）  

- ページネーション対応
- リッチなデータ構造
- ビジネスロジック含む

### パラメータ付きエンドポイント（例：/variants）

- POSTリクエスト
- リクエストボディでパラメータ受け取り
- 複雑な検索条件対応

## 統一された命名規則

### ファイル名規則

- **DTO**: `{entity}_list_dto.rs`, `{entity}_dto.rs`
- **Repository**: `{entity}_repository.rs`
- **Handler**: `get_{entity}_list_handler.rs`, `find_{entity}s_handler.rs`
- **Controller**: `get_{entity}_list_controller.rs`
- **Presenter**: `get_{entity}_list_presenter.rs`
- **Response**: `get_{entity}_list_response.rs`
- **Request**: `get_{entity}_list_request.rs` (パラメータがある場合)
- **Routes**: `routes.rs` (各エンティティフォルダ内)

### struct名規則

- **DTO**: `{Entity}DTO`, `{Entity}ListDTO`
- **Repository**: `{Entity}Repository`
- **Handler**: `Get{Entity}ListHandler`, `Find{Entity}sHandler`
- **Controller**: `Get{Entity}ListController`
- **Presenter**: `Get{Entity}ListPresenter`
- **Response**: `Get{Entity}ListResponse`, `{Entity}Response`
- **Request**: `Get{Entity}ListRequest` (パラメータがある場合)

### 命名パターンの例

#### 色（Colors）エンティティの場合
```
ファイル名                      → struct名
get_color_list_response.rs     → GetColorListResponse
get_color_list_presenter.rs    → GetColorListPresenter
get_color_list_controller.rs   → GetColorListController
color_list_dto.rs              → ColorListDTO
```

#### カテゴリ（Categories）エンティティの場合
```
ファイル名                         → struct名
get_category_list_response.rs     → GetCategoryListResponse
get_category_list_presenter.rs    → GetCategoryListPresenter
get_category_list_controller.rs   → GetCategoryListController
category_list_dto.rs              → CategoryListDTO
```

## モジュール構造の統一

### エンティティフォルダ構造

各エンティティフォルダは以下の統一された構造に従います：

```
src/presentation/{entities}/
├── controllers/
│   ├── mod.rs
│   └── get_{entity}_list_controller.rs
├── presenters/
│   ├── mod.rs
│   └── get_{entity}_list_presenter.rs
├── responses/
│   ├── mod.rs
│   └── get_{entity}_list_response.rs
├── requests/           # パラメータがある場合のみ
│   ├── mod.rs
│   └── get_{entity}_list_request.rs
├── mod.rs
└── routes.rs
```

### mod.rs構造の統一

#### メインmod.rs（`src/presentation/{entities}/mod.rs`）

```rust
pub mod controllers;
pub mod presenters;
pub mod responses;
pub mod routes;
// requests モジュールはパラメータがある場合のみ追加

pub use presenters::Get{Entity}ListPresenter;
pub use responses::Get{Entity}ListResponse;
pub use routes::routes;
```

#### routes.rs構造の統一

```rust
use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::{entities}::controllers::Get{Entity}ListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(Get{Entity}ListController::routes())
}
```

## ステップバイステップ実装手順

### Step 1: Application層 - DTO作成

#### 1.1 DTOファイル作成

```bash
touch src/application/dto/{entity}_list_dto.rs
```

複数データを取得する場合

```rust
// src/application/dto/{entity}_list_dto.rs

#[derive(Debug, Clone)]
pub struct {Entity}ListDTO {
    pub items: Vec<{Entity}DTO>,
}

#[derive(Debug, Clone)]
pub struct {Entity}DTO {
    pub id: String,
    pub name: String,
    // 必要なフィールドを追加
}

impl {Entity}ListDTO {
    pub fn new(items: Vec<{Entity}DTO>) -> Self {
        Self { items }
    }
}
```

#### 1.2 mod.rsにDTOを追加

```rust
// src/application/dto/mod.rs に追加
mod {entity}_list_dto;

// 必要な型のみを明示的にエクスポート
pub use self::{entity}_list_dto::{Entity}ListDTO, {Entity}DTO};
```

### Step 2: Application層 - Repository trait作成

```bash
touch src/application/repositories/{entity}_repository.rs
```

```rust
// src/application/repositories/{entity}_repository.rs
use crate::application::error::RepositoryError;
use crate::application::dto::{Entity}ListDTO;

#[async_trait::async_trait]
pub trait {Entity}Repository {
    async fn find_all(&self) -> Result<{Entity}ListDTO, RepositoryError>;
    // 必要に応じて他のメソッドを追加
}
```

```rust
// src/application/repositories/mod.rs に追加
mod {entity}_repository;

// traitのみを明示的にエクスポート
pub use {entity}_repository::{Entity}Repository;
```

### Step 3: Application層 - Query Handler作成

```bash
touch src/application/queries/handlers/get_{entity}_list_handler.rs
```

```rust
// src/application/queries/handlers/get_{entity}_list_handler.rs
use std::sync::Arc;
use crate::application::repositories::{Entity}Repository;
use crate::application::error::ApplicationError;
use crate::application::dto::{Entity}ListDTO;

/// {Entity}リスト取得クエリハンドラ
pub struct Get{Entity}ListHandler {
    {entity}_repository: Arc<dyn {Entity}Repository + Send + Sync>,
}

impl Get{Entity}ListHandler {
    pub fn new({entity}_repository: Arc<dyn {Entity}Repository + Send + Sync>) -> Self {
        Self { {entity}_repository }
    }

    /// クエリを実行
    pub async fn handle(&self) -> Result<{Entity}ListDTO, ApplicationError> {
        println!("->> Get{Entity}ListHandler::handle");
        
        let result = self.{entity}_repository.find_all().await?;
        
        Ok(result)
    }
}
```

```rust
// src/application/queries/handlers/mod.rs に追加
pub mod get_{entity}_list_handler;

// Handlerのみを明示的にエクスポート
pub use get_{entity}_list_handler::Get{Entity}ListHandler;
```

### Step 4: Application層 - Dispatcher更新

```rust
// src/application/dispatcher.rs を更新
use crate::application::queries::handlers::Get{Entity}ListHandler;
use crate::application::dto::{Entity}ListDTO;

pub struct Dispatcher {
    // 既存のハンドラ...
    get_{entity}_list_handler: Arc<Get{Entity}ListHandler>,
}

impl Dispatcher {
    pub fn new(
        // 既存のパラメータ...
        get_{entity}_list_handler: Arc<Get{Entity}ListHandler>,
    ) -> Self {
        Self {
            // 既存のフィールド...
            get_{entity}_list_handler,
        }
    }

    /// 新しいクエリ実行メソッドを追加
    pub async fn execute_get_{entity}_list_query(&self) -> Result<{Entity}ListDTO, ApplicationError> {
        self.get_{entity}_list_handler.handle().await
    }
}
```

### Step 5: Infrastructure層 - Repository実装

```bash
touch src/infrastructure/database/repositories_impl/sqlite_{entity}_repository.rs
```

```rust
// src/infrastructure/database/repositories_impl/sqlite_{entity}_repository.rs
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::repositories::{Entity}Repository;
use crate::application::error::RepositoryError;
use crate::application::dto::{Entity}ListDTO, {Entity}DTO};

/// SQLite実装の{Entity}Repository
/// Clean Architecture: Infrastructure層
pub struct Sqlite{Entity}Repository {
    pool: SqlitePool,
}

impl Sqlite{Entity}Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl {Entity}Repository for Sqlite{Entity}Repository {
    async fn find_all(&self) -> Result<{Entity}ListDTO, RepositoryError> {
        // 実際のSQL実装（例）
        let rows = sqlx::query("SELECT id, name FROM {entities} ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let items: Vec<{Entity}DTO> = rows
            .into_iter()
            .map(|row| {Entity}DTO {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect();

        Ok({Entity}ListDTO::new(items))
    }
}
```

```rust
// src/infrastructure/database/repositories_impl/mod.rs に追加
mod sqlite_{entity}_repository;

// 実装クラスのみを明示的にエクスポート
pub use sqlite_{entity}_repository::Sqlite{Entity}Repository;
```

### Step 6: Infrastructure層 - DI Container更新

```rust
// src/infrastructure/di/container.rs を更新
use crate::infrastructure::database::repositories_impl::Sqlite{Entity}Repository;
use crate::application::repositories::{Entity}Repository;
use crate::application::queries::handlers::Get{Entity}ListHandler;

pub struct Container {
    // 既存のリポジトリ...
    pub {entity}_repository: Arc<dyn {Entity}Repository + Send + Sync>,
    // dispatcher は既存
}

impl Container {
    /// プールを指定してコンテナを作成します
    async fn new_with_pool(pool: sqlx::SqlitePool) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // 既存のリポジトリ...
        let {entity}_repository = Arc::new(Sqlite{Entity}Repository::new(pool.clone()));
        
        // 既存のハンドラ...
        let get_{entity}_list_handler = Arc::new(Get{Entity}ListHandler::new({entity}_repository.clone()));
        
        // ディスパッチャを更新
        let dispatcher = Arc::new(Dispatcher::new(
            // 既存のハンドラ引数...
            get_{entity}_list_handler,
        ));
        
        Ok(Self {
            // 既存のフィールド...
            {entity}_repository,
            dispatcher,
        })
    }
}
```

### Step 7: Presentation層 - Response構造作成

```bash
mkdir -p src/presentation/{entities}/responses
touch src/presentation/{entities}/responses/get_{entity}_list_response.rs
```

```rust
// src/presentation/{entities}/responses/get_{entity}_list_response.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Get{Entity}ListResponse {
    pub items: Vec<{Entity}ListItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {Entity}ListItemResponse {
    pub id: String,
    #[schema(nullable = false)]
    pub name: Option<String>,
    // フロントエンド要件に合わせたフィールド
}

impl Get{Entity}ListResponse {
    pub fn new(items: Vec<{Entity}ListItemResponse>) -> Self {
        Self { items }
    }
}
```

```rust
// src/presentation/{entities}/responses/mod.rs
mod get_{entity}_list_response;

// Response型のみを明示的にエクスポート
pub use get_{entity}_list_response::{Get{Entity}ListResponse, {Entity}ListItemResponse};
```

### Step 8: Presentation層 - Presenter作成

```bash
mkdir -p src/presentation/{entities}/presenters
touch src/presentation/{entities}/presenters/get_{entity}_list_presenter.rs
```

```rust
// src/presentation/{entities}/presenters/get_{entity}_list_presenter.rs
use crate::application::dto::{Entity}ListDTO;
use crate::presentation::{entities}::responses::{Get{Entity}ListResponse, {Entity}ListItemResponse};

/// {Entity}リストプレゼンター
/// Clean Architecture: Interface Adapters層
/// アプリケーション層のDTOをHTTPレスポンス用DTOに変換する
pub struct Get{Entity}ListPresenter;

impl Get{Entity}ListPresenter {
    /// DTOをResponseに変換
    pub fn present(dto: {Entity}ListDTO) -> Get{Entity}ListResponse {
        let items = dto
            .items
            .into_iter()
            .map(|item| {Entity}ListItemResponse {
                id: item.id,
                name: Some(item.name),
            })
            .collect();

        Get{Entity}ListResponse::new(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{Entity}ListDTO, {Entity}DTO};

    #[test]
    fn test_present() {
        let dto = {Entity}ListDTO::new(vec![
            {Entity}DTO {
                id: "1".to_string(),
                name: "Test Item".to_string(),
            },
        ]);

        let response = Get{Entity}ListPresenter::present(dto);
        
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "1");
        assert_eq!(response.items[0].name, Some("Test Item".to_string()));
    }
}
```

```rust
// src/presentation/{entities}/presenters/mod.rs
mod get_{entity}_list_presenter;

// Presenterのみを明示的にエクスポート
pub use get_{entity}_list_presenter::Get{Entity}ListPresenter;
```

### Step 9: Presentation層 - Controller作成

```bash
mkdir -p src/presentation/{entities}/controllers
touch src/presentation/{entities}/controllers/get_{entity}_list_controller.rs
```

```rust
// src/presentation/{entities}/controllers/get_{entity}_list_controller.rs
use axum::extract::State;
use axum::{Json, Router, routing::get};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::{entities}::presenters::Get{Entity}ListPresenter;
use crate::presentation::{entities}::responses::Get{Entity}ListResponse;
use crate::presentation::ErrorResponse;

/// {Entity} List Controller
pub struct Get{Entity}ListController;

impl Get{Entity}ListController {
    /// ルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/{entities}", get(handle))
    }
}

/// GET /{entities} - {Entity}リスト取得処理
#[utoipa::path(
    get,
    path = "/{entities}",
    operation_id = "get_{entity}_list",
    responses(
        (status = 200, description = "{Entity}リスト取得成功", body = Get{Entity}ListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "{Entities}"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<Get{Entity}ListResponse>> {
    println!("->> Get{Entity}ListController::handle");

    let dispatcher = container.get_dispatcher();
    let result = dispatcher.execute_get_{entity}_list_query().await?;

    println!("->> Get{Entity}ListController::handle - success");

    Ok(Json(Get{Entity}ListPresenter::present(result)))
}
```

```rust
// src/presentation/{entities}/controllers/mod.rs
pub mod get_{entity}_list_controller;

// Controllerクラスのみを明示的にエクスポート
pub use get_{entity}_list_controller::Get{Entity}ListController;
```

### Step 10: Presentation層 - モジュール統合とRoutes作成

```bash
touch src/presentation/{entities}/routes.rs
touch src/presentation/{entities}/mod.rs
```

#### routes.rs（統一構造）

```rust
// src/presentation/{entities}/routes.rs
use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::{entities}::controllers::Get{Entity}ListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(Get{Entity}ListController::routes())
}
```

#### mod.rs（統一構造）

```rust
// src/presentation/{entities}/mod.rs
pub mod controllers;
pub mod presenters;
pub mod responses;
pub mod routes;

pub use presenters::Get{Entity}ListPresenter;
pub use responses::Get{Entity}ListResponse;
pub use routes::routes;
```

### Step 11: ルートの登録

```rust
// src/presentation/routes.rs を更新
use crate::presentation::{entities};

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        // 既存のルート...
        .merge({entities}::routes())
        // その他のルート...
}
```

```rust
// src/presentation/mod.rs を更新（必要に応じて）
mod {entities};
```

### Step 12: OpenAPI/Swagger設定更新

#### 12.1 Response型のインポート追加

```rust
// src/presentation/swagger/openapi.rs のuse文に追加
use crate::presentation::{entities}::responses::{
    Get{Entity}ListResponse,
    {Entity}ListItemResponse,
};
```

#### 12.2 パス（エンドポイント）の追加

```rust
// src/presentation/swagger/openapi.rs の #[openapi] マクロ内のpaths()セクションに追加
#[openapi(
    paths(
        // 既存のパス...
        crate::presentation::categories::controllers::get_category_list_controller::handle,
        crate::presentation::colors::controllers::get_color_list_controller::handle,
        crate::presentation::variants::controllers::find_variants_controller::handle,
        // 新しいパスを追加
        crate::presentation::{entities}::controllers::get_{entity}_list_controller::handle,
    ),
    // ...
)]
```

#### 12.3 スキーマコンポーネントの追加

```rust
// src/presentation/swagger/openapi.rs の components()セクションに追加
components(
    schemas(
        // 既存のスキーマ...
        GetCategoryListResponse,
        GetColorListResponse,
        // 新しいスキーマを追加
        Get{Entity}ListResponse,
        {Entity}ListItemResponse,
    )
),
```

#### 12.4 タグの追加

```rust
// src/presentation/swagger/openapi.rs のtags()セクションに追加
tags(
    // 既存のタグ...
    (name = "Categories", description = "カテゴリ関連のAPI"),
    (name = "Colors", description = "色関連のAPI"),
    (name = "Variants", description = "バリアント関連のAPI"),
    // 新しいタグを追加
    (name = "{Entities}", description = "{Entity}関連のAPI"),
),
```

## 実装済みエンドポイント例（統一後）

### シンプルなリスト取得

#### Colors実装例
```
src/presentation/colors/
├── controllers/
│   ├── mod.rs
│   └── get_color_list_controller.rs    → GetColorListController
├── presenters/
│   ├── mod.rs
│   └── get_color_list_presenter.rs     → GetColorListPresenter
├── responses/
│   ├── mod.rs
│   └── get_color_list_response.rs      → GetColorListResponse
├── mod.rs
└── routes.rs
```

#### Categories実装例
```
src/presentation/categories/
├── controllers/
│   ├── mod.rs
│   └── get_category_list_controller.rs → GetCategoryListController
├── presenters/
│   ├── mod.rs
│   └── get_category_list_presenter.rs  → GetCategoryListPresenter
├── responses/
│   ├── mod.rs
│   └── get_category_list_response.rs   → GetCategoryListResponse
├── mod.rs
└── routes.rs
```

### 複雑なリスト取得

#### Products実装例
```
src/presentation/products/
├── controllers/
│   ├── mod.rs
│   ├── get_product_controller.rs       → GetProductController
│   └── get_product_list_controller.rs  → GetProductListController
├── presenters/
│   ├── mod.rs
│   ├── get_product_presenter.rs        → GetProductPresenter
│   └── get_product_list_presenter.rs   → GetProductListPresenter
├── responses/
│   ├── mod.rs
│   ├── get_product_response.rs         → GetProductResponse
│   └── get_product_list_response.rs    → GetProductListResponse
├── mod.rs
└── routes.rs
```

### パラメータ付きエンドポイント

#### Variants実装例
```
src/presentation/variants/
├── controllers/
│   ├── mod.rs
│   └── find_variants_controller.rs     → FindVariantsController
├── presenters/
│   ├── mod.rs
│   └── find_variants_presenter.rs      → FindVariantsPresenter
├── requests/
│   ├── mod.rs
│   └── find_variants_request.rs        → FindVariantsRequest
├── responses/
│   ├── mod.rs
│   └── find_variants_response.rs       → FindVariantsResponse
├── mod.rs
└── routes.rs
```

## チェックリスト

実装完了後、以下をチェックしてください：

### 命名規則チェック

- [ ] ファイル名が統一された規則に従っている
- [ ] struct名が統一された規則に従っている
- [ ] エンドポイントパスが適切に設定されている
- [ ] OpenAPIタグが適切に設定されている

### モジュール構造チェック

- [ ] routes.rsが適切に作成されている
- [ ] mod.rsが統一された構造になっている
- [ ] 各mod.rsで必要最小限のもののみがpubになっている
- [ ] ワイルドカード（`*`）を使用していない

### コンパイルチェック

- [ ] `cd app/backend && cargo check` でコンパイルエラーなし
- [ ] `cd app/backend && cargo clippy` で警告なし
- [ ] `cd app/backend && cargo fmt` でフォーマット済み

### 機能チェック

- [ ] `cd app/backend && cargo run` でサーバー起動
- [ ] `curl http://localhost:4000/{entities}` でレスポンス確認
- [ ] Swagger UI (`http://localhost:4000/swagger-ui/`) で仕様確認

### テストチェック

- [ ] `cd app/backend && cargo test` でテスト通過
- [ ] Presenterのユニットテスト実装
- [ ] 必要に応じてRepositoryのテスト実装

## mod.rs設計原則

### Clean Architectureでのモジュール公開ルール

1. **最小権限の原則**: 必要最小限のもののみをpubにする
2. **明示的エクスポート**: `*`によるワイルドカード使用を避ける
3. **層間インターフェースの限定**: 各層のAPIを明確に定義する
4. **内部実装の隠蔽**: 実装の詳細は外部に公開しない
5. **統一された構造**: すべてのエンティティで同じmod.rs構造を使用する

### 良い例と悪い例

**❌ 悪い実装:**
```rust
// 何でもかんでもpubにする
pub mod internal_module;
pub use internal_module::*;  // 全部エクスポート

// 古い命名規則
pub use responses::ColorListResponse;  // 非統一
pub use presenters::CategoryListPresenter;  // 非統一
```

**✅ 良い実装:**
```rust
// 必要なもののみを公開
mod internal_module;  // 内部モジュールはpubにしない
pub use internal_module::PublicInterface;  // 必要な部分のみ

// 統一された命名規則
pub use responses::GetColorListResponse;     // 統一
pub use presenters::GetCategoryListPresenter; // 統一
```

## トラブルシューティング

### よくあるエラー

1. **命名規則の不一致エラー**
   - ファイル名とstruct名の不一致
   - 新しい命名規則への移行漏れ
   - OpenAPI設定での型名ミス

2. **モジュール構造エラー**
   - routes.rsの作成漏れ
   - mod.rsでの統一構造への移行漏れ
   - 不適切なpub指定

3. **コンパイルエラー: "trait ... is not implemented"**
   - `use` 文の追加漏れ
   - `#[async_trait]` の追加漏れ

4. **ランタイムエラー: "method not found"**
   - Dispatcherにメソッド追加漏れ
   - DIコンテナにハンドラ登録漏れ

5. **HTTP 404エラー**
   - ルート登録漏れ
   - パス指定ミス

### デバッグのコツ

1. 既存の統一された実装（colors、categories、variants）を参考にする
2. `println!` でログを仕込む
3. `cd app/backend && cargo run` で直接エラーメッセージを確認
4. 各層を個別にテストする
5. `bacon` コマンドで継続的なビルドチェック

## まとめ

このガイドに従って実装することで：

- **統一された命名規則**でプロジェクト全体の一貫性が保たれる
- **Clean Architecture**の原則が遵守される
- **保守性と可読性**が向上する
- **新しい開発者**がコードベースを理解しやすくなる
- **テスト**が容易になる

新しくエンドポイントを作成する際は、colors、categories、variantsの統一された実装を参考にしてください。 