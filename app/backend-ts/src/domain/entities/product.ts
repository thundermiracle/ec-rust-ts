import { ProductId, CategoryId, ColorId } from '../value-objects/identifiers';
import { Money } from '../value-objects/money';
import { DomainError } from '../errors/domain.error';
import { SKU } from './sku';
import { ProductImage } from './product-image';
import { Tag } from './tag';

export class Product {
  private constructor(
    private readonly id: ProductId,
    private name: string,
    private description: string,
    private readonly categoryId: CategoryId,
    private isBestSeller: boolean,
    private isQuickShip: boolean,
    private isAvailable: boolean,
    private readonly skus: Map<string, SKU> = new Map(),
    private readonly images: ProductImage[] = [],
    private readonly tags: Tag[] = [],
    private readonly createdAt: Date = new Date(),
    private updatedAt: Date = new Date(),
  ) {}

  static create(
    id: ProductId,
    name: string,
    description: string,
    categoryId: CategoryId,
  ): Product {
    if (!name || name.trim().length === 0) {
      throw new DomainError('Product name cannot be empty');
    }
    if (!description || description.trim().length === 0) {
      throw new DomainError('Product description cannot be empty');
    }

    return new Product(
      id,
      name.trim(),
      description.trim(),
      categoryId,
      false, // isBestSeller
      false, // isQuickShip
      true, // isAvailable
    );
  }

  // SKU Management
  addSku(sku: SKU): void {
    if (this.skus.has(sku.code())) {
      throw new DomainError(`SKU with code ${sku.code()} already exists`);
    }
    if (!sku.getProductId().equals(this.id)) {
      throw new DomainError('SKU does not belong to this product');
    }
    this.skus.set(sku.getId().value(), sku);
    this.updatedAt = new Date();
  }

  removeSku(skuId: string): SKU | null {
    const sku = this.skus.get(skuId);
    if (sku) {
      this.skus.delete(skuId);
      this.updatedAt = new Date();
      return sku;
    }
    return null;
  }

  findSkuById(skuId: string): SKU | null {
    return this.skus.get(skuId) || null;
  }

  // Product-level logic
  hasVariants(): boolean {
    return this.skus.size > 1;
  }

  isAvailableForPurchase(): boolean {
    if (!this.isAvailable) {
      return false;
    }
    return Array.from(this.skus.values()).some((sku) => sku.isPurchasable());
  }

  totalAvailableStock(): number {
    return Array.from(this.skus.values())
      .filter((sku) => sku.isPurchasable())
      .reduce((total, sku) => total + sku.availableQuantity(), 0);
  }

  priceRange(): [Money, Money] | null {
    const purchasableSkus = Array.from(this.skus.values()).filter((sku) =>
      sku.isPurchasable(),
    );

    if (purchasableSkus.length === 0) {
      return null;
    }

    const prices = purchasableSkus.map((sku) => sku.currentPrice());
    const minPrice = prices.reduce((min, price) =>
      price.yen() < min.yen() ? price : min,
    );
    const maxPrice = prices.reduce((max, price) =>
      price.yen() > max.yen() ? price : max,
    );

    return [minPrice, maxPrice];
  }

  availableColors(): ColorId[] {
    const colors = new Set<number>();
    Array.from(this.skus.values())
      .filter((sku) => sku.isPurchasable())
      .forEach((sku) => {
        const colorId = sku.colorId();
        if (colorId) {
          colors.add(colorId.value());
        }
      });

    return Array.from(colors).map((id) => ColorId.new(id));
  }

  lowStockSkus(): SKU[] {
    return Array.from(this.skus.values()).filter((sku) => sku.isLowStock());
  }

  // Status management
  publish(): void {
    if (this.skus.size === 0) {
      throw new DomainError('Cannot publish product without SKUs');
    }
    if (!this.isAvailableForPurchase()) {
      throw new DomainError('Cannot publish product without purchasable SKUs');
    }
    this.isAvailable = true;
    this.updatedAt = new Date();
  }

  discontinue(): void {
    this.isAvailable = false;
    // Discontinue all SKUs
    Array.from(this.skus.values()).forEach((sku) => sku.discontinue());
    this.updatedAt = new Date();
  }

  markAsBestSeller(): void {
    this.isBestSeller = true;
    this.updatedAt = new Date();
  }

  unmarkAsBestSeller(): void {
    this.isBestSeller = false;
    this.updatedAt = new Date();
  }

  // Getters
  getId(): ProductId {
    return this.id;
  }

  getName(): string {
    return this.name;
  }

  getDescription(): string {
    return this.description;
  }

  getCategoryId(): CategoryId {
    return this.categoryId;
  }

  getIsBestSeller(): boolean {
    return this.isBestSeller;
  }

  getIsQuickShip(): boolean {
    return this.isQuickShip;
  }

  getIsAvailable(): boolean {
    return this.isAvailable;
  }

  getSkus(): SKU[] {
    return Array.from(this.skus.values());
  }

  getImages(): ProductImage[] {
    return [...this.images];
  }

  getTags(): Tag[] {
    return [...this.tags];
  }

  getCreatedAt(): Date {
    return this.createdAt;
  }

  getUpdatedAt(): Date {
    return this.updatedAt;
  }
}
