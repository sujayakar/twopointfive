// THE KIT PICTURE'S STAGE, TWO WAYS: src/host/kitView.ts's staging, stood headless on the TypeScript game, against rust/game_rules' kitstage (the bin kitstage). `bun run tools/blitz/kitstage-diff.ts`
//   exit 0 nothing differs · 1 a difference · 2 a side could not run. KITSTAGE_DIFF_BREAK=1 moves one number on the TypeScript's side, and the compare must fail.
// Each case: each man's place and yaws, each light (sorted by name), the moon, the sky, the boxes under the ground, the props taken away, the last static, the pin at 1.6 with no frame and with one.
// WHAT THIS CANNOT SEE: the page's staging after its first picture (the loop's own writes are the later pieces'), and anything drawn.
// (the page's kitView.ts stands here with a small window: it reads location, posts to its parent, listens; `&stillkit` so that the staging alone runs. Each TypeScript case is a process: the staging keeps its numbers in module tables.)
process.env.TPS_GAME = 'none';
import { readFileSync, mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir, homedir } from 'node:os';
import { join } from 'node:path';
/* eslint-disable @typescript-eslint/no-explicit-any */
const ROOT = join(import.meta.dir, '../..');
const TOL = 1e-6;

/** no level writes a way's `add` today: records' first way with a crate and a lamp, written for this compare */
const ADDED = 'kp-added.json';
const pathOf = (level: string) => (level === ADDED ? join(process.env.KITSTAGE_TMP ?? '', level) : `${ROOT}/levels/${level}`);
/** [level file, the link's words, the size a card asks for afterwards] */
const CASES: [string, string, number?][] = [
  ['records.json', 'roomclear&squad4', 2], ['records.json', 'roomclear&squad4&way=2'], ['warehouse.json', 'roomclear&squad=2'], ['mousehole.json', 'roomclear&squad4'], ['mousehole.json', 'roomclear&squad4&way=2'],
  ['compound.json', 'roomclear&squad=3'], ['compound.json', 'roomclear&squad4&way=2'], ['consulate.json', 'rescue&squad=2'], ['bazaar.json', 'quiet&squad=1'], ['vault.json', 'quiet'], ['office.json', ''], ['registry.json', 'rescue&squad=1'],
  ['office.json', 'nightbreach&squad=4'], ['records.json', 'roomclear&squad=1&hour=midday'], ['lights-out.json', 'quiet'], [ADDED, 'roomclear&squad=2'], [ADDED, 'roomclear&squad=2&hour=midday'],
];
const linkOf = (level: string, words: string) => `?levelFile=${level}&kitview${words ? '&' + words : ''}`;

async function tsSide(level: string, words: string, size: number | undefined) {
  const { EXACT } = await import('../../src/math/exact.ts'); EXACT.on = true;
  const link = linkOf(level, words), q = new URLSearchParams(link);
  const { kitViewSetLevel, kitViewAddLevel, kitViewWayOfLink, kitViewSetQuery, kitViewSetDress } = await import('../../src/host/kitViewSet.ts');
  const { standUp } = await import('../qa/headless.ts');
  const { dressAll } = await import('../../src/game/dress.ts');
  const { parseLevelData } = await import('../../src/scene/levelData.ts');
  const { loadLevel, levelFileOf } = await import('../../src/scene/levelLoad.ts');
  const { setSuitAs } = await import('../../src/scene/look.ts');
  const { KITANIM } = await import('../../src/game/kitanim.ts');
  const { startAtAddress } = await import('../../src/host/standup.ts');
  dressAll(false);
  const data0: any = parseLevelData(readFileSync(pathOf(level), 'utf8'), {}), way = kitViewWayOfLink(q, data0), data1 = kitViewSetLevel(data0, way), on = data1 !== data0, data = on ? data1 : kitViewAddLevel(data0, way);
  // (a small window, before the staging's own module is read: it reads the link at its load)
  const heard: ((e: any) => void)[] = [], parent = { postMessage() { /* nobody holds this frame */ } };
  const loc = { search: `${link}&stillkit`, protocol: 'http:', origin: 'http://here' };
  const win: any = { parent, location: loc, addEventListener(k: string, f: any) { if (k === 'message') heard.push(f); }, removeEventListener() { /* */ } };
  const lvl = loadLevel(data as any);
  const dress = on ? kitViewSetDress(q) : null;
  if (dress) { setSuitAs(dress.suit as any); (KITANIM as any).sling = dress.sling; }
  const W: any = await standUp({ level: lvl, seed: 3, address: link, dress: false } as any), g: any = W.game;
  (globalThis as any).window = win; (globalThis as any).location = loc;
  const { kitViewStart, kitViewQuietOf } = await import('../../src/host/kitView.ts');
  if (!dress && kitViewQuietOf(q)) (KITANIM as any).sling = 2;
  startAtAddress(g, on ? kitViewSetQuery(q) : q);
  const { teamOf } = await import('../../src/game/scenarioRun.ts'), { sideOf } = await import('../../src/game/orders.ts'), { hourOf, HOURS } = await import('../../src/game/hours.ts'), { makeBox, BoxFlag } = await import('../../src/scene/boxes.ts');
  const { KIT, kitRows } = await import('../../src/game/kit.ts'), { setTurn } = await import('../../src/game/character.ts');
  const engine = W.engine, view: any = { clientWidth: 1600, clientHeight: 1000, parentElement: null, getBoundingClientRect: () => ({ left: 0, top: 0, width: 1600, height: 1000 }) };
  // (THE TYPESCRIPT TICK IS GONE, so the staging's one zero-length step is no step here: what that step writes is compared by what decides it, the fixtures' switches, and not by the lights it wrote)
  g.update = () => { /* */ }; engine.resetTaa = () => { /* */ };
  const V: any = kitViewStart({ game: g, engine, cam: W.cam, input: W.input, view, loop: { render() { /* */ } } as any, audio: { enabled: true, setVolumes() { /* */ } }, team: () => teamOf(g), doorSide: sideOf,
    file: () => levelFileOf.get(g.level) ?? null as any, lampsOut: () => { const h = hourOf(g); return h !== 'night' && !(HOURS as any)[h].lamps; }, glowBox: (c: any, h: any, yaw: number, glow: any) => makeBox({ c, h, yaw, albedo: [0.04, 0.04, 0.04], emissive: glow, flags: BoxFlag.Emissive | BoxFlag.AreaLit }),
    kitCards() { /* the cards are as issued on both sides */ }, kit: () => KIT.members, tiles: (i: number) => kitRows(i).map((r: any) => r.field), smoke() { /* */ }, gun: () => '', tints: () => ({ suit: null, trim: null }), samCard() { /* */ }, turn: (s: string) => { setTurn(s); }, fire() { /* */ }, panes: () => [], mendGlass() { /* */ } } as any);
  const say = (d: any) => { for (const f of heard) f({ origin: loc.origin, source: parent, data: d }); };
  const num = (v: any) => [v[0], v[1], v[2]].map(Number);
  const staged = () => { const T: any[] = (V as any).men, order = T.slice(1).sort((a, b) => a.slot - b.slot).map(m => (teamOf(g) as any[]).find(x => x.callsign === m.name));
    const man = (name: string, c: any) => ({ name, x: c.pos[0], z: c.pos[2], yaw: c.bodyYaw, aim: c.aimYaw });
    const under: number[] = []; (g.level.boxes as any[]).forEach((b, i) => { if (b.c[1] < -5) under.push(i); });
    const props: number[] = []; (g.props.all as any[]).forEach((p, i) => { if (!g.props.props.includes(p)) props.push(i); });
    const moon = engine.lights.lights.find((l: any) => l.name === 'moon' && l.kind === 2), st = engine.world.statics, last = st[st.length - 1], S = engine.settings;
    return { men: [man('Sam', g.player.char), ...order.map((x: any) => man(x.callsign, x.char))], lights: engine.lights.lights.map((l: any) => ({ name: l.name, group: l.group, fixture: g.targets.some((t: any) => t.light === l), enabled: !!l.enabled, intensity: l.intensity, color: num(l.color), pos: num(l.pos) })),
      fixtures: g.targets.map((t: any) => ({ name: t.name, on: !!t.on, broken: !!t.broken, base: t.baseIntensity, emissive: num(t.baseEmissive) })),
      moon: moon ? { intensity: moon.intensity, color: num(moon.color) } : null, zenith: num(S.skyZenith), horizon: num(S.skyHorizon), under, props, statics: st.length, last: last ? { c: num(last.c), h: num(last.h), yaw: last.yaw, emissive: num(last.emissive), flags: last.flags } : null }; };
  const pinOf = () => { const p = V.pin; return p ? { look: num(p.look), yaw: p.yaw, pitch: p.pitch, dist: p.dist } : null; };
  if (V.state === 'none') return { none: true };
  const o: any = { none: false, staged: staged() };
  if (process.env.KITSTAGE_DIFF_BREAK === '1') o.staged.men[0].x += 1e-3;
  o.pin0 = pinOf();
  say({ kv: 'frame', cx: -0.4, cy: 0, h: 0.6, w: 0.5, man: 0.19, door: -0.2, left: -0.9, right: 0.1, keep: 0.85 }); o.pin1 = pinOf();
  if (size !== undefined) { say({ kv: 'kit', squad: size }); o.restaged = V.state === 'none' ? null : staged(); }
  return o;
}

if (process.argv[2] === '--ts') {
  const r = await tsSide(process.argv[3], process.argv[4] ?? '', process.argv[5] ? Number(process.argv[5]) : undefined);
  process.stdout.write(`\n@@${JSON.stringify(r)}\n`);
  process.exit(0);
}

const tmp = mkdtempSync(join(tmpdir(), 'kitstage-')), reqFile = join(tmp, 'requests.jsonl');
process.env.KITSTAGE_TMP = tmp;
{ const d = JSON.parse(readFileSync(`${ROOT}/levels/records.json`, 'utf8')), y = d.scenarios.kitView.ways[0];
  d.scenarios.kitView.ways = [{ ...y, add: { boxes: [{ id: 'kp_crate', c: [35, 0.5, 25], h: [0.4, 0.5, 0.4], albedo: [1, 1, 1] }], lamps: [{ id: 'kp_lamp', kind: 'point', pos: [35, 2, 25], dir: [0, -1, 0], color: [1, 1, 1], intensity: 3, range: 5, innerDeg: 30, outerDeg: 40, group: 'kp_group', enabled: true, volumetric: 0 }] } }];
  writeFileSync(pathOf(ADDED), JSON.stringify(d)); }
writeFileSync(reqFile, CASES.map(([l, w, size]) => JSON.stringify({ level: l === ADDED ? pathOf(l) : l, link: linkOf(l, w), seed: 3, ...(size !== undefined ? { size } : {}) })).join('\n') + '\n');
const env = { ...process.env, PATH: `${join(homedir(), '.cargo/bin')}:${process.env.PATH ?? ''}`, RUSTFLAGS: '-Awarnings' };
const bin = Bun.spawnSync(['cargo', 'run', '-q', '--release', '--bin', 'kitstage', '--', '--ask', reqFile], { cwd: `${ROOT}/rust/game_start`, env, stderr: 'pipe', stdout: 'pipe' });
if (bin.exitCode !== 0) { console.log(`kitstage-diff: the bin failed\n${bin.stderr.toString().slice(-3000)}`); process.exit(2); }
const rust = bin.stdout.toString().trim().split('\n').map(l => JSON.parse(l));

const sorted = (L: any[]) => [...L].sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : a.pos[0] - b.pos[0] || a.pos[2] - b.pos[2]);
let worst = 0;
function cmp(t: any, r: any, at: string, out: string[]) {
  if (out.length > 12) return;
  if (typeof t === 'number' && typeof r === 'number') { const d = Math.abs(t - r); worst = Math.max(worst, d); if (!(d <= TOL)) out.push(`${at}: ts ${t} rust ${r}`); return; }
  if (t === null || r === null || t === undefined || r === undefined || typeof t !== 'object' || typeof r !== 'object') { if ((t ?? null) !== (r ?? null)) out.push(`${at}: ts ${JSON.stringify(t)} rust ${JSON.stringify(r)}`); return; }
  if (Array.isArray(t)) { if (!Array.isArray(r) || t.length !== r.length) { out.push(`${at}: ts ${t.length} long, rust ${r.length}: ${JSON.stringify(t).slice(0, 300)} / ${JSON.stringify(r).slice(0, 300)}`); return; } t.forEach((x, i) => cmp(x, r[i], `${at}[${x?.name ?? i}]`, out)); return; }
  for (const k of new Set([...Object.keys(t), ...Object.keys(r)])) cmp(t[k], r[k], `${at}.${k}`, out);
}
// (what only the zero-length step writes: a fixture's light and its area lights, whose switch and base are compared in `fixtures`, a camera's cone, and where a weapon light hangs on its gun and which of Sam's two burns. Sam's own mark is written after the step, so its strength stays)
const lightsIn = (o: any) => { for (const s of [o?.staged, o?.restaged]) if (s?.lights) s.lights = sorted(s.lights).map(({ fixture, group, ...l }) => { const c: any = { ...l }; const step = fixture || /_area\d+$/.test(l.name) || /_cone$/.test(l.name); if (step) { delete c.enabled; delete c.intensity; } if (/_cone$/.test(l.name) || group === 'player' || group === 'flashlights') delete c.pos; if (l.name === 'pistol_light' || l.name === 'rifle_light') delete c.enabled; return c; }); return o; };
let bad = 0, same = 0;
for (let i = 0; i < CASES.length; i++) {
  const [l, w, size] = CASES[i], c = Bun.spawnSync(['bun', 'run', '--preload', './tools/blitz/expose.ts', import.meta.path, '--ts', l, w, ...(size !== undefined ? [String(size)] : [])], { cwd: ROOT, env, stderr: 'pipe', stdout: 'pipe' });
  const line = c.stdout.toString().split('\n').find(x => x.startsWith('@@'));
  if (!line) { console.log(`kitstage-diff: the TypeScript staging of ${l} ?${w} did not run (${c.exitCode})\n${c.stderr.toString().slice(-2500)}\n${c.stdout.toString().slice(-500)}`); process.exit(2); }
  const out: string[] = [], t = lightsIn(JSON.parse(line.slice(2))), r = lightsIn(rust[i]);
  cmp(t, r, '', out);
  if (out.length) { bad++; console.log(`DIFFERS ${l} ?${w}\n  ${out.join('\n  ')}`); } else { same++; console.log(`same    ${l} ?${w}${t.none ? ' (nothing to stage)' : ''}`); }
}
console.log(`kitstage-diff: ${same} the same, ${bad} differ, of ${CASES.length}; the furthest apart any number stood: ${worst}`);
rmSync(tmp, { recursive: true });
process.exit(bad ? 1 : 0);
