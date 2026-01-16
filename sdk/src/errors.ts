export class ProvaError extends Error {
  statusCode?: number;

  constructor(message: string, statusCode?: number) {
    super(message);
    this.name = 'ProvaError';
    this.statusCode = statusCode;
  }
}

export class AuthenticationError extends ProvaError {
  constructor(message = 'Invalid API key') {
    super(message, 401);
    this.name = 'AuthenticationError';
  }
}

export class NotFoundError extends ProvaError {
  constructor(message = 'Resource not found') {
    super(message, 404);
    this.name = 'NotFoundError';
  }
}

export class ValidationError extends ProvaError {
  constructor(message: string) {
    super(message, 400);
    this.name = 'ValidationError';
  }
}

export class RateLimitError extends ProvaError {
  constructor(message = 'Rate limit exceeded') {
    super(message, 429);
    this.name = 'RateLimitError';
  }
}

export class TimeoutError extends ProvaError {
  constructor(message = 'Request timed out') {
    super(message, 408);
    this.name = 'TimeoutError';
  }
}
