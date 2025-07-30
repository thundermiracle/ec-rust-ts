import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { PaymentMethodListDto } from '$application/dto';
import { GetPaymentMethodListQuery } from '$application/queries';
import { IPaymentMethodRepository } from '$application/repositories';

@QueryHandler(GetPaymentMethodListQuery)
export class GetPaymentMethodListHandler
  implements IQueryHandler<GetPaymentMethodListQuery>
{
  constructor(
    @Inject('IPaymentMethodRepository')
    private readonly paymentMethodRepository: IPaymentMethodRepository,
  ) {}

  async execute(): Promise<PaymentMethodListDto> {
    return await this.paymentMethodRepository.findAllPaymentMethods();
  }
}
