// Panel contents: binds engine / game / smoke / camera settings to UI controls.
import { Engine, FrameFlags } from '../engine';
import { Game } from '../game/game';
import { SmokeSystem } from '../smoke/smoke';
import { FollowCamera } from '../game/camera';
import { Overlay } from './overlay';
import { Panel } from './panel';
import { DEG } from '../math/vec';
import { defaultRcConfig } from '../render/rc';
import { AudioEngine } from '../audio/audio';
import { Quality, QUALITY_NAMES } from './quality';
import * as fx from '../game/effects';

export function buildPanel(panel: Panel, engine: Engine, game: Game, smoke: SmokeSystem, cam: FollowCamera, overlay: Overlay, audio: AudioEngine, quality: Quality) {
  const S = engine.settings;
  const flag = (sec: HTMLElement, label: string, f: number, title?: string) => panel.toggleBox(sec, label, () => (S.flags & f) !== 0, v => { S.flags = v ? S.flags | f : S.flags & ~f; }, title);

  // ---------------- frame (top-level knobs you actually reach for)
  const r = panel.section('Frame');
  panel.text(r, () => { const t = engine.timer; return `gpu: ${[...t.results.entries()].filter(([k]) => !['composite', 'post', 'fxaa', 'gbuffer'].includes(k)).map(([k, v]) => `${k} ${v.toFixed(2)}`).join(' · ')}`; });
  panel.select(r, 'quality preset', ['auto', ...QUALITY_NAMES], () => quality.auto ? 0 : 1 + QUALITY_NAMES.indexOf(quality.current), v => { if (v === 0) quality.setAuto(); else quality.pin(QUALITY_NAMES[v - 1]); }, 'resolution cap / scale, beam steps, cache probe density, smoke solver iterations (also ?quality=low|medium|high|ultra)');
  panel.toggleBox(r, 'adaptive resolution', () => quality.adaptive, v => { quality.adaptive = v; }, 'nudges the resolution scale down when the frame rate drops under ~27 fps and back up when there is headroom');
  panel.text(r, () => quality.lastChange ? `last change: ${quality.lastChange}` : '');
  panel.slider(r, 'max internal height', 480, 1600, 10, () => S.maxInternalHeight, v => { S.maxInternalHeight = v; }, 'caps internal render resolution (height in px)');
  panel.slider(r, 'resolution scale', 0.3, 1, 0.05, () => S.renderScale, v => { S.renderScale = v; });
  panel.select(r, 'debug view (`)', ['final', 'albedo', 'normals', 'direct light', 'indirect light', 'volumetrics', 'depth', 'cache irradiance (dice)', 'lighting only', 'denoiser blur hints'], () => S.debugView, v => { S.debugView = v; });
  panel.toggleBox(r, 'fxaa', () => engine.passes.fxaaEnabled, v => engine.passes.setFxaa(v));
  flag(r, 'dither', FrameFlags.Dither, '±½ LSB triangular noise against 8-bit banding');

  // ---------------- look (post: everything after the HDR composite)
  const lk = panel.section('Look');
  panel.slider(lk, 'exposure', 0.2, 8, 0.05, () => S.exposure, v => { S.exposure = v; }, 'linear gain before the AgX curve (default 2.0)');
  panel.slider(lk, 'bloom', 0, 0.3, 0.005, () => engine.passes.bloom, v => { engine.passes.bloom = v; }, 'fraction of the blurred HDR frame added back before tonemapping (no threshold: every pixel scatters a little, bright ones visibly)');
  panel.slider(lk, 'saturation', 0.6, 1.6, 0.05, () => engine.passes.saturation, v => { engine.passes.saturation = v; }, 'AgX look: 1 = neutral AgX (rather grey), 1.15 default');
  panel.slider(lk, 'indirect scale', 0, 2, 0.05, () => S.indirectScale, v => { S.indirectScale = v; }, 'multiplies the final-gather bounce on screen only (light meter / AI unaffected). 1 = physical, 0.35 default look');
  panel.slider(lk, 'emissive scale', 0, 4, 0.05, () => S.emissiveScale, v => { S.emissiveScale = v; }, 'brightness of emissive fixtures AND of the area lights derived from them');
  panel.slider(lk, 'night vision tube gain', 5, 1200, 5, () => S.nvGain, v => { S.nvGain = v; }, 'log gain of the intensifier: signal = log(1+L·g)/log(1+g)');
  panel.slider(lk, 'night vision phosphor', 0, 1, 0.01, () => engine.passes.nvPhosphor, v => { engine.passes.nvPhosphor = v; }, '0 = white phosphor (P45, modern), 1 = classic green (P43); default 0.09 = white with a touch of green');
  panel.slider(lk, 'film grain', 0, 0.1, 0.002, () => engine.passes.grain, v => { engine.passes.grain = v; }, 'display-space grain, faded out of highlights');

  // ---------------- direct light
  const dl = panel.section('Direct light', false);
  flag(dl, 'enabled', FrameFlags.Direct, 'sampled ray-traced direct light: every light scored per pixel, the strongest per emitter class get shadow rays to points on their emitters');
  panel.toggleBox(dl, 'per-tile light culling (lossless)', () => S.tileCull, v => { S.tileCull = v; }, 'each 8×8 pixel tile bounds its shading points and scores only the lights that can reach that box (in range, inside the spot cone, in front of a one-sided panel); a dropped light provably evaluates to zero at every pixel of the tile, so the image is bit-identical — off = every pixel scores every light (A/B, frame flag 4096)');
  panel.toggleBox(dl, 'trace grid: skip empty runs (lossless, all rays)', () => S.gridSkip, v => { S.gridSkip = v; }, 'every traced ray (shadow rays here, the final gather, cascade intervals, beams, probes) walks the 1 m XZ grid cell by cell; the grid build stores each cell\'s Chebyshev distance to the nearest occupied cell, so an empty cell at distance d lets the DDA take its next d−1 steps without loading anything (those cells are provably empty too). Same stepping arithmetic, one iteration per cell as before, so hits are bit-identical — off = a load pair per cell (A/B, frame flag 131072)');
  panel.toggleBox(dl, 'trace grid: height cull (lossless, all rays)', () => S.gridYCull, v => { S.gridYCull = v; }, 'the grid is 2D but the level is not: each cell also stores which 15 cm height slices its boxes occupy and each item its own padded [ymin, ymax]. A cell (or a single box) whose band the REST of the ray — from where it enters the cell to tmax / the current hit — cannot reach is not slab-tested: such a box could only have been hit behind the cell, where the walk already met it, so every dropped test was a miss or a no-op and the result is bit-identical (argument above traceClosest in common.wgsl). Rays skimming over desks toward ceiling panels, cascade intervals at chest height, gather rays under the furniture stop paying for geometry above/below them — off = every registered box tested (A/B, frame flag 262144)');
  panel.toggleBox(dl, 'slab test: unrotated fast path (lossless, all rays)', () => S.axisBoxFast, v => { S.axisBoxFast = v; }, 'a box with yaw 0 packs its rotation as exactly (cos 1, sin 0), and rotating by that is the identity down to the last bit (1·x − 0·z = x however the compiler contracts it), so the ray/box test can use the ray as it is and the inverse direction the traversal already holds instead of two half-float unpacks, two rotations and three divisions — that is two thirds of all grid items and every global. Off = always rotate + divide (A/B, frame flag 524288)');
  panel.toggleBox(dl, 'trace grid: globals by segment + tight spans (lossless, all rays)', () => S.gridSlabs, v => { S.gridSlabs = v; }, 'the handful of huge slabs kept out of the grid (ground, carpet base, corridor / break-room finishes, ceiling) used to be slab-tested by every ray whose HEIGHTS came within 4 cm of theirs — so a ray lifted 1–2 cm off the floor tested all four floors, four of its ~ten box tests. Now a global is skipped unless the ray segment\'s bounds meet its own on all three axes, 5 mm proud; and each cell\'s height span is judged 3.5 cm tighter at both ends (the packed extents carry 4 cm against < 0.4 mm of float differences), so a ray stops re-testing the desk / floor / wall top it just left. Every dropped test was a provable miss, so hits are bit-identical (argument above traceClosest in common.wgsl; tools/qa/grid-globals.ts: −40 % slab tests for rays off the floor, −60 % outdoors, −19 % cascades) — off = the height cull as it was (A/B, frame flag 1048576)');
  panel.text(dl, () => { const g = engine.world.gridStats; return `grid ${g.cells} cells · ${g.empty} empty (mean look-ahead ${g.meanSkip.toFixed(1)}) · busiest cell ${g.maxPerCell} boxes · ${engine.world.itemCount} items · slack ${(g.slack * 1000).toFixed(2)} mm`; });
  panel.toggleBox(dl, 'penumbra filter: skip hint-free tiles (lossless)', () => S.softTileSkip, v => { S.softTileSkip = v; }, 'the à-trous penumbra filter only ever changes pixels within reach of a blur hint (a shadow-ray blocker measurement); two tiny pre-passes flag the 8×8 tiles holding a hint and measure every tile\'s distance to the nearest one, and the filter steps copy whole tiles straight through when they are further away than a hint can travel over the entire chain — those provably pass through unchanged anyway, so the image is bit-identical while lit floors, unlit rooms and sky stop paying for the 3×3 hint scan. Off = every pixel decides for itself (A/B, frame flag 8192)');
  panel.toggleBox(dl, 'LOSSY: checkerboard shadow rays', () => S.checkerDirect, v => { S.checkerDirect = v; }, 'only the checkerboard half of the pixels with (x + y + frame) even trace shadow rays; the other half keeps its exact unshadowed light U and takes S = U × the visibility ratio S/U of its traced 4-neighbours in the tile (weighted by same plane / same normal / lit alike, hint = the widest in reach). Pixels on a hard shadow edge (neighbour ratios > 0.35 apart under < 2 px hints) or without a usable neighbour trace anyway, after the exchange. Lanes are dealt so whole SIMD groups skip the ray phase. Off = untouched path (frame flag 65536)');
  panel.toggleBox(dl, 'LOSSY: 1 ray per leader in near-black', () => S.dimRays, v => { S.dimRays = v; }, 'luminance-adaptive shadow rays: a pixel whose total UNshadowed irradiance (both classes, exact) is under the threshold below takes one stratified emitter sample per leader instead of the strongest/other budgets — its penumbrae come out grainier, but at that level the AgX curve puts the whole pixel within a few 8-bit steps of black. Off = untouched (frame.lossyCfg.z = 0)');
  panel.slider(dl, 'near-black threshold (E)', 0.002, 0.05, 0.002, () => S.dimRaysBelow, v => { S.dimRaysBelow = v; }, 'unshadowed irradiance luminance under which dimRays applies: 0.01 ≈ 2 % of display white for a mid-grey surface at exposure 2 (0.005 ≈ 1 %, 0.02 ≈ 4 %); only read while the toggle above is on');
  panel.slider(dl, 'rays: strongest light', 1, 8, 1, () => S.directSamplesTop, v => { S.directSamplesTop = v; }, 'stratified emitter samples per pixel for each class\'s strongest light (noise ~ 1/sqrt(n); cost ~ n)');
  panel.slider(dl, 'rays: other leaders', 1, 4, 1, () => S.directSamplesOther, v => { S.directSamplesOther = v; }, 'samples for the 2nd (and, broad class, 3rd) strongest light');
  panel.slider(dl, 'temporal reuse', 0, 0.95, 0.05, () => S.directHistory, v => { S.directHistory = v; }, '0 = history-free. Otherwise the per-class estimate is blended with reprojected, depth-validated, variance-clipped history (0.7 ≈ 3 frames). Transient lights change the estimate abruptly, which the clip follows — no lingering flashes');
  panel.slider(dl, 'spatial passes', 0, 6, 1, () => S.directPasses, v => { S.directPasses = v; }, 'à-trous passes of the ratio (visibility) filter per class: strides 1,2,4,8,16,1 — fewer = sharper/grainier');
  panel.slider(dl, 'penumbra filter cap (px)', 2, 32, 1, () => S.directBlurCap, v => { S.directBlurCap = v; }, 'upper bound on the PCSS-derived filter radius');
  panel.slider(dl, 'adaptive: min agreeing rays', 2, 4, 1, () => S.directAdaptiveMin, v => { S.directAdaptiveMin = v; }, 'stop sampling a light after this many stratified samples all came back lit (or all blocked); the full ray budget is only spent where they disagree, i.e. in penumbrae. 2 = cheapest, 4 = safest');
  panel.slider(dl, 'cascade hit light threshold', 0.001, 0.1, 0.001, () => S.secMinE, v => { S.secMinE = v; }, 'W/m² (unshadowed): at cascade interval hits a light dimmer than this gets no shadow ray and is dropped — raises bounce-lighting cost/accuracy trade (default 0.006)');
  flag(dl, 'tail: chroma-exact (experimental)', FrameFlags.TailChroma, 'direct-light tail estimator: off = luminance-unbiased, bounded (the tail keeps its aggregate hue); on = per-channel unbiased E_pick·V/p (two differently coloured lights cast two coloured shadows) at the price of chroma noise where hues mix — the filter\'s ratio clamp opens to 4× for it');
  panel.slider(dl, 'torch emitter radius (m)', 0, 0.15, 0.005, () => S.flashlightRadius, v => { S.flashlightRadius = v; }, 'size of the flashlight front for shadow purposes (guards; the rail light uses 0.7×). A real bezel is ~1.5-2 cm → crisp shadows; larger = penumbrae that widen with distance');
  flag(dl, 'denoiser', FrameFlags.SoftShadow, 'master switch for temporal + spatial (off = raw samples, for inspection)');
  flag(dl, 'volumetric beams', FrameFlags.Volumetrics, 'in-scattered light from spot/point lights in the thin haze (equi-angular, shadow-rayed)');
  panel.slider(dl, 'haze density', 0, 0.2, 0.005, () => S.hazeDensity, v => { S.hazeDensity = v; }, 'uniform thin haze that makes light beams visible');
  panel.slider(dl, 'beam march steps', 4, 40, 1, () => S.volSteps, v => { S.volSteps = v; });
  panel.toggleBox(dl, 'LOSSY: volumetrics at 1/4 res (not 1/2)', () => S.volQuarter, v => { S.volQuarter = v; }, 'haze beams + smoke march on a quarter-res target (each texel marches one pixel of its 4×4 block, interleaved), composited with the same depth-aware 3×3 upsample scaled to the texel spacing; frame.lossyCfg.y carries the divisor, the target is recreated on change. Off = the half-res path untouched');
  panel.text(dl, () => `volumetric target ${engine.passes.vw}×${engine.passes.vh} (1/${engine.passes.volDiv})`);
  flag(dl, 'smoke rendering', FrameFlags.Smoke);
  flag(dl, 'smoke shadows', FrameFlags.SmokeShadows, 'shadow rays are attenuated by smoke volumes (lights dim behind a cloud)');

  // ---------------- indirect light
  const il = panel.section('Indirect light', false);
  flag(il, 'enabled', FrameFlags.Indirect);
  flag(il, 'temporal accumulation', FrameFlags.Temporal, 'reprojected, depth-validated, variance-clipped history on the half-res indirect only (also drives the cache jitter/EMA); direct light has its own temporal-reuse knob above; beams never use history');
  flag(il, 'multi-bounce', FrameFlags.Bounce, 'hit points read the cached irradiance volume → light keeps bouncing');
  panel.toggleBox(il, 'LOSSY: gather at 1/3 res (not 1/2)', () => S.gatherThird, v => { S.gatherThird = v; }, 'final gather, its temporal accumulation and the à-trous run at a third of the internal resolution (4/9 of the pixels of half res); fgather / temporal / giblur / the composite upsample read the divisor from frame.lossyCfg.x and the targets are recreated when it changes. Off = the half-res path untouched');
  panel.text(il, () => `gather targets ${engine.passes.gw}×${engine.passes.gh} (1/${engine.passes.gatherDiv})`);
  // the world-space cache behind the gather (radiance-cascade probe volume): rarely worth touching now
  const rcs = panel.section('Indirect cache (probe volume)', false);
  panel.text(rcs, () => `grid ${engine.rc.cfg.nx}×${engine.rc.cfg.ny}×${engine.rc.cfg.nz} (${(40 / engine.rc.cfg.nx).toFixed(2)} m) · ${engine.rc.cfg.cascades} cascades · ${engine.rc.cfg.d0 * engine.rc.cfg.d0}→${(engine.rc.cfg.d0 << (engine.rc.cfg.cascades - 1)) ** 2} dirs · ${(engine.rc.stats.intervals / 1e6).toFixed(2)}M intervals/frame · ${(engine.rc.stats.bytes / 1048576).toFixed(1)} MB`);
  let pendingCascades = engine.rc.cfg.cascades, pendingNx = engine.rc.cfg.nx;
  const RC_GRIDS: { label: string; nx: number; ny: number; nz: number }[] = [ { label: '1 m (40×4×28) — default', nx: 40, ny: 4, nz: 28 }, { label: '0.67 m (60×5×42)', nx: 60, ny: 5, nz: 42 }, { label: '0.5 m (80×6×56)', nx: 80, ny: 6, nz: 56 }, { label: '1.25 m (32×3×22)', nx: 32, ny: 3, nz: 22 } ];
  const gridIndex = () => Math.max(0, RC_GRIDS.findIndex(g => g.nx === pendingNx));
  panel.select(rcs, 'probe density', RC_GRIDS.map(g => g.label), gridIndex, v => { pendingNx = RC_GRIDS[v].nx; }, 'read one bounce away from the eye, so density barely shows through the gather');
  panel.slider(rcs, 'cascade count', 3, 5, 1, () => pendingCascades, v => { pendingCascades = v; });
  panel.buttons(rcs, '', [{ text: 'rebuild cache', run: () => { const gsel = RC_GRIDS[gridIndex()]; engine.rebuildRc({ ...defaultRcConfig, nx: gsel.nx, ny: gsel.ny, nz: gsel.nz, cascades: pendingCascades, interval0: engine.rc.cfg.interval0 }); } }]);
  panel.slider(rcs, 'base interval (m)', 0.2, 2.0, 0.05, () => engine.rc.cfg.interval0, v => engine.rc.setInterval0(v), 'cascade n traces [L·(4ⁿ−1)/3, L·(4ⁿ⁺¹−1)/3]');
  panel.toggleBox(rcs, 'stagger upper cascades', () => engine.rc.staggerUpper, v => { engine.rc.staggerUpper = v; }, 'update c2+ every other frame (every fourth with the half-rate option below)');
  panel.toggleBox(rcs, 'LOSSY: cache at half rate', () => S.rcHalfRate, v => { S.rcHalfRate = v; }, 'upper cascades trace on even frames, c0 + the dice bake on odd ones (one direction jitter per pair, so merges and the bake stay consistent); the dice volume the gather / composite / volumetrics / light meter read is at most one frame older. Resting passes still show in the HUD as ~0 ms entries so the smoothed numbers average honestly. Off = every cascade every frame, untouched');
  flag(rcs, 'probe visibility weights', FrameFlags.RcVisWeight, 'ray-traced probe visibility when merging cascades (thin-wall leak fix)');

  // ---------------- lights
  const l = panel.section('Lights');
  const groups = [...new Set(game.targets.filter(t => t.kind !== 'breaker' && t.group !== 'emergency').map(t => t.group))];   // breaker / beacons are driven by the blackout logic, not switches
  panel.buttons(l, 'all switchable', [{ text: 'on', run: () => groups.forEach(g => game.setGroup(g, true)) }, { text: 'off', run: () => groups.forEach(g => game.setGroup(g, false)) }, { text: 'repair', run: () => game.repairLights() }]);
  panel.buttons(l, 'mains', [{ text: 'trip 30 s', run: () => game.setBlackout(30, 'debug') }, { text: 'kill', run: () => game.setBlackout(Infinity, 'debug') }, { text: 'restore', run: () => game.endBlackout() }]);
  for (const g of groups) {
    const ts = game.targets.filter(t => t.group === g);
    panel.buttons(l, `${g} (${ts.length})`, [{ text: 'on', run: () => game.setGroup(g, true) }, { text: 'off', run: () => game.setGroup(g, false) }]);
  }
  const moon = engine.lights.lights.find(x => x.name === 'moon');
  if (moon) { panel.slider(l, 'moonlight', 0, 4, 0.05, () => moon.intensity, v => { moon.intensity = v; moon.enabled = v > 0; }); }
  panel.slider(l, 'guard flashlight intensity', 0, 300, 5, () => game.tune.flashIntensity, v => { game.tune.flashIntensity = v; });
  panel.slider(l, 'pistol light intensity', 0, 300, 5, () => game.tune.playerLight, v => { game.tune.playerLight = v; });
  panel.slider(l, 'flashlight outer °', 8, 45, 1, () => game.tune.flashOuter, v => { game.tune.flashOuter = v; game.tune.flashInner = Math.min(game.tune.flashInner, v - 2); });
  panel.slider(l, 'flashlight inner °', 1, 30, 1, () => game.tune.flashInner, v => { game.tune.flashInner = Math.min(v, game.tune.flashOuter - 2); });
  panel.slider(l, 'OCP disable secs', 2, 30, 1, () => game.tune.ocpDuration, v => { game.tune.ocpDuration = v; });
  let skyScale = 1;
  const z0 = [...S.skyZenith], h0 = [...S.skyHorizon];
  panel.slider(l, 'sky brightness', 0, 5, 0.05, () => skyScale, v => { skyScale = v; S.skyZenith = [z0[0] * v, z0[1] * v, z0[2] * v]; S.skyHorizon = [h0[0] * v, h0[1] * v, h0[2] * v]; });

  // ---------------- smoke
  const sm = panel.section('Smoke');
  panel.text(sm, () => `domains live ${smoke.stats.live}/${smoke.domains.length} · emitters ${smoke.stats.emitters} · ${smoke.stats.steps} dispatches · sim ${engine.timer.results.get('smoke')?.toFixed(2) ?? '-'} ms`);
  panel.toggleBox(sm, 'simulate', () => smoke.params.enabled, v => { smoke.params.enabled = v; });
  panel.toggleBox(sm, 'skip empty bricks (lossless)', () => smoke.params.brickSkip, v => { smoke.params.brickSkip = v; }, 'the scalar passes (advection + inject/decay) early-out in 8³ bricks that provably hold nothing this step; velocity / pressure stay full-domain (pressure is global). Occupancy below.');
  panel.toggleBox(sm, 'render: skip empty bricks (lossless)', () => S.smokeRenderSkip, v => { S.smokeRenderSkip = v; }, 'every smoke read the renderer and the AI make (volumetric march + self-shadow taps, smoke on shadow rays / haze beams / probe sight lines) first tests one bit per 8³ brick that the solver packs at the end of each step from the very density texture this frame samples: set = the brick or the one-cell shell around it holds density. A trilinear fetch only ever touches the sample\'s cell ±1, so a clear bit means the fetch returns exactly 0 and it is skipped — bit-identical image, the empty part of every domain stops costing fetches ("render N" below = bricks still fetched). Off = always fetch (A/B, frame flag 32768)');
  panel.text(sm, () => `bricks active ${smoke.stats.bricksActive}/${smoke.stats.bricksTotal} · scalar ${smoke.stats.bricksScalar} · vel ${smoke.stats.bricksVel} · render ${smoke.stats.bricksRender} · vmax ${smoke.stats.vmax.toFixed(1)} m/s`);
  panel.slider(sm, 'flush ε (density)', 0, 0.0002, 0.00001, () => smoke.params.flushEps, v => { smoke.params.flushEps = v; }, 'density / temperature at or below this store as exactly 0 (default 2⁻¹⁴ ≈ 6e-5: below the half-float normal range decay stalls and cells never empty; 6 m of it moves an 8-bit pixel by less than half a step). 0 = never flush.');
  panel.buttons(sm, '', [{ text: 'canister at cursor', run: () => smoke.spawnCanister(game.aimPoint) }, { text: 'clear', run: () => smoke.clearAll() }]);
  panel.slider(sm, 'buoyancy', 0, 6, 0.1, () => smoke.params.buoyancy, v => { smoke.params.buoyancy = v; });
  panel.slider(sm, 'vorticity conf.', 0, 15, 0.25, () => smoke.params.vortConf, v => { smoke.params.vortConf = v; });
  panel.slider(sm, 'turbulence', 0, 10, 0.25, () => smoke.params.turbulence, v => { smoke.params.turbulence = v; });
  panel.slider(sm, 'density decay /s', 0, 1, 0.01, () => smoke.params.densDecay, v => { smoke.params.densDecay = v; });
  panel.slider(sm, 'temp decay /s', 0, 4, 0.05, () => smoke.params.tempDecay, v => { smoke.params.tempDecay = v; });
  panel.slider(sm, 'velocity damping', 0, 2, 0.05, () => smoke.params.velDamp, v => { smoke.params.velDamp = v; });
  panel.slider(sm, 'pressure: rb pairs', 1, 10, 1, () => smoke.params.rbIters, v => { smoke.params.rbIters = v; }, 'red-black Gauss-Seidel + over-relaxation: red/black sweep pairs per step (2 dispatches each; the quality preset sets it)');
  panel.slider(sm, 'pressure: omega (SOR)', 1, 1.95, 0.05, () => smoke.params.omega, v => { smoke.params.omega = v; });
  panel.toggleBox(sm, 'LOSSY: pressure at half res', () => smoke.params.pressureHalf, v => { smoke.params.pressureHalf = v; }, 'the pressure Poisson solve runs on a 32³ grid per domain instead of 64³: the fine flux balances are summed over 2×2×2 blocks face by face (a coarse face carries how many of its 4 fine face pairs are open, so the coarse system is sealed exactly where the fine grid is — partitions, desk tops — while one-cell fluid layers along walls and the ceiling keep their equations), the same rb pairs / omega sweep the blocks (warm-started; zeroed on placement and on every switch-on), the result is interpolated back trilinearly along open connections only, and the unchanged projection subtracts its gradient. Advection, curl, buoyancy, injection stay 64³. One ordinary full-res rb pair then runs on the result (params.pressureHalfSmooth, default 1: without it the grid-scale divergence of the plume skin, invisible to a 2×2×2 block, piles up — a CPU soak measured 5× the residual and +40 % mass — with it the residual sits at or under the full-res path\'s). Costs divergence under two cells wide: softer billow at the canister mouth, slightly puffier plumes. stats.pressure / stats.perStep say which solve ran and its dispatches per domain step (fine 19 at 4 pairs, half 23 = 21 + the pair). Off = the untouched full-res path; the option\'s shaders only compile on first switch-on (stats.halfStatus)');
  panel.slider(sm, 'render density ×', 0.1, 4, 0.05, () => smoke.params.densityScale, v => { smoke.params.densityScale = v; });
  panel.slider(sm, 'wind x', -1, 1, 0.02, () => smoke.params.windX, v => { smoke.params.windX = v; });
  panel.slider(sm, 'wind z', -1, 1, 0.02, () => smoke.params.windZ, v => { smoke.params.windZ = v; });
  panel.slider(sm, 'canister density /s', 5, 200, 1, () => smoke.params.canisterDensity, v => { smoke.params.canisterDensity = v; }, 'applies to canisters that start emitting after the change');
  panel.slider(sm, 'canister duration s', 3, 60, 1, () => smoke.params.canisterDuration, v => { smoke.params.canisterDuration = v; });
  panel.slider(sm, 'canister radius m', 0.05, 0.4, 0.01, () => smoke.params.canisterRadius, v => { smoke.params.canisterRadius = v; });
  panel.slider(sm, 'canister heat', 0, 8, 0.1, () => smoke.params.canisterTemp, v => { smoke.params.canisterTemp = v; }, 'buoyancy source: low = smoke hugs the floor and spreads, high = rises to the ceiling');
  panel.slider(sm, 'canister jet m/s', 0, 5, 0.1, () => smoke.params.canisterSpeed, v => { smoke.params.canisterSpeed = v; });
  panel.buttons(sm, 'player', [{ text: '+4 canisters', run: () => { game.player.canisters += 4; } }]);
  panel.slider(sm, 'fine voxel (cm)', 1.5, 4, 0.1, () => smoke.params.voxelFine * 100, v => { smoke.params.voxelFine = v / 100; }, 'gun smoke domains (applies to newly placed domains)');
  panel.slider(sm, 'coarse voxel (cm)', 3, 9, 0.1, () => smoke.params.voxelCoarse * 100, v => { smoke.params.voxelCoarse = v / 100; }, 'canister domains (applies to newly placed domains)');

  // ---------------- effects (the graded numbers in src/game/effects.ts, live)
  const ef = panel.section('Effects', false);
  panel.text(ef, () => `sparks live ${game.sparks.live}/${fx.sparks.maxLive} · smoke emitters ${smoke.stats.emitters} (room for ${smoke.budget()})`);
  panel.buttons(ef, 'at cursor', [{ text: 'stun can', run: () => game.dropThrowable('flash', game.aimPoint) }, { text: 'smoke can (vent)', run: () => game.dropThrowable('smoke', game.aimPoint) }, { text: '+2 stun / +4 smoke', run: () => { game.player.flashbangs += 2; game.player.canisters += 4; } }]);
  panel.slider(ef, 'shot light power', 0, 400, 5, () => fx.shot.lightPower, v => { fx.shot.lightPower = v; }, 'peak intensity of the warm muzzle light (guards ×1, the suppressed Five-seveN × suppressedGain)');
  panel.slider(ef, 'shot light s', 0.02, 0.3, 0.01, () => fx.shot.lightDuration, v => { fx.shot.lightDuration = v; });
  panel.slider(ef, 'shot jet density /s', 0, 120, 1, () => fx.shot.jetDensity, v => { fx.shot.jetDensity = v; }, 'bore jet; the two side ports carry portFraction of it');
  panel.slider(ef, 'shot jet heat /s', 0, 400, 5, () => fx.shot.jetTemp, v => { fx.shot.jetTemp = v; });
  panel.slider(ef, 'shot wisp density /s', 0, 30, 0.5, () => fx.shot.wispDensity, v => { fx.shot.wispDensity = v; });
  panel.slider(ef, 'shot wisp heat /s', 0, 150, 1, () => fx.shot.wispTemp, v => { fx.shot.wispTemp = v; }, 'buoyancy: what makes the thread climb off the barrel on its own');
  panel.slider(ef, 'shot sparks', 0, 48, 1, () => fx.shot.sparkCount, v => { fx.shot.sparkCount = v; });
  panel.slider(ef, 'shot spark speed m/s', 2, 40, 0.5, () => fx.shot.sparkSpeed, v => { fx.shot.sparkSpeed = v; }, 'streak length = speed × shutter');
  panel.slider(ef, 'shot spark cone rad', 0.05, 1.2, 0.01, () => fx.shot.sparkCone, v => { fx.shot.sparkCone = v; });
  panel.slider(ef, 'shot spark life s', 0.03, 0.6, 0.01, () => fx.shot.sparkLife, v => { fx.shot.sparkLife = v; });
  const SP = game.sparks.params;
  panel.slider(ef, 'spark gravity', 0, 30, 0.5, () => SP.gravity, v => { SP.gravity = v; });
  panel.slider(ef, 'spark drag /s', 0, 6, 0.1, () => SP.drag, v => { SP.drag = v; });
  panel.slider(ef, 'spark brightness ×', 0.1, 8, 0.1, () => SP.brightness, v => { SP.brightness = v; });
  panel.slider(ef, 'spark hold (frac of life)', 0, 0.9, 0.02, () => SP.hold, v => { SP.hold = v; }, 'fraction of its life a spark stays white-hot before fading orange → red');
  panel.slider(ef, 'spark streak thickness mm', 2, 30, 1, () => SP.thickness * 1000, v => { SP.thickness = v / 1000; });
  panel.slider(ef, 'spark shutter ms', 2, 50, 1, () => SP.shutter * 1000, v => { SP.shutter = v / 1000; }, 'how much motion one streak shows');
  panel.toggleBox(ef, 'ejection-port curl', () => fx.flags.ejectionWisp, v => { fx.flags.ejectionWisp = v; }, 'the small second wisp the shipped look had (not in the reference set)');
  panel.slider(ef, 'stun light power', 0, 3000, 20, () => fx.bang.lightPower, v => { fx.bang.lightPower = v; }, 'white transient point light, history-free: safe to make absurd');
  panel.slider(ef, 'stun light s', 0.03, 0.5, 0.01, () => fx.bang.lightDuration, v => { fx.bang.lightDuration = v; });
  panel.slider(ef, 'stun density ×', 0, 4, 0.05, () => fx.bang.densityGain, v => { fx.bang.densityGain = v; }, 'multiplies the jets, body vents, wisp and trails (base numbers in effects.ts)');
  panel.slider(ef, 'stun heat ×', 0, 3, 0.05, () => fx.bang.tempGain, v => { fx.bang.tempGain = v; });
  panel.slider(ef, 'stun sparks', 0, 96, 1, () => fx.bang.sparkCount, v => { fx.bang.sparkCount = v; });
  panel.slider(ef, 'stun smoke trails', 0, 20, 1, () => fx.bang.trailCount, v => { fx.bang.trailCount = v; }, 'smoke emitters riding the first N sparks (charged against the solver\'s 32 emitter slots)');
  panel.slider(ef, 'stun fuse s', 0.2, 4, 0.1, () => fx.bang.fuse, v => { fx.bang.fuse = v; }, 'from first contact with the floor / a prop');
  panel.slider(ef, 'dazzle radius m', 0, 16, 0.5, () => fx.bang.dazzleRadius, v => { fx.bang.dazzleRadius = v; });
  panel.slider(ef, 'dazzle seconds', 0, 10, 0.25, () => fx.bang.dazzleSeconds, v => { fx.bang.dazzleSeconds = v; fx.bang.dazzleMin = Math.min(fx.bang.dazzleMin, v); });
  panel.toggleBox(ef, 'dazzle guards', () => fx.flags.dazzle, v => { fx.flags.dazzle = v; });
  panel.slider(ef, 'spark brightness ×', 0, 8, 0.1, () => fx.sparks.brightness, v => { fx.sparks.brightness = v; }, `× ${fx.sparks.emissive} HDR at white heat; the bloom does the glow`);
  panel.slider(ef, 'spark thickness cm', 0.3, 4, 0.1, () => fx.sparks.thickness * 100, v => { fx.sparks.thickness = v / 100; });
  panel.slider(ef, 'spark gravity', 0, 40, 0.5, () => fx.sparks.gravity, v => { fx.sparks.gravity = v; });
  panel.slider(ef, 'spark drag /s', 0, 8, 0.05, () => fx.sparks.drag, v => { fx.sparks.drag = v; });
  panel.toggleBox(ef, 'sparks hit geometry', () => fx.sparks.collide, v => { fx.sparks.collide = v; }, 'ray cast fast sparks against walls / props / doors; off = floor only');
  panel.toggleBox(ef, 'slot-2 can = floor vent', () => fx.flags.ventCanister, v => { fx.flags.ventCanister = v; }, 'ON: the ported cold directional jet along the throw (effects.ts `vent`). OFF: the old upward canister jet driven by the Smoke section sliders (T key always uses that one)');
  panel.slider(ef, 'vent density /s', 0, 80, 1, () => fx.vent.density, v => { fx.vent.density = v; });
  panel.slider(ef, 'vent jet m/s', 0, 8, 0.1, () => fx.vent.speed, v => { fx.vent.speed = v; });
  panel.slider(ef, 'vent radius m', 0.1, 0.6, 0.01, () => fx.vent.radius, v => { fx.vent.radius = v; });
  panel.toggleBox(ef, 'expansion sources', () => smoke.params.expansion, v => { smoke.params.expansion = v; }, 'solver: honour emitters\' `expand` (pressure-driven divergence + density volume correction). Off = jets only, for A/B');

  // ---------------- gameplay
  const gp = panel.section('Sandbox');
  panel.buttons(gp, 'sandbox', [
    { text: 'trip / restore mains', run: () => { if (game.blackout.active) game.endBlackout(); else game.setBlackout(30, 'debug'); }, title: 'blackout: everything on the breaker goes dark for 30 s, beacons spin up (again = restore)' },
    { text: 'smoke at cursor', run: () => { smoke.spawnCanister(game.aimPoint); game.msg('smoke canister deployed'); } },
    { text: 'clear smoke', run: () => { smoke.clearAll(); game.msg('smoke cleared'); } },
    { text: 'teleport to cursor', run: () => game.teleportPlayer(game.aimPoint) },
    { text: 'spectate next', run: () => { game.spectate = game.spectate + 1 >= game.guards.length ? -1 : game.spectate + 1; game.msg(game.spectate < 0 ? 'camera: player' : `camera: guard ${game.spectate}`); }, title: 'cycle the camera through the guards and back to the player' },
  ]);   // (these were the B / T / K / Y / . hotkeys)
  panel.toggleBox(gp, 'guard AI', () => game.aiEnabled, v => { game.aiEnabled = v; });
  panel.toggleBox(gp, 'god mode', () => game.godMode, v => { game.godMode = v; }, 'guards still see and shoot you, but hits do no damage');
  panel.toggleBox(gp, 'unlimited ammo', () => game.infiniteAmmo, v => { game.infiniteAmmo = v; }, 'bottomless magazine: rounds still cycle (cadence, reload animation if you press R) but the count never drops');
  panel.buttons(gp, 'encounter', [{ text: 'restart (Shift+Enter)', run: () => game.restartEncounter() }]);
  panel.toggleBox(gp, 'show AI debug labels', () => game.showDebug, v => { game.showDebug = v; }, 'state / awareness tags over the guards (was the H key)');
  panel.toggleBox(gp, 'light fixture dots', () => overlay.showLightDots, v => { overlay.showLightDots = v; });
  panel.slider(gp, 'detection rate', 0.2, 5, 0.1, () => game.tune.detectRate, v => { game.tune.detectRate = v; });
  panel.slider(gp, 'visibility floor E', 0.005, 0.2, 0.005, () => game.tune.visFloor, v => { game.tune.visFloor = v; }, 'irradiance at the player that reads as fully hidden (0%)');
  panel.slider(gp, 'visibility slope', 0.05, 0.5, 0.01, () => game.tune.visSlope, v => { game.tune.visSlope = v; }, 'visibility = slope · ln(E / floor)');
  panel.slider(gp, 'guard walk m/s', 0.5, 2.5, 0.05, () => game.tune.guardWalk, v => { game.tune.guardWalk = v; });
  panel.slider(gp, 'player walk m/s', 0.5, 3, 0.05, () => game.tune.playerWalk, v => { game.tune.playerWalk = v; });
  panel.slider(gp, 'player sprint m/s', 3, 7, 0.1, () => game.tune.playerSprint, v => { game.tune.playerSprint = v; });
  panel.slider(gp, 'anim walk speed', 0.8, 2.5, 0.05, () => game.player.char.anim.params.walkSpeed, v => { for (const c of [game.player.char, ...game.guards.map(g => g.char)]) c.anim.params.walkSpeed = v; }, 'ground speed at which the walk cycle plays at 1× (foot-slide tuning)');
  panel.slider(gp, 'anim jog speed', 2, 5, 0.05, () => game.player.char.anim.params.jogSpeed, v => { for (const c of [game.player.char, ...game.guards.map(g => g.char)]) c.anim.params.jogSpeed = v; });
  panel.buttons(gp, 'guards', [{ text: 'reset', run: () => game.resetGuards() }, { text: 'alert all', run: () => game.guards.forEach(g => { if (g.state !== 'dead') { g.awareness = 1; g.lastKnown = [...game.player.char.pos]; } }) }, { text: 'calm all', run: () => game.guards.forEach(g => { if (g.state !== 'dead') { g.awareness = 0; g.state = 'patrol'; g.path = []; } }) }]);
  panel.text(gp, () => game.escalationSummary());   // alarm escalation: calm / heightened / lockdown, seconds left on that level's clock, and the lockdown pair + post (read-only)
  panel.buttons(gp, 'alarm level', [{ text: 'raise', run: () => game.escalate(), title: 'one step up by hand, as if an alarm had just found nobody: calm → heightened (pistols out on patrol) → lockdown (pair + post, doors pulled to, the pair clears the rooms nearest the last fix)' }, { text: 'stand down', run: () => game.standDown(), title: 'straight back to calm: torches out, own routes, a clear in progress called off (the men keep whatever suspicion they have)' }]);
  panel.text(gp, () => game.clearingSummary());   // 'clearing: <room> · <stage / drill phase> · who · then … · seconds' while the lockdown pair clears (read-only; also __game.clearing)
  panel.buttons(gp, 'room clearing', [{ text: 'clear nearest room now', run: () => game.clearNearestRoom(), title: 'test: puts the alarm fix where the player stands and locks the floor down (or re-plans the running lockdown) — the pair walks over, stacks on that room\'s door, opens it hard, hooks in to their corners, sweeps, calls it and moves to the next nearest; a shot, a sighting, a bump or a creak breaks it off into the normal alert AI' }]);
  panel.slider(gp, 'rooms per clear (0 = auto)', 0, 5, 1, () => game.clearRooms, v => { game.clearRooms = v; }, '0 = by what raised the lockdown: a body found → the 3 rooms nearest him, an alarm that came to nothing → the 2 nearest the last fix; 1‥5 = exactly that many. Kicked-in / picked doors\' rooms jump the queue; a room called clear in the last minute of the same lockdown is skipped unless the new fix is inside it (applies to the next clear)');
  panel.text(gp, () => game.clearQueueSummary());   // 'clear queue: A ▸ B ▸ C · cleared this lockdown: X (23 s ago, holds)' (read-only)
  panel.buttons(gp, 'player', [{ text: 'teleport to cursor', run: () => game.teleportPlayer(game.aimPoint) }, { text: 'refill ammo', run: () => { game.player.pistol.spare = [10, 10, 10]; game.player.pistol.mag = 10; game.player.pistol.chamber = 1; } }]);
  panel.buttons(gp, 'door locks', [{ text: 'relock (as authored)', run: () => { for (const d of game.doors.list) { d.locked = !!d.def.locked; d.lockBroken = false; d.noticed = false; d.pick = 0; } }, title: 'server / storage / manager locked again on the corridor face, kicked keeps mended, pick progress gone (leaves stay where they are)' }, { text: 'unlock all', run: () => { for (const d of game.doors.list) d.locked = false; } }]);
  panel.text(gp, () => game.doors.list.filter(d => d.def.locked).map(d => `${d.def.name}: ${d.lockBroken ? 'kicked in' : d.locked ? (d.pick > 0 ? `locked · ${Math.round(d.pick * 100)}% picked` : 'locked') : 'picked'}${d.noticed ? ' (noticed)' : ''}`).join(' · '));
  panel.text(gp, () => `props ${game.props.props.length} · awake ${game.props.stats.awake} · solver ${game.props.stats.ms.toFixed(2)} ms`);
  panel.toggleBox(gp, 'prop physics', () => game.props.enabled, v => { game.props.enabled = v; }, 'chairs / cartons / bins / plants get shoved by characters, bullets, canisters and door leaves (and guards hear it); off = frozen where they stand, still solid');
  panel.slider(gp, 'prop friction ×', 0.2, 3, 0.05, () => game.props.frictionScale, v => { game.props.frictionScale = v; }, 'ground friction multiplier: low = chairs roll across the room, high = everything stops dead');
  panel.buttons(gp, 'props', [{ text: 'reset', run: () => game.props.reset() }, { text: 'scatter', run: () => game.props.scatter(), title: 'random shove on every prop (solver smoke test)' }]);
  panel.select(gp, 'camera follows', ['player', ...game.guards.map((_, i) => `guard ${i} (${game.level.routes[i].name})`)], () => game.spectate + 1, v => { game.spectate = v - 1; });

  // ---------------- audio
  const au = panel.section('Audio', false);
  panel.text(au, () => audio.started ? `voices ${audio.stats.voices} · reverb send ${audio.stats.wet.toFixed(2)} (traced free path) · occluded plays ${audio.stats.occluded} · routed around corners ${audio.stats.routed}` : 'click / press a key in the view to start audio');
  panel.toggleBox(au, 'sound', () => audio.enabled, v => { audio.enabled = v; audio.setVolumes(); });
  panel.toggleBox(au, 'music', () => audio.music?.enabled ?? true, v => { if (audio.music) audio.music.enabled = v; });
  panel.slider(au, 'master', 0, 1, 0.01, () => audio.volume.master, v => { audio.volume.master = v; audio.setVolumes(); });
  panel.slider(au, 'effects', 0, 1.5, 0.01, () => audio.volume.sfx, v => { audio.volume.sfx = v; audio.setVolumes(); });
  panel.slider(au, 'music', 0, 1, 0.01, () => audio.volume.music, v => { audio.volume.music = v; audio.setVolumes(); });
  panel.slider(au, 'ambience', 0, 1.5, 0.01, () => audio.volume.ambience, v => { audio.volume.ambience = v; audio.setVolumes(); });
  let forced = -1;
  panel.slider(au, 'force tension', -1, 1, 0.05, () => forced, v => { forced = v; if (audio.music) audio.music.intensity = v < 0 ? game.tension() : v; }, '-1 = follow the game; otherwise pin the score intensity');
  setInterval(() => { window.__forcedTension = forced >= 0; if (forced >= 0 && audio.music) audio.music.intensity = forced; }, 200);

  // ---------------- camera
  const c = panel.section('Camera', false);
  panel.slider(c, 'pitch °', 25, 85, 1, () => cam.pitch / DEG, v => { cam.pitch = v * DEG; });
  panel.slider(c, 'yaw °', -180, 180, 1, () => cam.yaw / DEG, v => { cam.yaw = v * DEG; });
  panel.slider(c, 'distance', 5, 45, 0.5, () => cam.desiredDistance, v => { cam.desiredDistance = v; });
  panel.slider(c, 'fov °', 12, 70, 1, () => cam.fov / DEG, v => { cam.fov = v * DEG; });
  panel.slider(c, 'aim lead', 0, 0.5, 0.01, () => cam.aimLead, v => { cam.aimLead = v; });
}
