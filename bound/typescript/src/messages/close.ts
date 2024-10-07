import { MessageMetadata } from ".";
import { BearerDid } from "../dids/bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class Close {
  readonly metadata: MessageMetadata;
  readonly data: CloseData;
  signature: string;

  constructor(metadata: MessageMetadata, data: CloseData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Close => {
    const obj = JSON.parse(json);
    return new Close(obj.metadata, obj.data, obj.signature);
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
    data: CloseData,
    protocol?: string,
    externalId?: string
  ): Close => {
    try {
      let json = wasm.close_create(
        to,
        from,
        exchangeId,
        JSON.stringify(data),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new Close(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.close_sign(
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
      await wasm.close_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type CloseData = {
  reason?: string;
  success?: boolean;
};
