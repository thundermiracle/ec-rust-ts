import { PaymentMethodId } from '../../value-objects/identifiers';
import { Money } from '../../value-objects/money';
import { DomainError } from '../../errors/domain.error';

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

  // 支払方法の検証ロジックを追加
  static createFromMethod(paymentMethodData: {
    id: string;
    name: string;
    fee: Money;
    isActive: boolean;
  }): PaymentInfo {
    if (!paymentMethodData.isActive) {
      throw new DomainError(
        `Payment method ${paymentMethodData.id} is not available`,
      );
    }

    const methodId = PaymentMethodId.new(paymentMethodData.id);

    return new PaymentInfo(
      methodId,
      paymentMethodData.name,
      paymentMethodData.fee,
    );
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
