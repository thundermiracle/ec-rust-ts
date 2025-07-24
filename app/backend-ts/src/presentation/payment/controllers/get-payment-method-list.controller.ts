import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { PaymentMethodListResponse } from '../responses';
import { PaymentPresenter } from '../presenters';
import { GetPaymentMethodListQuery } from '../../../application/queries/models';
import { PaymentMethodListDto } from '../../../application/dto';

@ApiTags('Payment')
@Controller('payment-methods')
export class GetPaymentMethodListController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get payment method list',
    description: 'Get all available payment methods with fees',
  })
  @ApiResponse({
    status: 200,
    description: 'Payment method list retrieved successfully',
    type: PaymentMethodListResponse,
  })
  async execute(): Promise<PaymentMethodListResponse> {
    const query = new GetPaymentMethodListQuery();
    const result = await this.queryBus.execute<
      GetPaymentMethodListQuery,
      PaymentMethodListDto
    >(query);
    return PaymentPresenter.toPaymentMethodListResponse(result);
  }
}
