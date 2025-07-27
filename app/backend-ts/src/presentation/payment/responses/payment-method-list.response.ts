import { ApiProperty } from '@nestjs/swagger';

export class PaymentMethodResponse {
  @ApiProperty({ description: 'Payment method ID' })
  id: string;

  @ApiProperty({ description: 'Payment method name', nullable: true })
  name?: string;

  @ApiProperty({ description: 'Method description', nullable: true })
  description?: string;

  constructor(data: { id: string; name?: string; description?: string }) {
    this.id = data.id;
    this.name = data.name;
    this.description = data.description;
  }
}

export class PaymentMethodListResponse {
  @ApiProperty({
    description: 'List of payment methods',
    type: [PaymentMethodResponse],
  })
  items: PaymentMethodResponse[];

  constructor(data: { items: PaymentMethodResponse[] }) {
    this.items = data.items;
  }
}
