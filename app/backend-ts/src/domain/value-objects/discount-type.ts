import { Money } from './money';

export abstract class DiscountType {
  abstract getType(): string;
}

export class FixedAmountDiscount extends DiscountType {
  constructor(private readonly amount: Money) {
    super();
  }

  getAmount(): Money {
    return this.amount;
  }

  getType(): string {
    return 'FixedAmount';
  }

  equals(other: FixedAmountDiscount): boolean {
    return this.amount.equals(other.amount);
  }
}

export class PercentageDiscount extends DiscountType {
  constructor(private readonly percentage: number) {
    super();
    if (percentage < 0 || percentage > 100) {
      throw new Error('Percentage must be between 0 and 100');
    }
  }

  getPercentage(): number {
    return this.percentage;
  }

  getType(): string {
    return 'Percentage';
  }

  equals(other: PercentageDiscount): boolean {
    return this.percentage === other.percentage;
  }
}
