import { ApiProperty } from '@nestjs/swagger';

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

  constructor(data: {
    orderId: string;
    orderNumber: string;
    total: number;
    status: string;
    createdAt: string;
  }) {
    this.orderId = data.orderId;
    this.orderNumber = data.orderNumber;
    this.total = data.total;
    this.status = data.status;
    this.createdAt = data.createdAt;
  }
}
