import { Global, Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';

import { CategoryEntity } from './entities/category.entity';
import { ColorEntity } from './entities/color.entity';
import { OrderEntity } from './entities/order.entity';
import { OrderItemEntity } from './entities/order-item.entity';
import { PaymentMethodEntity } from './entities/payment-method.entity';
// Entities
import { ProductEntity } from './entities/product.entity';
import { ProductImageEntity } from './entities/product-image.entity';
import { ShippingMethodEntity } from './entities/shipping-method.entity';
import { SkuEntity } from './entities/sku.entity';
import { CategoryRepository } from './repositories/category.repository';
import { ColorRepository } from './repositories/color.repository';
import { OrderRepository } from './repositories/order.repository';
import { PaymentMethodRepository } from './repositories/payment-method.repository';
// Repositories
import { ProductRepository } from './repositories/product.repository';
import { ShippingMethodRepository } from './repositories/shipping-method.repository';

const entities = [
  ProductEntity,
  SkuEntity,
  CategoryEntity,
  ColorEntity,
  ProductImageEntity,
  OrderEntity,
  OrderItemEntity,
  ShippingMethodEntity,
  PaymentMethodEntity,
];

const repositories = [
  {
    provide: 'IProductRepository',
    useClass: ProductRepository,
  },
  {
    provide: 'ICategoryRepository',
    useClass: CategoryRepository,
  },
  {
    provide: 'IColorRepository',
    useClass: ColorRepository,
  },
  {
    provide: 'IOrderRepository',
    useClass: OrderRepository,
  },
  {
    provide: 'IShippingMethodRepository',
    useClass: ShippingMethodRepository,
  },
  {
    provide: 'IPaymentMethodRepository',
    useClass: PaymentMethodRepository,
  },
];

@Global()
@Module({
  imports: [TypeOrmModule.forFeature(entities)],
  providers: [...repositories],
  exports: [...repositories],
})
export class DatabaseModule {}
