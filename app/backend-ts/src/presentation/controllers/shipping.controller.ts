import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { GetShippingMethodListQuery } from '../../application/queries/models';
import { ShippingMethodListDto } from '../../application/dto';

@ApiTags('Shipping')
@Controller('shipping-methods')
export class ShippingController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get shipping method list',
    description: 'Get all available shipping methods with fees',
  })
  @ApiResponse({
    status: 200,
    description: 'Shipping method list retrieved successfully',
    type: ShippingMethodListDto,
  })
  async getShippingMethodList(): Promise<ShippingMethodListDto> {
    const query = new GetShippingMethodListQuery();
    return await this.queryBus.execute(query);
  }
}
