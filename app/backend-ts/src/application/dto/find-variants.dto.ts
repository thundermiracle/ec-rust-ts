export class FindVariantsItemDto {
  constructor(
    public readonly skuId: string,
    public readonly price: number,
    public readonly salePrice: number | null,
    public readonly image: string | null,
    public readonly material: string | null,
    public readonly dimensions: string | null,
  ) {}
}

export class FindVariantsDto {
  constructor(public readonly variants: FindVariantsItemDto[]) {}
}
