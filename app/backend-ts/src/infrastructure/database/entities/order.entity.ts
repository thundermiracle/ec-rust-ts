import {
  Entity,
  Column,
  PrimaryColumn,
  OneToMany,
  CreateDateColumn,
  UpdateDateColumn,
} from 'typeorm';
import { OrderItemEntity } from './order-item.entity';

@Entity('orders')
export class OrderEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  order_number: string;

  @Column('text')
  customer_first_name: string;

  @Column('text')
  customer_last_name: string;

  @Column('text')
  customer_email: string;

  @Column('text')
  customer_phone: string;

  @Column('text')
  shipping_postal_code: string;

  @Column('text')
  shipping_prefecture: string;

  @Column('text')
  shipping_city: string;

  @Column('text')
  shipping_street: string;

  @Column('text', { nullable: true })
  shipping_building?: string;

  @Column('text')
  shipping_method_id: string;

  @Column('text')
  shipping_method_name: string;

  @Column('integer')
  shipping_fee: number;

  @Column('text')
  payment_method_id: string;

  @Column('text')
  payment_method_name: string;

  @Column('integer')
  payment_fee: number;

  @Column('integer')
  subtotal: number;

  @Column('integer')
  tax_amount: number;

  @Column('integer')
  total: number;

  @Column('text')
  status: string;

  @Column('text', { nullable: true })
  notes?: string;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;

  @Column('datetime', { nullable: true })
  paid_at?: Date;

  @Column('datetime', { nullable: true })
  shipped_at?: Date;

  @Column('datetime', { nullable: true })
  delivered_at?: Date;

  @Column('datetime', { nullable: true })
  cancelled_at?: Date;

  @OneToMany(() => OrderItemEntity, (item) => item.order)
  items: OrderItemEntity[];
}
