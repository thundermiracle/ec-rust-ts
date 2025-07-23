export class PaymentMethodDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly fee: number,
    public readonly description?: string,
  ) {}
}

export class PaymentMethodListDto {
  constructor(public readonly paymentMethods: PaymentMethodDto[]) {}
}
