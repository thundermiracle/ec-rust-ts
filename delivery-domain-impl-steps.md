# 配送情報ドメイン実装プラン

## 概要
配送情報のdomain部分を実装するプランです。既存のAddress、Email value objectsを活用し、配送に必要な情報を管理する構造を構築します。

## 必須フィールド
- メールアドレス (既存のEmailを使用)
- 姓 (FirstName)
- 名 (LastName) 
- 郵便番号 (既存のAddressに含まれる)
- 都道府県 (既存のAddressに含まれる)
- 市区町村 (既存のAddressに含まれる)
- 番地 (既存のAddressに含まれる)
- 電話番号 (既存のAddressに含まれる)

## オプションフィールド
- アパート・マンション名 (既存のAddressに含まれる)

## 実装ステップ

### 1. 新しいValue Objectsの作成

#### 1.1 PersonalInfoの実装
**ファイル**: `app/backend/src/domain/value_objects/personal_info.rs`
- PersonalInfo struct (個人情報)
  - first_name: FirstName (姓)
  - last_name: LastName (名)
- FirstName struct (姓)
- LastName struct (名)
- バリデーション機能
  - 空文字チェック
  - 長さ制限 (例: 50文字)
  - 文字種制限 (日本語、英語、数字、記号)
- Display trait実装 (文字列表現)

#### 1.2 PhoneNumberの独立化
**ファイル**: `app/backend/src/domain/value_objects/phone_number.rs`
- 現在Addressに含まれているphone_numberを独立したvalue objectとして抽出
- 日本の電話番号形式のバリデーション
- ハイフンありなし両方対応
- Display trait実装 (ハイフン付き形式で表示)

### 2. 既存Value Objectsの改良

#### 2.1 Addressの構造改善
**ファイル**: `app/backend/src/domain/value_objects/address.rs`
- PhoneNumberをPhoneNumber value objectに変更
- より詳細なバリデーション追加
- 郵便番号の形式チェック (例: 123-4567)
- Display trait実装 (full_address形式で表示)

#### 2.2 Emailの改良
**ファイル**: `app/backend/src/domain/value_objects/email.rs`
- 既存実装は問題なし
- 必要に応じて日本語ドメインサポート
- Display trait実装 (既存のvalue()メソッドを使用)

### 3. DeliveryInfoの実装

#### 3.1 DeliveryInfo Entity
**ファイル**: `app/backend/src/domain/entities/delivery_info.rs`
```rust
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeliveryInfo {
    // 識別子
    pub id: DeliveryInfoId,
    
    // 基本配送情報（不変）
    pub email: Email,
    pub personal_info: PersonalInfo,
    pub address: Address,
    pub phone_number: PhoneNumber,
    
    // 配送状態（可変）
    pub status: DeliveryStatus,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,
    pub shipping_method: Option<String>,
    
    // タイムスタンプ
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    Processing,
    Shipped,
    InTransit,
    Delivered,
    Failed,
}
```

#### 3.2 DeliveryInfoIdの追加
**ファイル**: `app/backend/src/domain/value_objects/identifiers.rs`
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeliveryInfoId(pub String);
```

#### 3.3 Entity機能の実装
- **新規作成**: `new()` メソッド
- **状態更新**: `update_status()`, `set_tracking_info()` メソッド
- **配送開始**: `mark_as_shipped()` メソッド
- **配送完了**: `mark_as_delivered()` メソッド
- **バリデーション**: 状態遷移の妥当性チェック

#### 3.4 Display trait実装
- 配送情報の読みやすい形式での表示
- 個人情報、住所、連絡先、配送状態の整理された出力

### 4. mod.rsファイルの更新

#### 4.1 value_objects/mod.rs
**ファイル**: `app/backend/src/domain/value_objects/mod.rs`
- 新しいvalue objectsのモジュール追加
- pub useでエクスポート

#### 4.2 entities/mod.rs
**ファイル**: `app/backend/src/domain/entities/mod.rs`
- DeliveryInfoエンティティの追加
- pub useでエクスポート

### 5. テストの実装

#### 5.1 単体テスト
各value objectに対して:
- 正常パターンのテスト
- 異常パターンのテスト (バリデーションエラー)
- 境界値テスト

#### 5.2 統合テスト
- DeliveryInfoエンティティの作成・検証テスト
- 状態遷移のテスト
- 複数のvalue objectsの組み合わせテスト

### 6. エラーハンドリング

#### 6.1 エラータイプの定義
各value objectに対応するエラータイプ:
- PersonalInfoError (Empty, TooLong, InvalidCharacter)
- PhoneNumberError (Empty, InvalidFormat, TooLong)
- DeliveryInfoError (複合エラー、状態遷移エラー)

#### 6.2 エラーのDisplay trait実装
- 各エラータイプにDisplay trait実装
- 日本語でのエラーメッセージ
- ユーザーフレンドリーなエラー表示

### 7. 実装順序

1. **PhoneNumber** value objectの実装
2. **PersonalInfo** (FirstName, LastName含む) value objectの実装
3. **Address** value objectの改良 (PhoneNumber使用)
4. **DeliveryInfoId** value objectの実装
5. **DeliveryInfo** entityの実装
6. **Display trait** の実装 (全value objects、entity、エラータイプ)
7. **mod.rs** の更新
8. **テスト** の実装
9. **エラーハンドリング** の完成

### 8. 考慮事項

#### 8.1 国際化対応
- 日本語文字のサポート
- 文字エンコーディング (UTF-8)
- 住所形式の日本固有対応

#### 8.2 セキュリティ
- 個人情報の適切な取り扱い
- バリデーション時のサニタイゼーション
- 機密情報のログ出力回避

#### 8.3 パフォーマンス
- 文字列操作の最適化
- 正規表現の事前コンパイル
- 不要なクローンの回避

### 9. 将来の拡張性

#### 9.1 配送オプション
- 配送日時指定
- 配送方法選択
- 特別な配送指示

#### 9.2 複数配送先対応
- 複数のDeliveryInfoを管理
- 配送先の優先度設定

## 完了基準

1. 全ての必須フィールドが適切にバリデーションされる
2. 既存のEmail、Address value objectsが再利用される
3. 新しいPersonalInfo、PhoneNumber value objectsが実装される
4. DeliveryInfo entityが正しく機能する
5. 包括的なテストカバレッジが達成される
6. エラーハンドリングが適切に実装される
7. 全value objects、entity、エラータイプにDisplay traitが実装される
8. mod.rsファイルが正しく更新される