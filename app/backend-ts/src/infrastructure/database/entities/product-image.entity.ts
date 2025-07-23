import { Entity, Column, PrimaryColumn, ManyToOne, JoinColumn } from 'typeorm';
import { ProductEntity } from './product.entity';

@Entity('product_images')
export class ProductImageEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  product_id: string;

  @Column('text')
  url: string;

  @Column('text')
  alt_text: string;

  @Column('integer', { default: 0 })
  display_order: number;

  @Column('boolean', { default: false })
  is_main: boolean;

  @ManyToOne(() => ProductEntity, (product) => product.images)
  @JoinColumn({ name: 'product_id' })
  product: ProductEntity;
}
