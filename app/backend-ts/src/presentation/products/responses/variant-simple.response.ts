import { ApiProperty } from '@nestjs/swagger';

export class VariantSimpleResponse {
  @ApiProperty({ description: 'Variant ID' })
  id: string;

  @ApiProperty({ description: 'SKU code' })
  skuCode: string;

  @ApiProperty({ description: 'Variant name' })
  name: string;

  @ApiProperty({ description: 'Color' })
  color: string;

  @ApiProperty({ description: 'Material' })
  material: string;

  @ApiProperty({ description: 'Dimensions' })
  dimensions: string;

  @ApiProperty({ description: 'Price in cents converted to dollars' })
  price: number;

  @ApiProperty({
    description: 'Sale price in cents converted to dollars',
    nullable: true,
  })
  salePrice?: number;

  @ApiProperty({ description: 'Display order' })
  displayOrder: number;

  @ApiProperty({ description: 'Variant image URL', nullable: true })
  image?: string;

  @ApiProperty({ description: 'Whether the variant is on sale' })
  isOnSale: boolean;

  @ApiProperty({ description: 'Whether the variant is sold out' })
  isSoldOut: boolean;

  constructor(data: {
    id: string;
    skuCode: string;
    name: string;
    color: string;
    material: string;
    dimensions: string;
    price: number;
    salePrice?: number;
    displayOrder: number;
    image?: string;
    isOnSale: boolean;
    isSoldOut: boolean;
  }) {
    this.id = data.id;
    this.skuCode = data.skuCode;
    this.name = data.name;
    this.color = data.color;
    this.material = data.material;
    this.dimensions = data.dimensions;
    this.price = data.price;
    this.salePrice = data.salePrice;
    this.displayOrder = data.displayOrder;
    this.image = data.image;
    this.isOnSale = data.isOnSale;
    this.isSoldOut = data.isSoldOut;
  }
}
