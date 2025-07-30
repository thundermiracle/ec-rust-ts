import { Money, PaymentMethodId } from '$domain/value-objects';

export class PaymentMethod {
  private constructor(
    private readonly id: PaymentMethodId,
    private readonly name: string,
    private readonly fee: Money,
    private readonly description: string | undefined,
    private readonly isActive: boolean,
  ) {}

  static create(
    id: PaymentMethodId,
    name: string,
    fee: Money,
    description: string | undefined,
    isActive: boolean,
  ): PaymentMethod {
    return new PaymentMethod(id, name, fee, description, isActive);
  }

  getId(): PaymentMethodId {
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
