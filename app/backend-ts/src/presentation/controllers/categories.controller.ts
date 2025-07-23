import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { GetCategoryListQuery } from '../../application/queries/models';
import { CategoryListDto } from '../../application/dto';

@ApiTags('Categories')
@Controller('categories')
export class CategoriesController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get category list',
    description: 'Get all product categories in hierarchical order',
  })
  @ApiResponse({
    status: 200,
    description: 'Category list retrieved successfully',
    type: CategoryListDto,
  })
  async getCategoryList(): Promise<CategoryListDto> {
    const query = new GetCategoryListQuery();
    return await this.queryBus.execute(query);
  }
}
