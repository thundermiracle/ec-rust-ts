use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::application::error::RepositoryError;
use crate::application::repositories::OrderRepository;
use crate::domain::aggregates::order::Order;

/// SQLite実装のOrderRepository
/// Clean Architecture: Frameworks & Drivers層
pub struct SqliteOrderRepository {
    pool: SqlitePool,
}

impl SqliteOrderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for SqliteOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), RepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                println!("->> [SqliteOrderRepository::save] Transaction begin failed: {}", error_msg);
                RepositoryError::QueryExecution(format!("[SqliteOrderRepository::save_transaction_begin] {}", error_msg))
            })?;

        // 注文データを挿入
        sqlx::query(
            r#"
            INSERT INTO orders (
                id, order_number, customer_first_name, customer_last_name,
                customer_email, customer_phone, shipping_method_id, shipping_fee,
                shipping_postal_code, shipping_prefecture, shipping_city,
                shipping_street, shipping_building, payment_method_id,
                payment_fee, payment_details, subtotal, shipping_fee_total,
                payment_fee_total, tax_amount, total_amount, status,
                created_at, updated_at, notes
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25
            )
            "#,
        )
        .bind(order.id.value().to_string())
        .bind(order.order_number.value())
        .bind(order.customer_info.personal_info.first_name().value())
        .bind(order.customer_info.personal_info.last_name().value())
        .bind(order.customer_info.email.value())
        .bind(order.customer_info.phone.value())
        .bind(order.shipping_info.method_id.value())
        .bind(order.shipping_info.fee.amount_in_yen() as i64)
        .bind(order.shipping_info.address.postal_code())
        .bind(order.shipping_info.address.prefecture())
        .bind(order.shipping_info.address.city())
        .bind(order.shipping_info.address.street())
        .bind(order.shipping_info.address.building())
        .bind(order.payment_info.method_id.value().to_string())
        .bind(order.payment_info.fee.amount_in_yen() as i64)
        .bind(
            order
                .payment_info
                .payment_details
                .as_ref()
                .map(|d| d.to_json_string().to_string()),
        )
        .bind(order.pricing.subtotal.amount_in_yen() as i64)
        .bind(order.pricing.shipping_fee.amount_in_yen() as i64)
        .bind(order.pricing.payment_fee.amount_in_yen() as i64)
        .bind(order.pricing.tax_amount.amount_in_yen() as i64)
        .bind(order.pricing.total.amount_in_yen() as i64)
        .bind(order.status.to_string())
        .bind(order.timestamps.created_at.to_rfc3339())
        .bind(order.timestamps.updated_at.to_rfc3339())
        .bind(order.notes.as_deref())
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            let error_msg = e.to_string();
            println!("->> [SqliteOrderRepository::save] Order insertion failed: {}", error_msg);
            
            if error_msg.contains("FOREIGN KEY constraint failed") {
                let field = if error_msg.contains("shipping_method_id") {
                    "shipping_method_id"
                } else if error_msg.contains("payment_method_id") {
                    "payment_method_id"
                } else {
                    // SQLiteエラーメッセージを解析してカラム名を特定
                    if error_msg.contains("orders.shipping_method_id") {
                        "shipping_method_id"
                    } else if error_msg.contains("orders.payment_method_id") {
                        "payment_method_id"
                    } else {
                        println!("->> [SqliteOrderRepository::save] Could not determine FK field from error: {}", error_msg);
                        "unknown_order_field"
                    }
                };
                
                RepositoryError::ForeignKeyConstraint {
                    field: field.to_string(),
                    message: format!("[SqliteOrderRepository::save] {}", error_msg),
                }
            } else {
                RepositoryError::QueryExecution(format!("[SqliteOrderRepository::save] {}", error_msg))
            }
        })?;

        // 注文アイテムを挿入
        for item in &order.items {
            sqlx::query(
                r#"
                INSERT INTO order_items (
                    order_id, sku_id, sku_code, product_name, sku_name,
                    unit_price, quantity, subtotal
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
            )
            .bind(order.id.value().to_string())
            .bind(item.sku_id.value().to_string())
            .bind(item.sku_code.value())
            .bind(item.product_name.value())
            .bind(item.sku_name.value())
            .bind(item.unit_price.amount_in_yen() as i64)
            .bind(item.quantity)
            .bind(item.subtotal().unwrap().amount_in_yen() as i64)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                println!("->> [SqliteOrderRepository::save] Order item insertion failed for SKU {}: {}", 
                    item.sku_id.value(), error_msg);
                
                if error_msg.contains("FOREIGN KEY constraint failed") {
                    let field = if error_msg.contains("sku_id") || error_msg.contains("order_items.sku_id") {
                        "sku_id"
                    } else if error_msg.contains("order_id") || error_msg.contains("order_items.order_id") {
                        "order_id"  
                    } else {
                        println!("->> [SqliteOrderRepository::save] Could not determine FK field from order_items error: {}", error_msg);
                        "unknown_order_item_field"
                    };
                    
                    RepositoryError::ForeignKeyConstraint {
                        field: field.to_string(),
                        message: format!("[SqliteOrderRepository::save_order_item] SKU: {}, Error: {}", 
                            item.sku_id.value(), error_msg),
                    }
                } else {
                    RepositoryError::QueryExecution(format!("[SqliteOrderRepository::save_order_item] SKU: {}, Error: {}", 
                        item.sku_id.value(), error_msg))
                }
            })?;
        }

        tx.commit()
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                println!("->> [SqliteOrderRepository::save] Transaction commit failed: {}", error_msg);
                RepositoryError::QueryExecution(format!("[SqliteOrderRepository::save_transaction_commit] {}", error_msg))
            })?;

        Ok(())
    }

    async fn update(&self, order: &Order) -> Result<(), RepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // 注文データを更新
        sqlx::query(
            r#"
            UPDATE orders SET
                customer_first_name = ?1, customer_last_name = ?2,
                customer_email = ?3, customer_phone = ?4,
                shipping_method_id = ?5, shipping_fee = ?6,
                shipping_postal_code = ?7, shipping_prefecture = ?8,
                shipping_city = ?9, shipping_street = ?10,
                shipping_building = ?11, payment_method_id = ?12,
                payment_fee = ?13, payment_details = ?14,
                subtotal = ?15, shipping_fee_total = ?16,
                payment_fee_total = ?17, tax_amount = ?18,
                total_amount = ?19, status = ?20, updated_at = ?21,
                notes = ?22
            WHERE id = ?23
            "#,
        )
        .bind(order.customer_info.personal_info.first_name().value())
        .bind(order.customer_info.personal_info.last_name().value())
        .bind(order.customer_info.email.value())
        .bind(order.customer_info.phone.value())
        .bind(order.shipping_info.method_id.value())
        .bind(order.shipping_info.fee.amount_in_yen() as i64)
        .bind(order.shipping_info.address.postal_code())
        .bind(order.shipping_info.address.prefecture())
        .bind(order.shipping_info.address.city())
        .bind(order.shipping_info.address.street())
        .bind(order.shipping_info.address.building())
        .bind(order.payment_info.method_id.value().to_string())
        .bind(order.payment_info.fee.amount_in_yen() as i64)
        .bind(
            order
                .payment_info
                .payment_details
                .as_ref()
                .map(|d| d.to_json_string().to_string()),
        )
        .bind(order.pricing.subtotal.amount_in_yen() as i64)
        .bind(order.pricing.shipping_fee.amount_in_yen() as i64)
        .bind(order.pricing.payment_fee.amount_in_yen() as i64)
        .bind(order.pricing.tax_amount.amount_in_yen() as i64)
        .bind(order.pricing.total.amount_in_yen() as i64)
        .bind(order.status.to_string())
        .bind(order.timestamps.updated_at.to_rfc3339())
        .bind(order.notes.as_deref())
        .bind(order.id.value().to_string())
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // 既存の注文アイテムを削除
        sqlx::query("DELETE FROM order_items WHERE order_id = ?")
            .bind(order.id.value().to_string())
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // 注文アイテムを再挿入
        for item in &order.items {
            sqlx::query(
                r#"
                INSERT INTO order_items (
                    order_id, sku_id, sku_code, product_name, sku_name,
                    unit_price, quantity, subtotal
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
            )
            .bind(order.id.value().to_string())
            .bind(item.sku_id.value().to_string())
            .bind(item.sku_code.value())
            .bind(item.product_name.value())
            .bind(item.sku_name.value())
            .bind(item.unit_price.amount_in_yen() as i64)
            .bind(item.quantity)
            .bind(item.subtotal().unwrap().amount_in_yen() as i64)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        Ok(())
    }
}
