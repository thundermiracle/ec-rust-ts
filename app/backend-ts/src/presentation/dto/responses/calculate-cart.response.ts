import { ApiProperty } from '@nestjs/swagger';
import {
  CalculateCartResultDto,
  CalculatedCartItemDto,
} from '../../../application/dto';

export class CalculatedCartItemResponse {
  @ApiProperty({
    description: 'SKU ID',
    example: '123e4567-e89b-12d3-a456-426614174000',
  })
  skuId: string;

  @ApiProperty({
    description: 'Product ID',
    example: '987fcdeb-51a2-43d7-8f9e-123456789abc',
  })
  productId: string;

  @ApiProperty({ description: 'Product name', example: 'Premium T-Shirt' })
  productName: string;

  @ApiProperty({
    description: 'SKU name',
    example: 'Premium T-Shirt - Red - M',
  })
  skuName: string;

  @ApiProperty({ description: 'Unit price in yen', example: 2980 })
  unitPrice: number;

  @ApiProperty({ description: 'Quantity', example: 2 })
  quantity: number;

  @ApiProperty({ description: 'Item subtotal in yen', example: 5960 })
  subtotal: number;

  constructor(dto: CalculatedCartItemDto) {
    this.skuId = dto.skuId;
    this.productId = dto.productId;
    this.productName = dto.productName;
    this.skuName = dto.skuName;
    this.unitPrice = dto.unitPrice;
    this.quantity = dto.quantity;
    this.subtotal = dto.subtotal;
  }
}

export class CalculateCartResponse {
  @ApiProperty({
    description: 'Cart items',
    type: [CalculatedCartItemResponse],
  })
  items: CalculatedCartItemResponse[];

  @ApiProperty({ description: 'Subtotal in yen', example: 5960 })
  subtotal: number;

  @ApiProperty({ description: 'Shipping fee in yen', example: 500 })
  shippingFee: number;

  @ApiProperty({ description: 'Payment fee in yen', example: 0 })
  paymentFee: number;

  @ApiProperty({ description: 'Tax amount in yen', example: 646 })
  taxAmount: number;

  @ApiProperty({ description: 'Total amount in yen', example: 7106 })
  total: number;

  @ApiProperty({
    description: 'Shipping method ID',
    example: 'standard-shipping',
  })
  shippingMethodId: string;

  @ApiProperty({
    description: 'Shipping method name',
    example: 'Standard Shipping',
  })
  shippingMethodName: string;

  @ApiProperty({ description: 'Payment method ID', example: 'credit-card' })
  paymentMethodId: string;

  @ApiProperty({ description: 'Payment method name', example: 'Credit Card' })
  paymentMethodName: string;

  constructor(dto: CalculateCartResultDto) {
    this.items = dto.items.map((item) => new CalculatedCartItemResponse(item));
    this.subtotal = dto.subtotal;
    this.shippingFee = dto.shippingFee;
    this.paymentFee = dto.paymentFee;
    this.taxAmount = dto.taxAmount;
    this.total = dto.total;
    this.shippingMethodId = dto.shippingMethodId;
    this.shippingMethodName = dto.shippingMethodName;
    this.paymentMethodId = dto.paymentMethodId;
    this.paymentMethodName = dto.paymentMethodName;
  }
}
