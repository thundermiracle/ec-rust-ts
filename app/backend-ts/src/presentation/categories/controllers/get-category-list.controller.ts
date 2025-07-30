import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { CategoryListDto } from '$application/dto';
import { GetCategoryListQuery } from '$application/queries';
import { CategoriesPresenter } from '$presentation/categories/presenters';
import { CategoryListResponse } from '$presentation/categories/responses';

@ApiTags('Categories')
@Controller('categories')
export class GetCategoryListController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get category list',
    description: 'Get all product categories in hierarchical order',
  })
  @ApiResponse({
    status: 200,
    description: 'Category list retrieved successfully',
    type: CategoryListResponse,
  })
  async execute(): Promise<CategoryListResponse> {
    const query = new GetCategoryListQuery();
    const result = await this.queryBus.execute<
      GetCategoryListQuery,
      CategoryListDto
    >(query);
    return CategoriesPresenter.toCategoryListResponse(result);
  }
}
