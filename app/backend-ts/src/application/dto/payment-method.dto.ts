export class PaymentMethodDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly description: string,
  ) {}
}

export class PaymentMethodListDto {
  constructor(public readonly items: PaymentMethodDto[]) {}
}
