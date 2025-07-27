import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetShippingMethodListQuery } from '../models/get-product.query';
import { ShippingMethodListDto } from '../../dto/shipping-method.dto';
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
    return await this.shippingMethodRepository.findAllShippingMethods();
  }
}
