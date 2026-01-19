import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['esm'],
  dts: true,
  clean: true,
  sourcemap: true,
  // Ensure WASM artifacts exist next to the built dist output (esp. in --watch),
  // otherwise browser/Node init will hang or fail when fetching the .wasm file.
  onSuccess: 'node scripts/copy-rust.mjs',
})