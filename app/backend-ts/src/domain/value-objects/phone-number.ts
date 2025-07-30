import { PhoneNumberError } from '$domain/errors/domain.error';

export class PhoneNumber {
  private readonly phoneValue: string;

  private constructor(value: string) {
    this.phoneValue = value;
  }

  static new(value: string): PhoneNumber {
    const trimmedValue = value.trim();

    if (trimmedValue.length === 0) {
      throw new PhoneNumberError('電話番号は必須です');
    }

    if (trimmedValue.length > 20) {
      throw new PhoneNumberError('電話番号は20文字以内で入力してください');
    }

    if (!PhoneNumber.validate(trimmedValue)) {
      throw new PhoneNumberError('有効な日本の電話番号形式で入力してください');
    }

    return new PhoneNumber(trimmedValue);
  }

  static validate(value: string): boolean {
    // Japanese phone number format: 0X-XXXX-XXXX or 0XXXXXXXXX (landline), 0X0-XXXX-XXXX or 0X0XXXXXXXX (mobile)
    const phoneRegex = /^(0\d{1,4}-\d{1,4}-\d{1,4}|0\d{9,10})$/;
    return phoneRegex.test(value);
  }

  value(): string {
    return this.phoneValue;
  }

  formatted(): string {
    // Auto-format phone numbers with hyphens
    const digitsOnly = this.phoneValue.replace(/-/g, '');

    if (digitsOnly.length === 10) {
      // XX-XXXX-XXXX
      return `${digitsOnly.slice(0, 2)}-${digitsOnly.slice(2, 6)}-${digitsOnly.slice(6)}`;
    } else if (digitsOnly.length === 11) {
      // XXX-XXXX-XXXX
      return `${digitsOnly.slice(0, 3)}-${digitsOnly.slice(3, 7)}-${digitsOnly.slice(7)}`;
    }

    // Return as-is if already formatted or unusual length
    return this.phoneValue;
  }

  toString(): string {
    return this.formatted();
  }

  equals(other: PhoneNumber): boolean {
    return this.phoneValue === other.value();
  }
}
