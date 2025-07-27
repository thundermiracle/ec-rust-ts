export class VariantDto {
  constructor(
    public readonly id: string,
    public readonly skuCode: string,
    public readonly name: string,
    public readonly color: string,
    public readonly material: string,
    public readonly dimensions: string,
    public readonly price: number,
    public readonly salePrice: number | null,
    public readonly stockQuantity: number,
    public readonly reservedQuantity: number,
    public readonly displayOrder: number,
    public readonly image: string | null,
    public readonly isOnSale: boolean,
    public readonly isSoldOut: boolean,
  ) {}
}

export class ProductDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly images: string[],
    public readonly category: string,
    public readonly description: string,
    public readonly isBestSeller: boolean,
    public readonly isQuickShip: boolean,
    public readonly variants: VariantDto[],
  ) {}
}

export class ProductSummaryDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly category: string,
    public readonly basePrice: number,
    public readonly salePrice: number | null,
    public readonly image: string | null,
    public readonly colors: string[],
    public readonly isBestSeller: boolean,
    public readonly isQuickShip: boolean,
    public readonly stockQuantity: number,
  ) {}
}

export class ProductListDto {
  constructor(
    public readonly products: ProductSummaryDto[],
    public readonly totalCount: number,
    public readonly page: number,
    public readonly perPage: number,
    public readonly hasNextPage: boolean,
    public readonly hasPreviousPage: boolean,
  ) {}
}
