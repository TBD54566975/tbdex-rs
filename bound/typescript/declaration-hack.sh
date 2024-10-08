#!/bin/bash

sed -i '' "/export { default } from \"\.\/generated\";/r src/wasm/generated.d.ts" dist/index.d.ts && \
  sed -i '' "/export { default } from \"\.\/generated\";/d" dist/index.d.ts