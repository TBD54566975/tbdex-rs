import wasm from "./wasm";
import { Jwk } from "./wasm/mappings";

export type KeyManager = {
  importPrivateJwk(privateJwk: Jwk): Jwk;
};

export namespace KeyManager {
  export const toWASM = (keyManager: KeyManager): wasm.WasmKeyManager => {
    const foreignKeyManager = {
      importPrivateJwk: (privateJwk: wasm.WasmJwk): wasm.WasmJwk => {
        const publicJwk = keyManager.importPrivateJwk(Jwk.fromWASM(privateJwk));
        return Jwk.toWASM(publicJwk);
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
    };

    return keyManager;
  };
}
