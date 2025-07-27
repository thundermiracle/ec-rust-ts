export class OrderItemDto {
  constructor(
    public readonly skuId: string,
    public readonly productId: string,
    public readonly name: string,
    public readonly unitPrice: number,
    public readonly quantity: number,
    public readonly subtotal: number,
  ) {}
}

export class OrderDto {
  constructor(
    public readonly id: string,
    public readonly orderNumber: string,
    public readonly status: string,
    public readonly customerEmail: string,
    public readonly customerFirstName: string,
    public readonly customerLastName: string,
    public readonly customerPhone: string,
    public readonly shippingAddress: string,
    public readonly shippingCity: string,
    public readonly shippingState: string,
    public readonly shippingPostalCode: string,
    public readonly shippingCountry: string,
    public readonly shippingMethod: string,
    public readonly shippingFee: number,
    public readonly paymentMethod: string,
    public readonly paymentFee: number,
    public readonly subtotal: number,
    public readonly total: number,
    public readonly items: OrderItemDto[],
    public readonly notes?: string,
    public readonly createdAt?: Date,
    public readonly updatedAt?: Date,
    public readonly paidAt?: Date,
    public readonly shippedAt?: Date,
    public readonly deliveredAt?: Date,
    public readonly cancelledAt?: Date,
  ) {}
}

export class OrderListDto {
  constructor(
    public readonly orders: OrderDto[],
    public readonly total: number,
    public readonly page: number,
    public readonly limit: number,
  ) {}
}
