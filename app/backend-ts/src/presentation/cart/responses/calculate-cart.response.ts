import { ApiProperty } from '@nestjs/swagger';

export class CalculatedCartItemResponse {
  @ApiProperty({ description: 'SKU ID of the product variant' })
  skuId: string;

  @ApiProperty({ description: 'Product ID' })
  productId: string;

  @ApiProperty({ description: 'Product name' })
  productName: string;

  @ApiProperty({ description: 'SKU name' })
  skuName: string;

  @ApiProperty({ description: 'Unit price in cents' })
  unitPrice: number;

  @ApiProperty({ description: 'Quantity ordered' })
  quantity: number;

  @ApiProperty({ description: 'Subtotal in cents (unitPrice * quantity)' })
  subtotal: number;

  constructor(data: {
    skuId: string;
    productId: string;
    productName: string;
    skuName: string;
    unitPrice: number;
    quantity: number;
    subtotal: number;
  }) {
    this.skuId = data.skuId;
    this.productId = data.productId;
    this.productName = data.productName;
    this.skuName = data.skuName;
    this.unitPrice = data.unitPrice;
    this.quantity = data.quantity;
    this.subtotal = data.subtotal;
  }
}

export class CalculateCartResponse {
  @ApiProperty({
    description: 'List of calculated cart items',
    type: [CalculatedCartItemResponse],
  })
  items: CalculatedCartItemResponse[];

  @ApiProperty({ description: 'Subtotal in cents (sum of all item subtotals)' })
  subtotal: number;

  @ApiProperty({ description: 'Shipping fee in cents' })
  shippingFee: number;

  @ApiProperty({ description: 'Payment processing fee in cents' })
  paymentFee: number;

  @ApiProperty({ description: 'Tax amount in cents' })
  taxAmount: number;

  @ApiProperty({ description: 'Total amount in cents' })
  total: number;

  @ApiProperty({ description: 'Shipping method ID' })
  shippingMethodId: string;

  @ApiProperty({ description: 'Shipping method name' })
  shippingMethodName: string;

  @ApiProperty({ description: 'Payment method ID' })
  paymentMethodId: string;

  @ApiProperty({ description: 'Payment method name' })
  paymentMethodName: string;

  constructor(data: {
    items: CalculatedCartItemResponse[];
    subtotal: number;
    shippingFee: number;
    paymentFee: number;
    taxAmount: number;
    total: number;
    shippingMethodId: string;
    shippingMethodName: string;
    paymentMethodId: string;
    paymentMethodName: string;
  }) {
    this.items = data.items;
    this.subtotal = data.subtotal;
    this.shippingFee = data.shippingFee;
    this.paymentFee = data.paymentFee;
    this.taxAmount = data.taxAmount;
    this.total = data.total;
    this.shippingMethodId = data.shippingMethodId;
    this.shippingMethodName = data.shippingMethodName;
    this.paymentMethodId = data.paymentMethodId;
    this.paymentMethodName = data.paymentMethodName;
  }
}
