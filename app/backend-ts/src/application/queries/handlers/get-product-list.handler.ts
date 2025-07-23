import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetProductListQuery } from '../models/get-product.query';
import { ProductListDto, ProductDto, VariantDto } from '../../dto/product.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { ICategoryRepository } from '../../repositories/category.repository.interface';
import { CategoryId } from '../../../domain/value-objects';
import { ValidationError, NotFoundError } from '../../errors/application.error';
import { Product, SKU } from '../../../domain/entities';

@QueryHandler(GetProductListQuery)
export class GetProductListHandler
  implements IQueryHandler<GetProductListQuery>
{
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
    @Inject('ICategoryRepository')
    private readonly categoryRepository: ICategoryRepository,
  ) {}

  async execute(query: GetProductListQuery): Promise<ProductListDto> {
    // Validate and parse category filter
    let categoryId: CategoryId | undefined;
    if (query.categoryId) {
      try {
        categoryId = CategoryId.fromUuid(query.categoryId);

        // Verify category exists
        const category = await this.categoryRepository.findById(categoryId);
        if (!category) {
          throw new NotFoundError('Category', query.categoryId);
        }
      } catch (error: unknown) {
        if (
          error instanceof ValidationError ||
          error instanceof NotFoundError
        ) {
          throw error;
        }
        throw new ValidationError('Invalid category ID format');
      }
    }

    // Fetch products
    const products = categoryId
      ? await this.productRepository.findByCategory(categoryId)
      : await this.productRepository.findAll();

    // Apply pagination
    const page = query.page || 1;
    const limit = Math.min(query.limit || 20, 100); // Max 100 items per page
    const offset = (page - 1) * limit;
    const paginatedProducts = products.slice(offset, offset + limit);

    // Convert to DTOs
    const productDtos = paginatedProducts.map((product) =>
      this.convertToProductDto(product),
    );

    return new ProductListDto(productDtos, products.length, page, limit);
  }

  private convertToProductDto(product: Product): ProductDto {
    const variants = product
      .getSkus()
      .map((sku) => this.convertToVariantDto(sku));

    const priceRange = product.priceRange();
    const minPrice = priceRange ? priceRange[0].yen() : null;
    const maxPrice = priceRange ? priceRange[1].yen() : null;

    return new ProductDto(
      product.getId().value(),
      product.getName(),
      product.getDescription(),
      product.getCategoryId().value(),
      product.getIsBestSeller(),
      product.getIsQuickShip(),
      product.getIsAvailable(),
      variants,
      minPrice,
      maxPrice,
      product.totalAvailableStock(),
      product.hasVariants(),
    );
  }

  private convertToVariantDto(sku: SKU): VariantDto {
    const colorId = sku.colorId();

    return new VariantDto(
      sku.getId().value(),
      sku.code(),
      sku.getName(),
      sku.getBasePrice().yen(),
      sku.getSalePrice()?.yen() || null,
      sku.currentPrice().yen(),
      sku.getSalePrice() !== null,
      sku.availableQuantity(),
      !sku.isOutOfStock(),
      sku.isOutOfStock(),
      colorId?.value() || null,
      null, // Color name would need to be fetched from color repository
      null, // Color hex would need to be fetched from color repository
      sku.dimensions() || null,
      sku.material() || null,
      sku.getDisplayOrder(),
    );
  }
}
