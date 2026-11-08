// WHAT THIS MACHINE KEEPS OF HOW IT PLAYS, as the commands a game is handed: the difficulty, its levers and the men's cards before it is made, the sandbox row and the autosave switch once it stands. The
// address's words win over what is kept, for the visit.
import type { ShownRow } from './drawgame';
import type { Commands } from '../draw/commands';
import { KIT_KEPT } from '../view/desk';
import { autosaveKept, sandboxKept } from './sandbox';

/** the levers' keys and the address's word for each (game/difficulty.ts) */
const LEVERS: readonly (readonly [string, string])[] = [['fog', 'fog'], ['ai', 'enemyai'], ['kit', 'enemykit'], ['told', 'told']];
/** this machine keeps difficulty levers of its own */
export const leversKept = (): boolean => Object.keys(json('twopointsix.levers')).length > 0;
/** the levers this machine keeps, each as its address word says it (a word that is no rung is the rules' to refuse) */
export const leversSaid = (): [string, string][] => { const mine = json('twopointsix.levers'); return LEVERS.flatMap(([key, word]) => typeof mine[key] === 'string' ? [[word, mine[key]] as [string, string]] : []); };
const kept = (key: string): string | null => { try { return localStorage.getItem(key); } catch { return null; } };
function json(key: string): Record<string, unknown> { try { const o: unknown = JSON.parse(kept(key) ?? 'null'); return typeof o === 'object' && o !== null ? o as Record<string, unknown> : {}; } catch { return {}; } }

/** before a game is made: what is kept, then the address's word over it (a word that is no rung is refused by the rules, and what was kept stands). A tutorial is
 *  taught on the standard rules, whatever this machine keeps */
/** the rows of the game's numbers this machine keeps, as each row shows them */
const KEPT_ROWS = 'twopointsix.rows', PICKS = 'twopointsix.picks';
export function keepRow(name: string, v: number, def: number): void { const o = json(KEPT_ROWS); if (v === def) delete o[name]; else o[name] = v; try { localStorage.setItem(KEPT_ROWS, JSON.stringify(o)); } catch { /* no store */ } }
/** the panel's picks are the visit's */
export function keepPick(name: string, word: string): void { try { const o = JSON.parse(sessionStorage.getItem(PICKS) ?? '{}') as Record<string, string>; o[name] = word; sessionStorage.setItem(PICKS, JSON.stringify(o)); } catch { /* no store */ } }
/** before a game is made: what the rows kept say, each in its row's range, and the visit's picks. The address's own word for one wins. */
function keptRows(q: URLSearchParams, rows: readonly ShownRow[], C: Commands): void {
  const o = json(KEPT_ROWS);
  for (const r of rows) { const k = o[r.name]; if (r.kept && typeof k === 'number' && Number.isFinite(k) && !q.has(`knob.${r.name}`)) C.knob(r.name, Math.min(r.hi, Math.max(r.lo, k)) / r.scale); }
  try { const p = JSON.parse(sessionStorage.getItem(PICKS) ?? '{}') as Record<string, string>; for (const [name, word] of Object.entries(p)) if (typeof word === 'string' && !q.has(name)) C.act(name, 0, word); } catch { /* no store */ }
}
export function machineBefore(q: URLSearchParams, level: string, C: Commands, rows: readonly ShownRow[] = []): void {
  keptRows(q, rows, C);
  const taught = /^tut-/.test(level), word = q.has('hard') ? (q.get('hard') === '0' ? 'standard' : 'hard') : q.get('difficulty');
  const hard = word === 'hard' || (word !== 'standard' && kept('twopointsix.difficulty') === 'hard' && (!taught || q.has('lessonhard')));
  if (hard) C.difficulty(1);
  if (!taught) {
    const mine = json('twopointsix.levers');
    for (const [key] of LEVERS) if (typeof mine[key] === 'string') C.lever(key, mine[key]);
    for (const [key, said] of LEVERS) { const rung = q.get(said); if (rung) C.lever(key, rung); }
  }
  { const cards = kept(KIT_KEPT); if (cards !== null && !q.has('nokit')) C.kit(cards); }
  { const rung = kept('twopointsix.quietfog'), cams = kept('twopointsix.quietcams'); if (rung !== null) C.quietFog(rung); if (cams === '1' || cams === '0') C.quietCams(cams === '1' ? 1 : 0); }
  if (q.has('digin')) C.digIn(1);
  if (q.has('pressure')) C.pressure(q.get('pressure') !== '0' ? 1 : 0);
  if (q.has('fogcone')) C.fogCone(q.get('fogcone') !== '0' ? 1 : 0);
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
