// Sticky cameras (the maintainer, 10-12: "i think we do really need to figure out if you get omniscience in stealth levels or not … maybe a limited number of
// sticky cameras or something like that would work"): a small count of them for the rifle's launcher — its THIRD load, Chaos Theory's own way: 2 again with the
// rifle up steps ring → 40 mm → camera, the right button held shows the arc and letting go sends it. It flies flat at head height over the spot the cursor
// names and STICKS to the first wall, pane or piece of furniture built into the level it meets (a leaf, loose furniture or a man: it drops to the floor under
// where it struck and watches from there), and from then on it is ONE MORE EYE OF THE FOG OF WAR (fogofwar.ts: the same list Sam's eyes, a teammate's and the
// optical wand's are on): on a wall a wide wedge about the wall's normal, lying flat all round. Nothing else: with no fog on the floor a camera shows nothing
// the picture did not already have.
//   WHOSE: a Player's own. The count rides his rifle (Rifle.cams, beside the rings and the 40 mm rounds), the ones he has sent are `Player.cams` — in flight
//   and stuck, one list — so a second player has his own. With STICKY.take up he TAKES A STUCK ONE BACK by hand (the use key, standing at it: takeCam, below) and it is on his
//   launcher again; nobody else touches one, nobody on the floor notices one (calls kept open: the page).
//   ONE MODEL: the eye is tested by the fog's own line test (col.segmentBlocked — statics and leaves where they stand, glass passes) and by the picture's own
//   ray, like every other eye. No light term (a fog eye has none), no range limit (Sam's has none).
//   THE COST: a pouch each on the card (slots.json), the launcher's small noise at the shooter and a tick where it sticks ('prop' events: a calm man close by
//   may come and look at the tick — he finds nothing), and one of the fog's eyes for each camera that sends (FOG.eyes: at five a squad of four leaves none — drawn, blind; at seven it has two).
//   THE BALANCE (the maintainer, 10-13: "we find a way to balance sticky cams"): ONE FEED AT A TIME — only the newest stuck camera sends a picture
//   (`STICKY.live` 1; its lens is drawn red, a dark one's black), so the second camera is a second LOOK, not a second eye: sending it gives up what the
//   first one saw — and the lens is a wedge of 110°, not 160. `live` 3 and `fovDeg` 160 = the cameras as they first shipped (a recording from before the
//   key replays so: journal.ts).
// No dice, no clock but the Game's, nothing read off the picture. STICKY.on: on in the page (?nosticky), off in the module and on the frozen stand
// (TPS_STICKY=1) — off, no card row, no load on any rifle, no eye: the build as it was.
import { GameSideMap } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import { Box } from '../scene/boxes';
import type { Game, Interactable, Player, Verb } from './game';
import type { FogEye } from './fogofwar';
import { FOG, fieldDeg } from './fogofwar';   // (read inside functions only)
import './combat';
import './civilians';
import './player';   // (read inside functions only)
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** Every number of it (Tab ▸ Player & weapons ▸ Sticky cameras; in the knob registry and a journal's tables).
 *  `carry` = the most a rifle is handed (kit.ts CAMS_MAX: the card's own cap, the same three) · `max` = the most that stay out on their walls (one more sent
 *  retires the oldest) · `live` = how many of those SEND A PICTURE, the newest first (1: his headset shows one feed — sending the next camera gives the last
 *  one's view up; `max` or more = every camera out is an eye, as it first shipped) · `issue` = what a mission
 *  started the 'cameras' way hands him when his card carries fewer · the flight (a Ballistics: speed m/s, gravity, drag /s, contact radius m, seconds, substeps
 *  a frame) · `cd` s before the tube's next round · `aimY` m: the height it flies over the spot the cursor names (above a head, under a door's head) ·
 *  `fovDeg`: a wall camera's wedge about the wall's normal (a camera lying flat sees all round) · `off` m: how far off its surface the lens stands ·
 *  `flatY`: |normal.y| past which a surface is 'flat' · hearing: the tube at the shooter, the tick where it sticks ('prop' events, who 0 for the tick). */
export const STICKY = {
  on: false,
  carry: 3, max: 3, live: 1, issue: 2,
  speed: 26, gravity: 9.81, drag: 0.5, radius: 0.03, life: 1.6, substeps: 4, solveIters: 2,
  cd: 1.2, aimY: 1.9,
  fovDeg: 110, off: 0.06, flatY: 0.7,
  launchNoise: 0.3, stickNoise: 0.12,
  /** a stuck camera of his own is taken back by hand and goes on his launcher again (stickyRows / takeCam): OFF here and on the frozen stand, ON in the
   *  page (?camtake=0) · `takeReach` m on the plan from his feet to it · `takeNoise`: the tick of pulling it off, at his feet (a 'prop' event: under the
   *  landing's, over the 0.08 a calm man's ears start at — guards.ts — so one within about 2 m of him turns to it) */
  take: false, takeReach: 1.3, takeNoise: 0.1,
  /** HIS OWN STUCK CAMERAS SHOW IN THE FOG OF WAR (the maintainer, 10-17: "you should be able to see your sticky cameras in fog of war"): OFF here and on the
   *  frozen stand, ON in the page (?cammark=0). Up, a camera of his is a thing the fog never takes out of the picture (stickyBoxes keys its boxes as a
   *  wall device's: his own, it gives nobody away — one that DROPS off a wall blown out where nobody looks is seen to drop, as it is heard to) and the HUD puts a small lens on each while the fog is live (game.view.marks.cams), filled red on the
   *  one that sends. Down: a camera on floor no eye of the team has is not drawn at all, and nothing marks one — as it was. Picture only. */
  mark: false,
};
export const STICKY_STATS = { fired: 0, stuck: 0, dropped: 0, retired: 0, blind: 0, taken: 0 };
export function stickyStatsReset() { const S = STICKY_STATS; S.fired = S.stuck = S.dropped = S.retired = S.blind = S.taken = 0; }

/** One camera a player has sent: in flight while `vel` is set, else stuck at `pos` on a surface with normal `n` (`si` = the static it is on, −1 = the bare
 *  floor / dropped). `id` = its place in the order they were sent (1 up): the newest is served an eye first, and the fog's smoke sensors key on it. */
export interface StickyCam { pos: Vec3; n: Vec3; vel: Vec3 | null; age: number; si: number; id: number; }
/** a stuck camera as a checkpoint holds it (one in flight refuses the save: savestate.ts) */
export interface StickySnap { p: number[]; n: number[]; si: number; id: number; }

/** the character ids a camera's eye goes by in the fog's frame (the owner byte of nothing: no man's kit is skipped for it) */
const EYE_ID = 200;

/** THE MISSION'S OWN ISSUE: a quiet mission whose link carries `cams=N` (quietfog.ts: the Cameras row under the fog lever) hands the player at least N
 *  for the visit, over whatever his card carries — written by scenarioRun.ts at the start and let go of at the quit, read by kit.ts where his card is put on
 *  him (every staging: a restart hands them again; a checkpoint's own count speaks over it). */
const VISIT = new GameSideMap<Game, number>('stickycam.VISIT');   // (per Game: two stood up in one process each keep their own)
export const stickyVisit = (g: Game): number => (STICKY.on ? VISIT.get(g) ?? 0 : 0);
export function stickyVisitSet(g: Game, n: number) { const k = Math.max(0, Math.min(STICKY.carry, Math.floor(n) || 0)); if (k) VISIT.set(g, k); else VISIT.delete(g); }

/** hand a player `n` cameras for his launcher (kit.ts kitPlayer: his card's; scenarioRun.ts: a mission started the 'cameras' way; a checkpoint's count): the
 *  count on the rifle, and 'cam' on its list of loads exactly while the switch is on, the gun has a tube and he was issued any (`listed`: a rifle whose
 *  cameras are all sent keeps the load, dry). Nothing issued = the rifle as it was built. */
export function stickyIssue(pl: Player, n: number, listed = n > 0) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('stickyIssue', undefined, [pl, n, listed]); if (__r !== RUST_PASS) return __r as any; } }
  const rf = pl.rifle, has = rf.attachments.indexOf('cam');
  const want = STICKY.on && listed && rf.attachments.includes('ring');   // (the shotgun in the rifle's slot has no launcher)
  rf.cams = want ? Math.max(0, Math.min(Math.floor(n), STICKY.carry)) : 0;
  if (want && has < 0) rf.attachments.push('cam');
  else if (!want && has >= 0) { rf.attachments.splice(has, 1); if (rf.attIdx >= rf.attachments.length) rf.attIdx = rf.attachments.length ? 0 : -1; }
}


/** One camera leaves `shooter`'s tube (player.ts, the launcher's gesture with the 'cam' load up): in the air from this frame, the tube's sound, and the room
 *  close by hears it go (a 'prop' event at the shooter — not gunfire). The count is the caller's to take. */
export function fireCam(g: Game, pl: Player, from: Vec3, target: Vec3): StickyCam { return crossToRust('fireCam', undefined, [g, pl, from, target]) as any; }



const flat = (c: StickyCam) => Math.abs(c.n[1]) > STICKY.flatY;
/** where a stuck camera's lens stands */
export function camLens(c: StickyCam): Vec3 { return v3.mad(c.pos, c.n, STICKY.off); }

/** the stuck cameras that send a picture: the newest STICKY.live of them (the rest stay on their walls, dark) */
export function stickyLive(pl: { cams: StickyCam[] }): StickyCam[] {
  const out: StickyCam[] = [], n = Math.max(0, Math.floor(STICKY.live));
  for (let i = pl.cams.length - 1; i >= 0 && out.length < n; i--) if (!pl.cams[i].vel) out.push(pl.cams[i]);
  return out;
}

/** The fog's eyes a player's LIVE stuck cameras are, newest first (fogofwar.ts update appends as many as its frame still has room for): on a wall a wedge of
 *  STICKY.fovDeg about the wall's normal (under the fog's cone the cone's own width, as the optical wand's lens is: the picture has one soft edge for every
 *  wedge), lying flat all round; no near field. Empty with the switch off or nothing out. */
export function stickyEyes(g: Game): FogEye[] {
  if (!STICKY.on) return NONE;
  const cams = g.player.cams; if (!cams.length) return NONE;
  const out: FogEye[] = [], wide = FOG.cone ? Math.min(STICKY.fovDeg, fieldDeg()) : STICKY.fovDeg;
  for (const c of stickyLive(g.player)) {
    const p = camLens(c), all = flat(c);
    out.push({ id: EYE_ID + (c.id % 50), pos: p, look: all ? [1, 0, 0] : v3.normalize([c.n[0], 0, c.n[2]]), at: p, fov: all ? 360 : wide, near: 0 });
  }
  return out;
}
const NONE: FogEye[] = [];

// ---------------------------------------------------------------- taking one back (STICKY.take)
type RowFn = (base: Interactable, verb: Verb, text: string, hints?: string, over?: Partial<Interactable>) => Interactable;
type OfferFn = (it: Interactable, verbs: Interactable[]) => void;
/** the camera `pl` could take back this frame: his nearest stuck one, in reach, hands free (null: none, or the switch is down) */
export function camToTake(g: Game, pl: Player): StickyCam | null { return crossToRust('camToTake', undefined, [g, pl]) as any; }
/** The interaction row of a stuck camera (player.ts buildInteractables): the NEAREST one only (two in reach would be one row to the selector), a marker from
 *  NEARBY_M and 'take it back' in reach. It is listed past the rows Space had before it (player.ts: it takes no row from a door or a man). */
export function stickyRows(g: Game, pl: Player, busy: boolean, row: RowFn, offer: OfferFn): void { return goneWithTick('stickyRows') as any; }

/** Drawn (game.ts, beside the rings and the 40 mm rounds): a matt dark block the size of a fist flush on its surface and a darker lens proud of it — in
 *  flight, lens first along the line it flies; the lens of a camera that is SENDING — one the fog took as an eye this frame (fogofwar.ts: none with no fog
 *  up, none past the fog's eyes, FOG.eyes) — is a dull red, a dark one's black (paint, never a light).
 *  Picture only: NoShadow dynamics no ray, eye or foot meets. */
export function stickyBoxes(g: Game, out: Box[]): void { return goneWithTick('stickyBoxes') as any; }

/** a camera of his is still in the air (savestate.ts: a plain save waits for it) */
export function stickyFlying(g: Game): boolean { return g.player.cams.some(c => !!c.vel); }
/** what a checkpoint writes as left to send: the rifle's count and any still in the air (only ever under a forced save — a plain one waits: the camera in
 *  flight is handed back unsent rather than lost) */
export function stickyLeft(pl: Player): number { let n = pl.rifle.cams; for (const c of pl.cams) if (c.vel) n++; return Math.min(n, STICKY.carry); }
/** the checkpoint's slice (the stuck ones; absent with none out) and its way back */
export function stickySnapshot(pl: Player): StickySnap[] | undefined {
  const out: StickySnap[] = []; for (const c of pl.cams) if (!c.vel) out.push({ p: [...c.pos], n: [...c.n], si: c.si, id: c.id });
  return out.length ? out : undefined;
}
export function stickyRestore(pl: Player, snap: StickySnap[] | undefined, left: number | undefined, seq?: number) {
  pl.cams.length = 0; pl.camSeq = STICKY.on && Number.isInteger(seq) && seq! > 0 ? seq! : 0;   // (the run's own count: one that was in the air at a forced save, handed back unsent, keeps its id used)
  stickyIssue(pl, left ?? 0, left !== undefined);
  if (!STICKY.on || !Array.isArray(snap)) return;
  for (const s of snap) if (s && Array.isArray(s.p) && Array.isArray(s.n)) pl.cams.push({ pos: [+s.p[0] || 0, +s.p[1] || 0, +s.p[2] || 0], n: [+s.n[0] || 0, +s.n[1] || 0, +s.n[2] || 0], vel: null, age: 0, si: Number.isInteger(s.si) ? s.si : -1, id: s.id | 0 });
  for (const c of pl.cams) pl.camSeq = Math.max(pl.camSeq, c.id);
}

/** the panel's line */
export function stickySummary(g: Game): string {
  const pl = g.player, out = pl.cams.filter(c => !c.vel).length, eyes = g.fogOfWar.active ? g.fogOfWar.eyes.filter(e => e.id >= EYE_ID).length : 0;
  return !STICKY.on ? 'off' : `${pl.rifle.cams} to send · ${out} out${g.fogOfWar.active ? ` · ${eyes} lifting the fog` : out ? ' · no fog on this floor: nothing for them to show' : ''}`;
}
