import { Coupon } from '$domain/entities/coupon';
import {
  CouponCode,
  CouponId,
  DiscountPolicy,
  FixedAmountDiscount,
  MinimumPurchaseCondition,
  Money,
  PercentageDiscount,
} from '$domain/value-objects';

import { CouponEntity } from '../entities/coupon.entity';

export class CouponMapper {
  static toDomain(entity: CouponEntity): Coupon {
    const discountType =
      entity.discount_type === 'percentage'
        ? new PercentageDiscount(entity.discount_value)
        : new FixedAmountDiscount(Money.fromYen(entity.discount_value));

    const discountCondition = entity.minimum_amount
      ? new MinimumPurchaseCondition(Money.fromYen(entity.minimum_amount))
      : undefined;

    const discountPolicy = new DiscountPolicy(discountType, discountCondition);

    return new Coupon(
      CouponId.fromUuid(entity.id),
      CouponCode.fromString(entity.code),
      entity.name,
      entity.description,
      discountPolicy,
      new Date(`${entity.valid_from}T00:00:00Z`),
      new Date(`${entity.valid_until}T23:59:59Z`),
      entity.usage_limit,
      entity.used_count,
    );
  }

  static toEntity(domain: Coupon): CouponEntity {
    const entity = new CouponEntity();
    entity.id = domain.getId().value();
    entity.code = domain.getCode().getValue();
    entity.name = domain.getName();
    entity.description = domain.getDescription();

    const discountType = domain.getDiscountPolicy().getDiscountType();
    if (discountType instanceof PercentageDiscount) {
      entity.discount_type = 'percentage';
      entity.discount_value = discountType.getPercentage();
    } else if (discountType instanceof FixedAmountDiscount) {
      entity.discount_type = 'fixed_amount';
      entity.discount_value = discountType.getAmount().yen();
    }

    const condition = domain.getDiscountPolicy().getDiscountCondition();
    if (condition instanceof MinimumPurchaseCondition) {
      entity.minimum_amount = condition.getMinimumAmount().yen();
    }

    entity.usage_limit = domain.getUsageLimit();
    entity.used_count = domain.getUsageCount();
    entity.valid_from = domain.getValidFrom().toISOString().split('T')[0];
    entity.valid_until = domain.getValidUntil().toISOString().split('T')[0];

    return entity;
  }
}
