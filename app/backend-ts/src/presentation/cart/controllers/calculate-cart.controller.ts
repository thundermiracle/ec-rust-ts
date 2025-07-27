import { Controller, Post, Body } from '@nestjs/common';
import { CommandBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse, ApiBody } from '@nestjs/swagger';
import { CalculateCartRequest } from '../requests';
import { CalculateCartResponse } from '../responses';
import { CartPresenter } from '../presenters';
import { CalculateCartCommand } from '../../../application/commands/models';
import { CalculateCartResultDto } from '../../../application/dto';

@ApiTags('Cart')
@Controller('cart')
export class CalculateCartController {
  constructor(private readonly commandBus: CommandBus) {}

  @Post()
  @ApiOperation({
    summary: 'Calculate cart totals',
    description:
      'Calculate subtotal, taxes, shipping, and total for a cart with given items',
  })
  @ApiBody({
    description: 'Cart calculation request',
    type: CalculateCartRequest,
  })
  @ApiResponse({
    status: 200,
    description: 'Cart calculation completed successfully',
    type: CalculateCartResponse,
  })
  @ApiResponse({
    status: 400,
    description: 'Invalid request parameters',
  })
  @ApiResponse({
    status: 404,
    description: 'Product or shipping/payment method not found',
  })
  async execute(
    @Body() request: CalculateCartRequest,
  ): Promise<CalculateCartResponse> {
    const command = request.toCommand();
    const result = await this.commandBus.execute<
      CalculateCartCommand,
      CalculateCartResultDto
    >(command);
    return CartPresenter.toCalculateCartResponse(result);
  }
}
