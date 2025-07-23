import { Controller, Post, Body, HttpStatus, HttpCode } from '@nestjs/common';
import { CommandBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { CalculateCartRequest } from '../dto/requests/calculate-cart.request';
import { CalculateCartResponse } from '../dto/responses/calculate-cart.response';
import {
  CalculateCartCommand,
  CalculateCartCommandItem,
} from '../../application/commands/models';
import { CalculateCartResultDto } from '../../application/dto/calculate-cart-result.dto';

@ApiTags('Cart')
@Controller('cart')
export class CartController {
  constructor(private readonly commandBus: CommandBus) {}

  @Post()
  @HttpCode(HttpStatus.OK)
  @ApiOperation({
    summary: 'Calculate cart totals',
    description:
      'Calculate cart subtotal, taxes, shipping and payment fees, and total amount',
  })
  @ApiResponse({
    status: 200,
    description: 'Cart calculation successful',
    type: CalculateCartResponse,
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
  async calculateCart(
    @Body() request: CalculateCartRequest,
  ): Promise<CalculateCartResponse> {
    const commandItems = request.items.map(
      (item) => new CalculateCartCommandItem(item.skuId, item.quantity),
    );

    const command = new CalculateCartCommand(
      commandItems,
      request.shippingMethodId,
      request.paymentMethodId,
    );

    const result = await this.commandBus.execute<
      CalculateCartCommand,
      CalculateCartResultDto
    >(command);
    return new CalculateCartResponse(result);
  }
}
