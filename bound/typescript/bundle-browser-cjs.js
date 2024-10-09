import esbuild from "esbuild";

esbuild.build({
  entryPoints : ["./src/index.ts"],
  bundle      : true,
  sourcemap   : true,
  minify      : true,
  platform    : "browser",
  target      : ["chrome101", "firefox108", "safari16"],
  define      : {
    "global": "globalThis",
  },
  format     : "iife",
  globalName : "tbDEX",
  outfile    : "dist/browser.js",
});
