import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import {
  IShippingMethodRepository,
  ShippingMethodData,
} from '../../../application/repositories/shipping-method.repository.interface';
import { ShippingMethodId, Money } from '../../../domain/value-objects';
import { ShippingMethodEntity } from '../entities/shipping-method.entity';

@Injectable()
export class ShippingMethodRepository implements IShippingMethodRepository {
  constructor(
    @InjectRepository(ShippingMethodEntity)
    private readonly shippingMethodRepository: Repository<ShippingMethodEntity>,
  ) {}

  async findById(id: ShippingMethodId): Promise<ShippingMethodData | null> {
    const entity = await this.shippingMethodRepository.findOne({
      where: { id: id.value(), is_active: true },
    });

    return entity ? this.toDomain(entity) : null;
  }

  async findAll(): Promise<ShippingMethodData[]> {
    const entities = await this.shippingMethodRepository.find({
      where: { is_active: true },
      order: { display_order: 'ASC', name: 'ASC' },
    });

    return entities.map((entity) => this.toDomain(entity));
  }

  async save(method: ShippingMethodData): Promise<void> {
    const entity = this.toEntity(method);
    await this.shippingMethodRepository.save(entity);
  }

  async update(method: ShippingMethodData): Promise<void> {
    const entity = this.toEntity(method);
    await this.shippingMethodRepository.save(entity);
  }

  async delete(id: ShippingMethodId): Promise<void> {
    await this.shippingMethodRepository.update(
      { id: id.value() },
      { is_active: false },
    );
  }

  private toDomain(entity: ShippingMethodEntity): ShippingMethodData {
    return {
      id: ShippingMethodId.new(entity.id),
      name: entity.name,
      fee: Money.fromYen(entity.fee),
      description: entity.description,
    };
  }

  private toEntity(domain: ShippingMethodData): ShippingMethodEntity {
    const entity = new ShippingMethodEntity();
    entity.id = domain.id.value();
    entity.name = domain.name;
    entity.fee = domain.fee.yen();
    entity.description = domain.description;
    entity.is_active = true;
    entity.display_order = 0;
    return entity;
  }
}
