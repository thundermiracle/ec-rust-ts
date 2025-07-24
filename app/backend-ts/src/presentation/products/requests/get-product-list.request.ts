import { IsString, IsOptional, IsInt, Min, Max } from 'class-validator';
import { Type } from 'class-transformer';
import { ApiPropertyOptional } from '@nestjs/swagger';
import { IQuery } from '../../base';
import { GetProductListQuery } from '../../../application/queries/models';
import { ProductListDto } from '../../../application/dto';

export class GetProductListRequest implements IQuery<ProductListDto> {
  @ApiPropertyOptional({ description: 'Filter by category ID' })
  @IsOptional()
  @IsString()
  categoryId?: string;

  @ApiPropertyOptional({
    description: 'Page number (default: 1)',
    minimum: 1,
  })
  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  page?: number;

  @ApiPropertyOptional({
    description: 'Items per page (default: 20, max: 100)',
    minimum: 1,
    maximum: 100,
  })
  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  @Max(100)
  limit?: number;

  toQuery(): GetProductListQuery {
    return new GetProductListQuery(this.categoryId, this.page, this.limit);
  }
}
