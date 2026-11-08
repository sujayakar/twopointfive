// A WALL CHARGE AND THE FURNITURE BEHIND THE WALL (src/game/wallcharge.ts mouthAfter / floorAfter, WALLCHARGE.farFurn; src/game/furnblast.ts firstToGo / wouldTake).
//
// The maintainer on the barracks, 10-11: "weird that it doesn't let me use a wall charge here" — the long outer wall between two bunk rooms' windows, refused
// 'no floor behind it.' The floor behind it is two cots stood against the wall: the nav grid has no open cell there TODAY, and the charge that opens the wall
// shatters the cot behind it. The rule now: where the nav grid has no floor behind a place, floor that only the piece of furniture this charge breaks first
// stands on counts — provided the hole has a way through wherever the wall engine lands it.
//
//   bun run tools/qa/wallfurn-check.ts [--levels barracks.json,…] [--cap N] [--baseline] [--verbose]      exit 0 = all pass, 1 = a FAIL line
//   WALLFURN_BREAK=1 …   = every charge is set and never set off: the promise's line, the barracks' second charge and the squads' F lines must fail
//   bun run tools/qa/probes.ts wallfurn      (also under `wallcharge`)
//
// Per level — every level a squad mission or a squad's training lesson plays on (the plain files: a look's second file is the same boxes), and the office (the room clear staged where the level has one; destruction, the wall charge and outer walls on; `held`, last below, asks more levels than these):
//   promise   the places the rule adds, an even sample of 8 a level (--cap N = of N; --cap 9999 = every one; a place it adds = one the game accepts where the nav grid as it stands has
//             no floor behind the wall): Sam really sets a charge there and sets it off (charge-rig.ts: a charge pushed by hand on the tools' copy of
//             the game never reaches the Rust game). After it the wall is holed where the charge settled, the plain nav grid has floor behind it, and
//             a short walk leads from the setting man's spot through to it. The line says how many charges settled on a place only the rule takes (the game may settle a charge on the grid's own floor a step along), and how many at the asked place itself. A place where
//             Sam is offered no row (the cursor on that wall lies on a door, a man or a switch) is counted, said, and not judged. (A sample since
//             2026-11-07: a charge really set costs seconds of play where one pushed by hand cost frames. The barracks' 45 and Mousehole's 29 were read whole.)
//   barracks  the wall in his screenshot (static 52): the nav grid alone has no floor behind it along its whole length (the fault, as he met it), and a
//             charge goes on it wherever it is asked; then the squad of four ordered onto it, the charge blown on F, and every man through the hole;
//             and a second charge, really set, on that wall once the first has holed it
//   held      A LEVEL'S PLACES FOR A WALL CHARGE, HELD IN A FILE OF ITS OWN: `tools/qa/fixtures/wallcharge/<name>.txt` holds how many places on
//             `levels/<name>.json` have floor behind them as the nav grid stands (the '(N places)' this check prints for the level), the level with its
//             lived-in rooms in: one whole number on a line; a line that starts with `#` says what was given up on purpose, which places and why. FAIL
//             when the level has another number than its file holds. FEWER: something took a place where a charge could be set and nobody chose
//             it (the compound's dressing, 11-03: wood piles at the outside foot of the wall the squad breaches took 23 of 891). MORE is red too and
//             says what to write, so that no slack builds up under which a later loss would pass. A file a level, not a table here, so that two
//             branches that each dress a level do not write at one place. A run that names no level asks EVERY level a mission on the list plays
//             on and every level with a file; the ones the lines below do not sweep are stood up for this count alone. A look's second file is asked
//             against its level's file. RED, never 'nothing to compare': a level with no file, a file that is not one count, a file for no level
//             under levels/, a file for a look's second file. A run with the lived-in rooms pinned off counts another level: it says so and is
//             not asked.
//             WHAT IT DOES NOT SEE: a place lost and another gained in one change (it is a count, not the set of places).
//             WALLPLACES_BREAK=1 = the sweep is read as one place short on every level: the line must fail on each.
// --baseline: the promise asked instead of the places with floor behind them as the nav grid stands (a count for whoever owns a dud there; no line).
// 2026-11-07: the rule is settled, and so are the wall charge's and the destruction's switches: none is written here. Gone with the rule's off side: 'a
// place the plain rule accepts is the same place with the rule on' (the game settles a place on the nav grid's own floor first: one run, one answer),
// Mousehole's 'the furniture index alone: refused …' (the hole's clearing switched off) and the held files' first number (the old stand's).
import { standUp, ROOT } from './headless.ts';
import { samSets, samBlows } from './charge-rig.ts';
import { sample, swept } from './sweep.ts';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

export async function wallfurnCheck(o: { levels?: string[]; cap?: number; baseline?: boolean; verbose?: boolean } = {}): Promise<boolean> {
  const { wallAt, WALL_ORDER } = await import(`${ROOT}/src/game/wallcharge.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { SCENARIOS, ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const BREAK = process.env.WALLFURN_BREAK === '1', CAP = o.cap ?? swept(3, 8);
  const all: string[] = [...new Set<string>([...[...SCENARIOS].filter((e: any) => e.kind !== 'quiet').map((e: any) => e.level).filter((l: any) => typeof l === 'string'), 'office.json'])].sort();
  const LEVELS = o.levels ? [...o.levels] : sample(all, 1); if (BREAK) for (const f of ['mousehole.json', 'barracks.json']) if (!LEVELS.includes(f)) LEVELS.unshift(f);
  const held = await heldPlaces(), promised = new Set(LEVELS);   // (`promised`: the levels the lines of this check were written for; a level added below is stood up for its held count alone)
  if (!o.levels && swept(0, 1)) for (const f of [...new Set<string>([...[...ALL_MISSIONS].filter((e: any) => !e.offList).map((e: any) => typeof e.level === 'string' ? e.level : 'office.json'), ...held.rows.keys()])].sort()) if (!LEVELS.includes(f)) LEVELS.push(f);
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const rnd = Math.random;

  async function rig(file: string) {
    Math.random = seeded(0x77a1);
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl })); const g = W.game;
    g.tune.chaos = 0.5; g.godMode = true;
    let started = false;
    if (lvl.scenarios?.roomClear) { const r: any = await quiet(() => startFromQuery(g, new URLSearchParams('roomclear&squad4'))); started = !r?.why; }
    const still = () => { for (const gd of g.guards) if (gd.team !== 'operative') { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; if (gd.flashlight) gd.flashlight.enabled = false; } };
    const each = () => { g.player.visibility = 0.05; still(); }, step = (n = 1) => { for (let i = 0; i < n; i++) { each(); W.step(DT); } };
    step(30);
    const pathLen = (a: number[], b: number[]): number => { const path = g.col.nav.findPath(a, b); if (!path) return Infinity; let L = 0, q = a; for (const w of path) { L += flat(q, w); q = w; } return L + flat(q, b); };
    return { W, g, C: g.wallCharge as any, O: g.orders as any, step, each, pathLen, started };
  }
  type Rig = Awaited<ReturnType<typeof rig>>;
  /** one asked place: where the game settles the charge (`w`), whether it takes one there (`ok`), and whether the nav grid as it stands has floor behind it
   *  there (`plain`: the game settles on such floor first, over every slide, so a place with it is the place the charge always went) */
  interface Place { w: any; ask: any; ok: boolean; plain: boolean }

  /** every place a charge can be asked for on this level (settled as the menu settles it), both faces, every 0.25 m */
  function sweep(R: Rig): Place[] {
    const { g, C } = R, boxes = g.col.boxes, N = Math.min(g.destr.walls.authored, boxes.length), out: Place[] = [];
    for (let i = 0; i < N; i++) {
      const b = boxes[i]; if (b.h[1] < 0.9 || Math.min(b.h[0], b.h[2]) > 0.2 || Math.max(b.h[0], b.h[2]) < 0.75) continue;
      const alongX = b.h[0] >= b.h[2], L = alongX ? b.h[0] : b.h[2];
      for (let u = -L + 0.1; u <= L - 0.1 + 1e-6; u += 0.25) for (const side of [1, -1]) {
        const at = alongX ? [b.c[0] + u, 0, b.c[2]] : [b.c[0], 0, b.c[2] + u], from = alongX ? [at[0], 0, b.c[2] + side * 2] : [b.c[0] + side * 2, 0, at[2]];
        const w0 = wallAt(g, at, from); if (!w0 || w0.index !== i || !w0.ok) continue;
        const w = C.settle(w0); if (!C.setSpot(w)) continue;   // (nowhere to set it from: another refusal's business)
        const ok = !C.openAt(w) && !!C.farFloor(w); out.push({ w, ask: w0, ok, plain: ok && !!C.farFloor(w, false) });
      }
    }
    return out;
  }
  const uniq = (ws: any[]) => { const seen = new Set<string>(); return ws.filter(w => { const k = `${w.index}:${w.side}:${w.u.toFixed(3)}`; if (seen.has(k)) return false; seen.add(k); return true; }); };

  async function level(file: string): Promise<void> {
    console.log(`\n=== wallfurn: ${file} ===`);
    const R = await rig(file); const { g, C } = R;
    const P = sweep(R), added = uniq(P.filter(p => p.ok && !p.plain).map(p => ({ ...p.w, ask: p.ask }))), plain = uniq(P.filter(p => p.plain).map(p => ({ ...p.w, ask: p.ask })));   // (`ask`: the point of the wall the place was settled FROM — where a cursor must lie for the charge to settle there)
    console.log(`  ${P.length} asks · floor behind as the nav grid stands at ${P.filter(p => p.plain).length} (${plain.length} places), a charge taken at ${P.filter(p => p.ok).length} · places the rule adds ${added.length}`);
    { const own = file.replace(/-gwot\.json$/, '.json'), name = `${held.dir}/${own.replace(/\.json$/, '.txt')}`, want = held.rows.get(own), n = plain.length - (held.broken ? 1 : 0), short = held.broken ? ' · WALLPLACES_BREAK: the sweep read as one place short' : '';
      if (!held.asked) console.log(`  held: not asked · ${held.stand}`);
      else if (want === undefined) { if (!held.bad.some(b => b.startsWith(name + ' '))) verdict(false, 'held: the level has a file of its places for a wall charge', `${name} is not there · ${n} places on ${held.stand}: its line is that count${short}`); }
      else verdict(n === want, 'held: the level has the places for a wall charge that its file holds', `${n} places on ${held.stand} · ${name} holds ${want}${n < want ? ' · FEWER: something took a place where a charge could be set' : n > want ? ` · MORE: write ${n} there` : ''}${short}`); }
    if (!promised.has(file)) return;
    const pool = o.baseline ? plain : added;
    const take = pool.length <= CAP ? pool : pool.filter((_, k) => Math.floor(k * CAP / pool.length) !== Math.floor((k - 1) * CAP / pool.length));
    const bad: string[] = [], unset: string[] = []; let there = 0, ruled = 0;
    for (const w of take) {
      g.restartEncounter(); R.step(10);
      const S = samSets(R.W, w, R.each, w.ask); if (S.charge && !C.farFloor(S.wall, false)) ruled++;   // (where it settled, before the blast: a place the nav grid has no floor behind is the rule's own)
      const B = S.charge && !BREAK ? samBlows(R.W, S.charge, R.each) : null; R.step(45);
      if (!S.charge) { unset.push(`static ${w.index} u ${f2(w.u)} side ${w.side}: ${S.why}`); continue; }   // (no row for Sam there: the cursor lay on a door, a man or a switch, not on the wall. Counted, said, and not judged: the line asks what a charge set there does)
      const at = S.charge ? S.wall : w, set = C.setSpot(at), hole = C.holeAt(at), far = C.farFloor(at, false), walk = set && far ? R.pathLen(set, far) : Infinity;   // (floor behind as the nav grid has it AFTER the blast)
      if (S.charge && at.index === w.index && Math.abs(at.u - w.u) < 1e-6) there++;
      if (!B?.hole || !hole || !far || !(walk < 7)) bad.push(`static ${w.index} u ${f2(w.u)} side ${w.side}: set ${S.set}${S.why ? ` (${S.why})` : ''} · set off ${!!B?.blown}${B?.why ? ` (${B.why})` : ''} · hole ${!!hole} floor ${!!far} walk ${f2(walk)} m`);
    }
    if (o.baseline) console.log(`  (baseline) places with floor behind as the nav grid stands: ${take.length - unset.length} charges set and set off (${unset.length} not set), ${bad.length} with no way in${bad.length ? '\n    ' + bad.slice(0, 12).join('\n    ') : ''}`);
    else if (take.length) verdict(bad.length === 0 && unset.length < take.length, `every place the rule adds (${take.length} of ${pool.length}): a charge really set there and set off holes the wall, leaves floor behind it and a short walk through`, `${take.length - unset.length} charges set and set off, ${bad.length} with no way in · ${ruled} of them settled on a place only the rule takes, ${there} at the asked place itself${unset.length ? ` · ${unset.length} not set, no row for Sam with the cursor there: ${unset.slice(0, 4).join(' · ')}` : ''}${bad.length ? '\n    ' + bad.slice(0, 12).join('\n    ') : ''}`);
    g.restartEncounter(); R.step(10);

    if (file === 'mousehole.json') {
      // THE WALL HE MET (10-11, "still seeing no wall behind it"): the ward's west outer wall, asked from outside. Cots stand against its inside face, each a hair
      // over the furniture index's size bar (0.706 m³ against 0.7), so the promise — which knew the index alone — refused six metres of it, while a charge blown
      // there opens a way in: the blown hole's own clearing (furnblast.ts mouth()) takes the cots whole. The promise now asks that rule too (WALLCHARGE.farMouth).
      const N = Math.min(g.destr.walls.authored, g.col.boxes.length); let wi = -1;
      for (let i = 0; i < N && wi < 0; i++) { const b = g.col.boxes[i]; if (Math.abs(b.c[0] - 9) < 0.05 && Math.abs(b.c[2] - 11.71) < 0.15 && b.h[2] > 4 && b.h[1] > 0.9) wi = i; }
      const b = g.col.boxes[wi], asks: any[] = [];
      if (b) for (let u = -b.h[2] + 0.1; u <= b.h[2] - 0.1 + 1e-6; u += 0.25) { const w0 = wallAt(g, [9, 0, b.c[2] + u], [7, 0, b.c[2] + u]); if (w0 && w0.index === wi && w0.ok) asks.push(w0); }
      const taken = asks.filter(w0 => { const w = C.settle(w0); return !!C.setSpot(w) && !C.openAt(w) && !!C.farFloor(w); }).length, bare = asks.filter(w0 => { const w = C.settle(w0); return !!C.setSpot(w) && !C.openAt(w) && !!C.farFloor(w, false); }).length;
      verdict(wi >= 0 && asks.length >= 20, 'the ward\'s west outer wall is found, and asked from outside along its length', b ? `static ${wi} c (${f2(b.c[0])}, ${f2(b.c[2])}) half ${f2(b.h[2])} · ${asks.length} asks` : 'no such wall');
      verdict(taken === asks.length && bare < asks.length, 'cots against its inside face, and with what the blown hole clears counted a charge goes on it wherever it is asked', `${taken} of ${asks.length} accepted · the nav grid alone has floor behind ${bare} of them`);
      if (R.started) {
        const { O } = R; if (C.carried !== null) C.setCarried(2);
        const near = [7.2, 0, 10.2], p = [9, 1.05, 10.2];
        g.teleportPlayer(near); R.step(3); O.give('fallIn', O.targetAt(g.player.char.pos), 'all');
        for (let i = 0; i < 75 * 60 && !O.squad().filter((m: any) => m.state !== 'dead').every((m: any) => flat(m.char.pos, near) < 6 && O.of(m)?.arrived); i++) R.step();
        const why = O.give(WALL_ORDER, O.targetAt(p), 'all');
        for (let i = 0; i < 60 * 60 && C.ops[0]?.phase !== 'armed'; i++) R.step();
        const op = C.ops[0], armed = op?.phase === 'armed';
        verdict(!why && armed, 'the squad ordered onto it from the yard: the order is taken and the charge is set', `refusal ${JSON.stringify(why ?? null)} · phase ${op?.phase ?? '—'}`);
        if (!BREAK) R.W.input.pressed.add('KeyF'); R.step(); R.step(60 * 25);
        const men = O.squad().filter((m: any) => m.state !== 'dead'), inside = men.filter((m: any) => m.char.pos[0] > 9.1 && m.char.pos[0] < 15 && m.char.pos[2] > 7 && m.char.pos[2] < 15);
        verdict(!!op?.hole && inside.length === men.length && men.length > 0, 'F: the wall opens and every man goes into the ward through it', `hole ${!!op?.hole} · ended '${op?.ended}' · ${inside.length} of ${men.length} inside · ${men.map((m: any) => `${m.callsign} (${f2(m.char.pos[0])}, ${f2(m.char.pos[2])})`).join(' ')}`);
        g.restartEncounter(); R.step(10);
      }
      return;
    }
    if (file !== 'barracks.json') return;
    // the wall in the screenshot: the south face between the windows of bunks 3 and bunks 4, asked from outside
    const S = P.filter(p => p.w.index === 52 && p.w.side === 1), b = g.col.boxes[52];
    verdict(!!b && Math.abs(b.c[0] - 19.4) < 0.05 && Math.abs(b.c[2] - 24) < 0.05 && S.length > 0, 'static 52 is that wall, and the sweep asked it from outside', b ? `c (${f2(b.c[0])}, ${f2(b.c[2])}) half ${f2(b.h[0])} · ${S.length} asks` : 'no static 52');
    verdict(S.length > 0 && S.every(p => !p.plain), 'the nav grid alone has no floor behind it along its whole length (the fault, as he met it: "no floor behind it")', `${S.filter(p => !p.plain).length} of ${S.length} with no floor behind as the grid stands`);
    verdict(S.length > 0 && S.every(p => p.ok), 'a charge goes on it wherever it is asked along it', `${S.filter(p => p.ok).length} of ${S.length} accepted · settled at x ${[...new Set(S.filter(p => p.ok).map(p => f2(p.w.u)))].join(', ')}`);
  }

  try { for (const f of LEVELS) await level(f); }
  finally { Math.random = rnd; }
  if (held.bad.length) verdict(false, 'held: every file of held wall-charge places is one count for a level under levels/', held.bad.join('\n    '));
  console.log(`\n=== wallfurn-check: ${fails ? `${fails} FAILED` : 'ALL PASS'} (${checks} checks) ===`);
  return fails === 0;
}

/** the held counts (the head's `held` says what a file is): each level's number, the files that are no such line, and whether this run is one a file is counted on */
async function heldPlaces(): Promise<{ dir: string; rows: Map<string, number>; bad: string[]; asked: boolean; stand: string; broken: boolean }> {
  const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
  const dir = 'tools/qa/fixtures/wallcharge', rows = new Map<string, number>(), bad: string[] = [];
  if (existsSync(resolve(ROOT, dir))) for (const t of readdirSync(resolve(ROOT, dir)).filter(t => t.endsWith('.txt')).sort()) {
    const level = t.replace(/\.txt$/, '.json');
    if (!existsSync(resolve(ROOT, 'levels', level))) { bad.push(`${dir}/${t} is the count of no file under levels/`); continue; }
    if (/-gwot\.txt$/.test(t)) { bad.push(`${dir}/${t} is a count for a look's second file: such a file is asked against its level's file and has none of its own`); continue; }
    const body = readFileSync(resolve(ROOT, dir, t), 'utf8').split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('#')), m = body.length === 1 ? /^(\d+)$/.exec(body[0]) : null;
    if (!m) { bad.push(`${dir}/${t} is not one count (the places with floor behind them as the nav grid stands, the level with its lived-in rooms in: one whole number on a line; a line that starts with # is a note)`); continue; }
    rows.set(level, Number(m[1]));
  }
  return { dir, rows, bad, asked: !!LIVED.on, stand: LIVED.on ? 'the level with its lived-in rooms in' : 'a run with the lived-in rooms pinned off counts another level than a file holds', broken: process.env.WALLPLACES_BREAK === '1' };
}

if (import.meta.main) {
  const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
  process.exit((await wallfurnCheck({ levels: flag('--levels')?.split(','), cap: flag('--cap') ? Number(flag('--cap')) : undefined, baseline: argv.includes('--baseline'), verbose: argv.includes('--verbose') })) ? 0 : 1);
}
