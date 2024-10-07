import { MessageMetadata } from ".";
import { BearerDid } from "../dids/bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class OrderStatus {
  readonly metadata: MessageMetadata;
  readonly data: OrderStatusData;
  signature: string;

  constructor(
    metadata: MessageMetadata,
    data: OrderStatusData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): OrderStatus => {
    const obj = JSON.parse(json);
    return new OrderStatus(obj.metadata, obj.data, obj.signature);
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
    data: OrderStatusData,
    protocol?: string,
    externalId?: string
  ): OrderStatus => {
    try {
      let json = wasm.order_status_create(
        to,
        from,
        exchangeId,
        JSON.stringify(data),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new OrderStatus(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.order_status_sign(
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
      await wasm.order_status_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type OrderStatusData = {
  details?: string;
  status: string;
};
