import { Module } from '@nestjs/common';
import { APP_FILTER } from '@nestjs/core';
import { CqrsModule } from '@nestjs/cqrs';

// Domain-based Controllers
import { CalculateCartController } from './cart';
import { GetCategoryListController } from './categories';
import { GetColorListController } from './colors';
import { HttpExceptionFilter } from './filters/http-exception.filter';
import { CreateOrderController } from './orders';
import { GetPaymentMethodListController } from './payment';
import { GetProductController, GetProductListController } from './products';
import { GetShippingMethodListController } from './shipping';
import { FindVariantsController } from './variants';

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
