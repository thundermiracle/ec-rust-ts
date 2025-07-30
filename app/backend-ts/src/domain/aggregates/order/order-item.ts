import { DomainError } from '$domain/errors/domain.error';
import { Money, ProductId, SKUId } from '$domain/value-objects';

export class OrderItem {
  private constructor(
    private readonly skuId: SKUId,
    private readonly productId: ProductId,
    private readonly productName: string,
    private readonly skuName: string,
    private readonly unitPrice: Money,
    private readonly quantity: number,
  ) {}

  static create(
    skuId: SKUId,
    productId: ProductId,
    productName: string,
    skuName: string,
    unitPrice: Money,
    quantity: number,
  ): OrderItem {
    if (!productName || productName.trim().length === 0) {
      throw new DomainError('Product name cannot be empty');
    }
    if (!skuName || skuName.trim().length === 0) {
      throw new DomainError('SKU name cannot be empty');
    }
    if (!unitPrice.isPositive()) {
      throw new DomainError('Unit price must be positive');
    }
    if (quantity <= 0) {
      throw new DomainError('Quantity must be positive');
    }

    return new OrderItem(
      skuId,
      productId,
      productName.trim(),
      skuName.trim(),
      unitPrice,
      quantity,
    );
  }

  subtotal(): Money {
    return this.unitPrice.multiply(this.quantity);
  }

  // Getters
  getSkuId(): SKUId {
    return this.skuId;
  }

  getProductId(): ProductId {
    return this.productId;
  }

  getProductName(): string {
    return this.productName;
  }

  getSkuName(): string {
    return this.skuName;
  }

  getUnitPrice(): Money {
    return this.unitPrice;
  }

  getQuantity(): number {
    return this.quantity;
  }

  getFullDisplayName(): string {
    return `${this.productName} - ${this.skuName}`;
  }
}
