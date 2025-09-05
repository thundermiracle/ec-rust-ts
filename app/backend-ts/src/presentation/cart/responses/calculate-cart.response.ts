import { ApiProperty } from '@nestjs/swagger';

export class AppliedCouponResponse {
  @ApiProperty({ description: 'Coupon code' })
  couponCode: string;

  @ApiProperty({ description: 'Coupon name' })
  couponName: string;

  @ApiProperty({ description: 'Discount amount applied' })
  discountAmount: number;

  @ApiProperty({ description: 'Message describing the applied coupon' })
  message: string;

  constructor(data: {
    couponCode: string;
    couponName: string;
    discountAmount: number;
    message: string;
  }) {
    this.couponCode = data.couponCode;
    this.couponName = data.couponName;
    this.discountAmount = data.discountAmount;
    this.message = data.message;
  }
}

export class CouponErrorResponse {
  @ApiProperty({ description: 'Coupon code that failed', required: false })
  couponCode?: string;

  @ApiProperty({ description: 'Error message' })
  errorMessage: string;

  constructor(data: { couponCode?: string; errorMessage: string }) {
    this.couponCode = data.couponCode;
    this.errorMessage = data.errorMessage;
  }
}

export class CalculatedCartItemResponse {
  @ApiProperty({ description: 'SKU ID of the product variant' })
  skuId: string;

  @ApiProperty({ description: 'Product ID' })
  productId: string;

  @ApiProperty({ description: 'Product name' })
  productName: string;

  @ApiProperty({ description: 'Unit price in original currency units' })
  unitPrice: number;

  @ApiProperty({ description: 'Quantity ordered' })
  quantity: number;

  @ApiProperty({ description: 'Subtotal (unitPrice * quantity)' })
  subtotal: number;

  constructor(data: {
    skuId: string;
    productId: string;
    productName: string;
    unitPrice: number;
    quantity: number;
    subtotal: number;
  }) {
    this.skuId = data.skuId;
    this.productId = data.productId;
    this.productName = data.productName;
    this.unitPrice = data.unitPrice;
    this.quantity = data.quantity;
    this.subtotal = data.subtotal;
  }
}

export class CalculateCartResponse {
  @ApiProperty({
    description: 'List of calculated cart items',
    type: [CalculatedCartItemResponse],
  })
  items: CalculatedCartItemResponse[];

  @ApiProperty({ description: 'Total quantity of all items' })
  totalQuantity: number;

  @ApiProperty({ description: 'Number of different items in cart' })
  itemCount: number;

  @ApiProperty({ description: 'Subtotal (sum of all item subtotals)' })
  subtotal: number;

  @ApiProperty({ description: 'Tax amount' })
  taxAmount: number;

  @ApiProperty({ description: 'Subtotal with discount applied' })
  subtotalWithTax: number;

  @ApiProperty({ description: 'Total amount' })
  total: number;

  @ApiProperty({ description: 'Whether the cart is empty' })
  isEmpty: boolean;

  @ApiProperty({ description: 'Shipping fee' })
  shippingFee: number;

  @ApiProperty({ description: 'Payment processing fee' })
  paymentFee: number;

  @ApiProperty({ description: 'Applied coupon information', required: false })
  appliedCoupon?: AppliedCouponResponse;

  @ApiProperty({ description: 'Coupon error information', required: false })
  couponError?: CouponErrorResponse;

  constructor(data: {
    items: CalculatedCartItemResponse[];
    totalQuantity: number;
    itemCount: number;
    subtotal: number;
    taxAmount: number;
    subtotalWithTax: number;
    total: number;
    isEmpty: boolean;
    shippingFee: number;
    paymentFee: number;
    appliedCoupon?: AppliedCouponResponse;
    couponError?: CouponErrorResponse;
  }) {
    this.items = data.items;
    this.totalQuantity = data.totalQuantity;
    this.itemCount = data.itemCount;
    this.subtotal = data.subtotal;
    this.taxAmount = data.taxAmount;
    this.subtotalWithTax = data.subtotalWithTax;
    this.total = data.total;
    this.isEmpty = data.isEmpty;
    this.shippingFee = data.shippingFee;
    this.paymentFee = data.paymentFee;
    this.appliedCoupon = data.appliedCoupon;
    this.couponError = data.couponError;
  }
}
