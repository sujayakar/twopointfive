// A RUN PLAYED AGAIN FROM ITS RECORD (record.ts): the Rust game alone. Exit 0 the end is the recorded end · 1 it parted, or the game panicked · 2 no such file.
//   bun run tools/blitz/replay.ts <file> [--wasm FILE]
import { existsSync, readFileSync } from 'node:fs';
import { Reader, readRecord, ROLL, END, MEM, endOf } from './record';
import { faceHere, FACE_WASM } from './face-build.ts';
const argv = process.argv.slice(2), file = argv[0], at = argv.indexOf('--wasm'), wasm = at < 0 ? faceHere() ?? FACE_WASM : argv[at + 1];
if (!file || !existsSync(file)) { console.error(`no record at ${file}`); process.exit(2); }
const R: Reader = readRecord(file); const names: string[] = []; const rolls: number[] = [];
const X: any = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { if (!rolls.length) throw new Error('the record has no die here'); return rolls.shift(); } } }).exports;
let h = 0, slot = 0, calls = 0, steps = 0, dropped = 0, end: { die: number; hashes: number[] } | null = null; const t0 = performance.now();
/** a caller's die is written as it is asked, so inside its call: the ones behind this call are read ahead */
const readAheadRolls = () => { for (;;) { const p = R.p; if (!R.more) return; const k = R.u16(); if (k !== ROLL) { R.p = p; return; } rolls.push(R.f64()); } };
while (R.more) { const k = R.u16();
  if (k === 0xfffd) { names.push(R.str()); continue; }
  if (k === ROLL) { rolls.push(R.f64()); continue; }
  if (k === MEM) { const at = R.f64(); new Uint8Array(X.memory.buffer).set(R.bytes(), at); continue; }
  if (k === END) { const die = R.f64(), n = R.u32(), H: number[] = []; for (let i = 0; i < n; i++) H.push(R.u32()); end = { die, hashes: H }; break; }
  const name = names[k], argc = R.u16(), a: number[] = [], made: [number, number][] = [];
  for (let j = 0; j < argc; j++) { const tag = R.u16(); if (tag === 0) { a[j] = R.f64(); continue; } if (tag === 1) { const u = R.bytes(), p = X.face_alloc(u.length) >>> 0; new Uint8Array(X.memory.buffer).set(u, p); a[j] = p; a[j + 1] = u.length; made.push([p, u.length]); j++; R.u16(); } }
  readAheadRolls();
  if (process.env.RECORD_BREAK === '1' && name === 'face_step' && ++dropped % 50 === 0) continue;
  try { const r = X[name](...a); calls++; if (name === 'face_step') steps++; if ((name === 'face_new' || name === 'face_new_at') && r) { h = r; slot = X.face_slot(r) >>> 0; } }
  catch (e) { const p = X.face_panic_ptr() >>> 0, n = X.face_panic_len() >>> 0; console.log(`the Rust game panicked in call ${calls} (${name}), after ${steps} steps: ${new TextDecoder().decode(new Uint8Array(X.memory.buffer, p, n)) || e}`); process.exit(1); }
  for (const [p, n] of made) X.face_free(p, n); }
if (!end) { console.log(`the record ends with no end written (the run was cut off): ${calls} calls, ${steps} steps played, no panic`); process.exit(0); }
const now = endOf(X, h, slot), parted: number[] = []; for (let i = 0; i < now.hashes.length; i++) if (now.hashes[i] !== end.hashes[i]) parted.push(i);
const bad = now.die !== end.die || parted.length > 0 || now.hashes.length !== end.hashes.length;
console.log(`${calls} calls, ${steps} steps in ${((performance.now() - t0) / 1000).toFixed(1)} s: ${bad ? `PARTED: die ${now.die} against ${end.die}; parts ${[...new Set(parted)].slice(0, 12).map(i => { if (i >= (X.face_parts() >>> 0)) return 'the whole export'; X.face_part_name(i); const p = X.face_out_ptr() >>> 0, n = X.face_out_len() >>> 0; return new TextDecoder().decode(new Uint8Array(X.memory.buffer, p, n)); }).join(', ')}` : `the end is the recorded end (the die ${end.die}, ${now.hashes.length - 2} parts of the block, the whole export)`}`);
process.exit(bad ? 1 : 0);
