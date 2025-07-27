import { CreateOrderResultDto } from '../../../application/dto';
import { CreateOrderResponse } from '../responses';

export class OrdersPresenter {
  static toCreateOrderResponse(dto: CreateOrderResultDto): CreateOrderResponse {
    return new CreateOrderResponse({
      order_id: dto.orderId,
      order_number: dto.orderNumber,
      total_amount: dto.total,
      status: dto.status,
    });
  }
}
