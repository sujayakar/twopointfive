// A PRIMARY AND A SECONDARY FOR EVERY MAN (GUNSLOTS). The maintainer, 10-18: "also should we model more firearms? and separate primary and secondary slots?"
// Until now you carried two guns (the pistol on 1, the long gun on 2) and a teammate ONE: his card had a single Weapon row and the sidearm on his thigh was
// drawn only. With the switch on every card names two guns — `primary` (a long gun, or none: the pistol alone) and `secondary` (a sidearm) — as two rows where
// the one gun row stood (kit.ts), each a PICK of the table below: what the tile says, and the weapons.ts row it is with a suppressor on and with it off. A
// teammate is staged with both (Guard.second: the gun that is NOT in his hands), and with `dry` he goes to his sidearm when the primary runs dry on a man
// close in front of him — `out` seconds, where standing there changing a magazine is the row's whole reload — and takes the primary up again, reloaded, once
// nobody has been in front of him for `lull` seconds (sideTick). You: 1 is your secondary, 2 your primary, the swap as it was — and which two guns they
// are is your card's (kit.ts kitPlayer builds them from the picks; the issue pair is the pair you were built with).
// Off (the module, the frozen stand, `?nogunslots`): the card of the build before — one gun row — and nobody but you has a second gun. `GUNSLOTS_BREAK=1`
// (tools/qa/gunslots-check.ts): a teammate is staged without his second gun and the card keeps its one gun row.
// The picks ride on the big slot's card (kit.ts BIGSLOT: the shotgun is kit there, never a man's gun): with that switch off the card is the build before's too.
import type { Game, Guard } from './game';
import { Firearm, WEAPONS, type WeaponId } from './weapons';
import { crossToRust, goneWithTick } from './crossing';

export const GUNSLOTS = {
  on: false,
  near: 10,
  /** seconds from the dry click to the sidearm on his man: the long gun let go onto its sling, the pistol out of the holster and up */
  out: 0.7,
  /** seconds with nobody in front of him before the sidearm goes back, and the seconds that takes on top of the primary's own reload */
  lull: 1.6, back: 0.65,   // (as long as your own swap back to the long gun: no quicker hands than yours)
  /** a TEAMMATE's round below the gunshot line (a suppressed pistol, the MP7) is the odd noise yours is to a man of the other side near enough to hear it
   *  (guards.ts hearingCheck). Off: only your own quiet rounds were ever heard — a teammate's reached nobody, however near. */
  heard: true,
  /** a pick with its suppressor off is THE SAME GUN without it (weapons.ts mk18bare, g19bare: its name, magazine, handling and look; the report and flame of
   *  a bare one). Off: the old card's meaning — a carbine with no suppressor was the defenders' AKS-74U row, a pistol with none their sidearm. */
  bare: true,
};
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.GUNSLOTS_BREAK ?? '') === '1'; } catch { return false; } })();
/** GUNSLOTS_BREAK=1 (the check's break path): the card keeps its one gun row and nobody is handed a second gun */
export const gunsBroken = (): boolean => BREAK;
export const GUNSLOTS_STATS = { staged: 0, out: 0, back: 0 };

export type GunSlot = 'primary' | 'secondary';
/** One thing a gun row offers: the rows of weapons.ts it is with a suppressor fitted (`can`) and without (`bare`) — null where it cannot be had that way
 *  (the card's Suppressor row then changes nothing on it). */
export interface GunPick { id: string; slot: GunSlot; can: WeaponId | null; bare: WeaponId | null; /** the row `bare` was before the gun had one of its own (GUNSLOTS.bare off) */ bareWas?: WeaponId }
/** (plain words only: nothing of another game module is read as this loads) */
const PICKS: GunPick[] = [
  { id: 'mk18', slot: 'primary', can: 'fn2000', bare: 'mk18bare', bareWas: 'aks74u' },   // (the issue carbine — `bareWas`: the two rows a teammate's Weapon + Suppressor rows had always meant)
  { id: 'mk12', slot: 'primary', can: 'mk12', bare: null },   // reach
  { id: 'mp7', slot: 'primary', can: 'mp7', bare: null },   // quiet and quick
  { id: 'mk46', slot: 'primary', can: null, bare: 'mk46' },   // volume
  { id: 'm1014', slot: 'primary', can: null, bare: 'm1014' },   // the room
  { id: 'g19', slot: 'secondary', can: 'fiveseven', bare: 'g19bare', bareWas: 'guard9mm' },
  { id: 'hk45', slot: 'secondary', can: 'hk45', bare: null },   // the quietest
  { id: 'g18', slot: 'secondary', can: null, bare: 'g18' },   // a fight in the holster
];
/** no long gun: the sidearm is the gun in his hands */
export const NO_PRIMARY = 'none';
export const gunPick = (id: unknown): GunPick | null => PICKS.find(p => p.id === id) ?? null;
/** what a slot's row steps through, in order (a primary may be left at home) — only picks with a row this build may construct */
export function gunPicks(slot: GunSlot): string[] { const ids = PICKS.filter(p => p.slot === slot && [p.can, p.bare].some(w => w && WEAPONS[w].enabled)).map(p => p.id); return slot === 'primary' ? [...ids, NO_PRIMARY] : ids; }
/** the weapons.ts row a pick is on a card with / without the suppressor (the other one where it cannot be had that way, or this build does not hand that row out) */
export function pickRow(id: unknown, suppressed: boolean): WeaponId | null { const p = gunPick(id); if (!p) return null; const bare = GUNSLOTS.bare ? p.bare : p.bareWas ?? p.bare; return (suppressed ? [p.can, bare] : [bare, p.can]).find(w => w && WEAPONS[w].enabled) ?? null; }
/** the pick's name on its tile: its row's short name, as the card would hand it to him */
export function pickWord(id: unknown, suppressed = true): string { if (id === NO_PRIMARY) return 'none'; const w = pickRow(id, suppressed); return w ? WEAPONS[w].short : String(id); }
/** which of the card's gun drawings stands for a pick (ui/kitIcons.ts), by its row's class — null: none carried */
export function pickIcon(id: unknown): 'rifle' | 'pistol' | 'shotgun' | null { const w = pickRow(id, true) ?? pickRow(id, false); if (!w) return null; const c = WEAPONS[w].cls; return c === 'pistol' ? 'pistol' : c === 'shotgun' ? 'shotgun' : 'rifle'; }

// ------------------------------------------------------------ a teammate's second gun
/** kit.ts kitStage: he is handed the gun that is not in his hands (null row: none — the pistol alone is his whole kit) */
export function sideStage(gd: Guard, row: WeaponId | null) {
  gd.second = null; if (!GUNSLOTS.on || BREAK || !row || !WEAPONS[row].enabled || row === gd.weapon.def.id) return;
  gd.second = { gun: new Firearm(WEAPONS[row]), id: row, ahead: WEAPONS[row].muzzleAhead, out: false, t: -100, lock: -100, shots: 0 };
  GUNSLOTS_STATS.staged++;
}
/** DRY ON A MAN (roomfight.ts, where his trigger found nothing to fire): the sidearm comes out instead of the reload — true, and no round
 *  goes for `out` seconds (sideLocked). False: he has none, it is out already or empty, his hands hold the big slot's shotgun, or his man is past `near`. */
export function sideOut(g: Game, gd: Guard, dist: number): boolean { return crossToRust('sideOut', undefined, [g, gd, dist]) as any; }
/** his hands are between guns — or off his gun for a two-handed job, until it is back up (workstow.ts, Guard.workStow: never set with that switch off): no round
 *  (asked by every trigger a teammate has: roomfight.ts, coverfire.ts, the fight AI's fire site in guards.ts) */
export const sideLocked = (g: Game, gd: Guard): boolean => goneWithTick('sideLocked') as any;
/** the long gun he let go of hangs down his front (combat.ts guardProps draws it): its rig, while the sidearm is out */
export function sideHung(gd: Guard): 'carbine' | 'sc20k' | null { const s = gd.second; if (!s?.out || gd.state === 'dead') return null; const r = s.gun.def.rig; return r === 'carbine' || r === 'sc20k' ? r : null; }
