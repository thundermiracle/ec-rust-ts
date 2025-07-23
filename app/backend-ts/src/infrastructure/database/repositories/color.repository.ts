import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { IColorRepository } from '../../../application/repositories/color.repository.interface';
import { Color } from '../../../domain/entities';
import { ColorId } from '../../../domain/value-objects';
import { ColorEntity } from '../entities/color.entity';
import { ColorMapper } from '../mappers/color.mapper';

@Injectable()
export class ColorRepository implements IColorRepository {
  constructor(
    @InjectRepository(ColorEntity)
    private readonly colorRepository: Repository<ColorEntity>,
  ) {}

  async findById(id: ColorId): Promise<Color | null> {
    const entity = await this.colorRepository.findOne({
      where: { id: id.value() },
    });

    return entity ? ColorMapper.toDomain(entity) : null;
  }

  async findAll(): Promise<Color[]> {
    const entities = await this.colorRepository.find({
      order: { name: 'ASC' },
    });

    return entities.map((entity) => ColorMapper.toDomain(entity));
  }

  async save(color: Color): Promise<void> {
    const entity = ColorMapper.toEntity(color);
    await this.colorRepository.save(entity);
  }

  async update(color: Color): Promise<void> {
    const entity = ColorMapper.toEntity(color);
    await this.colorRepository.save(entity);
  }

  async delete(id: ColorId): Promise<void> {
    await this.colorRepository.delete(id.value());
  }
}
