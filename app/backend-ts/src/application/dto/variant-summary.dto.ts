export class VariantSummaryDto {
  constructor(
    public readonly id: string,
    public readonly skuCode: string,
    public readonly productId: string,
    public readonly productName: string,
    public readonly name: string,
    public readonly currentPrice: number,
    public readonly isInStock: boolean,
    public readonly stockQuantity: number,
    public readonly colorId: number | null,
    public readonly colorName: string | null,
    public readonly colorHex: string | null,
    public readonly dimensions: string | null,
    public readonly material: string | null,
  ) {}
}
