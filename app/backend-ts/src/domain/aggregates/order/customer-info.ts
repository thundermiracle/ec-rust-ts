import { DomainError } from '$domain/errors/domain.error';
import { Email, PhoneNumber } from '$domain/value-objects';

export class PersonalInfo {
  private constructor(
    private readonly firstName: string,
    private readonly lastName: string,
  ) {}

  static create(firstName: string, lastName: string): PersonalInfo {
    if (!firstName || firstName.trim().length === 0) {
      throw new DomainError('First name cannot be empty');
    }
    if (!lastName || lastName.trim().length === 0) {
      throw new DomainError('Last name cannot be empty');
    }
    if (firstName.trim().length > 50) {
      throw new DomainError('First name cannot exceed 50 characters');
    }
    if (lastName.trim().length > 50) {
      throw new DomainError('Last name cannot exceed 50 characters');
    }

    return new PersonalInfo(firstName.trim(), lastName.trim());
  }

  getFirstName(): string {
    return this.firstName;
  }

  getLastName(): string {
    return this.lastName;
  }

  getFullName(): string {
    return `${this.firstName} ${this.lastName}`;
  }
}

export class CustomerInfo {
  private constructor(
    private readonly personalInfo: PersonalInfo,
    private readonly email: Email,
    private readonly phoneNumber: PhoneNumber,
  ) {}

  static create(
    firstName: string,
    lastName: string,
    email: string,
    phoneNumber: string,
  ): CustomerInfo {
    const personalInfo = PersonalInfo.create(firstName, lastName);
    const customerEmail = Email.new(email);
    const customerPhone = PhoneNumber.new(phoneNumber);

    return new CustomerInfo(personalInfo, customerEmail, customerPhone);
  }

  // Getters
  getPersonalInfo(): PersonalInfo {
    return this.personalInfo;
  }

  getEmail(): Email {
    return this.email;
  }

  getPhoneNumber(): PhoneNumber {
    return this.phoneNumber;
  }

  getFullName(): string {
    return this.personalInfo.getFullName();
  }

  getFirstName(): string {
    return this.personalInfo.getFirstName();
  }

  getLastName(): string {
    return this.personalInfo.getLastName();
  }
}
