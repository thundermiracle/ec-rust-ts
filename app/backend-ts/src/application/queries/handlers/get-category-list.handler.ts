import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { CategoryListDto } from '$application/dto';
import { GetCategoryListQuery } from '$application/queries';
import { ICategoryRepository } from '$application/repositories';

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
