import { ApiProperty } from '@nestjs/swagger';

export class CreateOrderResponse {
  @ApiProperty({
    description: 'Order ID',
    example: '123e4567-e89b-12d3-a456-426614174000',
  })
  order_id: string;

  @ApiProperty({ description: 'Order number', example: 'ORD24072100001' })
  order_number: string;

  @ApiProperty({ description: 'Total amount in yen', example: 7106 })
  total_amount: number;

  @ApiProperty({ description: 'Order status', example: 'pending' })
  status: string;

  constructor(data: {
    order_id: string;
    order_number: string;
    total_amount: number;
    status: string;
  }) {
    this.order_id = data.order_id;
    this.order_number = data.order_number;
    this.total_amount = data.total_amount;
    this.status = data.status;
  }
}
