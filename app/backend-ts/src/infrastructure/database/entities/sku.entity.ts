import { Entity, Column, PrimaryColumn, ManyToOne, JoinColumn } from 'typeorm';
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
  base_price: number;

  @Column('integer', { nullable: true })
  sale_price?: number;

  @Column('integer')
  stock_quantity: number;

  @Column('integer', { default: 0 })
  reserved_quantity: number;

  @Column('text')
  status: string;

  @Column('integer', { default: 0 })
  display_order: number;

  @Column('integer', { nullable: true })
  color_id?: number;

  @Column('text', { nullable: true })
  dimensions?: string;

  @Column('text', { nullable: true })
  material?: string;

  @ManyToOne(() => ProductEntity, (product) => product.skus)
  @JoinColumn({ name: 'product_id' })
  product: ProductEntity;

  @ManyToOne(() => ColorEntity, { nullable: true })
  @JoinColumn({ name: 'color_id' })
  color?: ColorEntity;
}
