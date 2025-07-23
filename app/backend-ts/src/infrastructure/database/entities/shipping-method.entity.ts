import { Entity, Column, PrimaryColumn } from 'typeorm';

@Entity('shipping_methods')
export class ShippingMethodEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  name: string;

  @Column('integer')
  fee: number;

  @Column('text', { nullable: true })
  description?: string;

  @Column('boolean', { default: true })
  is_active: boolean;

  @Column('integer', { default: 0 })
  display_order: number;
}
