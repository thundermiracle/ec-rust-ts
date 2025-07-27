import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetColorListQuery } from '../models/get-product.query';
import { ColorListDto } from '../../dto/color.dto';
import { IColorRepository } from '../../repositories/color.repository.interface';

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
