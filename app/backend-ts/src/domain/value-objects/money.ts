import { DomainError } from '../errors/domain.error';

export class Money {
  private readonly amountInYen: number;

  private constructor(amountInYen: number) {
    this.amountInYen = amountInYen;
  }

  static zero(): Money {
    return new Money(0);
  }

  static fromYen(yen: number): Money {
    if (yen < 0) {
      throw new DomainError('Money amount cannot be negative');
    }
    if (!Number.isInteger(yen)) {
      throw new DomainError('Money amount must be an integer');
    }
    if (yen > Number.MAX_SAFE_INTEGER) {
      throw new DomainError('Money amount exceeds maximum value');
    }
    return new Money(yen);
  }

  static fromString(amountStr: string): Money {
    const amount = parseInt(amountStr, 10);
    if (isNaN(amount)) {
      throw new DomainError('Invalid money amount format');
    }
    return Money.fromYen(amount);
  }

  add(other: Money): Money {
    const newAmount = this.amountInYen + other.amountInYen;
    if (newAmount > Number.MAX_SAFE_INTEGER) {
      throw new DomainError('Money overflow during addition');
    }
    return new Money(newAmount);
  }

  subtract(other: Money): Money {
    if (this.amountInYen < other.amountInYen) {
      throw new DomainError(
        'Cannot subtract larger amount from smaller amount',
      );
    }
    return new Money(this.amountInYen - other.amountInYen);
  }

  multiply(factor: number): Money {
    if (factor < 0) {
      throw new DomainError('Money multiplication factor cannot be negative');
    }
    const newAmount = Math.ceil(this.amountInYen * factor);
    if (newAmount > Number.MAX_SAFE_INTEGER) {
      throw new DomainError('Money overflow during multiplication');
    }
    return new Money(newAmount);
  }

  withTax(): Money {
    return this.multiply(1.1); // 10% consumption tax with ceiling
  }

  taxAmount(): Money {
    return this.withTax().subtract(this);
  }

  percentage(rate: number): Money {
    if (rate < 0 || rate > 1) {
      throw new DomainError('Percentage must be between 0.0 and 1.0');
    }
    return this.multiply(rate);
  }

  applyDiscount(discountPercent: number): Money {
    if (discountPercent < 0 || discountPercent > 100) {
      throw new DomainError('Discount percent must be between 0 and 100');
    }
    const discountRate = discountPercent / 100;
    const discountAmount = this.multiply(discountRate);
    return this.subtract(discountAmount);
  }

  isZero(): boolean {
    return this.amountInYen === 0;
  }

  isPositive(): boolean {
    return this.amountInYen > 0;
  }

  asFloat(): number {
    return this.amountInYen;
  }

  yen(): number {
    return this.amountInYen;
  }

  formatJPY(): string {
    return `¥${this.amountInYen.toLocaleString('ja-JP')}`;
  }

  toString(): string {
    return this.formatJPY();
  }

  equals(other: Money): boolean {
    return this.amountInYen === other.amountInYen;
  }
}
