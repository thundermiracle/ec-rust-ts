import { Controller, Post, Body } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { FindVariantsRequest } from '../requests';
import { FindVariantsResponse } from '../responses';
import { VariantsPresenter } from '../presenters';
import { FindVariantsQuery } from '../../../application/queries/models';
import { FindVariantsItemDto } from '../../../application/dto';

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
