import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { readWasmClasses } from "./wasm.js";
import { generateMappingsCode } from "./generate.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const tsCode = fs.readFileSync(
  path.resolve(__dirname, "../../src/wasm/generated.d.ts"),
  "utf8"
);

const inputCode = tsCode;
const wasmClasses = readWasmClasses(inputCode);
const outputCode = generateMappingsCode(wasmClasses);
fs.writeFileSync(
  path.resolve(__dirname, "../mappings.ts"),
  outputCode
);
