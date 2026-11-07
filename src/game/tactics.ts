// Pursuit tactics (STATUS E12 (a) + (b), the maintainer's words: "they all just run straight at sam. instead they should coordinate flanking maneuvers" and
// "if they notice their buddies dead by a door, they shouldn't just blindly charge in past them!"). Three things, all for HUNTING men — a fix but no eyes on
// their man; the fight inside six metres, the standoff and the shot are untouched — all of them off with Game.tune.tactics (then not one call below does
// anything and guards.ts runs the chase it always ran: tools/qa tactics-identity holds that to the byte) and none of them under the tour (quietUtility):
//   (A) THE FLANKING APPROACH (planApproach). Two or more alert men of one side on one fix, all more than six metres off it and none with eyes on the man:
//       ROLES over the nav grid. The PRESSER has the shortest route: he goes up it at the jog and stops SHORT — kneeling at the last corner or jamb that still
//       covers him from the fix (the last blocked→open transition of his line along his path — the line of FIRE, losOpen: a pane is no opening, and along a
//       glazed wall that corner is the door's jamb) — until the flanker is set, anybody gets eyes on, or
//       tune.flankWait runs out; then 'go go go' and everybody pushes at the run. The FLANKER(s): A* again with the presser's corridor cells (one cell either
//       side too) and the whole quarter of the fix's disc on the presser's bearing made six times dearer, so the route comes into the fix's four-metre disc
//       from a bearing at least 90° round from the presser's — if such a route exists inside 2.2× the direct length; he radios 'moving round', jogs it, radios
//       'set' five metres short of HIS last corner and kneels there for the word. No such route (one door, one corridor): he is a SECOND presser, three
//       metres BEHIND the first up the same line, never abreast. Arriving with nothing there they drop into the search they always had (its spots split
//       between them by whoever is nearer, which two bearings make two halves of the room). Re-planned when the fix moves four metres, when a role-holder
//       drops out, and swapped when the presser is pinned by fire and the flanker is not.
//   (B) THE LONE MAN'S ANGLE. One alert man, nobody else of his side up, his fix on an ARMED intruder (a man of another side always is; Sam once he has fired
//       lately or been seen with a gun in his hand) more than five metres off: he does not run across open floor at a gun. He takes the nearest corner on his
//       way that has a LINE on the fix (hard by a wall, the fix in his line of fire — through a pane is not — never nearer than 5.5 m), calls it in over the net — which brings his side — and holds,
//       kneeling, up to tune.loneWait for a second man (the moment one is up, (A) deals the two of them roles); then he goes on alone at a WALK, pieing the
//       doorways as ever. Inside five metres he fights as he always did.
//   (C) THE KILL-ZONE DOORWAY (dangerDoors + doorGate). A doorway turns dangerous for a side for tune.doorDanger seconds when a man of that side goes down
//       within 2.2 m of it IN A COLLEAGUE'S SIGHT (combat.ts killGuard → tacticsManDown), when one of them finds a colleague's body within 2.2 m of it (the
//       body discovery → tacticsBodyFound), or when a round crosses its opening at a man of that side (the shot events' muzzle → impact segments, scanned in
//       tacticsTick). Then, for that side: every path plans round it if another way exists inside 2.5× the length (goTo → tacticsPenalty: its cells ×20 and
//       more); a man whose path still runs through it — or ends within two metres of it — does NOT walk in (followPath → doorGate): he stacks on the near jamb
//       out of the opening's line (the RoomClearPlan's stack marks when the level has a plan for that door from his side, else 0.9 m along the wall), radios it,
//       which fetches the nearest free colleague to stack behind him, waits up to tune.doorWait for that man, and the two take it as a drill: a stun charge in
//       first if either carries one (giveStuns — nobody on the night staff does; a response team might), else the lead slices the pie — side-steps across the
//       opening a couple of metres back in four steps, muzzle into the room, a quarter second a step — then both cross fast and split (the plan's entry legs,
//       else a stride and a half in and sixty degrees either way); alone after the wait, he slices and goes in at a run. Nobody kneels by a body in that
//       funnel meanwhile (bodyAftermath → bodyDutyBlocked drops the duty). A doorway banged AND entered is plain again at once; entered, soon; else it decays.
// Sites (the whole seam): game.ts update() → tacticsTick, its constructor → loadTactics; guards.ts: the alert branch's chase → tacticsAlert (else chaseFix, the
// old lines), followPath → doorGate, goTo → tacticsPenalty, the aim block → tacticsLook, the body discovery → tacticsBodyFound, bodyAftermath → bodyDutyBlocked;
// combat.ts killGuard → tacticsManDown. The radio lines go through Game.say like everybody's (so the debrief's chronicle has them). Everything readable from
// above is the point: the presser kneeling at his corner, the flanker jogging the long way round, the pair on the jamb slicing the opening before they cross.
import { SideMap, SideSet, GameSideMap } from '../math/side';
import { Vec3, v3, wrapAngle, DEG } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { Item } from './throwables';
import type { Team } from './teams';
import './wallreact';
import { pointInView, doorCells } from './guards';
import './condition';
import './lanes';
import './teams';
import './glasspen';
import './combat';
import './dialogue';
import './doornames';
import './orders';
import './suppression';
import './pressure';
import './enemysquads';
import './handleside';
import './rails';   // (nobody is handed a colleague's man who is on his knee, spared)
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

// ---------------------------------------------------------------- knobs (Game.tune carries them; these are the shipped values and the per-machine copy)
type TacticsTune = { tactics: boolean; flankWait: number; loneWait: number; doorDanger: number; doorWait: number };
export const TACTICS_DEFAULTS = { flankWait: 6, loneWait: 8, doorDanger: 25, doorWait: 6 };
const LS_KEY = 'twopointsix.tactics';
const KEPT = ['flankWait', 'loneWait', 'doorDanger', 'doorWait'] as const;
/** read the kept knobs into `tune` (game.ts constructor, once): finite numbers in a sane range only; the on/off switch is not kept (an A/B seam boots ON) */
export function loadTactics(tune: TacticsTune) {
  try {
    const j = JSON.parse(localStorage.getItem(LS_KEY) ?? 'null'); if (!j || typeof j !== 'object') return;
    for (const k of KEPT) { const v = (j as Record<string, unknown>)[k]; if (typeof v === 'number' && Number.isFinite(v) && v >= 0 && v <= 600) tune[k] = v; }
  } catch { /* no storage (the harness, a locked-down page): the shipped values stand */ }
}
/** write them (the panel calls it on every change) */
export function saveTactics(tune: TacticsTune) {
  try { const out: Record<string, number> = {}; for (const k of KEPT) out[k] = tune[k]; localStorage.setItem(LS_KEY, JSON.stringify(out)); } catch { /* nothing to keep it in */ }
}
/** the four back to the shipped values, the stored copy dropped */
export function resetTactics(tune: TacticsTune) { Object.assign(tune, TACTICS_DEFAULTS); try { localStorage.removeItem(LS_KEY); } catch { /* none */ } }

// ---------------------------------------------------------------- constants
/** (A): men further than JOIN_M from the fix are dealt roles (nearer, the fight / chase as it was); a fix within SHARE_M of another man's is the same fix; the
 *  target's disc the bearings are measured on; the least angle round from the presser a flank must come in at; the longest a flank may be against the direct
 *  route; how dear the presser's corridor (and his quarter of the disc) is to the flanker's search; the corridor's dilation in cells; 'set' this far short of the
 *  flanker's corner; the second presser's station behind the first (stop inside, follow beyond + the hysteresis); a plan re-deals when the fix moves REPLAN_M */
const CORRIDOR_COST = 6;
/** (C): a body / a fall within DOOR_NEAR_M of a doorway's middle marks it; a round's far segment within ROUND_AT_M of a man is 'at' him; the gate picks a
 *  dangerous doorway up this far ahead along the path, or a goal this near it; the door cells' least multiplier (raised with the route's length so a way round
 *  inside 2.5× is always the cheaper); the stack spot along the wall from the jamb and off the wall; the slice line's stand-off (the largest that fits the
 *  floor), its steps and dwell; the entry legs; how long a gate may hold a man all told; the summoned man comes from within SUMMON_M */
const DOOR_NEAR_M = 2.2, DOOR_COST_MIN = 20;

// ---------------------------------------------------------------- records
export type RoleKind = 'presser' | 'flanker' | 'second';
export interface Role {
  kind: RoleKind; plan: Plan;
  /** presser: the corner he stops short at; flanker: his last corner (where he is 'set'); second: unused (his station is off the presser) */
  holdAt: Vec3 | null;
  /** the route dealt him (for the panel / the probes; his feet re-plan it live through goTo with the same costs) and its numbers */
  route: Vec3[]; len: number; bearing: number;
  /** the cells his live A* pays six-fold for (flanker: the presser's corridor + the presser's quarter of the disc); null = plain */
  avoid: Set<number> | null;
  set: boolean; holding: boolean; holdT: number; arrived: boolean; saidMoving: boolean; saidSet: boolean; t0: number;
  /** flanker: metres of ground covered lately (a decaying sum) and where he stood last frame — a flank still being run is waited for */
  ground?: number; lastPos?: Vec3;
}
export interface Plan {
  id: number; team: Team; target: Vec3; t0: number; replannedAt: number; replans: number;
  members: Guard[]; roles: Map<Guard, Role>;
  phase: 'approach' | 'push' | 'done'; goT: number; why: string;
  presserBearing: number; presserLen: number; corridor: Set<number>;
  firstArriveT: number; lastRadioT: number; contact: boolean;
}
export interface Lone { fix: Vec3; holdAt: Vec3 | null; los: boolean; t0: number; holdT: number; called: boolean; released: boolean; releasedWhy: string; }
export type DangerWhy = 'drop' | 'body' | 'rounds' | 'press';
export interface DangerRec { door: Door; team: Team; until: number; t0: number; why: DangerWhy; victim: string | null; banged: boolean; entered: boolean; called: boolean; summoned: Guard | null; }
type GatePhase = 'stack' | 'open' | 'wait' | 'stacked' | 'toss' | 'bang' | 'slice' | 'cross' | 'done';
export interface Gate {
  door: Door; rec: DangerRec; phase: GatePhase; t: number; t0: number;
  side: 1 | -1;              // the face he takes it from (+1 = the +n side of the closed frame line, crossesDoor's convention)
  nearA: number;             // the jamb on his side of the opening along the frame (0 = hinge, W = latch): he stacks past it, the slice starts from it
  spot: Vec3; face: number; role: 'lead' | 'second'; lead: Guard | null;
  slicePts: Vec3[]; sliceLook: Vec3[]; sliceI: number; sliceT: number;
  legs: Vec3[]; legI: number; crossAt: number;
  look: Vec3 | null; crouch: number;
  path: Vec3[] | null; pathI: number; pathFor: Vec3 | null; stallRef: Vec3 | null; stallT: number;
  can: Item | null; tossT: number; landT: number; bangT: number;
  goalNear: boolean; opened: boolean; crossed: boolean;
  hs?: { pos: Vec3; yaw: number } | null;   // (handleside.ts) the place at the handle the lead steps to before he works a shut leaf — null once dealt with
  /** what the wait resolved to (the charge if anybody stacked has one, else the slice) and who throws it */
  action: 'toss' | 'slice'; thrower: Guard | null;
}
interface TState {
  plans: Plan[]; roles: Map<Guard, Role>; lone: Map<Guard, Lone>;
  danger: DangerRec[]; gates: Map<Guard, Gate>; cleared: Map<Guard, Map<Door, number>>;
  seenEv: WeakSet<object>; planSerial: number; lastLoneCallT: number;
  /** game time of the last deal that came to nothing (fewer than two men with a route): no re-try inside a second, or two men on an unreachable fix would run A* every frame */
  dealFailT: number;
}
const states = new GameSideMap<Game, TState>('tactics.states');
function S(g: Game): TState {
  let s = states.get(g);
  if (!s) { s = { plans: [], roles: new Map(), lone: new Map(), danger: [], gates: new Map(), cleared: new Map(), seenEv: new SideSet(), planSerial: 0, lastLoneCallT: -100, dealFailT: -100 }; states.set(g, s); }
  return s;
}
/** stun charges on a man (nobody on the night staff carries one; a response team, or a probe, hands them out) */
const kit = new SideMap<Guard, { stuns: number }>('tactics.kit');
export function giveStuns(gd: Guard, n: number): void { return goneWithTick('giveStuns') as any; }

/** QA hook: set `log` and every plan dealt / re-dealt / ended, every lone hold, every doorway marked and every gate phase says so */
export const TACTICS_DEBUG: { log: ((s: string) => void) | null } = { log: null };
/** counters for the soak's smell line (across games in one process: a run's share is the difference) */
export const TACTICS_STATS = { plans: 0, replans: 0, flankers: 0, seconds: 0, swaps: 0, goes: 0, goTimeouts: 0, arrivals: 0, loneHolds: 0, loneCalls: 0, loneReleases: 0, dangers: 0, gates: 0, summons: 0, bangs: 0, slices: 0, entries: 0, loneEntries: 0, reroutes: 0 };

/** Is any of it live: the switch on, not the tour, the AI on. */
export function tacticsOn(g: Game): boolean { return !!g.tune.tactics && !g.quietUtility && g.aiEnabled; }
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const degApart = (a: number, b: number) => Math.abs(wrapAngle(a - b)) / DEG;

// ---------------------------------------------------------------- geometry over paths and the grid
function pathLen(from: Vec3, path: Vec3[]): number { let L = 0, q = from; for (const w of path) { L += flat(q, w); q = w; } return L; }
/** the doorway whose middle lies within `r` of p (nearest such), or null */
export function doorNear(g: Game, p: Vec3, r = DOOR_NEAR_M): Door | null {
  let best: Door | null = null, bd = r;
  for (const d of g.doors.list) { const dd = flat(d.frameCentre, p); if (dd < bd) { bd = dd; best = d; } }
  return best;
}
/** 'storage' → 'storage', 'breakroom_w' → 'break room west', 'server_manager' → 'server–manager', 'fire_exit' → 'fire exit' — what a man calls a door on the net */
export function doorLabel(d: Door): string { return goneWithTick('doorLabel') as any; }

// ================================================================ (C) dangerous doorways: the records
/** every live record (until > now), optionally for one side */
export function dangerDoors(g: Game, team?: Team): DangerRec[] { const now = g.time; return S(g).danger.filter(r => r.until > now && (!team || r.team === team)); }
/** the live record for this (door, side), or null */
export function dangerAt(g: Game, d: Door, team: Team): DangerRec | null { return goneWithTick('dangerAt') as any; }
/** Mark (or re-arm) a doorway dangerous for `team` for tune.doorDanger seconds. */
export function markDanger(g: Game, d: Door, team: Team, why: DangerWhy, victim: string | null = null): DangerRec { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('markDanger', undefined, [g, d, team, why, victim]); if (__r !== RUST_PASS) return __r as any; } }
  const st = S(g); let r = st.danger.find(x => x.door === d && x.team === team);
  const until = g.time + Math.max(1, g.tune.doorDanger);
  if (r) { if (r.until <= g.time) { r.t0 = g.time; r.banged = false; r.entered = false; r.called = false; r.summoned = null; TACTICS_STATS.dangers++; } r.until = Math.max(r.until, until); r.why = why; if (victim) r.victim = victim; }
  else { r = { door: d, team, until, t0: g.time, why, victim, banged: false, entered: false, called: false, summoned: null }; st.danger.push(r); TACTICS_STATS.dangers++; }
  TACTICS_DEBUG.log?.(`${g.time.toFixed(2)}s DANGER ${d.def.name} for ${team}: ${why}${victim ? ' (' + victim + ')' : ''} until ${r.until.toFixed(1)}`);
  return r;
}
// ---------------------------------------------------------------- checkpoints (game/savestate.ts): the kill-zone doorways and the doorways men have taken as a drill; the plans, roles,
// lone holds and gates are transient hunt state the tick re-deals inside a frame from the men's own (restored) alert states and fixes
export interface TacticsSnap { danger: { door: number; team: Team; until: number; t0: number; why: DangerWhy; victim: string | null; banged: boolean; entered: boolean; called: boolean }[]; cleared: [number, number, number][]; lastLoneCallT: number; dealFailT: number; }
export function tacticsSnapshot(g: Game): TacticsSnap {
  const st = states.get(g); if (!st) return { danger: [], cleared: [], lastLoneCallT: -100, dealFailT: -100 };
  const di = (d: Door) => g.doors.list.indexOf(d);
  const cleared: [number, number, number][] = [];
  for (const [gd, m] of st.cleared) { const i = g.guards.indexOf(gd); if (i < 0) continue; for (const [d, until] of m) if (until > g.time && di(d) >= 0) cleared.push([i, di(d), until]); }
  return { danger: st.danger.filter(r => r.until > g.time && di(r.door) >= 0).map(r => ({ door: di(r.door), team: r.team, until: r.until, t0: r.t0, why: r.why, victim: r.victim, banged: r.banged, entered: r.entered, called: r.called })), cleared, lastLoneCallT: st.lastLoneCallT, dealFailT: st.dealFailT };
}
/** Game.resetGuards (a restart, the tour's hand-over, a checkpoint's clean slate): everything this Game's tactics held goes with the men it referred to — the
 *  plans, roles, lone holds and gates keyed by Guard objects that no longer stand on the floor, AND the kill-zone doorways (a doorway dangerous to the last
 *  encounter's shift priced the fresh shift's routes round the building for two minutes — review G3.3). The next S(g) builds it afresh. */
export function tacticsReset(g: Game) { states.delete(g); }
/** …into a state built afresh (whatever this Game's tactics held goes: the men it referred to are gone) */
export function tacticsRestore(g: Game, s: TacticsSnap | null | undefined) {
  states.delete(g); if (!s) return;
  const st = S(g);
  st.lastLoneCallT = Number.isFinite(s.lastLoneCallT) ? s.lastLoneCallT : -100; st.dealFailT = Number.isFinite(s.dealFailT) ? s.dealFailT : -100;
  for (const r of s.danger ?? []) { const d = g.doors.list[r.door]; if (!d || !(r.until > g.time)) continue; st.danger.push({ door: d, team: r.team, until: r.until, t0: r.t0, why: r.why, victim: r.victim ?? null, banged: !!r.banged, entered: !!r.entered, called: !!r.called, summoned: null }); }
  for (const [gi, dI, until] of s.cleared ?? []) { const gd = g.guards[gi], d = g.doors.list[dI]; if (gd && d && until > g.time) setCleared(g, gd, d, until); }
}
/** has this man taken this doorway as a drill already inside its current danger (he walks it freely from then on) */
function clearedFor(g: Game, gd: Guard, d: Door): boolean { const t = S(g).cleared.get(gd)?.get(d); return t !== undefined && t > g.time; }
/** QA / panel: has this man already taken this doorway as a drill inside its current danger (the soak's walk-in smell lets him through) */
export function tacticsCleared(g: Game, gd: Guard, d: Door): boolean { return !!states.get(g) && clearedFor(g, gd, d); }
function setCleared(g: Game, gd: Guard, d: Door, until: number) { const st = S(g); let m = st.cleared.get(gd); if (!m) { m = new Map(); st.cleared.set(gd, m); } m.set(d, until); }

/** combat.ts killGuard, the frame a man goes down (his state already 'dead'): within DOOR_NEAR_M of a doorway AND in the open view of a living man of his own
 *  side → that doorway is a kill zone for them ('drop'). Nobody saw it: nothing yet — whoever finds him there will mark it (tacticsBodyFound). */
export function tacticsManDown(g: Game, victim: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('tacticsManDown', undefined, [g, victim]); if (__r !== RUST_PASS) return __r as any; } }
  if (!tacticsOn(g)) return;
  const d = doorNear(g, victim.char.pos); if (!d) return;
  const at = victim.char.bones.chest ?? v3.add(victim.char.pos, [0, 1.1, 0]);
  const seen = g.guards.some(o => o !== victim && o.state !== 'dead' && o.team === victim.team && !o.held && g.time >= o.dazzledUntil && pointInView(g, o, at, 16, 60));
  if (seen) markDanger(g, d, victim.team, 'drop', victim.callsign);
}

// ---------------------------------------------------------------- goTo's cost function
/** guards.ts goTo: the per-cell multiplier this man's plan pays from here to `goal`, or undefined = plain A* (always, with the tactics off; and for a corpse, a
 *  choreographed man, or a man with neither a dangerous doorway on his side nor a flank to keep to). Dangerous doorways he has not yet taken as a drill cost
 *  max(DOOR_COST_MIN, 1 + 1.5·L) per cell, L his plain route's length — so any other way in shorter than 2.5× is the cheaper and a longer one is not (he goes to
 *  the door and the gate takes him); a flanker's avoid-set costs CORRIDOR_COST. */
export function tacticsPenalty(g: Game, gd: Guard, goal: Vec3): ((ix: number, iz: number) => number) | undefined { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('tacticsPenalty', undefined, [g, gd, goal]); if (__r !== RUST_PASS) return __r as any; } }
  if (!tacticsOn(g) || gd.script || gd.state === 'dead') return undefined;
  const st = S(g); const role = st.roles.get(gd); const avoid = role?.avoid ?? null;
  const recs = dangerDoors(g, gd.team).filter(r => !clearedFor(g, gd, r.door) && st.gates.get(gd)?.door !== r.door);   // (the doorway he is stacked on right now is his to take: its cells cost him nothing, or his own entry legs would route round the building)
  if (!avoid && !recs.length) return undefined;
  const nav = g.col.nav, W = nav.W;
  let doorIdx: Set<number> | null = null, doorMult = DOOR_COST_MIN;
  if (recs.length) {
    doorIdx = new Set(); const DC = doorCells(g);
    for (const [i, d] of DC) if (recs.some(r => r.door === d)) doorIdx.add(i);
    const plain = nav.findPath(gd.char.pos, goal); const L = plain ? pathLen(gd.char.pos, plain) : 10;
    doorMult = Math.max(DOOR_COST_MIN, 1 + 1.5 * L);
  }
  return (ix, iz) => { const i = iz * W + ix; let m = 1; if (doorIdx && doorIdx.has(i)) m = doorMult; if (avoid && avoid.has(i)) m = Math.max(m, CORRIDOR_COST); return m; };
}

// ================================================================ (A) the flanking approach: the role planner
/** the roster's role of a man, if any (his plan's), for the panel / soak / probes: 'presser' | 'flanker' | 'second' | 'lone' | 'door:lead' | 'door:second' | null */
export function tacticsRoleOf(g: Game, gd: Guard): string | null {
  const st = states.get(g); if (!st) return null;
  const r = st.roles.get(gd); if (r) return r.kind;
  const G = st.gates.get(gd); if (G) return `door:${G.role}`;
  const L = st.lone.get(gd); if (L && !L.released) return 'lone';
  return null;
}
export function roleRecord(g: Game, gd: Guard): Role | null { return states.get(g)?.roles.get(gd) ?? null; }
export function loneRecord(g: Game, gd: Guard): Lone | null { return states.get(g)?.lone.get(gd) ?? null; }
export function gateRecord(g: Game, gd: Guard): Gate | null { return states.get(g)?.gates.get(gd) ?? null; }
export function plansOf(g: Game): Plan[] { return states.get(g)?.plans ?? []; }

export function planApproach(g: Game, men: Guard[], target: Vec3, why = 'formed', prefer: Guard | null = null): Plan | null { return crossToRust('planApproach', undefined, [g, men, target, why, prefer]) as any; }






// ---------------------------------------------------------------- panel / QA
/** One line: the plans (who presses, who flanks, phase), the lone men, the dangerous doorways with their seconds left, the men on a gate. '' when nothing is up. */
export function tacticsSummary(g: Game): string {
  const st = states.get(g); if (!st) return g.tune.tactics ? '' : 'tactics ▸ off';
  const parts: string[] = [];
  if (!g.tune.tactics) parts.push('OFF');
  for (const p of st.plans) parts.push(`plan #${p.id} ${p.phase} on (${p.target[0].toFixed(1)}, ${p.target[2].toFixed(1)}): ` + p.members.map(m => { const r = p.roles.get(m); return `${m.callsign} ${r?.kind ?? '?'}${r?.holding ? ' (holding)' : ''}${r?.kind === 'flanker' ? ` ${Math.round(degApart(r.bearing, p.presserBearing))}° round` : ''}`; }).join(', '));
  for (const [m, L] of st.lone) parts.push(`${m.callsign} lone: ${L.released ? `on at a walk (${L.releasedWhy})` : L.holdT > 0 ? `holding ${L.holdT.toFixed(1)} s${L.called ? ', called' : ''}` : 'to his corner'}`);
  for (const r of st.danger) if (r.until > g.time) parts.push(`${r.door.def.name} dangerous for ${r.team} (${r.why}${r.victim ? ' ' + r.victim : ''}) ${Math.ceil(r.until - g.time)} s${r.banged ? ' banged' : ''}${r.entered ? ' entered' : ''}`);
  for (const [m, G] of st.gates) parts.push(`${m.callsign} on the ${G.door.def.name} door: ${G.role} ${G.phase}`);
  return parts.length ? 'tactics ▸ ' + parts.join(' · ') : '';
}
