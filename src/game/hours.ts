// THE HOUR: a level started at another time of day than the one it was authored for (the maintainer, 10-08: "can you also try alternate weather and time of day
// for levels? we may want some loud missions to happen at day"). Every level is authored as a NIGHT: a moon, sodium lamps outside, the rooms' own fixtures. An
// hour is a small record laid over that night without touching the level file: a SUN (one directional light — the same kind-2 light the moon is), the SKY the
// bounce and the reflections read, the haze, the exposure the picture is taken at, and which of the place's own lights burn. `night` is no record at all: the
// level exactly as it is, every byte and every dice draw (tools/qa `probes.ts hours` holds it).
//
//   ?hour=dawn | midday | overcast | dusk | night        the page (host/standup.ts addressBeforeGame → hourAsked); the mission sheet's Hour row puts it on the link it loads (ui/missions.ts)
//   applyHour(g, id)                                      live, both ways, any number of times (Tab ▸ Frame ▸ Hour; `__hour.set('dusk')`)
//   Game.stood.sun 0‥1 / setSunStrength(g, k)                   THE ONE NUMBER for whoever has clouds: the sun's strength on the picture and in the game's light
//
// WHAT AN HOUR DOES, all of it reversible (applyHour(g, 'night') puts back exactly what it found — the light list, every switch, the sky, the storm):
//   · the level's moon out, the hour's sun in at the moon's place in the light list (never last: a loud minute's flashes must not push it past the 64 a frame draws)
//   · sky, haze, exposure: engine.settings, as lighting.ts setDaylight writes them for the breach showcase
//   · THE FIXTURES. Outside lamps (group 'exterior': the sodium heads) are on a photocell — they burn at dawn and dusk as authored and are OUT at midday and on
//     an overcast afternoon. Everything indoors is left as authored at every hour: a windowless room is as dark at noon as at midnight and the men who use a room
//     leave its light on. (Re-made after every restart: game.ts resetLevelState → hourAfterReset.)
//   · the night's STORM held off (Game.stood.dry — its own switch, put back with the night). A weather PICKED for the level (weather.ts `kind`) is not the night's
//     storm: it runs at every hour, and its cloud dims this sun (THE WEATHER OVER THE HOUR, below).
//
// ONE MODEL. What an AI eye makes of a man is the light ON him, and the Rust game works that out (rust/game_rules/src/guards light_at; hours sky_now): so by day a man in
// a yard is plain to see from across it, the team's weapon lights go out and its goggles come up outdoors, and a shut room is still a dark room. No AI-only
// shortcut: nothing here reads a side, a state or a difficulty.
//
// SEAMS. Weather: weather.ts calls hourWeatherChanged(g) when its kind changes; this file holds the level's own storm switch and reads Weather.kind / .cloud
// (the stand-in reads the live sky, so the greyed sky is followed). Destruction: a wall that goes changes what the sky reaches — call
// hourWorldChanged(g). Checkpoints carry the hour (savestate.ts `hour`), and with them the session journal (its frame 0 is a checkpoint).
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import { GameSideMap } from '../math/side';
import { Vec3, v3, clamp, smoothstep } from '../math/vec';
import type { Game, LightTarget } from './game';
import type { RtLight } from '../render/lights';
import type { WeatherSnap } from './weather';
import { isDaylight } from './lighting';
import { ENV } from './env';
import { LIGHT_IN } from '../sim/lightshome';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export type HourId = 'night' | 'dawn' | 'midday' | 'overcast' | 'dusk';
/** the order of a day, the authored night last — and the order the sheet's row and the keys walk */
export const HOUR_IDS: HourId[] = ['dawn', 'midday', 'overcast', 'dusk', 'night'];

export interface HourDef {
  id: HourId; label: string;
  /** the local time it stands for (Central America, about 15° north: a high noon sun, short twilights) */
  clock: string;
  /** the sheet's sentence */
  words: string;
  /** the sun: where it stands (bearing 0 = north = −z, 90 = east = +x — weather.ts's convention; elevation above the horizon), its colour, its irradiance on a
   *  face turned to it (the light meter reads 100 % at 4.5), its angular radius (rad: the penumbra — an overcast sky is a very wide sun) and how much it lights
   *  the building's dust (shafts through a door, under a ceiling only: volumetrics.wgsl's rule for directional lights) */
  sun: { bearingDeg: number; elevDeg: number; color: Vec3; intensity: number; radius: number; volumetric: number };
  sky: { zenith: Vec3; horizon: Vec3 };
  haze: number; exposure: number;
  /** the outside lamps burn as authored (a photocell: dawn, dusk) or are out */
  lamps: boolean;
  /** the cloud cover already IN these numbers (the grey day's deck; none = a clear sky): a picked weather's cloud counts only past it (THE WEATHER OVER THE HOUR) */
  cloud?: number;
}
/** The hours to try. Numbers chosen by the meter's arithmetic and by software-rendered stills (rebuild/weather-and-hours/hours): nobody has seen one on a GPU. */
export const HOURS: Record<Exclude<HourId, 'night'>, HourDef> = {
  dawn: { id: 'dawn', label: 'Dawn', clock: '06:40', words: 'early light, with a low orange sun, long shadows across the yard and the lights still on',
    sun: { bearingDeg: 105, elevDeg: 22, color: [1.0, 0.62, 0.38], intensity: 2.6, radius: 0.02, volumetric: 1.2 },
    sky: { zenith: [0.16, 0.22, 0.38], horizon: [0.56, 0.42, 0.34] }, haze: 0.05, exposure: 1.9, lamps: true },
  midday: { id: 'midday', label: 'Midday', clock: '12:30', words: 'a high white sun, with hard short shadows, every yard in plain sight and the rooms dark by contrast',
    sun: { bearingDeg: 195, elevDeg: 56, color: [1.0, 0.97, 0.9], intensity: 6.0, radius: 0.012, volumetric: 0.6 },
    sky: { zenith: [0.3, 0.45, 0.8], horizon: [0.7, 0.76, 0.84] }, haze: 0.03, exposure: 1.7, lamps: false },
  overcast: { id: 'overcast', label: 'Grey day', clock: '15:30', words: 'an overcast afternoon, with flat gray light, almost no shadows and nothing to hide in outdoors',
    sun: { bearingDeg: 240, elevDeg: 50, color: [0.92, 0.95, 1.0], intensity: 1.0, radius: 0.25, volumetric: 0 },
    sky: { zenith: [0.55, 0.58, 0.63], horizon: [0.6, 0.62, 0.65] }, haze: 0.045, exposure: 2.0, lamps: false, cloud: 0.9 },
  dusk: { id: 'dusk', label: 'Dusk', clock: '17:00', words: 'late light, with a low red sun, the lights coming on and the yard half in shadow',
    sun: { bearingDeg: 255, elevDeg: 22, color: [1.0, 0.5, 0.26], intensity: 2.2, radius: 0.02, volumetric: 1.2 },
    sky: { zenith: [0.13, 0.18, 0.36], horizon: [0.6, 0.38, 0.24] }, haze: 0.045, exposure: 2.0, lamps: true },
};
const NIGHT = { label: 'Night', clock: '02:00', words: 'the level as it was built, with the moon, the lights outside and the storm' };
export function hourLabel(id: HourId): string { return id === 'night' ? NIGHT.label : HOURS[id].label; }

/** THE VISIT'S PICK (the page's ?hour=, the harness's PT_HOUR): what a game stood with nothing said for it is stood at (stood.ts stoodOfPick), and what a page's screens
 *  ask. No rule reads it and the bridge does not carry it: a game's own is Game.stood. `sun` is the one number a weather dims under cloud (1 = the hour's own sun; picture and
 *  meter stand-in alike). `dry`: hold the night's storm off while an hour other than night is in. */
export const HOUR: { id: HourId; sun: number; /** `sun` was SET for the visit (?sun=, PT_SUN): it stands whatever cloud a picked weather brings; not set = the cloud dims the sun (THE WEATHER OVER THE HOUR, below) */ sunGiven: boolean; dry: boolean } = { id: 'night', sun: 1, sunGiven: false, dry: true };

const ALIAS: Record<string, HourId> = { day: 'midday', noon: 'midday', afternoon: 'overcast', sunrise: 'dawn', sunset: 'dusk', evening: 'dusk' };
/** a word → an hour ('' / unknown = night) */
export function hourIdOf(v: unknown): HourId { const s = typeof v === 'string' ? v.toLowerCase() : ''; return (HOUR_IDS as string[]).includes(s) ? s as HourId : Object.hasOwn(ALIAS, s) ? ALIAS[s] : 'night'; }
/** what an address asks of the hour: ?hour=<id> (and ?sun=0‥1, the sun's strength) for this visit, written nowhere (host/standup.ts addressBeforeGame): with no flag the level is its own night */
export function hourAsked(q: URLSearchParams): { id: HourId; sun: number; sunGiven: boolean; dry: boolean } {
  const s = q.get('sun'), said = s !== null && Number.isFinite(parseFloat(s));
  return { id: hourIdOf(q.get('hour')), sun: said ? clamp(parseFloat(s), 0, 1) : 1, sunGiven: said, dry: !q.has('noweather') };   // (dry false: the storm is already off for the visit by its own flag, nothing of it to hold, nothing to bring back with the night)
}
/** the hour off a query handed to a RUNNING game (scenarioRun.ts startFromQuery — the harness's way in: a tool that passes 'hour=midday&sun=0.5' beside the scenario's
 *  own flags); no `hour` key = nothing touched, so a start from a plain scenario query leaves the visit's hour where the stand-up put it */
export function hourFromQuery(g: Game, q: URLSearchParams): void {
  if (!q.has('hour')) return;
  applyHour(g, hourIdOf(q.get('hour')));
  const s = q.get('sun'); if (s !== null && Number.isFinite(parseFloat(s))) setSunStrength(g, parseFloat(s));
}
/** a link with the hour on it (night = none: the bare link is the level as it was made) — whatever hour the link carried is replaced */
export function hourHref(href: string, id: HourId = HOUR.id): string {
  const [path, query = ''] = href.split('?'); const keys = query.split('&').filter(k => k && !/^hour(=|$)/.test(k));
  if (id !== 'night') keys.push(`hour=${id}`);
  return keys.length ? `${path}?${keys.join('&')}` : path;
}

// ================================================================================================================================ the hour on a Game
interface HourState {
  toSun: Vec3;
  sun: RtLight; def: HourDef; sunK: number;
  /** sunK was set by hand (?sun=, the panel's slider, a file that says so): a picked weather's cloud leaves it alone */
  given: boolean;
  /** how far a picked weather's cloud has the sun's disc spread and the colour out of the sky (cloudSoft: 0 = the hour's own) */
  soft: number;
  /** the sky's luminance under that cloud, of the hour's own (cloudLight) */
  skyK: number;
  /** the level's own storm was found OFF when the hour came in (the panel, ?noweather): a picked weather leaving puts it off again, and the night does not bring it back */
  wasOff: boolean;
  moons: RtLight[]; lamps: Map<LightTarget, boolean>;
  /** the night's storm as it stood when this hour held it off (weather.ts's own checkpoint slice: its dice, the stroke it had scheduled, thunder still due) — null: not held */
  storm: WeatherSnap | null;
}
const states = new GameSideMap<Game, HourState>('hours.states');
export const SUN_NAME = 'sun', SUN_GROUP = 'hour';
/** the hour this game stands at ('night' = the level as authored) */
export function hourOf(g: Game): HourId { return states.get(g)?.def.id ?? 'night'; }
/** THE EXPOSURE THE GAME'S SCENE IS TAKEN AT NOW (env.ts): the show's day's while it is in (lighting.ts setDaylight), else the hour's own, else the level's
 *  night's (ENV.exposure) — the numbers putIn and setDaylight write to the picture, read here from what the game keeps and never from the picture's
 *  settings, which a player's Brightness turns. Never 0: a sum divides by it. */
export function exposureOf(g: Game): number {
  const e = isDaylight(g) ? g.level.daylight?.exposure ?? ENV.exposure : states.get(g)?.def.exposure ?? ENV.exposure;
  return e > 0 ? e : 1;
}
/** the hour's sun in this game's light set (null at night) */
export function hourSun(g: Game): RtLight | null { return states.get(g)?.sun ?? null; }
export function isHourSun(g: Game, l: RtLight): boolean { return states.get(g)?.sun === l; }
/** IS THIS DIRECTIONAL LIGHT ALREADY IN WHAT THE GAME SAYS OF A POINT (guards.ts lightEstimateAt): the hour's sun, or the night's own light from the sky (`skyLampOf`) */
export function inEstimate(g: Game, l: RtLight): boolean { return isHourSun(g, l) || (!states.get(g) && skyLampOf(g) === l); }
/** THE LEVEL'S OWN LIGHT FROM THE SKY when no hour is laid over it: its brightest steady directional light that is up — the moon of every level as authored,
 *  or the sun a show has put in its place (lighting.ts setDaylight). null: none (the moon turned out; an hour, which has its own sun) */
export function skyLampOf(g: Game): RtLight | null {
  if (states.get(g)) return null;
  let moon: RtLight | null = null, best = 0.002;
  for (const l of g.engine.lights.lights) { if (l.kind !== 2 || !l.enabled || l.ttl >= 0 || l.transient || !(l.dir[1] < 0)) continue; const I = lum(l.color) * l.intensity; if (I > best) { best = I; moon = l; } }
  return moon;
}
/** the sun's strength 0‥1 on this game (1 at night: there is none to dim) */
export function sunStrength(g: Game): number { return states.get(g)?.sunK ?? 1; }
/** was the sun's strength set by hand (?sun=, the panel) rather than left to the weather's cloud */
export function sunGiven(g: Game): boolean { return states.get(g)?.given ?? false; }
/** THE ONE NUMBER, live: the sun at k of the hour's own (0 = behind solid cloud). The picture's light and the meter stand-in's read the same RtLight.
 *  `given` (the default: a hand set it) holds it against a picked weather's cloud; false = a number carried over (a file, the breach's day handing the hour
 *  back) that the cloud may move again. */
export function setSunStrength(g: Game, k: number, given = true) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('setSunStrength', undefined, [g, k, given]); if (__r !== RUST_PASS) return __r as any; } }
  const st = states.get(g); if (!st) return;
  st.sunK = clamp(Number.isFinite(k) ? k : 1, 0, 1); st.given = given; st.sun.intensity = st.sun.peakIntensity = st.def.sun.intensity * st.sunK;
  if (!given) underWeather(g, st);   // (a carried number is the cloud's to move: under the weather this game is in, it is that cloud's now — a file made under another, or before the cloud took the sky's share)
}

// ================================================================================================================================ the weather over the hour
// A PICKED WEATHER (weather.ts `kind`: ?weather=, the sheet's Weather row) and an hour are chosen apart and meet here, in three rules:
//   · THE STORM. A picked weather always runs — rain at midday rains. With none picked the hour keeps the level's own night storm off (Game.stood.dry), as before.
//   · CLOUD → THE LIGHT. Weather.cloud c (0‥1) takes light off the GROUND and moves what is left from the sun to the sky — Kasten & Czeplak's two fits (1980):
//       all of it, on open ground      G(c) = G(0) · (1 − 0.75 c^3.4)        cloudSun      clear 1.00 · dust 0.99 · fog 0.78 · overcast 0.48 · rain 0.37 · downpour 0.25
//       the sky's share of it          d(c) = d0 + (1 − d0) · c²                           (d0 = the hour's own share under its own sky, off its own numbers)
//     so the SUN stands at (1 − c²) × cloudSun(c) of the hour's own — the ONE NUMBER above: midday's is 0.93 in dust, 0.40 in fog, 0.09 under an
//     overcast, 0.04 in rain and NOTHING in a downpour — and the SKY at d(c) ∕ d0 × cloudSun(c) of its own luminance (it brightens as the deck closes: cloud is
//     brighter than blue). Both are the lights the picture is drawn with AND the ones the meter's stand-in reads (the sun's RtLight, the live sky),
//     so they dim together. A sun strength set by hand (?sun=, the slider) stands; the sky still follows the cloud. sunToCloud() hands it back.
//     An hour that is ITSELF a weather (the grey day: HourDef.cloud 0.9) already has that much cloud in its numbers: a picked weather moves it only by what it
//     adds past that (a clear, dusty, foggy or overcast grey day is the grey day; rain and a downpour darken it — c² and cloudSun both taken from the hour's
//     own cover, so level open ground still gets exactly cloudSun(c) ∕ cloudSun(own)).
//   · CLOUD → THE SUN'S DISC AND THE COLOURS. Past half cover (cloudSoft) the sun spreads toward the grey day's wide disc (shadows soften and go), its shafts leave
//     the dust, and the colour drains out of sun and sky toward a deck's grey at the same luminance.
// Night is not touched by any of it: the moon stays as the level has it whatever the cloud (the weather part's 'overcast' at night is 'clear' — a call left open).
// Not touched by day either: the exposure (the hour's; a downpour at noon is a dim picture, as it is a dim day) and the photocell (the lamps follow the hour).
/** the light on open ground under cloud cover c (0‥1), of a clear sky's */
export function cloudSun(c: number): number { return 1 - 0.75 * Math.pow(clamp(c, 0, 1), 3.4); }
/** how far cloud cover c has spread the sun and greyed the sky (0 under half cover, 1 under a solid deck) */
export function cloudSoft(c: number): number { return smoothstep(0.5, 1, c); }
const SOFT_RADIUS = 0.25;   // (the grey day's own disc)
const GREY: Vec3 = [0.948, 1, 1.086], GREY_LUM = 0.2126 * 0.948 + 0.7152 + 0.0722 * 1.086;   // a cloud deck's cool grey (the grey day's own sky)
/** colour c drained toward that grey by w and scaled by k, its luminance otherwise kept */
export function greyed(c: Vec3, w: number, k = 1): Vec3 { const l = lum(c) / GREY_LUM; return [(c[0] + (l * GREY[0] - c[0]) * w) * k, (c[1] + (l * GREY[1] - c[1]) * w) * k, (c[2] + (l * GREY[2] - c[2]) * w) * k]; }
/** the sky's dome in three bands (zenith cap above 70°, six at 50°, eight at 16°, each weighted by its band's solid angle): how high a direction stands, and how much of it
 *  falls on a level face */
const SKY: { y: number; up: number }[] = [];
function skyDome(): void {
  if (SKY.length) return;
  const add = (elev: number, n: number, w: number) => { const y = Math.sin(elev); for (let i = 0; i < n; i++) SKY.push({ y, up: Math.max(0, y) * w }); };
  const s = (deg: number) => Math.sin(deg * DEG);
  add(90 * DEG, 1, 2 * Math.PI * (1 - s(70)));
  add(50 * DEG, 6, 2 * Math.PI * (s(70) - s(32)) / 6);
  add(16 * DEG, 8, 2 * Math.PI * s(32) / 8);
}
/** the sky's share of the light on open ground at an hour AS AUTHORED (its sun on a level face against its sky's dome on it) */
function skyShare(d: HourDef): number {
  skyDome(); const zen = lum(d.sky.zenith), hor = lum(d.sky.horizon); let K = 0;
  for (const q of SKY) K += (hor + (zen - hor) * clamp(q.y * 2.5, 0, 1)) * q.up;
  const S = lum(d.sun.color) * d.sun.intensity * Math.max(0, Math.sin(d.sun.elevDeg * DEG));
  return K + S > 0 ? K / (K + S) : 1;
}
/** [sun, sky] under cover c, each as a factor of the hour's own (1, 1 with no more cloud than the hour has in it) */
export function cloudLight(d: HourDef, c: number): [number, number] {
  const ch = d.cloud ?? 0; c = Math.max(clamp(c, 0, 1), ch);
  if (c === ch) return [1, 1];
  // the share moves from the hour's OWN (at its own cover) to 1 under a solid deck along c²: b = own + (1 − own)·u, u = (c² − ch²) ∕ (1 − ch²) — so the sun's
  // factor (1 − b) ∕ (1 − own) is 1 − u whatever the share (no sun, no sky: nothing divides by it), and sun + sky on level ground is exactly the total's
  const own = skyShare(d), u = (c * c - ch * ch) / (1 - ch * ch), f = cloudSun(c) / cloudSun(ch);
  return [(1 - u) * f, own > 1e-6 ? (own + (1 - own) * u) / own * f : 1];
}
/** the light on level open ground under cover c as a factor of the hour's own (the sun's and the sky's parts together) */
export function cloudTotal(d: HourDef, c: number): number { const ch = d.cloud ?? 0; return cloudSun(Math.max(clamp(c, 0, 1), ch)) / cloudSun(ch); }
function underWeather(g: Game, st: HourState) {
  const W = g.weather, kind = W?.kind ?? null, d = st.def, S = g.engine.settings;
  // the level's own storm: held off while no weather is picked (kept as it stood, to come back with the night); a picked weather is on and stays on
  if (!kind && W?.enabled) { if (st.wasOff) W.setEnabled(false); else if (g.stood.dry) { st.storm ??= W.snapshot(); W.setEnabled(false); } }
  const c = kind ? W!.cloud : 0, [sunK, skyK] = cloudLight(d, c), soft = Math.max(0, cloudSoft(c) - cloudSoft(d.cloud ?? 0));
  if (!st.given) st.sunK = sunK;
  // the fog's veil glows with the frame's sky horizon (common.wgsl wxFogApply) — which the cloud has just BRIGHTENED (fog at midday: sky ×1.97 while the light on
  // the ground is ×0.78). What a parcel of fog scatters is the light that reaches it, so the glow follows the total, not the sky's share: × total ∕ sky.
  if (W) W.fogGlow = skyK > 1e-6 ? cloudTotal(d, c) / skyK : 1;
  st.sun.intensity = st.sun.peakIntensity = d.sun.intensity * st.sunK;
  if (soft === st.soft && skyK === st.skyK) return;
  st.soft = soft; st.skyK = skyK;
  st.sun.color = greyed(d.sun.color, soft); st.sun.volumetric = d.sun.volumetric * (1 - soft);
  const r = d.sun.radius + (Math.max(d.sun.radius, SOFT_RADIUS) - d.sun.radius) * soft; if (r !== st.sun.radius) { st.sun.radius = r; g.engine.reach?.sun(st.sun, r); }
  S.skyZenith = greyed(d.sky.zenith, soft, skyK); S.skyHorizon = greyed(d.sky.horizon, soft, skyK);
}
/** weather.ts Weather.setKind, last: the weather under a running game changed — the hour's storm switch, sun and sky follow (nothing at night) */
export function hourWeatherChanged(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('hourWeatherChanged', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } const st = states.get(g); if (!st) return; underWeather(g, st); g.engine.invalidateWorldCache?.(); }
/** a sun strength set by hand (the slider, ?sun=) handed back to the weather's cloud */
export function sunToCloud(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sunToCloud', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } const st = states.get(g); if (!st) return; st.given = false; underWeather(g, st); }

const lum = (c: Vec3) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
const DEG = Math.PI / 180;
function toSunOf(d: HourDef): Vec3 { const e = d.sun.elevDeg * DEG, b = d.sun.bearingDeg * DEG; return v3.normalize([Math.sin(b) * Math.cos(e), Math.sin(e), -Math.cos(b) * Math.cos(e)]); }

/** Lay hour `id` over the game, or take it off ('night'). Idempotent; any hour → any hour. True when something changed. */
export function applyHour(g: Game, id: HourId): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('applyHour', undefined, [g, id]); if (__r !== RUST_PASS) return __r as any; } }
  const cur = states.get(g);
  if ((cur?.def.id ?? 'night') === id || !(id === 'night' || Object.hasOwn(HOURS, id))) return false;
  if (isDaylight(g)) return false;   // the breach showcase's own day is in (lighting.ts setDaylight took the hour out and puts it back): two days are not laid over each other
  if (cur) takeOut(g, cur);
  if (id !== 'night') putIn(g, HOURS[id], cur);   // (hour → hour: a strength set by hand stays where it was; left to the weather, it is the cloud's over the new hour)
  g.engine.invalidateWorldCache?.();   // the light rig swapped wholesale: the radiance cache snaps to the hour instead of blending toward it (lighting.ts setDaylight's reason; a stand-in engine has none)
  return true;
}
function putIn(g: Game, def: HourDef, from?: HourState) {
  const S = g.engine.settings, L = g.engine.lights, toSun = toSunOf(def), W = g.weather, kind = !!W?.kind;
  const st: HourState = { def, toSun, sunK: from ? from.sunK : clamp(g.stood.sun, 0, 1), given: from ? from.given : g.stood.sunGiven || g.stood.sun !== 1, soft: 0, skyK: 1,
    wasOff: kind ? from?.wasOff ?? false : !!W && !W.enabled, moons: [], lamps: new Map(), storm: kind ? from?.storm ?? null : null,
    sun: null as unknown as RtLight };
  // the moon out (every steady directional light of the level: it has no switch of its own), the sun in where the moon stood
  let at = 0;
  L.lights.forEach((l, i) => { if (l.kind === 2 && l.ttl < 0 && !l.transient && l.enabled) { l.enabled = false; st.moons.push(l); at = i + 1; } });
  const I = def.sun.intensity * st.sunK;
  st.sun = { name: SUN_NAME, group: SUN_GROUP, kind: 2, pos: [0, 0, 0], dir: v3.scale(toSun, -1), color: v3.copy(def.sun.color), intensity: I, range: 1e6, innerDeg: 0, outerDeg: 0, radius: def.sun.radius,
    volumetric: def.sun.volumetric, owner: 0, enabled: true, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: I, broad: false };
  L.lights.splice(at, 0, st.sun);
  S.skyZenith = v3.copy(def.sky.zenith); S.skyHorizon = v3.copy(def.sky.horizon); S.hazeDensity = def.haze; S.exposure = def.exposure;
  states.set(g, st);
  underWeather(g, st);   // the storm held off — or a picked weather kept, with its cloud over this sun and sky
  g.engine.reach?.sun(st.sun, st.sun.radius);   // which indoor cells see the sky its way (the direct estimator's candidate domain): the picture's to keep (render/reachKeep.ts), asked once, for the disc the cloud left it
  lampsOut(g, st);
}
/** what the picture's settings hold on a level's own night (src/settings.ts, held equal by tools/qa/settings-check.ts) */
export const NIGHT_HAZE = 0.035;
/** THE PICTURE TOLD THE LEVEL'S OWN NIGHT, once the hour or the show's day taken off was at this exposure: the level's sky, the night's haze and exposure, with a turn of the player's Brightness under it
 *  carried back (untouched = exactly the night's). What the settings held before is the picture's, so the game keeps none of it. */
export function nightPicture(g: Game, wasExposure: number): void {
  const S = g.engine.settings; S.skyZenith = v3.copy(g.level.sky.zenith); S.skyHorizon = v3.copy(g.level.sky.horizon); S.hazeDensity = NIGHT_HAZE; S.exposure = ENV.exposure * (S.exposure / wasExposure);
}
function takeOut(g: Game, st: HourState) {
  for (const [t, was] of st.lamps) if (!t.on) g.lightHome.in(LIGHT_IN.on, t, was ? 1 : 0);   // (a lamp somebody switched ON under the hour — the perimeter's call for the lot lights — stays as they left it)
  for (const l of st.moons) l.enabled = true;
  g.engine.lights.remove(st.sun);
  nightPicture(g, st.def.exposure);
  states.delete(g); if (g.weather) g.weather.fogGlow = 1;
  if (st.storm && g.weather && !g.weather.enabled) { g.weather.setEnabled(true); g.weather.restore(st.storm); }   // the storm back where it stood: its dice, its next stroke, the thunder it still owed
}
/** the photocell: with the lamps out for the hour, every outside fixture's switch recorded as it stands and opened */
function lampsOut(g: Game, st: HourState) {
  if (st.def.lamps) return;
  for (const t of g.targets) if (t.group === 'exterior' && t.kind !== 'breaker') { st.lamps.set(t, t.on); g.lightHome.in(LIGHT_IN.on, t, 0); }
}
/** game.ts resetLevelState, last: the level's switches are back as authored — the hour's are made again over them, and what the sky reaches is asked afresh
 *  (the destruction's hidden statics are back) */
export function hourAfterReset(g: Game) { const st = states.get(g); if (!st) return; st.lamps.clear(); lampsOut(g, st); }
/** the statics changed under a running game (a wall gone, a cover knocked flat): what the sky reaches is asked afresh */
export function hourWorldChanged(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('hourWorldChanged', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } }

/** one line for the panel / `__hour.summary()` */
export function hourSummary(g: Game): string {
  const st = states.get(g); if (!st) return 'night (the level as authored)';
  const d = st.def, out = [...st.lamps.keys()].length;
  return `${d.label} ${d.clock} · sun ${d.sun.elevDeg}° up at ${(st.sun.intensity).toFixed(2)} (×${st.sunK.toFixed(2)}${st.given ? ' set' : g.weather?.kind ? ` under ${g.weather.kind}'s cloud` : ''}${st.skyK !== 1 ? `, sky ×${st.skyK.toFixed(2)}` : ''}) · exposure ${g.engine.settings.exposure.toFixed(2)} · outside lamps ${d.lamps ? 'as authored' : `out (${out})`}${st.storm ? ' · storm held' : ''}`;
}
