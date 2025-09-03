import { AppliedCouponDto, CouponErrorDto } from './applied-coupon.dto';

export class CalculatedCartItemDto {
  constructor(
    public readonly skuId: string,
    public readonly productId: string,
    public readonly productName: string,
    public readonly unitPrice: number,
    public readonly quantity: number,
    public readonly subtotal: number,
  ) {}
}

export class CalculateCartResultDto {
  constructor(
    public readonly items: CalculatedCartItemDto[],
    public readonly totalQuantity: number,
    public readonly itemCount: number,
    public readonly subtotal: number,
    public readonly taxAmount: number,
    public readonly subtotalWithTax: number,
    public readonly total: number,
    public readonly isEmpty: boolean,
    public readonly shippingFee: number,
    public readonly paymentFee: number,
    public readonly appliedCoupon?: AppliedCouponDto,
    public readonly couponError?: CouponErrorDto,
  ) {}
}
