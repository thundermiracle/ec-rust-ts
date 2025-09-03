import {
  Column,
  CreateDateColumn,
  Entity,
  PrimaryColumn,
  UpdateDateColumn,
} from 'typeorm';

@Entity('coupons')
export class CouponEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text', { unique: true })
  code: string;

  @Column('text')
  name: string;

  @Column('text', { nullable: true })
  description?: string;

  @Column('text')
  discount_type: string;

  @Column('integer')
  discount_value: number;

  @Column('integer', { nullable: true })
  minimum_amount?: number;

  @Column('integer', { nullable: true })
  usage_limit?: number;

  @Column('integer', { default: 0 })
  used_count: number;

  @Column('text')
  valid_from: string;

  @Column('text')
  valid_until: string;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;
}
