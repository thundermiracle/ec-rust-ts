# コマンドエンドポイント作成手順書

このドキュメントでは、Clean Architectureパターンに従って新しいコマンドエンドポイント（POST操作）を作成する手順を説明します。

## アーキテクチャ概要

本プロジェクトは以下の層構造に従っています：

```
src/
├── domain/              # ドメイン層：ビジネスルールと企業ロジック
├── application/         # アプリケーション層：ユースケースとビジネスロジック
├── infrastructure/      # インフラストラクチャ層：外部との接続
└── presentation/        # プレゼンテーション層：HTTPインターフェース
```

## コマンドとクエリの違い

### コマンド（Command）
- **目的**: 状態を変更する操作（Create, Update, Delete）
- **操作**: POST, PUT, PATCH, DELETE HTTP メソッド
- **戻り値**: 最小限の情報（成功/失敗、作成されたリソースなど）
- **例**: カート計算、商品購入、在庫更新

### クエリ（Query）
- **目的**: データを取得する操作（Read）
- **操作**: GET HTTP メソッド
- **戻り値**: 要求されたデータ
- **例**: 商品リスト取得、カテゴリ一覧取得

## 実装パターン（カート計算の例）

### シンプルなコマンドエンドポイント（例：/cart）

- POSTリクエスト
- リクエストボディでデータ受け取り
- バリデーション含む
- ビジネスロジック実行
- 計算結果を返却

## 統一された命名規則

### ファイル名規則

- **Command Model**: `{action}_{entity}_command.rs`
- **Command Handler**: `{action}_{entity}_handler.rs`
- **Controller**: `{action}_{entity}_controller.rs`
- **Presenter**: `{action}_{entity}_presenter.rs`
- **Request**: `{action}_{entity}_request.rs`
- **Response**: `{action}_{entity}_response.rs`
- **Routes**: `routes.rs` (各エンティティフォルダ内)

### struct名規則

- **Command Model**: `{Action}{Entity}Command`
- **Command Handler**: `{Action}{Entity}Handler`
- **Controller**: `{Action}{Entity}Controller`
- **Presenter**: `{Action}{Entity}Presenter`
- **Request**: `{Action}{Entity}Request`
- **Response**: `{Action}{Entity}Response`

### 命名パターンの例

#### カート計算（Calculate Cart）の場合
```
ファイル名                         → struct名
calculate_cart_command.rs         → CalculateCartCommand
calculate_cart_handler.rs         → CalculateCartHandler
calculate_cart_controller.rs      → CalculateCartController
calculate_cart_presenter.rs       → CartPresenter
calculate_cart_request.rs         → CalculateCartRequest
calculate_cart_response.rs        → CalculateCartResponse
```

## モジュール構造の統一

### エンティティフォルダ構造

```
src/presentation/{entity}/
├── controllers/
│   ├── mod.rs
│   └── {action}_{entity}_controller.rs
├── presenters/
│   ├── mod.rs
│   └── {action}_{entity}_presenter.rs
├── requests/
│   ├── mod.rs
│   └── {action}_{entity}_request.rs
├── responses/
│   ├── mod.rs
│   └── {action}_{entity}_response.rs
├── mod.rs
└── routes.rs
```

### mod.rs構造の統一

#### メインmod.rs（`src/presentation/{entity}/mod.rs`）

```rust
pub mod controllers;
pub mod presenters;
pub mod requests;
pub mod responses;
pub mod routes;

pub use requests::*;
pub use responses::*;
pub use presenters::*;
pub use routes::*;
```

#### routes.rs構造の統一

```rust
use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;
use crate::presentation::{entity}::controllers::{Action}{Entity}Controller;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge({Action}{Entity}Controller::routes())
}
```

## ステップバイステップ実装手順

### Step 1: Domain層 - ドメインモデルの確認/作成

#### 1.1 必要なドメインモデルの確認

コマンドが操作するドメインオブジェクトが存在するか確認：

```rust
// src/domain/aggregates/{entity}/mod.rs
// 例: Cart アグリゲート
pub use self::cart::Cart;
pub use self::cart_item::CartItem;
```

#### 1.2 必要に応じてドメインモデルを作成

```rust
// src/domain/aggregates/{entity}/{entity}.rs
use crate::domain::value_objects::*;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct {Entity} {
    // フィールド定義
}

impl {Entity} {
    pub fn new() -> Self {
        // 実装
    }
    
    // ビジネスロジックメソッド
    pub fn {business_action}(&mut self, params: SomeParams) -> Result<(), DomainError> {
        // ビジネスルール実装
    }
}
```

### Step 2: Application層 - Command Model作成

#### 2.1 Commandファイル作成

```bash
touch src/application/commands/models/{action}_{entity}_command.rs
```

```rust
// src/application/commands/models/{action}_{entity}_command.rs
use serde::{Deserialize, Serialize};

/// {Action} {Entity}用のコマンドアイテム
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Action}{Entity}CommandItem {
    pub field1: String,
    pub field2: u32,
    // 必要なフィールドを追加
}

/// {Action} {Entity}コマンド
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Action}{Entity}Command {
    pub items: Vec<{Action}{Entity}CommandItem>,
    // またはその他の必要なフィールド
}

impl {Action}{Entity}Command {
    pub fn new(items: Vec<{Action}{Entity}CommandItem>) -> Self {
        Self { items }
    }
}
```

#### 2.2 mod.rsにCommandを追加

```rust
// src/application/commands/models/mod.rs に追加
mod {action}_{entity}_command;

pub use {action}_{entity}_command::{Action}{Entity}Command, {Action}{Entity}CommandItem;
```

### Step 3: Application層 - Command Handler作成

#### 3.1 Handlerファイル作成

```bash
touch src/application/commands/handlers/{action}_{entity}_handler.rs
```

```rust
// src/application/commands/handlers/{action}_{entity}_handler.rs
use std::sync::Arc;
use crate::application::commands::models::{Action}{Entity}Command;
use crate::application::repositories::{Entity}Repository; // 必要に応じて
use crate::application::error::ApplicationError;
use crate::domain::{Entity}; // 操作対象のドメインオブジェクト

/// {Action} {Entity}コマンドハンドラ
pub struct {Action}{Entity}Handler<R: {Entity}Repository> {
    {entity}_repository: Arc<R>,
}

impl<R: {Entity}Repository> {Action}{Entity}Handler<R> {
    pub fn new({entity}_repository: Arc<R>) -> Self {
        Self { {entity}_repository }
    }

    /// コマンドを実行
    pub async fn handle(&self, command: {Action}{Entity}Command) -> Result<{Entity}, ApplicationError> {
        println!("->> {Action}{Entity}Handler::handle");
        
        // 1. バリデーション
        self.validate_command(&command)?;
        
        // 2. ドメインオブジェクトの構築/取得
        let mut {entity} = self.build_{entity}_from_command(&command).await?;
        
        // 3. ビジネスロジックの実行
        {entity}.{business_action}(/* params */)?;
        
        // 4. 永続化（必要に応じて）
        // self.{entity}_repository.save(&{entity}).await?;
        
        Ok({entity})
    }

    fn validate_command(&self, command: &{Action}{Entity}Command) -> Result<(), ApplicationError> {
        // コマンドレベルのバリデーション
        if command.items.is_empty() {
            return Err(ApplicationError::InvalidInput(
                "Items cannot be empty".to_string()
            ));
        }
        
        // その他のバリデーションロジック
        Ok(())
    }

    async fn build_{entity}_from_command(&self, command: &{Action}{Entity}Command) -> Result<{Entity}, ApplicationError> {
        // コマンドからドメインオブジェクトを構築
        // リポジトリから既存データを取得することもある
        
        Ok({Entity}::new())
    }
}
```

#### 3.2 Handlerをmod.rsに追加

```rust
// src/application/commands/handlers/mod.rs に追加
mod {action}_{entity}_handler;

pub use {action}_{entity}_handler::{Action}{Entity}Handler;
```

### Step 4: Application層 - Dispatcher更新

```rust
// src/application/dispatcher.rs を更新
use crate::application::commands::handlers::{Action}{Entity}Handler;
use crate::application::commands::models::{Action}{Entity}Command;
use crate::domain::{Entity};

pub struct Dispatcher {
    // 既存のハンドラ...
    {action}_{entity}_handler: Arc<{Action}{Entity}Handler<SqliteProductRepository>>,
}

impl Dispatcher {
    pub fn new(
        // 既存のパラメータ...
        {action}_{entity}_handler: Arc<{Action}{Entity}Handler<SqliteProductRepository>>,
    ) -> Self {
        Self {
            // 既存のフィールド...
            {action}_{entity}_handler,
        }
    }

    /// 新しいコマンド実行メソッドを追加
    pub async fn execute_{action}_{entity}_command(&self, command: {Action}{Entity}Command) -> Result<{Entity}, ApplicationError> {
        self.{action}_{entity}_handler.handle(command).await
    }
}
```

### Step 5: Infrastructure層 - DI Container更新

```rust
// src/infrastructure/di/container.rs を更新
use crate::application::commands::handlers::{Action}{Entity}Handler;

impl Container {
    async fn new_with_pool(pool: sqlx::SqlitePool) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // 既存のリポジトリ...
        
        // 新しいハンドラを追加
        let {action}_{entity}_handler = Arc::new({Action}{Entity}Handler::new(
            product_repository.clone() // 必要なリポジトリを注入
        ));
        
        // ディスパッチャを更新
        let dispatcher = Arc::new(Dispatcher::new(
            // 既存のハンドラ引数...
            {action}_{entity}_handler,
        ));
        
        Ok(Self {
            // 既存のフィールド...
            dispatcher,
        })
    }
}
```

### Step 6: Presentation層 - Request構造作成

#### 6.1 Requestファイル作成

```bash
mkdir -p src/presentation/{entity}/requests
touch src/presentation/{entity}/requests/{action}_{entity}_request.rs
```

```rust
// src/presentation/{entity}/requests/{action}_{entity}_request.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::application::commands::{Action}{Entity}Command, {Action}{Entity}CommandItem};

/// HTTP リクエスト用の{Entity}アイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {Action}{Entity}ItemRequest {
    pub field1: String,
    pub field2: u32,
    // フロントエンド要件に合わせたフィールド
}

/// HTTP リクエスト用の{Action} {Entity}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct {Action}{Entity}Request {
    pub items: Vec<{Action}{Entity}ItemRequest>,
}

impl {Action}{Entity}Request {
    /// アプリケーション層のコマンドに変換
    pub fn to_command(&self) -> {Action}{Entity}Command {
        let items = self.items
            .iter()
            .map(|item| {Action}{Entity}CommandItem {
                field1: item.field1.clone(),
                field2: item.field2,
            })
            .collect();

        {Action}{Entity}Command::new(items)
    }

    /// バリデーション
    pub fn validate(&self) -> Result<(), String> {
        if self.items.is_empty() {
            return Err("Items cannot be empty".to_string());
        }

        for (index, item) in self.items.iter().enumerate() {
            if item.field1.trim().is_empty() {
                return Err(format!("Item {} has empty field1", index));
            }

            if item.field2 == 0 {
                return Err(format!("Item {} has zero field2", index));
            }

            // その他のバリデーションロジック
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request_converts_to_command() {
        let request = {Action}{Entity}Request {
            items: vec![
                {Action}{Entity}ItemRequest {
                    field1: "test".to_string(),
                    field2: 1,
                },
            ],
        };

        assert!(request.validate().is_ok());
        
        let command = request.to_command();
        assert_eq!(command.items.len(), 1);
        assert_eq!(command.items[0].field1, "test");
        assert_eq!(command.items[0].field2, 1);
    }

    #[test]
    fn empty_items_fails_validation() {
        let request = {Action}{Entity}Request {
            items: vec![],
        };
        
        assert!(request.validate().is_err());
    }
}
```

#### 6.2 Requestをmod.rsに追加

```rust
// src/presentation/{entity}/requests/mod.rs
mod {action}_{entity}_request;

pub use {action}_{entity}_request:{{Action}{Entity}Request, {Action}{Entity}ItemRequest};
```

### Step 7: Presentation層 - Response構造作成

#### 7.1 Responseファイル作成

```bash
mkdir -p src/presentation/{entity}/responses
touch src/presentation/{entity}/responses/{action}_{entity}_response.rs
```

```rust
// src/presentation/{entity}/responses/{action}_{entity}_response.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {Action}{Entity}Response {
    pub items: Vec<{Action}{Entity}ItemResponse>,
    pub total_quantity: u32,
    pub item_count: u32,
    pub total: u32,
    pub is_empty: bool,
    // ビジネス要件に応じたフィールド
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {Action}{Entity}ItemResponse {
    pub field1: String,
    pub field2: String,
    pub quantity: u32,
    pub subtotal: u32,
    // フロントエンド要件に合わせたフィールド
}

impl {Action}{Entity}Response {
    pub fn new(
        items: Vec<{Action}{Entity}ItemResponse>,
        total_quantity: u32,
        item_count: u32,
        total: u32,
        is_empty: bool,
    ) -> Self {
        Self {
            items,
            total_quantity,
            item_count,
            total,
            is_empty,
        }
    }
}
```

#### 7.2 Responseをmod.rsに追加

```rust
// src/presentation/{entity}/responses/mod.rs
mod {action}_{entity}_response;

pub use {action}_{entity}_response:{{Action}{Entity}Response, {Action}{Entity}ItemResponse};
```

### Step 8: Presentation層 - Presenter作成

#### 8.1 Presenterファイル作成

```bash
mkdir -p src/presentation/{entity}/presenters
touch src/presentation/{entity}/presenters/{action}_{entity}_presenter.rs
```

```rust
// src/presentation/{entity}/presenters/{action}_{entity}_presenter.rs
use crate::domain::{Entity};
use crate::presentation::{entity}::responses:{{Action}{Entity}Response, {Action}{Entity}ItemResponse};

/// {Entity}プレゼンター
/// Clean Architecture: Interface Adapters層
/// ドメインオブジェクトをHTTPレスポンス用DTOに変換する
pub struct {Entity}Presenter;

impl {Entity}Presenter {
    /// ドメインの{Entity}を{Action}{Entity}Responseに変換
    pub fn to_response({entity}: {Entity}) -> Result<{Action}{Entity}Response, String> {
        let mut items = Vec::new();
        
        // 各アイテムを変換
        for item in {entity}.items() {
            let item_response = {Action}{Entity}ItemResponse {
                field1: item.field1().to_string(),
                field2: item.field2().to_string(),
                quantity: item.quantity(),
                subtotal: item.calculate_subtotal()?,
            };
            
            items.push(item_response);
        }

        // エンティティ全体の計算
        let total = {entity}.calculate_total()
            .map_err(|e| format!("Failed to calculate total: {}", e))?;

        Ok({Action}{Entity}Response::new(
            items,
            {entity}.total_quantity(),
            {entity}.item_count(),
            total,
            {entity}.is_empty(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    fn create_test_{entity}() -> {Entity} {
        {Entity}::new()
    }

    #[test]
    fn empty_{entity}_to_response() {
        let {entity} = {Entity}::new();
        let response = {Entity}Presenter::to_response({entity}).unwrap();
        
        assert!(response.is_empty);
        assert_eq!(response.item_count, 0);
        assert_eq!(response.total_quantity, 0);
        assert_eq!(response.total, 0);
        assert!(response.items.is_empty());
    }

    #[test]
    fn {entity}_with_items_to_response() {
        let mut {entity} = {Entity}::new();
        // テストデータを追加
        
        let response = {Entity}Presenter::to_response({entity}).unwrap();
        
        assert!(!response.is_empty);
        // アサーションを追加
    }
}
```

#### 8.2 Presenterをmod.rsに追加

```rust
// src/presentation/{entity}/presenters/mod.rs
mod {action}_{entity}_presenter;

pub use {action}_{entity}_presenter::{Entity}Presenter;
```

### Step 9: Presentation層 - Controller作成

#### 9.1 Controllerファイル作成

```bash
mkdir -p src/presentation/{entity}/controllers
touch src/presentation/{entity}/controllers/{action}_{entity}_controller.rs
```

```rust
// src/presentation/{entity}/controllers/{action}_{entity}_controller.rs
use axum::{routing::post, Router, extract::State, Json};
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::error::Result;
use crate::presentation::{entity}:{{Action}{Entity}Request, {Entity}Presenter, {Action}{Entity}Response};
use crate::presentation::ErrorResponse;

/// {Action} {Entity} Controller
pub struct {Action}{Entity}Controller;

impl {Action}{Entity}Controller {
    /// ルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/{entity}", post(handle))
    }
}

/// POST /{entity} - {Action} {Entity}処理
/// {具体的な処理の説明}
#[utoipa::path(
    post,
    path = "/{entity}",
    operation_id = "{action}_{entity}",
    request_body = {Action}{Entity}Request,
    responses(
        (status = 200, description = "{Action} {Entity}成功", body = {Action}{Entity}Response),
        (status = 400, description = "リクエストが無効です", body = ErrorResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "{Entity}"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
    Json(request): Json<{Action}{Entity}Request>,
) -> Result<Json<{Action}{Entity}Response>> {
    println!("->> {Action}{Entity}Controller::handle - {} items", request.items.len());

    // 1. リクエストバリデーション
    request.validate()
        .map_err(|msg| crate::application::error::ApplicationError::Validation(msg))?;

    // 2. アプリケーション層のコマンドに変換
    let command = request.to_command();

    // 3. Dispatcherを通じてユースケースを実行
    let dispatcher = container.get_dispatcher();
    let {entity} = dispatcher
        .execute_{action}_{entity}_command(command)
        .await?; // ApplicationErrorからErrorへの自動変換を利用

    // 4. プレゼンターでレスポンスに変換
    let response = {Entity}Presenter::to_response({entity})
        .map_err(|msg| crate::application::error::ApplicationError::InvalidInput(msg))?;

    println!("->> {Action}{Entity}Controller::handle - success for {entity} with {} items", response.item_count);
    Ok(Json(response))
}
```

#### 9.2 Controllerをmod.rsに追加

```rust
// src/presentation/{entity}/controllers/mod.rs
pub mod {action}_{entity}_controller;

pub use {action}_{entity}_controller::{Action}{Entity}Controller;
```

### Step 10: Presentation層 - モジュール統合とRoutes作成

#### 10.1 routes.rs作成

```bash
touch src/presentation/{entity}/routes.rs
```

```rust
// src/presentation/{entity}/routes.rs
use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;
use crate::presentation::{entity}::controllers::{Action}{Entity}Controller;

/// {Entity}関連のルーティング
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge({Action}{Entity}Controller::routes())
}
```

#### 10.2 mod.rs作成

```bash
touch src/presentation/{entity}/mod.rs
```

```rust
// src/presentation/{entity}/mod.rs
pub mod presenters;
pub mod controllers;
pub mod requests;
pub mod responses;
pub mod routes;

pub use requests::*;
pub use responses::*;
pub use presenters::*;
pub use routes::*;
```

### Step 11: ルートの登録

```rust
// src/presentation/routes.rs を更新
use crate::presentation::{entity};

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        // 既存のルート...
        .merge({entity}::routes())
        // その他のルート...
}
```

```rust
// src/presentation/mod.rs を更新（必要に応じて）
mod {entity};
```

### Step 12: OpenAPI/Swagger設定更新

#### 12.1 Response型のインポート追加

```rust
// src/presentation/swagger/openapi.rs のuse文に追加
use crate::presentation::{entity}::responses::{
    {Action}{Entity}Response,
    {Action}{Entity}ItemResponse,
};
use crate::presentation::{entity}::requests::{
    {Action}{Entity}Request,
    {Action}{Entity}ItemRequest,
};
```

#### 12.2 パス（エンドポイント）の追加

```rust
// src/presentation/swagger/openapi.rs の #[openapi] マクロ内のpaths()セクションに追加
#[openapi(
    paths(
        // 既存のパス...
        // 新しいパスを追加
        crate::presentation::{entity}::controllers::{action}_{entity}_controller::handle,
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
        // 新しいスキーマを追加
        {Action}{Entity}Request,
        {Action}{Entity}ItemRequest,
        {Action}{Entity}Response,
        {Action}{Entity}ItemResponse,
    )
),
```

#### 12.4 タグの追加

```rust
// src/presentation/swagger/openapi.rs のtags()セクションに追加
tags(
    // 既存のタグ...
    // 新しいタグを追加
    (name = "{Entity}", description = "{Entity}関連のAPI"),
),
```

## 実装済みコマンドエンドポイント例

### カート計算実装例
```
src/presentation/cart/
├── controllers/
│   ├── mod.rs
│   └── calculate_cart_controller.rs    → CalculateCartController
├── presenters/
│   ├── mod.rs
│   └── calculate_cart_presenter.rs     → CartPresenter
├── requests/
│   ├── mod.rs
│   └── calculate_cart_request.rs       → CalculateCartRequest
├── responses/
│   ├── mod.rs
│   └── calculate_cart_response.rs      → CalculateCartResponse
├── mod.rs
└── routes.rs
```

### Application層構造例
```
src/application/commands/
├── handlers/
│   ├── mod.rs
│   └── calculate_cart_handler.rs       → CalculateCartHandler
├── models/
│   ├── mod.rs
│   └── calculate_cart_command.rs       → CalculateCartCommand
└── mod.rs
```

## コマンドエンドポイントの特徴

### 1. バリデーション層

コマンドエンドポイントは複数レベルでバリデーションを行います：

1. **Request DTO レベル** - HTTP入力の妥当性
2. **Command ハンドラレベル** - ビジネスルールの妥当性
3. **Domain レベル** - ドメイン不変条件

### 2. エラーハンドリング

```rust
// カスタムエラーの種類
pub enum ApplicationError {
    Validation(String),           // バリデーションエラー
    InvalidInput(String),         // 入力データエラー
    BusinessRuleViolation(String), // ビジネスルール違反
    NotFound(String),             // リソース不存在
    Repository(RepositoryError),   // データアクセスエラー
}
```

### 3. トランザクション管理

```rust
impl CalculateCartHandler<R> {
    pub async fn handle(&self, command: CalculateCartCommand) -> Result<Cart, ApplicationError> {
        // トランザクション開始（必要に応じて）
        
        // 1. バリデーション
        // 2. ドメインロジック実行
        // 3. 永続化
        
        // トランザクションコミット
        Ok(result)
    }
}
```

## チェックリスト

実装完了後、以下をチェックしてください：

### 命名規則チェック

- [ ] ファイル名が統一された規則に従っている
- [ ] struct名が統一された規則に従っている
- [ ] エンドポイントパスが適切に設定されている
- [ ] OpenAPIタグが適切に設定されている

### アーキテクチャチェック

- [ ] Command/Query分離が適切に行われている
- [ ] 依存関係の方向が正しい（内側に向かっている）
- [ ] RequestからCommandへの変換が適切
- [ ] DomainからResponseへの変換が適切

### バリデーションチェック

- [ ] Request DTOでの入力バリデーション実装
- [ ] Command Handlerでのビジネスルールバリデーション実装
- [ ] 適切なエラーメッセージとステータスコード

### コンパイルチェック

- [ ] `cd app/backend && cargo check` でコンパイルエラーなし
- [ ] `cd app/backend && cargo clippy` で警告なし
- [ ] `cd app/backend && cargo fmt` でフォーマット済み

### 機能チェック

- [ ] `cd app/backend && cargo run` でサーバー起動
- [ ] `curl -X POST http://localhost:4000/{entity} -H "Content-Type: application/json" -d '{json}'` でレスポンス確認
- [ ] Swagger UI (`http://localhost:4000/swagger-ui/`) で仕様確認

### テストチェック

- [ ] `cd app/backend && cargo test` でテスト通過
- [ ] Request DTOのバリデーションテスト実装
- [ ] Presenterのユニットテスト実装
- [ ] Command Handlerのユニットテスト実装

## トラブルシューティング

### よくあるエラー

1. **バリデーションエラーが適切にハンドリングされない**
   - Request DTOのvalidateメソッド実装漏れ
   - エラーの型変換処理漏れ

2. **ドメインオブジェクトの作成で失敗**
   - 必要なRepositoryの注入漏れ
   - ドメインオブジェクトのコンストラクタエラー

3. **コンパイルエラー: "trait ... is not implemented"**
   - `use` 文の追加漏れ
   - `#[async_trait]` の追加漏れ

4. **レスポンス変換エラー**
   - Presenterでの型変換処理ミス
   - ドメインオブジェクトのメソッド呼び出しエラー

5. **HTTP 400/500エラー**
   - リクエストのJSONフォーマットエラー
   - バリデーション処理のバグ

### デバッグのコツ

1. 既存の実装（cart計算）を参考にする
2. `println!` でログを仕込む
3. 各層を個別にテストする
4. Postmanやcurlでリクエストをテストする
5. エラーレスポンスの詳細を確認する

## まとめ

このガイドに従って実装することで：

- **CQRS原則**が遵守された設計
- **Clean Architecture**の層分離が適切
- **統一された命名規則**でプロジェクト全体の一貫性が保たれる
- **バリデーション**が多層で適切に実装される
- **エラーハンドリング**が適切に行われる
- **テスト**が容易になる

新しくコマンドエンドポイントを作成する際は、cart計算の実装を参考にしてください。 