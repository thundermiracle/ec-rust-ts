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

import {
  CreateOrderCommand,
  CreateOrderCommandCustomerInfo,
  CreateOrderCommandItem,
  CreateOrderCommandShippingAddress,
} from '$application/commands/models';

import { IRequest } from '../../base';

export class CreateOrderItemRequest {
  @ApiProperty({
    description: 'SKU ID',
    example: 'sku-tshirt-red-m',
  })
  @IsString()
  sku_id: string;

  @ApiProperty({ description: 'Quantity to order', example: 2, minimum: 1 })
  @IsNumber()
  @Min(1)
  quantity: number;
}

export class CreateOrderCustomerInfoRequest {
  @ApiProperty({ description: 'Customer first name', example: '太郎' })
  @IsString()
  first_name: string;

  @ApiProperty({ description: 'Customer last name', example: '山田' })
  @IsString()
  last_name: string;

  @ApiProperty({
    description: 'Customer email',
    example: 'taro.yamada@example.com',
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
  postal_code: string;

  @ApiProperty({ description: 'Prefecture', example: '東京都' })
  @IsString()
  prefecture: string;

  @ApiProperty({ description: 'City', example: '渋谷区' })
  @IsString()
  city: string;

  @ApiProperty({ description: 'Street address', example: '道獠4-1-1' })
  @IsString()
  street_address: string;

  @ApiProperty({
    description: 'Building name (optional)',
    example: '山田マンション101号室',
    required: false,
  })
  @IsOptional()
  @IsString()
  building?: string;
}

export class CreateOrderRequest implements IRequest<CreateOrderCommand> {
  @ApiProperty({
    description: 'Customer information',
    type: CreateOrderCustomerInfoRequest,
  })
  @ValidateNested()
  @Type(() => CreateOrderCustomerInfoRequest)
  customer_info: CreateOrderCustomerInfoRequest;

  @ApiProperty({
    description: 'Order items',
    type: [CreateOrderItemRequest],
    example: [{ sku_id: 'sku-tshirt-red-m', quantity: 2 }],
  })
  @IsArray()
  @ValidateNested({ each: true })
  @Type(() => CreateOrderItemRequest)
  items: CreateOrderItemRequest[];

  @ApiProperty({
    description: 'Shipping method ID',
    example: 'standard',
  })
  @IsString()
  shipping_method_id: string;

  @ApiProperty({
    description: 'Payment method ID',
    example: 'credit_card',
  })
  @IsString()
  payment_method_id: string;

  @ApiProperty({
    description: 'Shipping address',
    type: CreateOrderShippingAddressRequest,
  })
  @ValidateNested()
  @Type(() => CreateOrderShippingAddressRequest)
  shipping_address: CreateOrderShippingAddressRequest;

  toCommand(): CreateOrderCommand {
    const commandItems = this.items.map(
      (item) => new CreateOrderCommandItem(item.sku_id, item.quantity),
    );

    const customerInfo = new CreateOrderCommandCustomerInfo(
      this.customer_info.first_name,
      this.customer_info.last_name,
      this.customer_info.email,
      this.customer_info.phone,
    );

    const shippingAddress = new CreateOrderCommandShippingAddress(
      this.shipping_address.postal_code,
      this.shipping_address.prefecture,
      this.shipping_address.city,
      this.shipping_address.street_address,
      this.shipping_address.building,
    );

    return new CreateOrderCommand(
      commandItems,
      customerInfo,
      shippingAddress,
      this.shipping_method_id,
      this.payment_method_id,
    );
  }
}
