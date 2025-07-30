import {
  Column,
  CreateDateColumn,
  Entity,
  JoinColumn,
  ManyToOne,
  OneToMany,
  PrimaryColumn,
  UpdateDateColumn,
} from 'typeorm';

import { CategoryEntity } from './category.entity';
import { ProductImageEntity } from './product-image.entity';
import { SkuEntity } from './sku.entity';

@Entity('products')
export class ProductEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  name: string;

  @Column('text')
  description: string;

  @Column('text')
  category_id: string;

  @Column('boolean', { default: false })
  is_best_seller: boolean;

  @Column('boolean', { default: false })
  is_quick_ship: boolean;

  @Column('boolean', { default: true })
  is_available: boolean;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;

  @OneToMany(() => SkuEntity, (sku) => sku.product)
  skus: SkuEntity[];

  @ManyToOne(() => CategoryEntity)
  @JoinColumn({ name: 'category_id' })
  category: CategoryEntity;

  @OneToMany(() => ProductImageEntity, (image) => image.product)
  images: ProductImageEntity[];
}
