import { fileURLToPath } from 'node:url';
import esbuild from 'esbuild';
import path from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

esbuild.buildSync({
  entryPoints: [`${__dirname}/../src/index.ts`],
  format: 'esm',
  bundle: true,
  minify: true,
  sourcemap: true,
  outfile: `${__dirname}/../dist/index.js`,
  target: 'esnext',
  platform: 'node',
  external: ['node_modules/*']
});