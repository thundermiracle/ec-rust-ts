import { Module } from '@nestjs/common';
import { APP_FILTER } from '@nestjs/core';
import { CqrsModule } from '@nestjs/cqrs';
import { DatabaseModule } from '../infrastructure/database/database.module';
import { HttpExceptionFilter } from './filters/http-exception.filter';

// Controllers
import { CartController } from './controllers/cart.controller';
import { OrdersController } from './controllers/orders.controller';
import { ProductsController } from './controllers/products.controller';
import { VariantsController } from './controllers/variants.controller';
import { CategoriesController } from './controllers/categories.controller';
import { ColorsController } from './controllers/colors.controller';
import { ShippingController } from './controllers/shipping.controller';
import { PaymentController } from './controllers/payment.controller';

const controllers = [
  CartController,
  OrdersController,
  ProductsController,
  VariantsController,
  CategoriesController,
  ColorsController,
  ShippingController,
  PaymentController,
];

@Module({
  imports: [CqrsModule, DatabaseModule],
  controllers,
  providers: [
    {
      provide: APP_FILTER,
      useClass: HttpExceptionFilter,
    },
  ],
})
export class PresentationModule {}
