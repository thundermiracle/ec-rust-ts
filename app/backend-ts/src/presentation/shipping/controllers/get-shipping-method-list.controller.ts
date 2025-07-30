import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { ShippingMethodListDto } from '$application/dto';
import { GetShippingMethodListQuery } from '$application/queries/models';

import { ShippingPresenter } from '../presenters';
import { ShippingMethodListResponse } from '../responses';

@ApiTags('Shipping')
@Controller('shipping-methods')
export class GetShippingMethodListController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get shipping method list',
    description: 'Get all available shipping methods with fees',
  })
  @ApiResponse({
    status: 200,
    description: 'Shipping method list retrieved successfully',
    type: ShippingMethodListResponse,
  })
  async execute(): Promise<ShippingMethodListResponse> {
    const query = new GetShippingMethodListQuery();
    const result = await this.queryBus.execute<
      GetShippingMethodListQuery,
      ShippingMethodListDto
    >(query);
    return ShippingPresenter.toShippingMethodListResponse(result);
  }
}
