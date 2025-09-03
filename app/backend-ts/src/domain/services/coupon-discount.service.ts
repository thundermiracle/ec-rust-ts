import { Coupon } from '$domain/entities/coupon';
import { DomainError } from '$domain/errors/domain.error';
import {
  CategorySpecificCondition,
  MinimumPurchaseCondition,
  ProductSpecificCondition,
} from '$domain/value-objects/discount-condition';
import {
  FixedAmountDiscount,
  PercentageDiscount,
} from '$domain/value-objects/discount-type';
import { ProductId } from '$domain/value-objects/identifiers';
import { Money } from '$domain/value-objects/money';
import { PurchaseInfo } from '$domain/value-objects/purchase-info';

export interface DiscountResult {
  discountAmount: Money;
  discountedAmount: Money;
  message: string;
}

export class CouponDiscountService {
  static applyCoupon(
    coupon: Coupon,
    purchaseInfo: PurchaseInfo,
  ): DiscountResult {
    this.validateCoupon(coupon);
    this.checkConditions(coupon, purchaseInfo);

    const discountAmount = this.calculateDiscount(coupon, purchaseInfo);
    const originalAmount = purchaseInfo.subtotal();
    const discountedAmount = originalAmount.subtract(discountAmount);

    const message = `クーポン「${coupon.getName()}」が適用されました。割引額: ${discountAmount.formatJPY()}`;

    return {
      discountAmount,
      discountedAmount,
      message,
    };
  }

  private static validateCoupon(coupon: Coupon): void {
    if (!coupon.isValid()) {
      throw new DomainError('クーポンの有効期限が切れています');
    }

    if (!coupon.isValidUsageLimit()) {
      throw new DomainError('クーポンの使用回数が上限に達しています');
    }
  }

  private static checkConditions(
    coupon: Coupon,
    purchaseInfo: PurchaseInfo,
  ): void {
    const condition = coupon.getDiscountPolicy().getDiscountCondition();

    if (!condition) {
      return;
    }

    if (condition instanceof MinimumPurchaseCondition) {
      if (!purchaseInfo.meetsMinimumAmount(condition.getMinimumAmount())) {
        throw new DomainError(
          `最低購入金額${condition.getMinimumAmount().formatJPY()}に満たないため、クーポンを適用できません`,
        );
      }
    } else if (condition instanceof ProductSpecificCondition) {
      if (!this.hasTargetProducts(purchaseInfo, condition.getProductIds())) {
        throw new DomainError('対象商品がカートに含まれていません');
      }
    } else if (condition instanceof CategorySpecificCondition) {
      throw new DomainError(
        'カテゴリ指定のクーポンは現在サポートされていません',
      );
    }
  }

  private static hasTargetProducts(
    purchaseInfo: PurchaseInfo,
    targetProductIds: ProductId[],
  ): boolean {
    return targetProductIds.some((productId) =>
      purchaseInfo.containsProduct(productId),
    );
  }

  private static calculateDiscount(
    coupon: Coupon,
    purchaseInfo: PurchaseInfo,
  ): Money {
    const baseAmount = purchaseInfo.subtotal();
    const discountType = coupon.getDiscountPolicy().getDiscountType();

    if (discountType instanceof FixedAmountDiscount) {
      const amount = discountType.getAmount();
      if (amount.yen() > baseAmount.yen()) {
        return baseAmount;
      } else {
        return amount;
      }
    } else if (discountType instanceof PercentageDiscount) {
      const percentage = discountType.getPercentage();
      return baseAmount.subtract(baseAmount.applyDiscount(percentage));
    } else {
      throw new DomainError('Unknown discount type');
    }
  }
}
