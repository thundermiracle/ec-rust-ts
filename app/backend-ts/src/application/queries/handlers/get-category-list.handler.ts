import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetCategoryListQuery } from '../models/get-product.query';
import { CategoryListDto, CategoryDto } from '../../dto/category.dto';
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
    const categories = await this.categoryRepository.findAll();

    const categoryDtos = categories.map(
      (category) =>
        new CategoryDto(
          category.getId().value(),
          category.getName(),
          category.getSlug(),
          category.getParentId()?.value() || null,
          category.getDisplayOrder(),
        ),
    );

    return new CategoryListDto(categoryDtos);
  }
}
