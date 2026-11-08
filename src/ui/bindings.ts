// Panel contents: binds engine / game / smoke / camera settings to UI controls — the game's ONE tweak panel (the debugger, on backtick), laid out for ui/panel.ts v2:
// SYSTEMS down the rail: the Sandbox and the difficulty, the world (Weather & wind), the picture's pipeline in order (Frame · Direct · Indirect · Volumetrics · Materials ·
// Denoiser · Renderer fx), then the tools (Audio · Checkpoints); the game's own tuning pages went on 2026-11-07. On each page SECTIONS made of the old flat groups split
// each with a live one-line summary in its header and a '↺' where a shipped-defaults table exists. Rows built inside adv(…) are A/B-only knobs: out of the
// way until 'all knobs'. Where a knob TABLE is bound row after row the same way (get / set that key, save, the shipped value for the revert dot) the rows are
// a declarative spec through knobRows(); anything with logic of its own stays written out.
import { rowWeather, rowOfQuery } from '../game/rowset';
import { WEATHER_KINDS, weatherSearch, type WeatherKind } from '../game/weatherKinds';
import { keepSandbox } from '../host/sandbox';
import { HOUR_IDS, hourLabel, hourOf, applyHour, hourSummary, sunStrength, setSunStrength, sunToCloud } from '../game/hours';
import { Engine } from '../engine';
import { Game } from '../game/game';
import { SmokeSystem } from '../smoke/smoke';
import { FollowCamera } from '../game/camera';
import { Overlay } from './overlay';
import { Panel } from './panel';
import { SYS, buildGraphics, graphicsSystems, type HudStats } from './graphicspanel';
import { rowHeld } from '../host/held';
import { AudioEngine } from '../audio/audio';
import { PIECE_NAMES, MUSIC } from '../audio/pieces';
import { Quality } from './quality';
import { MORALE, TRAININGS, NERVES, moraleSummary, moraleRoster, moraleKeep } from '../game/morale';
import { runnerSummary } from '../game/runner';
import { darkDoorSummary } from '../game/darkdoor';
import { netQuietSummary } from '../game/netquiet';
import { sprayTestPattern, sprayRandom } from '../game/marks';
import { testBurst, PFX } from '../game/particleFx';
import { WET, WET_DEFAULTS, saveWet, resetWet, soak as wetSoak, dryAll as wetDryAll, everyCharacter as wetEveryone } from '../game/wet';
import { PRAIN, PRAIN_DEFAULTS, PRAIN_V1_LOOK, savePrain, resetPrain, setPrain } from '../game/rainFx';
import { WIND, WIND_DEFAULTS, saveWind, resetWindKnobs } from '../game/wind';
import { MOTHS } from '../game/moths';
import { canSave, CKPT, SAVETIMELINE } from '../game/savestate';
import { squadsLive, squadsWhere } from '../game/enemysquads';
import { DIFFICULTIES, difficultyNow, setDifficulty, difficultyWords } from '../game/difficulty';
import { STRESS, standExtra, stressSummary } from '../game/stress';
import { FIXEDSTEP } from '../host/loop';
import { TWEEN } from '../host/tween';
import { saveFile, savedWords, savedEnd } from '../host/savefile';
import { pageSay } from '../view/pagesaid';
import { senseTool } from '../game/sense';

/** One row of a knob table bound MECHANICALLY: `k` a (dotted) key into the table; a slider over [min, max] by `step` (or a toggle: kind 'toggle'), get / set that
 *  value, the table's save after every set, the shipped value off the defaults table (the revert dot) unless `def: false`; `adv` = an A/B-only row. */
interface Knob { k: string; label: string; kind?: 'toggle'; min?: number; max?: number; step?: number; title?: string; adv?: boolean; def?: boolean }
type KnobTable = Record<string, unknown>;
const leafOf = (o: KnobTable, path: string[]): KnobTable => path.reduce<KnobTable>((x, p) => x[p] as KnobTable, o);
/** rows from a spec over `table` (see Knob): the same rows the hand-written lines built — same labels, ranges, tooltips, save calls, revert dots — only declared */
function knobRows(panel: Panel, sec: HTMLElement, table: KnobTable, defs: KnobTable | null, save: () => void, spec: Knob[]) {
  for (const r of spec) {
    const path = r.k.split('.'); const leaf = path.pop()!; const at = () => leafOf(table, path);
    const host = path.reduce<KnobTable | undefined>((x, p) => x?.[p] as KnobTable | undefined, table);
    if (!host || !(leaf in host)) { console.error(`panel: '${r.label}' is a row over ${r.k}, and its table has no such key: no row built`); continue; }
    if (rowHeld(host, leaf)) continue;   // (a settled row, held at one value: a box or a slider there could not move — host/held.ts)
    const def = defs && r.def !== false ? leafOf(defs, path)[leaf] : undefined;
    const was = panel.advanced; if (r.adv) panel.advanced = true;
    if (r.kind === 'toggle') panel.toggleBox(sec, r.label, () => !!at()[leaf], v => { at()[leaf] = v; save(); }, r.title, def as boolean | undefined);
    else panel.slider(sec, r.label, r.min!, r.max!, r.step!, () => at()[leaf] as number, v => { at()[leaf] = v; save(); }, r.title, def as number | undefined);
    panel.advanced = was;
  }
}

export function buildPanel(panel: Panel, engine: Engine, game: Game, smoke: SmokeSystem, cam: FollowCamera, overlay: Overlay, audio: AudioEngine, quality: Quality, hud?: HudStats) {
  const S = engine.settings;
  const flag = (sec: HTMLElement, label: string, f: number, title?: string) => panel.toggleBox(sec, label, () => (S.flags & f) !== 0, v => { S.flags = v ? S.flags | f : S.flags & ~f; }, title);
  /** rows built inside are ADVANCED (A/B knobs, measurement switches): hidden until 'all knobs' or a search finds them */
  const adv = (f: () => void) => { const was = panel.advanced; panel.advanced = true; try { f(); } finally { panel.advanced = was; } };
  const W_ = window as unknown as { __readout?: { on: boolean; set(v: boolean): void }; __taa?: { summary?: () => string }; __sharp?: { summary?: () => string }; __sdfrig?: { summary?: () => string }; __dn?: { summary: () => string }; __quality?: { motionCalm?: boolean; applyMotionCalm(v: boolean): void }; __save?: { load: (k: string) => string | null } };
  // THE RAIL, in its order (a system named again later only re-selects its page): debug actions and the difficulty first, then the world's systems, then the picture's pipeline, then the tools.
  // A section lands on the page that is current where its code stands: the panel.system() lines below file each one (a new section goes under the nearest one above it)
  panel.system(SYS.sand, { icon: '⚑', short: 'Sandbox', title: 'debug actions: mains · smoke · teleport · god mode / AI / ammo · spectate · extra enemies' });
  panel.system(SYS.missions, { icon: '◎', short: 'Missions', title: 'difficulty · the room fights\' staging · room clear · hostage rescue · fog of war' });
  panel.system(SYS.wx, { icon: '☂', short: 'Weather', title: 'weather · particle rain · wind (+ its field view) · moths · wet characters' });
  graphicsSystems(panel);
  panel.system(SYS.audio, { icon: '♪', short: 'Audio', title: 'mixer volumes · music tension' });
  panel.system(SYS.ckpt, { icon: '⟲', short: 'Saves', title: 'checkpoints + autosave' });

  // ================================================================ THE GRAPHICS PAGES (ui/graphicspanel.ts), with the game's own rows in them
  buildGraphics(panel, engine, smoke, cam, quality, { chars: game.ch, marks: game.decals, mains: () => `mains ${game.blackout.active ? 'TRIPPED' : 'on'} · torch ${game.tune.flashIntensity}`, glass: () => game.glassSummary(), markLights: () => game.markLights.length, rows: {
    quality: r => {
    panel.toggleBox(r, 'fixed 60 Hz game step', () => FIXEDSTEP.on, v => { FIXEDSTEP.on = v; }, 'the game stepped in whole 1/60 s ticks whatever the display\'s rate (host/loop.ts FIXEDSTEP; ON in the page, ?fixedstep=0 = off): the same simulation on a 60, a 120 and a 144 Hz display — groundwork for two-player co-op. Off = one update a frame on the frame\'s own time, as before. A script\'s __runFrames always names its own dt', true);
    panel.toggleBox(r, 'draw between ticks', () => TWEEN.on, v => { TWEEN.on = v; }, 'with the fixed step: a frame that falls between two game ticks is drawn at the blend of them (host/tween.ts TWEEN; ON in the page, ?tween=0 = off) — doors, props, debris, what the men carry, the men, carried lights and the camera (it steps with the tick too); sparks, tracers and laser lines are drawn as the last tick left them. The picture stands a constant part of a tick behind the game: none on a 60 Hz display, half a tick at 120 Hz, never more than one. Off = every frame draws the last tick: off 60 Hz the men and the camera move at 60 a second whatever the display draws', true);
    },
    hour: () => {
    // the hour (game/hours.ts): the level at another time of day than its own night — live, both ways
    const hourSec = panel.section('Hour', { open: false, summary: () => hourSummary(game) });
    panel.select(hourSec, 'hour', HOUR_IDS.map(hourLabel), () => HOUR_IDS.indexOf(hourOf(game)), v => { applyHour(game, HOUR_IDS[v] ?? 'night'); }, 'the level at another hour: a sun in the moon\'s place, the sky, haze and exposure of that hour, the outside lamps on a photocell (out at midday and under overcast), the night\'s storm held off. Night = the level exactly as it was made. The same pick as ?hour= and the mission sheet\'s Hour row; the AI\'s light meter reads the same sun and sky.');
    panel.slider(hourSec, 'sun strength', 0, 1, 0.05, () => sunStrength(game), v => { setSunStrength(game, v); }, 'the hour\'s sun at this share of its own strength (1) — the one number a cloud dims: the picture\'s light and the light meter\'s stand-in read the same value. Moved by hand it STAYS where it is put, whatever the weather (the button below hands it back). Nothing at night.');
    panel.buttons(hourSec, 'sun', [{ text: 'back to the weather\'s cloud', run: () => sunToCloud(game), title: 'a sun strength set by hand, handed back: under a picked weather the sun stands where its cloud puts it, with none at the hour\'s own' }]);
    panel.text(hourSec, () => hourSummary(game));
    },
    lights: l => {
    const groups = [...new Set(game.targets.filter(t => t.kind !== 'breaker' && t.group !== 'emergency').map(t => t.group))];   // breaker / beacons are driven by the blackout logic, not switches
    panel.buttons(l, 'all switchable', [{ text: 'on', run: () => groups.forEach(g => game.setGroup(g, true)) }, { text: 'off', run: () => groups.forEach(g => game.setGroup(g, false)) }, { text: 'repair', run: () => game.repairLights() }]);
    panel.buttons(l, 'mains', [{ text: 'trip 30 s', run: () => game.setBlackout(30, 'debug') }, { text: 'kill', run: () => game.setBlackout(Infinity, 'debug') }, { text: 'restore', run: () => game.endBlackout() }]);
    for (const g of groups) {
      const ts = game.targets.filter(t => t.group === g);
      panel.buttons(l, `${g} (${ts.length})`, [{ text: 'on', run: () => game.setGroup(g, true) }, { text: 'off', run: () => game.setGroup(g, false) }]);
    }
    const moon = engine.lights.lights.find(x => x.name === 'moon');
    if (moon) { panel.slider(l, 'moonlight', 0, 4, 0.05, () => moon.intensity, v => { moon.intensity = v; if (hourOf(game) === 'night') moon.enabled = v > 0; }, 'the level\'s moon. At a day hour (Frame ▸ Hour) the moon is out and the sun stands in its place: the slider then only sets what the moon comes back at'); }
    panel.slider(l, 'torch / unit light intensity', 0, 300, 5, () => game.tune.flashIntensity, v => { game.tune.flashIntensity = v; }, 'every carried torch and every spawned unit\'s weapon light (teammates, scenario defenders, the response team)');
    panel.slider(l, 'your light intensity', 0, 300, 5, () => game.tune.playerLight, v => { game.tune.playerLight = v; }, 'your own rail light, on whichever gun is in your hands');
    panel.slider(l, 'flashlight outer °', 8, 45, 1, () => game.tune.flashOuter, v => { game.tune.flashOuter = v; game.tune.flashInner = Math.min(game.tune.flashInner, v - 2); });
    panel.slider(l, 'flashlight inner °', 1, 30, 1, () => game.tune.flashInner, v => { game.tune.flashInner = Math.min(v, game.tune.flashOuter - 2); });
    panel.slider(l, 'OCP disable secs', 2, 30, 1, () => game.tune.ocpDuration, v => { game.tune.ocpDuration = v; });
    },
    smoke: sm => {
    panel.buttons(sm, '', [{ text: 'canister at cursor', run: () => { senseTool(game, 2, [game.aimPoint[0], game.aimPoint[1] + 0.12, game.aimPoint[2]]); } }, { text: 'clear', run: () => { senseTool(game, 3, []); } }]);
    },
    smokePlayer: sm => {
    panel.buttons(sm, 'player', [{ text: '+4 canisters', run: () => { game.player.canisters += 4; } }]);
    },
    glass: gl => {
    panel.buttons(gl, 'panes', [
      { text: 'crack test', run: () => { const bi = game.crackTest(); pageSay(bi >= 0 ? `pane ${bi} starred` : 'no pane near the cursor'); }, title: 'star the pane under the cursor (or the nearest within 4 m of the aim point) as a round would: a hole, a frosted collar and a crack star ON the glass — the floor hears it; the decals ▸ clear button (or a restart) wipes them' },
    ]);
    panel.text(gl, () => `glass ▸ ${game.glassSummary()}`);
    },
    decals: dc => {
    panel.buttons(dc, 'decals', [
      { text: 'spray test pattern', run: () => { const n = sprayTestPattern(game); pageSay(`decals: ${n} test marks in front of you (a five-round group + spray, a scorch and its splashes, a scuff on the nearest door, chips)`); }, title: 'every kind the shader draws, on what the player is facing / standing on right now' },
      { text: '+100 random', run: () => { const n = sprayRandom(game, 100, game.decals.count + 1); pageSay(`decals: ${n} marks scattered over the interior (${game.decals.count} live)`); }, title: 'a realistic mix (holes, chips, the odd scorch) shot from random points inside the building at random walls — for the cost measurement' },
      { text: 'clear decals', run: () => { game.decals.clear(); pageSay('decals cleared'); } },
    ]);
    },
    particles: pp => {
      panel.buttons(pp, 'burst test at cursor', [{ text: 'sparks', run: () => testBurst(game, 'sparks', game.aimPoint) }, { text: 'debris', run: () => testBurst(game, 'debris', game.aimPoint) }, { text: 'dust', run: () => testBurst(game, 'dust', game.aimPoint) }, { text: 'casings', run: () => testBurst(game, 'casings', game.aimPoint) }, { text: 'bang', run: () => testBurst(game, 'bang', game.aimPoint) }]);
    },
    impacts: pp => {
        panel.slider(pp, 'impact ×rifle', 0.5, 3, 0.1, () => PFX.impactScaleRifle, v => { PFX.impactScaleRifle = v; }, 'a rifle round knocks this much more out of a wall than a pistol round');
        panel.slider(pp, 'chip size mm', 4, 40, 1, () => PFX.chipSize * 1000, v => { PFX.chipSize = v / 1000; });
        panel.slider(pp, 'chip life s', 2, 60, 1, () => PFX.chipLife, v => { PFX.chipLife = v; });
        panel.slider(pp, 'dust puff size cm', 2, 40, 1, () => PFX.dustSize * 100, v => { PFX.dustSize = v / 100; });
        panel.slider(pp, 'bang sparks', 0, 2000, 10, () => PFX.bangSparks, v => { PFX.bangSparks = v; });
        panel.slider(pp, 'bang dust ring', 0, 200, 1, () => PFX.bangRing, v => { PFX.bangRing = v; });
        panel.slider(pp, 'casing life s', 2, 120, 1, () => PFX.casingLife, v => { PFX.casingLife = v; });
        panel.slider(pp, 'casing tink gain', 0, 1, 0.01, () => PFX.tinkGain, v => { PFX.tinkGain = v; }, 'the little sound of a case landing (CPU-scheduled off the shot; not a hearing event)');
    },
  } }, hud);

  // ================================================================ WEATHER & WIND: the night storm (E10, game/weather.ts), the wind field (game/wind.ts), the moths (game/moths.ts), wet characters (game/wet.ts)
  panel.system(SYS.wx);
  const W = game.weather;
  const wx = panel.section('Weather', { open: true, summary: () => W.summary() });
  panel.toggleBox(wx, 'storm', () => W.enabled, v => { W.setEnabled(v); if (!v && W.kind) pageSay(`a picked weather (${W.kind}) cannot be switched off here: drop ?weather= or pick 'as it is' below`); }, 'the level\'s own weather, the whole system: rain drawn in the outdoor air (volumetrics.wgsl), lightning as a transient directional light through the ordinary direct pass, thunder after distance ÷ 343 s, the wet glossy lot. Off puts everything it touched back (also ?noweather at boot). A weather picked for the mission (?weather=, the buttons below) cannot be switched off here: the men see and hear under it');
  panel.slider(wx, 'storm intensity', 0, 1, 0.05, () => W.storm, v => { W.storm = v; W.nextFlashAt = -1; }, 'how often lightning strikes (mean gap 11 s ÷ this, clamped 8‥25 s, jittered) and how close the cells sit; 0 = rain without lightning');
  panel.slider(wx, 'rain rate', 0, 1, 0.05, () => W.rainRate, v => { W.rainRate = v; }, 'fraction of the drop-column lattice falling: 0 = a dry electrical storm, 1 = a downpour (also drives the splash rings, the rain bed\'s level and — over 20 s — how wet the lot is)');
  panel.slider(wx, 'wind x (m/s)', -4, 4, 0.1, () => W.wind[0], v => { W.wind[0] = v; }, 'the streaks lean by wind ÷ 9 m/s of fall');
  panel.slider(wx, 'wind z (m/s)', -4, 4, 0.1, () => W.wind[1], v => { W.wind[1] = v; });
  panel.toggleBox(wx, 'wet lot → gloss (RT reflections off)', () => W.wetGloss, v => { W.wetGloss = v; }, 'while the asphalt is wet it carries BoxFlag.Gloss; with this on the storm also switches Indirect light ▸ Final gather ▸ glossy floors on for its duration so the lamps, the bollards and the lit facade mirror in it (not the strokes: transient lights stay out of the gloss term) — and the polished floors indoors reflect too, at the gloss pass\'s cost. No effect on a tier with RT reflections on (ultra / high / medium): they already mirror the wet lot. Takes effect at the next dry → wet flip (toggle the storm)');
  { const go = (k: WeatherKind | null) => { if (typeof location === 'undefined') { pageSay('weather kinds: in the page only (?weather=)'); return; } if (rowWeather(rowOfQuery(location.search))) { pageSay('this mission is always started in its own weather: ?rowset=0 lets the link pick'); return; } if (k !== W.kind) location.search = weatherSearch(location.search, k); };   // the weather the level is STARTED in (?weather=<kind>, the mission list's Weather row; game/weatherKinds.ts): the page's for its whole life (the renderer's half is compiled at boot), so a button RELOADS the page in that weather — 'as it is' = the level's own
    panel.buttons(wx, 'weather kind (reloads)', [{ text: 'as it is', run: () => go(null) }, ...WEATHER_KINDS.map(k => ({ text: k, run: () => go(k) }))]); }
  panel.buttons(wx, 'lightning', [{ text: 'strike now', run: () => { W.strike(); } }, { text: 'close one (0.4 km)', run: () => { W.strike({ distKm: 0.4 }); } }, { text: 'far (3.5 km)', run: () => { W.strike({ distKm: 3.5 }); } }, { text: 'from the south', run: () => { W.strike({ bearing: Math.PI, elevDeg: 35 }); } }]);
  panel.text(wx, () => W.summary());
  // PARTICLE RAIN (game/rainFx.ts + the 'rain' particle kind): the near-field layer of real GPU drops over the lattice rain — v2 (lit as water, refracting, wind-driven) ON by default; ?norain2 boots without it
  const pr = panel.section('Rain — particles', { open: false, summary: () => S.particleRain ? game.rainSummary() : 'off — the lattice rain alone (?norain2)', reset: () => { resetPrain(); pageSay('particle rain: v2 defaults'); }, resetTitle: 'every rain look / wind knob back to the shipped v2 values, and the copy kept on this machine dropped' });
  panel.toggleBox(pr, 'particle rain (near field)', () => S.particleRain, v => { S.particleRain = v; }, 'a NEAR-FIELD layer of real GPU rain drops (render/particles.ts kind rain) recycled over the outdoor ground inside a disc round the view: they fall at terminal speed with the wind, END on what they meet (roof, bonnet, pallet, a shoulder) as a splash crown there, are lit by the strongest SHADOW-TESTED lights (the lamp cone glows, the building\'s shadow cuts them), drip off roof / car edges, and mark themselves reactive for the TAA; the lattice rain stands back by as much under the disc and stays as it was beyond it. Off = the lattice rain alone, the frame byte for byte as it was before the layer (?norain2 boots it off)');
  panel.slider(pr, 'p-rain: max drops alive', 200, 7000, 100, () => S.particleRainCount, v => { S.particleRainCount = v; }, 'the cap on drops alive at once (the recycler aims at the LATTICE\'s own density over the outdoor ground inside the disc and stops here; a capped tier keeps the rest of the lattice under the disc). Preset: ultra 6000 · high 4000 · medium 2500 · low 1500 · verylow 800');
  panel.slider(pr, 'p-rain: disc radius (0 = auto)', 0, 30, 0.5, () => S.particleRainRadius, v => { S.particleRainRadius = v; }, 'radius of the disc round the look-at point the drops are recycled over (m); 0 = 0.6 × the camera distance (≈ 12.5 m at the follow camera: the view\'s corners), 8‥30');
  panel.slider(pr, 'p-rain: density × lattice', 0.25, 3, 0.05, () => S.particleRainDensity, v => { S.particleRainDensity = v; }, '× the lattice\'s drops per m² of ground at the current rain rate — 1 = the same rain near and far');
  panel.slider(pr, 'p-rain: brightness', 0.2, 4, 0.05, () => S.particleRainBright, v => { S.particleRainBright = v; }, '× the streaks\' / crowns\' scattered radiance (1 = calibrated on the lattice\'s gain)');
  panel.slider(pr, 'p-rain: lattice kept inside (−1 auto)', -1, 1, 0.05, () => S.particleRainLattice, v => { S.particleRainLattice = v < 0 ? -1 : v; }, 'the share of the LATTICE rain kept under the disc: −1 = auto (1 − what the particles supply: nothing doubles up, a capped tier keeps the difference), 0 = particles only inside, 1 = both layers (doubled up: the A/B of what each does)');
  panel.slider(pr, 'p-rain: TAA reactivity ×', 0, 12, 0.5, () => S.particleRainReact, v => { S.particleRainReact = v; }, '× a streak\'s display-referred luminance into the resolve\'s reactive mask (0 = none: the TAA treats the streaks as history-less noise and smears them to a tenth — the smear-map A/B)');
  panel.toggleBox(pr, 'p-rain: drips off edges', () => S.particleRainDrips, v => { S.particleRainDrips = v; }, 'fat slow drops off the top edges of whatever outdoors gets rained on and stands over a drop — the roof\'s rim, car roofs and bonnets, the van, the dumpster, the lamp heads (found once from the level\'s boxes) — ∝ the rain rate, from the edges inside the disc');
  // v2 LOOK + WIND (game/rainFx.ts PRAIN — every row rides to the GPU each frame; kept per machine)
  const prRow = (label: string, k: keyof typeof PRAIN, lo: number, hi: number, step: number, title: string) => panel.slider(pr, 'p-rain: ' + label, lo, hi, step, () => PRAIN[k], v => { PRAIN[k] = v; savePrain(); }, title + ` (default ${PRAIN_DEFAULTS[k]})`);
  prRow('phase — isotropic floor', 'iso', 0, 1, 0.05, 'the isotropic share of a drop\'s phase function: 1 = v1\'s grey scatterer (the same brightness whatever the angle); the forward lobe below is 4π-normalised, so iso + forward = 1 keeps v1\'s all-angle mean');
  prRow('phase — forward lobe ×', 'fwdK', 0, 2, 0.05, 'weight of the FORWARD lobe: refraction through a drop throws most light on within a few tens of degrees — a streak between the eye and a lamp flares, one lit from behind the eye is dim (Arkham\'s backlit rain)');
  prRow('phase — forward g', 'fwdG', 0, 0.95, 0.01, 'the lobe\'s Henyey-Greenstein g: 0.55 = ×7.8 looking straight into the light through the drop, ×1.3 at 55° off it, ×0.19 with the light behind the eye (a water sphere\'s transmitted fan reaches ~80°: broad, not a spike); higher = a tighter, fiercer flare');
  prRow('glint ×', 'glintK', 0, 8, 0.1, 'the STRAND GLINT: the water cylinder along a streak mirrors a light where the half vector stands square to it (× Schlick Fresnel for water) — slanted streaks sparkle where they line up between a lamp and the eye; 0 = off');
  prRow('glint roughness', 'glintRough', 0.02, 0.5, 0.01, 'the glint lobe\'s own width; each light adds its angular half-size (Karis\' area widening = the energy bound: a mirror shows a lamp at the lamp\'s radiance, never more)');
  prRow('glint cap (× E)', 'glintCap', 0, 100, 1, 'hard cap on one light\'s glint, × its irradiance × the gain — belt and braces against a firefly');
  prRow('gain ×', 'gain', 0.1, 8, 0.05, '× everything a drop scatters (1 at iso 1 = v1\'s lattice calibration; the follow camera mostly sees drops lit from its own side, under the forward lobe, so the base comes up)');
  prRow('tail keeps last frame', 'tailKeep', 0, 1, 0.05, 'the streak\'s TAIL shows the shade the drop had LAST frame — the point a frame\'s fall up today\'s streak, lit as it was there: a streak entering a torch beam brightens along its length; 0 = head colour end to end (v1)');
  prRow('refraction (0 = off)', 'refract', 0, 1, 0.05, 'a drop is a LENS on what is behind it: the streak is a thin water cylinder refracting the composite\'s copy through the module shared with the solid-glass lane (refract_common.wgsl: Snell in and out of the circular section, the exit ray followed to the scene\'s depth behind and projected back for the fetch, both Fresnel transmissions folded in), laid \'over\' the background by the streak\'s coverage — beside the lamp\'s pool a streak carries the pool\'s light out over the dark asphalt, over an even field it vanishes into it. Reads GBAA\'s copy of last frame when GBAA is on (free), else copies hdr before the splat (measured inside the noise). 0 = v1: additive scatter only');
  prRow('refraction reach behind (m)', 'refractReachM', 0.25, 40, 0.25, 'how far behind the drop the refracted ray is followed at most (the scene depth behind the pixel less the drop\'s own, capped): a drop images what lies well off its line, minified with the sides swapped — smaller = subtler, larger = the true wide view');
  prRow('refraction taps across', 'refractTaps', 2, 4, 1, 'taps across the cylinder\'s width, averaged (the pixel spans the whole width): 2 = ±0.55 · 3 = −0.7 / 0 / +0.7 · 4 = ±0.3 ±0.8 (each = two Snell steps, a projection, a depth load, a bilinear fetch per streak pixel)');
  prRow('refraction clamp (× white)', 'refractClamp', 0.5, 40, 0.5, 'each tap\'s display-referred luminance is held under this — an emitter\'s face behind a streak must not become a white needle');
  prRow('refraction depth reject (m)', 'refractDepth', 0.02, 5, 0.02, 'a tap landing on something this much NEARER the eye than the drop (the player\'s head beside the streak) is not behind it: the centre sample stands in');
  prRow('refraction: paint from beside', 'refractPaint', 0, 1, 0.05, 'How much of the picture a read taken BESIDE a streak keeps. 1 = all of it (as it was: rain beside a striped tent came up striped). 0 = its light only: no colour of its own, its brightness half way to what is straight behind, each pixel of the streak reading another spot');
  prRow('length spread', 'lenJit', 0, 0.9, 0.05, 'streak length × (1 ± this) per drop');
  prRow('brightness spread', 'brightJit', 0, 1, 0.05, '0 = every drop alike; 1 = a few fat bright ones among many fine (0.5 + 1.5 h², mean 1 — the lattice\'s own size spread)');
  prRow('half-width floor (px)', 'minPx', 0.5, 2, 0.05, 'thinner streaks are drawn this wide (half-width) and dimmed by as much: 0.75 = a 1.5 px line (v1); lower = finer, more aliased');
  prRow('half-width ceiling (px)', 'maxPx', 0.5, 8, 0.05, 'a drop a couple of metres off the lens is a thin bright LINE, not an 8-px bar: what is wider is drawn this wide (half-width) and brightened by as much, up to the cap below (v1 had no ceiling; 8 = none in practice)');
  prRow('… narrowing brightens up to ×', 'boostCap', 1, 8, 0.1, 'the most the width ceiling may brighten a narrowed streak (its light is otherwise kept)');
  prRow('beads along the streak', 'beadAmp', 0, 1, 0.05, 'a big drop OSCILLATES as it falls (a hundred-odd Hz), so a real streak photographs brightness-modulated along its length: × (1 ± this), 3‥6 beats per streak, phase and count per drop; 0 = an even line (v1)');
  prRow('near fade reach (m)', 'nearFadeM', 0.5, 8, 0.1, 'no depth of field here, so drops right under the lens go instead of smearing the frame: nothing nearer than 0.35 × this, fully drawn from here out (v1: 0.7 → 2 m)');
  prRow('wind × on the drops', 'windK', 0, 2, 0.05, '× the WIND FIELD\'s air on each drop (game/wind.ts: the storm outdoors easing in off the facade and gusting, a draught through an open door): born with the air, dragged toward it over ~0.9 s — the streaks slant and swing with the gusts; 0 = they fall plumb');
  prRow('lattice leans with it ×', 'latticeLean', 0, 1.5, 0.02, 'the far LATTICE rain is handed the same air: shear = −air × this ÷ 9 m/s, eased by the drops\' own response (0.92 = the mean of the height profile a drop falls through); 0 = the weather\'s raw shear (v1 — which leans AGAINST the wind field)');
  prRow('sheets — amplitude', 'sheetAmp', 0, 0.95, 0.05, 'births thinned into drifting SHEETS on a value noise carried downwind: (1 + amp·noise) ÷ (1 + amp), the CPU filing (1 + amp) × as many so the mean holds; 0 = uniform rain');
  prRow('sheets — scale (m)', 'sheetLen', 1.5, 30, 0.5, 'the sheets\' size');
  prRow('sheets — drift × air', 'sheetDrift', 0, 3, 0.1, 'how fast they cross the lot, × the storm\'s air speed');
  panel.buttons(pr, 'p-rain look', [
    { text: 'v1 look', run: () => { setPrain(PRAIN_V1_LOOK); savePrain(); pageSay('particle rain: v1 look (grey scatter, no refraction, plumb fall) — kept on this machine; ?rainv1 boots it for one visit'); } },
    { text: 'v2 defaults', run: () => { resetPrain(); pageSay('particle rain: v2 defaults'); }, title: 'the same as this section\'s ↺' },
  ]);
  panel.text(pr, () => game.rainSummary());
  // WIND (game/wind.ts): the building's air — the vents, the storm outdoors, the gust through an open exterior door — as ONE field the particles and the smoke solver both take
  const T = game.tune as { wind?: number; moths?: boolean };
  const wd = panel.section('Wind', { open: true, summary: () => game.wind.summary(), reset: () => { resetWindKnobs(game); pageSay('wind: knobs back to the shipped values'); }, resetTitle: 'every wind knob (and the moths switch) back to the shipped values, and the copy kept on this machine dropped' });
  panel.slider(wd, 'wind: master', 0, 1, 0.05, () => T.wind ?? 1, v => { T.wind = v; saveWind(game); }, 'the master on every source of moving air (game/wind.ts): the level\'s HVAC supply VENTS (a cone jet out of each ceiling diffuser), the STORM outdoors (the weather\'s wind vector, gusting), a GUST in through every open exterior door — one analytic field handed each frame to the GPU particles (paper trembles and slides under a diffuser, dust rides it, chips a share) AND to the smoke solver (a plume under a vent bends, one by an open door leans in; the guards\' sight lines march that same density). × every speed; 0 = still air: nothing packed, nothing forced — the game as it was (kept per machine)');
  const windSave = () => saveWind(game);
  knobRows(panel, wd, WIND, null, windSave, [
    { k: 'ventK', label: 'wind: vent speed ×', min: 0, max: 3, step: 0.05, title: `× every vent's authored face speed (2‥2.8 m/s: an office diffuser; the air halves by its reach and is gone by 2.5 × it, a fifth of the face speed on the floor under a ceiling one). 3 = a wind tunnel (default ${WIND_DEFAULTS.ventK})` },
    { k: 'gustAmp', label: 'wind: gust amplitude', min: 0, max: 0.8, step: 0.05, title: `the vents' ± (the storm's stays ${WIND_DEFAULTS.stormGustAmp}); it is the gust PEAKS that pick a leaf up off the desk under a diffuser (default ${WIND_DEFAULTS.gustAmp})` },
    { k: 'smokeRelax', label: 'wind: smoke relaxation (/s)', min: 0, max: 12, step: 0.25, title: `how hard the solver's air is pulled toward the field at a jet's CORE (× the jet's own profile off it, so a plume rising beside a vent keeps its own motion): u += k·w·(v_air − u)·dt in the forces step, then the pressure solve spreads a down-jet along the floor by itself. 0 = the smoke ignores the wind (the particles still ride it) (default ${WIND_DEFAULTS.smokeRelax})` },
    { k: 'sheetWake', label: 'wind: paper wake speed (m/s)', min: 0.1, max: 1.5, step: 0.05, title: `air speed over a RESTING leaf (the down-jet turned along the desk it lies on) past which a wake draw may pick it up — once every 0.75 s a hashed share of the eligible leaves (more the harder it blows, ≤ 40 %) hops up into the air and slides; under it they only tremble. Under a shipped diffuser the desk sees ~0.3 m/s, so at 0.35 it is the gusts that do it (default ${WIND_DEFAULTS.sheetWake})` },
    { k: 'sheetKick', label: 'wind: paper kick ×', min: 0.5, max: 5, step: 0.1, title: `a picked-up leaf's hop = the air speed over it × this, edge lifted (the impulse lane's own tilt / tumble response) — 2 = a hand's slide and settle, 5 = leaves cartwheeling off the desk (default ${WIND_DEFAULTS.sheetKick})` },
    { k: 'debrisK', label: 'wind: chip coupling', min: 0, max: 1, step: 0.05, title: `the share of the air's velocity a flying CHIP's drag pulls it toward (dust and paper take all of it, brass and sparks none): 0.15 = a plaster crumb drifts a little in a jet, 1 = chips blown like dust (default ${WIND_DEFAULTS.debrisK})` },
  ]);
  let ventIx = -1;   // 'put me under a vent ▸' cycles the level's vents (the maintainer could not find one to test with, 08-12)
  panel.buttons(wd, 'wind', [
    { text: 'still air (0)', run: () => { T.wind = 0; saveWind(game); pageSay('wind: still air — nothing packed, nothing forced'); } },
    { text: 'full (1)', run: () => { T.wind = 1; saveWind(game); pageSay('wind: on'); } },
    { text: 'put me under a vent ▸', run: () => { const vs = game.level.vents ?? []; if (!vs.length) { pageSay('wind: this level has no vents'); return; } ventIx = (ventIx + 1) % vs.length; const v = vs[ventIx]; game.teleportPlayer([v.pos[0] + 0.9, 0, v.pos[2]]); pageSay(`wind: ${v.name} — ${v.speed.toFixed(1)} m/s at the face, over your head (pop a can here; the grille is the dark square in the ceiling)`); } },
    { text: 'log sources', run: () => { for (const ln of game.wind.describe()) console.log(ln); pageSay(`wind: ${game.wind.active.length} source(s) blowing — see the console`); } },
    { text: 'air at cursor', run: () => { const v = game.wind.at([game.aimPoint[0], game.aimPoint[1] + 0.05, game.aimPoint[2]]); pageSay(`wind at cursor: ${Math.hypot(v[0], v[1], v[2]).toFixed(2)} m/s (${v.map(x => x.toFixed(2)).join(', ')})`); } },
    { text: 'reset wind knobs', run: () => { resetWindKnobs(game); pageSay('wind: knobs back to the shipped values'); }, title: 'the same as this section\'s ↺' },
  ]);
  panel.text(wd, () => game.wind.summary());
  // THE FIELD MADE VISIBLE (ui/windviz.ts): arrows of game.wind.at() on a lattice round the look-at, the sources' cones / the storm's arrow / door gusts, what the particles were handed, the air at the cursor —
  // a 2D canvas over the view; the renderer and the game are untouched whether it is on or off
  { const V = overlay.windViz;
    const wv = panel.section('Wind — field view (debug)', { open: false, summary: () => V.summary() });
    panel.toggleBox(wv, 'wind: show field', () => V.showField, v => { V.showField = v; }, 'a lattice of arrows on a horizontal plane round the camera\'s look-at point: each is game.wind.at(p) — the same analytic sum of every blowing source the GPU particles and the smoke solver evaluate — its length ∝ the air speed there (capped), its colour the HUD\'s: thin green under 0.5 m/s, amber by 1.5, red from 3; a faint dot = still air. Under a ceiling diffuser at 2 m the down-jet is a tight red cluster, at 0.1 m the wall jet fans out along the floor; by an open exterior door in the storm the gust points in. ≤ 600 samples a frame on the CPU (the cost is in the summary above)');
    panel.select(wv, 'wind: field height', ['0.1 m (the floor\'s wall jets)', '1.0 m (chest)', '2.0 m (under the diffusers)'], () => V.height < 0.5 ? 0 : V.height < 1.5 ? 1 : 2, v => { V.height = [0.1, 1.0, 2.0][v] ?? 1; }, 'the plane the lattice is sampled on');
    panel.slider(wv, 'wind: field radius (m)', 3, 13, 1, () => V.radius, v => { V.radius = v; }, 'the disc round the look-at the arrows cover (a 1 m lattice; past ~600 samples the lattice coarsens to stay inside the budget)');
    panel.slider(wv, 'wind: arrow scale (m per m/s)', 0.2, 2, 0.1, () => V.scale, v => { V.scale = v; }, 'metres of arrow per m/s of air (each arrow is capped at 0.9 m so a vent\'s core does not swamp the picture — the colour still says how fast)');
    panel.toggleBox(wv, 'wind: show sources', () => V.showSources, v => { V.showSources = v; }, 'every source the level has, live ones bright: a VENT = its cone — the apex at the diffuser, the axis arrow to its reach, the dashed ring where the air has halved (reach) and the solid rim where it is gone (2.5 × reach, widened by the spread) — with its name and face speed; the STORM = one big arrow standing off the upwind facade along the wind; a DOOR GUST = an arrow in through the doorway while that door stands open in the storm');
    panel.toggleBox(wv, 'wind: show what particles get', () => V.showPacked, v => { V.showPacked = v; }, 'the same sources, but the ≤ 8 records wind.ts packed for the GPU particles THIS frame (the jets nearest the look-at + the storm) drawn amber and tagged ▶GPU, the rest greyed — the smoke solver takes every live source and culls per domain itself');
    panel.toggleBox(wv, 'wind: probe at cursor', () => V.probe, v => { V.probe = v; }, 'the air at the aim point, live: an arrow at the cursor and its m/s beside it, the vector in this section\'s header line');
    panel.text(wv, () => V.on ? V.summary() : ''); }
  // MOTHS (game/moths.ts): round the lit sodium lamps at night
  const mo = panel.section('Moths', { open: false, summary: () => game.moths.summary() });
  panel.toggleBox(mo, 'moths at the sodium lamps', () => T.moths !== false, v => { T.moths = v; saveWind(game); }, `up to ${MOTHS.max} moths (${MOTHS.perLamp} a lamp) orbiting under the LIT street lamps at NIGHT — small NoShadow boxes lit by the lamp like anything else, gathering over ${MOTHS.gatherSecs} s when it comes on, scattering when it goes out / by day / in a gale (${MOTHS.galeMs[0]}‥${MOTHS.galeMs[1]} m/s at the head) / a downpour`);
  panel.text(mo, () => game.moths.summary());
  // wet characters (game/wet.ts): the model's rates and the look's numbers — the look rides to the GPU per character instance, so every row is live
  const wt = panel.section('Wet characters', { open: false, summary: () => S.wetChars ? game.wetSummary() : 'renderer switch off', reset: () => { resetWet(); pageSay('wet: knobs back to the shipped values'); }, resetTitle: 'every wet knob back to the shipped value, and the copy kept on this machine dropped' });
  panel.toggleBox(wt, 'wet characters', () => S.wetChars, v => { S.wetChars = v; }, 'whoever stands under the open sky in the rain soaks (Character.wet: darker saturated cloth under a glossy film that streams and glints, drops beading and running down him — gbuffer.wgsl fsSkin + composite.wgsl wetCoatSpec) and dries indoors; a soaked player come in drips and leaves wet prints. This is the RENDERER\'s switch: off = every character shades bit for bit as before (the model underneath keeps running)');
  const wetRow = (label: string, k: keyof typeof WET, lo: number, hi: number, step: number, title: string) => panel.slider(wt, 'wet: ' + label, lo, hi, step, () => WET[k], v => { WET[k] = v; saveWet(); }, title + ` (default ${WET_DEFAULTS[k]})`);
  wetRow('soak rate (/s)', 'soakPerSec', 0.02, 1, 0.01, 'coat rise per second in a downpour (× 0.4 + 0.6 × rain rate): 1/6 = soaked in six seconds');
  wetRow('dry rate (/s)', 'dryPerSec', 0.005, 0.5, 0.005, 'coat fall per second out of the rain: 1/25');
  wetRow('bead run-off (s)', 'beadRunoffSecs', 0.5, 15, 0.5, 'the drops standing on him are gone this long after he comes out of the rain (the coat itself dries slower)');
  adv(() => {
    wetRow('bead density', 'beadDensity', 0, 1, 0.05, 'fraction of the drop lattice\'s cells holding a drop at full beads');
    wetRow('bead size (m)', 'beadSize', 0.003, 0.02, 0.0005, 'drop radius; ×0.75‥1.25 per drop');
    wetRow('trail length (m)', 'trailLen', 0.02, 0.3, 0.01, 'how far a drop runs before it is spent — its wet trail is at most this long');
    wetRow('film roughness', 'roughWet', 0.03, 0.5, 0.01, 'GGX roughness (perceptual) of the water film\'s streaks: the tightness of the glints');
    wetRow('damp cloth roughness', 'roughDry', 0.2, 0.65, 0.01, 'roughness of merely damp cloth — the wide dim lobe the coat dries out through');
    wetRow('darken exponent', 'darken', 0, 1.5, 0.05, 'wet fabric: albedo × its luminance^(this × coat) — dark cloth drops hardest, the hue holds …');
    wetRow('darken multiplier', 'darkenMul', 0, 0.6, 0.02, '… × (1 − this × coat), then a quarter more saturated about that grey');
    wetRow('specular gain', 'specGain', 0, 2, 0.05, '× the coat\'s specular (direct glints + the mirrored ambient); 0 = the darkening alone');
  });
  wetRow('prints after rain', 'prints', 0, 30, 1, 'wet footprints the player leaves after coming in soaked, fainter each step (decals; the player sees them, nobody reacts yet)');
  wetRow('print fade (s)', 'printFadeSecs', 5, 120, 5, 'each print dries out over this long');
  wetRow('waterline wicking band (m)', 'lineBand', 0.01, 0.5, 0.01, 'a man wet only up to a WATERLINE (wading — the water lane; or the two buttons below) is damp this far above it, fading to dry across the band; the rain wets all of him and this does not apply');
  panel.buttons(wt, 'wet', [
    { text: 'soak the player now', run: () => { wetSoak(game, game.player.char, 1, 1, 0.8); pageSay('wet: the player soaked through (as if just in from a downpour — watch him drip, print and dry)'); } },
    { text: 'soak everyone', run: () => { for (const ch of wetEveryone(game)) wetSoak(game, ch, 1, 1, 0.8); pageSay('wet: everybody soaked'); } },
    { text: 'the player wet to the knee', run: () => { wetSoak(game, game.player.char, 1, 0, 0, 0.55); pageSay('wet: the player soaked to the knee (as if he had waded: dark cloth below the line, no drops, the rest of him dry)'); }, title: 'the waterline: coat 1 below 0.55 m of his bind pose, nothing above the wicking band, no beads (they are the rain\'s) — what the water lane writes from its own depth' },
    { text: '… to the waist', run: () => { wetSoak(game, game.player.char, 1, 0, 0, 1.05); pageSay('wet: the player soaked to the waist'); } },
    { text: 'dry all', run: () => { wetDryAll(game); pageSay('wet: everybody dry'); } },
    { text: 'reset wet knobs', run: () => { resetWet(); pageSay('wet: knobs back to the shipped values'); }, title: 'the same as this section\'s ↺' },
  ]);
  panel.text(wt, () => game.wetSummary());

  // ================================================================ SANDBOX: the debug actions + AI switches, movement + detection, bodies by light, weapons, aim, whistle, tactics, needs, civilians, dialogue / alarm / clearing, QRF, devices, player + doors + props
  panel.system(SYS.sand);
  const gp = panel.section('Sandbox', { open: true, summary: () => `AI ${game.aiEnabled ? 'on' : 'OFF'} · god ${game.godMode ? 'ON' : 'off'} · ammo ${game.infiniteAmmo ? '∞' : 'counted'}${game.spectate >= 0 ? ` · spectating ${game.spectate}` : ''}` });
  panel.buttons(gp, 'sandbox', [
    { text: 'trip / restore mains', run: () => { if (game.blackout.active) game.endBlackout(); else game.setBlackout(30, 'debug'); }, title: 'blackout: everything on the breaker goes dark for 30 s, beacons spin up (again = restore)' },
    { text: 'smoke at cursor', run: () => { senseTool(game, 2, [game.aimPoint[0], game.aimPoint[1] + 0.12, game.aimPoint[2]]); pageSay('smoke canister deployed'); } },
    { text: 'clear smoke', run: () => { senseTool(game, 3, []); pageSay('smoke cleared'); } },
    { text: 'teleport to cursor', run: () => game.teleportPlayer(game.aimPoint) },
    { text: 'spectate next', run: () => { game.spectate = game.spectate + 1 >= game.guards.length ? -1 : game.spectate + 1; pageSay(game.spectate < 0 ? 'camera: player' : `camera: ${game.guards[game.spectate]?.callsign ?? `man ${game.spectate}`}`); }, title: 'cycle the camera through every man on the floor (teammates included) and back to the player' },
  ]);
  panel.toggleBox(gp, 'guard AI', () => game.aiEnabled, v => { game.aiEnabled = v; keepSandbox('noAi', !v); });
  panel.toggleBox(gp, 'god mode', () => game.godMode, v => { game.godMode = v; keepSandbox('god', v); }, 'guards still see and shoot you, but hits do no damage');
  panel.toggleBox(gp, 'unlimited ammo', () => game.infiniteAmmo, v => { game.infiniteAmmo = v; keepSandbox('ammo', v); }, 'bottomless magazine: rounds still cycle (cadence, reload animation if you press R) but the count never drops; cans, ring airfoils and wall charges do not run out either');
  panel.buttons(gp, 'encounter', [{ text: 'restart (Shift+Enter)', run: () => game.restart() }]);
  panel.toggleBox(gp, 'show AI debug labels', () => game.showDebug, v => { game.showDebug = v; }, 'state / awareness tags over the guards');
  panel.toggleBox(gp, 'light fixture dots', () => overlay.showLightDots, v => { overlay.showLightDots = v; });
  panel.system(SYS.missions);
  // DIFFICULTY (game/difficulty.ts): the ONE switch over two tables (Harder defenders, Defenders dig in) — hard = both of them on at hard's rows (difficulty.ts HARD: the shipped rows + stand to, stand and fight and far rooms by one door, with the room's keeper not dug in); standard = both off, the
  // defenders as they were. Read off those rows; kept per machine through their own stores; a scenario restart takes it.
  { const df = panel.section('Difficulty (loud scenarios)', { open: true, summary: () => difficultyNow(), reset: () => setDifficulty('standard'), resetTitle: 'standard: both harder-defender sections off and at their shipped rows, the copies kept on this machine dropped' });
    panel.toggleBox(df, 'hard defenders', () => difficultyNow() === 'hard', v => setDifficulty(v ? 'hard' : 'standard'), `THE difficulty switch for the loud squad missions (also on the mission sheet). Off = STANDARD: the defenders exactly as they were. On = HARD: the harder defenders' and the dig-in rows both on at hard's numbers (their shipped rows plus stand to, stand and fight and far rooms by one door, but the room's keeper does not dig in) — defenders near a room you have hit come at it together and round by a second door, the men of a room farther off as one by one door, call it on their radio first, stand and fight where they meet you out of it, and fall back to fight again; heard before your first entry they report it, wait, and all come when a room is hit; the man left to keep a room slips out by another door when you reach his. Few barricade: a defender shuts, wedges and covers his door only when he hears a fight he has no way into. No hit points, aim or dice change. Hard also narrows your own team's fog of war to cones (Missions ▸ Fog of war ▸ \'fog of war ▸ HARD: eyes are cones\'). Restart the scenario after flipping it. Levels: ${DIFFICULTIES.join(' · ')}.`, false);
    panel.text(df, () => difficultyWords(undefined, squadsLive(game) && (squadsWhere(game) || true)));
  }
  // ENEMY NERVE (game/morale.ts): the two choices a playtest turns — who is green, and how brittle they all are. A choice made by hand stands for the visit
  // (moraleKeep: through a restart and the page load a change of level costs). The nine rules' own switches are settled and have no box.
  { const TRAINING_WORDS = ['by difficulty', 'all green', 'leader and green', 'all trained', 'none (an old recording)'], SHOWN = [...TRAININGS, 'none'] as const;   // (morale.ts TRAININGS, in its order — and 'none', which only a recording from before training sets: shown so the row says what is really set)
    const nv = panel.section('Enemy nerve', { open: true, summary: () => MORALE.on ? `${TRAINING_WORDS[Math.max(0, SHOWN.indexOf(MORALE.training))]} · ${MORALE.nerve}` : 'off' });
    panel.select(nv, 'training', TRAINING_WORDS, () => Math.max(0, SHOWN.indexOf(MORALE.training)), v => { MORALE.training = SHOWN[v] ?? 'difficulty'; moraleKeep('training'); }, 'who is GREEN and who is STEADY (it moves when a man breaks and how he copes, never his health, his aim or his reactions). by difficulty = the squad\'s own key in the level, else standard: the first man of each squad is its steady leader and the rest are green; hard: every man steady but the last of a squad of three or more (no squads: the men of a room are a squad; the night staff by temperament: the nervy ones are green). all green / leader and green / all trained = that for every squad, whatever the level and the difficulty. The response team is always steady. A man inside a sequence keeps the training he went into it with. Also ?training=green|mixed|trained');
    panel.select(nv, 'nerve', NERVES, () => Math.max(0, NERVES.indexOf(MORALE.nerve)), v => { MORALE.nerve = NERVES[v] ?? 'normal'; moraleKeep('nerve'); }, 'every shock a man takes × 1.4 (brittle) / 1 (normal) / 0.7 (steady): how readily the whole garrison is rattled. Also ?nerve=brittle|steady');
    panel.text(nv, () => moraleRoster(game) || 'men ▸ off');   // 'men ▸ S g g g · S g g g · S g g' — steady / green, a squad at a time (read-only)
    panel.text(nv, () => moraleSummary(game) || 'morale ▸ off');   // 'morale ▸ Tango 2 0.85 BROKEN hold (alone, dark) · Tango 4 0.35 jumpy duck' (read-only)
    panel.text(nv, () => runnerSummary(game) || 'runner ▸ off');   // 'runner ▸ Tango 2 out (told 2) · Tango 5 say' (read-only)
    panel.text(nv, () => darkDoorSummary(game) || 'dark door ▸ off');   // 'dark door ▸ Novak balk 6.2 s at breakroom (Reyes coming) · Reyes come' (read-only)
    panel.text(nv, () => netQuietSummary(game) || 'net ▸ off'); }
  panel.system(SYS.sand);
  // STRESS (game/stress.ts): extra enemies on the floor that is up — the enemy-ceiling check. Nothing until a button is pressed (or ?extra=N).
  { let n = 3; const st = panel.section('Stress: extra enemies', { open: false, summary: () => STRESS.want ? `${STRESS.want} asked` : 'off' });
    panel.slider(st, 'extra enemies', 1, 15, 1, () => n, v => { n = Math.round(v); }, 'How many more hostile men the button stands. They are ordinary defenders (held on a spot, gun on a door); under a room clear they are dealt to its rooms and count for it. Only as many as the character budget takes are stood: sixteen characters all told, bodies included — on The block (you + four + eight) that is three more.');
    panel.buttons(st, 'stand', [
      { text: 'stand them now', run: () => { standExtra(game, n); }, title: 'stand that many more enemies on this floor now (a restart takes them away again)' },
      { text: 'keep them (every restart)', run: () => { STRESS.want = n; }, title: 'the same count stood again after every restart of the scenario, as ?extra=N on the URL does' },
      { text: 'off', run: () => { STRESS.want = 0; pageSay('stress off — the extra men leave at the next restart (Shift+Enter)'); }, title: 'stop standing them: the next restart stages the scenario as it ships' }]);
    panel.text(st, () => `${stressSummary(game)} · read fps / gpu ∪ in the strip above (or ?hud)`);
    panel.text(st, () => STRESS.last === 'off' ? 'last staging: none yet' : STRESS.why ? `last staging stopped: ${STRESS.why}` : 'last staging: nothing refused');
  }
  // ================================================================ AUDIO
  panel.system(SYS.audio);
  const au = panel.section('Mixer', { open: true, summary: () => audio.started ? `voices ${audio.stats.voices} · master ${audio.volume.master.toFixed(2)}${audio.enabled ? '' : ' · MUTED'}` : 'audio not started (click the view)' });
  panel.text(au, () => audio.started ? `voices ${audio.stats.voices} · reverb send ${audio.stats.wet.toFixed(2)} (traced free path) · occluded plays ${audio.stats.occluded} · routed around corners ${audio.stats.routed}` : 'click / press a key in the view to start audio');
  panel.toggleBox(au, 'sound', () => audio.enabled, v => { audio.enabled = v; audio.setVolumes(); });
  panel.toggleBox(au, 'music', () => audio.music?.enabled ?? true, v => { if (audio.music) audio.music.enabled = v; });
  panel.select(au, 'piece', [...PIECE_NAMES], () => audio.music?.picked ?? 0, v => audio.music?.setPiece(v), 'which piece of the score is playing (audio/pieces.ts): picked here it comes in at the next bar line; left alone each level plays the two written down for it (audio/setlist.ts; ?shuffle = dealt from a shuffled bag, ?onetheme = the theme alone)');
  panel.slider(au, 'master', 0, 1, 0.01, () => audio.volume.master, v => { audio.volume.master = v; audio.setVolumes(); });
  panel.slider(au, 'effects', 0, 1.5, 0.01, () => audio.volume.sfx, v => { audio.volume.sfx = v; audio.setVolumes(); });
  panel.slider(au, 'music volume', 0, 1, 0.01, () => audio.volume.music, v => { audio.volume.music = v; audio.setVolumes(); });
  panel.slider(au, 'ambience', 0, 1.5, 0.01, () => audio.volume.ambience, v => { audio.volume.ambience = v; audio.setVolumes(); });
  let forced = -1;
  panel.slider(au, 'force tension', -1, 1, 0.05, () => forced, v => { forced = v; if (audio.music) audio.music.intensity = v < 0 ? game.tension(MUSIC.theirs) : v; }, '-1 = follow the game; otherwise pin the score intensity');
  setInterval(() => { window.__forcedTension = forced >= 0; if (forced >= 0 && audio.music) audio.music.intensity = forced; }, 200);

  // ================================================================ CHECKPOINTS (game/savestate.ts, STATUS G5): the dev rows — save now (the manual slot; 'force' lets go of Sam's hands and any drill in the FILE), load the
  // newest / a chosen slot, export the newest to a .json download, import a .json into the manual slot, wipe the store, autosave on / off, the controller's line
  panel.system(SYS.ckpt);
  const CK = game.saves; let ckPick = 0;
  const ckS = panel.section('Checkpoints', { open: true, summary: () => `${game.guards.some(x => x.unit && x.team === 'operative') ? (SAVETIMELINE.on ? 'a squad run saves in memory (this run only) · ' : 'no saves in a squad run · ') : ''}${CK.summary(game)}` });
  const ckLoad = (key: string | undefined) => { if (!key) { pageSay('checkpoint: nothing stored'); return; } const load = W_.__save?.load; const why = load ? load(key) : CK.load(game, key); if (why) pageSay(`checkpoint: ${why}`); };   // (main.ts's loader stops a show and cuts the camera; the bare controller otherwise)
  panel.toggleBox(ckS, 'autosave', () => CK.enabled, v => CK.setEnabled(v), `Solo runs only: any run with teammates or scenario defenders on the floor (rescue, room clear, night breach, the pair) refuses every save, forced or not. Written by itself on: the mission stage advancing, the first entry into a room region, a hauled body let go of ≥ ${CKPT.bodyHideM} m away, every ${CKPT.quietSecs} s of undetected play — never while anybody is alert / searching / over awareness ${CKPT.awareMax}, never mid-action, ${CKPT.debounceSecs} s debounce, ≥ ${CKPT.minGapSecs} s apart, a request unmet for ${CKPT.pendingSecs} s lapses; ${CKPT.autoSlots} rotating slots in localStorage (kept per machine with this switch)`);
  panel.buttons(ckS, 'save', [
    { text: 'save now', run: () => { const res = CK.write(game, 'manual', 'by hand'); pageSay(res.ok ? `checkpoint saved by hand (${(res.bytes! / 1024).toFixed(0)} KB)` : `checkpoint refused: ${res.why}`); }, title: 'the manual slot (one, apart from the three autosaves) — refused with the reason under a show / daylight / the QRF / a unit / the player down / a man scripted / the player mid-action / a thing on its fuse' },
    { text: 'save (force)', run: () => { const res = CK.write(game, 'manual', 'by hand (forced)', true); pageSay(res.ok ? `checkpoint forced (${(res.bytes! / 1024).toFixed(0)} KB${res.state?.forced.length ? '; let go of ' + res.state.forced.join(', ') : ''})` : `refused even forced: ${res.why}`); }, title: 'accepts you mid-action and scripted men (not a squad run: units on the floor still refuse): the FILE lets go of them (a held man free, a pick abandoned, the pair off their drill) — the running game is not touched' },
    { text: 'why not?', run: () => pageSay(`checkpoint: ${canSave(game) ?? 'a plain save is fine now'} · autosave gate: ${CK.gate(game) ?? 'clear'}`) },
  ]);
  panel.buttons(ckS, 'load', [
    { text: 'newest', run: () => ckLoad(CK.newest(false)?.key), title: 'the newest stored checkpoint of any run (the cards prefer this run\'s)' },
    { text: 'chosen ↓', run: () => ckLoad(CK.slots()[ckPick]?.key), title: 'the slot picked in the row below' },
  ]);
  panel.select(ckS, 'slot', ['(newest)', '2nd', '3rd', '4th (oldest)'], () => Math.min(ckPick, 3), v => { ckPick = v; }, 'which stored checkpoint "chosen" loads / "export" sends: newest first — the line below lists them in that order');
  panel.text(ckS, () => { const sl = CK.slots(); return sl.length ? sl.map((x, i) => `${i === ckPick ? '▸' : ' '}${x.key.replace('twopointsix.save.', '')} · ${x.label || x.kind} · ${x.stage} @ ${Math.floor(x.missionT / 60)}:${String(Math.floor(x.missionT % 60)).padStart(2, '0')} · ${CK.ageWords(game, x)} · ${(x.bytes / 1024).toFixed(0)} KB${x.regret ? ` · REGRETTED (${x.regret})` : ''}`).join('\n') : 'no checkpoints stored on this machine'; });
  panel.buttons(ckS, 'file', [
    { text: 'export chosen', run: () => { const info = CK.slots()[ckPick] ?? CK.newest(false);
      // (?rust=1, host/rustgame.ts: a save is the Rust game's own, kept in the module, and its file is the bridge's bytes under a version word: the same door, another name)
      { const R = (window as unknown as { __rust?: { file?: (key?: string) => Uint8Array | null } }).__rust; if (R?.file && (!info || info.mem)) { const bytes = R.file(info?.key); if (!bytes) { pageSay('checkpoint: nothing to export'); return; }
        const name = `twopointsix-checkpoint-${info?.stage ?? game.mission.stage}-${Math.round(info?.missionT ?? game.mission.stats.elapsed)}s.tpsr`, w = saveFile('checkpoint', name, new Blob([bytes as BlobPart], { type: 'application/octet-stream' }), { done: arrived => pageSay(savedEnd('The checkpoint', arrived)) });
        pageSay(w.local ? `exported ${name} (${(bytes.length / 1048576).toFixed(1)} MB)` : savedWords('The checkpoint', w.went)); return; } }
      const text = info ? CK.store.get(info.key) : null; if (!text || !info) { pageSay('checkpoint: nothing to export'); return; } const name = `twopointsix-checkpoint-${info.stage}-${Math.round(info.missionT)}s.json`, kb = (text.length / 1024).toFixed(0);
      // (NO DOWNLOAD since 2026-10-27, host/savefile.ts: the file goes to the build's backend where there is one, and the line says what became of it — sent, held for the sharing notice, or not sent — and, later, that it arrived or could not go. `?downloads=1` = the file downloaded, as it was)
      const w = saveFile('checkpoint', name, new Blob([text], { type: 'application/json' }), { done: arrived => pageSay(savedEnd('The checkpoint', arrived)) });
      if (w.local) pageSay(`exported ${name} (${kb} KB)`); else { try { console.info(`checkpoint export: ${name} (${kb} KB)`); } catch { /* */ } pageSay(savedWords('The checkpoint', w.went)); } }, title: 'send the chosen slot to the developer as a .json file (the same text localStorage holds); nothing is downloaded' },
    { text: 'import…', run: () => { const inp = document.createElement('input'); inp.type = 'file'; inp.accept = '.json,application/json,.tpsr'; inp.onchange = async () => { const f = inp.files?.[0]; if (!f) return;
      { const R = (window as unknown as { __rust?: { fileIn?: (b: Uint8Array) => { label: string; key: string } | string } }).__rust; if (R?.fileIn) { const r = R.fileIn(new Uint8Array(await f.arrayBuffer())); pageSay(typeof r === 'string' ? `import refused: ${r}` : `imported '${r.label}' into a slot by hand (${(f.size / 1048576).toFixed(1)} MB) — load ▸ newest to play it`); return; } }
      const text = await f.text(); const res = CK.importText(game, text); pageSay(typeof res === 'string' ? `import refused: ${res}` : `imported '${res.label}' into the manual slot (${(res.bytes / 1024).toFixed(0)} KB) — load ▸ newest to play it`); }; inp.click(); }, title: 'read a .json checkpoint into the manual slot (validated: version, level); it is not loaded until you say' },
    { text: 'clear all', run: () => { CK.clear(); pageSay('checkpoints: every stored slot wiped'); }, title: 'remove the three autosaves, the manual slot and the index from localStorage' },
  ]);
  panel.text(ckS, () => CK.summary(game));
  panel.text(ckS, () => CK.log.slice(-4).map(x => `${x.t.toFixed(0)}s ${x.what}`).join('\n') || '(nothing yet)');
}
