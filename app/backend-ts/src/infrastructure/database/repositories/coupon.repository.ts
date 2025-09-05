import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { ICouponRepository } from '$application/repositories/coupon.repository.interface';
import { Coupon } from '$domain/entities/coupon';
import { CouponCode } from '$domain/value-objects/coupon-code';

import { CouponEntity } from '../entities/coupon.entity';
import { CouponMapper } from '../mappers/coupon.mapper';

@Injectable()
export class CouponRepository implements ICouponRepository {
  constructor(
    @InjectRepository(CouponEntity)
    private readonly couponRepository: Repository<CouponEntity>,
  ) {}

  async findByCode(code: CouponCode): Promise<Coupon | null> {
    const entity = await this.couponRepository
      .createQueryBuilder('coupon')
      .where('coupon.code = :code', { code: code.getValue() })
      .andWhere('DATE() BETWEEN coupon.valid_from AND coupon.valid_until')
      .getOne();

    return entity ? CouponMapper.toDomain(entity) : null;
  }

  async updateUsageCount(coupon: Coupon): Promise<void> {
    const newCount = coupon.getUsageCount() + 1;

    await this.couponRepository
      .createQueryBuilder()
      .update(CouponEntity)
      .set({
        used_count: newCount,
        updated_at: new Date(),
      })
      .where('id = :id', { id: coupon.getId().value() })
      .execute();
  }
}
