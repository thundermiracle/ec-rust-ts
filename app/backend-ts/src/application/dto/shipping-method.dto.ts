export class ShippingMethodDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly fee: number,
    public readonly description?: string,
  ) {}
}

export class ShippingMethodListDto {
  constructor(public readonly shippingMethods: ShippingMethodDto[]) {}
}
