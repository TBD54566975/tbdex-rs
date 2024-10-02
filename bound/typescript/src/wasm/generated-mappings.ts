import wasm from "./";

export type Did = {
  fragment?: string;
  id: string;
  method: string;
  params?: any;
  path?: string;
  query?: string;
  uri: string;
  url: string;
};

export namespace Did {
  export const toWASM = (obj: Did): wasm.WasmDid => {
    return new wasm.WasmDid(
      obj.uri,
      obj.url,
      obj.method,
      obj.id,
      obj.params,
      obj.path,
      obj.query,
      obj.fragment,
    );
  };

  export const fromWASM = (obj: wasm.WasmDid): Did => {
    const result: Did = {
      id: obj.id,
      method: obj.method,
      uri: obj.uri,
      url: obj.url,
    };

    if (obj.fragment !== undefined) result.fragment = obj.fragment;
    if (obj.params !== undefined) result.params = obj.params;
    if (obj.path !== undefined) result.path = obj.path;
    if (obj.query !== undefined) result.query = obj.query;

    return result;
  };
}

export type Document = {
  alsoKnownAs?: string[];
  assertionMethod?: string[];
  authentication?: string[];
  capabilityDelegation?: string[];
  capabilityInvocation?: string[];
  context?: string[];
  controller?: string[];
  id: string;
  keyAgreement?: string[];
  service?: Service[];
  verificationMethod: VerificationMethod[];
};

export namespace Document {
  export const toWASM = (obj: Document): wasm.WasmDocument => {
    return new wasm.WasmDocument(
      obj.id,
      obj.context,
      obj.controller,
      obj.alsoKnownAs,
      obj.verificationMethod?.map(VerificationMethod.toWASM),
      obj.authentication,
      obj.assertionMethod,
      obj.keyAgreement,
      obj.capabilityInvocation,
      obj.capabilityDelegation,
      obj.service?.map(Service.toWASM),
    );
  };

  export const fromWASM = (obj: wasm.WasmDocument): Document => {
    const result: Document = {
      id: obj.id,
      verificationMethod: obj.verification_method?.map(
        VerificationMethod.fromWASM,
      ),
    };

    if (obj.also_known_as !== undefined) result.alsoKnownAs = obj.also_known_as;
    if (obj.assertion_method !== undefined)
      result.assertionMethod = obj.assertion_method;
    if (obj.authentication !== undefined)
      result.authentication = obj.authentication;
    if (obj.capability_delegation !== undefined)
      result.capabilityDelegation = obj.capability_delegation;
    if (obj.capability_invocation !== undefined)
      result.capabilityInvocation = obj.capability_invocation;
    if (obj.context !== undefined) result.context = obj.context;
    if (obj.controller !== undefined) result.controller = obj.controller;
    if (obj.key_agreement !== undefined)
      result.keyAgreement = obj.key_agreement;
    if (obj.service !== undefined)
      result.service = obj.service?.map(Service.fromWASM);

    return result;
  };
}

export type FetchOptions = {
  body?: Uint8Array;
  headers?: any;
  method?: string;
};

export namespace FetchOptions {
  export const toWASM = (obj: FetchOptions): wasm.WasmFetchOptions => {
    return new wasm.WasmFetchOptions(obj.method, obj.headers, obj.body);
  };

  export const fromWASM = (obj: wasm.WasmFetchOptions): FetchOptions => {
    const result: FetchOptions = {};

    if (obj.body !== undefined) result.body = obj.body;
    if (obj.headers !== undefined) result.headers = obj.headers;
    if (obj.method !== undefined) result.method = obj.method;

    return result;
  };
}

export type Jwk = {
  alg?: string;
  crv: string;
  d?: string;
  kty: string;
  x: string;
  y?: string;
};

export namespace Jwk {
  export const toWASM = (obj: Jwk): wasm.WasmJwk => {
    return new wasm.WasmJwk(obj.alg, obj.kty, obj.crv, obj.d, obj.x, obj.y);
  };

  export const fromWASM = (obj: wasm.WasmJwk): Jwk => {
    const result: Jwk = {
      crv: obj.crv,
      kty: obj.kty,
      x: obj.x,
    };

    if (obj.alg !== undefined) result.alg = obj.alg;
    if (obj.d !== undefined) result.d = obj.d;
    if (obj.y !== undefined) result.y = obj.y;

    return result;
  };
}

export type Response = {
  body: Uint8Array;
  headers?: any;
  statusCode: number;
};

export namespace Response {
  export const toWASM = (obj: Response): wasm.WasmResponse => {
    return new wasm.WasmResponse(obj.statusCode, obj.headers, obj.body);
  };

  export const fromWASM = (obj: wasm.WasmResponse): Response => {
    const result: Response = {
      body: obj.body,
      statusCode: obj.status_code,
    };

    if (obj.headers !== undefined) result.headers = obj.headers;

    return result;
  };
}

export type Service = {
  id: string;
  serviceEndpoint: string[];
  type: string;
};

export namespace Service {
  export const toWASM = (obj: Service): wasm.WasmService => {
    return new wasm.WasmService(obj.id, obj.type, obj.serviceEndpoint);
  };

  export const fromWASM = (obj: wasm.WasmService): Service => {
    const result: Service = {
      id: obj.id,
      serviceEndpoint: obj.service_endpoint,
      type: obj.type,
    };

    return result;
  };
}

export type TbdexError = {
  isWeb5Error: boolean;
  message: string;
  variant: string;
};

export namespace TbdexError {
  export const toWASM = (obj: TbdexError): wasm.WasmTbdexError => {
    return new wasm.WasmTbdexError();
  };

  export const fromWASM = (obj: wasm.WasmTbdexError): TbdexError => {
    const result: TbdexError = {
      isWeb5Error: obj.is_web5_error,
      message: obj.message,
      variant: obj.variant,
    };

    return result;
  };
}

export type VerificationMethod = {
  controller: string;
  id: string;
  publicKeyJwk: Jwk;
  type: string;
};

export namespace VerificationMethod {
  export const toWASM = (
    obj: VerificationMethod,
  ): wasm.WasmVerificationMethod => {
    return new wasm.WasmVerificationMethod(
      obj.id,
      obj.type,
      obj.controller,
      Jwk.toWASM(obj.publicKeyJwk),
    );
  };

  export const fromWASM = (
    obj: wasm.WasmVerificationMethod,
  ): VerificationMethod => {
    const result: VerificationMethod = {
      controller: obj.controller,
      id: obj.id,
      publicKeyJwk: Jwk.fromWASM(obj.public_key_jwk),
      type: obj.type,
    };

    return result;
  };
}
