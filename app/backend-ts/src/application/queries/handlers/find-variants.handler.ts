import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { FindVariantsQuery } from '../models/get-product.query';
import { VariantSummaryDto } from '../../dto/variant-summary.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { SKUId } from '../../../domain/value-objects';
import { ValidationError } from '../../errors/application.error';
import { SKU } from '../../../domain/entities';

@QueryHandler(FindVariantsQuery)
export class FindVariantsHandler implements IQueryHandler<FindVariantsQuery> {
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
  ) {}

  async execute(query: FindVariantsQuery): Promise<VariantSummaryDto[]> {
    if (!query.skuIds || query.skuIds.length === 0) {
      return [];
    }

    // Parse and validate SKU IDs
    const skuIds: SKUId[] = [];
    for (const skuIdString of query.skuIds) {
      try {
        skuIds.push(SKUId.fromUuid(skuIdString));
      } catch {
        throw new ValidationError(`Invalid SKU ID format: ${skuIdString}`);
      }
    }

    // Fetch SKUs
    const skus = await this.productRepository.findSkusByIds(skuIds);

    // Convert to DTOs
    return skus.map((sku) => this.convertToVariantSummaryDto(sku));
  }

  private convertToVariantSummaryDto(sku: SKU): VariantSummaryDto {
    const colorId = sku.colorId();

    return new VariantSummaryDto(
      sku.getId().value(),
      sku.code(),
      sku.getProductId().value(),
      '', // Product name would need to be fetched from product
      sku.getName(),
      sku.currentPrice().yen(),
      !sku.isOutOfStock(),
      sku.availableQuantity(),
      colorId?.value() || null,
      null, // Color name would need to be fetched from color repository
      null, // Color hex would need to be fetched from color repository
      sku.dimensions() || null,
      sku.material() || null,
    );
  }
}
