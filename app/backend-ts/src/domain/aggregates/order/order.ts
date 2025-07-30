import { SKU } from '$domain/entities';
import { DomainError } from '$domain/errors/domain.error';
import { Address, Money, OrderId, OrderNumber } from '$domain/value-objects';

import { CustomerInfo } from './customer-info';
import { OrderItem } from './order-item';
import { OrderPricing } from './order-pricing';
import { PaymentInfo } from './payment-info';
import { ShippingInfo } from './shipping-info';

export enum OrderStatus {
  Pending = 'pending',
  Paid = 'paid',
  Processing = 'processing',
  Shipped = 'shipped',
  Delivered = 'delivered',
  Cancelled = 'cancelled',
  Refunded = 'refunded',
}

export class OrderTimestamps {
  constructor(
    public readonly createdAt: Date,
    public updatedAt: Date,
    public paidAt?: Date,
    public shippedAt?: Date,
    public deliveredAt?: Date,
    public cancelledAt?: Date,
  ) {}

  update(): void {
    this.updatedAt = new Date();
  }

  markPaid(): void {
    this.paidAt = new Date();
    this.update();
  }

  markShipped(): void {
    this.shippedAt = new Date();
    this.update();
  }

  markDelivered(): void {
    this.deliveredAt = new Date();
    this.update();
  }

  markCancelled(): void {
    this.cancelledAt = new Date();
    this.update();
  }
}

export class Order {
  private constructor(
    private readonly id: OrderId,
    private readonly orderNumber: OrderNumber,
    private readonly customerInfo: CustomerInfo,
    private readonly items: OrderItem[],
    private readonly shippingInfo: ShippingInfo,
    private readonly paymentInfo: PaymentInfo,
    private readonly pricing: OrderPricing,
    private status: OrderStatus,
    private readonly timestamps: OrderTimestamps,
    private notes?: string,
  ) {}

  static create(
    id: OrderId,
    orderNumber: OrderNumber,
    customerInfo: CustomerInfo,
    items: OrderItem[],
    shippingInfo: ShippingInfo,
    paymentInfo: PaymentInfo,
  ): Order {
    if (items.length === 0) {
      throw new DomainError('Order must have at least one item');
    }

    // Calculate pricing
    const subtotal = items
      .map((item) => item.subtotal())
      .reduce((total, itemSubtotal) => total.add(itemSubtotal));

    const pricing = OrderPricing.calculate(
      subtotal,
      shippingInfo.getShippingFee(),
      paymentInfo.getPaymentFee(),
    );

    const timestamps = new OrderTimestamps(new Date(), new Date());

    return new Order(
      id,
      orderNumber,
      customerInfo,
      items,
      shippingInfo,
      paymentInfo,
      pricing,
      OrderStatus.Pending,
      timestamps,
    );
  }

  static createFromCommand(
    orderId: OrderId,
    orderNumber: OrderNumber,
    customerInfo: CustomerInfo,
    commandItems: Array<{ skuId: string; quantity: number }>,
    availableSkus: SKU[], // 既存のSKUエンティティを使用
    shippingMethodData: {
      id: string;
      name: string;
      fee: Money;
      isActive: boolean;
    },
    shippingAddress: Address,
    paymentMethodData: {
      id: string;
      name: string;
      fee: Money;
      isActive: boolean;
    },
  ): Order {
    // 既存のSKUエンティティのビジネスロジックを活用
    const orderItems = this.validateAndCreateOrderItems(
      commandItems,
      availableSkus,
    );

    // 拡張された既存クラスを使用
    const shippingInfo = ShippingInfo.createFromMethod(
      shippingMethodData,
      shippingAddress,
    );
    const paymentInfo = PaymentInfo.createFromMethod(paymentMethodData);

    return Order.create(
      orderId,
      orderNumber,
      customerInfo,
      orderItems,
      shippingInfo,
      paymentInfo,
    );
  }

  private static validateAndCreateOrderItems(
    commandItems: Array<{ skuId: string; quantity: number }>,
    availableSkus: SKU[],
  ): OrderItem[] {
    const orderItems: OrderItem[] = [];

    for (const commandItem of commandItems) {
      const sku = availableSkus.find(
        (s) => s.getId().value() === commandItem.skuId,
      );

      if (!sku) {
        throw new DomainError(`SKU not found: ${commandItem.skuId}`);
      }

      // 既存のSKUエンティティのビジネスロジックを活用
      if (!sku.isPurchasable()) {
        throw new DomainError(
          `SKU ${commandItem.skuId} is not available for purchase`,
        );
      }

      if (sku.availableQuantity() < commandItem.quantity) {
        throw new DomainError(
          `Insufficient stock for SKU ${commandItem.skuId}. ` +
            `Requested: ${commandItem.quantity}, Available: ${sku.availableQuantity()}`,
        );
      }

      const orderItem = OrderItem.create(
        sku.getId(),
        sku.getProductId(),
        sku.getName(),
        sku.getName(),
        sku.currentPrice(),
        commandItem.quantity,
      );

      orderItems.push(orderItem);
    }

    return orderItems;
  }

  // Status management
  updateStatus(newStatus: OrderStatus): void {
    this.validateStatusTransition(newStatus);

    // const oldStatus = this.status; // Reserved for future use (e.g., logging)
    this.status = newStatus;

    // Update timestamps based on status
    switch (newStatus) {
      case OrderStatus.Paid:
        this.timestamps.markPaid();
        break;
      case OrderStatus.Shipped:
        this.timestamps.markShipped();
        break;
      case OrderStatus.Delivered:
        this.timestamps.markDelivered();
        break;
      case OrderStatus.Cancelled:
        this.timestamps.markCancelled();
        break;
      default:
        this.timestamps.update();
        break;
    }
  }

  cancel(reason?: string): void {
    if (!this.canBeCancelled()) {
      throw new DomainError(`Cannot cancel order with status: ${this.status}`);
    }

    this.updateStatus(OrderStatus.Cancelled);

    if (reason && reason.trim().length > 0) {
      this.addNote(`Cancelled: ${reason.trim()}`);
    }
  }

  // Content management
  addNote(note: string): void {
    if (!note || note.trim().length === 0) {
      throw new DomainError('Note cannot be empty');
    }
    if (note.trim().length > 1000) {
      throw new DomainError('Note cannot exceed 1000 characters');
    }

    const trimmedNote = note.trim();
    if (this.notes) {
      this.notes += `\n${trimmedNote}`;
    } else {
      this.notes = trimmedNote;
    }

    this.timestamps.update();
  }

  // Business logic
  canBeCancelled(): boolean {
    return (
      this.status !== OrderStatus.Delivered &&
      this.status !== OrderStatus.Cancelled &&
      this.status !== OrderStatus.Refunded
    );
  }

  canBeModified(): boolean {
    return this.status === OrderStatus.Pending;
  }

  totalItemCount(): number {
    return this.items.reduce((total, item) => total + item.getQuantity(), 0);
  }

  private validateStatusTransition(newStatus: OrderStatus): void {
    const validTransitions: Record<OrderStatus, OrderStatus[]> = {
      [OrderStatus.Pending]: [OrderStatus.Paid, OrderStatus.Cancelled],
      [OrderStatus.Paid]: [OrderStatus.Processing, OrderStatus.Cancelled],
      [OrderStatus.Processing]: [OrderStatus.Shipped, OrderStatus.Cancelled],
      [OrderStatus.Shipped]: [OrderStatus.Delivered],
      [OrderStatus.Delivered]: [], // No transitions from delivered
      [OrderStatus.Cancelled]: [OrderStatus.Refunded],
      [OrderStatus.Refunded]: [], // No transitions from refunded
    };

    const allowedStatuses = validTransitions[this.status];
    if (!allowedStatuses.includes(newStatus)) {
      throw new DomainError(
        `Invalid status transition from ${this.status} to ${newStatus}`,
      );
    }
  }

  // Getters
  getId(): OrderId {
    return this.id;
  }

  getOrderNumber(): OrderNumber {
    return this.orderNumber;
  }

  getCustomerInfo(): CustomerInfo {
    return this.customerInfo;
  }

  getItems(): OrderItem[] {
    return [...this.items];
  }

  getShippingInfo(): ShippingInfo {
    return this.shippingInfo;
  }

  getPaymentInfo(): PaymentInfo {
    return this.paymentInfo;
  }

  getPricing(): OrderPricing {
    return this.pricing;
  }

  getStatus(): OrderStatus {
    return this.status;
  }

  getTimestamps(): OrderTimestamps {
    return this.timestamps;
  }

  getNotes(): string | undefined {
    return this.notes;
  }
}
