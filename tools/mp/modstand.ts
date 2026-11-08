// STANDING A NETWORKED GAME ON THE MODULE ALONE (bun): the game of rust/game_face, with no TypeScript game beside it. What a game server and a scripted client
// stand when a game is made from a start (src/net/start.ts) and not from a recipe (stand.ts).
//   freshStart(setup)   the start of a game for an address: its level's file in the look that is on, its seats, its die
//   standStart(start)   THE game a server or a client plays, stood from the start's bytes as every page stands it. As many as the module has room for.
import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { WireCmd } from '../../src/net/wire.ts';
import { commandBytes, startWhy, type NetStart } from '../../src/net/start.ts';
import { NetGame, type NetFace } from '../../src/net/modgame.ts';
import { wordTick } from '../../src/net/words.ts';
import { LOOK, lookLevelFile, lookOfQuery } from '../../src/scene/look.ts';
import { characterFiles, pack } from '../blitz/start-pack.ts';
import { faceHere } from '../blitz/face-build.ts';
import { wordsKept, type Stood } from './stood.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what whoever makes a game says of it: the page's address for the mission, the die, and what the host has set before the game is made (its commands' bytes as base64) */
export interface StartSetup { search: string; seed: number; commands?: string }
export function freshStart(setup: StartSetup): NetStart {
  const q = new URLSearchParams(setup.search), named = q.get('levelFile') ?? 'office.json';
  LOOK.on = lookOfQuery(q);
  const look = lookLevelFile(named), start: NetStart = { file: named, words: setup.search, seed: setup.seed, seats: Number(q.get('seats') ?? 1), commands: setup.commands ?? '' };
  const why = startWhy(start); if (why) throw new Error(`the setup: ${why}`);
  if (existsSync(resolve(ROOT, 'levels', look))) start.file = look;
  return start;
}

interface StartFace extends NetFace { face_start(p: number, n: number, slot: number, seed: number, seats: number, flags: number): number; face_reserve(n: number): void; face_show_seat(h: number, seat: number): void; face_panic_ptr(): number; face_panic_len(): number }
/** `face_start`'s: the rows that dress the picture are up, on a page and so on every machine of a game */
const DRESS = 1;
let MODULE: WebAssembly.Module | null = null;

/** every call of the face: a trap (a panic, the stack run out) is thrown as the panic's words */
function guarded(raw: Record<string, unknown>, memory: WebAssembly.Memory): StartFace {
  const out: Record<string, unknown> = {}, R = raw as unknown as StartFace;
  for (const k of Object.keys(raw)) { const f = raw[k]; out[k] = typeof f !== 'function' || !k.startsWith('face_') ? f : (...a: unknown[]): unknown => {
    try { return f(...a); } catch (e) { if (!(e instanceof WebAssembly.RuntimeError) && !(e instanceof RangeError)) throw e;
      let words = ''; try { words = new TextDecoder().decode(new Uint8Array(memory.buffer, R.face_panic_ptr() >>> 0, R.face_panic_len() >>> 0)); } catch { /* the trap's own words, then */ }
      throw new Error(`the game stopped in ${k}: ${words || (e as Error).message}`); } }; }
  return out as unknown as StartFace;
}

export async function standStart(start: NetStart): Promise<Stood> {
  const why = startWhy(start); if (why) throw new Error(`no networked game can be stood from this start: ${why}`);
  const level = resolve(ROOT, 'levels', start.file); if (!existsSync(level)) throw new Error(`${start.file} is not a level of this build`);
  if (!MODULE) { const file = faceHere(); if (!file) throw new Error('the game\'s module is not built and the checked-in one is of other sources (bun run tools/blitz/face-build.ts)'); MODULE = new WebAssembly.Module(new Uint8Array(readFileSync(file))); }
  const raw = new WebAssembly.Instance(MODULE, { env: { face_roll: () => { throw new Error('the game asked its host for a roll'); } } }).exports as Record<string, unknown>, memory = raw.memory as WebAssembly.Memory, X = guarded(raw, memory);
  const bytes = pack([readFileSync(level), new TextEncoder().encode(start.words), ...characterFiles(ROOT), new Uint8Array(0), commandBytes(start)]);
  const p = X.face_alloc(bytes.length) >>> 0; new Uint8Array(memory.buffer, p, bytes.length).set(bytes);
  const h = X.face_start(p, bytes.length, -1, start.seed, start.seats, DRESS) >>> 0;
  X.face_free(p, bytes.length); X.face_reserve(64 * 1048576);
  const game = new NetGame(X, memory, h), keep = wordsKept();
  game.clear();
  return {
    seats: start.seats, warnings: [], get ticks() { return game.ticks; },
    step(cmds: WireCmd[]) { game.step(cmds); game.clear(); if (keep && wordTick(game.ticks - 1)) keep(game.ticks, game.whole(true)); },
    quiet() { /* nothing of the game is kept beside it */ }, begin() { /* it stands */ }, show: seat => X.face_show_seat(h, seat),
    word: () => game.word(), watch: () => game.watch(), ended: () => game.ended(),
    ahead: (seat, cmds) => game.ahead(seat, cmds),
  };
}
