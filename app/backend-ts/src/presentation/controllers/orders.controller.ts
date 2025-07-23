import { Controller, Post, Body, HttpStatus, HttpCode } from '@nestjs/common';
import { CommandBus } from '@nestjs/cqrs';
import {
  ApiTags,
  ApiOperation,
  ApiResponse,
  ApiProperty,
} from '@nestjs/swagger';
import { CreateOrderRequest } from '../dto/requests/create-order.request';
import { CreateOrderResultDto } from '../../application/dto';
import {
  CreateOrderCommand,
  CreateOrderCommandItem,
  CreateOrderCommandCustomerInfo,
  CreateOrderCommandShippingAddress,
} from '../../application/commands/models';

export class CreateOrderResponse {
  @ApiProperty({
    description: 'Order ID',
    example: '123e4567-e89b-12d3-a456-426614174000',
  })
  orderId: string;

  @ApiProperty({ description: 'Order number', example: 'ORD24072100001' })
  orderNumber: string;

  @ApiProperty({ description: 'Total amount in yen', example: 7106 })
  total: number;

  @ApiProperty({ description: 'Order status', example: 'Pending' })
  status: string;

  @ApiProperty({
    description: 'Order creation timestamp',
    example: '2024-07-21T10:30:00.000Z',
  })
  createdAt: string;

  constructor(dto: CreateOrderResultDto) {
    this.orderId = dto.orderId;
    this.orderNumber = dto.orderNumber;
    this.total = dto.total;
    this.status = dto.status;
    this.createdAt = dto.createdAt;
  }
}

@ApiTags('Orders')
@Controller('orders')
export class OrdersController {
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
  async createOrder(
    @Body() request: CreateOrderRequest,
  ): Promise<CreateOrderResponse> {
    const commandItems = request.items.map(
      (item) => new CreateOrderCommandItem(item.skuId, item.quantity),
    );

    const customerInfo = new CreateOrderCommandCustomerInfo(
      request.customerInfo.firstName,
      request.customerInfo.lastName,
      request.customerInfo.email,
      request.customerInfo.phone,
    );

    const shippingAddress = new CreateOrderCommandShippingAddress(
      request.shippingAddress.postalCode,
      request.shippingAddress.prefecture,
      request.shippingAddress.city,
      request.shippingAddress.streetAddress,
      request.shippingAddress.building,
    );

    const command = new CreateOrderCommand(
      commandItems,
      customerInfo,
      shippingAddress,
      request.shippingMethodId,
      request.paymentMethodId,
    );

    const result = await this.commandBus.execute<
      CreateOrderCommand,
      CreateOrderResultDto
    >(command);
    return new CreateOrderResponse(result);
  }
}
