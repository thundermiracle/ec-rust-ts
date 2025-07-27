import { ApiProperty } from '@nestjs/swagger';
import { IsArray, IsString } from 'class-validator';
import { FindVariantsQuery } from '../../../application/queries/models';

export class FindVariantsRequest {
  @ApiProperty({
    description: 'Array of SKU IDs to find',
    type: [String],
    example: [
      '123e4567-e89b-12d3-a456-426614174000',
      '987fcdeb-51a2-43d7-8f9e-123456789abc',
    ],
  })
  @IsArray()
  @IsString({ each: true })
  skuIds: string[];

  toQuery(): FindVariantsQuery {
    return new FindVariantsQuery(this.skuIds);
  }
}
