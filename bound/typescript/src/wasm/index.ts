import wasm from "./generated";
export { default } from "./generated";

wasm.loadWasmSync();

import * as f from "./foreign-fetch";
f;
