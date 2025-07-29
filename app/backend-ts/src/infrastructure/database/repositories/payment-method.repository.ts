import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { IPaymentMethodRepository } from '../../../application/repositories/payment-method.repository.interface';
import { PaymentMethod } from '../../../domain';
import { PaymentMethodId, Money } from '../../../domain/value-objects';
import {
  PaymentMethodDto,
  PaymentMethodListDto,
} from '../../../application/dto';
import { PaymentMethodEntity } from '../entities/payment-method.entity';

@Injectable()
export class PaymentMethodRepository implements IPaymentMethodRepository {
  constructor(
    @InjectRepository(PaymentMethodEntity)
    private readonly paymentMethodRepository: Repository<PaymentMethodEntity>,
  ) {}

  // Query methods - return DTOs
  async findAllPaymentMethods(): Promise<PaymentMethodListDto> {
    const entities = await this.paymentMethodRepository.find({
      where: { is_active: true },
      order: { sort_order: 'ASC', name: 'ASC' },
    });

    const paymentMethodDtos = entities.map(
      (entity) =>
        new PaymentMethodDto(
          entity.id,
          entity.name,
          entity.description || '', // Ensure description is not null
        ),
    );

    return new PaymentMethodListDto(paymentMethodDtos);
  }

  // Command methods - work with data
  async findById(id: PaymentMethodId): Promise<PaymentMethod | null> {
    const entity = await this.paymentMethodRepository.findOne({
      where: { id: id.value(), is_active: true },
    });

    return entity ? this.toDomain(entity) : null;
  }

  private toDomain(entity: PaymentMethodEntity): PaymentMethod {
    return PaymentMethod.create(
      PaymentMethodId.new(entity.id),
      entity.name,
      Money.fromYen(0), // Payment methods in Rust schema don't have fees
      entity.description,
      entity.is_active,
    );
  }

  private toEntity(domain: PaymentMethod): PaymentMethodEntity {
    const entity = new PaymentMethodEntity();
    entity.id = domain.getId().value();
    entity.name = domain.getName();
    entity.description = domain.getDescription() || '';
    entity.is_active = domain.isAvailable();
    entity.sort_order = 0;
    return entity;
  }
}
