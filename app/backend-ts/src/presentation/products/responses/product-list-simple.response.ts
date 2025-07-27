import { ApiProperty } from '@nestjs/swagger';

export class ProductListItemResponse {
  @ApiProperty({ description: 'Product ID' })
  id: string;

  @ApiProperty({ description: 'Product name' })
  name: string;

  @ApiProperty({
    description: 'Base price (in original currency units)',
  })
  price: number;

  @ApiProperty({
    description: 'Sale price (in original currency units)',
    nullable: true,
  })
  salePrice?: number;

  @ApiProperty({ description: 'Product image URL' })
  image: string;

  @ApiProperty({ description: 'Category name' })
  category: string;

  @ApiProperty({ description: 'Available colors', type: [String] })
  colors: string[];

  @ApiProperty({
    description: 'Whether the product is on sale',
  })
  isOnSale: boolean;

  @ApiProperty({
    description: 'Whether the product is a best seller',
  })
  isBestSeller: boolean;

  @ApiProperty({
    description: 'Whether the product is available for quick shipping',
  })
  isQuickShip: boolean;

  @ApiProperty({
    description: 'Whether the product is sold out',
  })
  isSoldOut: boolean;

  constructor(data: {
    id: string;
    name: string;
    price: number;
    salePrice?: number;
    image: string;
    category: string;
    colors: string[];
    isOnSale: boolean;
    isBestSeller: boolean;
    isQuickShip: boolean;
    isSoldOut: boolean;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.price = data.price;
    this.salePrice = data.salePrice;
    this.image = data.image;
    this.category = data.category;
    this.colors = data.colors;
    this.isOnSale = data.isOnSale;
    this.isBestSeller = data.isBestSeller;
    this.isQuickShip = data.isQuickShip;
    this.isSoldOut = data.isSoldOut;
  }
}

export class ProductListSimpleResponse {
  @ApiProperty({
    description: 'List of products',
    type: [ProductListItemResponse],
  })
  products: ProductListItemResponse[];

  @ApiProperty({
    description: 'Total number of products matching the criteria',
  })
  totalCount: number;

  @ApiProperty({ description: 'Current page number' })
  page: number;

  @ApiProperty({ description: 'Number of items per page' })
  perPage: number;

  @ApiProperty({ description: 'Whether there is a next page' })
  hasNextPage: boolean;

  @ApiProperty({ description: 'Whether there is a previous page' })
  hasPreviousPage: boolean;

  constructor(data: {
    products: ProductListItemResponse[];
    totalCount: number;
    page: number;
    perPage: number;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
  }) {
    this.products = data.products;
    this.totalCount = data.totalCount;
    this.page = data.page;
    this.perPage = data.perPage;
    this.hasNextPage = data.hasNextPage;
    this.hasPreviousPage = data.hasPreviousPage;
  }
}
