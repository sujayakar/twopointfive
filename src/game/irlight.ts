// AN IR LIGHT ON THE GOGGLES (the maintainer, 10-15, of a depot whose rooms were flat black noise through the tube: "maybe we should model IR lights on the nvg
// so even totally dark rooms are visible. and we can assume enemies never have nvg"). A tube amplifies, it does not invent (nvg.ts): a room with no light of its
// own showed nothing through it. So the goggles carry what real ones carry — an infrared illuminator: a flood of light only a tube sees by.
//
//   WHO CARRIES ONE, AND WHEN IT BURNS. Sam, on the frames whose picture is the tube's (Player.nvAmount past 0.5, post.wgsl's own swap: it comes up with the
//     picture and goes with it — no bare-eyed frame has it) and, under IRLIGHT.auto below, only where his place is too dark for the tube. On his knee at a
//     door with the wand under it (wand.ts WAND.nv) the flood is the lens's. A teammate (IRLIGHT.team; a man of nvg.ts: an operative unit of a scenario
//     whose card left him his goggles), whenever HIS are down (nvg.ts nvgDown). No key, no order: it is part of the goggles. Nobody else — an enemy has no night
//     vision and no infrared anything.
//   ONLY WHERE IT IS TOO DARK (IRLIGHT.auto; the maintainer, 10-16, of a moonlit lot under five floods: "IR lights are a bit too bright. is it realistic to
//     only have them turn on when it's actually too dark to see?" — it is: a tube works on the light that is there first, and the illuminator is for where
//     there is none). Each carrier reads his own place: the floor under him, and four times a second a fan of floor across the flood's own width as far as
//     the first wall each way (fanDark) — the place's own light there (nvg.ts nvgPlaceLight: the fixtures and the moon, nothing anybody carries, nothing
//     infrared) taken through the tube (tubeVis). Any of it under NVG.blackAt — the line nvg.ts already calls 'too dark for the tube' — and his light burns,
//     from that reading until IRLIGHT.hold seconds after the last dark one (a look swept past a dark doorway does not blink it), then eases out.
//     Everywhere the tube shows the place by its own light the illuminator is out: a moonlit yard, a lit corridor. Off = it burns whenever the goggles are
//     down, as it did.
//   BY THE LIGHT THE PICTURE HAS (IRLIGHT.meter; the maintainer, 10-18, of a strongroom with its bulb off, its floor under his flood and the room round it
//     plain in the goggles: "still feels like the IR light here is unnecessary"). The estimate above is the CPU's: direct light only. A room with no lamp
//     of its own is 0 to it, and yet the tube — which makes a picture of a thousandth of a lit room's light — shows that room by what comes round the door
//     frame and off the walls. The renderer's light meter has that bounce (probe.wgsl: shadowed direct light + the radiance cache) and is given nothing by
//     an infrared light, so it still cannot answer its own question. So the CPU's rule stays the FIRST word — where it calls a place light nothing more is
//     asked — and the renderer gets the LAST one on what the CPU calls dark: the floor under him (Sam's and a teammate's own meters, asked every frame as
//     it is; the wand's lens asks its own) and a spread of up to IRL_METER.slots of the dark points his fan found, asked through the sensors' seam
//     (Game.lightQuery) on his beat and read every frame. A point is dark when the PICTURE of it is under IRLIGHT.show of the tube's range: the CPU's
//     direct light + the meter's bounce × settings.indirectScale (the meter's bounce is physical; the picture's is scaled — fgather.wgsl) through the
//     tube's curve (nvg.ts tubeSig). A line of its own, and higher than NVG.blackAt: that one is where a MAN is lost to a goggled eye, about 0.035 of
//     the range, and direct light is either well over it or nothing; bounce is every value between, and floor at 0.05 of the range is grain to the
//     player (the strongroom with its door shut, on the software adapter). Under this branch the CPU's first word is asked by the same line. The same
//     count decides: the floor under him, or FAN_NEED of the points. A point not yet answered waits IRL_METER.wait (no light: a light that came on by
//     the CPU's word and went out by the renderer's two frames on would blink for its whole hold every time the goggles came down in such a room), then
//     takes the CPU's word. Where no renderer answers at all — headless, the senses on the CPU (sense.ts), a page before its first frame: Sam's own
//     meter has no answer — the CPU's rule is the whole rule, at once, exactly as it was. So his light burns in a room that is sealed and unlit, and
//     not in one the picture already shows. Off = the CPU's estimate decides everywhere, as it did.
//   WHAT IT IS. A real light: a wide spot at his brow along the way he faces, tipped a little to the floor, shadowed by every wall, leaf and body like any other.
//     Its strength is said as what the TUBE makes of it (IRLIGHT.power: on-axis intensity × the game's own exposure for the hour, hours.ts exposureOf — the tube counts exposed luminance, so the
//     flood reads the same through a man's tube at every hour, and a player's Brightness moves it in his picture with everything else), not as a wattage.
//   WHO SEES BY IT. Only a tube. The lights are kept in a list of their own (render/lights.ts LightSet.ir), never in the list every other reader sums, and are
//     handed to the renderer flagged infrared: uploaded only on a frame whose picture IS the tube's, where each is a steady light like a torch (the frame
//     the tube comes off, the engine restarts every screen history: engine.ts), and given NOTHING by
//     the light meter (probe.wgsl → common.wgsl evalLightTrue) or the cascades its bounce comes from. So an enemy's eye, a camera's lens, Sam's own light meter
//     and every CPU light estimate read the room as it is without them — dark — by construction: the HUD's LIGHT bar stays where it was while the room shows.
//     A TEAMMATE'S TUBE reads them (IRLIGHT.team; nvg.ts nvgSight asks irOnMan for the man he looks at): a man standing in an illuminator's flood is lit to a
//     goggled eye and to no other. And a man who carries one is never 'too dark for the tube' (nvg.ts readPlace's `black`: the white-light fallback that file
//     kept for a room with no light at all retires — his goggles now show him the way everywhere he can face: under IRLIGHT.auto his light comes on for
//     the dark floor his fan finds, which reaches past everything readPlace asks along his gun).
//
// IRLIGHT.on false = no such light anywhere: the picture and the run of the build before this file. IRLIGHT.team false = Sam's alone, in the picture alone —
// nothing in the simulation reads it, so the run is the one before this file, digest for digest (tools/qa/irlight-probes.ts holds both).
// Nothing is saved: the list is made again every frame from who has his goggles down, and a carrier's reading of his place (IRLIGHT.auto: when it last read
// dark, how far his light has eased) is kept per Game and starts again after a load — a light inside its hold at the save is out or full at once. UNTUNED BY EYE: nothing here was seen on a GPU.
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import { hostMan } from './seats';
import { GameSideMap } from '../math/side';
import { Vec3, clamp, DEG } from '../math/vec';
import type { RtLight } from '../render/lights';
import type { Game, Guard } from './game';
import './nvg';
import './wand';
import './sense';
import './hours';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

export const IRLIGHT = {
  /** off = no infrared light on anybody's goggles (the page turns it on: src/host/profile.ts; ?noirlight) */
  on: false,
  /** the teammates carry one too, and a goggled teammate sees a man standing in anybody's (off = Sam's only, a picture and nothing else) */
  team: true,
  /** it burns only where the goggles alone show too little (the page turns it on: src/host/profile.ts; ?noirauto). off = whenever the goggles are down */
  auto: false,
  /** auto: the place is read off the renderer's light meter, bounced light and all, where it answers (the page turns it on: src/host/profile.ts;
   *  ?noirmeter). off = by the CPU's estimate of the direct light alone, to which a room lit only round a door frame is black */
  meter: false,
  /** meter: the least the tube's picture must make of a floor (0‥1 of its range) for his light to stay out. The maintainer's strongroom read about 0.3 in
   *  his shot; 0.05 is grain. Never judged on a real GPU */
  show: 0.1,
  /** auto: s the light stays on after the last reading that found floor too dark for the tube (NVG.blackAt, by the place's own light) */
  hold: 1.5,
  /** how bright the flood reads THROUGH THE TUBE: on-axis intensity × the picture's exposure. 8 (it was 30, and five of them at once put a lot at the tube's
   *  ceiling): by the shader's own curve one light puts the floor two metres ahead at about 0.6 of the tube's range, four metres at 0.4, a wall six metres
   *  off a little under half — and where all five of a stacked squad's overlap, four metres off reads about 0.6 where it read white */
  power: 8,
  /** m it reaches (the shader's range window: full to about half of this, gone at it) */
  reach: 22,
  /** the flood's half angle, degrees, and where inside it the full strength ends (20, was 40: the edge rolls off over most of the cone) */
  cone: 65, core: 20,
  /** how far below level it points, degrees (6, was 11: less of it on the floor at his feet, which is nearest the lens and so the hottest thing in it) */
  dip: 6,
  /** NO HOTTER THAN IT NEED BE (the maintainer, 10-22, the stealth lesson, stood at the code pad a pace from its wall, the wall white: "i still think the IR
   *  light is a bit too bright here. ideally it's just enough to be able to see" · "but i like how it's realistic"). The flood falls off by the square of the
   *  distance, so whatever stands a pace from the lens takes fifty times what a wall five metres off takes, and through the tube that is white whatever
   *  `power` says (the tube's curve is a logarithm: halving `power` takes a ninth of its range off). So the light turns ITSELF down, as a set with an
   *  automatic gain does: by the nearest thing the middle of its flood lands on inside `nearM`, by the square of that distance — that thing then reads as
   *  it would `nearM` off and no brighter (a wall face on at 5 m: about half the tube's range at power 8) — never under `nearMin` of its strength; down at
   *  `nearDown` a second, up again at `nearUp`. In the open, nothing inside `nearM`: full strength, as it was. The page turns it on (src/host/profile.ts;
   *  `?irnear=0`, or `?irnear=<metres>` for another distance). false = full strength whatever is in front of it, as it was. Never judged on a real GPU */
  near: false, nearM: 5, nearMin: 0.003, nearUp: 3, nearDown: 12,
};
/** ?irpower=<n> · ?irnear=<metres> — the light's strength and the distance it turns down inside, for a visit, (the Game's
 *  constructor; no page — the harness — nothing) */
export function irFlags() { try { const q = new URLSearchParams(location.search);
  const p = parseFloat(q.get('irpower') ?? ''); if (Number.isFinite(p) && p > 0) IRLIGHT.power = Math.min(100, p);
  const n = parseFloat(q.get('irnear') ?? ''); if (Number.isFinite(n) && n > 0) IRLIGHT.nearM = clamp(n, 0.5, 20); } catch { /* no page */ } }

/** one carrier's reading of his place (IRLIGHT.auto): when his fan was last read, when anything last read dark, how much of his light burns, and as of when */
/** …and under IRLIGHT.meter: `fresh`: no reading of his has been decided yet (the first one that is puts his light at full or out at once) · `t0`: when
 *  the reading began · `own`: the CPU's light on the floor under him where it calls that dark (−1: it does not) · `pts`: the dark points of his fan the
 *  renderer is asked about, x, z and the CPU's light at each · `since`: when each slot was first asked (Infinity: not in use; the last is the wand's own floor) */
interface Need { readT: number; darkT: number; k: number; t: number; fresh: boolean; t0: number; own: number; pts: number[]; since: number[] }
interface Kit { sam: RtLight | null; /** a later seat's man's, by his id (seats.ts) */ more: Map<number, RtLight>; men: Map<Guard, RtLight>; seen: Map<Guard, { t: number; E: number }>; need: Map<number, Need>;
  /** IRLIGHT.near: how far each carrier's light is turned down for what stands in front of it (1 = not at all), and as of when */
  near: Map<number, { t: number; k: number }> }
const kits = new GameSideMap<Game, Kit>('irlight.kits');
const kitOf = (g: Game): Kit => { let K = kits.get(g); if (!K) { K = { sam: null, more: new Map(), men: new Map(), seen: new Map(), need: new Map(), near: new Map() }; kits.set(g, K); } return K; };

/** THE RENDERER'S WORD ON WHAT THE CPU CALLS DARK (IRLIGHT.meter; see the head). `slots`: the most fan points one carrier asks about on a beat (the probe
 *  has 64 places a frame for everybody) · `wait`: s an unanswered point is waited for before the CPU's word stands */
export const IRL_METER = { slots: 5, wait: 0.25 };

/** his lamp is out of the list: the renderer's slot memory of it goes with it (lights.ts upload maps last frame's slot to this frame's by the object) */
const out = (L: RtLight | null | undefined) => { if (L) { L.enabled = false; L.slot = -1; } };

/** (QA) how far a carrier's light stands turned down this frame — 1: not at all, or no reading of his this frame */
export function irNearOf(g: Game, key: number): number { const s = kits.get(g)?.near.get(key); return s && s.t === g.time ? s.k : 1; }

/** THE INFRARED LIGHT AT A POINT, W/m² as the light meter would report it if it read infrared (its best axis face; the shader's own terms — common.wgsl
 *  evalLightRaw: 1 / max(d², r²), the range window, the cone's smoothstep — shadowed by one segment test over the statics and the leaves, as guards.ts
 *  lightEstimateAt shadows the lights the meter does read). `skip`: a carrier's own light is not asked (a man is not lit by the lamp on his own brow). */
export function irLightAt(g: Game, P: Vec3, skip?: number): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('irLightAt', undefined, [g, P, skip]); if (__r !== RUST_PASS) return __r as any; } }
  let E = 0;
  for (const l of g.engine.lights.ir) {
    if (!l.enabled || l.intensity <= 0 || (skip !== undefined && l.owner === skip)) continue;
    const dx = l.pos[0] - P[0], dy = l.pos[1] - P[1], dz = l.pos[2] - P[2], d2 = dx * dx + dy * dy + dz * dz, d = Math.sqrt(d2);
    if (d > l.range || d < 1e-3) continue;
    const Lx = dx / d, Ly = dy / d, Lz = dz / d, cd = -(Lx * l.dir[0] + Ly * l.dir[1] + Lz * l.dir[2]), cO = Math.cos(l.outerDeg * DEG), cI = Math.cos(l.innerDeg * DEG);
    if (cd < cO) continue;
    const x = d / l.range, w = clamp(1 - x * x * x * x, 0, 1), s = clamp((cd - cO) / Math.max(cI - cO, 1e-6), 0, 1);
    const e = l.intensity * w * w / Math.max(d2, l.radius * l.radius) * s * s * (3 - 2 * s) * Math.max(Math.abs(Lx), Math.abs(Ly), Math.abs(Lz));
    if (e < 0.002) continue;
    const tmax = d - l.radius - 0.02; if (tmax > 0.05 && g.col.segmentBlocked(P, [P[0] + Lx * tmax, P[1] + Ly * tmax, P[2] + Lz * tmax])) continue;
    E += e;
  }
  return E;
}
/** the infrared light ON a man this frame, as a goggled teammate's tube takes it (nvg.ts nvgSight adds it to the light the meter reads on him): 0 with the
 *  switch or its team half off. Asked once a man a frame (many eyes ask of one man); the lights are last frame's, as the meter's own answer is. */
export function irOnMan(g: Game, o: Guard): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('irOnMan', undefined, [g, o]); if (__r !== RUST_PASS) return __r as any; } }
  if (!IRLIGHT.on || !IRLIGHT.team || !g.engine.lights.ir.length) return 0;
  const K = kitOf(g), had = K.seen.get(o); if (had && had.t === g.time) return had.E;
  const c = o.char, E = irLightAt(g, c.bones.chest ?? [c.pos[0], c.pos[1] + 1.2, c.pos[2]], c.id);
  K.seen.set(o, { t: g.time, E }); return E;
}
/** he carries an illuminator of his own (a teammate with goggles, the team half on): nvg.ts asks before it calls a place too dark for his tube */
export function irCarried(): boolean { return goneWithTick('irCarried') as any; }

/** QA / the panel: whose illuminator burns */
export function irLightState(g: Game): { sam: boolean; men: Guard[]; shown: boolean } {
  const list = g.engine.lights.ir, K = kits.get(g);
  return { sam: !!K?.sam && list.includes(K.sam), men: K ? [...K.men].filter(([, L]) => list.includes(L)).map(([gd]) => gd) : [], shown: hostMan(g).nvAmount > 0.5 };
}
/** 'IR light off: …' / 'IR light: yours on · Blue One · Blue Two — nobody without goggles sees it' */
export function irLightSummary(g: Game): string {
  if (!IRLIGHT.on) return 'IR light off: a room with no light of its own is black through the goggles';
  const S = irLightState(g);
  return `IR light: yours ${S.sam ? 'on' : S.shown && IRLIGHT.auto ? 'off (light enough here)' : 'off (goggles up)'}${IRLIGHT.team ? (S.men.length ? ' · ' + S.men.map(m => m.callsign).join(' · ') : (IRLIGHT.auto ? ' · no teammate in the dark' : ' · no teammate on goggles')) : ' · the team carries none'} — only goggles see it`;
}
