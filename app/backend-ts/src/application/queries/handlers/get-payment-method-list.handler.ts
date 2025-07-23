import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetPaymentMethodListQuery } from '../models/get-product.query';
import {
  PaymentMethodListDto,
  PaymentMethodDto,
} from '../../dto/payment-method.dto';
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
    const methods = await this.paymentMethodRepository.findAll();

    const methodDtos = methods.map(
      (method) =>
        new PaymentMethodDto(
          method.id.value(),
          method.name,
          method.fee.yen(),
          method.description,
        ),
    );

    return new PaymentMethodListDto(methodDtos);
  }
}
