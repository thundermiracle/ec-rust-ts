import { ApiProperty } from '@nestjs/swagger';

import { VariantSimpleResponse } from './variant-simple.response';

export class ProductDetailResponse {
  @ApiProperty({ description: 'Product ID' })
  id: string;

  @ApiProperty({ description: 'Product name' })
  name: string;

  @ApiProperty({ description: 'Product image URLs', type: [String] })
  images: string[];

  @ApiProperty({ description: 'Category name' })
  category: string;

  @ApiProperty({ description: 'Product description' })
  description: string;

  @ApiProperty({ description: 'Whether the product is a best seller' })
  isBestSeller: boolean;

  @ApiProperty({
    description: 'Whether the product is available for quick shipping',
  })
  isQuickShip: boolean;

  @ApiProperty({
    description: 'List of product variants',
    type: [VariantSimpleResponse],
  })
  variants: VariantSimpleResponse[];

  constructor(data: {
    id: string;
    name: string;
    images: string[];
    category: string;
    description: string;
    isBestSeller: boolean;
    isQuickShip: boolean;
    variants: VariantSimpleResponse[];
  }) {
    this.id = data.id;
    this.name = data.name;
    this.images = data.images;
    this.category = data.category;
    this.description = data.description;
    this.isBestSeller = data.isBestSeller;
    this.isQuickShip = data.isQuickShip;
    this.variants = data.variants;
  }
}

export { VariantSimpleResponse } from './variant-simple.response';
