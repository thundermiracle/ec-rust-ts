import {
  Order,
  OrderItem,
  CustomerInfo,
  ShippingInfo,
  PaymentInfo,
  OrderStatus,
} from '../../../domain/aggregates';
import {
  OrderId,
  OrderNumber,
  SKUId,
  ProductId,
  ShippingMethodId,
  PaymentMethodId,
  Address,
  Money,
} from '../../../domain/value-objects';
import { OrderEntity } from '../entities/order.entity';
import { OrderItemEntity } from '../entities/order-item.entity';

export class OrderMapper {
  static toDomain(entity: OrderEntity): Order {
    const orderId = OrderId.fromUuid(entity.id);
    const orderNumber = OrderNumber.fromString(entity.order_number);

    const customerInfo = CustomerInfo.create(
      entity.customer_first_name,
      entity.customer_last_name,
      entity.customer_email,
      entity.customer_phone,
    );

    const shippingAddress = Address.new(
      entity.shipping_postal_code,
      entity.shipping_prefecture,
      entity.shipping_city,
      entity.shipping_street,
      entity.shipping_building,
    );

    const shippingInfo = ShippingInfo.create(
      ShippingMethodId.new(entity.shipping_method_id),
      entity.shipping_method_name,
      Money.fromYen(entity.shipping_fee),
      shippingAddress,
    );

    const paymentInfo = PaymentInfo.create(
      PaymentMethodId.new(entity.payment_method_id),
      entity.payment_method_name,
      Money.fromYen(entity.payment_fee),
    );

    const orderItems = entity.items.map((item) =>
      OrderItemMapper.toDomain(item),
    );

    const order = Order.create(
      orderId,
      orderNumber,
      customerInfo,
      orderItems,
      shippingInfo,
      paymentInfo,
    );

    // Set status
    const status = OrderStatus[entity.status as keyof typeof OrderStatus];
    if (status) {
      order.updateStatus(status);
    }

    // Set notes if present
    if (entity.notes) {
      order.addNote(entity.notes);
    }

    return order;
  }

  static toEntity(domain: Order): OrderEntity {
    const entity = new OrderEntity();
    entity.id = domain.getId().value();
    entity.order_number = domain.getOrderNumber().getValue();

    const customerInfo = domain.getCustomerInfo();
    entity.customer_first_name = customerInfo.getFirstName();
    entity.customer_last_name = customerInfo.getLastName();
    entity.customer_email = customerInfo.getEmail().value();
    entity.customer_phone = customerInfo.getPhoneNumber().value();

    const shippingInfo = domain.getShippingInfo();
    const address = shippingInfo.getShippingAddress();
    entity.shipping_postal_code = address.getPostalCode();
    entity.shipping_prefecture = address.getPrefecture();
    entity.shipping_city = address.getCity();
    entity.shipping_street = address.getStreet();
    entity.shipping_building = address.getBuilding();
    entity.shipping_method_id = shippingInfo.getShippingMethodId().value();
    entity.shipping_method_name = shippingInfo.getShippingMethodName();
    entity.shipping_fee = shippingInfo.getShippingFee().yen();

    const paymentInfo = domain.getPaymentInfo();
    entity.payment_method_id = paymentInfo.getPaymentMethodId().value();
    entity.payment_method_name = paymentInfo.getPaymentMethodName();
    entity.payment_fee = paymentInfo.getPaymentFee().yen();

    const pricing = domain.getPricing();
    entity.subtotal = pricing.getSubtotal().yen();
    entity.tax_amount = pricing.getTaxAmount().yen();
    entity.total = pricing.getTotal().yen();

    entity.status = domain.getStatus().toString();
    entity.notes = domain.getNotes();

    const timestamps = domain.getTimestamps();
    entity.created_at = timestamps.createdAt;
    entity.updated_at = timestamps.updatedAt;
    entity.paid_at = timestamps.paidAt;
    entity.shipped_at = timestamps.shippedAt;
    entity.delivered_at = timestamps.deliveredAt;
    entity.cancelled_at = timestamps.cancelledAt;

    return entity;
  }
}

export class OrderItemMapper {
  static toDomain(entity: OrderItemEntity): OrderItem {
    return OrderItem.create(
      SKUId.fromUuid(entity.sku_id),
      ProductId.fromUuid(entity.product_id),
      entity.product_name,
      entity.sku_name,
      Money.fromYen(entity.unit_price),
      entity.quantity,
    );
  }

  static toEntity(domain: OrderItem, orderId: string): OrderItemEntity {
    const entity = new OrderItemEntity();
    entity.order_id = orderId;
    entity.sku_id = domain.getSkuId().value();
    entity.product_id = domain.getProductId().value();
    entity.product_name = domain.getProductName();
    entity.sku_name = domain.getSkuName();
    entity.unit_price = domain.getUnitPrice().yen();
    entity.quantity = domain.getQuantity();
    entity.subtotal = domain.subtotal().yen();
    return entity;
  }
}
