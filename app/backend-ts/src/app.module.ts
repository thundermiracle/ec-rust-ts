import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { CqrsModule } from '@nestjs/cqrs';
import { DatabaseModule } from './infrastructure/database/database.module';
import { PresentationModule } from './presentation/presentation.module';

// Import entities for TypeORM configuration
import { ProductEntity } from './infrastructure/database/entities/product.entity';
import { SkuEntity } from './infrastructure/database/entities/sku.entity';
import { CategoryEntity } from './infrastructure/database/entities/category.entity';
import { ColorEntity } from './infrastructure/database/entities/color.entity';
import { ProductImageEntity } from './infrastructure/database/entities/product-image.entity';
import { OrderEntity } from './infrastructure/database/entities/order.entity';
import { OrderItemEntity } from './infrastructure/database/entities/order-item.entity';
import { ShippingMethodEntity } from './infrastructure/database/entities/shipping-method.entity';
import { PaymentMethodEntity } from './infrastructure/database/entities/payment-method.entity';

@Module({
  imports: [
    TypeOrmModule.forRoot({
      type: 'sqlite',
      database: '../backend/data/db.sqlite', // Use same database as Rust backend
      entities: [
        ProductEntity,
        SkuEntity,
        CategoryEntity,
        ColorEntity,
        ProductImageEntity,
        OrderEntity,
        OrderItemEntity,
        ShippingMethodEntity,
        PaymentMethodEntity,
      ],
      synchronize: false, // Don't auto-sync, use migrations
      logging: true,
    }),
    CqrsModule,
    DatabaseModule,
    PresentationModule,
  ],
})
export class AppModule {}
