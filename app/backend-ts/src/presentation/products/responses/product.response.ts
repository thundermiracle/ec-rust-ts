import { ApiProperty } from '@nestjs/swagger';
import { VariantResponse } from './product-list.response';

export class ProductDetailResponse {
  @ApiProperty({ description: 'Product ID' })
  id: string;

  @ApiProperty({ description: 'Product name' })
  name: string;

  @ApiProperty({ description: 'Product description' })
  description: string;

  @ApiProperty({ description: 'Category ID' })
  categoryId: string;

  @ApiProperty({ description: 'Whether the product is a best seller' })
  isBestSeller: boolean;

  @ApiProperty({
    description: 'Whether the product is available for quick shipping',
  })
  isQuickShip: boolean;

  @ApiProperty({ description: 'Whether the product is available for purchase' })
  isAvailable: boolean;

  @ApiProperty({
    description: 'List of product variants',
    type: [VariantResponse],
  })
  variants: VariantResponse[];

  @ApiProperty({
    description:
      'Minimum price across all variants in cents (null if no variants)',
    nullable: true,
  })
  minPrice: number | null;

  @ApiProperty({
    description:
      'Maximum price across all variants in cents (null if no variants)',
    nullable: true,
  })
  maxPrice: number | null;

  @ApiProperty({ description: 'Total stock across all variants' })
  totalStock: number;

  @ApiProperty({ description: 'Whether the product has multiple variants' })
  hasVariants: boolean;

  constructor(data: {
    id: string;
    name: string;
    description: string;
    categoryId: string;
    isBestSeller: boolean;
    isQuickShip: boolean;
    isAvailable: boolean;
    variants: VariantResponse[];
    minPrice: number | null;
    maxPrice: number | null;
    totalStock: number;
    hasVariants: boolean;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.description = data.description;
    this.categoryId = data.categoryId;
    this.isBestSeller = data.isBestSeller;
    this.isQuickShip = data.isQuickShip;
    this.isAvailable = data.isAvailable;
    this.variants = data.variants;
    this.minPrice = data.minPrice;
    this.maxPrice = data.maxPrice;
    this.totalStock = data.totalStock;
    this.hasVariants = data.hasVariants;
  }
}

export { VariantResponse } from './product-list.response';
