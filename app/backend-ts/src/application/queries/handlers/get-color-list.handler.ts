import { QueryHandler, IQueryHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { GetColorListQuery } from '../models/get-product.query';
import { ColorListDto, ColorDto } from '../../dto/color.dto';
import { IColorRepository } from '../../repositories/color.repository.interface';

@QueryHandler(GetColorListQuery)
export class GetColorListHandler implements IQueryHandler<GetColorListQuery> {
  constructor(
    @Inject('IColorRepository')
    private readonly colorRepository: IColorRepository,
  ) {}

  async execute(): Promise<ColorListDto> {
    const colors = await this.colorRepository.findAll();

    const colorDtos = colors.map(
      (color) =>
        new ColorDto(
          color.getId().value(),
          color.getName().getValue(),
          color.getHex(),
        ),
    );

    return new ColorListDto(colorDtos);
  }
}
