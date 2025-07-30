import { Body, Controller, HttpCode, HttpStatus, Post } from '@nestjs/common';
import { CommandBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { CreateOrderCommand } from '$application/commands/models';
import { CreateOrderResultDto } from '$application/dto';

import { OrdersPresenter } from '../presenters';
import { CreateOrderRequest } from '../requests';
import { CreateOrderResponse } from '../responses';

@ApiTags('Orders')
@Controller('orders')
export class CreateOrderController {
  constructor(private readonly commandBus: CommandBus) {}

  @Post()
  @HttpCode(HttpStatus.CREATED)
  @ApiOperation({
    summary: 'Create new order',
    description:
      'Create a new order with customer information, items, and shipping details',
  })
  @ApiResponse({
    status: 201,
    description: 'Order created successfully',
    type: CreateOrderResponse,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid request data',
  })
  @ApiResponse({
    status: 404,
    description: 'SKU, shipping method, or payment method not found',
  })
  @ApiResponse({
    status: 422,
    description: 'Business rule violation (insufficient stock, etc.)',
  })
  async execute(
    @Body() request: CreateOrderRequest,
  ): Promise<CreateOrderResponse> {
    const command = request.toCommand();
    const result = await this.commandBus.execute<
      CreateOrderCommand,
      CreateOrderResultDto
    >(command);
    return OrdersPresenter.toCreateOrderResponse(result);
  }
}
