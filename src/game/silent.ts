// SILENT OR LOUD — the squad's posture, on Sam's word.
//
// The play note: "some way of telling the squad to be silent or not. silent implies walking quietly, no flashlight, etc." One order on the squad's menu
// (orders.ts: X, then 5 — on the floor's menu and on a man's; never on a door's, where 5 is the door tools) puts the WHOLE squad in silent, whoever the menu
// was addressed to, and the row that stands in its place while they are silent takes them out again. It stays until he says otherwise. LOUD is the team
// exactly as it was: every rule here returns on its first line with the squad loud, so with the order never given nothing in the game changes.
//
// What a silent teammate does differently — each line has its switch, off = as loud:
//   pace     his legs are asked for no more than SILENT.pace (a walk, a touch over Sam's own upright 1.55 so a man told to fall in keeps up with him)
//            — guards.ts teamPace, so the order runner, the door tools and the fight AI's chase all take it. It holds with rounds going off too:
//            nothing ends the order but his word (`hotPace` on = the cap lifts for a man loud has reached since the order).
//   step     a walking footfall is filed (and mixed) at SILENT.step in place of the walk's 0.33: heard at about 3.6 m where the walk carries 5.7 — and
//            Hard's defenders, who listen for the team's boots (pressure.ts teamBoots), hear his at that level. His own skill, not Sam's — Sam's
//            crouched step is 0.125. "only the player is truly amazing" at it.
//   lights   weapon lights and torches stay off (guards.ts attachFlashlight); the goggles and the IR are nvg.ts's and irlaser.ts's, as they were — a man whose
//            light is off in a dim place brings his goggles down by that module's own rule.
//   hush     what he would have called out goes over the net instead (guards.ts say: the radio bubble, the log's '(radio)') — a radio line in every
//            sense: like any man's, it is not stepped on by his own next bark for the few seconds the barks leave a radio line alone.
//   doors    a leaf is eased, not shouldered: his contact is a quiet one (doors.ts DoorContact.quiet: the creak at 0.12 for 0.3) and the door tools' shove
//            is a creak, not the bash. (The leaf swings as fast as ever: a slow one shuts on the men behind it, and the shove files no noise either way.)
//   kicks    no boot by himself: an entry that finds its door locked picks it (`picks`), or says so and holds on it. A door NO pick opens — wedged from
//            inside (digin.ts: on a squads level the defenders wedge theirs on Standard too), a keypad's — is booted once the squad is COMPROMISED
//            (`bootWedged` — compromised at any point since the order, not only while the HUD's word lasts): the quiet is gone already and nothing quiet opens it. The posture stands (lights, pace, cans, the fire hold).
//            Not compromised, the lead says what it is and what would open it ('… say loud and we boot it.') and the order ends on the door.
//   cans     no stun canister by himself: the threshold's first can and its blind-pocket cans stay in the kit, on the word too (the pair's word reads
//            'go · step in · no bang' while they are silent).
//   fire     held on a man who has not noticed the team (not alert, not searching, awareness under `noticed`): the gun comes onto him and stays there,
//            and no round goes — the FN2000's report is modelled loud, suppressor or not (weapons.ts; `quietGun` is the one place that asks, for a
//            loadout whose kit says otherwise: a quiet gun is not held). A man who has noticed is shot as ever, and a
//            silent man loud has reached since the order (gunfire, a bang, a kick in his ears: roomfight.ts's hot clock) shoots whoever a loud one would.
//   glass    … and THROUGH STANDING GLASS the bar is higher (the maintainer, 10-08, on a teammate shooting a man through the conference partition: "it should
//            depend if they're loud or not. accept if loud, hold fire if they're trying to stay quiet"): a round that would have to break a pane on its way is
//            held until the man behind it has RAISED THE ALARM (alert, or searching after one) — not merely looking their way (the `noticed` bar lets go at a
//            suspicious man half sure of a shape; through a window that shot wakes the floor for a man who may yet look away). A quiet gun is held too: the
//            pane going is heard at thirty metres whatever the report. Loud in his ears since the order, or a captor at his last warning: no hold, as above.
//            While he holds for the glass the man is not his to stay on: anybody else he may shoot takes his muzzle, and a man who
//            has raised the alarm before any man he is only holding on (roomfight.ts GLASS_LOOK: the scoring in tickOp).
// SILENT NEVER COSTS THE HOSTAGE: with a captor at his last warning (rescue.ts lastWarning: 'about to', his gun on one of them, one shot) no round and no
// boot is held, and the door tools' own last-warning can goes in.
// THE LOUD TOOLS stay on the menu, marked LOUD while the squad is silent (LOUD_ROWS: bang and clear, the charge, bang then step in): picking one does it
// and takes the squad out of silent with one line — a can or a charge is his word for loud. So does a bang and clear started any other way (a
// scenario's F: DoorTools.order).
// COMPROMISED: the first time loud reaches a silent man after the order, one of them says so over the net and the HUD's word says it while it lasts.
// Who a man may shoot widens; the pace, the lights, the cans and the boots stay as ordered until Sam says loud. (Noise from before the order is not it.)
//
// Scope: the squad under Sam's orders (SquadOrders.active — never a showcase's cast, never the tour), every rule a return on its first line otherwise.
//
// `SILENT.start` is the posture a freshly staged squad stands up in (false = loud) — the loadout screen's row.
// State is per Game in a WeakMap (as nvg.ts keeps its own): nothing on the Game, nothing in the digest, nothing in a checkpoint (none is taken with a unit on
// the floor). Probes: tools/qa/silent-probes.ts (`probes.ts silent`) — the QA harness stands the game up with SILENT.on false (src/host/standup.ts), so every
// older menu probe sees the menu it was written against; that family turns it on.

import { viewOf } from './seats';
import { SideMap, SideSet } from '../math/side';
import type { Game, Guard } from './game';
import './frag';
import type { Door } from './doors';
import './sneak';
import { unseenDark, unseenHush, unseenEase } from './unseen';
import './glasspen';
import './holdfire';
import './guards';
import './pairorders';
import './spotted';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

export const SILENT = {
  /** THE switch: off = no rows on the menu, no word on the HUD, the team as it was whatever the state says */
  on: true,
  /** the posture a freshly staged squad stands up in: true = silent (the loadout screen's row) · false = loud, as ever */
  start: false,
  /** his legs, silent (m/s; 0 = no cap) — and whether the cap lifts for a man loud has reached since the order */
  pace: 1.6, hotPace: false,
  /** a walking footfall's hearing level, silent (the walk's is 0.33, a run's 0.8; Sam: walk 0.26, crouched 0.125; 0 = as loud) */
  step: 0.2,
  /** weapon lights and torches held off */
  lights: true,
  /** his lines over the net, not called out */
  hush: true,
  /** doors eased: quiet contacts, and the door tools' shove without the bash */
  doors: true,
  /** no boot by himself (a locked door: said, and held) */
  kicks: true,
  /** … and where the boot is held, the lead PICKS the lock (doors.ts pick: the player's own, clicks and time) and the order goes on as at an unlocked door.
   *  Off: 'locked. no boot while we are silent.' and the order ends on the door */
  picks: true,
  /** … and a door no pick opens (wedged from inside, a keypad's) is BOOTED by a silent squad that has been compromised since the order — they already know,
   *  and it is the only way through; the posture stands. Never compromised: no boot, and the line names the fix. Off: held like any boot, whatever has
   *  reached them ('locked. no boot while we are silent.') */
  bootWedged: true,
  /** no stun canister by himself */
  cans: true,
  /** fire held on a man who has not noticed the team — and the awareness at which he has */
  holdFire: true, noticed: 0.5,
  /** … and a round that would go THROUGH STANDING GLASS is held until the man behind it is alert or searching (a quiet gun's too: the pane is the noise).
   *  Off = glass is nothing to the hold: a man who has noticed is shot through it, and a quiet gun shoots through it at anybody */
  glass: true,
  /** picking a loud tool's row takes the squad out of silent */
  toolsLoud: true,
  /** told to stack, on his mark, his round held on a man by `holdFire` above: he is STACKED ('set.', the row's 'stacked') with his gun still on the man, and
   *  no hand goes to the handle while it is. Off (the module, the frozen stand; on in the page, ?nosilentset): he is 'stacking' until his gun is the door's */
  stackSet: false,
};
/** the rows that are loud whatever the posture (doortools.ts ids): marked on the menu while silent, and given, they end it */
export const LOUD_ROWS = ['bangClear', 'bigClear', 'charge', 'thrBang', 'fragClear', 'wallCharge', 'fragThere', 'stunThere', 'shellThere'];   // ('shellThere': a teammate's 40 mm on order — squad40.ts) ('wallCharge': the charge's twin on a wall — wallcharge.ts)
type By = 'order' | 'start' | 'api' | 'tool';
interface State { cast: Guard[]; silent: boolean; since: number; hot: boolean; /** loud has reached them at some point since the order (it does not lapse as `hot` does) */ was?: boolean; booted?: WeakSet<Door>; }
const states = new SideMap<Game, State>('silent.states');
/** what changed it, newest last (QA; the panel's summary) */
export const SILENT_LOG: { t: number; to: 'silent' | 'loud'; by: By }[] = [];
export const SILENT_STATS = { silent: 0, loud: 0, locked: 0, picked: 0, booted: 0, held: 0, heldGlass: 0, compromised: 0, tools: 0 };

const mate = (gd: Guard) => gd.team === 'operative' && !!gd.unit;   // (the roster's own test — orders.ts squad(), guards.ts teamPaced — for one man, with no list built)
const castOf = (g: Game) => g.orders.squad();

// ------------------------------------------------------------ the state
/** is the squad silent right now (false with the switch off, with nobody ever told, with no squad) */
export function squadSilent(g: Game): boolean { if (!SILENT.on) return false; const s = states.get(g); return !!s && s.silent && g.orders.active; }
/** put the squad in silent / take it out (the orders' issue; the panel; a loadout screen staging a run) */
export function setSilent(g: Game, on: boolean, by: By = 'api') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('setSilent', undefined, [g, on, by]); if (__r !== RUST_PASS) return __r as any; } }
  let s = states.get(g); if (!s) states.set(g, s = { cast: castOf(g), silent: false, since: g.time, hot: false });
  if (s.silent === on) return;
  s.silent = on; s.since = g.time; s.hot = false; s.was = false; s.booted = undefined; SILENT_STATS[on ? 'silent' : 'loud']++;
  SILENT_LOG.push({ t: g.time, to: on ? 'silent' : 'loud', by }); if (SILENT_LOG.length > 100) SILENT_LOG.splice(0, 50);
}
/** this teammate is under the silent posture this frame */
export function underSilence(g: Game, gd: Guard): boolean { return squadSilent(viewOf(g, gd)) && mate(gd); }   // (his seat's squad: seats.ts)

// ------------------------------------------------------------ the rules (each a return on its first line while the squad is loud)
/** the speed his legs may be asked for */
export function silentPace(g: Game, gd: Guard, speed: number): number { return crossToRust('silentPace', undefined, [g, gd, speed]) as any; }
/** a hostage's life is on it: the captor of a live rescue at his last warning (rescue.ts — asked of the room's doors, as the door tools ask it) */
export function urgent(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('urgent', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } return g.doors.list.some(d => g.rescue.lastWarning(d)); }
/** the two above, for spotted.ts's own fire rule beside this one */
export const silentUrgent = urgent;
/** a walking footfall's base level — null: his usual */
export function silentStep(g: Game, gd: Guard): number | null { return crossToRust('silentStep', undefined, [g, gd]) as any; }
/** his weapon light / torch is held off */
export function silentDark(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentDark', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } return underSilence(g, gd) ? SILENT.lights : unseenDark(g, gd); }   // (… or unseen.ts: the loud squad sneaking with Sam, his own light off)
/** his line goes over the net */
export function silentHush(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentHush', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } return underSilence(g, gd) ? SILENT.hush : unseenHush(g, gd); }   // (under the order, the order's own rule; else unseen.ts's)
/** he eases a door */
export function silentEase(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentEase', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } return underSilence(g, gd) ? SILENT.doors : unseenEase(g, gd); }
/** the boot is held back (the door tools ask at a locked door) */
export function silentNoBoot(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentNoBoot', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  if (!SILENT.kicks || !squadSilent(g) || urgent(g)) return false;
  SILENT_STATS.locked++; return true;
}
/** … and the lock is worked instead (the door tools ask before the boot's own question, and every frame the pick is on: the squad told loud, a loud tool
 *  picked, a captor at his last warning = false, and the boot takes over) */
export function silentPick(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentPick', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } return SILENT.picks && SILENT.kicks && squadSilent(g) && !urgent(g); }
/** A door NO pick opens (the door tools' own test: a wedge kicked under it from inside, a keypad's bolt) in front of a silent squad that HAS BEEN
 *  COMPROMISED since the order — at any point: the men behind a door do not forget in the 45 s the HUD's word lasts, and a boot that went by the last loud
 *  thing would stall a squad that fought its way to the door a minute ago. The boot goes — asked before silentNoBoot. 'first': nobody has said so at this
 *  door since the order · 'again': somebody has. null: the boot is not let go here (never compromised since the order, the switch off, boots not held at
 *  all, or a captor at his last warning — then it is the ordinary boot, with its ordinary line) */
export function silentBoot(g: Game, d: Door): 'first' | 'again' | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('silentBoot', undefined, [g, d]); if (__r !== RUST_PASS) return __r as any; } }
  const s = states.get(g); if (!SILENT.bootWedged || !SILENT.kicks || !squadSilent(g) || !s?.was || urgent(g)) return null;
  const said = (s.booted ??= new SideSet()); if (said.has(d)) return 'again';
  said.add(d); SILENT_STATS.booted++; return 'first';
}
/** the canisters stay in the kit */
export function silentNoCans(g: Game): boolean { return SILENT.cans && squadSilent(g); }
/** IS HIS GUN QUIET — the one place that asks (a loadout's kit plugs in here). Today by the weapon's own report: under the gunshot level and not flagged loud,
 *  which the team's FN2000 is not, suppressor or no (weapons.ts: its report is level 1). A man with a quiet gun has no round held. */
export function quietGun(gd: Guard): boolean { return goneWithTick('quietGun') as any; }
export function silentHoldsRound(g: Game, gd: Guard, h: Guard): boolean { return crossToRust('silentHoldsRound', undefined, [g, gd, h]) as any; }
/** THE MAN THE ROOM FIGHT HAS HIM ON IS ONE THE SILENT ORDER WILL NOT LET HIM SHOOT (orders.ts asks, for a man on his stack mark whose gun is not on his
 *  door): a silent teammate with a living man as his target whom the fire hold above holds. The hold's own conditions asked afresh, nothing counted, so it
 *  stands through a change of man. Only that hold: a round held for standing glass alone (a man past `noticed` behind a pane) or by another module's rule
 *  (spotted.ts, the pair's 'watch him') is not asked. Never a loud man (holdfire.ts holdFireHoldsTarget is his). False with `stackSet` off. */
export function silentHoldsTarget(g: Game, gd: Guard): boolean { return crossToRust('silentHoldsTarget', undefined, [g, gd]) as any; }
/** a loud tool is being used: a silent squad is loud from here, with one line (the menu's loud rows — LOUD_ROWS — and the door tools' own bang and clear) */
export function silentTool(g: Game, loud: boolean) {
  if (!loud || !SILENT.toolsLoud || !squadSilent(g)) return;
  setSilent(g, false, 'tool'); SILENT_STATS.tools++; g.msg('squad: going loud on that order.');
}
/** is the silent squad compromised right now (loud in a teammate's ears, and the order still standing) */
export function silentCompromised(g: Game): boolean { return squadSilent(g) && !!states.get(g)?.hot; }

const seats = new SideMap<Game, { on: boolean; goDigit: boolean }>('silent.seats');


export function silentSummary(g: Game): string {
  if (!SILENT.on) return 'off';
  const s = states.get(g), n = g.orders.men().length;
  if (!n) return `no squad · a new one starts ${SILENT.start ? 'silent' : 'loud'}`;
  return `${squadSilent(g) ? (s?.hot ? 'SILENT · compromised' : 'SILENT') : 'loud'}${s && s.since > 0 ? ` for ${(g.time - s.since).toFixed(0)} s` : ''} · ${n} men · a new squad starts ${SILENT.start ? 'silent' : 'loud'}`;
}
