import { CommandHandler, ICommandHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { CreateOrderCommand } from '../models/create-order.command';
import { CreateOrderResultDto } from '../../dto/create-order-result.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { IShippingMethodRepository } from '../../repositories/shipping-method.repository.interface';
import { IPaymentMethodRepository } from '../../repositories/payment-method.repository.interface';
import { IOrderRepository } from '../../repositories/order.repository.interface';
import {
  Order,
  OrderItem,
  CustomerInfo,
  ShippingInfo,
  PaymentInfo,
} from '../../../domain/aggregates';
import {
  OrderId,
  SKUId,
  ShippingMethodId,
  PaymentMethodId,
  Address,
} from '../../../domain/value-objects';
import {
  ValidationError,
  NotFoundError,
  BusinessRuleViolationError,
  InsufficientStockError,
} from '../../errors/application.error';

@CommandHandler(CreateOrderCommand)
export class CreateOrderHandler implements ICommandHandler<CreateOrderCommand> {
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
    @Inject('IShippingMethodRepository')
    private readonly shippingMethodRepository: IShippingMethodRepository,
    @Inject('IPaymentMethodRepository')
    private readonly paymentMethodRepository: IPaymentMethodRepository,
    @Inject('IOrderRepository')
    private readonly orderRepository: IOrderRepository,
  ) {}

  async execute(command: CreateOrderCommand): Promise<CreateOrderResultDto> {
    // 1. Input Validation
    this.validateInput(command);

    // 2. Create Customer Info
    const customerInfo = this.createCustomerInfo(command);

    // 3. Create Order Items
    const orderItems = await this.createOrderItems(command.items);

    // 4. Create Shipping Info
    const shippingInfo = await this.createShippingInfo(command);

    // 5. Create Payment Info
    const paymentInfo = await this.createPaymentInfo(command.paymentMethodId);

    // 6. Generate Order Number and Create Order
    const orderId = OrderId.new();
    const orderNumber = await this.orderRepository.generateOrderNumber();

    const order = Order.create(
      orderId,
      orderNumber,
      customerInfo,
      orderItems,
      shippingInfo,
      paymentInfo,
    );

    // 7. Save Order
    await this.orderRepository.save(order);

    // 8. Return Result DTO
    return new CreateOrderResultDto(
      order.getId().value(),
      order.getOrderNumber().getValue(),
      order.getPricing().getTotal().yen(),
      order.getStatus().toString(),
      order.getTimestamps().createdAt.toISOString(),
    );
  }

  private validateInput(command: CreateOrderCommand): void {
    if (!command.items || command.items.length === 0) {
      throw new ValidationError('Order must have at least one item');
    }

    if (!command.customerInfo) {
      throw new ValidationError('Customer information is required');
    }

    if (!command.shippingAddress) {
      throw new ValidationError('Shipping address is required');
    }

    if (
      !command.shippingMethodId ||
      command.shippingMethodId.trim().length === 0
    ) {
      throw new ValidationError('Shipping method ID is required');
    }

    if (
      !command.paymentMethodId ||
      command.paymentMethodId.trim().length === 0
    ) {
      throw new ValidationError('Payment method ID is required');
    }
  }

  private createCustomerInfo(command: CreateOrderCommand): CustomerInfo {
    return CustomerInfo.create(
      command.customerInfo.firstName,
      command.customerInfo.lastName,
      command.customerInfo.email,
      command.customerInfo.phone,
    );
  }

  private async createOrderItems(
    commandItems: Array<{ skuId: string; quantity: number }>,
  ): Promise<OrderItem[]> {
    // Parse SKU IDs
    const skuIds = commandItems.map((item) => {
      try {
        return SKUId.fromUuid(item.skuId);
      } catch {
        throw new ValidationError(`Invalid SKU ID format: ${item.skuId}`);
      }
    });

    // Fetch SKUs
    const skus = await this.productRepository.findSkusByIds(skuIds);

    // Create order items with validation
    const orderItems: OrderItem[] = [];

    for (const commandItem of commandItems) {
      const sku = skus.find((s) => s.getId().value() === commandItem.skuId);

      if (!sku) {
        throw new NotFoundError('SKU', commandItem.skuId);
      }

      if (!sku.isPurchasable()) {
        throw new BusinessRuleViolationError(
          `SKU ${commandItem.skuId} is not available for purchase`,
        );
      }

      if (sku.availableQuantity() < commandItem.quantity) {
        throw new InsufficientStockError(
          commandItem.skuId,
          commandItem.quantity,
          sku.availableQuantity(),
        );
      }

      // Reserve stock
      sku.reserveStock(commandItem.quantity);

      const orderItem = OrderItem.create(
        sku.getId(),
        sku.getProductId(),
        sku.getName(), // This should be product name from join
        sku.getName(), // This should be SKU display name
        sku.currentPrice(),
        commandItem.quantity,
      );

      orderItems.push(orderItem);
    }

    return orderItems;
  }

  private async createShippingInfo(
    command: CreateOrderCommand,
  ): Promise<ShippingInfo> {
    // Fetch shipping method
    const methodId = ShippingMethodId.new(command.shippingMethodId);
    const shippingMethod =
      await this.shippingMethodRepository.findById(methodId);

    if (!shippingMethod) {
      throw new NotFoundError('Shipping method', command.shippingMethodId);
    }

    // Create shipping address
    const address = Address.new(
      command.shippingAddress.postalCode,
      command.shippingAddress.prefecture,
      command.shippingAddress.city,
      command.shippingAddress.streetAddress,
      command.shippingAddress.building,
    );

    return ShippingInfo.create(
      shippingMethod.id,
      shippingMethod.name,
      shippingMethod.fee,
      address,
    );
  }

  private async createPaymentInfo(
    paymentMethodId: string,
  ): Promise<PaymentInfo> {
    const methodId = PaymentMethodId.new(paymentMethodId);
    const paymentMethod = await this.paymentMethodRepository.findById(methodId);

    if (!paymentMethod) {
      throw new NotFoundError('Payment method', paymentMethodId);
    }

    return PaymentInfo.create(
      paymentMethod.id,
      paymentMethod.name,
      paymentMethod.fee,
    );
  }
}
