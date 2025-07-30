import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { ProductDto } from '$application/dto';
import {
  NotFoundError,
  ValidationError,
} from '$application/errors/application.error';
import { GetProductQuery } from '$application/queries';
import { IProductRepository } from '$application/repositories';
import { ProductId } from '$domain/value-objects';

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

    // Fetch product DTO directly
    const productDto = await this.productRepository.findById(productId);
    if (!productDto) {
      throw new NotFoundError('Product', query.productId);
    }

    return productDto;
  }
}
