import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetCategoryListQuery } from '../models/get-product.query';
import { CategoryListDto } from '../../dto/category.dto';
import { ICategoryRepository } from '../../repositories/category.repository.interface';

@QueryHandler(GetCategoryListQuery)
export class GetCategoryListHandler
  implements IQueryHandler<GetCategoryListQuery>
{
  constructor(
    @Inject('ICategoryRepository')
    private readonly categoryRepository: ICategoryRepository,
  ) {}

  async execute(): Promise<CategoryListDto> {
    return await this.categoryRepository.findAllCategories();
  }
}
