import { Money, ShippingMethodId } from '$domain/value-objects';

export class ShippingMethod {
  private constructor(
    private readonly id: ShippingMethodId,
    private readonly name: string,
    private readonly fee: Money,
    private readonly description: string | undefined,
    private readonly isActive: boolean,
  ) {}

  static create(
    id: ShippingMethodId,
    name: string,
    fee: Money,
    description: string | undefined,
    isActive: boolean,
  ): ShippingMethod {
    return new ShippingMethod(id, name, fee, description, isActive);
  }

  getId(): ShippingMethodId {
    return this.id;
  }

  getName(): string {
    return this.name;
  }

  getFee(): Money {
    return this.fee;
  }

  getDescription(): string | undefined {
    return this.description;
  }

  isAvailable(): boolean {
    return this.isActive;
  }
}
