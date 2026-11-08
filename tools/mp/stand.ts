// STANDING A NETWORKED GAME (bun): the game of rust/game_face, as a game server and a scripted client hold it.
//   freshStart(setup)   the start of a game for an address: its level's file, its seats, its die, its difficulty
//   standStart(start)   THE game a server or a client plays, stood from the start's bytes as every page stands it. As many as the module has room for.
//   buildName()         the build this checkout is
import { appendFileSync, existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { WireCmd, Outcome } from '../../src/net/wire.ts';
import type { Ahead } from '../../src/net/foresee.ts';
import { commandBytes, commandText, startWhy, type NetStart } from '../../src/net/start.ts';
import { NetGame, type NetFace } from '../../src/net/modgame.ts';
import { wordTick } from '../../src/net/words.ts';
import { Commands } from '../../src/draw/commands.ts';
import { machineBefore } from '../../src/host/machine.ts';
import { LOOK } from '../../src/scene/look.ts';
import { characterFiles, pack } from '../blitz/start-pack.ts';
import { faceHere } from '../blitz/face-build.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what whoever makes a game says of it: the page's address for the mission, the die, and what the host has set before the game is made (its commands' bytes as base64; none said: what
 *  the address itself says of it, on a machine that keeps nothing) */
export interface StartSetup { search: string; seed: number; commands?: string }
export function freshStart(setup: StartSetup): NetStart {
  const q = new URLSearchParams(setup.search), named = q.get('levelFile') ?? 'office.json', C = new Commands();
  LOOK.on = true;
  if (setup.commands === undefined) machineBefore(q, named, C);
  const start: NetStart = { file: named, words: setup.search, seed: setup.seed, seats: Number(q.get('seats') ?? 1), commands: setup.commands ?? commandText(C.take()) };
  const why = startWhy(start); if (why) throw new Error(`the setup: ${why}`);
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

/** a networked game as a machine with no page holds it */
export interface Stood {
  readonly seats: number; readonly ticks: number;
  /** one tick on the seats' records, the state word folded after it */
  step(cmds: WireCmd[]): void;
  /** the chained state word after the last tick (src/net/words.ts) · the whole game's word after the last word tick (null before the first) */
  word(): string; watch(): string | null;
  /** how the mission ended, by the game's own verdict: null while it is being played */
  ended(): Outcome | null;
  /** which seat this machine shows: nothing a tick reads */
  show(seat: number): void;
  /** where a seat's man would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: WireCmd[]): Ahead;
}

export async function standStart(start: NetStart): Promise<Stood> {
  const why = startWhy(start); if (why) throw new Error(`no networked game can be stood from this start: ${why}`);
  const level = resolve(ROOT, 'levels', start.file); if (!existsSync(level)) throw new Error(`${start.file} is not a level of this build`);
  if (!MODULE) { const file = faceHere(); if (!file) throw new Error('the game\'s module is not built and the checked-in one is of other sources (bun run tools/blitz/face-build.ts)'); MODULE = new WebAssembly.Module(new Uint8Array(readFileSync(file))); }
  const raw = new WebAssembly.Instance(MODULE, { env: { face_roll: () => { throw new Error('the game asked its host for a roll'); } } }).exports as Record<string, unknown>, memory = raw.memory as WebAssembly.Memory, X = guarded(raw, memory);
  const bytes = pack([readFileSync(level), new TextEncoder().encode(start.words), ...characterFiles(ROOT), commandBytes(start)]);
  const p = X.face_alloc(bytes.length) >>> 0; new Uint8Array(memory.buffer, p, bytes.length).set(bytes);
  const h = X.face_start(p, bytes.length, -1, start.seed, start.seats, DRESS) >>> 0;
  X.face_free(p, bytes.length); X.face_reserve(64 * 1048576);
  const game = new NetGame(X, memory, h), keep = wordsKept();
  game.clear();
  return {
    seats: start.seats, get ticks() { return game.ticks; },
    step(cmds) { game.step(cmds); game.clear(); if (keep && wordTick(game.ticks - 1)) keep(game.ticks, game.whole(true)); },
    word: () => game.word(), watch: () => game.watch(), ended: () => game.ended(),
    show: seat => X.face_show_seat(h, seat),
    ahead: (seat, cmds) => game.ahead(seat, cmds),
  };
}

/** MP_WORDS=<a file's path up to its last dot>: this process writes the whole game's words after every word tick to `<that>.<pid>.words`, a line a tick with
 *  the parts' names first, for two machines' files to be held to each other (words-diff.ts). null = nobody asked */
function wordsKept(): ((tick: number, A: { names: string[]; words: number[] }) => void) | null {
  const file = process.env.MP_WORDS ? `${process.env.MP_WORDS}.${process.pid}.words` : ''; let named = false;
  return file ? (tick, A) => { if (!named) { named = true; appendFileSync(file, `names ${A.names.join(' ')}\n`); } appendFileSync(file, `${tick} ${A.words.map(x => x.toString(36)).join(' ')}\n`); } : null;
}
/** the build this checkout is: its commit, or '' when it has uncommitted changes (those are no commit's build) or is no checkout */
export function buildName(): string {
  try { const git = (...a: string[]) => { const r = Bun.spawnSync(['git', ...a], { cwd: ROOT }); if (!r.success) throw new Error('git'); return new TextDecoder().decode(r.stdout).trim(); };
    return git('status', '--porcelain') ? '' : git('rev-parse', '--short=12', 'HEAD'); } catch { return ''; }
}
