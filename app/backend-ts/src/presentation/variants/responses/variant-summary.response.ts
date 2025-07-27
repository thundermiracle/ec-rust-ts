import { ApiProperty } from '@nestjs/swagger';

export class VariantSummaryResponse {
  @ApiProperty({ description: 'Variant ID' })
  id: string;

  @ApiProperty({ description: 'SKU code' })
  skuCode: string;

  @ApiProperty({ description: 'Product ID' })
  productId: string;

  @ApiProperty({ description: 'Product name' })
  productName: string;

  @ApiProperty({ description: 'Variant name' })
  name: string;

  @ApiProperty({ description: 'Current price in cents' })
  currentPrice: number;

  @ApiProperty({ description: 'Whether the variant is in stock' })
  isInStock: boolean;

  @ApiProperty({ description: 'Available stock quantity' })
  stockQuantity: number;

  @ApiProperty({ description: 'Color ID (null if no color)', nullable: true })
  colorId: number | null;

  @ApiProperty({ description: 'Color name (null if no color)', nullable: true })
  colorName: string | null;

  @ApiProperty({
    description: 'Color hex code (null if no color)',
    nullable: true,
  })
  colorHex: string | null;

  @ApiProperty({
    description: 'Product dimensions (null if not specified)',
    nullable: true,
  })
  dimensions: string | null;

  @ApiProperty({
    description: 'Product material (null if not specified)',
    nullable: true,
  })
  material: string | null;

  constructor(data: {
    id: string;
    skuCode: string;
    productId: string;
    productName: string;
    name: string;
    currentPrice: number;
    isInStock: boolean;
    stockQuantity: number;
    colorId: number | null;
    colorName: string | null;
    colorHex: string | null;
    dimensions: string | null;
    material: string | null;
  }) {
    this.id = data.id;
    this.skuCode = data.skuCode;
    this.productId = data.productId;
    this.productName = data.productName;
    this.name = data.name;
    this.currentPrice = data.currentPrice;
    this.isInStock = data.isInStock;
    this.stockQuantity = data.stockQuantity;
    this.colorId = data.colorId;
    this.colorName = data.colorName;
    this.colorHex = data.colorHex;
    this.dimensions = data.dimensions;
    this.material = data.material;
  }
}
