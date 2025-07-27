export class ShippingMethodDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly description: string,
    public readonly price: number,
  ) {}
}

export class ShippingMethodListDto {
  constructor(public readonly methods: ShippingMethodDto[]) {}
}
