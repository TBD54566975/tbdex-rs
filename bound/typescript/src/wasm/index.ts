import wasm from "./generated";
export { default } from "./generated";

wasm.loadWasmSync();

const foreignFetch = {
  fetch: (url: string, options?: wasm.WasmFetchOptions): wasm.WasmResponse => {
    console.log("beginning wait");
    fetch("https://example.com").then(console.log);
    console.log("ending wait");

    throw new Error("implement but must be sync!!!");
  },
};

wasm.set_http_client(foreignFetch);
