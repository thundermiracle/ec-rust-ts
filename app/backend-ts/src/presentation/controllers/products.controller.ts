import { Controller, Get, Param, Query } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import {
  ApiTags,
  ApiOperation,
  ApiResponse,
  ApiParam,
  ApiQuery,
} from '@nestjs/swagger';
import {
  GetProductListQuery,
  GetProductQuery,
} from '../../application/queries/models';
import { ProductListDto, ProductDto } from '../../application/dto';

@ApiTags('Products')
@Controller('products')
export class ProductsController {
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
    type: ProductListDto,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid query parameters',
  })
  @ApiResponse({
    status: 404,
    description: 'Category not found',
  })
  async getProductList(
    @Query('categoryId') categoryId?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ): Promise<ProductListDto> {
    const query = new GetProductListQuery(
      categoryId,
      page ? parseInt(page, 10) : undefined,
      limit ? parseInt(limit, 10) : undefined,
    );

    return await this.queryBus.execute(query);
  }

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
    type: ProductDto,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid product ID format',
  })
  @ApiResponse({
    status: 404,
    description: 'Product not found',
  })
  async getProduct(@Param('id') id: string): Promise<ProductDto> {
    const query = new GetProductQuery(id);
    return await this.queryBus.execute(query);
  }
}
