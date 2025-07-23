import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { GetPaymentMethodListQuery } from '../../application/queries/models';
import { PaymentMethodListDto } from '../../application/dto';

@ApiTags('Payment')
@Controller('payment-methods')
export class PaymentController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get payment method list',
    description: 'Get all available payment methods with fees',
  })
  @ApiResponse({
    status: 200,
    description: 'Payment method list retrieved successfully',
    type: PaymentMethodListDto,
  })
  async getPaymentMethodList(): Promise<PaymentMethodListDto> {
    const query = new GetPaymentMethodListQuery();
    return await this.queryBus.execute(query);
  }
}
