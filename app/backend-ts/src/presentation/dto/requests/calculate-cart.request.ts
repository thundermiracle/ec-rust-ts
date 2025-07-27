import { ApiProperty } from '@nestjs/swagger';
import {
  IsArray,
  IsString,
  IsNumber,
  Min,
  ValidateNested,
} from 'class-validator';
import { Type } from 'class-transformer';

export class CalculateCartItemRequest {
  @ApiProperty({
    description: 'SKU ID',
    example: '123e4567-e89b-12d3-a456-426614174000',
  })
  @IsString()
  skuId: string;

  @ApiProperty({ description: 'Quantity to order', example: 2, minimum: 1 })
  @IsNumber()
  @Min(1)
  quantity: number;
}

export class CalculateCartRequest {
  @ApiProperty({
    description: 'Cart items',
    type: [CalculateCartItemRequest],
    example: [
      { skuId: '123e4567-e89b-12d3-a456-426614174000', quantity: 2 },
      { skuId: '987fcdeb-51a2-43d7-8f9e-123456789abc', quantity: 1 },
    ],
  })
  @IsArray()
  @ValidateNested({ each: true })
  @Type(() => CalculateCartItemRequest)
  items: CalculateCartItemRequest[];

  @ApiProperty({
    description: 'Shipping method ID',
    example: 'standard-shipping',
  })
  @IsString()
  shippingMethodId: string;

  @ApiProperty({
    description: 'Payment method ID',
    example: 'credit-card',
  })
  @IsString()
  paymentMethodId: string;
}
