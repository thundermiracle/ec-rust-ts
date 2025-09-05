import { DiscountCondition } from './discount-condition';
import { DiscountType } from './discount-type';

export class DiscountPolicy {
  constructor(
    private readonly discountType: DiscountType,
    private readonly discountCondition?: DiscountCondition,
  ) {}

  static unconditional(discountType: DiscountType): DiscountPolicy {
    return new DiscountPolicy(discountType);
  }

  hasCondition(): boolean {
    return this.discountCondition !== undefined;
  }

  getDiscountType(): DiscountType {
    return this.discountType;
  }

  getDiscountCondition(): DiscountCondition | undefined {
    return this.discountCondition;
  }

  equals(other: DiscountPolicy): boolean {
    const typeEquals =
      this.discountType.getType() === other.discountType.getType();
    const conditionEquals =
      (this.discountCondition === undefined &&
        other.discountCondition === undefined) ||
      (this.discountCondition !== undefined &&
        other.discountCondition !== undefined &&
        this.discountCondition.getType() === other.discountCondition.getType());

    return typeEquals && conditionEquals;
  }
}
