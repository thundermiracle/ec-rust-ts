import { ApiProperty } from '@nestjs/swagger';

export class CategoryResponse {
  @ApiProperty({ description: 'Category ID' })
  id: string;

  @ApiProperty({ description: 'Category name' })
  name: string;

  @ApiProperty({ description: 'URL-friendly slug' })
  slug: string;

  @ApiProperty({
    description: 'Parent category ID (null for root categories)',
    nullable: true,
  })
  parentId: string | null;

  @ApiProperty({ description: 'Display order for sorting' })
  displayOrder: number;

  constructor(data: {
    id: string;
    name: string;
    slug: string;
    parentId: string | null;
    displayOrder: number;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.slug = data.slug;
    this.parentId = data.parentId;
    this.displayOrder = data.displayOrder;
  }
}

export class CategoryListResponse {
  @ApiProperty({
    description: 'List of categories',
    type: [CategoryResponse],
  })
  categories: CategoryResponse[];

  constructor(data: { categories: CategoryResponse[] }) {
    this.categories = data.categories;
  }
}
