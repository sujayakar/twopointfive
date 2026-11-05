// AN OPENING'S LIGHT COMES UP NO FASTER THAN THE BOUNCE CAN ANSWER IT (src/engine.ts OPEN_RISE, openRise; settings.openRise, the address's &openrise=0).
// A place is shown by the light its floor holds, which is the bounce's and comes some frames after the light that makes it; an opening's own light was
// drawn whole from the frame it was lit, so a wall blown open to the day washed a lamplit room to white until the bounce and the field had come round.
//   1  the rule's arithmetic: a light that was out is lit at `first` of its strength, grows by e for every `folds` x the bounce's frames and by no more
//      than `step` of its strength in that many, never past its strength; one that should be dimmer is dimmer at once; a target that grows under it
//      (a door on its swing) is never under `first` of itself
//   2  the engine's own lights, on the pass with a door swung wide in one frame: the door's light follows the rule's numbers frame by frame and says its
//      whole strength beside them; shut, it is out at once; with the setting off, and on a frame the picture starts over, it is whole on the frame it is lit
//   3  THE NUMBERS, against a real card's read-back of the blast they were set by (the pass's store, its outer wall blown at midday, Sam in it; the place
//      field read back every frame, 11-05): what the cache held over the room before the hole was lit, what the hole's light then brought it over eight
//      frames, and the share of its distance the field closes a frame. By them the hole's light is never shown more than a stop and a half over where it
//      settles and is whole inside forty frames; lit whole at once it is shown about five stops over, which is what the card's pictures showed as white
//   bun run tools/qa/openrise-check.ts          exit 0 = all pass · 1 = a line failed
//   OPENRISE_BREAK=1 = the rule lights an opening whole at once: lines 1, 2 and 3 must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const BREAK = process.env.OPENRISE_BREAK === '1';
const { Engine, OPEN_RISE, openRise } = await import(`${ROOT}/src/engine.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { lookConfig } = await import(`${ROOT}/src/render/frame.ts`);
const { HOURS } = await import(`${ROOT}/src/game/hours.ts`);
const { Steps } = await import(`${ROOT}/src/render/steps.ts`);
const { ReachKeep } = await import(`${ROOT}/src/render/reachKeep.ts`);
if (BREAK) { OPEN_RISE.first = 1; }
let fails = 0; const check = (ok: boolean, what: string, detail = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
console.log(`\n=== openrise-check: an opening's light comes up with the bounce${BREAK ? ' (lit whole at once: must fail)' : ''} ===`);
const K = OPEN_RISE;

// 1 ---- the arithmetic
{ const lag = 8, n = K.folds * lag, seq: number[] = []; let v = 0; for (let k = 0; k < 200 && (seq.length === 0 || seq[seq.length - 1] < 1); k++) seq.push(v = openRise(v, 1, lag));
  const grow = seq.slice(1).map((x, i) => x / seq[i]), add = seq.slice(1).map((x, i) => x - seq[i]);
  const byShare = grow.every((r, i) => r <= Math.exp(1 / n) + 1e-9 || seq[i + 1] === 1), byStep = add.every(a => a <= K.step / n + 1e-9), up = add.every(a => a > 0);
  const door: number[] = []; let d = 0, under = 0; for (let k = 1; k <= 60; k++) { const full = Math.min(1, k / 20); d = openRise(d, full, lag); door.push(d); if (d < K.first * full - 1e-12) under++; }   // (a door swung wide over twenty frames)
  const three = (() => { let x = 0, k = 0; while (x < 1 && k < 200) { x = openRise(x, 1, 3); k++; } return k; })();
  check(Math.abs(seq[0] - K.first) < 1e-12 && seq[0] < 0.2 && byShare && byStep && up && seq[seq.length - 1] === 1 && seq.length > 2 * lag && seq.length < 6 * lag && openRise(0.8, 0.3, lag) === 0.3 && openRise(0, 0, lag) === 0 && under === 0 && door[19] < 0.5 && door[59] === 1 && three < seq.length / 2,
    'a light that was out is lit at a small share of its strength and grows by a share a frame, then evenly, never past its strength; one that should be dimmer is dimmer at once; a door\'s light on its swing is never under that share of what is open',
    `over a bounce of ${lag} frames: ${seq.slice(0, 4).map(x => x.toFixed(3)).join(' ')} … whole on frame ${seq.length} · the largest growth a frame x ${Math.max(...grow).toFixed(3)} (e to the 1/${n}: ${Math.exp(1 / n).toFixed(3)}), the largest step ${Math.max(...add).toFixed(4)} (${(K.step / n).toFixed(4)}) · a door wide on frame 20: ${door[19].toFixed(2)} of its light then, whole on frame ${door.findIndex(x => x === 1) + 1} · over a bounce of 3 frames whole on frame ${three}`); }

// 2 ---- the engine's own lights
{ const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'))), seed: 1 })); quiet(() => { for (let i = 0; i < 10; i++) W.step(1 / 60); });
  const g = W.game; g.godMode = true; g.aiEnabled = false;
  const hang = (name: string, a: number) => { const d = g.doors.byName(name); d.latched = a === 0; d.vel = 0; d.angle = a; }, shut = () => { for (const d of g.doors.list) hang(d.def.name, 0); };
  const mk = (rise: boolean | undefined) => Object.assign(Object.create(Engine.prototype), { lights: g.engine.lights, world: g.engine.world, settings: { skyZenith: HOURS.overcast.sky.zenith, skyHorizon: HOURS.overcast.sky.horizon, exposure: HOURS.overcast.exposure, openRise: rise }, viewFocus: [20, 0, 10], openRecs: [], dayLights: [], openLevel: null, openSerial: -1, openTick: 0, openSeen: NaN, openPlanned: NaN, openWait: 0, openLeaves: NaN, openWalls: null, openJob: new Steps(), bounceLag: 8, reach: new ReachKeep(g.engine.world, g.engine.lights) }) as any;
  const was = { on: lookConfig.on, windows: lookConfig.windows, rooms: lookConfig.rooms, eye: lookConfig.eyeAt, gaps: lookConfig.doorGaps };
  try { lookConfig.on = true; lookConfig.windows = 1; lookConfig.rooms = 1; lookConfig.eyeAt = () => [26, 8];
    lookConfig.doorGaps = () => g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
    const run = (rise: boolean | undefined, fresh: boolean) => { shut(); quiet(() => W.step(1 / 60)); const E = mk(rise), lit = () => (E.dayLights as any[]).filter(l => l.enabled);
      quiet(() => { E.lookOpeningLights(undefined, true); E.lookOpeningLights(undefined, true); });   // (the level's first frames: every opening found and masked, and what is open lit whole)
      const before = new Set(lit()), whole0 = lit().every((l: any) => l.intensity === l.peakIntensity);
      hang('back', 1.9); quiet(() => W.step(1 / 60));   // (the kitchen's back door, wide in one frame)
      const seq: number[] = [], peaks: number[] = []; let door: any = null;
      for (let k = 0; k < 60; k++) { quiet(() => E.lookOpeningLights(undefined, fresh && k === 0)); door ??= lit().find((l: any) => !before.has(l)) ?? null; if (door) { seq.push(door.enabled ? door.intensity : 0); peaks.push(door.peakIntensity); } }
      const others = lit().filter((l: any) => l !== door).every((l: any) => l.intensity === l.peakIntensity);
      shut(); quiet(() => W.step(1 / 60)); quiet(() => E.lookOpeningLights()); const out = !!door && !door.enabled;
      return { seq, peaks, door: !!door, whole0, others, out }; };
    const on = run(true, false), off = run(false, false), start = run(true, true);
    const want: number[] = []; { let v = 0; for (let k = 0; k < on.seq.length; k++) want.push(v = openRise(v, on.peaks[k] ?? 0, 8)); }
    const follows = on.door && on.seq.length > 30 && on.seq.every((x, i) => Math.abs(x - want[i]) <= 1e-9 * Math.max(1, want[i])), slow = on.seq[0] < 0.2 * on.peaks[0] && on.seq[on.seq.length - 1] === on.peaks[on.peaks.length - 1];
    check(follows && slow && on.whole0 && on.others && on.out && off.door && off.seq[0] === off.peaks[0] && start.door && start.seq[0] === start.peaks[0],
      'a door swung wide in one frame: its light follows the rule frame by frame and says its whole strength beside it, the lights that were lit stay whole, and shut it is out at once; with the setting off, and on a frame the picture starts over, it is whole on the frame it is lit',
      `lit at ${(on.seq[0] / on.peaks[0]).toFixed(3)} of ${on.peaks[0]?.toFixed(2)}, ${(on.seq[8] / on.peaks[8]).toFixed(3)} eight frames on, whole on frame ${on.seq.findIndex((x, i) => x === on.peaks[i]) + 1} · the setting off: ${(off.seq[0] / off.peaks[0]).toFixed(3)} · the picture starting over: ${(start.seq[0] / start.peaks[0]).toFixed(3)}`);
  } finally { lookConfig.on = was.on; lookConfig.windows = was.windows; lookConfig.rooms = was.rooms; lookConfig.eyeAt = was.eye; lookConfig.doorGaps = was.gaps; shut(); } }

// 3 ---- the numbers, against the card's read-back
{ // what the room's cells held of the cache (light on the floor, the field's measure) on each frame after the blast with the field solved to rest every
  // frame, before the hole's light was lit on the thirteenth (the bounce finding the bare hole by itself); then what the lit hole brought, evenly over
  // the steady bounce's eight frames, and a slower part after; the field with its four sweeps closed 0.14 of its distance (in stops) a frame
  const BARE = [-12.344, -12.093, -11.502, -10.710, -9.428, -8.956, -8.492, -8.124, -7.811, -7.394, -7.186, -7.022, -6.933].map(v => 2 ** v), REST = BARE[12] * 1.02;
  const FIRST = 2 ** -3.83 - 2 ** -6.93, LATER = 2 ** -3.42 - 2 ** -3.83, FIELD = 0.14, RING = 8, LIT = 13, SETTLED = -3.43;
  const run = (share: (was: number) => number) => { let L = Math.log2(BARE[0]), E = 0, peak = -Infinity, whole = -1; const f: number[] = [0];
    for (let k = 1; k <= 160; k++) { f.push(k < LIT ? 0 : share(f[k - 1])); let D = 0; for (let j = Math.max(0, k - RING); j < k; j++) D += f[j] / RING; E += (D - E) / 10;
      L += FIELD * (Math.log2((k <= 12 ? BARE[k] : REST) + FIRST * D + LATER * E) - L);
      if (f[k] > 0) peak = Math.max(peak, Math.log2(f[k]) + SETTLED - L); if (whole < 0 && f[k] >= 1) whole = k - LIT + 1; }
    return { peak, whole }; };
  const rule = run(was => openRise(was, 1, RING)), once = run(() => 1);
  check(rule.peak < 1.5 && rule.whole > 0 && rule.whole <= 40 && once.peak > 4.5 && once.peak < 5.5,
    'by a real card\'s read-back of the blast the numbers were set by, the hole\'s light is never shown more than a stop and a half over where it settles and is whole inside forty frames; lit whole at once it is five stops over',
    `the rule: ${rule.peak.toFixed(2)} stops over at the most, whole ${rule.whole} frames after it is lit · whole at once: ${once.peak.toFixed(2)} stops over (the card's own read-back: 5.0)`); }

console.log(fails ? `=== openrise-check: ${fails} FAILED ===` : '=== openrise-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
