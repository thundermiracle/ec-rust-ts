import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { ShippingMethodListResponse } from '../responses';
import { ShippingPresenter } from '../presenters';
import { GetShippingMethodListQuery } from '../../../application/queries/models';
import { ShippingMethodListDto } from '../../../application/dto';

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
