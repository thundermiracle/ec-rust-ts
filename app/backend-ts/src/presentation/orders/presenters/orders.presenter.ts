import { CreateOrderResultDto } from '../../../application/dto';
import { CreateOrderResponse } from '../responses';

export class OrdersPresenter {
  static toCreateOrderResponse(dto: CreateOrderResultDto): CreateOrderResponse {
    return new CreateOrderResponse({
      orderId: dto.orderId,
      orderNumber: dto.orderNumber,
      total: dto.total,
      status: dto.status,
      createdAt: dto.createdAt,
    });
  }
}
