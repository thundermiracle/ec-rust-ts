import { ProductId, SKUId, CategoryId } from '../../domain/value-objects';
import { ProductDto, ProductListDto } from '../dto';
import { VariantSummaryDto } from '../dto/variant-summary.dto';
import { FindVariantsItemDto } from '../dto/find-variants.dto';

export interface IProductRepository {
  // Query methods - return DTOs
  findById(id: ProductId): Promise<ProductDto | null>;
  findByCategory(categoryId: CategoryId): Promise<ProductListDto>;
  findAll(): Promise<ProductListDto>;
  findSkusByIds(skuIds: SKUId[]): Promise<VariantSummaryDto[]>;
  findVariantsBySkuIds(skuIds: SKUId[]): Promise<FindVariantsItemDto[]>;
}
