export class CalculatedCartItemDto {
  constructor(
    public readonly skuId: string,
    public readonly productId: string,
    public readonly productName: string,
    public readonly skuName: string,
    public readonly unitPrice: number,
    public readonly quantity: number,
    public readonly subtotal: number,
  ) {}
}

export class CalculateCartResultDto {
  constructor(
    public readonly items: CalculatedCartItemDto[],
    public readonly subtotal: number,
    public readonly shippingFee: number,
    public readonly paymentFee: number,
    public readonly taxAmount: number,
    public readonly total: number,
    public readonly shippingMethodId: string,
    public readonly shippingMethodName: string,
    public readonly paymentMethodId: string,
    public readonly paymentMethodName: string,
  ) {}
}
