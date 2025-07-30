import { Controller, Get, Param } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiParam, ApiResponse, ApiTags } from '@nestjs/swagger';

import { ProductDto } from '$application/dto';
import { GetProductQuery } from '$application/queries/models';

import { ProductsPresenter } from '../presenters';
import { ProductDetailResponse } from '../responses';

@ApiTags('Products')
@Controller('products')
export class GetProductController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get(':id')
  @ApiOperation({
    summary: 'Get product details',
    description:
      'Get detailed information about a specific product including all variants',
  })
  @ApiParam({ name: 'id', description: 'Product ID' })
  @ApiResponse({
    status: 200,
    description: 'Product details retrieved successfully',
    type: ProductDetailResponse,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid product ID format',
  })
  @ApiResponse({
    status: 404,
    description: 'Product not found',
  })
  async execute(@Param('id') id: string): Promise<ProductDetailResponse> {
    const query = new GetProductQuery(id);
    const result = await this.queryBus.execute<GetProductQuery, ProductDto>(
      query,
    );
    return ProductsPresenter.toProductResponse(result);
  }
}
