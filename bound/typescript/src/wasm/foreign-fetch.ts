import wasm from ".";
import { FetchOptions, Response } from "./generated-mappings";

export const ForeignFetch = {
  fetch: async (
    url: string,
    wasmFetchOptions?: wasm.WasmFetchOptions
  ): Promise<wasm.WasmResponse> => {
    const options = wasmFetchOptions
      ? FetchOptions.fromWASM(wasmFetchOptions)
      : undefined;

    const fetchResponse = await fetch(url, {
      method: options?.method || "GET",
      headers: options?.headers,
      body: options?.body ? Buffer.from(options.body) : undefined,
    });

    const body = new Uint8Array(await fetchResponse.arrayBuffer());
    const headers: Record<string, string> = {};
    fetchResponse.headers.forEach((value, key) => {
      headers[key] = value;
    });

    const response: Response = {
      statusCode: fetchResponse.status,
      body: body,
      headers: headers,
    };

    return Response.toWASM(response);
  },
};
