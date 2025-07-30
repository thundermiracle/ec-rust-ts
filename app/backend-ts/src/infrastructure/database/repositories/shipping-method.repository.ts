import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { ShippingMethodDto, ShippingMethodListDto } from '$application/dto';
import { IShippingMethodRepository } from '$application/repositories/shipping-method.repository.interface';
import { ShippingMethod } from '$domain/entities';
import { Money, ShippingMethodId } from '$domain/value-objects';

import { ShippingMethodEntity } from '../entities/shipping-method.entity';

@Injectable()
export class ShippingMethodRepository implements IShippingMethodRepository {
  constructor(
    @InjectRepository(ShippingMethodEntity)
    private readonly shippingMethodRepository: Repository<ShippingMethodEntity>,
  ) {}

  // Query methods - return DTOs
  async findAllShippingMethods(): Promise<ShippingMethodListDto> {
    const entities = await this.shippingMethodRepository.find({
      where: { is_active: true },
      order: { sort_order: 'ASC', name: 'ASC' },
    });

    const shippingMethodDtos = entities.map(
      (entity) =>
        new ShippingMethodDto(
          entity.id,
          entity.name,
          entity.description || '', // Ensure description is not null
          entity.price,
        ),
    );

    return new ShippingMethodListDto(shippingMethodDtos);
  }

  // Command methods - work with data
  async findById(id: ShippingMethodId): Promise<ShippingMethod | null> {
    const entity = await this.shippingMethodRepository.findOne({
      where: { id: id.value(), is_active: true },
    });

    return entity ? this.toDomain(entity) : null;
  }

  private toDomain(entity: ShippingMethodEntity): ShippingMethod {
    return ShippingMethod.create(
      ShippingMethodId.new(entity.id),
      entity.name,
      Money.fromYen(entity.price),
      entity.description,
      entity.is_active,
    );
  }
}
