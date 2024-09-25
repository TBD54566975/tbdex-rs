export class TbdexError extends Error {
  variant: string;

  constructor(variant: string, message: string) {
    super(message);
    this.variant = variant;
    this.name = "TbdexError";
  }
}

export const withError = <T>(fn: (...args: any[]) => T) => {
  return (...args: any[]): T => {
    try {
      return fn(...args);
    } catch (error: any) {
      if (error && typeof error === "object" && error.is_tbdex_error) {
        throw new TbdexError(error.variant, error.message);
      }
      throw error;
    }
  };
};
