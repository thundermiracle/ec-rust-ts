# Product中心のAggregate Refactoring計画

## 概要

このリファクタリングでは、Productドメインモデルを中心としたアグリゲートに再設計します。主な目標は：

1. Product集約ルートとSKUエンティティの明確な関係の確立
2. 値オブジェクトを使用した型安全性の強化
3. ドメインロジックの適切なカプセル化
4. バリエーション管理の柔軟な実装

## リファクタリングアプローチ

リファクタリングは以下のフェーズで段階的に進めます：

### フェーズ1: 新しいドメインモデルの設計と実装

- [ ] 値オブジェクトの実装:
  - [x] identifiers.rs (ProductId, SKUId, CategoryId, ColorId, TagId)
  - [x] names.rs (ProductName, SKUName, SKUCode)
  - [x] descriptions.rs (Description)
  - [x] size_material.rs (Size, Material)
  - [x] variant_attributes.rs (VariantAttributes)

- [ ] エンティティの実装:
  - [ ] sku.rs (SKU, Stock, StockAdjustment, SKUStatus)
  - [ ] product.rs (Product, ProductStatus)

### フェーズ2: リポジトリとユースケースの調整

- [ ] リポジトリインターフェースの更新:
  - [ ] product_repository.rs
  - [ ] category_repository.rs
  - [ ] tag_repository.rs

- [ ] リポジトリ実装の更新:
  - [ ] sqlite_product_repository.rs

- [ ] ユースケースの更新:
  - [ ] get_product_use_case.rs
  - [ ] create_product_use_case.rs
  - [ ] update_product_use_case.rs
  - [ ] buy_product_use_case.rs

### フェーズ3: フレームワーク層とアダプター層の調整

- [ ] コントローラーの更新:
  - [ ] get_product_controller.rs
  - [ ] buy_product_controller.rs

- [ ] プレゼンターの更新:
  - [ ] product_presenter.rs

- [ ] リクエスト/レスポンスDTOの更新:
  - [ ] product_request.rs
  - [ ] product_response.rs

### フェーズ4: データベースとマイグレーションの調整

- [ ] データベーススキーマの更新:
  - [ ] products テーブル
  - [ ] skus テーブル（新規）
  - [ ] product_images テーブル
  - [ ] tags テーブル

- [ ] マイグレーションスクリプトの作成

## テスト計画

- [ ] 単体テスト:
  - [ ] 値オブジェクト
  - [ ] エンティティ
  - [ ] ユースケース

- [ ] 統合テスト:
  - [ ] リポジトリ
  - [ ] コントローラー

- [ ] エンドツーエンドテスト

## デプロイ計画

1. ステージング環境でのリファクタリングテスト
2. データベースマイグレーションの検証
3. 本番環境へのロールアウト

## 参考資料

- Clean Architecture原則
- DDDパターン
- CQRSパターン 