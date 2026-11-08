// A NETWORKED GAME IN THE MODULE (rust/game_face net.rs): stepped on its seats' records, asked for its words, its end and where a seat's man would be. The same on
// a game server, a scripted client and a page; whoever starts the game (start.ts) and holds its number hands both in.
import type { WireCmd, Outcome } from './wire';
import { NET } from './wire';
import type { Ahead } from './foresee';
import { NET_WORD_SEED, foldWhole, foldWord, wordTick } from './words';

/** a record's numbers, as the module is handed them (rust/game_face net.rs RECORD) */
const RECORD = 18;
/** where each number of a look ahead lies (rust/game_face net.rs face_ahead); the path follows */
const AHEAD = { id: 0, down: 1, at: 2, yaw: 5, wall: 6, wallX: 7, wallZ: 8, near: 9, free: 10, pos: 11, aimYaw: 14, follow: 15, aims: 18, aim: 19, path: 22 } as const;

/** the module's calls that a networked game makes */
export interface NetFace {
  face_alloc(n: number): number; face_free(p: number, n: number): void; face_out_ptr(): number; face_out_len(): number; face_out_clear(h: number): void;
  face_networked(h: number): void; face_record(h: number, seat: number, p: number, n: number, keys: number, kn: number): void; face_ahead(h: number, seat: number, p: number, n: number, keys: number, kn: number): void;
  face_step(h: number, dt: number, flags: number): void; face_slot(h: number): number; face_die_n(h: number): number; face_parts(): number; face_part_hashes(slot: number): number;
  face_state_n(): number; face_state_name(i: number): void; face_state_words(h: number): void; face_verdict(h: number): number; face_part_name(i: number): void;
}

/** a key's code, as the referee has one: anything else is no key, and is left out before the codes become one text */
const CODE = /^[A-Za-z0-9]+$/;
const codes = (k: readonly string[] | undefined): string => k ? k.filter(c => CODE.test(c)).join(' ') : '';
/** a record's numbers into `N` at `at`; answers its keys' text: those held, a line feed, those that went down */
export function recordTo(N: Float64Array, at: number, w: WireCmd): string {
  N[at] = w.b; N[at + 1] = w.c ?? 0; N[at + 2] = w.w ?? 0; N[at + 3] = w.m[0]; N[at + 4] = w.m[1]; N[at + 5] = w.s[0]; N[at + 6] = w.s[1]; N[at + 7] = w.s[2]; N[at + 8] = w.s[3];
  for (let i = 0; i < 9; i++) N[at + 9 + i] = w.v[i];
  return `${codes(w.k)}\n${codes(w.p)}`;
}

export class NetGame {
  /** ticks stepped */
  ticks = 0;
  private w = NET_WORD_SEED; private wide: string | null = null; private names: string[] | null = null;
  private enc = new TextEncoder(); private dec = new TextDecoder();

  constructor(private readonly X: NetFace, private readonly memory: WebAssembly.Memory, readonly h: number) { X.face_networked(h); }

  /** records and their keys' text in the module's memory for one call */
  private handed(cmds: readonly WireCmd[], f: (p: number, n: number, keys: number, kn: number) => void): void {
    const N = new Float64Array(cmds.length * RECORD), keys = this.enc.encode(cmds.map((c, i) => recordTo(N, i * RECORD, c)).join('\n')), X = this.X;
    const p = X.face_alloc(N.byteLength) >>> 0, k = X.face_alloc(keys.length) >>> 0;
    new Uint8Array(this.memory.buffer, p, N.byteLength).set(new Uint8Array(N.buffer)); new Uint8Array(this.memory.buffer, k, keys.length).set(keys);
    f(p, N.byteLength, k, keys.length);
    X.face_free(p, N.byteLength); X.face_free(k, keys.length);   // (a call that trapped has lost the module: there is nothing to give back to)
  }
  private out(): Uint8Array { const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0; return new Uint8Array(this.memory.buffer, p, n); }
  private said(): string { return this.dec.decode(this.out()); }

  /** one tick on the seats' records, its word folded after it. What the tick told its host stays to be read (a page), until `clear` */
  step(cmds: readonly WireCmd[]): void {
    const X = this.X, h = this.h;
    cmds.forEach((c, seat) => this.handed([c], (p, n, k, kn) => X.face_record(h, seat, p, n, k, kn)));
    X.face_step(h, 1 / NET.hz, 0);   // (the saves and a rails level's view are a game of one's)
    this.w = foldWord(this.w, X.face_die_n(h), this.parts());
    if (wordTick(this.ticks++)) this.wide = foldWhole(this.whole().words);
  }
  private parts(): Uint32Array { const p = this.X.face_part_hashes(this.X.face_slot(this.h)) >>> 0, n = this.X.face_parts(); return new Uint32Array(this.memory.buffer, p, n); }
  /** what the ticks told their host is let go */
  clear(): void { this.X.face_out_clear(this.h); }
  /** the chained state word after the last tick */
  word(): string { return this.w.toString(36); }
  /** the WHOLE game's word after the last word tick (null before the first): every machine of the game started it alike, so every part of it is in the word */
  watch(): string | null { return this.wide; }
  /** the whole game's words, one a part, with the parts' names; after them the state word's own, as `block.<part>` */
  whole(named = false): { names: string[]; words: number[] } {
    const X = this.X; X.face_state_words(this.h); const b = this.out().slice(), words = Array.from(new Uint32Array(b.buffer));
    if (!named) return { names: [], words };
    this.names ??= [...Array.from({ length: X.face_state_n() }, (_, i) => { X.face_state_name(i); return this.said(); }), 'block.die', ...Array.from({ length: X.face_parts() }, (_, i) => { X.face_part_name(i); return `block.${this.said()}`; })];
    return { names: this.names, words: [...words, X.face_die_n(this.h), ...this.parts()] };
  }
  /** how the mission ended (null while it is being played) */
  ended(): Outcome | null { const r = this.X.face_verdict(this.h); if (!r) return null; const [end, why = ''] = this.said().split('\n'); return { won: r === 2, end, why }; }

  /** where the man of a seat would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: readonly WireCmd[]): Ahead {
    let a!: Ahead; this.handed(cmds, (p, n, k, kn) => { this.X.face_ahead(this.h, seat, p, n, k, kn);
      const b = this.out().slice(), v = new Float64Array(b.buffer), three = (i: number): [number, number, number] => [v[i], v[i + 1], v[i + 2]];
      a = { id: v[AHEAD.id], down: v[AHEAD.down] !== 0, at: three(AHEAD.at), yaw: v[AHEAD.yaw], wall: v[AHEAD.wall] ? [v[AHEAD.wallX], v[AHEAD.wallZ]] : null, near: v[AHEAD.near], free: v[AHEAD.free] !== 0, pos: three(AHEAD.pos), aimYaw: v[AHEAD.aimYaw], follow: three(AHEAD.follow), aim: v[AHEAD.aims] ? three(AHEAD.aim) : null,
        path: Array.from({ length: (v.length - AHEAD.path) / 3 }, (_, i) => three(AHEAD.path + i * 3)) }; });
    return a;
  }
}
