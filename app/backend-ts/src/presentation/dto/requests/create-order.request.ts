import { ApiProperty } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import {
  IsArray,
  IsEmail,
  IsNumber,
  IsOptional,
  IsString,
  Matches,
  Min,
  ValidateNested,
} from 'class-validator';

export class CreateOrderItemRequest {
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

export class CreateOrderCustomerInfoRequest {
  @ApiProperty({ description: 'Customer first name', example: '太郎' })
  @IsString()
  firstName: string;

  @ApiProperty({ description: 'Customer last name', example: '田中' })
  @IsString()
  lastName: string;

  @ApiProperty({
    description: 'Customer email',
    example: 'taro.tanaka@example.com',
  })
  @IsEmail()
  email: string;

  @ApiProperty({
    description: 'Customer phone number',
    example: '090-1234-5678',
  })
  @IsString()
  @Matches(/^(0\d{1,4}-\d{1,4}-\d{1,4}|0\d{9,10})$/, {
    message: 'Phone number must be in Japanese format',
  })
  phone: string;
}

export class CreateOrderShippingAddressRequest {
  @ApiProperty({ description: 'Postal code', example: '123-4567' })
  @IsString()
  @Matches(/^\d{3}-\d{4}$/, {
    message: 'Postal code must be in format 123-4567',
  })
  postalCode: string;

  @ApiProperty({ description: 'Prefecture', example: '東京都' })
  @IsString()
  prefecture: string;

  @ApiProperty({ description: 'City', example: '渋谷区' })
  @IsString()
  city: string;

  @ApiProperty({ description: 'Street address', example: '渋谷1-1-1' })
  @IsString()
  streetAddress: string;

  @ApiProperty({
    description: 'Building name (optional)',
    example: '渋谷ビル',
    required: false,
  })
  @IsOptional()
  @IsString()
  building?: string;
}

export class CreateOrderRequest {
  @ApiProperty({
    description: 'Order items',
    type: [CreateOrderItemRequest],
    example: [{ skuId: '123e4567-e89b-12d3-a456-426614174000', quantity: 2 }],
  })
  @IsArray()
  @ValidateNested({ each: true })
  @Type(() => CreateOrderItemRequest)
  items: CreateOrderItemRequest[];

  @ApiProperty({
    description: 'Customer information',
    type: CreateOrderCustomerInfoRequest,
  })
  @ValidateNested()
  @Type(() => CreateOrderCustomerInfoRequest)
  customerInfo: CreateOrderCustomerInfoRequest;

  @ApiProperty({
    description: 'Shipping address',
    type: CreateOrderShippingAddressRequest,
  })
  @ValidateNested()
  @Type(() => CreateOrderShippingAddressRequest)
  shippingAddress: CreateOrderShippingAddressRequest;

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
