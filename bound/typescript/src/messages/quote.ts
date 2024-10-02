import { MessageMetadata } from ".";
import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class Quote {
  readonly metadata: MessageMetadata;
  readonly data: QuoteData;
  signature: string;

  constructor(metadata: MessageMetadata, data: QuoteData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Quote => {
    const obj = JSON.parse(json);
    return new Quote(obj.metadata, obj.data, obj.signature);
  };

  toJSONString = (): string => {
    return JSON.stringify({
      metadata: this.metadata,
      data: this.data,
      signature: this.signature,
    });
  };

  static create = (
    to: string,
    from: string,
    exchangeId: string,
    data: QuoteData,
    protocol?: string,
    externalId?: string
  ): Quote => {
    try {
      let json = wasm.quote_create(
        to,
        from,
        exchangeId,
        JSON.stringify(data),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new Quote(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.quote_sign(
        this.toJSONString(),
        bearerDid.toWASM()
      );
      this.signature = signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = async () => {
    try {
      await wasm.quote_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type QuoteData = {
  expiresAt: string;
  payin: QuoteDetails;
  payout: QuoteDetails;
  payoutUnitsPerPayinUnit: string;
};

export type QuoteDetails = {
  currencyCode: string;
  fee?: string;
  subtotal: string;
  total: string;
};
