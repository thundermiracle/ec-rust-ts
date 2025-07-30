import { DomainError } from '$domain/errors/domain.error';
import { ColorId } from '$domain/value-objects';

export class ColorName {
  private constructor(private readonly value: string) {}

  static create(name: string): ColorName {
    if (!name || name.trim().length === 0) {
      throw new DomainError('Color name cannot be empty');
    }
    if (name.trim().length > 50) {
      throw new DomainError('Color name cannot exceed 50 characters');
    }
    return new ColorName(name.trim());
  }

  getValue(): string {
    return this.value;
  }

  toString(): string {
    return this.value;
  }

  equals(other: ColorName): boolean {
    return this.value === other.value;
  }
}

export class Color {
  private constructor(
    private readonly id: ColorId,
    private name: ColorName,
    private hex: string,
  ) {}

  static create(id: ColorId, name: string, hex: string): Color {
    Color.validateHexCode(hex);
    const colorName = ColorName.create(name);
    return new Color(id, colorName, hex);
  }

  static validateHexCode(hex: string): void {
    if (!hex.startsWith('#')) {
      throw new DomainError('Hex code must start with #');
    }
    if (hex.length !== 7) {
      throw new DomainError('Hex code must be exactly 7 characters long');
    }

    const hexDigits = hex.slice(1);
    const hexRegex = /^[0-9A-Fa-f]{6}$/;
    if (!hexRegex.test(hexDigits)) {
      throw new DomainError(
        'Hex code must contain only valid hexadecimal digits',
      );
    }
  }

  // Getters
  getId(): ColorId {
    return this.id;
  }

  getName(): ColorName {
    return this.name;
  }

  getHex(): string {
    return this.hex;
  }

  // Update methods
  updateName(name: string): void {
    this.name = ColorName.create(name);
  }

  updateHex(hex: string): void {
    Color.validateHexCode(hex);
    this.hex = hex;
  }

  toString(): string {
    return `${this.name.toString()} (${this.hex})`;
  }

  equals(other: Color): boolean {
    return this.id.equals(other.id);
  }
}
