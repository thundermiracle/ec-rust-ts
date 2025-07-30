import { DomainError } from '$domain/errors/domain.error';
import { ProductId } from '$domain/value-objects';

export class ProductImage {
  private constructor(
    private readonly id: string,
    private readonly productId: ProductId,
    private url: string,
    private altText: string,
    private displayOrder: number,
    private readonly isMain: boolean = false,
  ) {}

  static create(
    id: string,
    productId: ProductId,
    url: string,
    altText: string,
    displayOrder: number = 0,
    isMain: boolean = false,
  ): ProductImage {
    if (!id || id.trim().length === 0) {
      throw new DomainError('Product image ID cannot be empty');
    }
    if (!url || url.trim().length === 0) {
      throw new DomainError('Product image URL cannot be empty');
    }
    if (!ProductImage.isValidUrl(url)) {
      throw new DomainError('Product image URL is not valid');
    }
    if (!altText || altText.trim().length === 0) {
      throw new DomainError('Product image alt text cannot be empty');
    }

    return new ProductImage(
      id.trim(),
      productId,
      url.trim(),
      altText.trim(),
      displayOrder,
      isMain,
    );
  }

  private static isValidUrl(url: string): boolean {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  }

  // Getters
  getId(): string {
    return this.id;
  }

  getProductId(): ProductId {
    return this.productId;
  }

  getUrl(): string {
    return this.url;
  }

  getAltText(): string {
    return this.altText;
  }

  getDisplayOrder(): number {
    return this.displayOrder;
  }

  getIsMain(): boolean {
    return this.isMain;
  }

  // Update methods
  updateUrl(url: string): void {
    if (!url || url.trim().length === 0) {
      throw new DomainError('Product image URL cannot be empty');
    }
    if (!ProductImage.isValidUrl(url)) {
      throw new DomainError('Product image URL is not valid');
    }
    this.url = url.trim();
  }

  updateAltText(altText: string): void {
    if (!altText || altText.trim().length === 0) {
      throw new DomainError('Product image alt text cannot be empty');
    }
    this.altText = altText.trim();
  }

  updateDisplayOrder(order: number): void {
    this.displayOrder = order;
  }
}
