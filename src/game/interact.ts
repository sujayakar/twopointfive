// The Space selector — one key over the interaction list (player.ts buildInteractables: every verb its own row, the highest-priority MAX_NEAR of what is in
// reach): a TAP (up again inside TAP_SECS without having scrolled) performs the highlighted row; HELD past that the list OPENS — the panel shows it, a cancel
// row appears at its foot, his feet stop, the wheel and ↑ / ↓ move the highlight — and LETTING GO performs whatever is highlighted then (cancel: nothing);
// Esc while it is open backs out. ↑ / ↓ move the highlight at any time; the wheel is his walking pace whenever Space is not down (a hold included). The
// sustained verbs — ease a leaf open, pick a lock, the choke, the pistol-whip, the drive — start on their row like any other and then run on their own clock:
// a fresh Space press, Esc, or a movement key pressed INTO them (one already down when they began does not count until it has come up) stops them where they
// are, exactly as letting go of the held key used to. What is performed goes out as Game.act for this one frame; game.ts updateDoors, player.ts updateHold and
// mission.ts updateMission act on it where they used to read the key, so the frame's order of business is unchanged.
import type { Game } from './game';
import { pickHover, easeOffHold, holdKilling, stopPicking, stopWand } from './player';
import { cancelDrive } from './mission';
import { stopCoding } from './devices';
import './samstun';
import './lightswitch';
import { HANDRING } from './handring';
import './close';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

/** a press up again inside this (without having scrolled the list) is a tap; held past it the list opens */
export const TAP_SECS = 0.25;

/** Which sustained verb is running right now, as an identity ('' = none): the leaf he is easing, the lock he is picking, the choke / the whip chosen (its
 *  commit beat) or under way, the drive coming out of the rack. While one runs its row is the only row and nothing else starts (buildInteractables). */
export function runningVerb(g: Game): string {
  const pl = g.player, H = pl.holding;
  if (pl.easing) return `ease:${pl.easing.def.name}`;
  if (pl.picking) return `pick:${pl.picking.def.name}`;
  if (pl.wanding) return `wand:${pl.wanding.door.def.name}`;   // the optical wand under a door (wand.ts)
  if (pl.coding) return `code:${pl.coding.def.name}`;   // a keypad's code going in (devices.ts updateCoding)
  if (H && (H.windT >= 0 || H.phase === 'choke' || H.phase === 'whip' || holdKilling(H))) return `hold:${H.g.callsign}`;   // (the knife in the hold — close.ts CLOSE.heldClicks — is the running verb too: no row is on offer under it)
  if (g.mission.driveT >= 0) return 'drive';
  if (pl.breakerMove) return 'breaker';   // his hand on the mains lever (handwork.ts) — a half second COMMITTED like a kick: nothing in cancelSustained stops it, and no other row is on offer under it
  if (pl.charging) return `charge:${pl.charging.door.def.name}`;   // a breaching charge going on a door by hand (doortools.ts samTick)
  if (g.wallCharge?.setting) return 'charge:wall';   // a breaching charge going on a wall by hand (wallcharge.ts)
  return '';
}
/** Stop the running sustained verb where it is — what letting go of the held key used to do, now on a Space press, Esc, or a step: the leaf stays where it got
 *  to (its closer wakes a few seconds on, as after a crack let go), the pick comes off the tools with the progress kept on the door a moment, the man in the
 *  arm drags a breath / gets the pistol back at his head (rattled for it), the drive is let go of. True if there was anything to stop. */
export function cancelSustained(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('cancelSustained', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, H = pl.holding;
  if (pl.easing) { pl.easing = null; return true; }
  if (pl.picking) { stopPicking(g); return true; }
  if (pl.wanding) { stopWand(g); return true; }
  if (pl.coding) { stopCoding(g); return true; }
  if (H && holdKilling(H)) return false;   // the knife in the hold is COMMITTED, like the hand on the breaker's lever: no tap, step or Esc takes a blade back (player.ts killOut ends it)
  if (H && (H.windT >= 0 || H.phase === 'choke' || H.phase === 'whip')) { easeOffHold(g); return true; }
  if (g.mission.driveT >= 0) { cancelDrive(g); return true; }
  if (g.doorTools?.cancelSam()) return true;
  if (g.wallCharge?.cancel()) return true;
  return false;
}
/** the digits his ring has a wedge on, a bit each (bit 1 the first): what `view.hand` draws, less every word of it — all the game reads of the ring's
 *  picture (cmdring.ts ringWant `lone` and `shapes`) */
export function handRingKeys(g: Game): number { return goneWithTick('handRingKeys') as any; }
/** Esc, ahead of the pause menu (main.ts asks this first; the headless harness's Escape lands in updateInteract): the open selector backs out — that press is
 *  spent — else a running sustained verb stops. True = it was used here (no menu). */
export function escapeInteract(g: Game): boolean {
  const pl = g.player, S = pl.sel;
  if (S.open || S.downT >= 0) { closeSelector(g, true); return true; }
  return cancelSustained(g);
}
/** his hands are on a choice or a verb: a list open or a press in hand — but not a ring that merely STANDS (handring.ts: he walks, saves, takes a wall,
 *  puts his gun away and gives the squad an order under it) — or a sustained verb running */
export function handsBusy(g: Game): boolean { const S = g.player.sel; return (S.open && !S.ring?.free) || S.downT >= 0 || !!runningVerb(g); }
/** is there anything here the back key would back out of — the open list or a press in hand, a running sustained verb (asked, nothing touched) */
export function backableInteract(g: Game): boolean { const S = g.player.sel; return S.open || S.downT >= 0 || !!runningVerb(g); }
/** the list shut (the cancel row goes, a highlight left on it returns to the top) — the press itself untouched: shutList alone when the rows empty under an open list, closeSelector when the press is over */
function shutList(g: Game) {
  const pl = g.player, ring = !!pl.sel.ring; pl.sel.open = false; pl.sel.ring = null;
  if (ring && !g.puppet && HANDRING.on && pl.nearSel) { pl.nearSel = 0; pickHover(g); }   // (handring.ts: a wedge lit and not done is not left as what a tap does: the top row is)
  if (g.nearby.some(r => r.kind === 'cancel')) { const hv = g.hover; g.nearby = g.nearby.filter(r => r.kind !== 'cancel'); if (hv?.kind === 'cancel') pl.nearSel = 0; pickHover(g); }
}
/** shut the list AND end the press (`spent`: it must also come up before Space arms again — Esc, a teleport) */
function closeSelector(g: Game, spent: boolean) {
  const pl = g.player, S = pl.sel;
  shutList(g); S.downT = -1; S.moved = false; if (spent) pl.fHeld = true;
}

