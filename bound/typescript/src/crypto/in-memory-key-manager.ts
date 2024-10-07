import wasm from "../wasm";
import { Jwk } from "./jwk";
import { KeyManager } from "./key-manager";
import { Signer } from "./signer";

export class InMemoryKeyManager implements KeyManager {
  private readonly wasmKeyManager: wasm.WasmKeyManager;

  constructor() {
    this.wasmKeyManager = wasm.new_in_memory_key_manager();
  }

  importPrivateJwk(privateJwk: Jwk): Jwk {
    const privateJwkJson = JSON.stringify(privateJwk);
    const publicJwkJson =
      this.wasmKeyManager.import_private_jwk(privateJwkJson);
    const publicJwk: Jwk = JSON.parse(publicJwkJson);
    return publicJwk;
  }

  getSigner(publicJwk: Jwk): Signer {
    const publicJwkJson = JSON.stringify(publicJwk);
    const wasmSigner = this.wasmKeyManager.get_signer(publicJwkJson);
    return Signer.fromWASM(wasmSigner);
  }
}
