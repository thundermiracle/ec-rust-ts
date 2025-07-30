import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { FindVariantsItemDto } from '$application/dto';
import { ValidationError } from '$application/errors/application.error';
import { FindVariantsQuery } from '$application/queries';
import { IProductRepository } from '$application/repositories';
import { SKUId } from '$domain/value-objects';

@QueryHandler(FindVariantsQuery)
export class FindVariantsHandler implements IQueryHandler<FindVariantsQuery> {
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
  ) {}

  async execute(query: FindVariantsQuery): Promise<FindVariantsItemDto[]> {
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

    // Fetch variant DTOs directly with the new method
    return await this.productRepository.findVariantsBySkuIds(skuIds);
  }
}
