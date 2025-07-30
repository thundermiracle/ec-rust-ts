import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { PaymentMethodListDto } from '$application/dto';
import { GetPaymentMethodListQuery } from '$application/queries/models';

import { PaymentPresenter } from '../presenters';
import { PaymentMethodListResponse } from '../responses';

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
