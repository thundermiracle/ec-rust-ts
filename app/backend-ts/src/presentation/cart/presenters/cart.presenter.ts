import { IPresenter } from '../../base';
import { CalculateCartResultDto } from '../../../application/dto';
import {
  CalculateCartResponse,
  CalculatedCartItemResponse,
} from '../responses';

export class CartPresenter
  implements IPresenter<CalculateCartResultDto, CalculateCartResponse>
{
  static toCalculateCartResponse(
    dto: CalculateCartResultDto,
  ): CalculateCartResponse {
    const items = dto.items.map(
      (item) =>
        new CalculatedCartItemResponse({
          skuId: item.skuId,
          productId: item.productId,
          productName: item.productName,
          skuName: item.skuName,
          unitPrice: item.unitPrice,
          quantity: item.quantity,
          subtotal: item.subtotal,
        }),
    );

    return new CalculateCartResponse({
      items,
      subtotal: dto.subtotal,
      shippingFee: dto.shippingFee,
      paymentFee: dto.paymentFee,
      taxAmount: dto.taxAmount,
      total: dto.total,
      shippingMethodId: dto.shippingMethodId,
      shippingMethodName: dto.shippingMethodName,
      paymentMethodId: dto.paymentMethodId,
      paymentMethodName: dto.paymentMethodName,
    });
  }
}
