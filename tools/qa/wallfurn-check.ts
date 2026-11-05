// A WALL CHARGE AND THE FURNITURE BEHIND THE WALL (src/game/wallcharge.ts mouthAfter / floorAfter, WALLCHARGE.farFurn; src/game/furnblast.ts firstToGo / wouldTake).
//
// The maintainer on the barracks, 10-11: "weird that it doesn't let me use a wall charge here" — the long outer wall between two bunk rooms' windows, refused
// 'no floor behind it.' The floor behind it is two cots stood against the wall: the nav grid has no open cell there TODAY, and the charge that opens the wall
// shatters the cot behind it. The rule now: where the nav grid has no floor behind a place, floor that only the piece of furniture this charge breaks first
// stands on counts — provided the hole has a way through wherever the wall engine lands it.
//
//   bun run tools/qa/wallfurn-check.ts [--levels barracks.json,…] [--cap N] [--baseline] [--verbose]      exit 0 = all pass, 1 = a FAIL line
//   WALLFURN_BREAK=1 …   = the rule never switched on and the same lines asked: must fail (the barracks' own lines and, where blasts clear their holes — the
//                          shipped stand — Mousehole's: both are always among the levels then)
//   bun run tools/qa/probes.ts wallfurn      (also under `wallcharge`)
//
// Per level — every level a squad mission or a squad's training lesson plays on (the plain files: a look's second file is the same boxes), and the office (the room clear staged where the level has one; destruction, the wall charge and outer walls on; `held`, last below, asks more levels than these):
//   same      a place the plain rule accepts is THE SAME PLACE with the rule on: the charge settles where it always did (the rule is asked only where the
//             plain one found no floor on any slide)
//   promise   EVERY place the rule adds (--cap N = an even sample of N a level): the charge really set and blown there — blow() itself reports the hole, and
//             after it the wall is holed, the plain nav grid has floor behind it, and a short walk leads from the setting man's spot through to it
//   barracks  the wall in his screenshot (static 52): refused along its whole length with the rule off, accepted with it on; then the squad of four ordered
//             onto it, the charge blown on F, and every man through the hole; and a second charge on that wall once the first has holed it
//   held      A LEVEL'S PLACES FOR A WALL CHARGE, HELD IN A FILE OF ITS OWN: `tools/qa/fixtures/wallcharge/<name>.txt` holds how many places the plain rule
//             accepts on `levels/<name>.json` (the '(N places)' this check prints for the level), on the frozen stand and then on the shipped one: two
//             whole numbers on a line, a space between; a line that starts with `#` says what was given up on purpose, which places and why. FAIL
//             when the level has another number than its file holds on the stand of the run. FEWER: something took a place where a charge could be
//             set and nobody chose it (the compound's dressing, 11-03: wood piles at the outside foot of the wall the squad breaches took 23 of 891).
//             MORE is red too and says what to write, so that no slack builds up under which a later loss would pass. A file a level, not a table
//             here, so that two branches that each dress a level do not write at one place (tools/qa/fixtures/level-base/, round/ and carlamps/ are
//             the same mend). A run that names no level asks EVERY level a mission on the list plays on (the tutorials and the quiet missions too: a
//             wall there can be breached by a player who chooses to) and every level with a file; the ones the lines below do not sweep are stood up
//             for this count alone. A look's second file is asked against its level's file. RED, never 'nothing to compare': a level with no file, a
//             file that is not two counts, a file for no level under levels/, a file for a look's second file. A run that is neither stand as it
//             comes (lived-in rooms pinned on over the frozen stand, or off under the shipped one) counts another level than either number: it
//             says so and is not asked.
//             WHAT IT DOES NOT SEE: a place lost and another gained in one change (it is a count, not the set of places).
//             WALLPLACES_BREAK=1 = the sweep is read as one place short on every level: the line must fail on each, on both stands.
// --baseline: the promise asked instead of the places the PLAIN rule accepts, at the plain rule's own spots (what a charge does there with or without this
// rule — the plain rule's duds are not this check's to fail on; a count for whoever owns them).
import { standUp, ROOT } from './headless.ts';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

export async function wallfurnCheck(o: { levels?: string[]; cap?: number; baseline?: boolean; verbose?: boolean } = {}): Promise<boolean> {
  const { WALLCHARGE, wallAt, WALL_ORDER } = await import(`${ROOT}/src/game/wallcharge.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { FURN } = await import(`${ROOT}/src/game/furnblast.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { SCENARIOS, ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const BREAK = process.env.WALLFURN_BREAK === '1', RULE = !BREAK, CAP = o.cap ?? Infinity;
  const all: string[] = [...new Set<string>([...[...SCENARIOS].filter((e: any) => e.kind !== 'quiet').map((e: any) => e.level).filter((l: any) => typeof l === 'string'), 'office.json'])].sort();
  const LEVELS = [...(o.levels ?? all)]; if (BREAK) for (const f of ['mousehole.json', 'barracks.json']) if (!LEVELS.includes(f)) LEVELS.unshift(f);
  const held = await heldPlaces(), promised = new Set(LEVELS);   // (`promised`: the levels the lines of this check were written for; a level added below is stood up for its held count alone)
  if (!o.levels) for (const f of [...new Set<string>([...[...ALL_MISSIONS].filter((e: any) => !e.offList).map((e: any) => typeof e.level === 'string' ? e.level : 'office.json'), ...held.rows.keys()])].sort()) if (!LEVELS.includes(f)) LEVELS.push(f);
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const dOn = DESTRUCTION.on, rnd = Math.random, saved = JSON.parse(JSON.stringify(WALLCHARGE));

  async function rig(file: string) {
    Math.random = seeded(0x77a1); DESTRUCTION.on = true;   // (before the level is loaded and stood up: its dressing and the destruction lane's own index are made under the switch)
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl })); const g = W.game;
    DESTRUCTION.on = true; WALLCHARGE.on = true; WALLCHARGE.outer = true; WALLCHARGE.farFurn = false; g.tune.chaos = 0.5; g.godMode = true;
    let started = false;
    if (lvl.scenarios?.roomClear) { const r: any = await quiet(() => startFromQuery(g, new URLSearchParams('roomclear&squad4'))); started = !r?.why; }
    const still = () => { for (const gd of g.guards) if (gd.team !== 'operative') { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; if (gd.flashlight) gd.flashlight.enabled = false; } };
    const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; still(); W.step(DT); } };
    step(30);
    const pathLen = (a: number[], b: number[]): number => { const path = g.col.nav.findPath(a, b); if (!path) return Infinity; let L = 0, q = a; for (const w of path) { L += flat(q, w); q = w; } return L + flat(q, b); };
    return { W, g, C: g.wallCharge as any, O: g.orders as any, step, pathLen, started };
  }
  type Rig = Awaited<ReturnType<typeof rig>>;
  /** one asked place: where the charge settles and whether the floor behind lets it go on, under the plain rule (a) and with the rule on (b) */
  interface Place { a: any; aOk: boolean; b: any; bOk: boolean }

  /** every place a charge can be asked for on this level (settled as the menu settles it), both faces, every 0.25 m */
  function sweep(R: Rig): Place[] {
    const { g, C } = R, boxes = g.col.boxes, N = Math.min(g.destr.walls.authored, boxes.length), out: Place[] = [];
    for (let i = 0; i < N; i++) {
      const b = boxes[i]; if (b.h[1] < 0.9 || Math.min(b.h[0], b.h[2]) > 0.2 || Math.max(b.h[0], b.h[2]) < 0.75) continue;
      const alongX = b.h[0] >= b.h[2], L = alongX ? b.h[0] : b.h[2];
      for (let u = -L + 0.1; u <= L - 0.1 + 1e-6; u += 0.25) for (const side of [1, -1]) {
        const at = alongX ? [b.c[0] + u, 0, b.c[2]] : [b.c[0], 0, b.c[2] + u], from = alongX ? [at[0], 0, b.c[2] + side * 2] : [b.c[0] + side * 2, 0, at[2]];
        const w0 = wallAt(g, at, from); if (!w0 || w0.index !== i || !w0.ok) continue;
        const ask = (rule: boolean) => { WALLCHARGE.farFurn = rule; const w = C.settle(w0); return { w, ok: !!C.setSpot(w) && !C.openAt(w) && !!C.farFloor(w) }; };
        const A = ask(false), B = ask(RULE); WALLCHARGE.farFurn = false;
        if (!C.setSpot(A.w) && !C.setSpot(B.w)) continue;   // (nowhere to set it from: another refusal's business)
        out.push({ a: A.w, aOk: A.ok, b: B.w, bOk: B.ok });
      }
    }
    return out;
  }
  const uniq = (ws: any[]) => { const seen = new Set<string>(); return ws.filter(w => { const k = `${w.index}:${w.side}:${w.u.toFixed(3)}`; if (seen.has(k)) return false; seen.add(k); return true; }); };

  async function level(file: string): Promise<void> {
    console.log(`\n=== wallfurn: ${file} ===`);
    const R = await rig(file); const { g, C } = R;
    const P = sweep(R), moved = P.filter(p => p.aOk && (!p.bOk || Math.abs(p.a.u - p.b.u) > 1e-9)), added = uniq(P.filter(p => p.bOk && !p.aOk).map(p => p.b)), plain = uniq(P.filter(p => p.aOk).map(p => p.a));
    console.log(`  ${P.length} asks · the plain rule accepts ${P.filter(p => p.aOk).length} (${plain.length} places), with the rule on ${P.filter(p => p.bOk).length} · places added ${added.length}`);
    verdict(moved.length === 0, 'a place the plain rule accepts is the same place with the rule on', `moved or lost ${moved.length}${moved.length ? ' · ' + moved.slice(0, 4).map(p => `static ${p.a.index} u ${f2(p.a.u)} → ${p.bOk ? f2(p.b.u) : 'refused'}`).join(' · ') : ''}`);
    { const own = file.replace(/-gwot\.json$/, '.json'), name = `${held.dir}/${own.replace(/\.json$/, '.txt')}`, want = held.rows.get(own), n = plain.length - (held.broken ? 1 : 0), short = held.broken ? ' · WALLPLACES_BREAK: the sweep read as one place short' : '';
      if (held.col < 0) console.log(`  held: not asked · ${held.stand}`);
      else if (!want) { if (!held.bad.some(b => b.startsWith(name + ' '))) verdict(false, 'held: the level has a file of its places for a wall charge', `${name} is not there · ${n} places on ${held.stand}: its line is the frozen stand's count, a space, the shipped stand's${short}`); }
      else verdict(n === want[held.col], 'held: the level has the places for a wall charge that its file holds', `${n} places on ${held.stand} · ${name} holds ${want[held.col]}${n < want[held.col] ? ' · FEWER: something took a place where a charge could be set' : n > want[held.col] ? ` · MORE: write ${n} there` : ''}${short}`); }
    if (!promised.has(file)) return;
    const pool = o.baseline ? plain : added;
    const take = pool.length <= CAP ? pool : pool.filter((_, k) => Math.floor(k * CAP / pool.length) !== Math.floor((k - 1) * CAP / pool.length));
    const bad: string[] = [];
    for (const w of take) {
      g.restartEncounter(); R.step(10);
      WALLCHARGE.farFurn = false; const set = C.setSpot(w);
      C.set.push({ wall: w, t: g.time, by: 'probe', paid: 0 }); const r = C.blow(C.set[0]); R.step(45);
      const hole = C.holeAt(w), far = C.farFloor(w), walk = set && far ? R.pathLen(set, far) : Infinity;
      if (!r?.hole || !hole || !far || !(walk < 7)) bad.push(`static ${w.index} u ${f2(w.u)} side ${w.side}: blow() hole ${!!r?.hole} · later hole ${!!hole} floor ${!!far} walk ${f2(walk)} m`);
    }
    if (o.baseline) console.log(`  (baseline) the plain rule's own places: ${take.length} blown, ${bad.length} with no way in${bad.length ? '\n    ' + bad.slice(0, 12).join('\n    ') : ''}`);
    else if (take.length) verdict(bad.length === 0, `every place the rule adds (${take.length} of ${pool.length}): the charge blown there holes the wall, leaves floor behind it and a short walk through`, bad.length ? bad.slice(0, 12).join('\n    ') : `${take.length} charges blown, ${take.length} ways in`);
    g.restartEncounter(); R.step(10);

    if (file === 'mousehole.json') {
      // THE WALL HE MET (10-11, "still seeing no wall behind it"): the ward's west outer wall, asked from outside. Cots stand against its inside face, each a hair
      // over the furniture index's size bar (0.706 m³ against 0.7), so the promise — which knew the index alone — refused six metres of it, while a charge blown
      // there opens a way in: the blown hole's own clearing (furnblast.ts mouth()) takes the cots whole. The promise now asks that rule too (WALLCHARGE.farMouth).
      const N = Math.min(g.destr.walls.authored, g.col.boxes.length); let wi = -1;
      for (let i = 0; i < N && wi < 0; i++) { const b = g.col.boxes[i]; if (Math.abs(b.c[0] - 9) < 0.05 && Math.abs(b.c[2] - 11.71) < 0.15 && b.h[2] > 4 && b.h[1] > 0.9) wi = i; }
      const b = g.col.boxes[wi], asks: any[] = [];
      if (b) for (let u = -b.h[2] + 0.1; u <= b.h[2] - 0.1 + 1e-6; u += 0.25) { const w0 = wallAt(g, [9, 0, b.c[2] + u], [7, 0, b.c[2] + u]); if (w0 && w0.index === wi && w0.ok) asks.push(w0); }
      const count = (furn: boolean, mouth: boolean) => { WALLCHARGE.farFurn = furn; WALLCHARGE.farMouth = mouth; const n = asks.filter(w0 => { const w = C.settle(w0); return !!C.setSpot(w) && !C.openAt(w) && !!C.farFloor(w); }).length; WALLCHARGE.farFurn = false; WALLCHARGE.farMouth = true; return n; };
      verdict(wi >= 0 && asks.length >= 20, 'the ward\'s west outer wall is found, and asked from outside along its length', b ? `static ${wi} c (${f2(b.c[0])}, ${f2(b.c[2])}) half ${f2(b.h[2])} · ${asks.length} asks` : 'no such wall');
      const plain = count(false, false), index = count(RULE, false), both = count(RULE, true);
      if (FURN.mouth.on) {
        verdict(asks.length - index >= 8, 'the furniture index alone: refused for the floor behind it along the cots (the fault, as he met it)', `${asks.length - index} of ${asks.length} refused (plain rule ${asks.length - plain})`);
        verdict(both === asks.length, 'with what the blown hole clears counted: a charge goes on it wherever it is asked', `${both} of ${asks.length} accepted`);
      } else verdict(both === index, 'this stand\'s blasts clear no hole (FURN.mouth off): the promise says no more than the blast does', `accepted ${both} with the hole\'s clearing asked, ${index} without, of ${asks.length}`);
      if (R.started && FURN.mouth.on) {
        const { O } = R; WALLCHARGE.farFurn = RULE; if (C.carried !== null) C.setCarried(2);
        const near = [7.2, 0, 10.2], p = [9, 1.05, 10.2];
        g.teleportPlayer(near); R.step(3); O.give('fallIn', O.targetAt(g.player.char.pos), 'all');
        for (let i = 0; i < 75 * 60 && !O.squad().filter((m: any) => m.state !== 'dead').every((m: any) => flat(m.char.pos, near) < 6 && O.of(m)?.arrived); i++) R.step();
        const why = O.give(WALL_ORDER, O.targetAt(p), 'all');
        for (let i = 0; i < 60 * 60 && C.ops[0]?.phase !== 'armed'; i++) R.step();
        const op = C.ops[0], armed = op?.phase === 'armed';
        verdict(!why && armed, 'the squad ordered onto it from the yard: the order is taken and the charge is set', `refusal ${JSON.stringify(why ?? null)} · phase ${op?.phase ?? '—'}`);
        R.W.input.pressed.add('KeyF'); R.step(); R.step(60 * 25);
        const men = O.squad().filter((m: any) => m.state !== 'dead'), inside = men.filter((m: any) => m.char.posX > 9.1 && m.char.posX < 15 && m.char.posZ > 7 && m.char.posZ < 15);
        verdict(!!op?.hole && inside.length === men.length && men.length > 0, 'F: the wall opens and every man goes into the ward through it', `hole ${!!op?.hole} · ended '${op?.ended}' · ${inside.length} of ${men.length} inside · ${men.map((m: any) => `${m.callsign} (${f2(m.char.posX)}, ${f2(m.char.posZ)})`).join(' ')}`);
        WALLCHARGE.farFurn = false; g.restartEncounter(); R.step(10);
      }
      return;
    }
    if (file !== 'barracks.json') return;
    // the wall in the screenshot: the south face between the windows of bunks 3 and bunks 4, asked from outside
    const S = P.filter(p => p.a.index === 52 && p.a.side === 1), b = g.col.boxes[52];
    verdict(!!b && Math.abs(b.c[0] - 19.4) < 0.05 && Math.abs(b.c[2] - 24) < 0.05 && S.length > 0, 'static 52 is that wall, and the sweep asked it from outside', b ? `c (${f2(b.c[0])}, ${f2(b.c[2])}) half ${f2(b.h[0])} · ${S.length} asks` : 'no static 52');
    verdict(S.length > 0 && S.every(p => !p.aOk), 'rule off: refused along its whole length for the floor behind it (the fault, as he met it)', `${S.filter(p => !p.aOk).length} of ${S.length} refused`);
    verdict(S.length > 0 && S.every(p => p.bOk), 'rule on: a charge goes on it wherever it is asked along it', `${S.filter(p => p.bOk).length} of ${S.length} accepted · settled at x ${[...new Set(S.filter(p => p.bOk).map(p => f2(p.b.u)))].join(', ')}`);
    // a wall that already has a hole in it: the charge beside the first one's hole is still taken (the wall is then pieces standing in for its box)
    { WALLCHARGE.farFurn = RULE; const ask = (x: number) => { const w0 = wallAt(g, [x, 0, 24], [x, 0, 26]); const w = w0 && w0.ok ? C.settle(w0) : null; return w && C.setSpot(w) && !C.openAt(w) && C.farFloor(w) ? w : null; };
      const w1 = ask(18.4), w2 = ask(20.4); let second: any = null, way = false, kept = false;
      if (w1 && w2) { C.set.push({ wall: w1, t: g.time, by: 'probe', paid: 0 }); C.blow(C.set[0]); R.step(45); second = ask(20.4);
        const again = wallAt(g, [w2.u, 0, 24], [w2.u, 0, 26]); kept = !!again && !!C.mouthAfter({ ...again, u: w2.u });   // (the very place the rule took before, asked of the rule alone)
        if (second) { const set = C.setSpot(second); C.set.push({ wall: second, t: g.time, by: 'probe', paid: 0 }); const r = C.blow(C.set[0]); R.step(45); WALLCHARGE.farFurn = false; const far = C.farFloor(second); way = !!r?.hole && !!C.holeAt(second) && !!far && R.pathLen(set, far) < 7; } }
      WALLCHARGE.farFurn = false;
      verdict(!!w1 && !!w2 && kept && !!second && way, 'one charge blown over the west cot: the place over the east cot is still the rule\'s, and a second charge asked there opens a way in', `first ${w1 ? 'x ' + f2(w1.u) : 'refused'} · second before ${w2 ? 'x ' + f2(w2.u) : 'refused'} · that place by the rule after the first's hole: ${kept ? 'taken' : 'refused'} · asked again ${second ? 'x ' + f2(second.u) + ' on static ' + second.index : 'refused'} · way in ${way}`);
      g.restartEncounter(); R.step(10); }
    // the squad of four, by the menu's own order
    if (R.started) {
      const { O } = R; WALLCHARGE.farFurn = RULE; if (C.carried !== null) C.setCarried(2);
      const near = [19.4, 0, 26.8], p = [19.4, 1.05, 24];
      g.teleportPlayer(near); R.step(3); O.give('fallIn', O.targetAt(g.player.char.pos), 'all');
      for (let i = 0; i < 75 * 60 && !O.squad().filter((m: any) => m.state !== 'dead').every((m: any) => flat(m.char.pos, near) < 6 && O.of(m)?.arrived); i++) R.step();
      const why = O.give(WALL_ORDER, O.targetAt(p), 'all');
      for (let i = 0; i < 60 * 60 && C.ops[0]?.phase !== 'armed'; i++) R.step();
      const op = C.ops[0], armed = op?.phase === 'armed';
      verdict(!why && armed, 'the squad ordered onto it: the order is taken and the charge is set', `refusal ${JSON.stringify(why ?? null)} · phase ${op?.phase ?? '—'}`);
      R.W.input.pressed.add('KeyF'); R.step(); R.step(60 * 25);
      const men = O.squad().filter((m: any) => m.state !== 'dead'), inside = men.filter((m: any) => m.char.posZ < 23.8 && m.char.posZ > 20 && m.char.posX > 16 && m.char.posX < 23);
      verdict(!!op?.hole && inside.length === men.length && men.length > 0, 'F: the wall opens and every man goes in through it', `hole ${!!op?.hole} · ended '${op?.ended}' · ${inside.length} of ${men.length} inside · ${men.map((m: any) => `${m.callsign} (${f2(m.char.posX)}, ${f2(m.char.posZ)})`).join(' · ')}`);
    } else verdict(false, 'the room clear starts on the barracks', 'start refused');
  }

  try { for (const f of LEVELS) await level(f); }
  finally { Object.assign(WALLCHARGE, saved); DESTRUCTION.on = dOn; Math.random = rnd; }
  if (held.bad.length) verdict(false, 'held: every file of held wall-charge places is two counts for a level under levels/', held.bad.join('\n    '));
  console.log(`\n=== wallfurn-check: ${fails ? `${fails} FAILED` : 'ALL PASS'} (${checks} checks) ===`);
  return fails === 0;
}

/** the held counts (the head's `held` says what a file is): each level's two numbers, the files that are no such line, and which number this run is held to */
async function heldPlaces(): Promise<{ dir: string; rows: Map<string, [number, number]>; bad: string[]; col: number; stand: string; broken: boolean }> {
  const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`), { harnessProfile } = await import(`${ROOT}/src/host/profile.ts`);
  const dir = 'tools/qa/fixtures/wallcharge', rows = new Map<string, [number, number]>(), bad: string[] = [];
  if (existsSync(resolve(ROOT, dir))) for (const t of readdirSync(resolve(ROOT, dir)).filter(t => t.endsWith('.txt')).sort()) {
    const level = t.replace(/\.txt$/, '.json');
    if (!existsSync(resolve(ROOT, 'levels', level))) { bad.push(`${dir}/${t} is the count of no file under levels/`); continue; }
    if (/-gwot\.txt$/.test(t)) { bad.push(`${dir}/${t} is a count for a look's second file: such a file is asked against its level's file and has none of its own`); continue; }
    const body = readFileSync(resolve(ROOT, dir, t), 'utf8').split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('#')), m = body.length === 1 ? /^(\d+) (\d+)$/.exec(body[0]) : null;
    if (!m) { bad.push(`${dir}/${t} is not two counts (the places the plain rule accepts on the frozen stand, then on the shipped one: two whole numbers on one line, a space between; a line that starts with # is a note)`); continue; }
    rows.set(level, [Number(m[1]), Number(m[2])]);
  }
  const shipped = harnessProfile() === 'shipped', col = shipped && LIVED.on ? 1 : !shipped && !LIVED.on ? 0 : -1;
  return { dir, rows, bad, col, stand: col === 1 ? 'the shipped stand' : col === 0 ? 'the frozen stand' : `the ${shipped ? 'shipped' : 'frozen'} stand with its lived-in rooms pinned ${LIVED.on ? 'on' : 'off'} is neither stand a file is counted on`, broken: process.env.WALLPLACES_BREAK === '1' };
}

if (import.meta.main) {
  const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
  process.exit((await wallfurnCheck({ levels: flag('--levels')?.split(','), cap: flag('--cap') ? Number(flag('--cap')) : undefined, baseline: argv.includes('--baseline'), verbose: argv.includes('--verbose') })) ? 0 : 1);
}
