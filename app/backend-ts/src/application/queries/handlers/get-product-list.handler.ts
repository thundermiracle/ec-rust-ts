import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { ProductListDto } from '$application/dto';
import {
  NotFoundError,
  ValidationError,
} from '$application/errors/application.error';
import { GetProductListQuery } from '$application/queries';
import {
  ICategoryRepository,
  IProductRepository,
} from '$application/repositories';
import { CategoryId } from '$domain/value-objects';

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

    // Fetch products with pagination handled by repository
    const productListDto = categoryId
      ? await this.productRepository.findByCategory(categoryId)
      : await this.productRepository.findAll();

    return productListDto;
  }
}
