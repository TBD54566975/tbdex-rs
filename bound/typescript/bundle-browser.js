import esbuild from "esbuild";

const browserConfig = {
  entryPoints : ["./src/index.ts"],
  bundle      : true,
  format      : "esm",
  sourcemap   : true,
  minify      : true,
  platform    : "browser",
  target      : ["chrome101", "firefox108", "safari16"],
  define      : {
    "global": "globalThis",
  },
};

// esm polyfilled bundle for browser
esbuild.build({
  ...browserConfig,
  outfile: "dist/browser.mjs",
});

// iife polyfilled bundle for browser
esbuild.build({
  ...browserConfig,
  format     : "iife",
  globalName : "tbDEX",
  outfile    : "dist/browser.js",
});
