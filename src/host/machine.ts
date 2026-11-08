// WHAT THIS MACHINE KEEPS OF HOW IT PLAYS, as the commands a game is handed: the difficulty, its levers and the men's cards before it is made, the sandbox row and the autosave switch once it stands. The
// address's words win over what is kept, for the visit.
import type { Commands } from '../draw/commands';
import { KIT_KEPT } from '../view/desk';
import { autosaveKept, sandboxKept } from './sandbox';

/** the levers' keys and the address's word for each (game/difficulty.ts) */
const LEVERS: readonly (readonly [string, string])[] = [['fog', 'fog'], ['ai', 'enemyai'], ['kit', 'enemykit'], ['told', 'told']];
const kept = (key: string): string | null => { try { return localStorage.getItem(key); } catch { return null; } };
function json(key: string): Record<string, unknown> { try { const o: unknown = JSON.parse(kept(key) ?? 'null'); return typeof o === 'object' && o !== null ? o as Record<string, unknown> : {}; } catch { return {}; } }

/** before a game is made: what is kept, then the address's word over it (a word that is no rung is refused by the rules, and what was kept stands). A tutorial is
 *  taught on the standard rules, whatever this machine keeps */
export function machineBefore(q: URLSearchParams, level: string, C: Commands): void {
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
export function machineAfter(C: Commands): void {
  const s = sandboxKept();
  if (s.god) C.god(1);
  if (s.noAi) C.ai(0);
  if (s.ammo) C.ammo(1);
  if (!autosaveKept()) C.autosave(0);
}
