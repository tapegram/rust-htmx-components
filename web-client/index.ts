console.log("Hello via Bun!");

await Bun.build({
  entrypoints: ['./src/client/common.ts'],
  outdir: './out',
});
