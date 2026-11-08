// WRITTEN BY tools/sim/build.ts — never by hand. The Rust dressing (rust/dress) built to WebAssembly, its bytes as base64: src/dress/dress.ts loads it.
// `bun run tools/sim/build.ts` after any change under rust/; `bun run tools/sim/build.ts --check` (and tools/qa/rust-check.ts) is red when this file is not
// the build of rust/dress as it stands.
/** the hash of the crates' sources this was built from */
export const DRESS_SOURCE = '55e52078728cf5b1031078ed2e5834b4';
/** the compiler and the kind of machine that built it (the same sources built by another may give other bytes) */
export const DRESS_BUILDER = 'rustc 1.97.0 on x86_64-unknown-linux-gnu';
/** the module's size in bytes */
export const DRESS_BYTES = 117;
export const DRESS_WASM: readonly string[] = [
  'AGFzbQEAAAABBQFgAAF/AwQDAAAABAUBcAEBAQUDAQARBgkBfwFBgIDAAAsHMAQGbWVtb3J5AgAJZHJlc3NfYWJpAAAIZHJlc3NfaW4AAQxkcmVzc19pbl9sZW4AAgoVAwQAQQELCABBgIDAgAALBQBBgCAL',
];
