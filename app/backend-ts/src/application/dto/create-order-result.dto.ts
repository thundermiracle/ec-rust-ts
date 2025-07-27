export class CreateOrderResultDto {
  constructor(
    public readonly orderId: string,
    public readonly orderNumber: string,
    public readonly total: number,
    public readonly status: string,
    public readonly createdAt: string,
  ) {}
}
