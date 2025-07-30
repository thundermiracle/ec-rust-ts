import { Inject } from '@nestjs/common';
import { CommandHandler, ICommandHandler } from '@nestjs/cqrs';

import { CalculateCartCommand } from '$application/commands';
import {
  CalculateCartResultDto,
  CalculatedCartItemDto,
  VariantSummaryDto,
} from '$application/dto';
import {
  BusinessRuleViolationError,
  InsufficientStockError,
  NotFoundError,
  ValidationError,
} from '$application/errors/application.error';
import {
  IPaymentMethodRepository,
  IProductRepository,
  IShippingMethodRepository,
} from '$application/repositories';
import { Cart, CartItem } from '$domain/aggregates';
import {
  Money,
  PaymentMethodId,
  ProductId,
  ShippingMethodId,
  SKUId,
} from '$domain/value-objects';

@CommandHandler(CalculateCartCommand)
export class CalculateCartHandler
  implements ICommandHandler<CalculateCartCommand>
{
  constructor(
    @Inject('IProductRepository')
    private readonly productRepository: IProductRepository,
    @Inject('IShippingMethodRepository')
    private readonly shippingMethodRepository: IShippingMethodRepository,
    @Inject('IPaymentMethodRepository')
    private readonly paymentMethodRepository: IPaymentMethodRepository,
  ) {}

  async execute(
    command: CalculateCartCommand,
  ): Promise<CalculateCartResultDto> {
    // 1. Input Validation
    this.validateInput(command);

    // 2. Parse SKU IDs and fetch variants
    const skuIds = this.parseSkuIds(command.items);
    const skus = await this.productRepository.findSkusByIds(skuIds);

    // 3. Validate existence and business rules
    this.validateSkusAndQuantities(command.items, skus);

    // 4. Fetch shipping and payment methods
    const shippingMethod = await this.getShippingMethod(
      command.shippingMethodId,
    );
    const paymentMethod = await this.getPaymentMethod(command.paymentMethodId);

    // 5. Create domain objects and calculate
    const cart = this.createCartWithItems(command.items, skus);
    cart.applyShippingMethod(shippingMethod);
    cart.applyPaymentMethod(paymentMethod);

    // 6. Convert to DTO
    return this.createResultDto(cart);
  }

  private validateInput(command: CalculateCartCommand): void {
    if (!command.items || command.items.length === 0) {
      throw new ValidationError('Cart items are required');
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

    for (const item of command.items) {
      if (!item.skuId || item.skuId.trim().length === 0) {
        throw new ValidationError('SKU ID is required for all items');
      }
      if (item.quantity <= 0) {
        throw new ValidationError('Quantity must be positive for all items');
      }
    }
  }

  private parseSkuIds(
    items: Array<{ skuId: string; quantity: number }>,
  ): SKUId[] {
    return items.map((item) => {
      try {
        return SKUId.fromUuid(item.skuId);
      } catch {
        throw new ValidationError(`Invalid SKU ID format: ${item.skuId}`);
      }
    });
  }

  private validateSkusAndQuantities(
    items: Array<{ skuId: string; quantity: number }>,
    skus: VariantSummaryDto[],
  ): void {
    // Check all SKUs exist
    for (const item of items) {
      const sku = skus.find((s) => s.id === item.skuId);
      if (!sku) {
        throw new NotFoundError('SKU', item.skuId);
      }

      // Check if SKU is purchasable
      if (!sku.isInStock) {
        throw new BusinessRuleViolationError(
          `SKU ${item.skuId} is not available for purchase`,
        );
      }

      // Check stock availability
      if (sku.stockQuantity < item.quantity) {
        throw new InsufficientStockError(
          item.skuId,
          item.quantity,
          sku.stockQuantity,
        );
      }
    }
  }

  private async getShippingMethod(shippingMethodId: string) {
    const methodId = ShippingMethodId.new(shippingMethodId);
    const method = await this.shippingMethodRepository.findById(methodId);
    if (!method) {
      throw new NotFoundError('Shipping method', shippingMethodId);
    }
    return method;
  }

  private async getPaymentMethod(paymentMethodId: string) {
    const methodId = PaymentMethodId.new(paymentMethodId);
    const method = await this.paymentMethodRepository.findById(methodId);
    if (!method) {
      throw new NotFoundError('Payment method', paymentMethodId);
    }
    return method;
  }

  private createCartWithItems(
    commandItems: Array<{ skuId: string; quantity: number }>,
    skus: VariantSummaryDto[],
  ): Cart {
    const cart = new Cart();

    for (const commandItem of commandItems) {
      const sku = skus.find((s) => s.id === commandItem.skuId);
      if (!sku) {
        throw new NotFoundError('SKU', commandItem.skuId);
      }

      const cartItem = CartItem.create(
        SKUId.fromUuid(sku.id),
        ProductId.fromUuid(sku.productId),
        sku.productName,
        Money.fromYen(sku.currentPrice),
        commandItem.quantity,
      );

      cart.addItem(cartItem);
    }

    return cart;
  }

  private createResultDto(cart: Cart): CalculateCartResultDto {
    const items = cart
      .getAllItems()
      .map(
        (item) =>
          new CalculatedCartItemDto(
            item.getSkuId().value(),
            item.getProductId().value(),
            item.getProductName(),
            item.getUnitPrice().yen(),
            item.getQuantity(),
            item.subtotal().yen(),
          ),
      );

    return new CalculateCartResultDto(
      items,
      cart.totalQuantity(),
      cart.itemCount(),
      cart.subtotal().yen(),
      cart.taxAmount().yen(),
      cart.total().yen(),
      cart.isEmpty(),
      cart.shippingFee().yen(),
      cart.paymentFee().yen(),
    );
  }
}
