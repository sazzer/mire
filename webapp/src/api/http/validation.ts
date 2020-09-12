/**
 * The error with a particular field
 */
export class FieldError {
  /** The field that the error occurred with */
  readonly field: string;
  /** The actual error type that occurred */
  readonly type: string;

  /**
   * Constructor
   * @param field The name of the field
   * @param type The error type
   */
  constructor(field: string, type: string) {
    this.field = field;
    this.type = type;
  }
}

/**
 * A validation error occurred on a request
 */
export class ValidationError {
  /** The fields that were in error */
  readonly fields: FieldError[];

  /**
   * Constructor
   * @param fields The field that were in error
   */
  constructor(fields: FieldError[]) {
    this.fields = fields;
  }

  /**
   * Get all of the errors with a particular field
   * @param field The name of the field
   */
  getFieldErrors(field: string) {
    return this.fields.filter((f) => f.field === field);
  }
}
