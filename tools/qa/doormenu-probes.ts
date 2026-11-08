// The 'doormenu' family: X ON A DOOR IS THAT DOOR'S MENU, THE SAME ONE IN EVERY SCENARIO (the maintainer, 10-05, off the hostage rescue: "the two man hostage
// situation doesn't have the door tools menu? this should all be consistent"). The menu's rows never differed by scenario — what differed is how often the
// cursor ON A DOORWAY AS DRAWN was taken for a spot of floor: through an OPEN doorway its ray lands on the room's floor past the frame line, and the floor's
// menu has no door rows — and the rescues are the scenarios whose doors stand open. Run through probes.ts:
//   bun run tools/qa/probes.ts doormenu | doormenu-drawn | doormenu-open | doormenu-rows | doormenu-man | doormenu-knobs
// PASS / FAIL each, all headless (the Game of headless.ts stood up on each row's own level; nothing is drawn — "as drawn" is the cursor's ray against the same
// boxes the picture is made of, so this is the rule held to itself over every door, NOT a picture looked at). THE SCENARIOS are scene/scenarioList.ts
// SCENARIOS, row by row, each started as a click on its row starts it.
//   doormenu-drawn   in each scenario, every door as the scenario stages it: Sam on each free spot round it, the camera settled, the doorway's rectangle
//                    sampled across and up; every sample where the doorway IS DRAWN (nothing of the scene nearer along the cursor's ray than the doorway —
//                    men and glass looked past — or the door's own leaf there) must pick THAT door. Counted apart, as the rule has them: a teammate under
//                    the cursor (his menu), Sam's own body under it (his), another door nearer the landing point (that door's), another door's OWN LEAF seen through this doorway (that door's: a leaf the cursor is on is the
//                    door, orders.ts targetUnder — two doors swung open in one corner). A door never
//                    drawn from any spot is a FAIL (nothing was checked on it).
//   doormenu-open    the same with every door of the level stood open: the cursor through an open doorway is the door, in every scenario
//   doormenu-rows    in each scenario: X on a shut door and on an open one (the cursor THROUGH the opening), the panel read — the first page's rows and keys,
//                    the row that opens the tools on 5, then 5 pressed and the tools page read. Every scenario must show the same rows on the same keys (the
//                    four's 'wait for my word' row counted apart) with at least one tool usable on the shut door; a tool that cannot be used there (a
//                    charge on an open door, no bangs left) is LISTED, greyed, with its reason — never absent. Row 4 of the tools is the one order
//                    'clear from outside', which 4 pressed must give
//   doormenu-man     what the rule leaves alone: a teammate stood in a doorway, the cursor on his chest — X is HIS menu and T is him
//                    (one door a scenario)
// THE DOORWAY RULE AND THE ONE OUTSIDE ORDER ARE SETTLED (ORDERS.doorRay, DOORTOOLS.outsidePlain; 2026-11-07): nothing here writes either or asks the pick by
// nearness alone. Out with that: the probe `knobs` (the knob off is the pick as it was), the 'by nearness alone' count beside every line of `drawn` and
// `open`, the threshold's page behind row 4 in `rows`, and the off pick `man` used to tell a hidden man from a shown one (now read off the scene's own boxes).
// The family's own red path: DOORMENU_BREAK=1 bun run tools/qa/probes.ts doormenu-drawn must FAIL (the cursor read a quarter of the screen off every sample).
// And of the door a reading is taken on: DOORMENUFIND_BREAK=1 bun run tools/qa/probes.ts doormenu-rows must FAIL (find, below: a leaf that moved under the stand).
import { listedOf, ringUp, ringDown } from './squadwords';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { DOORTOOLS, TOOLS_PAGE, THR_PAGE, isOpen } = await import(`${ROOT}/src/game/doortools.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { RESCUE } = await import(`${ROOT}/src/game/rescue.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { ORDERS, WORD_HEAD } = await import(`${ROOT}/src/game/orders.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);

const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const clone = (o: any) => JSON.parse(JSON.stringify(o));
// (startFromQuery writes the scenarios' team-start knobs — NIGHTBREACH.samAt, RESCUE.opsAt … — per level: put back with the rest, as the sibling families do)
const DEFAULTS = { DT: clone(DOORTOOLS), RC: clone(ROOMCLEAR), NB: clone(NIGHTBREACH), RS: clone(RESCUE), ORD: clone(ORDERS) };
const reset = () => { Object.assign(DOORTOOLS, clone(DEFAULTS.DT)); Object.assign(ROOMCLEAR, clone(DEFAULTS.RC)); Object.assign(NIGHTBREACH, clone(DEFAULTS.NB)); Object.assign(RESCUE, clone(DEFAULTS.RS)); Object.assign(ORDERS, clone(DEFAULTS.ORD)); };
/** every probe leaves the knob tables as it found them, and one that throws is a FAIL line, not a crash of the runner */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const quiet = <T>(fn: () => T): T => { if (VERBOSE) return fn(); const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const levelOfFile = (file: string): any => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); };
const pct = (a: number, b: number) => b ? `${Math.round(100 * a / b)} %` : '—';

interface Scn { name: string; query: string; level?: string; squad: 2 | 4 }
const LIST: Scn[] = SCENARIOS.map((e: any) => ({ name: e.title as string, query: scenarioQuery(e) as string, level: e.level as string | undefined, squad: e.squad as 2 | 4 }));

/** One game on the scenario's own level, the scenario started as its row starts it. step() keeps Sam dark and the defenders dazzled: this family is about what
 *  the cursor is on, not the fight. */
async function rig(scn: Scn, seed = 33) {
  reset(); Math.random = seeded(0xd00e + seed * 7919);
  let W: any;
  { const log = console.log, err = console.error, warn = console.warn; if (!VERBOSE) console.log = console.error = console.warn = () => { };
    try { W = await standUp({ uploadEvery: 1, ...(scn.level ? { level: levelOfFile(scn.level) } : {}) }); W.game.godMode = true; startFromQuery(W.game, new URLSearchParams(scn.query)); }
    finally { console.log = log; console.error = err; console.warn = warn; } }
  const game = W.game, input = W.input, O = game.orders, world = game.engine.world, CW = W.canvas.clientWidth as number, CH = W.canvas.clientHeight as number;
  const R = { W, game, input, O, scn, CW, CH,
    hostiles(): any[] { return [...(game.nightBreach.hostiles ?? []), ...(game.rescue.hostiles ?? [])]; },
    step(n = 1) { quiet(() => { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; for (const h of R.hostiles()) if (h.state !== 'dead') h.dazzledUntil = game.time + 5; W.step(DT); } }); },
    press(code: string) { input.pressed.add(code); R.step(); },
    /** the frame line of a door: along it, its normal, its centre, its width */
    frame(d: any) { const th = d.def.closedDir; return { ax: Math.cos(th), az: Math.sin(th), nx: -Math.sin(th), nz: Math.cos(th), c: d.frameCentre as number[], w: d.def.width as number }; },
    /** the free spots round a door Sam is stood on: either face, square on and off to each side, with a clear line to the doorway */
    spots(d: any): number[][] {
      const F = R.frame(d), out: number[][] = [];
      for (const off of [1.6, -1.6, 2.6, -2.6]) for (const lat of [0, 1.2, -1.2]) {
        const p = [F.c[0] + F.nx * off + F.ax * lat, 0, F.c[2] + F.nz * off + F.az * lat]; if (game.col.nav.isBlocked(p[0], p[2])) continue;
        if (game.col.solidBlocked([p[0], 1.2, p[2]], [F.c[0] + F.nx * Math.sign(off) * 0.3, 1.2, F.c[2] + F.nz * Math.sign(off) * 0.3])) continue;
        out.push(p); }
      return out;
    },
    /** Sam put there, the camera on him and settled (it is damped: a pick taken mid-glide is a pick of another picture) */
    stand(p: number[]) { quiet(() => game.teleportPlayer(p)); W.cam.target = [p[0], 0, p[2]]; R.step(40); },
    /** the doorway's rectangle, sampled: 7 across (a hand in from each jamb) × 6 up (shin to head height) */
    samples(d: any): number[][] {
      const F = R.frame(d), out: number[][] = [];
      for (let j = 0; j < 6; j++) for (let i = 0; i < 7; i++) { const u = (i / 6 - 0.5) * (F.w - 0.16), y = 0.15 + j / 5 * 1.8; out.push([F.c[0] + F.ax * u, y, F.c[2] + F.az * u]); }
      return out;
    },
    /** A point of a doorway as the camera has it now: where on the screen (null: off it), whether the doorway is DRAWN there, and what the ray lands on.
     *  DRAWN, read off the boxes the frame is made of: along the cursor's ray — through glass, on past men and their kit — the first landing is the door's
     *  own leaf, or lies no nearer than the sample (the doorway is open there and the room shows through it, or the leaf is shut in it). `through`: the
     *  landing is past the frame line by more than a stride AND farther than doorPickM from the doorway (an open doorway looked through, where nearness alone
     *  does not say door). `man` / `sam`: a teammate's / Sam's own body is under the cursor. `at`: the
     *  point the cursor's own ray lands on (what targetAt is asked about). */
    look(d: any, q: number[]): { sx: number; sy: number; drawn: boolean; through: boolean; own: boolean; man: boolean; sam: boolean; at: number[] | null; leafOf: any } | null {
      const [sx, sy, front] = W.cam.project(q, CW, CH); if (!front || sx < 2 || sy < 2 || sx > CW - 2 || sy > CH - 2) return null;
      const ray = W.cam.ray(sx / CW, sy / CH), tq = Math.hypot(q[0] - ray.ro[0], q[1] - ray.ro[1], q[2] - ray.ro[2]);
      const first = world.raycast(ray.ro, ray.rd, 200, undefined, PLAYER_ID), at = first ? [0, 1, 2].map(k => ray.ro[k] + ray.rd[k] * first.t) : null;
      const man = O.squad().some((gd: any) => gd.state !== 'dead' && gd.char.rayHit(ray.ro, ray.rd, first ? first.t + 0.3 : 200) >= 0);
      const sam = game.player.char.rayHit(ray.ro, ray.rd, first ? first.t + 0.3 : 200) >= 0;
      const out = (drawn: boolean, through: boolean, own: boolean, leafOf: any = null) => ({ sx, sy, drawn, through, own, man, sam, at, leafOf });   // (`leafOf`: the OTHER door whose own leaf the ray lands on, seen through this doorway)
      let t0 = 0, skip = PLAYER_ID;
      for (let k = 0; k < 24; k++) {
        const ro = [0, 1, 2].map(i => ray.ro[i] + ray.rd[i] * t0), hit = world.raycast(ro, ray.rd, 200, BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Glass, skip);
        if (!hit) return out(true, true, false);
        const b = world.all[hit.index];
        if (b === d.box) return out(true, false, true);
        if (b.owner) { t0 += hit.t + 0.02; skip = b.owner; continue; }
        const t = t0 + hit.t, far = Math.hypot(ray.ro[0] + ray.rd[0] * t - d.frameCentre[0], ray.ro[2] + ray.rd[2] * t - d.frameCentre[2]);
        return out(t >= tq - 0.12, t > tq + 0.6 && far > ORDERS.doorPickM, false, game.doors.list.find((x: any) => x.box === b && !x.off) ?? null);
      }
      return out(false, false, false);
    },
    /** what X would open on with the cursor at that pixel */
    pick(sx: number, sy: number): any { input.mouseX = sx; input.mouseY = sy; return O.targetUnder(input, W.cam, W.canvas); },
    /** a teammate stands where the target says and his body is on that pixel: the squad itself, picked on him */
    bodyAt(sx: number, sy: number, at: number[]): boolean { const ray = W.cam.ray(sx / CW, sy / CH); return O.squad().some((gd: any) => gd.state !== 'dead' && Math.hypot(gd.char.pos[0] - at[0], gd.char.pos[2] - at[2]) < 1e-3 && gd.char.rayHit(ray.ro, ray.rd, 200) >= 0); },
    /** every door of the level stood open and held there (the closer off, as the Director's doors.open(…, {hold}) does it) */
    openAll() { for (const d of game.doors.list) standOpen(d); R.step(4); },
    /** X pressed with the cursor at that pixel, a few frames with every key up: the panel's menu (or null) */
    openX(sx: number, sy: number): any { if (O.menu) { ringDown(input); R.step(); } input.mouseX = sx; input.mouseY = sy; ringUp(input, R); R.step(3); return listedOf(O); },
  };
  R.step(30);
  return R;
}
type Rig = Awaited<ReturnType<typeof rig>>;
const what = (t: any, d: any) => t.kind === 'door' ? (t.door === d ? 'door' : `the ${t.door.def.name} door`) : t.kind === 'floor' ? 'floor' : t.man ? 'a teammate' : 'nothing / Sam';

/** DOORMENU_BREAK=1, the red path: the cursor is read a quarter of the screen to the side of every sample, so what X opens on is not the door drawn there */
const SHIFT = process.env.DOORMENU_BREAK === '1' ? 0.25 : 0;
/** every door of a rig from every spot round it: the samples where it is drawn, and what each picks */
interface Sweep { doors: number; spots: number; drawn: number; hit: number; through: number; throughHit: number; hidden: number; onMan: number; atSam: number; other: number; leaf: number; misses: string[]; unseen: string[] }
function sweep(R: Rig): Sweep {
  const S: Sweep = { doors: 0, spots: 0, drawn: 0, hit: 0, through: 0, throughHit: 0, hidden: 0, onMan: 0, atSam: 0, other: 0, leaf: 0, misses: [], unseen: [] };
  for (const d of sample(R.game.doors.list as any[], 6)) {
    S.doors++; let seen = 0;
    for (const p of sample(R.spots(d), 3)) {
      R.stand(p); S.spots++;
      for (const q of R.samples(d)) {
        const L = R.look(d, q); if (!L) continue; if (!L.drawn) { S.hidden++; continue; }
        const t = R.pick(L.sx + SHIFT * R.CW, L.sy), ok = t.kind === 'door' && t.door === d;
        // what the rule gives to something else, each counted apart: a teammate under the cursor, Sam's own body, a door nearer the landing point
        // (with orders going to the whole squad, a teammate under the cursor is the squad itself and the target names no man: it is still at his feet, and it is his only where his body is on the pixel picked)
        if (!ok && L.man && t.kind === 'none' && (t.man || R.bodyAt(L.sx + SHIFT * R.CW, L.sy, t.at))) { S.onMan++; continue; }
        if (!ok && L.sam && t.kind !== 'door') { S.atSam++; continue; }
        if (!ok && t.kind === 'door' && L.at && R.O.targetAt(L.at).door === t.door) { S.other++; continue; }   // (nearer the landing point by doorPickM, as ever — a wrong door off the ray is a miss)
        if (!ok && t.kind === 'door' && L.leafOf === t.door) { S.leaf++; continue; }   // (another door's own leaf shows through this doorway and the cursor is on it: the leaf under the cursor is its door, before any doorway — orders.ts targetUnder)
        S.drawn++; seen++; if (ok) S.hit++; if (L.through) { S.through++; if (ok) S.throughHit++; }
        if (!ok && S.misses.length < 6) S.misses.push(`${d.def.name} from ${p[0].toFixed(1)},${p[2].toFixed(1)} at height ${q[1].toFixed(2)}: ${what(t, d)}${L.at ? ` (the ray lands ${Math.hypot(L.at[0] - d.frameCentre[0], L.at[2] - d.frameCentre[2]).toFixed(2)} m from the doorway)` : ''}`);
      }
    }
    if (!seen) S.unseen.push(d.def.name);
  }
  return S;
}
const sweepWords = (S: Sweep) => `${S.doors} doors from ${S.spots} spots: the doorway drawn under the cursor ${S.drawn} times, that door's menu ${S.hit} (${pct(S.hit, S.drawn)}); `
  + `of them through an open doorway ${S.through}, that door's ${S.throughHit}`
  + `\n    not counted: hidden behind a wall or furniture ${S.hidden} · a teammate's body under the cursor ${S.onMan} · Sam's own body under the cursor ${S.atSam} · another door nearer ${S.other} · another door's own leaf seen through the doorway ${S.leaf}`
  + (S.unseen.length ? `\n    NEVER DRAWN from any spot: ${S.unseen.join(', ')}` : '') + (S.misses.length ? `\n    NOT THE DOOR: ${S.misses.join(' | ')}` : '');

async function drawnFamily(title: string, open: boolean): Promise<boolean> {
  const F = family(title); let tot = 0, hit = 0;
  for (const scn of sample(LIST, 2)) {
    const R = await rig(scn); if (open) R.openAll();
    const S = sweep(R); tot += S.drawn; hit += S.hit;
    F.verdict(S.drawn > 0 && S.hit === S.drawn && !S.unseen.length, `${scn.name}: the cursor on a door as drawn is that door${open ? ', every door stood open' : ''}`, sweepWords(S));
    reset();
  }
  console.log(`  all the scenarios asked: ${hit} of ${tot} (${pct(hit, tot)})`);
  return F.done();
}
export const doormenuDrawn = guarded(() => drawnFamily("doormenu-drawn: X on a door as it is drawn opens that door's menu, in every scenario", false));
export const doormenuOpen = guarded(() => drawnFamily('doormenu-open: the cursor through an OPEN doorway is the door, in every scenario', true));

/** the panel's rows of an open menu: [the who row, one row an order …, (the four's wait row), the cancel row] */
interface PanelRow { id: string; key: string; text: string; off: boolean; why: string | null; opens: string | null }
function panel(R: Rig): { rows: PanelRow[]; wait: boolean; page: string | null } {
  const M = R.O.menu, hud = listedOf(R.O), body = (hud?.rows ?? []).slice(0, M.rows.length);
  const rows: PanelRow[] = M.rows.map((d: any, i: number) => ({ id: d.id, key: String(body[i]?.opts?.[0]?.key ?? ''), text: String(body[i]?.opts?.[0]?.text ?? ''), off: !!body[i]?.off, why: M.why[i] ?? null, opens: d.opens ?? null }));
  return { rows, wait: !!M.wordRow && hud?.rows?.[M.rows.length]?.label === WORD_HEAD, page: M.page ?? null };
}
const shape = (rows: PanelRow[]) => rows.map(r => `${r.key} ${r.id}${r.opens ? ' ▸' : ''}`).join(' · ');
/** a door of the rig with a spot and a pixel where the cursor is on it as asked: `through` an open doorway, or on the shut leaf */
/** A door that stands as asked (`through`: swung wide; else shut, by the door tools' own word for it too) with the cursor on it from a spot by it.
 *  THE LEAF IS ASKED AGAIN AFTER SAM IS STOOD BY IT: a stand steps the game, and the floor's own men open doors. In the records office on the page's stand
 *  records_west is swung wide while Sam is put by it; taken as the shut door it was a moment before, its tools page (rightly 'shut that door' on the
 *  charge's digit) was held to the shut door's shape. A door that moved under the stand is left for the next. DOORMENUFIND_BREAK=1 = the red path: the
 *  second asking is left out and the door found shut is stood open before it is handed back, as that man did. */
const FIND_BREAK = process.env.DOORMENUFIND_BREAK === '1';
const standsAs = (d: any, through: boolean) => (Math.abs(d.angle) > 0.6) === through && (through || !isOpen(d));
function standOpen(d: any) { const a = Math.abs(d.maxA) >= Math.abs(d.minA) ? Math.min(d.maxA, 1.3) : Math.max(d.minA, -1.3); d.latched = false; d.closing = false; d.angle = a; d.vel = 0; d.quietT = -3600; d.place(); }
function find(R: Rig, through: boolean): { d: any; sx: number; sy: number } | null {
  for (const d of R.game.doors.list) { if (!standsAs(d, through)) continue;
    for (const p of R.spots(d)) { R.stand(p); if (!FIND_BREAK && !standsAs(d, through)) break;
      for (const q of R.samples(d)) { const L = R.look(d, q); if (L && L.drawn && L.through === through && (through || L.own)) { if (FIND_BREAK && !through) { standOpen(d); R.step(4); } return { d, sx: L.sx, sy: L.sy }; } } } }
  return null;
}

