import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import {
  IPaymentMethodRepository,
  PaymentMethodData,
} from '../../../application/repositories/payment-method.repository.interface';
import { PaymentMethodId, Money } from '../../../domain/value-objects';
import { PaymentMethodEntity } from '../entities/payment-method.entity';

@Injectable()
export class PaymentMethodRepository implements IPaymentMethodRepository {
  constructor(
    @InjectRepository(PaymentMethodEntity)
    private readonly paymentMethodRepository: Repository<PaymentMethodEntity>,
  ) {}

  async findById(id: PaymentMethodId): Promise<PaymentMethodData | null> {
    const entity = await this.paymentMethodRepository.findOne({
      where: { id: id.value(), is_active: true },
    });

    return entity ? this.toDomain(entity) : null;
  }

  async findAll(): Promise<PaymentMethodData[]> {
    const entities = await this.paymentMethodRepository.find({
      where: { is_active: true },
      order: { display_order: 'ASC', name: 'ASC' },
    });

    return entities.map((entity) => this.toDomain(entity));
  }

  async save(method: PaymentMethodData): Promise<void> {
    const entity = this.toEntity(method);
    await this.paymentMethodRepository.save(entity);
  }

  async update(method: PaymentMethodData): Promise<void> {
    const entity = this.toEntity(method);
    await this.paymentMethodRepository.save(entity);
  }

  async delete(id: PaymentMethodId): Promise<void> {
    await this.paymentMethodRepository.update(
      { id: id.value() },
      { is_active: false },
    );
  }

  private toDomain(entity: PaymentMethodEntity): PaymentMethodData {
    return {
      id: PaymentMethodId.new(entity.id),
      name: entity.name,
      fee: Money.fromYen(entity.fee),
      description: entity.description,
    };
  }

  private toEntity(domain: PaymentMethodData): PaymentMethodEntity {
    const entity = new PaymentMethodEntity();
    entity.id = domain.id.value();
    entity.name = domain.name;
    entity.fee = domain.fee.yen();
    entity.description = domain.description;
    entity.is_active = true;
    entity.display_order = 0;
    return entity;
  }
}
