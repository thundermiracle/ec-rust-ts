import { ApiProperty } from '@nestjs/swagger';

export class ShippingMethodResponse {
  @ApiProperty({ description: 'Shipping method ID' })
  id: string;

  @ApiProperty({ description: 'Shipping method name' })
  name: string;

  @ApiProperty({ description: 'Shipping price in cents' })
  price: number;

  @ApiProperty({ description: 'Method description' })
  description: string;

  constructor(data: {
    id: string;
    name: string;
    price: number;
    description: string;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.price = data.price;
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
