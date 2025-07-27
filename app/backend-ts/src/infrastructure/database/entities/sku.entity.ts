import {
  Entity,
  Column,
  PrimaryColumn,
  ManyToOne,
  JoinColumn,
  CreateDateColumn,
  UpdateDateColumn,
} from 'typeorm';
import { ProductEntity } from './product.entity';
import { ColorEntity } from './color.entity';

@Entity('skus')
export class SkuEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  product_id: string;

  @Column('text')
  sku_code: string;

  @Column('text')
  name: string;

  @Column('integer')
  color_id: number;

  @Column('text', { nullable: true })
  dimensions?: string;

  @Column('text', { nullable: true })
  material?: string;

  @Column('integer')
  base_price: number;

  @Column('integer', { nullable: true })
  sale_price?: number;

  @Column('integer', { default: 0 })
  stock_quantity: number;

  @Column('integer', { default: 0 })
  reserved_quantity: number;

  @Column('integer', { default: 5 })
  low_stock_threshold: number;

  @Column('integer', { default: 0 })
  display_order: number;

  @Column('text', { nullable: true })
  image_url?: string;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;

  @ManyToOne(() => ProductEntity, (product) => product.skus)
  @JoinColumn({ name: 'product_id' })
  product: ProductEntity;

  @ManyToOne(() => ColorEntity)
  @JoinColumn({ name: 'color_id' })
  color: ColorEntity;
}
