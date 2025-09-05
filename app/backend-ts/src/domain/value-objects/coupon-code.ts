import { DomainError } from '$domain/errors/domain.error';

export class CouponCode {
  private static readonly MAX_LENGTH = 10;

  private constructor(private readonly value: string) {}

  static fromString(value: string): CouponCode {
    if (!value || value.trim().length === 0) {
      throw new DomainError('Coupon code cannot be empty');
    }

    if (value.length > this.MAX_LENGTH) {
      throw new DomainError(
        `Coupon code cannot exceed ${this.MAX_LENGTH} characters`,
      );
    }

    return new CouponCode(value);
  }

  getValue(): string {
    return this.value;
  }

  toString(): string {
    return this.value;
  }

  equals(other: CouponCode): boolean {
    return this.value === other.getValue();
  }
}
