import { v4 as uuidv4 } from 'uuid';

import { IdentifierError } from '$domain/errors/domain.error';

// UUID-based identifiers
abstract class UuidIdentifier {
  protected constructor(private readonly id: string) {
    if (!this.isValidUuid(id)) {
      throw new IdentifierError('Invalid UUID format');
    }
  }

  private isValidUuid(uuid: string): boolean {
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
    return uuidRegex.test(uuid);
  }

  value(): string {
    return this.id;
  }

  toString(): string {
    return this.id;
  }

  equals(other: UuidIdentifier): boolean {
    return this.id === other.value();
  }
}

export class ProductId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): ProductId {
    return new ProductId(uuidv4());
  }

  static fromUuid(uuid: string): ProductId {
    return new ProductId(uuid);
  }
}

export class SKUId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): SKUId {
    return new SKUId(uuidv4());
  }

  static fromUuid(uuid: string): SKUId {
    return new SKUId(uuid);
  }
}

export class DeliveryInfoId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): DeliveryInfoId {
    return new DeliveryInfoId(uuidv4());
  }

  static fromUuid(uuid: string): DeliveryInfoId {
    return new DeliveryInfoId(uuid);
  }
}

export class OrderId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): OrderId {
    return new OrderId(uuidv4());
  }

  static fromUuid(uuid: string): OrderId {
    return new OrderId(uuid);
  }
}

export class CustomerId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): CustomerId {
    return new CustomerId(uuidv4());
  }

  static fromUuid(uuid: string): CustomerId {
    return new CustomerId(uuid);
  }
}

export class CategoryId extends UuidIdentifier {
  private constructor(id: string) {
    super(id);
  }

  static new(): CategoryId {
    return new CategoryId(uuidv4());
  }

  static fromUuid(uuid: string): CategoryId {
    return new CategoryId(uuid);
  }
}

export class ColorId {
  private constructor(private readonly id: number) {
    if (id <= 0) {
      throw new IdentifierError('ColorId cannot be zero or negative');
    }
    if (!Number.isInteger(id)) {
      throw new IdentifierError('ColorId must be an integer');
    }
  }

  static new(id: number): ColorId {
    return new ColorId(id);
  }

  value(): number {
    return this.id;
  }

  toString(): string {
    return this.id.toString();
  }

  equals(other: ColorId): boolean {
    return this.id === other.value();
  }
}

// String-based identifiers
export class ShippingMethodId {
  private constructor(private readonly id: string) {
    if (!id || id.trim().length === 0) {
      throw new IdentifierError('ShippingMethodId cannot be empty');
    }
    if (id.length > 50) {
      throw new IdentifierError('ShippingMethodId cannot exceed 50 characters');
    }
  }

  static new(id: string): ShippingMethodId {
    return new ShippingMethodId(id);
  }

  value(): string {
    return this.id;
  }

  toString(): string {
    return this.id;
  }

  equals(other: ShippingMethodId): boolean {
    return this.id === other.value();
  }
}

export class PaymentMethodId {
  private constructor(private readonly id: string) {
    if (!id || id.trim().length === 0) {
      throw new IdentifierError('PaymentMethodId cannot be empty');
    }
    if (id.length > 50) {
      throw new IdentifierError('PaymentMethodId cannot exceed 50 characters');
    }
  }

  static new(id: string): PaymentMethodId {
    return new PaymentMethodId(id);
  }

  value(): string {
    return this.id;
  }

  toString(): string {
    return this.id;
  }

  equals(other: PaymentMethodId): boolean {
    return this.id === other.value();
  }
}
