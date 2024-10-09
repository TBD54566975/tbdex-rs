// import wasm from "./generated";
// export { default } from "./generated";
// import { ForeignFetch } from "./foreign-fetch";

// wasm.loadWasmSync();
// wasm.set_http_client(ForeignFetch);

// import * as tbdex_wasm from "tbdex_wasm";
// export { default } from "tbdex_wasm";
// import { ForeignFetch } from "./foreign-fetch";

// tbdex_wasm.set_http_client(ForeignFetch);

import init, * as wasm from 'tbdex_wasm';
import { ForeignFetch } from './foreign-fetch';

await init();
wasm.set_http_client(ForeignFetch);

export default wasm;