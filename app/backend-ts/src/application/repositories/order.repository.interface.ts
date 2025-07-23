import { Order } from '../../domain/aggregates';
import { OrderId, OrderNumber } from '../../domain/value-objects';

export interface IOrderRepository {
  findById(id: OrderId): Promise<Order | null>;
  findByOrderNumber(orderNumber: OrderNumber): Promise<Order | null>;
  findAll(): Promise<Order[]>;
  generateOrderNumber(): Promise<OrderNumber>;
  save(order: Order): Promise<void>;
  update(order: Order): Promise<void>;
  delete(id: OrderId): Promise<void>;
}
