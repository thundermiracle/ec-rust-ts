import { ApiProperty } from '@nestjs/swagger';

export class FindVariantsItemResponse {
  @ApiProperty({ description: 'SKU ID' })
  skuId: string;

  @ApiProperty({ description: 'Base price in original currency units' })
  price: number;

  @ApiProperty({
    description: 'Sale price in original currency units',
    nullable: true,
  })
  salePrice?: number;

  @ApiProperty({ description: 'Product image URL', nullable: true })
  image?: string;

  @ApiProperty({ description: 'Product material', nullable: true })
  material?: string;

  @ApiProperty({ description: 'Product dimensions', nullable: true })
  dimensions?: string;

  constructor(data: {
    skuId: string;
    price: number;
    salePrice?: number;
    image?: string;
    material?: string;
    dimensions?: string;
  }) {
    this.skuId = data.skuId;
    this.price = data.price;
    this.salePrice = data.salePrice;
    this.image = data.image;
    this.material = data.material;
    this.dimensions = data.dimensions;
  }
}

export class FindVariantsResponse {
  @ApiProperty({
    description: 'List of variants',
    type: [FindVariantsItemResponse],
  })
  variants: FindVariantsItemResponse[];

  constructor(data: { variants: FindVariantsItemResponse[] }) {
    this.variants = data.variants;
  }
}
