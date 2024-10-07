import { Jwk } from "./jwk";
import { Signer } from "./signer";
import wasm from "../wasm";

export type KeyManager = {
  importPrivateJwk(privateJwk: Jwk): Jwk;
  getSigner(publicJwk: Jwk): Signer;
};

export namespace KeyManager {
  export const toWASM = (keyManager: KeyManager): wasm.WasmKeyManager => {
    const foreignKeyManager = {
      import_private_jwk: (privateJwkJson: string): string => {
        const privateJwk: Jwk = JSON.parse(privateJwkJson);
        const publicJwk = keyManager.importPrivateJwk(privateJwk);
        const publicJwkJson = JSON.stringify(publicJwk);
        return publicJwkJson;
      },
      get_signer: (publicJwkJson: string): wasm.WasmSigner => {
        const publicJwk: Jwk = JSON.parse(publicJwkJson);
        return Signer.toWASM(keyManager.getSigner(publicJwk));
      },
    };

    return new wasm.WasmKeyManager(foreignKeyManager);
  };
}
