import wasm from ".";
import { Response } from "./mappings";

export const ForeignFetch = {
  fetch: (
    url: string,
    wasmFetchOptions?: wasm.WasmFetchOptions
  ): wasm.WasmResponse => {
    // TODO use the below proof of concept code to execute a fetch synchronously, but async within the worker thread,
    // TODO and then return the wasm.WasmResponse
    throw new Error("TODO!");
  },
};

const isNode =
  typeof process !== "undefined" &&
  process.versions != null &&
  process.versions.node != null;

if (isNode) {
  import("worker_threads")
    .then((module) => {
      const sharedBuffer = new SharedArrayBuffer(4);
      const sharedArray = new Int32Array(sharedBuffer);

      const workerCode = `
        const { parentPort } = require('worker_threads');
        const sharedArray = new Int32Array(require('worker_threads').workerData.sharedBuffer);

        parentPort.on('message', async (options) => {
          try {
            const { method, headers, body } = options;

            const response = await fetch(options.url, {
              method: method || 'GET',
              headers: headers,
              body: body ? Buffer.from(body) : undefined
            });

            const responseBody = new Uint8Array(await response.arrayBuffer());
            const wasmResponse = {
              status_code: response.status,
              headers: Array.from(response.headers.entries()), // Convert headers to array
              body: responseBody
            };

            // Store the status code in shared memory
            Atomics.store(sharedArray, 0, response.status);

            // Send the body and headers in a separate message
            parentPort.postMessage({
              headers: wasmResponse.headers,
              body: wasmResponse.body
            });

            // Notify the main thread that the status code is ready
            Atomics.notify(sharedArray, 0);
          } catch (error) {
            console.error('Worker fetch error:', error);
            Atomics.store(sharedArray, 0, -1); // Indicate failure
            Atomics.notify(sharedArray, 0);
          }
        });
        `;

      if (module.isMainThread) {
        const worker = new module.Worker(workerCode, {
          eval: true,
          workerData: { sharedBuffer },
        });

        const fetchOptions = new wasm.WasmFetchOptions(
          "POST",
          { "Content-Type": "application/json" },
          new Uint8Array(Buffer.from(JSON.stringify({ key: "value" })))
        );

        worker.postMessage({
          url: "https://httpbin.org/post",
          method: fetchOptions.method,
          headers: fetchOptions.headers,
          body: fetchOptions.body,
        });

        Atomics.wait(sharedArray, 0, 0);

        const statusCode = Atomics.load(sharedArray, 0);

        if (statusCode === -1) {
          console.error("Fetch request failed in the worker");
        } else {
          console.log("Fetch response status code:", statusCode);

          worker.on("message", (message) => {
            const { headers, body } = message;
            const wasmResponse = new wasm.WasmResponse(
              statusCode,
              headers,
              body
            );

            console.log("Response:", wasmResponse);
            console.log("Response:", Response.fromWASM(wasmResponse));
          });
        }
      }
    })
    .catch((err) => console.error("Failed to load Node module:", err));
} else {
  const workerCode = `
    onmessage = async (event) => {
      const url = event.data;
      try {
        const response = await fetch(url);
        const statusCode = response.status;
        postMessage(statusCode);
      } catch (error) {
        console.error('Worker fetch error:', error);
        postMessage({ status: 'Error', message: error.message });
      }
    };
  `;

  const blob = new Blob([workerCode], { type: "application/javascript" });
  const worker = new Worker(URL.createObjectURL(blob));

  const waitForWorker = () => {
    return new Promise((resolve) => {
      worker.onmessage = (event) => {
        console.log("fetch response status code", event.data);
        resolve(undefined);
      };
    });
  };

  const run = async () => {
    try {
      worker.postMessage("https://jsonplaceholder.typicode.com/todos/1");
      await waitForWorker();
      worker.terminate();
    } catch (err) {
      console.error(err);
    }
  };

  await run();
}
