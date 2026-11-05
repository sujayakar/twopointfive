// GLASS FRACTURE (rebuild/glass/fracture.md; the pure geometry is game/glassPattern.ts, the shaders' half common.wgsl gb* + composite.wgsl gbCracks): a pane cracks the way
// real glass cracks and comes apart piece by piece where it was hit. Behind ONE switch, `tune.glassBreak` (default ON, kept per machine: `twopointsix.glassfracture.on`; Tab ▸
// Destruction ▸ Glass): off — or chaos 0, or the tour (destruction.ts chaosOf) — = every entry point returns on its first line and a pane is exactly what glass.ts says it is
// (a round stars it and stops in it, for ever). The renderer's half is a COMPILE-TIME const (`GLASS_BREAK` in the scene prelude, render/frame.ts glassBreakConfig, raised by
// main.ts at boot from the persisted switch — a page reload applies a change): with it false the shaders are the base's programs.
//
//   • A BLOW leaves a SITE on the pane (≤ maxSites; glassPattern.ts): 9‥12 RADIAL cracks out of it at hashed bearings and straight CHORDS joining neighbouring radials at
//     hashed radii — the concentric cracks, partial arcs (a share of the sectors has no middle chord). Round the site that cuts a CRATER (band 0), small PIECES round it
//     (bands 1, 2) and, once the pane has RUN, long narrow DAGGERS out to the frame — every sector forks once past the rings, and most daggers are snapped across once on
//     the way (bands 3‥6). A later blow's ring zone is fitted into the room the earlier ones left and territories are a power diagram, so no piece is ever re-cut.
//   • TYPE decides what a blow does (typeOf: the facade's glazing is tempered, interior partitions annealed, narrow sidelights and clerestory strips laminated — or,
//     under GLASSREAL.place, on in the page, by where the pane sits: to the floor or a door's panel tempered, from a sill annealed; setType overrides one pane):
//       annealed  — a round (energy ≥ punchE) punches the CRATER out and leaves the star standing: radials part of the way into the sheet, two rings of pieces wedged round
//                   the hole. A round landing on a piece knocks THAT piece out (the hole grows). When the pane has taken breakE.annealed of energy (or one blow of
//                   runSingleE) the radials RUN to the frame: the sheet is daggers now, the blow that ran it breaks its own star's rings out (runStar), and the SUPPORT
//                   rule drops what that leaves unheld — a beat later, staggered.
//       tempered  — holds to breakE.tempered (one round), then DICES: the whole sheet lets go as a ragged wave outward from the blow over a fraction of a second, in
//                   irregular CLUMPS of dice.
//                   (GLASSREAL.drop, on in the page: no wave — the crazed sheet stands a tenth of a second and every clump leaves on one step.)
//       laminated — a tight web round each blow (short radials, an outer ring, a sub-chord in every band) that never opens; only a blast close enough blows the whole
//                   crazed SHEET out of its frame.
//   • SUPPORT (heldSet / supportPass): a piece is HELD if the uncut sheet grips it, if it STANDS — on the sill or on a held piece below it across a crack flatter than
//     45°, with its centre over that footing and the footing at least standFoot of its width — or if it HANGS: its area is no more than hangArea m² per metre of grip
//     (frame side × 2, frame top × 1, a held neighbour × ¼). Held grows outward from the frame, so nothing floats. Everything else falls, a beat later and staggered. So
//     after a pane runs the fan of daggers ABOVE the blow lets go (they stand on their points), the fans to the sides and below stay stacked on the sill as teeth, small
//     pieces stay wedged in the frame's corners, and shooting a tooth out brings down what stood on it.
//   • ONE MODEL: the moment a piece lets go it is open to every consumer — the renderer's glassTrace / glassTransmit (view, mirror, light: the table fill() packs),
//     StaticCollision.raycast / segmentBlocked through openAt() (rounds, sound, a step test, a line of fire), and feet once every piece reaching below walkClearH has gone but for teeth in the
//     frame (walkable; rays still stop in a tooth; the nav grid is NOT rebaked). A pane more than a quarter open on an exterior wall registers a GUST with the wind (wind.ts 'glass').
//   • TWO SEAMS for the glass goal's sibling parts: impact() — what penetration calls per round per pane (through glass.ts glassImpact, which adds the star, the crack heard
//     and the hearing event) — and onRelease — one call per fragment as it lets go, for the shards part. While onRelease is null the base's stand-in runs (thin Debris
//     boxes of kind 'shard' + chip particles), so the build is whole without it. blast() is what destruction's blast entry point calls; a stun charge reaches it weaker
//     through glass.ts updateGlass.
//   • Bounded: ≤ maxSites a pane, ≤ releasesPerFrame queued pieces and ≤ clumpsPerFrame clumps let go a frame (a round's own piece is never deferred), the debris pool's
//     cap in the stand-in; deterministic: every choice hashes the pane's own word (glass.ts boxSeed: what the box is, never its place in the level's list), the impact point (glass.ts pointSeed; a blast's by blastSeed) or a piece's id — never Math.random. reset() = every pane
//     whole (resetLevelState; a checkpoint load is a restart, so holes are never saved — and no static is appended, so savestate's levelFingerprint is untouched).
import { Vec3, v3 } from '../math/vec';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { VIEW_BLOWN, VIEW_GIVES, type PaneRow, type PaneView } from '../scene/boxMirror';
import type { Game } from './game';
import { men } from './seats';
import { glassImpact, glassEvent, GLASSFX, glassPanes } from './glass';
import { hash01 } from './debris';
import { chaosOf } from './destruction';
import { gpuFx } from './particleFx';
import { WIND, WIND_FIELD, type WindSource } from './wind';
import { segSegClosest } from './devices';
import { GP_MAX_SITES, GP_FRAGS_PER_SITE, GP_MAX_SECTORS, GP_BANDS, type GpSite, type GpFrag, type GpDice, type Pt, gpPolyArea, gpDiceCentre, gpDiceKey, gpFragments } from './glassPattern';
import { GlassBreakTwin } from './glassbreakTwin';
import { sim as simX, f64, u32, type SimExports } from '../sim/sim';
import type { SimState } from '../sim/state';
import { knobsToRust, RUST_KNOBS } from './rustknobs';
import { SHARD } from './shardModel';

export type GlassType = 'annealed' | 'tempered' | 'laminated';
/** the knobs (Tab ▸ Destruction ▸ Glass edits breakE / hangArea / blastM live; kept per machine by saveGlassBreak) */
export const GLASSBREAK = {
  maxSites: GP_MAX_SITES,
  /** energy (1 = one pistol round square on) a pane takes before it gives: annealed = its radials RUN to the frame (daggers, the support rule); tempered = it dices */
  breakE: { annealed: 3, tempered: 1 },
  /** a blow at least this hard punches the crater out of annealed glass / knocks the piece it lands on out; a softer one only cracks */
  punchE: 0.5,
  /** one blow this hard runs an annealed pane by itself */
  runSingleE: 2.5,
  /** the blow that RUNS a pane breaks its own star out: this many of the rings round the site it landed on (0‥2) follow the crater, runRingSecs apart */
  runStar: 2, runRingSecs: 0.03,
  /** falling glass is heard (one crack + one 'glass' hearing event) at most once per pane per this many seconds */
  crackGap: 0.25,
  /** base radii (m) of the three chords round a blow, by type — each ≥ 1.57× the last (glassPattern.ts: the chords of a sector must nest) — and [3] the base distance of
   *  the daggers' snaps (annealed) / of the web's outer ring (laminated); × (0.75 + 0.25 e) clamped 0.8‥1.4 */
  rho: { annealed: [0.03, 0.07, 0.14, 0.5], laminated: [0.018, 0.05, 0.11, 0.22] } as Record<'annealed' | 'laminated', [number, number, number, number]>,
  /** radial cracks a blow starts: [fewest, spread] (≤ 12 in all), and the share of an annealed site's sectors with no middle chord */
  radials: { annealed: [9, 4], laminated: [10, 3] } as Record<'annealed' | 'laminated', [number, number]>, merge: 0.45,
  /** a new site's ring zone is shrunk to the room the others leave; under this (m) there is no room and the blow counts on the nearest site instead */
  rzMin: 0.05,
  /** SUPPORT: seconds between a change and the pass that drops what it left unheld; each piece then goes after a further stagger (distance from the blow + a hash) */
  think: 0.2, staggerPerM: 0.18, staggerJit: 0.12,
  /** a piece that is not STANDING is held only up to this area (m²) per metre of grip — its edge on a frame side × grip.side, on the top × grip.top, against a held neighbour × grip.wedged */
  hangArea: 0.12, grip: { side: 2.0, top: 1.0, wedged: 0.25 },
  /** a piece STANDS on the sill / on the held neighbour below it across a crack flatter than standNy (the edge normal's downward share) when that footing is at least
   *  standFoot of its own width and its centre is over it (± balanceTol m) */
  standNy: 0.7, standFoot: 0.3, balanceTol: 0.02,
  /** blast(): default reach (m) to a pane's nearest point, and the power a stun charge's 'bang' carries (glass.ts updateGlass); a TOOTH — a piece on the frame under
   *  toothArea m² — is left in by a blast weaker than toothK there (the jagged rim a blown window keeps) */
  blastM: 2.6, bangPower: 0.6, toothArea: 0.03, toothK: 0.8,
  /** blast(): the strength k = power × (1 − d ∕ radius) past which each kind gives — annealed RUNS (plain float glass goes first), toughened glass DICES (four or five times
   *  the strength: only close in), a laminated sheet leaves its frame whole. Tuning round two: they were 0.35 / 0.2 / 0.75 as literals, which had a stun charge 1.2 m off dice
   *  a toughened facade sheet and only star a float partition — the wrong way round */
  blastRunK: 0.2, blastDiceK: 0.45, blastSheetK: 0.75,
  /** RAKING (tuning round two): Sam walking into what is left of an annealed pane that has run and is at least rakeOpen open takes every piece in his way with him (he alone — glassbreak.ts rake() — all
   *  but the teeth, everything reaching below walkClearH) — a magazine left jagged remains standing on the sill and the opening could never be stepped through. Heard as
   *  falling glass. 0 = off: the remains stand until a round or a blast takes them */
  rakeOpen: 0.5, rakeM: 0.5,
  /** tempered: the dice drawn and handed on as `grain` (m), the clump a diced pane lets go in (m), the wave's delay and speed outward from the blow */
  diceGrain: 0.012, clumpM: 0.12, diceHang: 0.09, diceSpeed: 4.2, clumpsPerFrame: 40,   // (round two: 0.05 / 5 — the sheet frosts for a beat before the curtain comes down, and comes down a little slower)
  /** queued pieces let go per frame (a round's own piece is never deferred) */
  releasesPerFrame: 8,
  /** how a piece leaves: out along the blow (m/s, × 0.4‥1.4), spin (rad/s); a blast throws at blastOut × its strength there */
  out: 0.7, spin: 5, blastOut: 5,
  /** the STAND-IN while no onRelease consumer is attached (the base's shards): thin Debris boxes + pale chips */
  shardAlbedo: [0.72, 0.8, 0.82] as Vec3, chipColor: [0.8, 0.88, 0.9] as Vec3, chipsPerM2: 90, chipsMin: 8, diceChipsPerM2: 260, crumbEvery: 12,
  gustSpeedK: 0.6,
  /** a pane is walked through once every piece reaching below this height (m above the floor) has fallen */
  walkClearH: 1.9,
};
/** GLASS AS BUILDINGS HAVE IT (two rules, each ON in the page — `?glassdrop=0`, `?glassplace=0` — and off here and on the frozen stand, `TPS_GLASSDROP=1`,
 *  `TPS_GLASSPLACE=1`; Tab ▸ Destruction ▸ Glass breaking). Off = the build as it was: the wave below and typeOf's first rule.
 *    drop  — toughened glass lets go ALL AT ONCE. Its stored stress crazes the whole sheet in a millisecond; nothing travels across it that an eye could follow. So
 *            the sheet stands white for `hang` seconds and then every clump leaves on the one step (stepDice → dropAll), handed on as at most `blocks` fragments
 *            that fall: out along the blow a little, no push across the pane. (The wave was the same release spread over half a second at diceSpeed.)
 *            HEARD ONCE: `hang` is kept under GLASSBREAK.crackGap, so the sheet's breaking is the one crack and the one hearing event the blow's own clump files
 *            (emit); its fall files no second, and the landings are falling glass's own sounds. A hang past crackGap would file a second of each.
 *    place — which glass a pane is comes from WHERE IT SITS, as a glazier would have it: glass a body could fall into is safety glass. A pane whose foot is under
 *            `lowM` above the floor (it reaches the floor: a partition, a shopfront, frontage on a kick wall) or that is a panel of a door (within `doorM` of a
 *            door's opening on the plan and its foot under `doorFootM`: glass beside a door within a body's reach, not a light over it) is TOUGHENED; a window that starts at a sill is plain FLOAT glass, outside wall or not. Narrow inside
 *            sidelights and high outside strips stay laminated, as they were. (The first rule read the wall instead: outside = toughened, inside = float.) */
export const GLASSREAL = {
  drop: false, place: false,
  hang: 0.1, blocks: 48,
  lowM: 0.7, doorM: 0.3, doorFootM: 1.5,
};
export const GLASSBREAK_DEFAULTS = { breakE: { ...GLASSBREAK.breakE }, hangArea: GLASSBREAK.hangArea, blastM: GLASSBREAK.blastM, blastRunK: GLASSBREAK.blastRunK, blastDiceK: GLASSBREAK.blastDiceK, blastSheetK: GLASSBREAK.blastSheetK, rakeOpen: GLASSBREAK.rakeOpen };
/** the tuning rounds' knobs the panel keeps beside the first four (each 0‥1) */
const GB_KEPT = ['blastRunK', 'blastDiceK', 'blastSheetK', 'rakeOpen'] as const;
const LS_KEY = 'twopointsix.glassfracture.on', LS_KNOBS = 'twopointsix.glassfracture';   // (the knobs under their own key: the first model's failHits / dropSupport kept on a machine mean nothing here)
type GbTune = { glassBreak?: boolean };
/** the persisted switch alone (main.ts reads it BEFORE the Engine builds a pipeline, to raise the prelude const) */
export function glassBreakPersisted(): boolean { try { const raw = localStorage.getItem(LS_KEY); if (!raw) return true; return JSON.parse(raw)?.on !== false; } catch { return true; } }   // (ON unless this machine said off — under a key of its own: 'twopointsix.glassbreak' holds choices made while off was the default)
/** Game constructor: the switch + knobs kept per machine */
export function loadGlassBreak(tune: GbTune) {
  try { const raw = localStorage.getItem(LS_KEY); if (raw) { const o = JSON.parse(raw); if (typeof o.on === 'boolean') tune.glassBreak = o.on; } } catch { /* no storage / bad JSON: defaults */ }
  try {
    const raw = localStorage.getItem(LS_KNOBS); if (!raw) return; const o = JSON.parse(raw); const num = (x: unknown): x is number => typeof x === 'number' && isFinite(x);
    if (o.breakE) for (const k of ['annealed', 'tempered'] as const) if (num(o.breakE[k])) GLASSBREAK.breakE[k] = Math.max(0.25, Math.min(20, o.breakE[k]));
    if (num(o.hangArea)) GLASSBREAK.hangArea = Math.max(0, Math.min(1, o.hangArea));
    if (num(o.blastM)) GLASSBREAK.blastM = Math.max(0, Math.min(6, o.blastM));
    for (const k of GB_KEPT) if (num(o[k])) GLASSBREAK[k] = Math.max(0, Math.min(1, o[k]));
  } catch { /* */ }
}
export function saveGlassBreak(g: Game) {
  try { localStorage.setItem(LS_KEY, JSON.stringify({ on: !!(g.tune as GbTune).glassBreak })); localStorage.setItem(LS_KNOBS, JSON.stringify({ breakE: { ...GLASSBREAK.breakE }, hangArea: GLASSBREAK.hangArea, blastM: GLASSBREAK.blastM, blastRunK: GLASSBREAK.blastRunK, blastDiceK: GLASSBREAK.blastDiceK, blastSheetK: GLASSBREAK.blastSheetK, rakeOpen: GLASSBREAK.rakeOpen })); } catch { /* */ }
}
/** the one gate every entry point takes first: the switch on AND destruction's contract (chaos 0 / the tour = the game byte for byte) */
export function glassBreakLive(g: Game): boolean { return GLASSFX.on && !!(g.tune as GbTune).glassBreak && chaosOf(g) > 0 && g.engine.glassFracture !== false; }   // (a host whose renderer has no port of the per-piece rule — the native engine today — says glassFracture false: the model stays inert there rather than open holes the picture does not show)

// ---------------------------------------------------------------- the two seams (rebuild/glass/fracture.md)
/** what impact() answers — the penetration part reads it and decides what the round does next */
export interface GlassImpact {
  /** false = not a glass box: nothing happened, ignore the rest */ pane: boolean;
  /** false = glass breaking is off (or chaos 0 / the tour / thick solid glass): nothing was filed */ live: boolean;
  type: GlassType;
  /** metres of glass across the pane's thin axis */ thickness: number;
  /** the glass AT the point had already fallen out before this call (nothing was filed) */ wasOpen: boolean;
  /** after this call there is no glass at the point: the crater, the piece under it or the pane's first clump let go */ punched: boolean;
  /** a tempered pane went with this call */ diced: boolean;
  /** fragments this call let go or queued */ released: number;
  /** this blow left its own mark in the fracture model (a new star, a piece out, the pane diced) — false = it only added energy to an old star: glass.ts draws its star decal */ cracked: boolean;
}
/** one fragment as it lets go — the shards part's input (onRelease) */
export interface GlassFragment {
  /** level box index of the pane it left */ pane: number;
  type: GlassType;
  /** a small piece from round the hole (the crater too: grain > 0) / a long dagger running to the frame / a patch of a diced tempered pane / a whole crazed laminated sheet */
  kind: 'piece' | 'dagger' | 'clump' | 'sheet';
  /** world space: a convex polygon on the pane's middle plane, 3‥12 points, anticlockwise seen from the side `normal` points to */ outline: Vec3[];
  centre: Vec3;
  /** unit: the pane's normal on the side the fragment leaves by */ normal: Vec3;
  thickness: number; area: number;
  /** m/s at release */ velocity: Vec3;
  /** rad/s — a suggestion */ spin: Vec3;
  /** the pane's transmission tint (its box albedo) */ tint: Vec3;
  /** metres: the fragment should burst into grains about this size (a clump's dice, a crater's powder); 0 = it stays one piece */ grain: number;
  cause: 'hit' | 'support' | 'blast' | 'dice' | 'rake';
  who: number;
  /** a u32 for the consumer's own hashes */ seed: number;
  /** where and when it comes down, as the rule asked it the moment it let go (none: the consumer asks), and whether its landing is already the simulation's (SHARDS.byFall) */
  fall?: { t: number; pos: Vec3 }; landed?: boolean;
}
export interface GlassBlastOpts { /** metres from the blast to a pane's nearest point within which it is touched (default GLASSBREAK.blastM) */ radius?: number; /** 0‥1+ (default 1) */ power?: number; who?: number }

export interface Clump { /** r × cols + c */ k: number; c: number; r: number; /** its place in the order clumps let go (glassPattern.ts gpDiceKey: metres from the blow, shuffled) */ key: number }
export interface Dice extends GpDice { t0: number; /** sorted by key */ clumps: Clump[]; next: number; out: Uint8Array }
export interface Pending { fid: number; at: number; cause: GlassFragment['cause']; from: Vec3 | null; k: number }
export interface PaneState {
  bi: number; box: Box; /** the pane's own word (glass.ts boxSeed: what the box IS): every hash of this pane's break starts from it, none from `bi`, its place in the level's list */ key: number; type: GlassType; uAxis: 0 | 2; hu: number; hv: number; ht: number; exterior: boolean;
  sites: GpSite[]; run: boolean; open: Uint8Array; nOpen: number;
  hits: number; energy: number; damage: number;
  frags: GpFrag[] | null; pending: Pending[]; nextThink: number;
  dice: Dice | null; gone: boolean;
  lastDir: Vec3; lastWho: number; lastUV: Pt; /** game time of the last falling-glass sound / hearing event from this pane */ soundAt: number;
  /** every piece reaching below walkClearH has fallen: movement passes */ walkable: boolean;
  log: string[];
}
export const GB_TWIN = { panes: 0, impacts: 0, sites: 0, runs: 0, dropped: 0, shards: 0, chips: 0, refused: 0, diced: 0, clumps: 0, sheets: 0, blasts: 0, thinks: 0, hooked: 0, raked: 0 };
/** where the module counts what panes.rs Tally counts, by the same order */
const TALLY_AT: Partial<Record<keyof typeof GB_TWIN, number>> = { panes: 0, impacts: 1, sites: 2, runs: 3, dropped: 4, diced: 5, clumps: 6, sheets: 7, blasts: 8, thinks: 9, raked: 10 };
const tally = (i: number | undefined): number => { if (i === undefined) return 0; const x = simX(); return f64()[x.panes_tally() / 8 + i]; };
/** the counts for a tool or a panel: the twin's own, and what the module has counted (no rule reads them back) */
export const GB_STATS = new Proxy(GB_TWIN, { get: (t, k: string) => (t as Record<string, number>)[k] + tally(TALLY_AT[k as keyof typeof GB_TWIN]) });
/** rows of the GPU record per pane (scene/boxes.ts GB_STRIDE must equal it; common.wgsl reads the same layout) */
export const GB_HEADER_ROWS = 4, GB_SITE_ROWS = 4, GB_RECORD_ROWS = GB_HEADER_ROWS + GP_MAX_SITES * GB_SITE_ROWS;

/** BOTH WAYS (tools/qa/pane-ident.ts hangs itself here): every call of the rule run by the twin first and by the module after, each from the state the other had, and
 *  what each answered, what each let go and each pane's state handed over to be compared. Null everywhere a game is played. */
export const PANES: { both: null | ((what: string, twin: GlassBreakTwin, answers: [unknown, unknown], twinFrags: (GlassFragment & { key: number; heard: boolean })[], frags: (GlassFragment & { key: number; heard: boolean })[]) => void) } = { both: null };
const TYPES: readonly GlassType[] = ['annealed', 'tempered', 'laminated'];
const CAUSES: readonly GlassFragment['cause'][] = ['hit', 'support', 'blast', 'dice', 'rake'];
const KINDS: readonly GlassFragment['kind'][] = ['piece', 'dagger', 'clump', 'sheet'];
const NO_TYPE = 9, NO_OVER = 3;
// rust/sim/src/panes.rs: a fragment's doubles, and the places in them
const OUT_LEN = 32, O_PANE = 0, O_TYPE = 1, O_KIND = 2, O_PTS = 3, O_NPTS = 4, O_CX = 5, O_NX = 8, O_THICK = 11, O_AREA = 12, O_VX = 13, O_SX = 16, O_GRAIN = 19, O_CAUSE = 20, O_WHO = 21, O_SEED = 22, O_KEY = 23, O_HEARD = 24, O_FALL_T = 25, O_FALL_X = 26, O_LANDED = 29;
// …a row's fields (panes_field), found once
let F: number[] | null = null;
const FIELDS = 32;
const F_D = 0, F_W = 1, F_E = 2, F_HITS = 3, F_C = 4, F_HU = 5, F_HV = 6, F_HT = 7, F_ENERGY = 8, F_DAMAGE = 9, F_NEXT = 10, F_DIR = 11, F_UV = 12, F_SOUND = 13, F_WHO = 14, F_KEY = 15, F_TY = 16, F_AXIS = 17, F_EXT = 18, F_HITN = 19, F_NOPEN = 20, F_DNEXT = 21, F_WALK = 22, F_Q0 = 23, F_QN = 24, F_QLEN = 26, F_QAT = 27, F_QFROM = 28, F_QK = 29, F_QFID = 30, F_QCAUSE = 31;
function fields(x: SimExports): number[] { if (!F) { F = []; for (let i = 0; i < FIELDS; i++) F.push(x.panes_field(i)); } return F; }
// glass.rs: a row's words and flags
const GW_COLS = 2, GW_ROWS = 3, GW_DSEED = 4, GW_N = 5, GW_SEED = 13, GW_OPEN = 21, GW_FLAGS = 1, G_GONE = 1, G_DICE = 2, G_RUN = 4, G_SITE = 9, G_DICE_AT = 72;

/** what the rule's call has for a game: its block, the world it asks, what it may ask of the world */
type Ask<T> = (x: SimExports, g: number, w: number) => T;

export class GlassBreak {
  /** the rule's twin (glassbreakTwin.ts): run beside the module by the both-ways check, and by nothing else */
  readonly twin: GlassBreakTwin;
  /** THE RELEASE HOOK: set by the shards part; null = the stand-in below spawns the base's Debris shards + chips */
  get onRelease(): ((f: GlassFragment) => void) | null { return this.hook; }
  set onRelease(f: ((f: GlassFragment) => void) | null) { this.hook = f; this.twin.onRelease = f; }
  private hook: ((f: GlassFragment) => void) | null = null;
  /** dev's log (the twin's: the rule keeps no text) */
  get log(): string[] { return this.twin.log; }
  constructor(private g: Game) {
    this.twin = new GlassBreakTwin(g);
    g.engine.world.glassCellFill = (bi, dst, off) => this.fill(bi, dst, off);
    // (also: a burst's own view of the glass, for the length of one judgment — seeThrough below. And `this`: where the lines and rays of the module read the rule)
    g.col.glassRule((bi, p) => this.ruleOpen(bi, p), () => this);
    g.engine.world.glassOpen = (bi, p) => glassBreakLive(g) && this.openAt(bi, p);   // the aim ray too (BoxWorld.raycast: reticle / arm / throw arc pass through a hole); inert with the switch off like everything else
    g.col.glassWalkable = (bi) => this.walkable(bi);
    // the destruction engine's blast feed, where the build has one (g.destr.onBlast; res.panes is its count of ours). Absent = nothing: a blast then reaches glass through blast() called by hand or the stun's 'bang' event.
    (g.destr as unknown as { onBlast?: (fn: (b: { at: Vec3; radius?: number; power?: number; who?: number; res?: { panes?: number } }) => void) => void })?.onBlast?.((b) => {
      const n = this.blast(b.at, { radius: b.radius, power: b.power, who: b.who }); if (b.res && typeof b.res.panes === 'number') b.res.panes += n;
    });
  }

  // ---------------------------------------------------------------- the call
  /** A QUESTION OR A RULE OF THE MODULE'S, over this game's block and the copy of its box world as it stands: `fn` gets the module, the block's slot and the world's. A
   *  call from outside a tick opens a window for its length. A pane a walker now goes through is said to the copy as the call comes back. */
  asking<T>(fn: Ask<T>): T { return this.ask(fn); }
  private ask<T>(fn: Ask<T>): T {
    const g = this.g, c = g.col;
    return c.window(() => {
      const M = c.rulesStatics(); if (M === null) throw new Error('glass: the box world\'s copy will not stand for the rule (no row up that opens it): rust/sim/src/panes.rs has nothing to ask');
      if (!RUST_KNOBS.tick) knobsToRust();
      const set = M.leaves(c.dynamicBoxes); if (set === -2) throw new Error('glass: more leaves than a list of the module holds');
      const x = M.x, s = g.sim.bind();
      if (!x.panes_booted(s)) this.boot(x, s);
      x.panes_ctx(s, M.slot, set, g.time, this.hook !== null && (g.tune as { glassShards?: boolean }).glassShards !== false && glassBreakLive(g) ? 1 : 0, SHARD.gravity);
      const r = fn(x, s, M.slot);
      if (x.panes_take_wrote()) c.wrote();
      return r;
    });
  }
  /** this game's glass, for the block: every Glass static of the level, ascending, with how far it stands on the plan from the nearest door's opening */
  private boot(x: SimExports, s: number): void {
    const list = glassPanes(this.g), m = f64(), at = x.sim_scratch() / 8, boxes = this.g.level.boxes;
    for (let i = 0; i < list.length; i++) { m[at + 2 * i] = list[i]; m[at + 2 * i + 1] = this.doorGap(boxes[list[i]]); }
    x.panes_boot(s, list.length);
  }
  /** metres on the floor plan from pane b to the nearest door's opening (the closed leaf's line, hinge to latch) — Infinity on a level with no door: the level's own fact, read as it is stood up (a door's frame does not move) */
  private doorGap(b: Box): number {
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], hu = Math.max(b.h[0], b.h[2]); const ax = wide ? cs : sn, az = wide ? -sn : cs;
    const a: Vec3 = [b.c[0] - ax * hu, 0, b.c[2] - az * hu], c: Vec3 = [b.c[0] + ax * hu, 0, b.c[2] + az * hu];
    let d = Infinity;
    for (const door of this.g.doors?.list ?? []) { const D = door.def; d = Math.min(d, segSegClosest(a, c, [D.hinge[0], 0, D.hinge[1]], [D.hinge[0] + Math.cos(D.closedDir) * D.width, 0, D.hinge[1] + Math.sin(D.closedDir) * D.width]).dist); }
    return d;
  }
  /** what the last call let go: its fragments handed on in the order they went (the module has them listed; none has been told to anybody yet). The caller stands in `ask`'s window: a fragment's handling may ask lines of its own. */
  private drain(x: SimExports, keep?: (GlassFragment & { key: number; heard: boolean })[]): void {
    const n = x.panes_out_n(); if (n === 0) return;
    const m = f64(), o0 = x.panes_out_addr() / 8, p0 = x.panes_out_pts_addr() / 8;
    // (the list is read whole first: what is told of a fragment may ask the module anything)
    const list: (GlassFragment & { key: number; heard: boolean; landed: boolean; bi: number })[] = [];
    for (let i = 0; i < n; i++) {
      const o = o0 + i * OUT_LEN, bi = m[o + O_PANE], at = p0 + 2 * m[o + O_PTS], np = m[o + O_NPTS], box = this.g.level.boxes[bi];
      const V = this.geom(bi, x), outline: Vec3[] = [];
      for (let k = 0; k < np; k++) outline.push(this.world(V, m[at + 2 * k], m[at + 2 * k + 1]));
      list.push({ pane: bi, bi, type: TYPES[m[o + O_TYPE]], kind: KINDS[m[o + O_KIND]], outline, centre: [m[o + O_CX], m[o + O_CX + 1], m[o + O_CX + 2]], normal: [m[o + O_NX], m[o + O_NX + 1], m[o + O_NX + 2]], thickness: m[o + O_THICK], area: m[o + O_AREA],
        velocity: [m[o + O_VX], m[o + O_VX + 1], m[o + O_VX + 2]], spin: [m[o + O_SX], m[o + O_SX + 1], m[o + O_SX + 2]], tint: v3.copy(box.albedo), grain: m[o + O_GRAIN], cause: CAUSES[m[o + O_CAUSE]], who: m[o + O_WHO], seed: m[o + O_SEED],
        key: m[o + O_KEY], heard: m[o + O_HEARD] !== 0, landed: m[o + O_LANDED] !== 0, fall: { t: m[o + O_FALL_T], pos: [m[o + O_FALL_X], m[o + O_FALL_X + 1], m[o + O_FALL_X + 2]] } });
    }
    x.panes_out_clear();
    if (keep) keep.push(...list);
    for (const f of list) this.letGo(f, x);
  }
  /** a fragment the rule let go, told to the game: its one way out */
  letGo(f: GlassFragment & { key: number; heard: boolean; bi: number }, x: SimExports): void {
    {
      const key = f.key, V = this.geom(f.bi, x);
      if (f.heard) {   // a round's own piece was heard as the round (glass.ts glassImpact); what falls LATER — by support, a blast, the dice — is heard here, once per pane per crackGap
        const dicing = f.kind === 'clump' || f.kind === 'sheet';
        this.g.audio.play('glassCrack', f.centre, dicing ? 1.0 : 0.75, { rate: dicing ? 0.7 : 1.25 + 0.3 * hash01(key + V.key) });
        glassEvent(this.g, this.g.level.boxes[f.bi], f.centre, 1.0, f.who);
      }
      if (this.hook) { GB_TWIN.hooked++; this.hook(f); return; }
      this.standIn(V, f, key);
    }
  }
  /** a pane's frame, for what the host still makes of a fragment */
  private geom(bi: number, x: SimExports): PaneFrame {
    const s = this.g.sim.bind(), r = x.panes_row_index(s, bi), at = x.panes_row_addr(s, r), Fl = fields(x), d = this.g.sim.d, w = this.g.sim.w;
    return { key: w[(at + Fl[F_KEY]) >>> 2], uAxis: w[(at + Fl[F_AXIS]) >>> 2] === 0 ? 0 : 2, hu: d[(at + Fl[F_HU]) >>> 3], hv: d[(at + Fl[F_HV]) >>> 3], ht: d[(at + Fl[F_HT]) >>> 3], box: this.g.level.boxes[bi] };
  }
  /** pane-local (u, v) → world */
  private world(V: PaneFrame, u: number, v: number, w = 0): Vec3 {
    const b = V.box, cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), lx = V.uAxis === 0 ? u : w, lz = V.uAxis === 0 ? w : u;
    return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz];
  }
  private uv(V: PaneFrame, X: Vec3): Pt {
    const b = V.box, cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), px = X[0] - b.c[0], pz = X[2] - b.c[2], lx = cs * px - sn * pz, lz = sn * px + cs * pz;
    return [V.uAxis === 0 ? lx : lz, X[1] - b.c[1]];
  }
  /** THE STAND-IN (no shards consumer attached): the base's picture — one to three thin Debris boxes of kind 'shard' over the fragment + a burst of pale chips; a clump is chips with a rigid crumb for every crumbEvery-th */
  private standIn(V: PaneFrame, f: GlassFragment, key: number) {
    const g = this.g; const uv = f.outline.map(p => this.uv(V, p)); const { cen } = gpPolyArea(uv);
    const grains = f.grain > 0; const sheet = f.kind === 'sheet';
    const nSh = grains ? (f.kind === 'clump' && key % GLASSBREAK.crumbEvery === 0 ? 1 : 0) : sheet ? 3 : Math.max(1, Math.min(3, Math.round(f.area / 0.07)));
    const b = V.box, cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), nrm: Vec3 = V.uAxis === 0 ? [sn, 0, cs] : [cs, 0, -sn];
    const outSign = v3.dot(nrm, f.normal) >= 0 ? 1 : -1;
    for (let k = 0; k < nSh; k++) {
      const h1 = hash01(V.key * 7919 + key * 131 + k * 17), h2 = hash01(V.key * 104729 + key * 193 + k * 29);
      const vtx = uv[(k * Math.max(1, Math.floor(uv.length / nSh))) % uv.length] ?? cen;
      const pu = cen[0] * (nSh === 1 ? 1 : 0.6) + vtx[0] * (nSh === 1 ? 0 : 0.4), pv = cen[1] * (nSh === 1 ? 1 : 0.6) + vtx[1] * (nSh === 1 ? 0 : 0.4);
      const side = Math.sqrt(f.area / nSh) * (grains ? 0.35 : 0.92); const su = side * (0.8 + 0.4 * h1), sv = side * (0.8 + 0.4 * h2);
      const hh: Vec3 = V.uAxis === 0 ? [su / 2, sv / 2, Math.max(0.006, V.ht)] : [Math.max(0.006, V.ht), sv / 2, su / 2];
      const box = makeBox({ c: this.world(V, pu, pv, outSign * 0.02), h: hh, yaw: b.yaw, albedo: v3.copy(GLASSBREAK.shardAlbedo), flags: BoxFlag.Dynamic });
      const ch = g.destr.debris.spawn('shard', box, v3.copy(f.velocity), v3.copy(f.spin), f.who);
      if (ch) { ch.box.rough = 0.05; GB_TWIN.shards++; } else GB_TWIN.refused++;
    }
    const P = gpuFx(g);   // chips: pale specks thrown out and down from across the fragment (GPU particles; nothing with them off / headless)
    if (P) { const nChips = Math.round(Math.max(grains ? 3 : GLASSBREAK.chipsMin, f.area * (grains ? GLASSBREAK.diceChipsPerM2 : GLASSBREAK.chipsPerM2))); P.emit('debris', f.centre, f.normal, nChips, { shape: 'cone', speed: 1.1, speedJit: 0.8, spread: 1.2, life: 30, lifeJit: 0.4, size: grains ? 0.012 : 0.008, sizeJit: 0.6, aspect: 1.4, posJit: Math.sqrt(f.area) * 0.45, color: v3.copy(GLASSBREAK.chipColor) }); GB_TWIN.chips += nChips; }
  }
  /** run the twin's half of a call for the check (it goes first: what it asks of the world finds the state the call began from), and → what it did */
  private shade<T>(fn: () => T): { a: T; fr: (GlassFragment & { key: number; heard: boolean })[] } {
    const t = this.twin; t.shadow = []; try { const a = fn(); return { a, fr: t.shadow }; } finally { t.shadow = null; }
  }

  // ---------------------------------------------------------------- which glass
  /** the type a pane breaks as when nobody has said (setType): its wall's or its place's word (panes.rs type_by_place) */
  typeOf(b: Box, bi = this.g.level.boxes.indexOf(b)): GlassType {
    return TYPES[this.ask((x) => bi >= 0 ? x.panes_type_place(bi) : x.panes_type_box(b.h[0], b.h[1], b.h[2], b.c[1], this.doorGap(b)))];   // (a box the level does not hold, a tool's question: indoors, by its place)
  }
  /** the type of pane bi as it will break (an override from setType, else typeOf) — null for anything that is not a sheet of glass */
  typeOfPane(bi: number): GlassType | null {
    const t = this.ask((x) => x.panes_type(bi)); return t === NO_TYPE ? null : TYPES[t];
  }
  /** make pane bi break as `type` from here on (tuning, probes, a level's own table); the pane is made whole first. null = back to typeOf. An override is the level's word on the pane: reset() keeps it */
  setType(bi: number, type: GlassType | null) {
    const gusts = (): void => { const src = this.gusts.get(bi); if (src) { const i = this.g.wind.all.indexOf(src); if (i >= 0) this.g.wind.all.splice(i, 1); this.gusts.delete(bi); } };
    if (PANES.both) this.twin.setType(bi, type);
    this.ask((x, s) => { x.panes_set_type(s, bi, type ? TYPES.indexOf(type) : NO_OVER); }); this.g.col.wrote(); gusts();
  }

  // ---------------------------------------------------------------- stimuli
  /** THE IMPACT ENTRY POINT (fracture.md): a blow of `energy` (1 = one pistol round square on) met pane `bi` at world X travelling `dir`. Files it and answers what the glass did; what the round does next is the caller's.
   *  `through` = the caller's own verdict on the round, where it has decided one before calling (the penetration part's toll): true = it went on through, so the glass is opened where it passed — false = it stopped in the glass — undefined = the glass decides by energy (punchE). glass.ts glassImpact wraps this with the star decal, the crack heard and the hearing event. */
  impact(bi: number, X: Vec3, dir: Vec3 | null, energy: number, who: number, through?: boolean): GlassImpact {
    const g = this.g, b = g.level.boxes[bi], isGlass = !!b && (b.flags & BoxFlag.Glass) !== 0;
    const res: GlassImpact = { pane: isGlass, live: false, type: 'annealed', thickness: isGlass ? 2 * Math.min(b.h[0], b.h[2]) : 0, wasOpen: false, punched: false, diced: false, released: 0, cracked: false };
    if (!isGlass) return res;
    if (!glassBreakLive(g)) { res.type = this.typeOfPane(bi) ?? 'laminated'; return res; }   // the switch off, chaos 0, or the tour = the game byte for byte: glass.ts's star and noise are all that happens
    const sh = PANES.both ? this.shade(() => this.twin.impact(bi, X, dir, energy, who, through)) : null, kept: (GlassFragment & { key: number; heard: boolean })[] | undefined = sh ? [] : undefined;
    const v = this.ask((x) => { const r = dir ? x.panes_impact(bi, X[0], X[1], X[2], 1, dir[0], dir[1], dir[2], energy, who, through === undefined ? -1 : through ? 1 : 0) : x.panes_impact(bi, X[0], X[1], X[2], 0, 0, 0, 0, energy, who, through === undefined ? -1 : through ? 1 : 0); this.drain(x, kept); return r; });
    res.live = (v & 1) !== 0; res.wasOpen = (v & 2) !== 0; res.punched = (v & 4) !== 0; res.diced = (v & 8) !== 0; res.cracked = (v & 16) !== 0; res.type = TYPES[(v >>> 5) & 7] ?? 'laminated'; res.released = v >>> 8;
    if (sh) PANES.both!('impact', this.twin, [sh.a, res], sh.fr, kept!);
    return res;
  }
  /** the base's name for a round's impact at energy 1 (probes, the console) */
  hit(bi: number, X: Vec3, dir: Vec3 | null, who: number, energy = 1) { this.impact(bi, X, dir, energy, who); }
  /** A PELLET THROUGH GLASS ITS OWN SHELL HAS JUST BROKEN (glasspen.ts, the shell rule): no blow is filed, but if a piece of an annealed pane still stands where it passed, that piece goes with it. Returns fragments let go. */
  widen(bi: number, X: Vec3): number {
    if (!glassBreakLive(this.g)) return 0;
    const sh = PANES.both ? this.shade(() => this.twin.widen(bi, X)) : null, kept: (GlassFragment & { key: number; heard: boolean })[] | undefined = sh ? [] : undefined;
    const n = this.ask((x) => { const r = x.panes_widen(bi, X[0], X[1], X[2]); this.drain(x, kept); return r; });
    if (sh) PANES.both!('widen', this.twin, [sh.a, n], sh.fr, kept!);
    return n;
  }
  /** A BLAST at `at` (destruction's blast entry point calls this; a stun charge's 'bang' comes through glass.ts at bangPower): every sheet whose nearest point is within the radius takes it by type and by k = power × (1 − d / radius). Returns the panes it changed. */
  blast(at: Vec3, opts: GlassBlastOpts = {}): number {
    if (!glassBreakLive(this.g)) return 0;
    const radius = opts.radius ?? GLASSBREAK.blastM, power = opts.power ?? 1, who = opts.who ?? 0;
    const sh = PANES.both ? this.shade(() => this.twin.blast(at, opts)) : null, kept: (GlassFragment & { key: number; heard: boolean })[] | undefined = sh ? [] : undefined;
    let changed = 0;
    if (radius > 0 && power > 0) this.ask((x, s) => {
      x.panes_blast_begin(at[0], at[1], at[2]);
      for (let i = 0, n = x.panes_sheets(s); i < n; i++) { if (x.panes_blast_sheet(i, at[0], at[1], at[2], radius, power, who)) changed++; if (x.panes_take_wrote()) this.g.col.wrote(); this.drain(x, kept); }
    });
    if (sh) PANES.both!('blast', this.twin, [sh.a, changed], sh.fr, kept!);
    return changed;
  }
  /** a stun charge went off at `at` (glass.ts updateGlass, off the 'bang' event): a blast at bangPower */
  bang(at: Vec3, who: number): number { return this.blast(at, { radius: GLASSBREAK.blastM, power: GLASSBREAK.bangPower, who }); }

  // ---------------------------------------------------------------- a burst's view of the glass (frag.ts: fragments are stopped by glass that stands, and by none that the blast took)
  private viewOf: { kind: 'blown' | 'wary' | 'counted'; from: Vec3; size: GlassBlastOpts } | null = null;
  /** THE PANES THAT ARE NO LONGER WHOLE for the box world's copy, while the rule is the twin's (scene/boxMirror.ts PaneSource) */
  each(to: (p: PaneRow) => void): void { this.twin.each(to); }
  /** where the rule's state stands, while it is the module's: the copy reads it there and copies nothing (boxMirror.ts panes) */
  get home(): SimState | null { return this.g.sim; }
  /** For the length of `fn`, every solid walk and every ray ALSO passes a pane where a burst at `from` of this size sees no glass, by the `view` the burst's judgment is made in (frag.ts GlassView: 'blown' = goingAt, 'wary' = wouldGive of the whole pane, 'counted' = wouldGive at the point met). Nothing is filed and nothing is kept: the view is gone when fn returns (or throws). */
  seeThrough<T>(view: 'blown' | 'wary' | 'counted', from: Vec3, size: GlassBlastOpts, fn: () => T): T {
    this.ask((x) => x.panes_settle_sky());   // (which glass each pane is, settled BEFORE the view is on: a first answer casts solid rays of its own — never from inside a walk, and never through this view)
    const was = this.viewOf; this.viewOf = { kind: view, from, size }; this.g.col.glassViewed();
    const on = () => { try { return fn(); } finally { this.viewOf = was; this.g.col.glassViewed(); } };
    return PANES.both ? this.twin.seeThrough(view, from, size, on) : on();
  }
  /** the view in hand as numbers (scene/boxMirror.ts PaneSource): which view, the burst's point, reach and power, whether a man counts on it */
  get view(): PaneView | null {
    const V = this.viewOf; if (V === null) return null;
    return { kind: V.kind === 'blown' ? VIEW_BLOWN : VIEW_GIVES, bi: [], at: [V.from[0], V.from[1], V.from[2], V.size.radius ?? GLASSBREAK.blastM, V.size.power ?? 1, V.kind === 'counted' ? 1 : 0] };
  }
  /** the rule's whole answer at a point (collision.ts glassOpen): a piece out, or the view in hand counts the glass there out */
  private ruleOpen(bi: number, p: Vec3): boolean {
    if (this.viewOf === null) return this.openAt(bi, p);
    return this.ask((x, _s, w) => x.glass_open(w, bi, p[0], p[1], p[2]) === 1);
  }

  // ---------------------------------------------------------------- per frame
  update(dt: number) {
    const g = this.g, sim = simX(), s = g.sim.bind();
    if (sim.panes_take_gust(s)) this.gustDirty = true;
    if (this.gusts.size || this.gustDirty) { this.updateGusts(); this.gustDirty = this.gusts.size > 0; }
    if (!glassBreakLive(g)) return;   // (as impact / blast: a pane still shedding when the tour starts, the switch goes off or chaos drops to 0 sheds nothing more and files no noise)
    const sh = PANES.both ? this.shade(() => this.twin.update(dt)) : null, kept: (GlassFragment & { key: number; heard: boolean })[] | undefined = sh ? [] : undefined;
    const n = sim.panes_row_n(s);
    if (n > 0) this.ask((x, gs) => {
      x.panes_frame_begin(gs);
      // who may walk into what is left of a pane: any player's man, on his feet with his hands free (seats.ts)
      const M = f64(), at = x.panes_men_addr() / 8; let k = 0;
      for (const pl of men(g)) { if (!pl || pl.down || pl.holding || pl.dragging || pl.wall) continue; const c = pl.char, o = at + 7 * k++; M[o] = c.pos[0]; M[o + 1] = c.pos[1]; M[o + 2] = c.pos[2]; M[o + 3] = c.vel[0]; M[o + 4] = c.vel[1]; M[o + 5] = c.vel[2]; M[o + 6] = c.id; }
      x.panes_men(k);
      for (let r = 0; r < n; r++) { x.panes_step(r); if (x.panes_take_wrote()) g.col.wrote(); this.drain(x, kept); }
    });
    if (sh) PANES.both!('update', this.twin, [sh.a, undefined], sh.fr, kept!);
  }
  /** a pane on an outside wall has changed: the gusts ask again */
  gustDirty = false;
  /** GUSTS through shot-out exterior panes: one wind source per pane more than a quarter open (a jet along the inward normal like an open door's, wind.ts 'glass'), its speed each frame = the storm at that face × the open share — the particles (papers by the window) and the smoke solver read the one field */
  private gusts = new Map<number, WindSource>();
  private updateGusts() {
    const g = this.g; const swv = g.wind.stormWind();   // wind.ts's ONE storm test (weather on, night, WIND.storm, the storm or rain actually running): a dead calm or the daylight breach blows nothing through a hole
    const sw = swv ? [swv[0], swv[2]] : null;
    const stormSpeed = sw ? Math.hypot(sw[0], sw[1]) : 0;
    for (const st of this.panes.values()) {
      if (!st.exterior) continue;
      const open = this.openFraction(st.bi);
      let src = this.gusts.get(st.bi);
      if (open < 0.25 || stormSpeed < 0.05) { if (src) src.base = 0; continue; }
      if (!src) {
        const b = st.box, cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), n: Vec3 = st.uAxis === 0 ? [sn, 0, cs] : [cs, 0, -sn]; const c = b.c;
        const inward: Vec3 = this.ask((x) => x.panes_sky(st.bi)) > 0 ? v3.scale(n, -1) : n;   // away from the face the sky is over (the level's own geometry, as exterior itself)
        const reach = WIND.doorReach, dMax = WIND_FIELD.cutK * reach, tanSpread = Math.tan(WIND.doorSpreadDeg * Math.PI / 180), r0 = st.hu * 0.7;
        const pos: Vec3 = [c[0] - inward[0] * 0.15, c[1], c[2] - inward[2] * 0.15];
        src = { kind: 'jet', why: 'glass', name: `glass:${st.bi}`, pos, dir: inward, speed: 0, base: 0, on: 1, gust: 1, reach, tanSpread, r0, dMax, bound: { c: v3.mad(pos, inward, dMax / 2), r: Math.hypot(dMax / 2, r0 + dMax * tanSpread) + 0.3 }, seed: (0x61A55 ^ Math.imul(st.key, 2654435761)) >>> 0 } as WindSource;
        g.wind.all.push(src); this.gusts.set(st.bi, src);
      }
      const facing = Math.max(0, (sw![0] * src.dir[0] + sw![1] * src.dir[2]) / Math.max(1e-3, stormSpeed));
      src.base = stormSpeed * WIND.doorK * open * (0.35 + 0.65 * facing) * GLASSBREAK.gustSpeedK / 0.6;
    }
  }

  // ---------------------------------------------------------------- queries
  /** is world point X (on / near pane bi) where a piece has fallen out? */
  openAt(bi: number, X: Vec3): boolean { return simX().panes_open_at(this.g.sim.bind(), bi, X[0], X[1], X[2]) === 1; }
  /** the movement rule's answer for pane bi (false for a pane never hit) */
  walkable(bi: number): boolean { return simX().panes_walkable(this.g.sim.bind(), bi) === 1; }
  /** a number that changes whenever what is OPEN in any pane changes — a piece out, a clump down, a sheet gone, every pane whole again (glasspen.ts glassTollSig) */
  openSig(): number { return simX().panes_sig(this.g.sim.bind()); }
  /** open fraction of pane bi (0 = whole) — by area */
  openFraction(bi: number): number { return simX().panes_fraction(this.g.sim.bind(), bi); }
  /** what a sibling needs to know about pane bi without touching it (works for a pane never hit); null = not a sheet of glass */
  paneInfo(bi: number): { type: GlassType; thickness: number; exterior: boolean; openFraction: number; walkable: boolean; gone: boolean; run: boolean; sites: number; energy: number } | null {
    const type = this.typeOfPane(bi); if (!type) return null; const b = this.g.level.boxes[bi]; const st = this.panes.get(bi);
    return { type, thickness: 2 * Math.min(b.h[0], b.h[2]), exterior: this.ask((x) => x.panes_sky(bi)) !== 0, openFraction: this.openFraction(bi), walkable: this.walkable(bi), gone: !!st?.gone, run: !!st?.run, sites: st?.sites.length ?? 0, energy: st?.energy ?? 0 };
  }
  /** THE PANES THAT HAVE BEEN TOUCHED, as a tool or the picture reads them (a copy, made when asked; nothing here is kept) */
  get panes(): Map<number, PaneState> {
    const out = new Map<number, PaneState>(), x = simX(), s = this.g.sim.bind();
    for (let r = 0, n = x.panes_row_n(s); r < n; r++) { const bi = s0Bi(this.g.sim, x, s, r); out.set(bi, this.handle(bi)); }
    return out;
  }
  /** A PANE AS A TOOL HOLDS IT: what it reads is the pane as it stands when it reads it (a pane made whole since: as it last stood) */
  private handle(bi: number): PaneState {
    let last: PaneState | null = null;
    const now = (): PaneState => { const x = simX(), s = this.g.sim.bind(), r = x.panes_row_index(s, bi); if (r >= 0) last = this.rowState(x, s, r); return last!; };
    return new Proxy({} as PaneState, { get: (_t, k) => (now() as unknown as Record<string | symbol, unknown>)[k], has: (_t, k) => k in now(), ownKeys: () => Reflect.ownKeys(now()), getOwnPropertyDescriptor: (_t, k) => Object.getOwnPropertyDescriptor(now(), k) });
  }
  private rowState(x: SimExports, s: number, r: number): PaneState {
    const Fl = fields(x), at = x.panes_row_addr(s, r), d = this.g.sim.d, w = this.g.sim.w, dAt = (f: number, i = 0) => d[((at + Fl[f]) >>> 3) + i], wAt = (f: number, i = 0) => w[((at + Fl[f]) >>> 2) + i];
    const flags = wAt(F_W, GW_FLAGS), ns = flags >>> 8, bi = wAt(F_W, 0), sites: GpSite[] = [];
    for (let i = 0; i < ns; i++) { const q = i * G_SITE; sites.push({ u: dAt(F_D, q), v: dAt(F_D, q + 1), rot: dAt(F_D, q + 2), rho: [dAt(F_D, q + 3), dAt(F_D, q + 4), dAt(F_D, q + 5), dAt(F_D, q + 6)], rz: dAt(F_D, q + 7), merge: dAt(F_D, q + 8), n: wAt(F_W, GW_N + i), seed: wAt(F_W, GW_SEED + i), e: dAt(F_E, i), hits: dAt(F_HITS, i) }); }
    const open = new Uint8Array(GP_MAX_SITES * GP_FRAGS_PER_SITE);
    for (let k = 0; k < open.length; k++) if (wAt(F_W, GW_OPEN + (k >>> 5)) >>> (k & 31) & 1) open[k] = 1;
    const q0 = wAt(F_Q0), qn = wAt(F_QN), qa = x.panes_q_addr(s), qlen = Fl[F_QLEN], pending: Pending[] = [];
    for (let i = 0; i < qn; i++) { const o = qa + (q0 + i) * qlen, cause = CAUSES[w[(o + Fl[F_QCAUSE]) >>> 2]]; pending.push({ fid: w[(o + Fl[F_QFID]) >>> 2], at: d[(o + Fl[F_QAT]) >>> 3], cause, from: cause === 'blast' || cause === 'rake' ? [d[(o + Fl[F_QFROM]) >>> 3], d[((o + Fl[F_QFROM]) >>> 3) + 1], d[((o + Fl[F_QFROM]) >>> 3) + 2]] : null, k: d[(o + Fl[F_QK]) >>> 3] }); }
    let dice: Dice | null = null;
    if (flags & G_DICE) {
      const D: GpDice = { u: dAt(F_D, G_DICE_AT), v: dAt(F_D, G_DICE_AT + 1), cols: wAt(F_W, GW_COLS), rows: wAt(F_W, GW_ROWS), seed: wAt(F_W, GW_DSEED), hu: dAt(F_D, G_DICE_AT + 2), hv: dAt(F_D, G_DICE_AT + 3) };
      const clumps: Clump[] = []; for (let rr = 0; rr < D.rows; rr++) for (let c = 0; c < D.cols; c++) clumps.push({ k: rr * D.cols + c, c, r: rr, key: gpDiceKey(D, c, rr) });
      clumps.sort((a, b) => a.key - b.key || a.k - b.k);
      const next = wAt(F_DNEXT), out = new Uint8Array(D.cols * D.rows); for (let i = 0; i < next; i++) out[clumps[i].k] = 1;
      dice = { ...D, t0: dAt(F_D, G_DICE_AT + 6), clumps, next, out };
    }
    const run = (flags & G_RUN) !== 0, uAxis: 0 | 2 = wAt(F_AXIS) === 0 ? 0 : 2, hu = dAt(F_HU), hv = dAt(F_HV), type = TYPES[wAt(F_TY)];
    return { bi, box: this.g.level.boxes[bi], key: wAt(F_KEY), type, uAxis, hu, hv, ht: dAt(F_HT), exterior: wAt(F_EXT) !== 0, sites, run, open, nOpen: wAt(F_NOPEN), hits: wAt(F_HITN), energy: dAt(F_ENERGY), damage: dAt(F_DAMAGE),
      get frags() { return type === 'annealed' ? gpFragments(sites, run, hu, hv) : []; }, set frags(_v) { /* (made when read) */ }, pending, nextThink: dAt(F_NEXT), dice, gone: (flags & G_GONE) !== 0, lastDir: [dAt(F_DIR), dAt(F_DIR, 1), dAt(F_DIR, 2)], lastWho: dAt(F_WHO), lastUV: [dAt(F_UV), dAt(F_UV, 1)], soundAt: dAt(F_SOUND), walkable: wAt(F_WALK) !== 0, log: [] };
  }
  /** a tool's question: which pieces of this pane are held (a set of piece ids) */
  heldSet(st: PaneState, without = -1): Set<number> {
    const x = simX(), at = x.panes_held(this.g.sim.bind(), st.bi, without) / 4, w = u32(), out = new Set<number>();
    for (let k = 0; k < GP_MAX_SITES * GP_FRAGS_PER_SITE; k++) if (w[at + (k >>> 5)] >>> (k & 31) & 1) out.add(k);
    return out;
  }
  /** pane-local helpers a tool reads with (the frame of a pane in hand) */
  toUV(st: PaneState, X: Vec3): Pt { return this.twin.toUV(st, X); }
  toWorld(st: PaneState, u: number, v: number, w = 0): Vec3 { return this.twin.toWorld(st, u, v, w); }
  normalOf(st: PaneState): Vec3 { return this.twin.normalOf(st); }
  /** a burst's reckoning, for a tool: glassbreak's rule through the view */
  goingAt(bi: number, X: Vec3): boolean { return this.seeThrough('blown', [0, 0, 0], {}, () => this.ruleOpen(bi, X)); }
  wouldGive(bi: number, at: Vec3, opts: GlassBlastOpts = {}, X?: Vec3): boolean {
    if (!glassBreakLive(this.g)) return false;
    return this.seeThrough(X ? 'counted' : 'wary', at, opts, () => this.ruleOpen(bi, X ?? this.g.level.boxes[bi].c));
  }
  /** the record for the renderer (boxes.ts packGlass → common.wgsl gb*): GB_RECORD_ROWS vec4 rows — dressing: reads the pane, writes nothing of it
   *    0: site count · flags (1 something is open, 2 run, 4 dicing, 8 gone, 16 web) · u axis (0 = local x, 1 = local z) · damage 0‥1
   *    1: hu · hv · type (0 annealed, 1 tempered, 2 laminated) · 0          2: the dicing blow's u, v · the open threshold (clumps whose key is under it are out) · grain
   *    3: dice cols · rows · seed · 0
   *    then per site 4 rows: (u, v, rot, rz) · (radials, seed, energy, merge) · the four rho · the open masks (bands 0 | 1 << 12, 2 | 3 << 12, 4 | 5 << 12, 6) */
  fill(bi: number, dst: Float32Array, off: number): boolean {
    const x = simX(), s = this.g.sim.bind(), r = x.panes_row_index(s, bi); if (r < 0) return false;
    const Fl = fields(x), at = x.panes_row_addr(s, r), d = this.g.sim.d, w = this.g.sim.w, dAt = (f: number, i = 0) => d[((at + Fl[f]) >>> 3) + i], wAt = (f: number, i = 0) => w[((at + Fl[f]) >>> 2) + i];
    const flags = wAt(F_W, GW_FLAGS), ns = flags >>> 8, gone = (flags & G_GONE) !== 0, dice = (flags & G_DICE) !== 0; if (ns === 0 && !dice && !gone) return false;
    const next = wAt(F_DNEXT), nOpen = wAt(F_NOPEN), anyOpen = gone || nOpen > 0 || (dice && next > 0), type = wAt(F_TY);
    dst[off] = ns; dst[off + 1] = (anyOpen ? 1 : 0) | ((flags & G_RUN) ? 2 : 0) | (dice && !gone ? 4 : 0) | (gone ? 8 : 0) | (type === 2 ? 16 : 0); dst[off + 2] = wAt(F_AXIS) === 0 ? 0 : 1; dst[off + 3] = dAt(F_DAMAGE);
    dst[off + 4] = dAt(F_HU); dst[off + 5] = dAt(F_HV); dst[off + 6] = type; dst[off + 7] = 0;
    if (dice) {
      const cols = wAt(F_W, GW_COLS), rows = wAt(F_W, GW_ROWS);
      dst[off + 8] = dAt(F_D, G_DICE_AT); dst[off + 9] = dAt(F_D, G_DICE_AT + 1); dst[off + 10] = x.panes_dice_threshold(s, bi); dst[off + 11] = GLASSBREAK.diceGrain; dst[off + 12] = cols; dst[off + 13] = rows; dst[off + 14] = wAt(F_W, GW_DSEED);
    }
    for (let i = 0; i < ns; i++) {
      const q = i * G_SITE, n = wAt(F_W, GW_N + i), o = off + (GB_HEADER_ROWS + i * GB_SITE_ROWS) * 4, m = [0, 0, 0, 0, 0, 0, 0];
      for (let b = 0; b < GP_BANDS; b++) for (let k = 0; k < n; k++) { const fid = i * GP_FRAGS_PER_SITE + b * GP_MAX_SECTORS + k; if (wAt(F_W, GW_OPEN + (fid >>> 5)) >>> (fid & 31) & 1) m[b] |= 1 << k; }
      dst[o] = dAt(F_D, q); dst[o + 1] = dAt(F_D, q + 1); dst[o + 2] = dAt(F_D, q + 2); dst[o + 3] = dAt(F_D, q + 7);
      dst[o + 4] = n; dst[o + 5] = wAt(F_W, GW_SEED + i); dst[o + 6] = Math.min(8, dAt(F_E, i)); dst[o + 7] = dAt(F_D, q + 8);
      dst[o + 8] = dAt(F_D, q + 3); dst[o + 9] = dAt(F_D, q + 4); dst[o + 10] = dAt(F_D, q + 5); dst[o + 11] = dAt(F_D, q + 6);
      dst[o + 12] = m[0] | (m[1] << 12); dst[o + 13] = m[2] | (m[3] << 12); dst[o + 14] = m[4] | (m[5] << 12); dst[o + 15] = m[6];
    }
    return true;
  }
  /** the panel's 'break test': three rounds' worth on the pane on Sam's line to the cursor, through glass.ts's path exactly as rounds would arrive — returns the pane or −1 */
  breakTest(): number {
    const g = this.g; const aim = g.aimPoint; const from: Vec3 = v3.add(g.player.char.pos, [0, 1.3, 0]);
    const d = v3.sub(aim, from); const L = v3.len(d); if (L < 1e-3) return -1;
    const rd = v3.scale(d, 1 / L);
    // (the first round opens the crater, so the next two would fly through it: walk them a hand's width along the pane instead — a burst)
    for (let k = 0, bi = -1; k < 3; k++) {
      const o: Vec3 = v3.add(from, [0, 0.07 * k, 0]); const h = g.col.raycast(o, rd, L + 6);
      if (!h || h.index >= 1e6) return bi;
      const b = g.level.boxes[h.index]; if (!b || !(b.flags & BoxFlag.Glass) || Math.min(b.h[0], b.h[2]) > 0.05) return bi;
      bi = h.index; glassImpact(g, bi, v3.mad(o, rd, h.t), rd, 1, 0);
      if (k === 2) return bi;
    }
    return -1;
  }
  summary(): string {
    const T = this.g.tune as GbTune;
    let s = `glass breaking ${T.glassBreak ? (glassBreakLive(this.g) ? 'ON' : 'on — but inert (chaos 0 / the tour)') : 'off'} ▸ ${this.panes.size} panes touched · ${GB_STATS.impacts} impacts · ${GB_STATS.sites} stars · ${GB_STATS.runs} run · ${GB_STATS.dropped} fragments out (${GB_STATS.clumps} clumps) · diced ${GB_STATS.diced}${this.hook ? ` · ${GB_STATS.hooked} to the shards hook` : ` · stand-in: ${GB_STATS.shards} shards (${GB_STATS.refused} refused), ${GB_STATS.chips} chips`}`;
    for (const st of this.panes.values()) if (st.nOpen || st.dice || st.gone) s += ` | pane ${st.bi} ${st.type} ${Math.round(100 * this.openFraction(st.bi))}% open${st.run ? ' run' : ''}${st.walkable ? ' (walkable)' : ''}`;
    return s;
  }
  reset() {
    if (PANES.both) this.twin.reset();
    const s = this.g.sim; if (s.open) simX().panes_reset(s.bind());
    this.g.col.wrote();
    for (const src of this.gusts.values()) { const i = this.g.wind.all.indexOf(src); if (i >= 0) this.g.wind.all.splice(i, 1); } this.gusts.clear(); this.gustDirty = false;
  }
}
/** row r's static */
function s0Bi(sim: SimState, x: SimExports, s: number, r: number): number { return sim.w[(x.panes_row_addr(s, r) + fields(x)[F_W]) >>> 2]; }
/** a pane's frame, as the host reads it for what it still makes of a fragment */
interface PaneFrame { key: number; uAxis: 0 | 2; hu: number; hv: number; ht: number; box: Box }
