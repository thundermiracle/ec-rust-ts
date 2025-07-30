import { PaymentMethodDto, PaymentMethodListDto } from '$application/dto';

import { PaymentMethodListResponse, PaymentMethodResponse } from '../responses';

export class PaymentPresenter {
  static toPaymentMethodResponse(dto: PaymentMethodDto): PaymentMethodResponse {
    return new PaymentMethodResponse({
      id: dto.id,
      name: dto.name,
      description: dto.description,
    });
  }

  static toPaymentMethodListResponse(
    dto: PaymentMethodListDto,
  ): PaymentMethodListResponse {
    const items = dto.items.map((method) =>
      this.toPaymentMethodResponse(method),
    );

    return new PaymentMethodListResponse({ items });
  }
}
