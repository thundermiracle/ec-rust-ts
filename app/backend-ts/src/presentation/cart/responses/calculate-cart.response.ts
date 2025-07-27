import { ApiProperty } from '@nestjs/swagger';

export class CalculatedCartItemResponse {
  @ApiProperty({ description: 'SKU ID of the product variant' })
  skuId: string;

  @ApiProperty({ description: 'Product ID' })
  productId: string;

  @ApiProperty({ description: 'Product name' })
  productName: string;

  @ApiProperty({ description: 'Unit price in original currency units' })
  unitPrice: number;

  @ApiProperty({ description: 'Quantity ordered' })
  quantity: number;

  @ApiProperty({ description: 'Subtotal (unitPrice * quantity)' })
  subtotal: number;

  constructor(data: {
    skuId: string;
    productId: string;
    productName: string;
    unitPrice: number;
    quantity: number;
    subtotal: number;
  }) {
    this.skuId = data.skuId;
    this.productId = data.productId;
    this.productName = data.productName;
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

  @ApiProperty({ description: 'Total quantity of all items' })
  totalQuantity: number;

  @ApiProperty({ description: 'Number of different items in cart' })
  itemCount: number;

  @ApiProperty({ description: 'Subtotal (sum of all item subtotals)' })
  subtotal: number;

  @ApiProperty({ description: 'Tax amount' })
  taxAmount: number;

  @ApiProperty({ description: 'Total amount' })
  total: number;

  @ApiProperty({ description: 'Whether the cart is empty' })
  isEmpty: boolean;

  @ApiProperty({ description: 'Shipping fee' })
  shippingFee: number;

  @ApiProperty({ description: 'Payment processing fee' })
  paymentFee: number;

  constructor(data: {
    items: CalculatedCartItemResponse[];
    totalQuantity: number;
    itemCount: number;
    subtotal: number;
    taxAmount: number;
    total: number;
    isEmpty: boolean;
    shippingFee: number;
    paymentFee: number;
  }) {
    this.items = data.items;
    this.totalQuantity = data.totalQuantity;
    this.itemCount = data.itemCount;
    this.subtotal = data.subtotal;
    this.taxAmount = data.taxAmount;
    this.total = data.total;
    this.isEmpty = data.isEmpty;
    this.shippingFee = data.shippingFee;
    this.paymentFee = data.paymentFee;
  }
}
