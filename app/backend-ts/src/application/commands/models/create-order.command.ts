export class CreateOrderCommandItem {
  constructor(
    public readonly skuId: string,
    public readonly quantity: number,
  ) {}
}

export class CreateOrderCommandCustomerInfo {
  constructor(
    public readonly firstName: string,
    public readonly lastName: string,
    public readonly email: string,
    public readonly phone: string,
  ) {}
}

export class CreateOrderCommandShippingAddress {
  constructor(
    public readonly postalCode: string,
    public readonly prefecture: string,
    public readonly city: string,
    public readonly streetAddress: string,
    public readonly building?: string,
  ) {}
}

export class CreateOrderCommand {
  constructor(
    public readonly items: CreateOrderCommandItem[],
    public readonly customerInfo: CreateOrderCommandCustomerInfo,
    public readonly shippingAddress: CreateOrderCommandShippingAddress,
    public readonly shippingMethodId: string,
    public readonly paymentMethodId: string,
  ) {}
}
