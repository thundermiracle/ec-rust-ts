import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import {
  IShippingMethodRepository,
  ShippingMethodData,
} from '../../../application/repositories/shipping-method.repository.interface';
import { ShippingMethodId, Money } from '../../../domain/value-objects';
import {
  ShippingMethodDto,
  ShippingMethodListDto,
} from '../../../application/dto';
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
  async findById(id: ShippingMethodId): Promise<ShippingMethodData | null> {
    const entity = await this.shippingMethodRepository.findOne({
      where: { id: id.value(), is_active: true },
    });

    return entity ? this.toDomain(entity) : null;
  }

  private toDomain(entity: ShippingMethodEntity): ShippingMethodData {
    return {
      id: ShippingMethodId.new(entity.id),
      name: entity.name,
      fee: Money.fromYen(entity.price),
      description: entity.description,
    };
  }
}
