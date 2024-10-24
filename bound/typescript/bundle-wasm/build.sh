#!/bin/bash
#
# Build the JavaScript modules
#
# This script is really a workaround for https://github.com/rustwasm/wasm-pack/issues/1074.
#
# Currently, the only reliable way to load WebAssembly in all the JS
# environments we want to target seems to be to pack the WASM into base64, 
# and then unpack it and instantiate it at runtime.
#
# Hopefully one day, https://github.com/rustwasm/wasm-pack/issues/1074 will be
# fixed and this will be unnecessary.

set -e

(cd $(dirname "$0")/../../../; just wasm)

cd $(dirname "$0")/..

# Convert the Wasm into a JS file that exports the base64'ed Wasm.
echo "module.exports = \`$(base64 -i pkg/tbdex_wasm_bg.wasm)\`;" > pkg/tbdex_wasm_bg.wasm.js

# In the JavaScript:
#  1. Strip out the lines that load the WASM, add our new epilogue.
#  2. Remove the imports of `TextDecoder` and `TextEncoder`. We rely on the global defaults.
{
  sed -e '/Text..coder.*= require(.util.)/d' \
      -e '/^const path = /,$d' pkg/tbdex_wasm.js
  cat bundle-wasm/epilogue.js
} > pkg/tbdex_wasm.js.new
mv pkg/tbdex_wasm.js.new pkg/tbdex_wasm.js

# also extend the typescript
cat bundle-wasm/epilogue.d.ts >> pkg/tbdex_wasm.d.ts