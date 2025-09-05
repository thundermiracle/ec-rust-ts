import { CartItem } from '$domain/aggregates/cart/cart-item';

import { ProductId } from './identifiers';
import { Money } from './money';

export class PurchaseInfo {
  constructor(
    private readonly cartItems: CartItem[],
    private readonly subtotalAmount: Money,
    private readonly shippingFee?: Money,
    private readonly paymentFee?: Money,
  ) {}

  subtotal(): Money {
    return this.subtotalAmount;
  }

  containsProduct(productId: ProductId): boolean {
    return this.cartItems.some((item) => item.getProductId().equals(productId));
  }

  meetsMinimumAmount(minimumAmount: Money): boolean {
    return this.subtotalAmount.yen() >= minimumAmount.yen();
  }

  getCartItems(): CartItem[] {
    return [...this.cartItems];
  }

  getShippingFee(): Money | undefined {
    return this.shippingFee;
  }

  getPaymentFee(): Money | undefined {
    return this.paymentFee;
  }
}
