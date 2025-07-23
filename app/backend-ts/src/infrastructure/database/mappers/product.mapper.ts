import { Product, SKU } from '../../../domain/entities';
import {
  ProductId,
  CategoryId,
  SKUId,
  ColorId,
  Money,
} from '../../../domain/value-objects';
import { ProductEntity } from '../entities/product.entity';
import { SkuEntity } from '../entities/sku.entity';

export class ProductMapper {
  static toDomain(entity: ProductEntity): Product {
    const product = Product.create(
      ProductId.fromUuid(entity.id),
      entity.name,
      entity.description,
      CategoryId.fromUuid(entity.category_id),
    );

    // Set additional properties using reflection or setters
    // Note: This would require making some Product properties mutable
    // or adding factory methods with all parameters

    if (entity.skus) {
      entity.skus.forEach((skuEntity) => {
        const sku = SkuMapper.toDomain(skuEntity);
        product.addSku(sku);
      });
    }

    return product;
  }

  static toEntity(domain: Product): ProductEntity {
    const entity = new ProductEntity();
    entity.id = domain.getId().value();
    entity.name = domain.getName();
    entity.description = domain.getDescription();
    entity.category_id = domain.getCategoryId().value();
    entity.is_best_seller = domain.getIsBestSeller();
    entity.is_quick_ship = domain.getIsQuickShip();
    entity.is_available = domain.getIsAvailable();
    entity.created_at = domain.getCreatedAt();
    entity.updated_at = domain.getUpdatedAt();
    return entity;
  }
}

export class SkuMapper {
  static toDomain(entity: SkuEntity): SKU {
    const variantAttributes = {
      colorId: entity.color_id ? ColorId.new(entity.color_id) : undefined,
      dimensions: entity.dimensions,
      material: entity.material,
    };

    const basePrice = Money.fromYen(entity.base_price);
    const initialStock = entity.stock_quantity;

    const sku = SKU.create(
      SKUId.fromUuid(entity.id),
      ProductId.fromUuid(entity.product_id),
      entity.sku_code,
      entity.name,
      variantAttributes,
      basePrice,
      initialStock,
      entity.display_order,
    );

    // Set sale price if exists
    if (entity.sale_price) {
      sku.setSalePrice(Money.fromYen(entity.sale_price));
    }

    // Set status
    switch (entity.status) {
      case 'Active':
        sku.activate();
        break;
      case 'Inactive':
        sku.deactivate();
        break;
      case 'Discontinued':
        sku.discontinue();
        break;
    }

    // Adjust stock for reserved quantity
    if (entity.reserved_quantity > 0) {
      sku.reserveStock(entity.reserved_quantity);
    }

    return sku;
  }

  static toEntity(domain: SKU): SkuEntity {
    const entity = new SkuEntity();
    entity.id = domain.getId().value();
    entity.product_id = domain.getProductId().value();
    entity.sku_code = domain.code();
    entity.name = domain.getName();
    entity.base_price = domain.getBasePrice().yen();
    entity.sale_price = domain.getSalePrice()?.yen();
    entity.stock_quantity = domain.getStock().getTotalQuantity();
    entity.reserved_quantity = domain.getStock().getReservedQuantity();
    entity.status = domain.getStatus().toString();
    entity.display_order = domain.getDisplayOrder();
    entity.color_id = domain.colorId()?.value();
    entity.dimensions = domain.dimensions();
    entity.material = domain.material();
    return entity;
  }
}
