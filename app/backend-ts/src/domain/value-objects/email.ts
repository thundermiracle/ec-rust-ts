import { EmailError } from '../errors/domain.error';

export class Email {
  private readonly emailValue: string;

  private constructor(value: string) {
    this.emailValue = value;
  }

  static new(value: string): Email {
    const trimmedValue = value.trim();

    if (trimmedValue.length === 0) {
      throw new EmailError('Email cannot be empty');
    }

    if (trimmedValue.length > 255) {
      throw new EmailError('Email cannot exceed 255 characters');
    }

    if (!Email.validate(trimmedValue)) {
      throw new EmailError('Invalid email format');
    }

    return new Email(trimmedValue);
  }

  static validate(value: string): boolean {
    const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    return emailRegex.test(value);
  }

  value(): string {
    return this.emailValue;
  }

  toString(): string {
    return this.emailValue;
  }

  equals(other: Email): boolean {
    return this.emailValue === other.value();
  }
}
