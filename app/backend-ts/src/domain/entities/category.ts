import { CategoryId } from '../value-objects/identifiers';
import { DomainError } from '../errors/domain.error';

export class Category {
  private constructor(
    private readonly id: CategoryId,
    private name: string,
    private slug: string,
    private readonly parentId: CategoryId | null,
    private displayOrder: number,
  ) {}

  static create(
    id: CategoryId,
    name: string,
    slug: string,
    parentId?: CategoryId,
    displayOrder: number = 0,
  ): Category {
    if (!name || name.trim().length === 0) {
      throw new DomainError('Category name cannot be empty');
    }
    if (!slug || slug.trim().length === 0) {
      throw new DomainError('Category slug cannot be empty');
    }
    if (!Category.isValidSlug(slug)) {
      throw new DomainError(
        'Category slug must contain only alphanumeric characters and hyphens',
      );
    }

    return new Category(
      id,
      name.trim(),
      slug.trim().toLowerCase(),
      parentId || null,
      displayOrder,
    );
  }

  private static isValidSlug(slug: string): boolean {
    const slugRegex = /^[a-zA-Z0-9-]+$/;
    return slugRegex.test(slug);
  }

  isRoot(): boolean {
    return this.parentId === null;
  }

  isSubcategory(): boolean {
    return this.parentId !== null;
  }

  // Getters
  getId(): CategoryId {
    return this.id;
  }

  getName(): string {
    return this.name;
  }

  getSlug(): string {
    return this.slug;
  }

  getParentId(): CategoryId | null {
    return this.parentId;
  }

  getDisplayOrder(): number {
    return this.displayOrder;
  }

  // Update methods
  updateName(name: string): void {
    if (!name || name.trim().length === 0) {
      throw new DomainError('Category name cannot be empty');
    }
    this.name = name.trim();
  }

  updateSlug(slug: string): void {
    if (!slug || slug.trim().length === 0) {
      throw new DomainError('Category slug cannot be empty');
    }
    if (!Category.isValidSlug(slug)) {
      throw new DomainError(
        'Category slug must contain only alphanumeric characters and hyphens',
      );
    }
    this.slug = slug.trim().toLowerCase();
  }

  updateDisplayOrder(order: number): void {
    this.displayOrder = order;
  }
}
