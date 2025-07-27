export class DomainError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
  ) {
    super(message);
    this.name = 'DomainError';
  }
}

export class EmailError extends DomainError {
  constructor(message: string) {
    super(message, 'EMAIL_ERROR');
    this.name = 'EmailError';
  }
}

export class PhoneNumberError extends DomainError {
  constructor(message: string) {
    super(message, 'PHONE_NUMBER_ERROR');
    this.name = 'PhoneNumberError';
  }
}

export class AddressError extends DomainError {
  constructor(message: string) {
    super(message, 'ADDRESS_ERROR');
    this.name = 'AddressError';
  }
}

export class IdentifierError extends DomainError {
  constructor(message: string) {
    super(message, 'IDENTIFIER_ERROR');
    this.name = 'IdentifierError';
  }
}
