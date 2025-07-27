import { SKUId, ProductId } from '../../value-objects/identifiers';
import { Money } from '../../value-objects/money';
import { DomainError } from '../../errors/domain.error';

export class CartItem {
  private constructor(
    private readonly skuId: SKUId,
    private readonly productId: ProductId,
    private readonly productName: string,
    private readonly unitPrice: Money,
    private quantity: number,
  ) {}

  static create(
    skuId: SKUId,
    productId: ProductId,
    productName: string,
    unitPrice: Money,
    quantity: number,
  ): CartItem {
    if (!productName || productName.trim().length === 0) {
      throw new DomainError('Product name cannot be empty');
    }
    if (!unitPrice.isPositive()) {
      throw new DomainError('Unit price must be positive');
    }
    if (quantity <= 0) {
      throw new DomainError('Quantity must be positive');
    }
    if (quantity > 999) {
      throw new DomainError('Quantity cannot exceed 999');
    }

    return new CartItem(
      skuId,
      productId,
      productName.trim(),
      unitPrice,
      quantity,
    );
  }

  subtotal(): Money {
    return this.unitPrice.multiply(this.quantity);
  }

  updateQuantity(newQuantity: number): void {
    if (newQuantity <= 0) {
      throw new DomainError('Quantity must be positive');
    }
    if (newQuantity > 999) {
      throw new DomainError('Quantity cannot exceed 999');
    }
    this.quantity = newQuantity;
  }

  increaseQuantity(additional: number): void {
    if (additional <= 0) {
      throw new DomainError('Additional quantity must be positive');
    }
    const newQuantity = this.quantity + additional;
    if (newQuantity > 999) {
      throw new DomainError('Total quantity cannot exceed 999');
    }
    this.quantity = newQuantity;
  }

  decreaseQuantity(reduction: number): void {
    if (reduction <= 0) {
      throw new DomainError('Reduction quantity must be positive');
    }
    if (reduction >= this.quantity) {
      throw new DomainError('Cannot reduce quantity below 1');
    }
    this.quantity -= reduction;
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

  getUnitPrice(): Money {
    return this.unitPrice;
  }

  getQuantity(): number {
    return this.quantity;
  }

  equals(other: CartItem): boolean {
    return this.skuId.equals(other.skuId);
  }
}
