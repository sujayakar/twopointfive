// The Space selector — one key over the interaction list (player.ts buildInteractables: every verb its own row, the highest-priority MAX_NEAR of what is in
// reach): a TAP (up again inside TAP_SECS without having scrolled) performs the highlighted row; HELD past that the list OPENS — the panel shows it, a cancel
// row appears at its foot, his feet stop, the wheel and ↑ / ↓ move the highlight — and LETTING GO performs whatever is highlighted then (cancel: nothing);
// Esc while it is open backs out. ↑ / ↓ move the highlight at any time; the wheel is his walking pace whenever Space is not down (a hold included). The
// sustained verbs — ease a leaf open, pick a lock, the choke, the pistol-whip, the drive — start on their row like any other and then run on their own clock:
// a fresh Space press, Esc, or a movement key pressed INTO them (one already down when they began does not count until it has come up) stops them where they
// are, exactly as letting go of the held key used to. What is performed goes out as Game.act for this one frame; game.ts updateDoors, player.ts updateHold and
// mission.ts updateMission act on it where they used to read the key, so the frame's order of business is unchanged.
import type { Game, Interactable } from './game';
import { Doors } from './doors';
import { pickHover, easeOffHold, holdKilling, stopPicking, stopWand, thingDist, PRESS_THINGS } from './player';
import { cancelDrive } from './mission';
import { stopCoding } from './devices';
import './samstun';
import { CMDRING, type RingView } from './cmdring';
import './lightswitch';
import { HANDRING, EMPTY_WORD, EMPTY_NEAR_M, EMPTY_SHY_M, EMPTY_WHY } from './handring';
import type { Vec3 } from '../math/vec';
import { whistleReadyIn } from './whistle';
import './close';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

/** every row a door can have (player.ts buildInteractables, the tools' rows with it), and those of them an OPEN leaf can: a mission's frame that
 *  offers none of them has no row for that door at any distance. (One that offers some may still have none for this door — a steel one, a lock it
 *  names no pick for: that is known only in reach, where the door's own rows are built) */
const OPEN_ROWS = ['pull', 'ease', 'charge', 'breach', 'hammer', 'wand'], SHUT_ROWS = [...OPEN_ROWS, 'push', 'pick', 'kick', 'rattle', 'unlock', 'code', 'nocode'];
/** THE THING AN EMPTY TAP IS BESIDE (handring.ts `near`), and why the key did nothing on it — only what the game knows exactly:
 *    - a thing worked by hand (player.ts PRESS_THINGS) in reach whose every row the mission's frame kept back (the marker's `framed`);
 *    - a door in his hand's reach with no row while his hands are not his (a leaf already in his hand);
 *    - a door his sprint alone keeps from him;
 *    - a door a step out of reach, within EMPTY_NEAR_M and in sight (the door markers are the whole level's: never one with a wall between).
 *  In that order, the nearest of each. A door of the last two that the frame has no row at all for (SHUT_ROWS / OPEN_ROWS) says that at once, not
 *  'step closer'; a door locked against him answers nothing while he sprints at it (its own line there is the boot's). Nothing where a living man
 *  in sight is nearer (that tap was for him, and his own line says why not); no door out of reach where another thing out of reach is nearer (a
 *  body, a switch: their reach is not a plain distance, and the tap may have been for them); nothing during a takedown. Null: open floor, or the
 *  rule off */
export function emptyThing(g: Game): { it: Interactable; d: number; why: string } | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('emptyThing', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player; if (!HANDRING.near || pl.takedown) return null;
  const p = pl.char.pos, eye: Vec3 = [p[0], pl.crouch ? 0.9 : 1.3, p[2]];
  // (a level line from his eye to a hand's breadth short of the thing: a shut leaf is no wall of its own)
  const walled = (to: Vec3) => { const dx = to[0] - eye[0], dz = to[2] - eye[2], k = Math.max(0, 1 - EMPTY_SHY_M / Math.max(1e-6, Math.hypot(dx, dz))); return g.col.solidBlocked(eye, [eye[0] + dx * k, eye[1], eye[2] + dz * k]); };
  let best: { it: Interactable; d: number; why: string } | null = null, rank = 3, man = Infinity, other = Infinity;
  for (const it of g.interactables) {
    const d = thingDist(it, p); if (d > EMPTY_NEAR_M) continue;
    if (it.kind === 'guard') { if (d < man && !walled(it.pos)) man = d; continue; }
    if (!PRESS_THINGS.has(it.kind)) continue;
    const D = it.door, r = D ? (d >= Doors.USE_REACH ? 2 : pl.sprinting ? 1 : 0) : it.inReach ? 0 : 3;   // (a door's reach is its distance and his feet: its `inReach` is also the big tool's, from where a stack stands)
    if (r === 3) { other = Math.min(other, d); continue; }
    const why = r === 0 ? (it.framed ? EMPTY_WHY.held : D ? EMPTY_WHY.no : '') : !(D!.isClosed() ? SHUT_ROWS : OPEN_ROWS).some(v => g.rails.offers(v)) ? EMPTY_WHY.held : r === 1 ? EMPTY_WHY.run : EMPTY_WHY.far;
    if (!why) { other = Math.min(other, d); continue; }   // (in reach with no row and no frame to blame)
    if (r > rank || (r === rank && best && d >= best.d)) continue;
    if (r === 2 ? walled(it.pos) && walled(D!.frameCentre) : r === 1 && g.doors.lockedOut(D!, p, pl.keys)) continue;
    best = { it, d, why }; rank = r;
  }
  return best && best.d <= man && (rank === 0 || best.d <= other) ? best : null;
}
/** what a tap that found no row says: 'nothing in reach', or beside a thing its name (the head of its marker's first line) and why */
export function emptyWord(g: Game, T = emptyThing(g)): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('emptyWord', undefined, [g, T]); if (__r !== RUST_PASS) return __r as any; } } return T ? `${T.it.line1.split(' · ')[0] || T.it.kind} · ${T.why}` : EMPTY_WORD; }

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
/** a row the ring has a wedge for: one with something to do */
const wedged = (r: Interactable) => !!r.verb && !r.running && r.kind !== 'cancel';
/** a row's words as the panel has them, less its key — and less what follows a dash, which the ring keeps for under it */
const rowWords = (r: Interactable) => r.line2.replace(/^Space\s+/, '').split(/\s+·\s+/)[0].trim();
/** … and up to that dash: what its wedge says */
const wedgeWords = (r: Interactable) => rowWords(r).split(/\s+[—–-]\s+/)[0];
/** (HANDRING.bare) a row's `off` is the THING's (game.ts Interactable: its marker and its panel line are drawn dim by it), and it says two things: on most
 *  rows that the row cannot be given (no code for the keypad, a charge that would kill him), on these kinds how the thing STANDS (the lights are off, the
 *  mains are thrown) — and there the row is the very one that can be given. Their wedge is not drawn faint for it. */
const STATE_OFF: readonly string[] = ['switch', 'breaker'];
/** THE WHISTLE'S WEDGE (handring.ts): the ring's last one, wherever the rows leave it free — no row of the list (nothing that reads the rows or the
 *  highlight meets it), so never what a tap does */
const WHISTLE_WEDGE = 'whistle', WHISTLE_CAPTION = 'whistle — one guard will come to look';
const whistleWedge = (g: Game) => { if (!ringStays(g) || g.player.down || g.player.takedown || g.rails?.locks('whistle')) return false;   // (rails.ts: a tutorial mission that has not taught the whistle has no wedge for it)
  let n = 0; for (const r of g.nearby) if (r.kind !== 'cancel') n++; return n < CMDRING.wedges; };
const ringStays = (g: Game) => !g.puppet && HANDRING.on;
/** What Sam's own ring shows this frame (null: none — cmdring.ts CMDRING.hands): where it is, how much larger than its px (ringZoom), the lit wedge (0: the
 *  highlight is on a row with no wedge), and a wedge a row on a digit — its words up to a dash, greyed where the row cannot be done; a row with no verb (a
 *  status line) has none. Under it the thing's name and the highlighted row's words in full. ui/cmdring.ts draws it; the simulation never reads it (ringKeys is what it reads). */
export function handRingView(g: Game): RingView | null {
  const pl = g.player, S = pl.sel, R = S.ring; if (!R || !S.open) return null;
  const wedges: RingView['wedges'] = [];
  g.nearby.slice(0, CMDRING.wedges).forEach((r, i) => { if (wedged(r)) wedges.push({ key: i + 1, text: wedgeWords(r), off: (r.off && !(HANDRING.bare && STATE_OFF.includes(r.kind))) || !r.inReach, opens: false }); });
  if (whistleWedge(g)) wedges.push({ key: CMDRING.wedges, text: WHISTLE_WEDGE, off: whistleReadyIn(g) > 0, opens: false });
  const hv = g.hover, lit = R.whistle ? CMDRING.wedges : hv && wedged(hv) && pl.nearSel < CMDRING.wedges ? pl.nearSel + 1 : 0;
  return { x: R.ax, y: R.ay, zoom: R.z, lit, tone: 'hands', caption: HANDRING.bare ? '' : R.whistle ? WHISTLE_CAPTION : hv ? (wedged(hv) ? `${hv.line1} · ${rowWords(hv)}` : hv.line1) : '', wedges };
}
/** the digits his ring has a wedge on, a bit each (bit 1 the first): what handRingView draws, less every word of it — all the game reads of the ring's
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
  if (ring && ringStays(g) && pl.nearSel) { pl.nearSel = 0; pickHover(g); }   // (handring.ts: a wedge lit and not done is not left as what a tap does: the top row is)
  if (g.nearby.some(r => r.kind === 'cancel')) { const hv = g.hover; g.nearby = g.nearby.filter(r => r.kind !== 'cancel'); if (hv?.kind === 'cancel') pl.nearSel = 0; pickHover(g); }
}
/** shut the list AND end the press (`spent`: it must also come up before Space arms again — Esc, a teleport) */
function closeSelector(g: Game, spent: boolean) {
  const pl = g.player, S = pl.sel;
  shutList(g); S.downT = -1; S.moved = false; if (spent) pl.fHeld = true;
}

