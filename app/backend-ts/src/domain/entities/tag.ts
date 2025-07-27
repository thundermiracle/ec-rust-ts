import { DomainError } from '../errors/domain.error';

export class Tag {
  private constructor(
    private readonly id: string,
    private name: string,
    private slug: string,
  ) {}

  static create(id: string, name: string, slug?: string): Tag {
    if (!id || id.trim().length === 0) {
      throw new DomainError('Tag ID cannot be empty');
    }
    if (!name || name.trim().length === 0) {
      throw new DomainError('Tag name cannot be empty');
    }
    if (name.trim().length > 50) {
      throw new DomainError('Tag name cannot exceed 50 characters');
    }

    const finalSlug = slug || Tag.generateSlug(name);
    if (!Tag.isValidSlug(finalSlug)) {
      throw new DomainError(
        'Tag slug must contain only alphanumeric characters and hyphens',
      );
    }

    return new Tag(id.trim(), name.trim(), finalSlug);
  }

  private static generateSlug(name: string): string {
    return name
      .trim()
      .toLowerCase()
      .replace(/[^a-zA-Z0-9\s-]/g, '')
      .replace(/\s+/g, '-')
      .replace(/-+/g, '-')
      .replace(/^-|-$/g, '');
  }

  private static isValidSlug(slug: string): boolean {
    const slugRegex = /^[a-zA-Z0-9-]+$/;
    return slugRegex.test(slug) && !slug.startsWith('-') && !slug.endsWith('-');
  }

  // Getters
  getId(): string {
    return this.id;
  }

  getName(): string {
    return this.name;
  }

  getSlug(): string {
    return this.slug;
  }

  // Update methods
  updateName(name: string): void {
    if (!name || name.trim().length === 0) {
      throw new DomainError('Tag name cannot be empty');
    }
    if (name.trim().length > 50) {
      throw new DomainError('Tag name cannot exceed 50 characters');
    }

    this.name = name.trim();
    this.slug = Tag.generateSlug(name);
  }

  updateSlug(slug: string): void {
    if (!Tag.isValidSlug(slug)) {
      throw new DomainError(
        'Tag slug must contain only alphanumeric characters and hyphens',
      );
    }
    this.slug = slug;
  }

  toString(): string {
    return this.name;
  }

  equals(other: Tag): boolean {
    return this.id === other.id;
  }
}
