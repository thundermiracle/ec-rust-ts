import { ApiProperty } from '@nestjs/swagger';

export class PaymentMethodResponse {
  @ApiProperty({ description: 'Payment method ID' })
  id: string;

  @ApiProperty({ description: 'Payment method name' })
  name: string;

  @ApiProperty({ description: 'Payment processing fee in cents' })
  fee: number;

  @ApiProperty({ description: 'Method description', required: false })
  description?: string;

  constructor(data: {
    id: string;
    name: string;
    fee: number;
    description?: string;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.fee = data.fee;
    this.description = data.description;
  }
}

export class PaymentMethodListResponse {
  @ApiProperty({
    description: 'List of payment methods',
    type: [PaymentMethodResponse],
  })
  paymentMethods: PaymentMethodResponse[];

  constructor(data: { paymentMethods: PaymentMethodResponse[] }) {
    this.paymentMethods = data.paymentMethods;
  }
}
