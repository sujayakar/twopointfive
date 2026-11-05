// THE LIGHTS' STATE, HELD — rust/sim/src/lights.rs on made-up fixtures and on every level's, each column of the part folded every tick, and each lamp held to its row. The arithmetic
// of the TypeScript it replaced was run beside it on every call of the way in to the bit (this tool's `Twin`, in the commit that moved them: git log -- rust/sim/src/lights.rs), and went with
// the deletion; what that run proved is kept as a word a seed.
//   bun run tools/qa/lights-ident.ts [--levels office.json,lights-out.json] [--ticks 2400] [--seed 1]
// Two parts:
//   made-up  fixtures and machines of the tool's own, every event kind on a clock and made-up men in front of the machines, no Game's tick: what it folds to is a word
//            per seed kept in tools/qa/fixtures/lights-ident.json (`--write` writes it, of 2400 ticks a seed: a change of the rule is said by a new word), which no level's edit moves.
//   levels   a whole game on each level, men and all, every event from the game's own functions on a clock (a round's and a blast's break, the OCP, the mains tripped
//            and thrown and shot, groups, the panel's toggle and repair, a wall switch both ways, the show's day, the hour, a save and a load, a restart, Sam walked
//            through a camera's picture and a beam). After every tick each lamp with a row: its record in the part is the record the renderer's list holds, to the bit.
//            A row left a tick stale is red here before any question reads it.
// LIGHTS_BREAK=1 is the red path: the made-up run's mains tripped a hundredth of a second late once, and a row written a millimetre out once. The run must FAIL.
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const H: any = await import(`${ROOT}/src/sim/lightshome.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const RB: any = await import(`${ROOT}/src/game/rollback.ts`), GD: any = await import(`${ROOT}/src/game/guards.ts`);
const LG: any = await import(`${ROOT}/src/game/lighting.ts`), LS: any = await import(`${ROOT}/src/game/lightswitch.ts`), SV: any = await import(`${ROOT}/src/game/savestate.ts`), HR: any = await import(`${ROOT}/src/game/hours.ts`);
const LIGHT_IN = H.LIGHT_IN, LIGHT_COLS: readonly string[] = H.LIGHT_COLS, WORDS: ReadonlySet<string> = H.WORD_COLS, CAPS = H.LIGHT_CAPS;
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; }, has = (n: string) => argv.includes(n);
const TICKS = Number(flag('--ticks') ?? 2400), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.LIGHTS_BREAK === '1', DT = 1 / 60;
const LEVELS = (flag('--levels') ?? 'office.json,lights-out.json,vault.json,tut-sky.json,bazaar.json,pass.json,depot.json').split(',');
const FIXTURE = resolve(ROOT, 'tools/qa/fixtures/lights-ident.json');
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const fail = (why: string) => { if (bad++ < 12) log(`- FAIL — ${why}`); };
const rng = (seed: number) => { let s = seed >>> 0; return () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; };

// ---------------------------------------------------------------- how long each column is
const WIDTH: Record<string, number> = {};
{ const groups: [string, number][] = [['blind_until bits factor stagger base hand', CAPS.targets], ['x y z range radius kind dx dy dz outer inner r g b intensity sx sy sz s_radius s_intensity row_base row_bits row_target', CAPS.rows], ['beacon_phase beacon_row beacon_target', CAPS.beacons],
    ['cam_yaw cam_phase cam_seen cam_reported cam_lx cam_ly cam_lz cam_rest cam_a cam_b cam_period cam_pitch cam_px cam_py cam_pz cam_state cam_bits cam_target cam_row', CAPS.cameras], ['las_last las_cut las_sense las_trips las_bits las_target', CAPS.lasers]];
  for (const k of LIGHT_COLS) WIDTH[k] = 1;
  for (const [names, n] of groups) for (const k of names.split(' ')) WIDTH[k] = n; }
const ROW_COLS = 'x y z range radius kind dx dy dz outer inner r g b intensity sx sy sz s_radius s_intensity row_bits'.split(' ');

/** one column of a home's part, as numbers */
function read(home: any, k: string): number[] { const st = home.block, at = home.col(k), src = WORDS.has(k) ? st.w : st.d, out = new Array(WIDTH[k]); for (let i = 0; i < WIDTH[k]; i++) out[i] = src[at + i]; return out; }

// ---------------------------------------------------------------- THE WORD: every column in use folded
function fold(home: any, into: number): number {
  let h = into >>> 0; const st = home.block, B = new Float64Array(1), W = new Uint32Array(B.buffer);
  const mix = (x: number) => { h = Math.imul(h ^ x, 0x01000193) >>> 0; };
  const T = (k: string) => read(home, k)[0];
  for (const k of LIGHT_COLS) {
    const n = k.startsWith('cam_') ? T('cameras') : k.startsWith('las_') ? T('lasers') : k.startsWith('beacon_') ? T('beacons') : WIDTH[k] === 1 ? 1 : ROW_COLS.includes(k) || k === 'row_base' || k === 'row_target' ? T('rows') : T('n');
    const at = home.col(k); if (WORDS.has(k)) for (let i = 0; i < n; i++) mix(st.w[at + i]); else for (let i = 0; i < n; i++) { B[0] = st.d[at + i]; mix(W[0]); mix(W[1]); }
  }
  return h;
}

// ---------------------------------------------------------------- PART ONE: made-up fixtures, every event kind on a clock
function lampDef(i: number, kind: number, enabled: boolean, intensity: number): any { const k = i + 1; return { name: `lamp${i}`, group: 'made', kind, pos: [k * 0.5, 2.5, -k * 0.25], dir: [0, -1, 0], color: [0.9, 0.8, 0.7], intensity, range: 6 + i % 5, innerDeg: 20, outerDeg: 40, radius: 0.1, volumetric: 0, owner: 0, enabled, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: intensity }; }
function madeUp(seed: number, ticks: number): number {
  const rnd = rng(seed), st = new ST.SimState(), home = new H.LightHome(st), list: any[] = [], targets: any[] = [];
  const NF = 40, fix = (i: number, extra: any = {}) => ({ name: `f${i}`, group: i % 3 ? 'a' : 'b', kind: i % 4 === 0 ? 'analytic' : 'emissive', pos: [i, 2, 0], boxes: [], baseEmissive: [1, 1, 1], baseIntensity: 2 + i * 0.25, on: i % 5 !== 0, broken: false, disabledUntil: -1, fluorescentFlicker: i % 7 === 0, factor: i % 5 !== 0 ? 1 : 0, mains: i % 3 !== 2, interactive: true, stagger: i * 0.13, ...extra });
  for (let i = 0; i < NF; i++) { const def = fix(i); if (def.kind === 'analytic') { def.light = lampDef(i, 0, def.on, def.baseIntensity); list.push(def.light); } const t = home.add(def); targets.push(t); if (i % 4 === 1) { const al = lampDef(100 + i, 3, def.on, 1.5); list.push(al); t.areaLights = [{ light: al, base: 1.5 }]; } home.in(LIGHT_IN.authored, t, i % 2); }
  const beacons: any[] = []; for (let i = 0; i < 3; i++) { const l = lampDef(200 + i, 1, false, 55); list.push(l); beacons.push({ light: l, phase: i * 1.7, target: targets[i * 5] }); }
  const cams: any[] = [], lasers: number[] = [];
  for (let i = 0; i < 3; i++) { const t = home.add(fix(NF + i, { kind: 'device', mains: i !== 1, stagger: 1.4 + 0.3 * i })); targets.push(t); const cone = lampDef(300 + i, 1, false, 1.1); list.push(cone); const slot = home.camera(t, 0.5 * i, i === 2 ? undefined : [-0.8, 0.9], 6 + i, -0.4, [i, 3, 2]); cams.push({ light: cone, target: t, slot }); }
  for (let i = 0; i < 2; i++) { const t = home.add(fix(NF + 3 + i, { kind: 'device', stagger: 1.1 + 0.3 * i })); targets.push(t); lasers.push(home.laser(t, 5 + i)); }
  home.stand(list, targets, beacons, cams);
  let time = 3, word = 0x811c9dc5, tick = 0;
  const someF = () => targets[Math.floor(rnd() * targets.length)];
  for (tick = 0; tick < ticks; tick++) {
    time += DT;
    if (tick % 7 === 0) {
      const t = someF(), k = Math.floor(rnd() * 15);
      switch (k) {
        case 0: home.in(LIGHT_IN.on, t, rnd() < 0.5 ? 1 : 0); break;
        case 1: home.in(LIGHT_IN.break, t); break;
        case 2: home.in(LIGHT_IN.mend, t); break;
        case 3: home.in(LIGHT_IN.toggle, t); break;
        case 4: home.in(LIGHT_IN.blind, t, rnd() < 0.2 ? -1 : time + rnd() * 3); break;
        case 5: home.in(LIGHT_IN.hand, t, rnd() < 0.5 ? NaN : time); break;
        case 6: home.in(LIGHT_IN.base, t, rnd() * 4); break;
        case 7: home.in(LIGHT_IN.put, t, rnd() < 0.5 ? 1 : 0, rnd() < 0.3 ? 1 : 0, rnd() < 0.5 ? -1 : time + 1, rnd() < 0.5 ? NaN : time - 1); break;
        case 8: home.in(LIGHT_IN.trip, null, BREAK && tick === 7 * 40 ? time + 0.01 : time, rnd() < 0.15 ? Infinity : rnd() * 7); break;
        case 9: home.in(LIGHT_IN.back, null, time); break;
        case 10: home.in(LIGHT_IN.mains, null, rnd() < 0.5 ? 0 : 1, time + rnd() * 5, time - rnd() * 4, time - rnd() * 2); break;
        case 11: home.in(LIGHT_IN.fix, null, rnd()); break;
        case 12: if (rnd() < 0.2) home.in(LIGHT_IN.fresh, null); break;
        case 13: home.in(rnd() < 0.5 ? LIGHT_IN.stagger : LIGHT_IN.authored, t, rnd() < 0.5 ? rnd() * 2 : 0); break;
        case 14: { const a = targets.filter(() => rnd() < 0.2); home.flip(a, rnd() < 0.5, time, rnd() < 0.7); break; }
      }
      if (rnd() < 0.1) { const c = cams[Math.floor(rnd() * cams.length)]; const M = new H.CameraMachine(c.slot, st); M.put(rnd(), rnd(), rnd() * 2, H.CAMERA_STATES[Math.floor(rnd() * 3)], time - rnd() * 30, rnd() < 0.5 ? null : [rnd(), 0, rnd()], rnd() < 0.3); }
      if (rnd() < 0.1) { const L = new H.LaserMachine(lasers[Math.floor(rnd() * lasers.length)], st); L.put(time - rnd() * 8, Math.floor(rnd() * 4), rnd() < 0.3, rnd() < 0.3); }
      if (rnd() < 0.3) { const l = list[Math.floor(rnd() * list.length)]; l.intensity = rnd() * 3; l.pos = [rnd(), rnd(), rnd()]; l.enabled = rnd() < 0.5; if (rnd() < 0.5) l.steady = { intensity: rnd(), pos: [rnd(), rnd(), rnd()], radius: rnd() }; else delete l.steady; home.feed(l); }
    }
    const dt = rnd() < 0.02 ? 0 : DT, near = rnd() < 0.15;
    home.step(time, dt, near, 1 + rnd() * 2, 0.5 + rnd());
    for (const c of cams) { const live = rnd() < 0.9, m = home.cameraPan(c.slot, c.light, dt, live, rnd() < 0.8); if (live && (m & H.CAM.ok)) { const vis = rnd() < 0.6; home.cameraSee(c.slot, time, dt, vis, vis ? rnd() * 3 * (rnd() < 0.2 ? 0 : 1) : 0, [rnd(), 0, rnd()], 0.8, 2); } }
    for (const l of lasers) { const m = home.laserOpen(l); if (m & H.LASER.ok) home.laserClose(l, time, rnd() < 0.9, rnd() * 5, rnd() * 5, rnd() < 0.3); }
    home.paint();
    word = fold(home, word);
  }
  return word;
}

// ---------------------------------------------------------------- PART TWO AND THREE: whole games
/** each lamp with a row: its record in the part is the record the renderer's list holds */
function rowsTrue(g: any, where: string) {
  const home = g.lightHome, st = home.block, d = st.d, w = st.w;
  for (const l of g.engine.lights.lights) {
    const r = l.row; if (r === undefined) continue; const s = l.steady;
    const say = (name: string, a: number, b: number) => { if (!Object.is(a, b)) fail(`${where}: the row of ${l.name}: ${name} is ${a} and the lamp's ${b}`); };
    const c = (k: string) => d[home.col(k) + r];
    say('on', (w[home.col('row_bits') + r] & 1) !== 0 ? 1 : 0, l.enabled ? 1 : 0);
    if (!l.enabled) continue;   // (no question reads the rest of a row that is out: lightshome.ts feedBurning)
    say('x', c('x'), l.pos[0]); say('y', c('y'), l.pos[1]); say('z', c('z'), l.pos[2]); say('range', c('range'), l.range); say('radius', c('radius'), l.radius); say('kind', c('kind'), l.kind);
    say('dx', c('dx'), l.dir[0]); say('dy', c('dy'), l.dir[1]); say('dz', c('dz'), l.dir[2]); say('outer', c('outer'), l.outerDeg); say('inner', c('inner'), l.innerDeg);
    say('r', c('r'), l.color[0]); say('g', c('g'), l.color[1]); say('b', c('b'), l.color[2]); say('intensity', c('intensity'), l.intensity);
    say('steady bit', (w[home.col('row_bits') + r] & 32) !== 0 ? 1 : 0, s ? 1 : 0);
    if (s) { say('steady x', c('sx'), s.pos[0]); say('steady y', c('sy'), s.pos[1]); say('steady z', c('sz'), s.pos[2]); say('steady radius', c('s_radius'), s.radius); say('steady intensity', c('s_intensity'), s.intensity); }
  }
}
async function level(file: string): Promise<void> {
  hush(true); let W: any;
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()), lv = file === 'office.json' ? null : loadLevel(data); W = await standUp({ uploadEvery: 1000, seed: SEED, ...(lv ? { level: lv } : {}) }); } finally { hush(false); }
  const g = W.game, rnd = rng(SEED * 7 + file.length); let tick = 0;
  const someT = (f: (t: any) => boolean = () => true) => { const a = g.targets.filter(f); return a.length ? a[Math.floor(rnd() * a.length)] : null; };
  const events: (() => void)[] = [
    () => { const t = someT(x => x.kind !== 'breaker'); if (t) g.ocp(t); g.player.ocp.charge = 1; },
    () => { const t = someT(x => x.kind === 'breaker'); if (t) g.ocp(t); g.player.ocp.charge = 1; },
    () => { const t = someT(x => x.boxes.length > 0 && x.kind !== 'breaker'); if (t && g.destr?.breakFixtureBox) g.destr.breakFixtureBox(t.boxes[0]); },
    () => { g.setBlackout(4 + rnd() * 6, 'debug'); },
    () => { g.setBlackout(86400, 'hand'); },
    () => { g.endBlackout(); },
    () => { const t = someT(); if (t) LG.setGroup(g, t.group, rnd() < 0.5); },
    () => { const t = someT(); if (t) LG.toggleTarget(g, t); },
    () => { LG.repairLights(g); },
    () => { if (LS.LIGHTSWITCH.on) { const a = LS.wallSwitches(g); if (a.length) { const sw = a[Math.floor(rnd() * a.length)]; LS.flipSwitch(g, sw, !LS.switchLit(sw), g.player.char); } } },
    () => { if (g.level.daylight && !LG.isDaylight(g)) LG.setDaylight(g, true); else if (LG.isDaylight(g)) LG.setDaylight(g, false); },
    () => { if (!LG.isDaylight(g)) HR.applyHour(g, HR.hourOf(g) === 'night' ? 'midday' : 'night'); },
    () => { if (LG.isDaylight(g)) LG.setDaylight(g, false); const s = SV.saveState(g); W.step(DT); SV.loadState(g, JSON.parse(JSON.stringify(s))); },
    () => { if (LG.isDaylight(g)) LG.setDaylight(g, false); g.restartEncounter(); },
    () => { g.setBlackout(Infinity, 'shot'); },
    () => { const c = g.devices?.cameras[Math.floor(rnd() * g.devices.cameras.length)]; if (c) { const look = [Math.sin(c.yaw), Math.cos(c.yaw)], p = g.col.nav.nearestFreePoint(c.def.pos[0] + look[0] * 3, c.def.pos[2] + look[1] * 3); if (p) g.teleportPlayer([p[0], 0, p[2]]); } },
    () => { const l = g.devices?.lasers[Math.floor(rnd() * g.devices.lasers.length)]; if (l) { const a = l.def.from, b = l.def.to, p = g.col.nav.nearestFreePoint((a[0] + b[0]) / 2, (a[2] + b[2]) / 2); if (p) g.teleportPlayer([p[0], 0, p[2]]); } },
    () => { const t = someT(x => x.kind === 'device'); if (t) g.lightHome.stage(t, null, true); },
  ];
  g.godMode = true;
  hush(true);
  try {
    for (tick = 0; tick < TICKS; tick++) {
      if (tick % 45 === 44) events[Math.floor(rnd() * events.length)]();
      W.step(DT);
      if (BREAK && tick === 300 && g.lightHome.block.d.length) { const r = 0, at = g.lightHome.col('x'); g.lightHome.block.d[at + r] += 0.001; }
      rowsTrue(g, `${file} tick ${tick}`);
    }
  } finally { hush(false); }
}

// ---------------------------------------------------------------- what a row-less lamp and a put-back must do
/** A LAMP THE MODULE WRITES MUST HAVE A ROW: a beacon's spot or a camera's cone with an owner is an error by name as the rows are stood */
function standRefuses() {
  for (const what of ['beacon', 'cone']) {
    const st = new ST.SimState(), home = new H.LightHome(st), t = home.add({ name: 'f', group: 'g', kind: 'emissive', pos: [0, 2, 0], boxes: [], baseEmissive: [1, 1, 1], baseIntensity: 0, on: true, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains: true, interactive: true, stagger: 0 });
    const l = lampDef(0, 1, false, 5); l.owner = 7; let said = '';
    try { if (what === 'beacon') home.stand([l], [t], [{ light: l, phase: 0, target: t }], []); else home.stand([l], [t], [], [{ light: l, target: t, slot: home.camera(t, 0, undefined, 9, 0, [0, 3, 0]) }]); } catch (e) { said = String((e as Error).message); }
    if (!said.includes(l.name)) fail(`a ${what} with an owner was stood as a row and no error named it (${said || 'no error'})`);
  }
}
/** A FIXTURE'S LAMP WITH NO ROW (hung after the level was stood, or with an owner) is still switched, by updateTargets from the factor: every area light has a row or is written there */
async function rowLess() {
  hush(true); let W: any; try { W = await standUp({ uploadEvery: 1000, seed: SEED }); } finally { hush(false); }
  const g = W.game; hush(true);
  try {
    const t = g.targets.find((x: any) => x.areaLights?.length && x.mains && x.on), own = g.targets.find((x: any) => x.kind === 'analytic' && x.light && x.mains && x.on); if (!t || !own) { fail('rowless: no lit area light and lamp on the mains in the office'); return; }
    const al = t.areaLights[0].light, l = own.light; al.row = undefined; l.row = undefined;
    for (let i = 0; i < 10; i++) W.step(DT);
    const up = al.enabled && al.intensity === t.areaLights[0].base * t.factor * g.engine.settings.emissiveScale && l.enabled && l.intensity === own.baseIntensity * own.factor;
    g.setBlackout(30, 'debug'); W.step(DT); const down = !al.enabled && al.intensity === 0 && !l.enabled && l.intensity === 0;
    if (!up || !down) fail(`a fixture's lamp with no row is not switched from the factor: lit ${up}, out in a blackout ${down}`);
  } finally { hush(false); }
}
/** A PUT-BACK ACROSS A BLACKOUT: the light question asked before the next updateTargets answers as the restored timeline did, every row is its lamp, in the same game and put into a fresh one (rollback.ts rootsOf: the light set is in a snapshot) */
async function putBack(file: string) {
  const stand = async () => { hush(true); try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()), lv = file === 'office.json' ? null : loadLevel(data); return await standUp({ uploadEvery: 1000, seed: SEED, ...(lv ? { level: lv } : {}) }); } finally { hush(false); } };
  const W: any = await stand(), g = W.game; hush(true);
  try {
    for (let i = 0; i < 60; i++) W.step(DT);
    const c = g.player.char.pos, pts: [number, number, number][] = [[c[0], 1.2, c[2]], ...g.targets.filter((t: any) => t.light || t.areaLights).slice(0, 6).map((t: any) => [t.pos[0], Math.max(0.6, t.pos[1] - 1), t.pos[2]])];
    const ask = (G: any) => pts.map(p => GD.lightEstimateAt(G, p)), snap = RB.takeSnapshot(g), was = ask(g), word = fold(g.lightHome, 1);
    g.setBlackout(30, 'debug'); LG.setGroup(g, g.targets[0].group, !g.targets[0].on); for (let i = 0; i < 40; i++) W.step(DT);
    const dark = ask(g); RB.restoreSnapshot(snap);
    const now = ask(g), same = (a: number[], b: number[]) => a.every((x, i) => Object.is(x, b[i]));
    if (same(was, dark) && was.some(x => x > 0)) fail(`${file}: the blackout changed no answer: nothing was proved`);
    if (!same(was, now)) fail(`${file}: put back across a blackout, a light question reads ${now.join(' ')} where the timeline read ${was.join(' ')}`);
    if (fold(g.lightHome, 1) !== word) fail(`${file}: put back, the lights' part is not what it was`);
    rowsTrue(g, `${file} put back`);
    const W2: any = await stand(), g2 = W2.game; for (let i = 0; i < 3; i++) W2.step(DT); RB.restoreSnapshot(snap, g2);
    const there = ask(g2); if (!same(was, there)) fail(`${file}: put into a fresh game, a light question reads ${there.join(' ')} where the timeline read ${was.join(' ')}`);
    if (fold(g2.lightHome, 1) !== word) fail(`${file}: put into a fresh game, the lights' part is not what it was`);
    rowsTrue(g2, `${file} put into a fresh game`);
  } finally { hush(false); }
}

// ---------------------------------------------------------------- the run
{ const no = SIM.simWhyNot(); if (no) { log(`=== lights-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const KEEP: Record<string, number> = await Bun.file(FIXTURE).json().catch(() => ({})), KEPT = TICKS === 2400;   // (the words are of 2400 ticks a seed)
if (!Object.keys(KEEP).length && !has('--write')) fail(`no kept words at ${FIXTURE}: --write makes them`);
const words: Record<string, number> = {};
for (const s of [1, 2, 3]) { const word = madeUp(s, TICKS); words[`seed${s}`] = word;
  if (KEPT && KEEP[`seed${s}`] !== undefined && KEEP[`seed${s}`] !== word && !has('--write')) fail(`made-up seed ${s}: the word is ${word.toString(16)} and the kept one ${KEEP[`seed${s}`].toString(16)}`); }
log(`made-up: 3 seeds, ${TICKS} ticks, words ${Object.values(words).map(w => w.toString(16)).join(' ')}`);
for (const f of LEVELS) { await level(f); log(`${f.padEnd(22)} every row its lamp`); }
standRefuses(); await rowLess(); for (const f of ['office.json', 'pass.json']) await putBack(f);
log('a lamp the module writes must have a row, a lamp with no row is switched from the factor, a put-back across a blackout reads the restored timeline');
if (has('--write')) { await Bun.write(FIXTURE, JSON.stringify(words, null, 1) + '\n'); log('words written'); }
log(bad ? `=== lights-ident: FAIL, ${bad} difference(s)${BREAK ? ' (the red path: as it should)' : ''} ===` : `=== lights-ident: PASS — the kept words stand and every row is its lamp ===`);
process.exit(bad ? 1 : 0);
