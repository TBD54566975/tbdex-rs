import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, OrderStatusData } from "../wasm/generated-mappings";

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

  static fromWASM = (wasmOrderStatus: wasm.WasmOrderStatus): OrderStatus => {
    try {
      return new OrderStatus(
        MessageMetadata.fromWASM(wasmOrderStatus.metadata),
        OrderStatusData.fromWASM(wasmOrderStatus.data),
        wasmOrderStatus.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmOrderStatus => {
    try {
      return new wasm.WasmOrderStatus(
        MessageMetadata.toWASM(this.metadata),
        OrderStatusData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): OrderStatus => {
    try {
      return OrderStatus.fromWASM(wasm.WasmOrderStatus.from_json_string(json));
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
    data: OrderStatusData,
    protocol?: string,
    externalId?: string
  ): OrderStatus => {
    try {
      return OrderStatus.fromWASM(
        wasm.WasmOrderStatus.create(
          to,
          from,
          exchangeId,
          OrderStatusData.toWASM(data),
          protocol,
          externalId
        )
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const wasmOrderStatus = this.toWASM();
      wasmOrderStatus.sign(bearerDid.toWASM());
      this.signature = wasmOrderStatus.signature;
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
