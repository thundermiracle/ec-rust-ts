import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { FindVariantsQuery } from '../models/get-product.query';
import { FindVariantsItemDto } from '../../dto/find-variants.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { SKUId } from '../../../domain/value-objects';
import { ValidationError } from '../../errors/application.error';

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
