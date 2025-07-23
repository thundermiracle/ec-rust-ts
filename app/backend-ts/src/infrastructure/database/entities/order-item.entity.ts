import {
  Entity,
  Column,
  PrimaryGeneratedColumn,
  ManyToOne,
  JoinColumn,
} from 'typeorm';
import { OrderEntity } from './order.entity';

@Entity('order_items')
export class OrderItemEntity {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column('text')
  order_id: string;

  @Column('text')
  sku_id: string;

  @Column('text')
  product_id: string;

  @Column('text')
  product_name: string;

  @Column('text')
  sku_name: string;

  @Column('integer')
  unit_price: number;

  @Column('integer')
  quantity: number;

  @Column('integer')
  subtotal: number;

  @ManyToOne(() => OrderEntity, (order) => order.items)
  @JoinColumn({ name: 'order_id' })
  order: OrderEntity;
}
