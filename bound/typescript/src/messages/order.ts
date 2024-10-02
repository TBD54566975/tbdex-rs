import { MessageMetadata } from ".";
import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class Order {
  readonly metadata: MessageMetadata;
  readonly data: OrderData;
  signature: string;

  constructor(metadata: MessageMetadata, data: OrderData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Order => {
    const obj = JSON.parse(json);
    return new Order(obj.metadata, obj.data, obj.signature);
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
    protocol?: string,
    externalId?: string
  ): Order => {
    try {
      let json = wasm.order_create(to, from, exchangeId, protocol, externalId);
      let obj = JSON.parse(json);
      return new Order(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.order_sign(
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
      await wasm.order_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type OrderData = {};
