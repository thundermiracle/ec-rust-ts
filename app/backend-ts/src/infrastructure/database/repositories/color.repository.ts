import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { ColorDto, ColorListDto } from '$application/dto';
import { IColorRepository } from '$application/repositories';
import { ColorEntity } from '$infrastructure/database/entities/color.entity';

@Injectable()
export class ColorRepository implements IColorRepository {
  constructor(
    @InjectRepository(ColorEntity)
    private readonly colorRepository: Repository<ColorEntity>,
  ) {}

  // Query methods - return DTOs
  async findAllColors(): Promise<ColorListDto> {
    const entities = await this.colorRepository.find({
      order: { name: 'ASC' },
    });

    const colorDtos = entities.map(
      (entity) => new ColorDto(entity.id, entity.name, entity.hex),
    );

    return new ColorListDto(colorDtos);
  }
}
