import { CouponCode } from '$domain/value-objects/coupon-code';
import { DiscountPolicy } from '$domain/value-objects/discount-policy';
import { CouponId } from '$domain/value-objects/identifiers';

export class Coupon {
  constructor(
    private readonly id: CouponId,
    private readonly code: CouponCode,
    private readonly name: string,
    private readonly description: string | undefined,
    private readonly discountPolicy: DiscountPolicy,
    private readonly validFrom: Date,
    private readonly validUntil: Date,
    private readonly usageLimit: number | undefined,
    private readonly usageCount: number,
  ) {}

  isValid(): boolean {
    const now = new Date();
    return this.validFrom <= now && now <= this.validUntil;
  }

  isValidUsageLimit(): boolean {
    return this.usageLimit === undefined || this.usageCount < this.usageLimit;
  }

  getId(): CouponId {
    return this.id;
  }

  getCode(): CouponCode {
    return this.code;
  }

  getName(): string {
    return this.name;
  }

  getDescription(): string | undefined {
    return this.description;
  }

  getDiscountPolicy(): DiscountPolicy {
    return this.discountPolicy;
  }

  getValidFrom(): Date {
    return this.validFrom;
  }

  getValidUntil(): Date {
    return this.validUntil;
  }

  getUsageLimit(): number | undefined {
    return this.usageLimit;
  }

  getUsageCount(): number {
    return this.usageCount;
  }
}
