import { Body, Controller, Post } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { FindVariantsItemDto } from '$application/dto';
import { FindVariantsQuery } from '$application/queries/models';

import { VariantsPresenter } from '../presenters';
import { FindVariantsRequest } from '../requests';
import { FindVariantsResponse } from '../responses';

@ApiTags('Variants')
@Controller('variants')
export class FindVariantsController {
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
    type: FindVariantsResponse,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid SKU ID format',
  })
  async execute(
    @Body() request: FindVariantsRequest,
  ): Promise<FindVariantsResponse> {
    const query = request.toQuery();
    const result = await this.queryBus.execute<
      FindVariantsQuery,
      FindVariantsItemDto[]
    >(query);
    return VariantsPresenter.toFindVariantsResponse(result);
  }
}
