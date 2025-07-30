import {
  Column,
  CreateDateColumn,
  Entity,
  JoinColumn,
  ManyToOne,
  PrimaryGeneratedColumn,
  UpdateDateColumn,
} from 'typeorm';

import { ProductEntity } from './product.entity';

@Entity('product_images')
export class ProductImageEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column('text')
  product_id: string;

  @Column('text')
  image_url: string;

  @Column('text', { nullable: true })
  alt_text?: string;

  @Column('integer', { default: 0 })
  display_order: number;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;

  @ManyToOne(() => ProductEntity, (product) => product.images)
  @JoinColumn({ name: 'product_id' })
  product: ProductEntity;
}
