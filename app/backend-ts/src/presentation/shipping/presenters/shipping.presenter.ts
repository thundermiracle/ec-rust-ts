import {
  ShippingMethodListDto,
  ShippingMethodDto,
} from '../../../application/dto';
import {
  ShippingMethodListResponse,
  ShippingMethodResponse,
} from '../responses';

export class ShippingPresenter {
  static toShippingMethodResponse(
    dto: ShippingMethodDto,
  ): ShippingMethodResponse {
    return new ShippingMethodResponse({
      id: dto.id,
      name: dto.name,
      fee: dto.fee,
      description: dto.description,
    });
  }

  static toShippingMethodListResponse(
    dto: ShippingMethodListDto,
  ): ShippingMethodListResponse {
    const shippingMethods = dto.shippingMethods.map((method) =>
      this.toShippingMethodResponse(method),
    );

    return new ShippingMethodListResponse({ shippingMethods });
  }
}
