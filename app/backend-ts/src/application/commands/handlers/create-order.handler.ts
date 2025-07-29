import { CommandHandler, ICommandHandler } from '@nestjs/cqrs';
import { Inject } from '@nestjs/common';
import { CreateOrderCommand } from '../models/create-order.command';
import { CreateOrderResultDto } from '../../dto/create-order-result.dto';
import { IProductRepository } from '../../repositories/product.repository.interface';
import { IShippingMethodRepository } from '../../repositories/shipping-method.repository.interface';
import { IPaymentMethodRepository } from '../../repositories/payment-method.repository.interface';
import { IOrderRepository } from '../../repositories/order.repository.interface';
import { Order, CustomerInfo } from '../../../domain/aggregates';
import {
  OrderId,
  SKUId,
  ShippingMethodId,
  PaymentMethodId,
  Address,
} from '../../../domain/value-objects';
import { ValidationError, NotFoundError } from '../../errors/application.error';

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

    // 2. SKUエンティティを取得（DTO使用を廃止）
    const skus = await this.getSkusWithValidation(command.items);

    // 3. 配送・支払方法データを取得（既存のrepositoryメソッドを使用）
    const shippingMethodData = await this.getShippingMethodData(
      command.shippingMethodId,
    );
    const paymentMethodData = await this.getPaymentMethodData(
      command.paymentMethodId,
    );

    // 4. 顧客情報・配送先住所を作成
    const customerInfo = this.createCustomerInfo(command);
    const shippingAddress = this.createShippingAddress(command.shippingAddress);

    // 5. Order集約で全体を組み立て
    const orderId = OrderId.new();
    const orderNumber = await this.orderRepository.generateOrderNumber();

    const order = Order.createFromCommand(
      orderId,
      orderNumber,
      customerInfo,
      command.items,
      skus, // SKUエンティティを直接使用
      shippingMethodData, // プレーンオブジェクト
      shippingAddress,
      paymentMethodData, // プレーンオブジェクト
    );

    // 6. Save Order
    await this.orderRepository.save(order);

    // 7. Return Result DTO
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

  private async getSkusWithValidation(
    commandItems: Array<{ skuId: string; quantity: number }>,
  ) {
    const skuIds = commandItems.map((item) => {
      try {
        return SKUId.fromUuid(item.skuId);
      } catch {
        throw new ValidationError(`Invalid SKU ID format: ${item.skuId}`);
      }
    });

    return await this.productRepository.findSkuEntitiesByIds(skuIds);
  }

  private async getShippingMethodData(shippingMethodId: string) {
    const methodId = ShippingMethodId.new(shippingMethodId);
    const shippingMethod =
      await this.shippingMethodRepository.findById(methodId);

    if (!shippingMethod) {
      throw new NotFoundError('Shipping method', shippingMethodId);
    }

    return {
      id: shippingMethod.getId().value(),
      name: shippingMethod.getName(),
      fee: shippingMethod.getFee(),
      isActive: shippingMethod.isAvailable(),
    };
  }

  private async getPaymentMethodData(paymentMethodId: string) {
    const methodId = PaymentMethodId.new(paymentMethodId);
    const paymentMethod = await this.paymentMethodRepository.findById(methodId);

    if (!paymentMethod) {
      throw new NotFoundError('Payment method', paymentMethodId);
    }

    return {
      id: paymentMethod.getId().value(),
      name: paymentMethod.getName(),
      fee: paymentMethod.getFee(),
      isActive: paymentMethod.isAvailable(),
    };
  }

  private createShippingAddress(addressData: {
    postalCode: string;
    prefecture: string;
    city: string;
    streetAddress: string;
    building?: string;
  }): Address {
    return Address.new(
      addressData.postalCode,
      addressData.prefecture,
      addressData.city,
      addressData.streetAddress,
      addressData.building,
    );
  }
}
