// THE DOOR MENU'S RIG, GIVEN BY KEYS (it was the 'thresholdmenu' family's file, and keeps the name its users import it by): every start-card scenario
// (scenarioList.ts) staged as the page stages it, a spot by a door of a held room found for Sam, X with the cursor on the door and one-frame presses of the
// digits and F, as the page's keydown writes them. calls-probes.ts, doortest-check.ts, openhand-check.ts, thrpie-check.ts and told-check.ts stand on it, and
// fifteen more checks take its `guarded` and `family`. The harness has no picture: nothing here says how a panel LOOKS.
// THE FAMILY ITSELF IS GONE (2026-11-06): thresholdmenu-rows and thresholdmenu-held walked the threshold's page behind X, 5, 4 (hold the door until I say ·
// step in once it goes quiet · clear from outside) and the held page of what next. The page has had neither since the squad game build of 10-16
// (b6c0aff1, where the rule enters this history already on): 'clear from outside' is one plain order on the tools' row 4 (DOORTOOLS.outsidePlain, by the maintainer's words quoted at it in
// src/game/doortools.ts), and both probes asked for the pages as they were to have anything to walk. Row 4 as each stand has it is
// orderkeys-high's, doormenu-rows' and doortools-menu's; the order behind it is threshold-probes.ts's and doortest-check.ts's. The calls that lived on the
// old page (calls-menu, calls-leave, calls-held, calls-rally) went 2026-11-07 with `pagesAsTheyWere`: both rows it wrote are settled.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { DOORTOOLS, TOOLS_PAGE } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, WORD_HEAD } = await import(`${ROOT}/src/game/orders.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { knobsNamed } = await import(`${ROOT}/src/game/knobs.ts`);
const feelNow = () => knobsNamed(['FLASH', 'PACE', 'LIGHTS', 'NVG', 'IR', 'IRLIGHT', 'FOG', 'LASER', 'DOORTOOLS', 'ROOMFIGHT', 'ORDERS', 'COVER', 'ENTRY', 'REAR', 'SILENT', 'FRAG', 'PAIR', 'DOORMANNERS']) as any;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
/** every probe leaves the feel tables (DOORTOOLS among them) as it found them, and one that throws is a FAIL line, not a crash of the runner */
const guarded = (fn: () => Promise<boolean>) => async () => { const T = feelNow(), keep = JSON.parse(JSON.stringify(T));
  try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { for (const n of Object.keys(keep)) Object.assign(T[n], keep[n]); } };
async function quiet<T>(fn: () => T | Promise<T>): Promise<T> { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } }
const levelOfFile = (file: string): any => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); };
const X = ORDERS.key as string;

/** a row of the open menu as the panel shows it */
interface Row { id: string; key: string; text: string; off: boolean; why: string | null; opens: string | null }
interface Page { page: string | null; rows: Row[]; wait: { key: string; text: string } | null }
const rowsOf = (p: Page | null) => p ? p.rows.map(r => `${r.key} ${r.id}${r.opens ? ' ▸' : ''}`).join(' · ') : '(no menu)';
const textOf = (p: Page | null) => p ? p.rows.map(r => `${r.key} ${r.text}${r.off ? ' (off)' : ''}`).join(' | ') + (p.wait ? ` | ${p.wait.key} ${p.wait.text}` : '') : '(no menu)';

/** a start-card scenario staged as the page stages it, under a seed (threshold-probes.ts's recipe: the page's chaos, Sam in god mode and pinned dark), with
 *  the keys of the page: the cursor held on a world point, one-frame presses */
async function stage(E: any, seed: number) {
  Math.random = seeded(0x9137 + seed * 7919);
  const W: any = await quiet(() => standUp({ uploadEvery: 4, ...(E.level ? { level: levelOfFile(E.level) } : {}) })); const g = W.game;
  g.godMode = true; g.tune.chaos = 0.5;
  let q = scenarioQuery(E) as string; if (E.kind === 'roomclear') q = q.replace(/roomclear(=\d+)?/, `roomclear=${seed}`);
  const { why } = await quiet(() => startFromQuery(g, new URLSearchParams(q))); if (why) throw new Error(`${E.id}: start refused — ${why}`);
  const O = g.orders, D = g.doorTools, input = W.input;
  const R = { E, W, g, O, D, input, aim: null as number[] | null, pressed: [] as string[],
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; if (R.aim) W.cursorAt([R.aim[0], R.aim[1], R.aim[2]]); W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** a one-frame press, as the page's keydown writes it */
    press(code: string) { R.pressed.push(code); input.pressed.add(code); R.step(); },
    digit(n: number) { R.press(`Digit${n}`); R.step(2); },
    /** X on what the cursor is on, then a few frames with every key up */
    openX(): any { if (O.menu) O.close(); R.press(X); R.step(6); return O.menu; },
    /** the open menu's page, read off the order table and off the panel (rows[0] of the panel = who the orders go to) */
    read(): Page | null { const M = O.menu; if (!M) return null; const hud = O.hud().menu, body = (hud?.rows ?? []).slice(1, 1 + M.rows.length);
      const rows: Row[] = M.rows.map((d: any, i: number) => ({ id: d.id, key: String(body[i]?.opts?.[0]?.key ?? ''), text: String(body[i]?.opts?.[0]?.text ?? ''), off: !!body[i]?.off, why: M.why[i] ?? null, opens: d.opens ?? null }));
      const w = M.wordRow ? hud?.rows?.[1 + M.rows.length] : null;
      return { page: M.page ?? null, rows, wait: w && w.label === WORD_HEAD ? { key: String(w.opts?.[0]?.key ?? ''), text: String(w.opts?.[0]?.text ?? '') } : null }; },
    /** Sam put at p (a ghost: he is put there), the cursor on door d: true when X would open on that door, on face `side` of it */
    place(p: number[], d: any, side?: number): boolean {
      g.teleportPlayer([p[0], 0, p[2]]); R.aim = [d.frameCentre[0], 1.0, d.frameCentre[2]]; R.step(20);
      const t = O.targetUnder(input, W.cam, W.canvas); return t.kind === 'door' && t.door === d && (side === undefined || t.side === side); },
    last(): any { return O.log[O.log.length - 1] ?? null; },
    highest(): number { return Math.max(0, ...R.pressed.filter(c => /^Digit/.test(c)).map(c => Number(c.slice(5)))); },
  };
  await quiet(() => R.step(20));
  return R;
}
type Rig = Awaited<ReturnType<typeof stage>>;

/** where the order is given in a scenario: a door, the face of it the squad works from, and a spot for the commander off the opening's line on that face */
interface Spot { door: string; sam: number[]; held: boolean }
/** The door this scenario's rows are given on: the nearest entry of a room that still holds a living defender (the room fight's own model) — else the nearest
 *  door at all — that X on the cursor opens on from a spot off its line, and where the door tools' 'clear from outside' is live (it asked the threshold's own
 *  page for its two forms until 2026-11-07: the menu has one plain order on the tools' row 4). Found once a scenario (the search steps
 *  the game: every run after it is staged afresh and goes straight to the spot). */
const SPOTS = new Map<string, Spot | string>();
async function spotOf(E: any): Promise<Spot | string> { if (!SPOTS.has(E.id)) SPOTS.set(E.id, await findSpot(await stage(E, 1))); return SPOTS.get(E.id)!; }
async function findSpot(R: Rig): Promise<Spot | string> {
  const g = R.g, mid = R.team[0]?.char.pos ?? g.player.char.pos, why: string[] = [];
  const cands: { d: any; out: number[] | null; at: number[]; held: boolean }[] = [];
  const rooms = new Set<any>(); for (const def of g.fight.defs) if (def.room && def.room.entries.length && def.room.cells.size <= 500 && def.gd.state !== 'dead') rooms.add(def.room);
  for (const room of rooms) for (const ent of room.entries) cands.push({ d: ent.door, out: ent.out, at: ent.at, held: true });
  cands.sort((a, b) => flat(a.at, mid) - flat(b.at, mid));
  for (const d of [...g.doors.list].sort((a: any, b: any) => flat(a.frameCentre, mid) - flat(b.frameCentre, mid))) if (!cands.some(c => c.d === d)) cands.push({ d, out: null, at: d.frameCentre, held: false });
  for (const c of cands) {
    const th = c.d.def.closedDir, n = [-Math.sin(th), Math.cos(th)], faces = c.out ? [c.out] : [n, [-n[0], -n[1]]];
    for (const out of faces) { const lx = -out[1], lz = out[0];
      for (const [back, lat] of [[3.4, 1.6], [3.4, -1.6], [4.4, 1.2], [4.4, -1.2], [2.8, 2.2], [2.8, -2.2], [2.6, 0], [1.8, 0]]) {
        const p = [c.at[0] + out[0] * back + lx * lat, 0, c.at[2] + out[1] * back + lz * lat];
        if (g.col.nav.isBlocked(p[0], p[2]) || g.col.solidBlocked([p[0], 1.1, p[2]], [c.at[0] + out[0] * 0.7, 1.1, c.at[2] + out[1] * 0.7])) continue;
        const ok = await quiet(() => R.place(p, c.d)); if (!ok) continue;
        const M = await quiet(() => { const m = R.openX(); if (m) R.digit(5); return R.O.menu; });
        const pg = M && M.target.door === c.d && M.page === TOOLS_PAGE ? R.read() : null; if (R.O.menu) R.O.close();
        const forms = pg?.rows.filter(r => r.id === 'thresholdHold') ?? [];
        if (forms.length === 1 && !forms[0].why) return { door: c.d.def.name, sam: p, held: c.held };
        if (forms.length) why.push(`${c.d.def.name}: ${forms.map(r => r.why).find(Boolean)}`);
      } } }
  return `no door found where X opens on it and 'clear from outside' is live${why.length ? ` (${[...new Set(why)].slice(0, 4).join(' · ')})` : ''}`;
}
/** a scenario staged afresh with the commander on the spot and the cursor on the door */
async function at(E: any, seed: number, spot: Spot) { const R = await stage(E, seed), d = R.g.doors.byName(spot.door); const on = await quiet(() => R.place(spot.sam, d)); return { R, d, on }; }
const opOn = (R: Rig, d: any) => R.D.ops.find((o: any) => o.kind === 'threshold' && o.door === d) ?? null;
const beatsOf = (op: any) => (op?.beats ?? []).map((b: any) => b.what as string);

const FIRST = 'stack cover fallIn hold tools';   // (a door's first page, by row id)

export { stage, at, spotOf, opOn, beatsOf, rowsOf, textOf, quiet, guarded, family, FIRST };
