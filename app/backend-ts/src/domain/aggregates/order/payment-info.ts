import { PaymentMethodId } from '../../value-objects/identifiers';
import { Money } from '../../value-objects/money';

export class PaymentInfo {
  private constructor(
    private readonly paymentMethodId: PaymentMethodId,
    private readonly paymentMethodName: string,
    private readonly paymentFee: Money,
  ) {}

  static create(
    paymentMethodId: PaymentMethodId,
    paymentMethodName: string,
    paymentFee: Money,
  ): PaymentInfo {
    return new PaymentInfo(paymentMethodId, paymentMethodName, paymentFee);
  }

  // Getters
  getPaymentMethodId(): PaymentMethodId {
    return this.paymentMethodId;
  }

  getPaymentMethodName(): string {
    return this.paymentMethodName;
  }

  getPaymentFee(): Money {
    return this.paymentFee;
  }
}
