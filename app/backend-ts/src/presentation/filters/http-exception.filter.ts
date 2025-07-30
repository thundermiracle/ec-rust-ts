import {
  ArgumentsHost,
  Catch,
  ExceptionFilter,
  HttpException,
  HttpStatus,
} from '@nestjs/common';
import { Response } from 'express';

import {
  ApplicationError,
  BusinessRuleViolationError,
  InsufficientStockError,
  NotFoundError,
  ValidationError,
} from '$application/errors/application.error';
import { DomainError } from '$domain/errors/domain.error';

interface ErrorResponse {
  statusCode: number;
  message: string;
  error: string;
  timestamp: string;
  path: string;
}

@Catch()
export class HttpExceptionFilter implements ExceptionFilter {
  catch(exception: unknown, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const response = ctx.getResponse<Response>();
    const request = ctx.getRequest<{ url: string }>();

    let status: number;
    let message: string;
    let error: string;

    if (exception instanceof HttpException) {
      status = exception.getStatus();
      const exceptionResponse = exception.getResponse() as
        | string
        | { message?: string };
      message =
        typeof exceptionResponse === 'string'
          ? exceptionResponse
          : (typeof exceptionResponse === 'object' &&
              exceptionResponse.message) ||
            exception.message;
      error = exception.name;
    } else if (exception instanceof ValidationError) {
      status = HttpStatus.BAD_REQUEST;
      message = exception.message;
      error = 'Validation Error';
    } else if (exception instanceof NotFoundError) {
      status = HttpStatus.NOT_FOUND;
      message = exception.message;
      error = 'Not Found';
    } else if (exception instanceof InsufficientStockError) {
      status = HttpStatus.UNPROCESSABLE_ENTITY;
      message = exception.message;
      error = 'Insufficient Stock';
    } else if (exception instanceof BusinessRuleViolationError) {
      status = HttpStatus.UNPROCESSABLE_ENTITY;
      message = exception.message;
      error = 'Business Rule Violation';
    } else if (exception instanceof ApplicationError) {
      status = HttpStatus.BAD_REQUEST;
      message = exception.message;
      error = 'Application Error';
    } else if (exception instanceof DomainError) {
      status = HttpStatus.BAD_REQUEST;
      message = exception.message;
      error = 'Domain Error';
    } else {
      status = HttpStatus.INTERNAL_SERVER_ERROR;
      message = 'Internal server error';
      error = 'Internal Server Error';

      // Log unexpected errors
      console.error('Unexpected error:', exception);
    }

    const errorResponse: ErrorResponse = {
      statusCode: status,
      message,
      error,
      timestamp: new Date().toISOString(),
      path: request.url,
    };

    response.status(status).json(errorResponse);
  }
}
