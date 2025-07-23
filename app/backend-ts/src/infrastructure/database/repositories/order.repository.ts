import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { IOrderRepository } from '../../../application/repositories/order.repository.interface';
import { Order } from '../../../domain/aggregates';
import { OrderId, OrderNumber } from '../../../domain/value-objects';
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

  async findById(id: OrderId): Promise<Order | null> {
    const entity = await this.orderRepository.findOne({
      where: { id: id.value() },
      relations: ['items'],
    });

    return entity ? OrderMapper.toDomain(entity) : null;
  }

  async findByOrderNumber(orderNumber: OrderNumber): Promise<Order | null> {
    const entity = await this.orderRepository.findOne({
      where: { order_number: orderNumber.getValue() },
      relations: ['items'],
    });

    return entity ? OrderMapper.toDomain(entity) : null;
  }

  async findAll(): Promise<Order[]> {
    const entities = await this.orderRepository.find({
      relations: ['items'],
      order: { created_at: 'DESC' },
    });

    return entities.map((entity) => OrderMapper.toDomain(entity));
  }

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

  async update(order: Order): Promise<void> {
    const orderEntity = OrderMapper.toEntity(order);
    await this.orderRepository.save(orderEntity);

    // Remove existing items and save new ones
    await this.orderItemRepository.delete({ order_id: orderEntity.id });

    const orderItems = order
      .getItems()
      .map((item) => OrderItemMapper.toEntity(item, orderEntity.id));

    if (orderItems.length > 0) {
      await this.orderItemRepository.save(orderItems);
    }
  }

  async delete(id: OrderId): Promise<void> {
    await this.orderRepository.delete(id.value());
  }
}
