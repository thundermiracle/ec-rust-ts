import { DomainError } from '$domain/errors/domain.error';
import { ColorId, Money, ProductId, SKUId } from '$domain/value-objects';

export enum SKUStatus {
  Active = 'Active',
  Inactive = 'Inactive',
  Discontinued = 'Discontinued',
}

export interface VariantAttributes {
  colorId?: ColorId;
  dimensions?: string;
  material?: string;
}

export class Stock {
  constructor(
    private totalQuantity: number,
    private reservedQuantity: number = 0,
  ) {
    if (totalQuantity < 0) {
      throw new DomainError('Total quantity cannot be negative');
    }
    if (reservedQuantity < 0) {
      throw new DomainError('Reserved quantity cannot be negative');
    }
    if (reservedQuantity > totalQuantity) {
      throw new DomainError('Reserved quantity cannot exceed total quantity');
    }
  }

  availableQuantity(): number {
    return this.totalQuantity - this.reservedQuantity;
  }

  reserve(quantity: number): void {
    if (quantity <= 0) {
      throw new DomainError('Reservation quantity must be positive');
    }
    if (quantity > this.availableQuantity()) {
      throw new DomainError('Insufficient stock for reservation');
    }
    this.reservedQuantity += quantity;
  }

  releaseReservation(quantity: number): void {
    if (quantity <= 0) {
      throw new DomainError('Release quantity must be positive');
    }
    if (quantity > this.reservedQuantity) {
      throw new DomainError('Cannot release more than reserved');
    }
    this.reservedQuantity -= quantity;
  }

  adjust(adjustment: number): void {
    const newTotal = this.totalQuantity + adjustment;
    if (newTotal < 0) {
      throw new DomainError(
        'Stock adjustment would result in negative quantity',
      );
    }
    if (newTotal < this.reservedQuantity) {
      throw new DomainError(
        'Stock adjustment would result in insufficient stock for reservations',
      );
    }
    this.totalQuantity = newTotal;
  }

  isLowStock(threshold: number = 10): boolean {
    return this.availableQuantity() <= threshold;
  }

  isOutOfStock(): boolean {
    return this.availableQuantity() === 0;
  }

  getTotalQuantity(): number {
    return this.totalQuantity;
  }

  getReservedQuantity(): number {
    return this.reservedQuantity;
  }
}

export class SKU {
  private constructor(
    private readonly id: SKUId,
    private readonly productId: ProductId,
    private readonly skuCode: string,
    private name: string,
    private readonly variantAttributes: VariantAttributes,
    private basePrice: Money,
    private salePrice: Money | null,
    private stock: Stock,
    private status: SKUStatus,
    private displayOrder: number,
  ) {}

  static create(
    id: SKUId,
    productId: ProductId,
    skuCode: string,
    name: string,
    variantAttributes: VariantAttributes,
    basePrice: Money,
    initialStock: number,
    displayOrder: number = 0,
  ): SKU {
    if (!skuCode || skuCode.trim().length === 0) {
      throw new DomainError('SKU code cannot be empty');
    }
    if (!name || name.trim().length === 0) {
      throw new DomainError('SKU name cannot be empty');
    }
    if (!basePrice.isPositive()) {
      throw new DomainError('Base price must be positive');
    }

    return new SKU(
      id,
      productId,
      skuCode.trim(),
      name.trim(),
      variantAttributes,
      basePrice,
      null, // salePrice
      new Stock(initialStock),
      SKUStatus.Active,
      displayOrder,
    );
  }

  // Stock management
  adjustStock(adjustment: number): void {
    this.stock.adjust(adjustment);
  }

  reserveStock(quantity: number): void {
    if (!this.isPurchasable()) {
      throw new DomainError('Cannot reserve stock for non-purchasable SKU');
    }
    this.stock.reserve(quantity);
  }

  releaseReservation(quantity: number): void {
    this.stock.releaseReservation(quantity);
  }

  // Pricing
  setSalePrice(price: Money): void {
    if (price.yen() >= this.basePrice.yen()) {
      throw new DomainError('Sale price must be less than base price');
    }
    this.salePrice = price;
  }

  clearSalePrice(): void {
    this.salePrice = null;
  }

  currentPrice(): Money {
    return this.salePrice || this.basePrice;
  }

  discountPercentage(): number | null {
    if (!this.salePrice) {
      return null;
    }
    const discount = this.basePrice.subtract(this.salePrice);
    return Math.round((discount.yen() / this.basePrice.yen()) * 100);
  }

  savingsAmount(): Money {
    if (!this.salePrice) {
      return Money.zero();
    }
    return this.basePrice.subtract(this.salePrice);
  }

  // Status management
  activate(): void {
    this.status = SKUStatus.Active;
  }

  deactivate(): void {
    this.status = SKUStatus.Inactive;
  }

  discontinue(): void {
    this.status = SKUStatus.Discontinued;
  }

  // Business logic
  isPurchasable(): boolean {
    return this.status === SKUStatus.Active && !this.stock.isOutOfStock();
  }

  isLowStock(): boolean {
    return this.stock.isLowStock();
  }

  isOutOfStock(): boolean {
    return this.stock.isOutOfStock();
  }

  availableQuantity(): number {
    return this.stock.availableQuantity();
  }

  fullDisplayName(): string {
    const attributes: string[] = [];

    if (this.variantAttributes.colorId) {
      // Note: In real implementation, you'd resolve color name from repository
      attributes.push(`Color: ${this.variantAttributes.colorId.toString()}`);
    }
    if (this.variantAttributes.dimensions) {
      attributes.push(`Size: ${this.variantAttributes.dimensions}`);
    }
    if (this.variantAttributes.material) {
      attributes.push(`Material: ${this.variantAttributes.material}`);
    }

    if (attributes.length === 0) {
      return this.name;
    }

    return `${this.name} (${attributes.join(', ')})`;
  }

  isSimpleSku(): boolean {
    return (
      !this.variantAttributes.colorId &&
      !this.variantAttributes.dimensions &&
      !this.variantAttributes.material
    );
  }

  // Getters
  getId(): SKUId {
    return this.id;
  }

  getProductId(): ProductId {
    return this.productId;
  }

  code(): string {
    return this.skuCode;
  }

  getName(): string {
    return this.name;
  }

  colorId(): ColorId | undefined {
    return this.variantAttributes.colorId;
  }

  dimensions(): string | undefined {
    return this.variantAttributes.dimensions;
  }

  material(): string | undefined {
    return this.variantAttributes.material;
  }

  getBasePrice(): Money {
    return this.basePrice;
  }

  getSalePrice(): Money | null {
    return this.salePrice;
  }

  getStatus(): SKUStatus {
    return this.status;
  }

  getDisplayOrder(): number {
    return this.displayOrder;
  }

  getStock(): Stock {
    return this.stock;
  }
}
