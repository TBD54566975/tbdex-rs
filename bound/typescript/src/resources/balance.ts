import { ResourceMetadata } from ".";
import { BearerDid } from "../dids/bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class Balance {
  readonly metadata: ResourceMetadata;
  readonly data: BalanceData;
  signature: string;

  constructor(
    metadata: ResourceMetadata,
    data: BalanceData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Balance => {
    const object = JSON.parse(json);
    return new Balance(object.metadata, object.data, object.signature);
  };

  toJSONString = (): string => {
    return JSON.stringify({
      metadata: this.metadata,
      data: this.data,
      signature: this.signature,
    });
  };

  static create = (
    from: string,
    data: BalanceData,
    protocol?: string
  ): Balance => {
    try {
      const json = wasm.balance_create(from, JSON.stringify(data), protocol);
      const obj = JSON.parse(json);
      return new Balance(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.balance_sign(
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
      await wasm.balance_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type BalanceData = {
  available: string;
  currencyCode: string;
};
