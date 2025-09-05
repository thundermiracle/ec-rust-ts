export class AppliedCouponDto {
  constructor(
    public readonly couponCode: string,
    public readonly couponName: string,
    public readonly discountAmount: number,
    public readonly message: string,
  ) {}
}

export class CouponErrorDto {
  constructor(
    public readonly couponCode: string | undefined,
    public readonly errorMessage: string,
  ) {}
}
