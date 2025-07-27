import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetProductListQuery } from '../models/get-product.query';
import { ProductListDto } from '../../dto/product.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { ICategoryRepository } from '../../repositories/category.repository.interface';
import { CategoryId } from '../../../domain/value-objects';
import { ValidationError, NotFoundError } from '../../errors/application.error';

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
