import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { ResourceMetadata } from "../wasm/generated-mappings";

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
    return JSON.stringify(this);
  };

  static create = (
    from: string,
    data: BalanceData,
    protocol?: string
  ): Balance => {
    try {
      const json = wasm.balance_create(from, JSON.stringify(data), protocol);
      const balance = JSON.parse(json);
      return new Balance(balance.metadata, balance.data, balance.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.balance_sign(
        JSON.stringify(this),
        bearerDid.toWASM()
      );
      this.signature = signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = async () => {
    try {
      await wasm.balance_verify(JSON.stringify(this));
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type BalanceData = {
  available: string;
  currencyCode: string;
};
