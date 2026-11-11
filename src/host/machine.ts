// WHAT THIS MACHINE KEEPS OF HOW IT PLAYS: one record before a game is made (the difficulty, its levers, the men's cards, the quiet picks, the panel's kept rows and picks), and the sandbox row and the
// autosave switch once it stands. No word of an address is read here: the game lays the address over the record.
import type { ActionRow, ShownRow } from '../view/read';
import type { Commands } from '../draw/commands';
import { KIT_KEPT } from '../view/desk';
import { autosaveKept, sandboxKept } from './sandbox';

/** the levers' keys and an address's word for each, for the links a co-op game is made by (rust/game_rules/src/pursuit/difficulty.rs) */
const LEVERS: readonly (readonly [string, string])[] = [['fog', 'fog'], ['ai', 'enemyai'], ['kit', 'enemykit'], ['told', 'told']];
/** this machine keeps difficulty levers of its own */
export const leversKept = (): boolean => Object.keys(json('twopointsix.levers')).length > 0;
/** the levers this machine keeps, each as its address word says it (a word that is no rung is the rules' to refuse) */
export const leversSaid = (): [string, string][] => { const mine = json('twopointsix.levers'); return LEVERS.flatMap(([key, word]) => typeof mine[key] === 'string' ? [[word, mine[key]] as [string, string]] : []); };
const kept = (key: string): string | null => { try { return localStorage.getItem(key); } catch { return null; } };
function json(key: string): Record<string, unknown> { try { const o: unknown = JSON.parse(kept(key) ?? 'null'); return typeof o === 'object' && o !== null ? o as Record<string, unknown> : {}; } catch { return {}; } }

/** the rows of the game's numbers this machine keeps, as each row shows them */
const KEPT_ROWS = 'twopointsix.rows', PICKS = 'twopointsix.picks';
export function keepRow(name: string, v: number, def: number): void { const o = json(KEPT_ROWS); if (v === def) delete o[name]; else o[name] = v; try { localStorage.setItem(KEPT_ROWS, JSON.stringify(o)); } catch { /* no store */ } }
/** the panel's picks are the visit's */
export function keepPick(name: string, word: string): void { try { const o = JSON.parse(sessionStorage.getItem(PICKS) ?? '{}') as Record<string, string>; o[name] = word; sessionStorage.setItem(PICKS, JSON.stringify(o)); } catch { /* no store */ } }
const session = (key: string): Record<string, unknown> => { try { const o: unknown = JSON.parse(sessionStorage.getItem(key) ?? 'null'); return typeof o === 'object' && o !== null ? o as Record<string, unknown> : {}; } catch { return {}; } };
const only = <T>(o: Record<string, unknown>, is: (v: unknown) => v is T): Record<string, T> => Object.fromEntries(Object.entries(o).filter((e): e is [string, T] => is(e[1])));
const isWord = (v: unknown): v is string => typeof v === 'string', isNumber = (v: unknown): v is number => typeof v === 'number' && Number.isFinite(v);

/** WHAT THIS MACHINE KEEPS, as the one record a start or the desk is handed (rust/game_start/src/host/kept.rs), which lays it under the address's words. `hosted`: a game hosted from here stands on
 *  this machine's difficulty, its levers and its quiet picks, and both players play the kit as issued and the panel's numbers as they ship */
export function machineKept(hosted = false): string {
  const r: Record<string, unknown> = {}, some = (k: string, o: Record<string, unknown>): void => { if (Object.keys(o).length) r[k] = o; };
  if (kept('twopointsix.difficulty') === 'hard') r.hard = true;
  { const mine = json('twopointsix.levers'); some('levers', Object.fromEntries(LEVERS.flatMap(([key]) => isWord(mine[key]) ? [[key, mine[key]]] : []))); }
  const rung = kept('twopointsix.quietfog'), cams = kept('twopointsix.quietcams');
  if (rung !== null) r.quietFog = rung;
  if (cams === '1' || cams === '0') r.quietCams = cams === '1';
  if (!hosted) {
    const cards = kept(KIT_KEPT); if (cards !== null) r.cards = cards;
    some('rows', only(json(KEPT_ROWS), isNumber));
    some('picks', only(session(PICKS), isWord));
  }
  return Object.keys(r).length ? JSON.stringify(r) : '';
}
/** the game a start stood, as far as this file asks it */
export interface Stands { shown(): readonly ShownRow[]; shownNow(): Float64Array; actions(): readonly ActionRow[]; keptTurn(name: string, value: number): void; keptPick(name: string, word: string): void }
/** the kept rows and picks that stand on the game a start stood: a load or a restart stands a game whose numbers are another moment's, and each of these is turned again. One the address's own word won
 *  over stands at that word, and is left there */
export function machineStood(game: Stands): void {
  const rows = only(json(KEPT_ROWS), isNumber), now = game.shownNow();
  game.shown().forEach((r, i) => { const k = rows[r.name]; if (!r.kept || k === undefined) return; const v = Math.min(r.hi, Math.max(r.lo, k)); if (Math.abs(now[i] - v) <= 1e-9 * Math.max(1, Math.abs(v))) game.keptTurn(r.name, v / r.scale); });
  const picks = only(session(PICKS), isWord);
  for (const a of game.actions()) if (a.kind === 'choice' && picks[a.name] === a.now) game.keptPick(a.name, a.now);
}
/** once it stands */
/** `sandbox`: the sandbox row too (a run's page stands it off: its fight is the set's) */
export function machineAfter(C: Commands, sandbox = true): void {
  const s = sandbox ? sandboxKept() : {};
  if (s.god) C.god(1);
  if (s.noAi) C.ai(0);
  if (s.ammo) C.ammo(1);
  if (!autosaveKept()) C.autosave(0);
}
