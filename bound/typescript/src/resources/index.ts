import wasm from "../wasm";

export type ResourceKind = "offering" | "balance";

export class ResourceMetadata {
  readonly from: string;
  readonly kind: ResourceKind;
  readonly id: string;
  readonly createdAt: string;
  readonly updatedAt?: string;
  readonly protocol: string; // todo previously `{number}`

  constructor(
    from: string,
    kind: ResourceKind,
    id: string,
    createdAt: string,
    protocol: string, // todo previously `${number}`
    updatedAt?: string
  ) {
    this.from = from;
    this.kind = kind;
    this.id = id;
    this.createdAt = createdAt;
    this.protocol = protocol;
    this.updatedAt = updatedAt;
  }

  static fromWASM(wasmMetadata: wasm.WasmResourceMetadata): ResourceMetadata {
    return new ResourceMetadata(
      wasmMetadata.from,
      wasmMetadata.kind.kind as ResourceKind, // todo casting?
      wasmMetadata.id,
      wasmMetadata.created_at,
      wasmMetadata.protocol,
      wasmMetadata.updated_at
    );
  }

  toWASM(): wasm.WasmResourceMetadata {
    return new wasm.WasmResourceMetadata(
      new wasm.WasmResourceKind(this.kind),
      this.from,
      this.id,
      this.protocol,
      this.createdAt,
      this.updatedAt
    );
  }
}
