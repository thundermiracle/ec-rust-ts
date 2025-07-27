import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetPaymentMethodListQuery } from '../models/get-product.query';
import { PaymentMethodListDto } from '../../dto/payment-method.dto';
import { IPaymentMethodRepository } from '../../repositories/payment-method.repository.interface';

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
