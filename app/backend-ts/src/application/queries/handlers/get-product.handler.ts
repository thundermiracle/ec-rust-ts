import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetProductQuery } from '../models/get-product.query';
import { ProductDto, VariantDto } from '../../dto/product.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { ProductId } from '../../../domain/value-objects';
import { ValidationError, NotFoundError } from '../../errors/application.error';
import { Product, SKU } from '../../../domain/entities';

@QueryHandler(GetProductQuery)
export class GetProductHandler implements IQueryHandler<GetProductQuery> {
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
  ) {}

  async execute(query: GetProductQuery): Promise<ProductDto> {
    // Parse and validate product ID
    let productId: ProductId;
    try {
      productId = ProductId.fromUuid(query.productId);
    } catch {
      throw new ValidationError(
        `Invalid product ID format: ${query.productId}`,
      );
    }

    // Fetch product
    const product = await this.productRepository.findById(productId);
    if (!product) {
      throw new NotFoundError('Product', query.productId);
    }

    // Convert to DTO
    return this.convertToProductDto(product);
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
