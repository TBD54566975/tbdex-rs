import { Signer } from "./signers";
import wasm from "./wasm";
import { Jwk } from "./wasm/generated-mappings";

export type KeyManager = {
  importPrivateJwk(privateJwk: Jwk): Jwk;
  getSigner(publicJwk: Jwk): Signer;
};

export namespace KeyManager {
  export const toWASM = (keyManager: KeyManager): wasm.WasmKeyManager => {
    const foreignKeyManager = {
      import_private_jwk: (privateJwk: wasm.WasmJwk): wasm.WasmJwk => {
        const publicJwk = keyManager.importPrivateJwk(Jwk.fromWASM(privateJwk));
        return Jwk.toWASM(publicJwk);
      },
      get_signer: (publicJwk: wasm.WasmJwk): wasm.WasmSigner => {
        return Signer.toWASM(keyManager.getSigner(Jwk.fromWASM(publicJwk)));
      },
    };

    return new wasm.WasmKeyManager(foreignKeyManager);
  };

  export const fromWASM = (wasmKeyManager: wasm.WasmKeyManager): KeyManager => {
    const keyManager: KeyManager = {
      importPrivateJwk: (privateJwk: Jwk): Jwk => {
        const wasmPublicJwk = wasmKeyManager.import_private_jwk(
          Jwk.toWASM(privateJwk)
        );
        return Jwk.fromWASM(wasmPublicJwk);
      },
      getSigner: (publicJwk: Jwk): Signer => {
        const wasmSigner = wasmKeyManager.get_signer(Jwk.toWASM(publicJwk));
        return Signer.fromWASM(wasmSigner);
      },
    };

    return keyManager;
  };
}
