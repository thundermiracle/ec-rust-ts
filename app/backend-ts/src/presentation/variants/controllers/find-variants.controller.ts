import { Controller, Post, Body } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { FindVariantsRequest } from '../requests';
import { VariantSummaryResponse } from '../responses';
import { VariantsPresenter } from '../presenters';
import { FindVariantsQuery } from '../../../application/queries/models';
import { VariantSummaryDto } from '../../../application/dto';

@ApiTags('Variants')
@Controller('variants')
export class FindVariantsController {
  constructor(private readonly queryBus: QueryBus) {}

  @Post('find')
  @ApiOperation({
    summary: 'Find variants by SKU IDs',
    description:
      'Get variant information for multiple SKU IDs in a single request',
  })
  @ApiResponse({
    status: 200,
    description: 'Variants found successfully',
    type: [VariantSummaryResponse],
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid SKU ID format',
  })
  async execute(
    @Body() request: FindVariantsRequest,
  ): Promise<VariantSummaryResponse[]> {
    const query = request.toQuery();
    const result = await this.queryBus.execute<
      FindVariantsQuery,
      VariantSummaryDto[]
    >(query);
    return VariantsPresenter.toVariantSummaryListResponse(result);
  }
}
