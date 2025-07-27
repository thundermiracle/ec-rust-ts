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

  // Customer information
  @Column('text')
  customer_first_name: string;

  @Column('text')
  customer_last_name: string;

  @Column('text')
  customer_email: string;

  @Column('text')
  customer_phone: string;

  // Shipping information
  @Column('text')
  shipping_method_id: string;

  @Column('integer')
  shipping_fee: number;

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

  // Payment information
  @Column('text')
  payment_method_id: string;

  @Column('integer')
  payment_fee: number;

  @Column('text', { nullable: true })
  payment_details?: string;

  // Price information
  @Column('integer')
  subtotal: number;

  @Column('integer')
  shipping_fee_total: number;

  @Column('integer')
  payment_fee_total: number;

  @Column('integer')
  tax_amount: number;

  @Column('integer')
  total_amount: number;

  // Status and timestamps
  @Column('text', { default: 'pending' })
  status: string;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;

  @Column('text', { nullable: true })
  paid_at?: string;

  @Column('text', { nullable: true })
  shipped_at?: string;

  @Column('text', { nullable: true })
  delivered_at?: string;

  @Column('text', { nullable: true })
  cancelled_at?: string;

  // Optional
  @Column('text', { nullable: true })
  delivery_info_id?: string;

  @Column('text', { nullable: true })
  notes?: string;

  @OneToMany(() => OrderItemEntity, (item) => item.order)
  items: OrderItemEntity[];
}
