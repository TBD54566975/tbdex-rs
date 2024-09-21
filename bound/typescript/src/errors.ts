export class TbdexError extends Error {
  variant: string;

  constructor(variant: string, message: string) {
    super(message);
    this.variant = variant;
    this.name = 'TbdexError'; 
  }
}

export const catchTbdexError = (error: any): Error => {
  if (error && typeof error === 'object' && error.is_tbdex_error) {
    return new TbdexError(error.variant, error.message);
  } 
  return error
}