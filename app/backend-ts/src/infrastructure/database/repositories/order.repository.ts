import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { IOrderRepository } from '../../../application/repositories/order.repository.interface';
import { Order } from '../../../domain/aggregates';
import { OrderNumber } from '../../../domain/value-objects';
import { OrderEntity } from '../entities/order.entity';
import { OrderItemEntity } from '../entities/order-item.entity';
import { OrderMapper, OrderItemMapper } from '../mappers/order.mapper';

@Injectable()
export class OrderRepository implements IOrderRepository {
  constructor(
    @InjectRepository(OrderEntity)
    private readonly orderRepository: Repository<OrderEntity>,
    @InjectRepository(OrderItemEntity)
    private readonly orderItemRepository: Repository<OrderItemEntity>,
  ) {}

  async generateOrderNumber(): Promise<OrderNumber> {
    // Simple implementation - in production, you might want to ensure uniqueness
    return Promise.resolve(OrderNumber.generate());
  }

  async save(order: Order): Promise<void> {
    const orderEntity = OrderMapper.toEntity(order);
    const savedOrder = await this.orderRepository.save(orderEntity);

    // Save order items
    const orderItems = order
      .getItems()
      .map((item) => OrderItemMapper.toEntity(item, savedOrder.id));

    if (orderItems.length > 0) {
      await this.orderItemRepository.save(orderItems);
    }
  }
}
