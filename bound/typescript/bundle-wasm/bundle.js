import { fileURLToPath } from 'node:url'

import esbuild from 'esbuild'
import path from 'node:path'
import fs from 'node:fs'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

esbuild.buildSync({
  format: 'esm',
  bundle: true,
  minify: true,
  entryPoints: [`${__dirname}/../pkg/tbdex_wasm.js`],
  outfile: `${__dirname}/../src/wasm/generated.js`,
  allowOverwrite: true,
})

fs.copyFileSync(`${__dirname}/../pkg/tbdex_wasm.d.ts`, `${__dirname}/../src/wasm/generated.d.ts`)
