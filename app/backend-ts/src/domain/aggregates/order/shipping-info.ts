import { ShippingMethodId } from '../../value-objects/identifiers';
import { Address } from '../../value-objects/address';
import { Money } from '../../value-objects/money';
import { DomainError } from '../../errors/domain.error';

export class ShippingInfo {
  private constructor(
    private readonly shippingMethodId: ShippingMethodId,
    private readonly shippingMethodName: string,
    private readonly shippingFee: Money,
    private readonly shippingAddress: Address,
  ) {}

  static create(
    shippingMethodId: ShippingMethodId,
    shippingMethodName: string,
    shippingFee: Money,
    shippingAddress: Address,
  ): ShippingInfo {
    return new ShippingInfo(
      shippingMethodId,
      shippingMethodName,
      shippingFee,
      shippingAddress,
    );
  }

  // 配送方法の検証ロジックを追加
  static createFromMethod(
    shippingMethodData: {
      id: string;
      name: string;
      fee: Money;
      isActive: boolean;
    },
    address: Address,
  ): ShippingInfo {
    if (!shippingMethodData.isActive) {
      throw new DomainError(
        `Shipping method ${shippingMethodData.id} is not available`,
      );
    }

    const methodId = ShippingMethodId.new(shippingMethodData.id);

    return new ShippingInfo(
      methodId,
      shippingMethodData.name,
      shippingMethodData.fee,
      address,
    );
  }

  // Getters
  getShippingMethodId(): ShippingMethodId {
    return this.shippingMethodId;
  }

  getShippingMethodName(): string {
    return this.shippingMethodName;
  }

  getShippingFee(): Money {
    return this.shippingFee;
  }

  getShippingAddress(): Address {
    return this.shippingAddress;
  }
}
