import esbuild from "esbuild";

esbuild.build({
  entryPoints: ["./src/index.ts"],
  bundle: true,
  format: "esm",
  sourcemap: true,
  minify: true,
  platform: "node",
  outfile: "dist/index.mjs",
});

