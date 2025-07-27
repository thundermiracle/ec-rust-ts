import { Module } from '@nestjs/common';
import { CqrsModule } from '@nestjs/cqrs';

// Command Handlers
import { CalculateCartHandler, CreateOrderHandler } from './commands/handlers';

// Query Handlers
import {
  GetProductListHandler,
  GetProductHandler,
  FindVariantsHandler,
  GetCategoryListHandler,
  GetColorListHandler,
  GetShippingMethodListHandler,
  GetPaymentMethodListHandler,
} from './queries/handlers';

const commandHandlers = [CalculateCartHandler, CreateOrderHandler];

const queryHandlers = [
  GetProductListHandler,
  GetProductHandler,
  FindVariantsHandler,
  GetCategoryListHandler,
  GetColorListHandler,
  GetShippingMethodListHandler,
  GetPaymentMethodListHandler,
];

@Module({
  imports: [CqrsModule],
  providers: [...commandHandlers, ...queryHandlers],
  exports: [...commandHandlers, ...queryHandlers],
})
export class ApplicationModule {}
