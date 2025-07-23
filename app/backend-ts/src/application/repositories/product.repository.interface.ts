import { Product, SKU } from '../../domain/entities';
import { ProductId, SKUId, CategoryId } from '../../domain/value-objects';

export interface IProductRepository {
  findById(id: ProductId): Promise<Product | null>;
  findByCategory(categoryId: CategoryId): Promise<Product[]>;
  findAll(): Promise<Product[]>;
  findSkuById(skuId: SKUId): Promise<SKU | null>;
  findSkusByIds(skuIds: SKUId[]): Promise<SKU[]>;
  save(product: Product): Promise<void>;
  update(product: Product): Promise<void>;
  delete(id: ProductId): Promise<void>;
}
