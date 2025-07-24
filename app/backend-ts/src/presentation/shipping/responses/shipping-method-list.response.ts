import { ApiProperty } from '@nestjs/swagger';

export class ShippingMethodResponse {
  @ApiProperty({ description: 'Shipping method ID' })
  id: string;

  @ApiProperty({ description: 'Shipping method name' })
  name: string;

  @ApiProperty({ description: 'Shipping fee in cents' })
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

export class ShippingMethodListResponse {
  @ApiProperty({
    description: 'List of shipping methods',
    type: [ShippingMethodResponse],
  })
  shippingMethods: ShippingMethodResponse[];

  constructor(data: { shippingMethods: ShippingMethodResponse[] }) {
    this.shippingMethods = data.shippingMethods;
  }
}
