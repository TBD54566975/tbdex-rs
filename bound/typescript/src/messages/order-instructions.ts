import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, OrderInstructionsData } from "../wasm/generated-mappings";

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

  static fromWASM = (
    wasmOrderInstructions: wasm.WasmOrderInstructions
  ): OrderInstructions => {
    try {
      return new OrderInstructions(
        MessageMetadata.fromWASM(wasmOrderInstructions.metadata),
        OrderInstructionsData.fromWASM(wasmOrderInstructions.data),
        wasmOrderInstructions.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmOrderInstructions => {
    try {
      return new wasm.WasmOrderInstructions(
        MessageMetadata.toWASM(this.metadata),
        OrderInstructionsData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): OrderInstructions => {
    try {
      return OrderInstructions.fromWASM(
        wasm.WasmOrderInstructions.from_json_string(json)
      );
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
    data: OrderInstructionsData,
    protocol?: string,
    externalId?: string
  ): OrderInstructions => {
    try {
      return OrderInstructions.fromWASM(
        wasm.WasmOrderInstructions.create(
          to,
          from,
          exchangeId,
          OrderInstructionsData.toWASM(data),
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
      const wasmOrderInstructions = this.toWASM();
      wasmOrderInstructions.sign(bearerDid.toWASM());
      this.signature = wasmOrderInstructions.signature;
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
