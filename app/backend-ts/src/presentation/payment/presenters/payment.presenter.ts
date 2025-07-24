// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { IPresenter } from '../../base';
import {
  PaymentMethodListDto,
  PaymentMethodDto,
} from '../../../application/dto';
import { PaymentMethodListResponse, PaymentMethodResponse } from '../responses';

export class PaymentPresenter {
  static toPaymentMethodResponse(dto: PaymentMethodDto): PaymentMethodResponse {
    return new PaymentMethodResponse({
      id: dto.id,
      name: dto.name,
      fee: dto.fee,
      description: dto.description,
    });
  }

  static toPaymentMethodListResponse(
    dto: PaymentMethodListDto,
  ): PaymentMethodListResponse {
    const paymentMethods = dto.paymentMethods.map((method) =>
      this.toPaymentMethodResponse(method),
    );

    return new PaymentMethodListResponse({ paymentMethods });
  }
}
