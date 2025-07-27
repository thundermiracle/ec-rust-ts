import { CalculateCartResultDto } from '../../../application/dto';
import {
  CalculateCartResponse,
  CalculatedCartItemResponse,
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

    return new CalculateCartResponse({
      items,
      totalQuantity: dto.totalQuantity,
      itemCount: dto.itemCount,
      subtotal: dto.subtotal,
      taxAmount: dto.taxAmount,
      total: dto.total,
      isEmpty: dto.isEmpty,
      shippingFee: dto.shippingFee,
      paymentFee: dto.paymentFee,
    });
  }
}
