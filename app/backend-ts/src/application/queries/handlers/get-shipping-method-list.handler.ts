import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { ShippingMethodListDto } from '$application/dto';
import { GetShippingMethodListQuery } from '$application/queries';
import { IShippingMethodRepository } from '$application/repositories';

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
