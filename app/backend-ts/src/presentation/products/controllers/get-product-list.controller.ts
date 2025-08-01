import { Controller, Get, Query } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiQuery, ApiResponse, ApiTags } from '@nestjs/swagger';

import { ProductListDto } from '$application/dto';
import { GetProductListQuery } from '$application/queries/models';

import { ProductsPresenter } from '../presenters';
import { GetProductListRequest } from '../requests';
import { ProductListSimpleResponse } from '../responses';

@ApiTags('Products')
@Controller('products')
export class GetProductListController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get product list',
    description:
      'Get a paginated list of products, optionally filtered by category',
  })
  @ApiQuery({
    name: 'categoryId',
    required: false,
    description: 'Filter by category ID',
  })
  @ApiQuery({
    name: 'page',
    required: false,
    description: 'Page number (default: 1)',
  })
  @ApiQuery({
    name: 'limit',
    required: false,
    description: 'Items per page (default: 20, max: 100)',
  })
  @ApiResponse({
    status: 200,
    description: 'Product list retrieved successfully',
    type: ProductListSimpleResponse,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid query parameters',
  })
  @ApiResponse({
    status: 404,
    description: 'Category not found',
  })
  async execute(
    @Query('categoryId') categoryId?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ): Promise<ProductListSimpleResponse> {
    const request = new GetProductListRequest();
    request.categoryId = categoryId;
    request.page = page ? parseInt(page, 10) : undefined;
    request.limit = limit ? parseInt(limit, 10) : undefined;

    const query = request.toQuery();
    const result = await this.queryBus.execute<
      GetProductListQuery,
      ProductListDto
    >(query);
    return ProductsPresenter.toProductListSimpleResponse(result);
  }
}
