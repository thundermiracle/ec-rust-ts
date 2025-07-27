import { ApiProperty } from '@nestjs/swagger';

export class VariantResponse {
  @ApiProperty({ description: 'Variant ID' })
  id: string;

  @ApiProperty({ description: 'SKU code' })
  skuCode: string;

  @ApiProperty({ description: 'Variant name' })
  name: string;

  @ApiProperty({ description: 'Base price in cents' })
  basePrice: number;

  @ApiProperty({
    description: 'Sale price in cents (null if not on sale)',
    nullable: true,
  })
  salePrice: number | null;

  @ApiProperty({ description: 'Current effective price in cents' })
  currentPrice: number;

  @ApiProperty({ description: 'Whether the variant is currently on sale' })
  isOnSale: boolean;

  @ApiProperty({ description: 'Available stock quantity' })
  stockQuantity: number;

  @ApiProperty({ description: 'Whether the variant is in stock' })
  isInStock: boolean;

  @ApiProperty({ description: 'Whether the variant is sold out' })
  isSoldOut: boolean;

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

  @ApiProperty({ description: 'Display order for sorting' })
  displayOrder: number;

  constructor(data: {
    id: string;
    skuCode: string;
    name: string;
    basePrice: number;
    salePrice: number | null;
    currentPrice: number;
    isOnSale: boolean;
    stockQuantity: number;
    isInStock: boolean;
    isSoldOut: boolean;
    colorId: number | null;
    colorName: string | null;
    colorHex: string | null;
    dimensions: string | null;
    material: string | null;
    displayOrder: number;
  }) {
    this.id = data.id;
    this.skuCode = data.skuCode;
    this.name = data.name;
    this.basePrice = data.basePrice;
    this.salePrice = data.salePrice;
    this.currentPrice = data.currentPrice;
    this.isOnSale = data.isOnSale;
    this.stockQuantity = data.stockQuantity;
    this.isInStock = data.isInStock;
    this.isSoldOut = data.isSoldOut;
    this.colorId = data.colorId;
    this.colorName = data.colorName;
    this.colorHex = data.colorHex;
    this.dimensions = data.dimensions;
    this.material = data.material;
    this.displayOrder = data.displayOrder;
  }
}

export class ProductResponse {
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

export class ProductListResponse {
  @ApiProperty({
    description: 'List of products',
    type: [ProductResponse],
  })
  products: ProductResponse[];

  @ApiProperty({
    description: 'Total number of products matching the criteria',
  })
  total: number;

  @ApiProperty({ description: 'Current page number' })
  page: number;

  @ApiProperty({ description: 'Number of items per page' })
  limit: number;

  constructor(data: {
    products: ProductResponse[];
    total: number;
    page: number;
    limit: number;
  }) {
    this.products = data.products;
    this.total = data.total;
    this.page = data.page;
    this.limit = data.limit;
  }
}
