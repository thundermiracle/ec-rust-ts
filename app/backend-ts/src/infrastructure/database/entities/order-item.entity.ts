import {
  Column,
  CreateDateColumn,
  Entity,
  JoinColumn,
  ManyToOne,
  PrimaryGeneratedColumn,
} from 'typeorm';

import { OrderEntity } from './order.entity';

@Entity('order_items')
export class OrderItemEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column('text')
  order_id: string;

  // SKU information
  @Column('text')
  sku_id: string;

  @Column('text')
  sku_code: string;

  @Column('text')
  product_name: string;

  @Column('text')
  sku_name: string;

  // Price information
  @Column('integer')
  unit_price: number;

  @Column('integer')
  quantity: number;

  @Column('integer')
  subtotal: number;

  @CreateDateColumn()
  created_at: Date;

  @ManyToOne(() => OrderEntity, (order) => order.items)
  @JoinColumn({ name: 'order_id' })
  order: OrderEntity;
}
