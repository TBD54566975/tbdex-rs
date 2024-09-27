import wasm from "./generated";
export { default } from "./generated";
import { ForeignFetch } from "./foreign-fetch";

wasm.loadWasmSync();
wasm.set_http_client(ForeignFetch);
