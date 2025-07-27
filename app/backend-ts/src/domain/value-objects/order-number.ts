import { DomainError } from '../errors/domain.error';

export class OrderNumber {
  private constructor(private readonly value: string) {}

  static generate(): OrderNumber {
    const now = new Date();
    const year = now.getFullYear().toString().slice(-2);
    const month = (now.getMonth() + 1).toString().padStart(2, '0');
    const day = now.getDate().toString().padStart(2, '0');
    const timestamp = now.getTime().toString().slice(-6);

    const orderNumber = `ORD${year}${month}${day}${timestamp}`;
    return new OrderNumber(orderNumber);
  }

  static fromString(value: string): OrderNumber {
    if (!value || value.trim().length === 0) {
      throw new DomainError('Order number cannot be empty');
    }
    if (!OrderNumber.isValidFormat(value)) {
      throw new DomainError('Invalid order number format');
    }
    return new OrderNumber(value.trim());
  }

  private static isValidFormat(value: string): boolean {
    const orderNumberRegex = /^ORD\d{8}\d{6}$/;
    return orderNumberRegex.test(value);
  }

  getValue(): string {
    return this.value;
  }

  toString(): string {
    return this.value;
  }

  equals(other: OrderNumber): boolean {
    return this.value === other.value;
  }
}
