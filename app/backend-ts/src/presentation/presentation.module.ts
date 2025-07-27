import { Module } from '@nestjs/common';
import { APP_FILTER } from '@nestjs/core';
import { CqrsModule } from '@nestjs/cqrs';
import { HttpExceptionFilter } from './filters/http-exception.filter';

// Domain-based Controllers
import { CalculateCartController } from './cart';
import { GetProductListController, GetProductController } from './products';
import { CreateOrderController } from './orders';
import { FindVariantsController } from './variants';
import { GetCategoryListController } from './categories';
import { GetColorListController } from './colors';
import { GetShippingMethodListController } from './shipping';
import { GetPaymentMethodListController } from './payment';

const controllers = [
  CalculateCartController,
  GetProductListController,
  GetProductController,
  CreateOrderController,
  FindVariantsController,
  GetCategoryListController,
  GetColorListController,
  GetShippingMethodListController,
  GetPaymentMethodListController,
];

@Module({
  imports: [CqrsModule],
  controllers,
  providers: [
    {
      provide: APP_FILTER,
      useClass: HttpExceptionFilter,
    },
  ],
})
export class PresentationModule {}
