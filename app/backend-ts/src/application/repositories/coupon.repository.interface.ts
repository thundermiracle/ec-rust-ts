import { Coupon } from '$domain/entities/coupon';
import { CouponCode } from '$domain/value-objects/coupon-code';

export interface ICouponRepository {
  findByCode(code: CouponCode): Promise<Coupon | null>;
  updateUsageCount(coupon: Coupon): Promise<void>;
}
