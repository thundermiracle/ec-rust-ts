import { AddressError } from '../errors/domain.error';

export class Address {
  private constructor(
    private readonly postalCode: string,
    private readonly prefecture: string,
    private readonly city: string,
    private readonly street: string,
    private readonly building?: string,
  ) {}

  static new(
    postalCode: string,
    prefecture: string,
    city: string,
    street: string,
    building?: string,
  ): Address {
    // Validate postal code
    const trimmedPostalCode = postalCode.trim();
    if (!Address.validatePostalCode(trimmedPostalCode)) {
      throw new AddressError('郵便番号は123-4567の形式で入力してください');
    }

    // Validate required fields
    const trimmedPrefecture = prefecture.trim();
    if (trimmedPrefecture.length === 0) {
      throw new AddressError('都道府県は必須です');
    }
    if (trimmedPrefecture.length > 100) {
      throw new AddressError('都道府県は100文字以内で入力してください');
    }

    const trimmedCity = city.trim();
    if (trimmedCity.length === 0) {
      throw new AddressError('市区町村は必須です');
    }
    if (trimmedCity.length > 100) {
      throw new AddressError('市区町村は100文字以内で入力してください');
    }

    const trimmedStreet = street.trim();
    if (trimmedStreet.length === 0) {
      throw new AddressError('番地は必須です');
    }
    if (trimmedStreet.length > 100) {
      throw new AddressError('番地は100文字以内で入力してください');
    }

    // Validate building (optional)
    let trimmedBuilding: string | undefined;
    if (building) {
      trimmedBuilding = building.trim();
      if (trimmedBuilding.length > 100) {
        throw new AddressError('建物名は100文字以内で入力してください');
      }
      if (trimmedBuilding.length === 0) {
        trimmedBuilding = undefined;
      }
    }

    return new Address(
      trimmedPostalCode,
      trimmedPrefecture,
      trimmedCity,
      trimmedStreet,
      trimmedBuilding,
    );
  }

  static validatePostalCode(postalCode: string): boolean {
    const postalCodeRegex = /^\d{3}-\d{4}$/;
    return postalCodeRegex.test(postalCode);
  }

  getPostalCode(): string {
    return this.postalCode;
  }

  getPrefecture(): string {
    return this.prefecture;
  }

  getCity(): string {
    return this.city;
  }

  getStreet(): string {
    return this.street;
  }

  getBuilding(): string | undefined {
    return this.building;
  }

  fullAddress(): string {
    const buildingPart = this.building ? ` ${this.building}` : '';
    return `〒${this.postalCode} ${this.prefecture} ${this.city} ${this.street}${buildingPart}`;
  }

  formatted(): string {
    return this.fullAddress();
  }

  toString(): string {
    return this.fullAddress();
  }

  equals(other: Address): boolean {
    return (
      this.postalCode === other.postalCode &&
      this.prefecture === other.prefecture &&
      this.city === other.city &&
      this.street === other.street &&
      this.building === other.building
    );
  }
}
