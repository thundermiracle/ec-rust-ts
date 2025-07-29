import { SKUId } from '../../value-objects/identifiers';
import { Money } from '../../value-objects/money';
import { DomainError } from '../../errors/domain.error';
import { CartItem } from './cart-item';
import { ShippingMethod, PaymentMethod } from '../../entities';

export class Cart {
  private readonly items: Map<string, CartItem> = new Map();
  private shippingMethod: ShippingMethod | null = null;
  private paymentMethod: PaymentMethod | null = null;

  constructor() {}

  // Item management
  addItem(item: CartItem): void {
    const skuId = item.getSkuId().value();
    const existingItem = this.items.get(skuId);

    if (existingItem) {
      // Consolidate by increasing quantity
      existingItem.increaseQuantity(item.getQuantity());
    } else {
      this.items.set(skuId, item);
    }
  }

  removeItem(skuId: SKUId): void {
    this.items.delete(skuId.value());
  }

  updateItemQuantity(skuId: SKUId, quantity: number): void {
    const item = this.items.get(skuId.value());
    if (!item) {
      throw new DomainError('Item not found in cart');
    }

    if (quantity <= 0) {
      this.removeItem(skuId);
    } else {
      item.updateQuantity(quantity);
    }
  }

  // Calculations
  subtotal(): Money {
    if (this.items.size === 0) {
      return Money.zero();
    }

    return Array.from(this.items.values())
      .map((item) => item.subtotal())
      .reduce((total, itemSubtotal) => total.add(itemSubtotal));
  }

  shippingFee(): Money {
    return this.shippingMethod?.getFee() || Money.zero();
  }

  paymentFee(): Money {
    return this.paymentMethod?.getFee() || Money.zero();
  }

  totalBeforeTax(): Money {
    return this.subtotal().add(this.shippingFee()).add(this.paymentFee());
  }

  taxAmount(): Money {
    return this.totalBeforeTax().taxAmount();
  }

  total(): Money {
    return this.totalBeforeTax().withTax();
  }

  // Shipping & Payment methods
  applyShippingMethod(method: ShippingMethod): void {
    this.shippingMethod = method;
  }

  applyPaymentMethod(method: PaymentMethod): void {
    this.paymentMethod = method;
  }

  clearShippingMethod(): void {
    this.shippingMethod = null;
  }

  clearPaymentMethod(): void {
    this.paymentMethod = null;
  }

  // Utility methods
  totalQuantity(): number {
    return Array.from(this.items.values()).reduce(
      (total, item) => total + item.getQuantity(),
      0,
    );
  }

  itemCount(): number {
    return this.items.size;
  }

  isEmpty(): boolean {
    return this.items.size === 0;
  }

  containsSku(skuId: SKUId): boolean {
    return this.items.has(skuId.value());
  }

  getItem(skuId: SKUId): CartItem | null {
    return this.items.get(skuId.value()) || null;
  }

  getAllItems(): CartItem[] {
    return Array.from(this.items.values());
  }

  // Getters
  getShippingMethod(): ShippingMethod | null {
    return this.shippingMethod;
  }

  getPaymentMethod(): PaymentMethod | null {
    return this.paymentMethod;
  }

  clear(): void {
    this.items.clear();
    this.shippingMethod = null;
    this.paymentMethod = null;
  }

  // Validation
  validateForCheckout(): void {
    if (this.isEmpty()) {
      throw new DomainError('Cart is empty');
    }
    if (!this.shippingMethod) {
      throw new DomainError('Shipping method is required');
    }
    if (!this.paymentMethod) {
      throw new DomainError('Payment method is required');
    }
  }
}
