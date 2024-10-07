import { MessageMetadata } from ".";
import { BearerDid } from "../dids/bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class Cancel {
  readonly metadata: MessageMetadata;
  readonly data: CancelData;
  signature: string;

  constructor(metadata: MessageMetadata, data: CancelData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Cancel => {
    const obj = JSON.parse(json);
    return new Cancel(obj.metadata, obj.data, obj.signature);
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
    data: CancelData,
    protocol?: string,
    externalId?: string
  ): Cancel => {
    try {
      let json = wasm.cancel_create(
        to,
        from,
        exchangeId,
        JSON.stringify(data),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new Cancel(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.cancel_sign(
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
      await wasm.cancel_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type CancelData = {
  reason?: string;
};
