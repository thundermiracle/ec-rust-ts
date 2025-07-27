import { Money } from '../../value-objects/money';

export class OrderPricing {
  private constructor(
    private readonly subtotal: Money,
    private readonly shippingFee: Money,
    private readonly paymentFee: Money,
    private readonly taxAmount: Money,
    private readonly total: Money,
  ) {}

  static calculate(
    subtotal: Money,
    shippingFee: Money,
    paymentFee: Money,
  ): OrderPricing {
    const beforeTax = subtotal.add(shippingFee).add(paymentFee);
    const taxAmount = beforeTax.taxAmount();
    const total = beforeTax.withTax();

    return new OrderPricing(
      subtotal,
      shippingFee,
      paymentFee,
      taxAmount,
      total,
    );
  }

  // Getters
  getSubtotal(): Money {
    return this.subtotal;
  }

  getShippingFee(): Money {
    return this.shippingFee;
  }

  getPaymentFee(): Money {
    return this.paymentFee;
  }

  getTaxAmount(): Money {
    return this.taxAmount;
  }

  getTotal(): Money {
    return this.total;
  }

  getTotalBeforeTax(): Money {
    return this.subtotal.add(this.shippingFee).add(this.paymentFee);
  }
}
