export class CalculateCartCommandItem {
  constructor(
    public readonly skuId: string,
    public readonly quantity: number,
  ) {}
}

export class CalculateCartCommand {
  constructor(
    public readonly items: CalculateCartCommandItem[],
    public readonly shippingMethodId: string,
    public readonly paymentMethodId: string,
  ) {}
}
