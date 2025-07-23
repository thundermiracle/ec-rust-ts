import { Controller, Post, Body } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { FindVariantsRequest } from '../dto/requests/find-variants.request';
import { FindVariantsQuery } from '../../application/queries/models';
import { VariantSummaryDto } from '../../application/dto';

@ApiTags('Variants')
@Controller('variants')
export class VariantsController {
  constructor(private readonly queryBus: QueryBus) {}

  @Post()
  @ApiOperation({
    summary: 'Find variants by SKU IDs',
    description:
      'Get variant information for multiple SKU IDs in a single request',
  })
  @ApiResponse({
    status: 200,
    description: 'Variants found successfully',
    type: [VariantSummaryDto],
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid SKU ID format',
  })
  async findVariants(
    @Body() request: FindVariantsRequest,
  ): Promise<VariantSummaryDto[]> {
    const query = new FindVariantsQuery(request.skuIds);
    return await this.queryBus.execute(query);
  }
}
