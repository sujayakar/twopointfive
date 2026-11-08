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
import { specOfQuery } from '../../src/scene/scenarioList.ts';
import { Drawn, type DrawFace } from '../../src/draw/drawn.ts';
import { Man, Section, Want } from '../../src/draw/sections.ts';
import { characterFiles, pack } from '../blitz/start-pack.ts';
import { faceHere } from '../blitz/face-build.ts';
import { wordsKept, type Stood } from './stood.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what whoever makes a game says of it: the page's address for the mission, the die, and what the host has set before the game is made (its commands' bytes as base64) */
export interface StartSetup { search: string; seed: number; commands?: string }
export function freshStart(setup: StartSetup): NetStart {
  const q = new URLSearchParams(setup.search), named = q.get('levelFile') ?? 'office.json';
  if (!specOfQuery(q)) throw new Error(`the address ${setup.search} names no mission`);
  LOOK.on = lookOfQuery(q);
  const look = lookLevelFile(named), start: NetStart = { file: named, words: setup.search, seed: setup.seed, seats: Number(q.get('seats') ?? 1), commands: setup.commands ?? '' };
  const why = startWhy(start); if (why) throw new Error(`the setup: ${why}`);
  if (existsSync(resolve(ROOT, 'levels', look))) start.file = look;
  return start;
}

interface StartFace extends NetFace, DrawFace { face_start(p: number, n: number, slot: number, seed: number, seats: number, flags: number): number; face_reserve(n: number): void; face_show_seat(h: number, seat: number): void; face_panic_ptr(): number; face_panic_len(): number }
/** `face_start`'s: the rows that dress the picture are up, on a page and so on every machine of a game */
const DRESS = 1;
let MODULE: WebAssembly.Module | null = null;

export async function standStart(start: NetStart): Promise<Stood> {
  const why = startWhy(start); if (why) throw new Error(`no networked game can be stood from this start: ${why}`);
  const level = resolve(ROOT, 'levels', start.file); if (!existsSync(level)) throw new Error(`${start.file} is not a level of this build`);
  if (!MODULE) { const file = faceHere(); if (!file) throw new Error('the game\'s module is not built and the checked-in one is of other sources (bun run tools/blitz/face-build.ts)'); MODULE = new WebAssembly.Module(new Uint8Array(readFileSync(file))); }
  const exports = new WebAssembly.Instance(MODULE, { env: { face_roll: () => { throw new Error('the game asked its host for a roll'); } } }).exports, X = exports as unknown as StartFace, memory = exports.memory as WebAssembly.Memory;
  const bytes = pack([readFileSync(level), new TextEncoder().encode(start.words), ...characterFiles(ROOT), new Uint8Array(0), commandBytes(start)]);
  const p = X.face_alloc(bytes.length) >>> 0; new Uint8Array(memory.buffer, p, bytes.length).set(bytes);
  let h = 0;
  try { h = X.face_start(p, bytes.length, -1, start.seed, start.seats, DRESS) >>> 0; }
  catch (e) { throw new Error(`the game could not be stood from its start: ${new TextDecoder().decode(new Uint8Array(memory.buffer, X.face_panic_ptr() >>> 0, X.face_panic_len() >>> 0)) || (e as Error).message}`); }
  X.face_free(p, bytes.length); X.face_reserve(64 * 1048576);
  const game = new NetGame(X, memory, h), keep = wordsKept();
  game.clear();
  const drawn = new Drawn(X, memory, Want.men);
  return {
    seats: start.seats, warnings: [], get ticks() { return game.ticks; },
    step(cmds: WireCmd[]) { game.step(cmds); game.clear(); if (keep && wordTick(game.ticks - 1)) keep(game.ticks, game.whole(true)); },
    quiet() { /* nothing of the game is kept beside it */ }, begin() { /* it stands */ }, show: seat => X.face_show_seat(h, seat),
    word: () => game.word(), watch: () => game.watch(), ended: () => game.ended(),
    ahead: (seat, cmds) => game.ahead(seat, cmds),
    near(seat) {
      const me = game.ahead(seat, []); drawn.take(h); let d = Infinity;
      for (let i = 0, o = drawn.at[Section.men]; i < drawn.n[Section.men]; i++, o += Man.row) if (drawn.u[o + Man.id] !== me.id) d = Math.min(d, Math.hypot(drawn.f[o + Man.feet] - me.at[0], drawn.f[o + Man.feet + 2] - me.at[2]));
      return d;
    },
  };
}
