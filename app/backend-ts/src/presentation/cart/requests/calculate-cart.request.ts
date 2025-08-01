import { ApiProperty } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import {
  ArrayMinSize,
  IsArray,
  IsNotEmpty,
  IsNumber,
  IsString,
  Min,
  ValidateNested,
} from 'class-validator';

import {
  CalculateCartCommand,
  CalculateCartCommandItem,
} from '$application/commands/models';

import { IRequest } from '../../base';

export class CalculateCartItemRequest {
  @ApiProperty({ description: 'SKU ID of the product variant' })
  @IsString()
  @IsNotEmpty()
  skuId: string;

  @ApiProperty({ description: 'Quantity of the item', minimum: 1 })
  @IsNumber()
  @Min(1)
  quantity: number;
}

export class CalculateCartRequest implements IRequest<CalculateCartCommand> {
  @ApiProperty({
    description: 'List of cart items',
    type: [CalculateCartItemRequest],
  })
  @IsArray()
  @ArrayMinSize(1)
  @ValidateNested({ each: true })
  @Type(() => CalculateCartItemRequest)
  items: CalculateCartItemRequest[];

  @ApiProperty({ description: 'Shipping method ID' })
  @IsString()
  @IsNotEmpty()
  shipping_method_id: string;

  @ApiProperty({ description: 'Payment method ID' })
  @IsString()
  @IsNotEmpty()
  payment_method_id: string;

  toCommand(): CalculateCartCommand {
    return new CalculateCartCommand(
      this.items.map(
        (item) => new CalculateCartCommandItem(item.skuId, item.quantity),
      ),
      this.shipping_method_id,
      this.payment_method_id,
    );
  }
}
