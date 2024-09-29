import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, OrderData } from "../wasm/generated-mappings";

export class Order {
  readonly metadata: MessageMetadata;
  readonly data: OrderData;
  signature: string;

  constructor(metadata: MessageMetadata, data: OrderData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromWASM = (wasmOrder: wasm.WasmOrder): Order => {
    try {
      return new Order(
        MessageMetadata.fromWASM(wasmOrder.metadata),
        OrderData.fromWASM(wasmOrder.data),
        wasmOrder.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmOrder => {
    try {
      return new wasm.WasmOrder(
        MessageMetadata.toWASM(this.metadata),
        OrderData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Order => {
    try {
      return Order.fromWASM(wasm.WasmOrder.from_json_string(json));
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toJSONString = (): string => {
    try {
      return this.toWASM().to_json_string();
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static create = (
    to: string,
    from: string,
    exchangeId: string,
    protocol?: string,
    externalId?: string
  ): Order => {
    try {
      return Order.fromWASM(
        wasm.WasmOrder.create(to, from, exchangeId, protocol, externalId)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const wasmOrder = this.toWASM();
      wasmOrder.sign(bearerDid.toWASM());
      this.signature = wasmOrder.signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = () => {
    try {
      this.toWASM().verify();
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
