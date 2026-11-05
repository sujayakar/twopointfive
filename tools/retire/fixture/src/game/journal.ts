// A recording from before a key replays with it off, as src/game/journal.ts applyKnobs has it.
import { K } from './k';
export function applyKnobs(t: { K?: Record<string, unknown> }): void {
  for (const key of ['viaList', 'on'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (from before both)
  for (const key of ['extra', 'stay'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (one of these stays)
  if (!('panelOnly' in (t.K ?? {}))) K.panelOnly = false;
  if ((t.K as Partial<typeof K> | undefined)?.snap === undefined) K.snap = false;   // (a recording's own word for the row, read off the recording)
  for (const key of SHARED) if (typeof t.K?.[key] !== 'boolean') K[key] = false;
}
/** the same for a recording read as it came off the disk, untyped: the row's own word is asked of the recording under the tables' names */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function applyOld(k: any): void {
  if ((k.tables.K as Partial<typeof K> | undefined)?.quote === undefined) K.quote = "it's";   // (from before the word)
  if ((k.tables.K as Partial<typeof K> | undefined)?.stay === undefined) K.stay = false;   // (a row that stays: the line stays)
  if ((k.tables.K as Partial<typeof K> | undefined)?.j1 === undefined) { K.j1 = false; K.stay = false; }   // (from before j1: it also puts a row that stays off)
  if ((k.tables.K as { j3?: boolean } | undefined)?.j3 === undefined) { K.j3 = false; K.stay = false; }   // (the same under a type written out: it also puts a row that stays off)
  if ((k.tables.K as { j4?: boolean } | undefined)?.j4 === undefined) K.j4 = K.stay = false;   // (two rows in one chain: the second stays)
  if (!('j5' in (k.tables.K ?? {}))) { K.j5 = false; K.stay = false; }   // (asked with `in`)
  const j2 = (k.tables.K as Partial<typeof K> | undefined)?.j2; if (j2 !== true) K.j2 = false;   // (the word read into a name)
  if (!k.tables.K) { K.on = false; K.extra = false; }   // (from before the table: the whole table is asked for, not a row, and two rows put off in a block)
  if (typeof (k.tables.K as { extra?: boolean } | undefined)?.extra !== 'boolean') K.extra = false;   // (the row's word, asked under a type written out)
  if (!k.tables.K) K.viaList = false; else if ((k.tables.K as { stay?: boolean }).stay === undefined) K.stay = false;   // (the table, then a row that stays)
}
/** the game handed in beside a recording is not a recording: a question asked of it may be a getter's call, and stays where it stood */
class Live { n = 0; get seen(): boolean { this.n++; return true; } }
export function applyLive(g: Live): void { if (g.seen) K.panelOnly = false; }
/** …nor is a thing typed by an interface: the question stays */
interface Seen { seen: boolean }
export function applySeen(g: Seen): void { if (g.seen) K.panelOnly = false; }
/** …nor is a question that asks the game AND the recording a recording's alone */
export function applyMix(g: Seen, k: { tables: Record<string, unknown> }): void { if (g.seen && !k.tables.K) K.panelOnly = false; }
export const LIVE = Live;
/** a list of keys two things read */
const SHARED = ['shared', 'stay'] as const;
export const SHARED_COUNT = SHARED.length;
