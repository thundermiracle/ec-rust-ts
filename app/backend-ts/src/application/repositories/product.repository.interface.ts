import {
  FindVariantsItemDto,
  ProductDto,
  ProductListDto,
  VariantSummaryDto,
} from '$application/dto';
import { SKU } from '$domain/entities';
import { CategoryId, ProductId, SKUId } from '$domain/value-objects';

export interface IProductRepository {
  // Query methods - return DTOs
  findById(id: ProductId): Promise<ProductDto | null>;
  findByCategory(categoryId: CategoryId): Promise<ProductListDto>;
  findAll(): Promise<ProductListDto>;
  findSkusByIds(skuIds: SKUId[]): Promise<VariantSummaryDto[]>;
  findVariantsBySkuIds(skuIds: SKUId[]): Promise<FindVariantsItemDto[]>;
  // New method for SKU entities
  findSkuEntitiesByIds(skuIds: SKUId[]): Promise<SKU[]>;
}
