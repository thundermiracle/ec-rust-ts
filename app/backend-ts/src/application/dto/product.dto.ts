export class VariantDto {
  constructor(
    public readonly id: string,
    public readonly skuCode: string,
    public readonly name: string,
    public readonly basePrice: number,
    public readonly salePrice: number | null,
    public readonly currentPrice: number,
    public readonly isOnSale: boolean,
    public readonly stockQuantity: number,
    public readonly isInStock: boolean,
    public readonly isSoldOut: boolean,
    public readonly colorId: number | null,
    public readonly colorName: string | null,
    public readonly colorHex: string | null,
    public readonly dimensions: string | null,
    public readonly material: string | null,
    public readonly displayOrder: number,
  ) {}
}

export class ProductDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly description: string,
    public readonly categoryId: string,
    public readonly isBestSeller: boolean,
    public readonly isQuickShip: boolean,
    public readonly isAvailable: boolean,
    public readonly variants: VariantDto[],
    public readonly minPrice: number | null,
    public readonly maxPrice: number | null,
    public readonly totalStock: number,
    public readonly hasVariants: boolean,
  ) {}
}

export class ProductListDto {
  constructor(
    public readonly products: ProductDto[],
    public readonly total: number,
    public readonly page: number,
    public readonly limit: number,
  ) {}
}
