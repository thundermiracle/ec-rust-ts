# Clean Architecture + DDD 実装原則 (Rust版)

## 🎯 基本原則

### 依存関係の方向
- **絶対原則**: 依存は常に内向き（Domain ← Application ← Infrastructure/Presentation）
- **Domain層**: 外部への依存は一切禁止（`std`のみ許可）
- **Application層**: Domain層とInfrastructure層のトレイトのみ依存
- **Infrastructure層**: Application層のトレイトを実装

### Repository配置の実用的判断
```rust
// ❌ 理想論: Repository Trait を Domain層
// ✅ 実用解: Repository Trait を Application層（N+1問題回避のため）

// application/repositories.rs
pub trait CartRepository {
    async fn find_by_id(&self, id: CartId) -> Result<Option<Cart>, RepositoryError>;
    async fn save(&self, cart: Cart) -> Result<(), RepositoryError>;
}
```

## 🏗️ ビジネスロジック配置の鉄則

### 1. Entity First（最優先）
```rust
// domain/aggregates/cart.rs
use crate::domain::value_objects::*;
use crate::domain::entities::*;

#[derive(Debug, Clone)]
pub struct Cart {
    id: CartId,
    items: Vec<CartItem>,
    shipping_fee: Option<Money>,
    applied_coupons: Vec<AppliedCoupon>,
}

impl Cart {
    // ✅ Entity内での完結を最優先
    pub fn apply_shipping_fee(
        &mut self, 
        method: &ShippingMethod, 
        rules: &[ShippingRule]
    ) -> Result<(), DomainError> {
        // 配送料計算はCart内部の責務
        let subtotal = self.calculate_subtotal();
        let fee = self.calculate_shipping_fee(subtotal, method, rules)?;
        self.shipping_fee = Some(fee);
        Ok(())
    }
    
    pub fn apply_coupon(
        &mut self, 
        coupon: &Coupon, 
        rules: &[CouponRule]
    ) -> Result<DiscountResult, DomainError> {
        // クーポン適用もCart内部の責務
        let mut total_discount = Money::zero();
        let mut applicable_items = Vec::new();

        for item in &mut self.items {
            if self.is_item_eligible(item, coupon, rules)? {
                let discount = self.calculate_item_discount(item, coupon, rules)?;
                item.apply_discount(discount);
                applicable_items.push(item.clone());
                total_discount = total_discount.add(discount)?;
            }
        }

        let result = DiscountResult::new(total_discount, applicable_items);
        self.applied_coupons.push(AppliedCoupon::new(coupon.clone(), result.clone()));
        Ok(result)
    }

    fn calculate_shipping_fee(
        &self,
        subtotal: Money,
        method: &ShippingMethod,
        rules: &[ShippingRule]
    ) -> Result<Money, DomainError> {
        let applicable_rule = rules
            .iter()
            .find(|rule| {
                rule.shipping_method_id() == method.id() &&
                subtotal >= rule.min_amount() &&
                subtotal <= rule.max_amount()
            });

        match applicable_rule {
            Some(rule) => rule.calculate_fee(subtotal),
            None => Ok(Money::zero()),
        }
    }
}
```

### 2. Domain Service（例外的使用）
```rust
// domain/services/money_transfer_service.rs
use crate::domain::entities::{Account, AccountId};
use crate::domain::value_objects::{Money, TransferResult};
use crate::domain::errors::DomainError;

// ✅ 複数Aggregateにまたがる場合のみ
pub struct MoneyTransferService;

impl MoneyTransferService {
    pub fn transfer(
        from_account: &mut Account,
        to_account: &mut Account,
        amount: Money,
    ) -> Result<TransferResult, DomainError> {
        // どちらのAccountにも属さない処理
        from_account.withdraw(amount)?;
        to_account.deposit(amount)?;
        
        Ok(TransferResult::new(
            from_account.balance(),
            to_account.balance(),
        ))
    }
}

// domain/services/user_unique_check_service.rs
// ✅ Aggregate横断的チェック
#[async_trait::async_trait]
pub trait UserUniqueCheckService {
    async fn is_username_unique(&self, username: &str) -> Result<bool, DomainError>;
}
```

### 3. Application Service（調整役）
```rust
// application/use_cases/calculate_cart_total.rs
use crate::application::repositories::{CartRepository, ShippingRepository, CouponRepository};
use crate::domain::entities::Cart;
use crate::domain::value_objects::{CartId, CartTotalResponse};

pub struct CalculateCartTotalUseCase<C, S, U> 
where
    C: CartRepository,
    S: ShippingRepository,
    U: CouponRepository,
{
    cart_repo: C,
    shipping_repo: S,
    coupon_repo: U,
}

impl<C, S, U> CalculateCartTotalUseCase<C, S, U>
where
    C: CartRepository,
    S: ShippingRepository,
    U: CouponRepository,
{
    // ✅ データ取得とオーケストレーションのみ
    pub async fn execute(
        &self,
        command: CalculateCartTotalCommand,
    ) -> Result<CartTotalResponse, ApplicationError> {
        // 1. データ取得（効率的にN+1回避）
        let mut cart = self.cart_repo
            .find_by_id(command.cart_id)
            .await?
            .ok_or(ApplicationError::CartNotFound)?;
            
        let shipping_data = self.shipping_repo
            .find_with_rules(command.shipping_method_id)
            .await?;
            
        let coupon_data = self.coupon_repo
            .find_with_rules(&command.coupon_code)
            .await?;

        // 2. Domain内で計算実行
        cart.apply_shipping_fee(&shipping_data.method, &shipping_data.rules)?;
        
        if let Some(coupon_data) = coupon_data {
            cart.apply_coupon(&coupon_data.coupon, &coupon_data.rules)?;
        }

        // 3. 永続化・変換
        self.cart_repo.save(cart.clone()).await?;
        Ok(CartTotalResponse::from(cart))
    }
}
```

## 🚫 アンチパターン

### ❌ Domain Serviceの濫用
```rust
// ❌ 間違い: Cart内部の計算をDomain Serviceに
pub struct ShippingFeeCalculationService;

impl ShippingFeeCalculationService {
    // ❌ これはCartの責務！Domain Service不要
    pub fn calculate(
        cart: &Cart, 
        method: &ShippingMethod
    ) -> Result<Money, DomainError> {
        // Cartの内部計算をサービスで行うのは間違い
        unimplemented!()
    }
}
```

### ❌ Application層へのビジネスロジック漏れ
```rust
// ❌ 間違い: Application層での計算
impl CartUseCase {
    pub async fn handle(&self, command: Command) -> Result<(), ApplicationError> {
        let mut cart = self.repository.find(command.cart_id).await?;
        
        // ❌ ビジネスロジックがApplication層に漏れている
        let discount = if command.coupon_code == "SPECIAL" {
            cart.total().multiply(0.1)? // ❌ 計算ロジックが分散
        } else {
            Money::zero()
        };
            
        cart.apply_discount(discount)?; // ❌ 計算と適用が分離
        Ok(())
    }
}
```

## 🔍 判断フローチャート

```
ビジネスロジックの配置決定
        ↓
単一Entityで完結する？
    ↓ Yes
   Entity内に実装
    ↓ No
複数Aggregateにまたがる？
    ↓ Yes
  Domain Service
    ↓ No
Application Serviceで調整
```

## 🔧 Rustの特徴を活かした実装

### Result型によるエラーハンドリング
```rust
// domain/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid money amount: {0}")]
    InvalidMoneyAmount(String),
    #[error("Coupon not applicable: {0}")]
    CouponNotApplicable(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
}

// application/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Cart not found")]
    CartNotFound,
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
}
```

### 所有権を活かした設計
```rust
// Value Objectは所有権で不変性を保証
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    amount: i64, // cents
    currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Result<Self, DomainError> {
        if amount < 0 {
            return Err(DomainError::InvalidMoneyAmount(
                "Amount cannot be negative".to_string()
            ));
        }
        Ok(Self { amount, currency })
    }
    
    // 新しいインスタンスを返すことで不変性を保証
    pub fn add(self, other: Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidMoneyAmount(
                "Currency mismatch".to_string()
            ));
        }
        Money::new(self.amount + other.amount, self.currency)
    }
}
```

## 🎯 重要な原則

1. **Entity First**: 99%の場合、適切なAggregate設計で解決
2. **Domain Serviceは最後の手段**: 本当に必要な場面は極めて限定的
3. **Application層は純粋な調整役**: ビジネスロジック禁止
4. **Rustの所有権を活用**: 不変性とメモリ安全性の確保
5. **Result型で明示的エラーハンドリング**: パニック禁止、全てResult型で処理