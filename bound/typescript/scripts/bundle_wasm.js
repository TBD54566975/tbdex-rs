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

// import { fileURLToPath } from 'node:url';
// import esbuild from 'esbuild';
// import path from 'node:path';
// import fs from 'node:fs';

// const __filename = fileURLToPath(import.meta.url);
// const __dirname = path.dirname(__filename);

// esbuild.buildSync({
//   entryPoints: [`${__dirname}/../src/index.ts`],  // Entry point for your project
//   format: 'esm',          // Output as ES module
//   bundle: true,           // Bundle all dependencies
//   minify: true,           // Minify the bundle for production
//   sourcemap: true,        // Enable source map for debugging
//   outfile: `${__dirname}/../dist/index.js`,  // Output file (bundled)
//   target: 'esnext',       // Target latest ECMAScript version
//   platform: 'node',       // Target platform (Node.js)
//   external: ['node_modules/*'] // Exclude node modules from the bundle
// });

// // Copy WASM files (if needed)
// fs.copyFileSync(path.resolve(__dirname, '../src/wasm/generated.js'), path.resolve(__dirname, '../dist/wasm/generated.js'));
// fs.copyFileSync(path.resolve(__dirname, '../src/wasm/generated.d.ts'), path.resolve(__dirname, '../dist/wasm/generated.d.ts'));
