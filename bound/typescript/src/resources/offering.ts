import { withError } from "../errors";
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

  private static fromWASM = withError(
    (wasmOffering: wasm.WasmOffering): Offering => {
      return new Offering(
        ResourceMetadata.fromWASM(wasmOffering.metadata),
        OfferingData.fromWASM(wasmOffering.data),
        wasmOffering.signature
      );
    }
  );

  private toWASM = withError((): wasm.WasmOffering => {
    return new wasm.WasmOffering(
      ResourceMetadata.toWASM(this.metadata),
      OfferingData.toWASM(this.data),
      this.signature
    );
  });

  static fromJSONString = withError((json: string): Offering => {
    return Offering.fromWASM(wasm.WasmOffering.from_json_string(json));
  });

  toJSONString = withError((): string => {
    return this.toWASM().to_json_string();
  });

  static create = withError(
    (from: string, data: OfferingData, protocol?: string): Offering => {
      return Offering.fromWASM(
        wasm.WasmOffering.create(from, OfferingData.toWASM(data), protocol)
      );
    }
  );

  verify = withError(() => {
    this.toWASM().verify();
  });
}
