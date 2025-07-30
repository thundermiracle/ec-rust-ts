import { Inject } from '@nestjs/common';
import { IQueryHandler, QueryHandler } from '@nestjs/cqrs';

import { ColorListDto } from '$application/dto';
import { GetColorListQuery } from '$application/queries';
import { IColorRepository } from '$application/repositories';

@QueryHandler(GetColorListQuery)
export class GetColorListHandler implements IQueryHandler<GetColorListQuery> {
  constructor(
    @Inject('IColorRepository')
    private readonly colorRepository: IColorRepository,
  ) {}

  async execute(): Promise<ColorListDto> {
    return await this.colorRepository.findAllColors();
  }
}
