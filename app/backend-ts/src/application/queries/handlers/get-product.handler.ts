import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetProductQuery } from '../models/get-product.query';
import { ProductDto } from '../../dto/product.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { ProductId } from '../../../domain/value-objects';
import { ValidationError, NotFoundError } from '../../errors/application.error';

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
