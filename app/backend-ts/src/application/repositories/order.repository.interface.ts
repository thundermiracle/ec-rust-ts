import { Order } from '$domain/aggregates';
import { OrderNumber } from '$domain/value-objects';

export interface IOrderRepository {
  // Command methods - work with entities
  generateOrderNumber(): Promise<OrderNumber>;
  save(order: Order): Promise<void>;
}
