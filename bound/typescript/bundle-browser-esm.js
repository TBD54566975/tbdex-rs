import esbuild from "esbuild";

esbuild.build({
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
  outfile: "dist/browser.mjs",
});
