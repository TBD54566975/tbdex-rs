import { MessageMetadata } from ".";
import { BearerDid } from "../dids/bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";

export class OrderInstructions {
  readonly metadata: MessageMetadata;
  readonly data: OrderInstructionsData;
  signature: string;

  constructor(
    metadata: MessageMetadata,
    data: OrderInstructionsData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): OrderInstructions => {
    const obj = JSON.parse(json);
    return new OrderInstructions(obj.metadata, obj.data, obj.signature);
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
    data: OrderInstructionsData,
    protocol?: string,
    externalId?: string
  ): OrderInstructions => {
    try {
      let json = wasm.order_instructions_create(
        to,
        from,
        exchangeId,
        JSON.stringify(data),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new OrderInstructions(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.order_instructions_sign(
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
      await wasm.order_instructions_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type OrderInstructionsData = {
  payin: PaymentInstruction;
  payout: PaymentInstruction;
};

export type PaymentInstruction = {
  instruction?: string;
  link?: string;
};
