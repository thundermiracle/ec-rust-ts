import { Coupon, PaymentMethod, ShippingMethod } from '$domain/entities';
import { DomainError } from '$domain/errors/domain.error';
import {
  CouponDiscountService,
  DiscountResult,
} from '$domain/services/coupon-discount.service';
import { Money, PurchaseInfo, SKUId } from '$domain/value-objects';

import { CartItem } from './cart-item';

export class Cart {
  private readonly items: Map<string, CartItem> = new Map();
  private shippingMethod: ShippingMethod | null = null;
  private paymentMethod: PaymentMethod | null = null;
  private appliedCoupon: Coupon | null = null;
  private discountResult: DiscountResult | null = null;

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

  subtotalWithDiscount(): Money {
    if (this.discountResult) {
      return this.discountResult.discountedAmount;
    }
    return this.subtotal();
  }

  discountAmount(): Money {
    return this.discountResult?.discountAmount || Money.zero();
  }

  shippingFee(): Money {
    return this.shippingMethod?.getFee() || Money.zero();
  }

  paymentFee(): Money {
    return this.paymentMethod?.getFee() || Money.zero();
  }

  totalBeforeTax(): Money {
    return this.subtotalWithDiscount()
      .add(this.shippingFee())
      .add(this.paymentFee());
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

  // Coupon methods
  applyCoupon(coupon: Coupon): void {
    // Create PurchaseInfo from current cart state
    const purchaseInfo = new PurchaseInfo(
      this.getAllItems(),
      this.subtotal(),
      this.shippingFee(),
      this.paymentFee(),
    );

    // Use domain service to calculate discount
    const discountResult = CouponDiscountService.applyCoupon(
      coupon,
      purchaseInfo,
    );
    this.appliedCoupon = coupon;
    this.discountResult = discountResult;
  }

  clearCoupon(): void {
    this.appliedCoupon = null;
    this.discountResult = null;
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

  getCoupon(): Coupon | null {
    return this.appliedCoupon;
  }

  getDiscountResult(): DiscountResult | null {
    return this.discountResult;
  }

  clear(): void {
    this.items.clear();
    this.shippingMethod = null;
    this.paymentMethod = null;
    this.appliedCoupon = null;
    this.discountResult = null;
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
