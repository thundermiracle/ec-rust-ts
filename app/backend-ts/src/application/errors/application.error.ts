export class ApplicationError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
  ) {
    super(message);
    this.name = 'ApplicationError';
  }
}

export class ValidationError extends ApplicationError {
  constructor(
    message: string,
    public readonly field?: string,
  ) {
    super(message, 'VALIDATION_ERROR');
    this.name = 'ValidationError';
  }
}

export class NotFoundError extends ApplicationError {
  constructor(resource: string, identifier?: string) {
    const message = identifier
      ? `${resource} with ID ${identifier} not found`
      : `${resource} not found`;
    super(message, 'NOT_FOUND');
    this.name = 'NotFoundError';
  }
}

export class BusinessRuleViolationError extends ApplicationError {
  constructor(message: string) {
    super(message, 'BUSINESS_RULE_VIOLATION');
    this.name = 'BusinessRuleViolationError';
  }
}

export class InsufficientStockError extends BusinessRuleViolationError {
  constructor(skuId: string, requested: number, available: number) {
    super(
      `Insufficient stock for SKU ${skuId}: requested ${requested}, available ${available}`,
    );
    this.name = 'InsufficientStockError';
  }
}
