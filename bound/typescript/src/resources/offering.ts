import { catchTbdexError } from "../errors";
import wasm from "../wasm";
import { OfferingData, ResourceMetadata } from "../wasm/mappings";

// TODO consider extending "Resource" class type
export class Offering {
  readonly metadata: ResourceMetadata;
  readonly data: OfferingData;
  readonly signature: string;

  constructor(
    metadata: ResourceMetadata,
    data: OfferingData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  private static fromWASM(wasmOffering: wasm.WasmOffering): Offering {
    return new Offering(
      ResourceMetadata.fromWASM(wasmOffering.metadata),
      OfferingData.fromWASM(wasmOffering.data),
      wasmOffering.signature
    );
  }

  private toWASM(): wasm.WasmOffering {
    return new wasm.WasmOffering(
      ResourceMetadata.toWASM(this.metadata),
      OfferingData.toWASM(this.data),
      this.signature
    );
  }

  static fromJSONString(json: string): Offering {
    try {
      return Offering.fromWASM(wasm.WasmOffering.from_json_string(json));
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  toJSONString(): string {
    try {
      return this.toWASM().to_json_string();
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  static create(from: string, data: OfferingData, protocol?: string): Offering {
    try {
      return Offering.fromWASM(
        wasm.WasmOffering.create(from, OfferingData.toWASM(data), protocol)
      );
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  verify() {
    try {
      this.toWASM().verify();
    } catch (error) {
      throw catchTbdexError(error);
    }
  }
}
