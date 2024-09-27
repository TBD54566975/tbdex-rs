import wasm from ".";
import { FetchOptions, Response } from "./mappings";

let workerThreads: any | undefined;

const IS_NODEJS =
  typeof process !== "undefined" &&
  process.versions != null &&
  process.versions.node != null;

if (IS_NODEJS) {
  try {
    workerThreads = await import("worker_threads");
  } catch (err) {
    console.error("Failed to load worker_threads in Node.js environment:", err);
  }
}

export const ForeignFetch = {
  fetch: (
    url: string,
    wasmFetchOptions?: wasm.WasmFetchOptions
  ): wasm.WasmResponse => {
    return fetchSync(url, wasmFetchOptions);
  },
};

const fetchSync = (
  url: string,
  wasmFetchOptions?: wasm.WasmFetchOptions
): wasm.WasmResponse => {
  if (IS_NODEJS) {
    const response = fetchSyncNode(
      url,
      wasmFetchOptions ? FetchOptions.fromWASM(wasmFetchOptions) : undefined
    );
    return Response.toWASM(response);
  } else {
    const response = fetchSyncBrowser(
      url,
      wasmFetchOptions ? FetchOptions.fromWASM(wasmFetchOptions) : undefined
    );
    return Response.toWASM(response);
  }
};

const fetchSyncNode = (url: string, options?: FetchOptions): Response => {
  const statusBuffer = new SharedArrayBuffer(4);
  const headersBuffer = new SharedArrayBuffer(1024);
  const bodyBuffer = new SharedArrayBuffer(1024 * 10);

  const statusArray = new Int32Array(statusBuffer);
  const headersArray = new Uint8Array(headersBuffer);
  const bodyArray = new Uint8Array(bodyBuffer);

  const workerCode = `
    const { parentPort } = require('worker_threads');
    const statusArray = new Int32Array(require('worker_threads').workerData.statusBuffer);
    const headersArray = new Uint8Array(require('worker_threads').workerData.headersBuffer);
    const bodyArray = new Uint8Array(require('worker_threads').workerData.bodyBuffer);

    parentPort.on('message', async (options) => {
      try {
        const { method, headers, body } = options;

        const response = await fetch(options.url, {
          method: method || 'GET',
          headers: headers,
          body: body ? Buffer.from(body) : undefined
        });

        const responseBody = new Uint8Array(await response.arrayBuffer());
        const responseHeaders = JSON.stringify(Array.from(response.headers.entries())); // Convert headers to JSON

        // Write status code to shared buffer
        Atomics.store(statusArray, 0, response.status);

        // Write headers to the headers buffer
        const encoder = new TextEncoder();
        const encodedHeaders = encoder.encode(responseHeaders);
        headersArray.set(encodedHeaders, 0); // Store headers starting at index 0

        // Write body to the body buffer
        bodyArray.set(responseBody, 0); // Store body starting at index 0

        // Notify the main thread that the response is ready
        Atomics.notify(statusArray, 0);
      } catch (error) {
        console.error('Worker fetch error:', error);
        Atomics.store(statusArray, 0, -1); // Indicate failure
        Atomics.notify(statusArray, 0);
      }
    });
  `;

  if (workerThreads === undefined)
    throw Error("worker_threads must be imported");

  if (workerThreads.isMainThread) {
    const worker = new workerThreads.Worker(workerCode, {
      eval: true,
      workerData: {
        statusBuffer,
        headersBuffer,
        bodyBuffer,
      },
    });

    worker.postMessage({
      url: url,
      method: options?.method,
      headers: options?.headers,
      body: options?.body,
    });

    Atomics.wait(statusArray, 0, 0);

    const statusCode = Atomics.load(statusArray, 0);

    if (statusCode === -1) {
      throw new Error("Fetch request failed in the worker");
    }

    const decoder = new TextDecoder();
    const decodedHeaders = decoder.decode(
      headersArray.subarray(0, headersArray.indexOf(0))
    );
    const headers = JSON.parse(decodedHeaders);

    const body = bodyArray.slice(0, bodyArray.indexOf(0));

    const response: Response = {
      statusCode,
      headers,
      body,
    };

    return response;
  }

  throw Error("must be main thread");
};

const fetchSyncBrowser = (
  url: string,
  fetchOptions?: FetchOptions
): Response => {
  const xhr = new XMLHttpRequest();
  xhr.open(fetchOptions?.method || "GET", url, false);

  if (fetchOptions?.headers) {
    Object.entries(fetchOptions.headers).forEach(([key, value]) => {
      xhr.setRequestHeader(key, value as string);
    });
  }

  xhr.send(fetchOptions?.body ? new Uint8Array(fetchOptions.body) : null);

  console.log("kw dbg", xhr.response);
  const response: Response = {
    statusCode: xhr.status,
    headers: xhr.getAllResponseHeaders(),
    body: new Uint8Array(xhr.response),
  };

  return response;
};
