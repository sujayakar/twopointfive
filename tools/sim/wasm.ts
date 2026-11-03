// A WEBASSEMBLY MODULE READ BY ITS BYTES — what tools/sim/build.ts needs to hold the line between gameplay and dressing on the thing that actually runs
// (docs/agents/rust.md): what a module IMPORTS (a module that imports nothing has no world outside: no clock, no random source, no host call, nobody
// else's memory), and which INSTRUCTIONS its code uses, by family. The families that matter:
//   relaxed   relaxed SIMD (0xFD 0x100 … 0x113): each may answer differently on different machines. Dressing may use it; the simulation never.
//   atomics   threads (0xFE …): an order nobody wrote down.
//   simd      fixed-width SIMD (the rest of 0xFD): the same answer everywhere, so the simulation may.
// EVERY instruction is decoded, because an opcode's bytes can only be told from an immediate's by walking them all; and an opcode this file does not know
// is reported, never skipped — a new proposal's instruction makes the check red until somebody says which family it is.

export interface WasmRead {
  /** 'module.field (kind)' of every import */
  imports: string[];
  exports: string[];
  /** linear memories defined by the module itself */
  memories: number;
  funcs: number;
  /** instructions counted by family */
  simd: number; relaxed: number; atomics: number;
  /** what could not be read ('' = the whole module was): the first thing, with where */
  unknown: string;
}

class R {
  i = 0;
  constructor(readonly b: Uint8Array, readonly end = b.length) {}
  byte(): number { if (this.i >= this.end) throw new Error('ran off the end'); return this.b[this.i++]; }
  /** an unsigned LEB128 (as a number: the tools' sizes and indices are far below 2^53) */
  u(): number { let r = 0, m = 1; for (;;) { const x = this.byte(); r += (x & 0x7f) * m; if (!(x & 0x80)) return r; m *= 128; } }
  /** a LEB128 of either sign, skipped */
  skipLeb(): void { while (this.byte() & 0x80) { /* */ } }
  skip(n: number): void { if (this.i + n > this.end) throw new Error('ran off the end'); this.i += n; }
  name(): string { const n = this.u(), s = Buffer.from(this.b.subarray(this.i, this.i + n)).toString('utf8'); this.skip(n); return s; }
}
const KIND = ['func', 'table', 'memory', 'global', 'tag'];
function limits(r: R): void { const f = r.byte(); r.u(); if (f & 1) r.u(); }
function memarg(r: R): void { const a = r.u(); if (a & 0x40) r.u(); r.skipLeb(); }

/** the numbers of the fixed-width SIMD space (0xFD 0x00 … 0xFF) that are NO instruction: 236 of the 256 are */
const SIMD_HOLES = new Set([0x9a, 0xa2, 0xa5, 0xa6, 0xaf, 0xb0, 0xb2, 0xb3, 0xb4, 0xbb, 0xc2, 0xc5, 0xc6, 0xcf, 0xd0, 0xd2, 0xd3, 0xd4, 0xe2, 0xee]);
/** one function body's instructions walked; counts into `out`, throws on an opcode nobody has named */
function walk(r: R, out: WasmRead): void {
  while (r.i < r.end) {
    const at = r.i, op = r.byte();
    if (op <= 0x01 || op === 0x05 || op === 0x0b || op === 0x0f || op === 0x1a || op === 0x1b || (op >= 0x45 && op <= 0xc4) || op === 0xd1) continue;
    if (op >= 0x02 && op <= 0x04) { r.skipLeb(); continue; }                       // block / loop / if: a block type
    if (op === 0x0c || op === 0x0d || op === 0x10 || op === 0x12 || (op >= 0x20 && op <= 0x26) || op === 0xd2) { r.u(); continue; }
    if (op === 0x0e) { const n = r.u(); for (let k = 0; k <= n; k++) r.u(); continue; }
    if (op === 0x11 || op === 0x13) { r.u(); r.u(); continue; }
    if (op === 0x1c) { const n = r.u(); r.skip(n); continue; }
    if (op >= 0x28 && op <= 0x3e) { memarg(r); continue; }
    if (op === 0x3f || op === 0x40) { r.u(); continue; }
    if (op === 0x41 || op === 0x42 || op === 0xd0) { r.skipLeb(); continue; }
    if (op === 0x43) { r.skip(4); continue; }
    if (op === 0x44) { r.skip(8); continue; }
    if (op === 0xfc) {
      const s = r.u();
      if (s <= 7) continue;                                                        // saturating float → int
      if (s === 8 || s === 10 || s === 12 || s === 14) { r.u(); r.u(); continue; } // memory.init / memory.copy / table.init / table.copy
      if (s === 9 || s === 11 || s === 13 || (s >= 15 && s <= 17)) { r.u(); continue; }
      throw new Error(`instruction 0xfc ${s} at byte ${at}`);
    }
    if (op === 0xfd) {
      const s = r.u();
      if (s >= 0x100 && s <= 0x113) { out.relaxed++; continue; }                   // relaxed SIMD: none carries an immediate
      if (s > 0xff || SIMD_HOLES.has(s)) throw new Error(`instruction 0xfd ${s} at byte ${at}`);
      out.simd++;
      if (s <= 0x0b || s === 0x5c || s === 0x5d) memarg(r);
      else if (s === 0x0c || s === 0x0d) r.skip(16);
      else if (s >= 0x15 && s <= 0x22) r.skip(1);
      else if (s >= 0x54 && s <= 0x5b) { memarg(r); r.skip(1); }
      continue;
    }
    if (op === 0xfe) {                                                             // threads: counted, then its immediate (a memarg, bar the fence's one byte)
      const s = r.u(); out.atomics++;
      if (s === 0x03) r.skip(1); else memarg(r);
      continue;
    }
    throw new Error(`instruction 0x${op.toString(16)} at byte ${at}`);
  }
}

/** the module read: never throws (what it could not read is `unknown`) */
export function readWasm(bytes: Uint8Array): WasmRead {
  const out: WasmRead = { imports: [], exports: [], memories: 0, funcs: 0, simd: 0, relaxed: 0, atomics: 0, unknown: '' };
  try {
    if (bytes.length < 8 || bytes[0] !== 0 || bytes[1] !== 0x61 || bytes[2] !== 0x73 || bytes[3] !== 0x6d) throw new Error('not a WebAssembly module');
    const r = new R(bytes); r.i = 8;
    while (r.i < bytes.length) {
      const id = r.byte(), size = r.u(), s = new R(bytes, r.i + size); s.i = r.i; r.skip(size);
      if (id === 2) {
        for (let n = s.u(); n > 0; n--) {
          const mod = s.name(), field = s.name(), kind = s.byte(); out.imports.push(`${mod}.${field} (${KIND[kind] ?? kind})`);
          if (kind === 0 || kind === 4) { if (kind === 4) s.byte(); s.u(); } else if (kind === 1) { s.byte(); limits(s); } else if (kind === 2) limits(s); else if (kind === 3) s.skip(2); else throw new Error(`import kind ${kind}`);
        }
      } else if (id === 5) out.memories += s.u();
      else if (id === 7) { for (let n = s.u(); n > 0; n--) { out.exports.push(s.name()); s.byte(); s.u(); } }
      else if (id === 10) {
        for (let n = s.u(); n > 0; n--) {
          const len = s.u(), f = new R(bytes, s.i + len); f.i = s.i; s.skip(len); out.funcs++;
          for (let groups = f.u(); groups > 0; groups--) { f.u(); f.byte(); }       // the locals: a count and a type each
          walk(f, out);
        }
      }
    }
  } catch (e) { out.unknown = String((e as Error)?.message ?? e); }
  return out;
}

// ---- a module made by hand, for the checks' red paths: one function `f` whose body is the bytes given, and the imports named
const leb = (n: number): number[] => { const o: number[] = []; do { let x = n & 0x7f; n = Math.floor(n / 128); if (n) x |= 0x80; o.push(x); } while (n); return o; };
const str = (s: string): number[] => [...leb(s.length), ...Buffer.from(s, 'utf8')];
const sec = (id: number, body: number[]): number[] => [id, ...leb(body.length), ...body];
/** a module with one exported function of no parameters and no results whose instructions are `body` (its `end` is added), importing each `imports` name as a function of the same type */
export function tinyWasm(body: number[], imports: string[] = []): Uint8Array {
  const code = [0, ...body, 0x0b];
  return new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...sec(1, [1, 0x60, 0, 0]),
    ...(imports.length ? sec(2, [...leb(imports.length), ...imports.flatMap(n => [...str('env'), ...str(n), 0, 0])]) : []),
    ...sec(3, [1, 0]),
    ...sec(7, [1, ...str('f'), 0, ...leb(imports.length)]),
    ...sec(10, [1, ...leb(code.length), ...code]),
  ]);
}
