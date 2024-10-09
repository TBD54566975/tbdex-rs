import esbuild from "esbuild";

esbuild.build({
  entryPoints: ["./src/index.ts"],
  bundle: true,
  format: "cjs",
  sourcemap: true,
  minify: true,
  platform: "node",
  outfile: "dist/index.cjs",
});
