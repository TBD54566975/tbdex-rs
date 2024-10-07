import wasm from ".";

export const ForeignFetch = {
  fetch: async (
    url: string,
    options?: wasm.WasmFetchOptions
  ): Promise<wasm.WasmResponse> => {
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

    return new wasm.WasmResponse(fetchResponse.status, headers, body);
  },
};
