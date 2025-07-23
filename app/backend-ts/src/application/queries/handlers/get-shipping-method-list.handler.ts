import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetShippingMethodListQuery } from '../models/get-product.query';
import {
  ShippingMethodListDto,
  ShippingMethodDto,
} from '../../dto/shipping-method.dto';
import { IShippingMethodRepository } from '../../repositories/shipping-method.repository.interface';

@QueryHandler(GetShippingMethodListQuery)
export class GetShippingMethodListHandler
  implements IQueryHandler<GetShippingMethodListQuery>
{
  constructor(
    @Inject('IShippingMethodRepository')
    private readonly shippingMethodRepository: IShippingMethodRepository,
  ) {}

  async execute(): Promise<ShippingMethodListDto> {
    const methods = await this.shippingMethodRepository.findAll();

    const methodDtos = methods.map(
      (method) =>
        new ShippingMethodDto(
          method.id.value(),
          method.name,
          method.fee.yen(),
          method.description,
        ),
    );

    return new ShippingMethodListDto(methodDtos);
  }
}
