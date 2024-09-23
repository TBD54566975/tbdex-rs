import wasm from "../wasm";

export type ResourceKind = "offering" | "balance";

export type ResourceMetadata = {
  from: string;
  kind: ResourceKind;
  id: string;
  createdAt: string;
  protocol: string; // previously `${number}`
  updatedAt?: string;
};

export namespace ResourceMetadata {
  export const fromWASM = (
    wasmMetadata: wasm.WasmResourceMetadata
  ): ResourceMetadata => {
    const metadata: ResourceMetadata = {
      from: wasmMetadata.from,
      kind: wasmMetadata.kind.kind as ResourceKind, // casting to ResourceKind
      id: wasmMetadata.id,
      createdAt: wasmMetadata.created_at,
      protocol: wasmMetadata.protocol,
    };

    if (wasmMetadata.updated_at !== undefined)
      metadata.updatedAt = wasmMetadata.updated_at;

    return metadata;
  };

  export const toWASM = (
    metadata: ResourceMetadata
  ): wasm.WasmResourceMetadata => {
    return new wasm.WasmResourceMetadata(
      new wasm.WasmResourceKind(metadata.kind),
      metadata.from,
      metadata.id,
      metadata.protocol,
      metadata.createdAt,
      metadata.updatedAt
    );
  };
}
