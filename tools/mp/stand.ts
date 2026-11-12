// STANDING A NETWORKED GAME (bun): the game of rust/game_face, as a game server and a scripted client hold it.
//   freshStart(setup)   the start of a game for an address: its level's file, its seats, its die, its difficulty
//   standStart(start)   THE game a server or a client plays, stood from the start's bytes as every page stands it. As many as the module has room for.
//   buildName()         the build this checkout is
import { StartFlag } from '../../src/draw/bits';
import { appendFileSync, existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { WireCmd, Outcome } from '../../src/net/wire.ts';
import { Face } from '../../src/draw/face.ts';
import type { AheadView } from '../../src/view/read.ts';
import { startWhy, type NetStart } from '../../src/net/start.ts';
import { NetGame } from '../../src/net/modgame.ts';
import { wordTick } from '../../src/net/words.ts';
import { toolDesk, toolModule } from '../desk.ts';
import { characterFiles } from '../blitz/character.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what whoever makes a game says of it: the page's address for the mission, the die, and what the host's machine keeps (its one text; none said: nothing, and the
 *  address itself says what it says) */
export interface StartSetup { search: string; seed: number; kept?: string }
/** a game is of a mission: the game says whether the words name one */
const namesNone = (s: NetStart): string => toolDesk().kind(s.words) ? '' : `the words ${s.words.slice(0, 60)} name no mission`;
export function freshStart(setup: StartSetup): NetStart {
  const q = new URLSearchParams(setup.search), named = q.get('levelFile') ?? 'office.json';
  const start: NetStart = { file: named, words: setup.search, seed: setup.seed, seats: Number(q.get('seats') ?? 1), kept: setup.kept ?? '' };
  const why = startWhy(start) || namesNone(start); if (why) throw new Error(`the setup: ${why}`);
  return start;
}

/** a networked game as a machine with no page holds it */
export interface Stood {
  readonly seats: number; readonly ticks: number;
  /** one tick on the seats' records, the state word folded after it */
  step(cmds: WireCmd[]): void;
  /** the state word after the last tick (src/net/words.ts) */
  word(): string;
  /** how the mission ended, by the game's own verdict: null while it is being played */
  ended(): Outcome | null;
  /** which seat this machine shows: nothing a tick reads */
  show(seat: number): void;
  /** where a seat's man would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: WireCmd[]): AheadView;
}

/** `bent`: a check's. This machine's word of the part so named is not the game's, as if that part of its game were another */
export async function standStart(start: NetStart, bent = ''): Promise<Stood> {
  const why = startWhy(start) || namesNone(start); if (why) throw new Error(`no networked game can be stood from this start: ${why}`);
  const level = resolve(ROOT, 'levels', start.file); if (!existsSync(level)) throw new Error(`${start.file} is not a level of this build`);
  const face = new Face(new WebAssembly.Instance(toolModule()).exports), [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
  const h = face.start(new Uint8Array(readFileSync(level)), start.words, mannequin, mannequinBin, cast, castLook, start.kept, start.seed, start.seats, StartFlag.Dress);
  face.reserve(64 * 1048576);
  if (bent) { const at = face.stateNames().indexOf(bent), words = face.stateWords.bind(face); if (at < 0) throw new Error(`the game has no part ${bent}`);
    face.stateWords = h => { const w = words(h); w[at] ^= 1; return w; }; }
  const game = new NetGame(face, h), keep = wordsKept();
  game.clear();
  return {
    seats: start.seats, get ticks() { return game.ticks; },
    step(cmds) { game.step(cmds); game.clear(); if (keep && wordTick(game.ticks - 1)) keep(game.ticks, game.whole(true)); },
    word: () => game.word(), ended: () => game.ended(),
    show: seat => face.showSeat(h, seat),
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
