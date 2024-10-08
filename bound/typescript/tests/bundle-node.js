import { fileURLToPath } from 'node:url'

import esbuild from 'esbuild'
import path from 'node:path'
import fs from 'node:fs'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

fs.copyFileSync(`${__dirname}/../src/wasm/generated.d.ts`, `${__dirname}/compiled/bound/typescript/src/wasm/generated.d.ts`)
fs.copyFileSync(`${__dirname}/../src/wasm/generated.js`, `${__dirname}/compiled/bound/typescript/src/wasm/generated.js`)

esbuild.buildSync({
  entryPoints: [`${__dirname}/compiled/bound/typescript/tests/**/*.test.js`],
  format: 'esm',
  bundle: true,
  sourcemap: true,
  platform: 'node',
  target: ['node18'],
  outdir: `${__dirname}/compiled/node`,
});
