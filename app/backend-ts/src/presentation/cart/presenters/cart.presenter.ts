import { CalculateCartResultDto } from '$application/dto';

import {
  AppliedCouponResponse,
  CalculateCartResponse,
  CalculatedCartItemResponse,
  CouponErrorResponse,
} from '../responses';

export class CartPresenter {
  static toCalculateCartResponse(
    dto: CalculateCartResultDto,
  ): CalculateCartResponse {
    const items = dto.items.map(
      (item) =>
        new CalculatedCartItemResponse({
          skuId: item.skuId,
          productId: item.productId,
          productName: item.productName,
          unitPrice: item.unitPrice,
          quantity: item.quantity,
          subtotal: item.subtotal,
        }),
    );

    const appliedCoupon = dto.appliedCoupon
      ? new AppliedCouponResponse({
          couponCode: dto.appliedCoupon.couponCode,
          couponName: dto.appliedCoupon.couponName,
          discountAmount: dto.appliedCoupon.discountAmount,
          message: dto.appliedCoupon.message,
        })
      : undefined;

    const couponError = dto.couponError
      ? new CouponErrorResponse({
          couponCode: dto.couponError.couponCode,
          errorMessage: dto.couponError.errorMessage,
        })
      : undefined;

    return new CalculateCartResponse({
      items,
      totalQuantity: dto.totalQuantity,
      itemCount: dto.itemCount,
      subtotal: dto.subtotal,
      taxAmount: dto.taxAmount,
      subtotalWithTax: dto.subtotalWithTax,
      total: dto.total,
      isEmpty: dto.isEmpty,
      shippingFee: dto.shippingFee,
      paymentFee: dto.paymentFee,
      appliedCoupon,
      couponError,
    });
  }
}
