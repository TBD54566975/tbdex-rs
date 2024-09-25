import wasm from "./wasm";

export type Signer = {
  sign: (payload: Uint8Array) => Uint8Array;
};

export namespace Signer {
  export const toWASM = (signer: Signer): wasm.WasmSigner => {
    const foreignSigner = {
      sign: (payload: Uint8Array): Uint8Array => {
        return signer.sign(payload);
      },
    };

    return new wasm.WasmSigner(foreignSigner);
  };

  export const fromWASM = (wasmSigner: wasm.WasmSigner): Signer => {
    const signer: Signer = {
      sign: (payload: Uint8Array): Uint8Array => {
        return wasmSigner.sign(payload);
      },
    };

    return signer;
  };
}
