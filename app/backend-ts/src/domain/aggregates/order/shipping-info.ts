import { ShippingMethodId } from '../../value-objects/identifiers';
import { Address } from '../../value-objects/address';
import { Money } from '../../value-objects/money';

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
