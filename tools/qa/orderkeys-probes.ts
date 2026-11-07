// The 'orderkeys' family: EVERY ORDER IS WITHIN REACH OF THE LEFT HAND — no row of any order menu, in any scenario on the start card's list, is on a digit above
// ORDER_DIGITS (5) (the maintainer, 10-04: "it's hard to hit numbers bigger than 5 when left hand is on wasd"). Run through probes.ts:
//   bun run tools/qa/probes.ts orderkeys | orderkeys-walk | orderkeys-give | orderkeys-four | orderkeys-high | orderkeys-table
// PASS / FAIL each, all headless (the Game of headless.ts stood up on each row's own level; nothing is drawn — the menu is read as the data the panel is built
// from). THE SCENARIOS are scene/scenarioList.ts SCENARIOS, row by row, each started the way a click on its row starts it (game/scenarioRun.ts startFromQuery
// on scenarioQuery(entry), on its own level file), plus the one hand-typed combination that stages a team the list does not (?nightbreach&squad4).
//   orderkeys-walk    in each scenario: X pressed on a spot of floor, on Sam, on a teammate and on a door, and the menu opened on EVERY door of the level;
//                     every page of every one of those menus is walked BY ITS KEYS (the row that opens a page pressed, Esc back, a right click back, X to
//                     close) and every row read twice — as the order table has it (OrderDef.key / hotKey) and as the panel shows it (hud().menu). A row FAILS
//                     when its digit is above ORDER_DIGITS, when it has no key at all, when its key is not one of the left hand's (X T B F, 1 to 5), when two
//                     rows of a page share a digit, or when the panel shows another key than the one that gives it. The hint line may name no digit.
//   orderkeys-give    in each scenario: every row of every page, on each kind of target, GIVEN by the keys the walk read — X, then the digits — on a fresh
//                     staging: the order is logged as that order and the menu is gone (a row greyed there, with its reason, must be refused and log
//                     nothing: it is counted apart). A FREE row (the silent word, a throw: it takes nobody off his order and give() logs no order for
//                     it) is given when its own deed ran once and did not refuse. Only X and the digits 1 to ORDER_DIGITS are ever pressed.
//   orderkeys-four    the four's two rows that are not orders, from the left hand: 'wait for my word' flipped by its digit on the tools page, by F there,
//                     and by F from the door's first page (which turns to the tools with it on); the word given by F with a floor menu up and with none
//   orderkeys-high    the digits above ORDER_DIGITS (and 0) pressed with each page of each menu up: nothing given, the menu as it was, Sam's slot as it was
//                     (the threshold's page where the tools page turns to one; where 'clear from outside' is a plain row on 4, that is what is held)
//   orderkeys-table   the table itself: ORDER_DIGITS is 5, the keys the orders read are X T B F and nothing else, register() never deals a digit above it
//                     (a page of twelve rows: five digits, the rest on the wheel), and no line of the order path names a digit key above it
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { LETTERS, leftKey as leftKeyOf, toolsRow4 } from './order-keys.ts';
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { RESCUE } = await import(`${ROOT}/src/game/rescue.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, ORDER_DIGITS, WORD_HEAD } = await import(`${ROOT}/src/game/orders.ts`);
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
const DEFAULTS = { DT: clone(DOORTOOLS), RC: clone(ROOMCLEAR), NB: clone(NIGHTBREACH), RS: clone(RESCUE), ORD: clone(ORDERS) };
const reset = () => { Object.assign(DOORTOOLS, clone(DEFAULTS.DT)); Object.assign(ROOMCLEAR, clone(DEFAULTS.RC)); Object.assign(NIGHTBREACH, clone(DEFAULTS.NB)); Object.assign(RESCUE, clone(DEFAULTS.RS)); Object.assign(ORDERS, clone(DEFAULTS.ORD)); };
/** every probe leaves the knob tables as it found them, and one that throws is a FAIL line, not a crash of the runner */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const quiet = <T>(fn: () => T): T => { if (VERBOSE) return fn(); const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const levelOfFile = (file: string): any => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); };

/** the scenarios walked: the list's rows, and the one team the list does not stage */
interface Scn { name: string; query: string; level?: string; squad: 2 | 4 }
const LIST: Scn[] = [
  ...SCENARIOS.map((e: any) => ({ name: e.title as string, query: scenarioQuery(e) as string, level: e.level as string | undefined, squad: e.squad as 2 | 4 })),
  { name: 'Night breach with the four (typed, not on the list)', query: '?nightbreach&squad4', squad: 4 },
];
/** a key of the left hand (order-keys.ts): its own letters, and the digits an order reaches off W A S D */
const leftKey = (k: string) => leftKeyOf(k, ORDER_DIGITS);
/** ORDERKEYS_BREAK, the family's red paths, one a value so that each shows its own lines can fail: `1` = one tool put on a digit out of reach (rig: the walk,
 *  the table, the digits pressed); `free` = every free row's deed refuses (orderkeys-give: each such row must read WRONG; the old stand has none on a digit, so
 *  it bites on the page's stand alone); `four` = the door tools' fourth row
 *  put on 6 (orderkeys-high: its page under the tools page, or the plain row on 4, is not there to hold); `man` = the walk asks its teammate's menu of
 *  another man than the one the cursor is on (orderkeys-walk: every level's teammate line must read WRONG) */
const BREAK = process.env.ORDERKEYS_BREAK ?? '';
const nameOf = (code: string) => code.replace(/^Key|^Digit/, '');

/** One game on the scenario's own level with the scenario started as its row starts it. step() keeps Sam dark and the defenders dazzled (this family is about
 *  the keys, not the fight) and writes down every key code the probe presses. */
async function rig(scn: Scn, seed = 21) {
  reset(); Math.random = seeded(0x0b5e + seed * 7919);
  let W: any; let why: string | null = null;
  { const log = console.log, err = console.error, warn = console.warn; if (!VERBOSE) console.log = console.error = console.warn = () => { };
    try { W = await standUp({ uploadEvery: 4, ...(scn.level ? { level: levelOfFile(scn.level) } : {}) }); W.game.godMode = true; why = startFromQuery(W.game, new URLSearchParams(scn.query)).why; }
    finally { console.log = log; console.error = err; console.warn = warn; } }
  const game = W.game, input = W.input, O = game.orders, D = game.doorTools;
  const R = { W, game, input, O, D, scn, why,
    /** every key code this rig pressed, in order (the proof that nothing above ORDER_DIGITS was needed) */
    pressed: [] as string[],
    aim: null as number[] | (() => number[]) | null,
    get ops(): any[] { return O.squad(); },
    hostiles(): any[] { return [...(game.nightBreach.hostiles ?? []), ...(game.rescue.hostiles ?? [])]; },
    step(n = 1) { quiet(() => { for (let i = 0; i < n; i++) {
      game.player.visibility = 0.05; for (const h of R.hostiles()) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
      if (R.aim) { const p = typeof R.aim === 'function' ? R.aim() : R.aim; W.cursorAt([p[0], p[1], p[2]]); }
      W.step(DT); } }); },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** a one-frame press, as the page's keydown writes it */
    press(code: string) { R.pressed.push(code); input.pressed.add(code); R.step(); },
    rclick() { input.clicked |= 4; R.step(); },
    last(): any { return O.log[O.log.length - 1] ?? null; },
    /** the scenario's own restart, Sam where it puts him */
    fresh() { quiet(() => game.restartEncounter()); R.aim = null; R.step(20); },
    /** Sam put on a free spot ~1.5 m off a door (either face, the first with floor), the cursor on the door: X then opens on it. False: no floor by it */
    atDoor(d: any): boolean {
      const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th), c = d.frameCentre;
      for (const off of [1.5, -1.5, 1.2, -1.2, 2.0, -2.0]) { const p = [c[0] + nx * off, 0, c[2] + nz * off]; if (game.col.nav.isBlocked(p[0], p[2])) continue;
        quiet(() => game.teleportPlayer(p)); R.aim = [c[0], 1.0, c[2]]; R.step(20);
        if (O.targetUnder(input, W.cam, W.canvas).door === d) return true; }
      return false;
    },
    /** the cursor on a spot of floor near Sam that the orders take for floor (null: none found) */
    onFloor(): number[] | null {
      const s = game.player.char.pos;
      for (const r of [1.8, 2.4, 1.4, 3.0]) for (let k = 0; k < 8; k++) { const p = [s[0] + Math.sin(k * Math.PI / 4) * r, 0.03, s[2] + Math.cos(k * Math.PI / 4) * r];
        if (game.col.nav.isBlocked(p[0], p[2])) continue; R.aim = p; R.step(2); if (O.targetUnder(input, W.cam, W.canvas).kind === 'floor') return p; }
      return null;
    },
    onSam() { R.aim = () => { const p = game.player.char.pos; return [p[0], 0.05, p[2]]; }; R.step(2); },
    /** the cursor put on the first teammate's chest; returns THE MAN IT IS ON THERE, by this file's own reckoning and not the orders': the nearest of
     *  the men's bodies along the cursor's ray (null: the ray meets none). A man stood between the camera and the first man covers his chest on the
     *  screen, and the cursor is on him. */
    onMan(): any | null {
      R.aim = () => { const m = R.ops[0].char.pos; return [m[0], 1.2, m[2]]; }; R.step(2);
      const ray = W.cam.ray(input.mouseX / Math.max(1, W.canvas.clientWidth), input.mouseY / Math.max(1, W.canvas.clientHeight));
      let man: any = null, best = Infinity; for (const o of R.ops) { if (o.state === 'dead') continue; const t = o.char.rayHit(ray.ro, ray.rd, 200); if (t >= 0 && t < best) { best = t; man = o; } }
      return man;
    },
    /** X pressed on what the cursor is on, then a few frames with every key up: the menu (or null) */
    openX(wait = 6): any { R.press(ORDERS.key); R.step(wait); return O.menu; },
  };
  // the family's own red paths (ORDERKEYS_BREAK=1 bun run tools/qa/probes.ts orderkeys-walk must FAIL): one tool put back on a digit out of reach; `four`: row 4 of the door tools on 6
  if (BREAK === '1') { const d = O.defs.find((x: any) => x.id === 'bangClear'); if (d) d.key = 7; }
  if (BREAK === 'four') { const d = O.defs.find((x: any) => x.id === toolsRow4(DOORTOOLS)); if (d) d.key = 6; }
  R.step(30);
  return R;
}
type Rig = Awaited<ReturnType<typeof rig>>;

/** A page of an open menu, read both ways: the order table's rows and the panel's. */
interface RowRead { id: string; digit: number | null; hot: string | null; opens: string | null; why: string | null; shown: string }
interface PageRead { page: string | null; rows: RowRead[]; wait: { shown: string; digit: number } | null; hint: string; bad: string[]; /** the keys that turn the menu to it (walkMenu's): a digit, or the code of an opener's own letter */ via?: (number | string)[] }
function readPage(O: any): PageRead {
  const M = O.menu, hud = O.hud().menu; const bad: string[] = [];
  // the panel: [who row, …one row an order…, (the wait row), the cancel row]
  const panel = (hud?.rows ?? []).slice(1, 1 + M.rows.length);
  const rows: RowRead[] = M.rows.map((d: any, i: number) => ({ id: d.id, digit: d.key ?? null, hot: d.hotKey ? nameOf(d.hotKey) : null, opens: d.opens ?? null, why: M.why[i] ?? null, shown: String(panel[i]?.opts?.[0]?.key ?? '') }));
  const where = `page ${M.page ?? 'first'}`;
  const digits = rows.filter(r => r.digit !== null).map(r => r.digit as number);
  for (const r of rows) {
    if (r.digit !== null && !(Number.isInteger(r.digit) && r.digit >= 1 && r.digit <= ORDER_DIGITS)) bad.push(`${where}: '${r.id}' is on digit ${r.digit}`);
    if (r.digit === null && !r.hot) bad.push(`${where}: '${r.id}' has no key at all`);
    const want = r.digit !== null ? String(r.digit) : r.hot ?? '';
    if (r.shown !== want) bad.push(`${where}: '${r.id}' is given by ${want || 'nothing'} but the panel shows "${r.shown}"`);
    if (!leftKey(r.shown)) bad.push(`${where}: '${r.id}' shows "${r.shown}", not a key of the left hand`);
  }
  if (new Set(digits).size !== digits.length) bad.push(`${where}: two rows share a digit (${digits.join(' ')})`);
  if (digits.length > ORDER_DIGITS) bad.push(`${where}: ${digits.length} rows on digits`);
  let wait: PageRead['wait'] = null;
  if (M.wordRow) { const w = hud?.rows?.[1 + M.rows.length], shown = String(w?.opts?.[0]?.key ?? ''); wait = { shown, digit: /^[1-9]$/.test(shown) ? Number(shown) : 0 };
    if (w?.label !== WORD_HEAD || !/wait for my word/.test(w?.opts?.[0]?.text ?? '')) bad.push(`${where}: the wait row is not where the panel has it`);
    if (!leftKey(shown)) bad.push(`${where}: the wait row shows "${shown}"`);
    if (wait.digit && digits.includes(wait.digit)) bad.push(`${where}: the wait row shares digit ${wait.digit} with an order`); }
  const hint = String(hud?.hint ?? ''); if (/\d/.test(hint)) bad.push(`${where}: the hint line names a digit ("${hint}")`);
  for (const r of hud?.rows ?? []) for (const o of r.opts ?? []) if (/^\d+$/.test(o.key) && !(Number(o.key) >= 1 && Number(o.key) <= ORDER_DIGITS)) bad.push(`${where}: the panel shows key "${o.key}" (${o.text})`);
  return { page: M.page ?? null, rows, wait, hint, bad };
}

/** The open menu walked by its keys: its first page read, each row that opens a page pressed (the page read — and the pages behind ITS openers, the same way;
 *  Esc back a page; pressed again; a right click back). Returns the pages, each with the digits that turn to it (`via`), and what went wrong on the way. The menu
 *  is left up on its first page. */
const keyCode = (k: number | string) => typeof k === 'number' ? `Digit${k}` : k;
function walkMenu(R: Rig): { pages: PageRead[]; bad: string[] } {
  const O = R.O, M = O.menu; const bad: string[] = [], pages: PageRead[] = [];
  /** the page now up, read, then every page behind it; false = the menu went away under the walk */
  const walk = (via: (number | string)[]): boolean => {
    const here = readPage(O); here.via = via; pages.push(here);
    const name = (p: string | null) => p === null ? 'the first page' : `'${p}'`;
    for (const r of here.rows.filter(x => x.opens)) {
      // (an opener sits on a digit, or on a letter of its own — the squad's throws are on G, where the first page's five digits are taken)
      if (r.digit === null && !r.hot) { bad.push(`'${r.id}' opens a page but has no key`); continue; }
      const k: number | string = r.digit ?? `Key${r.hot}`, code = keyCode(k);
      R.press(code); R.step(3);
      if (O.menu !== M || M.page !== r.opens) { bad.push(`${r.digit ?? r.hot} did not turn the menu to '${r.opens}'`); if (!O.menu) return false; continue; }
      if (!walk([...via, k])) return false;
      R.press('Escape'); R.step(2); if (O.menu !== M || M.page !== here.page) { bad.push(`Esc on '${r.opens}' did not step back to ${name(here.page)}`); if (!O.menu) return false; }
      R.press(code); R.step(2); R.rclick(); R.step(2); if (O.menu !== M || M.page !== here.page) { bad.push(`a right click on '${r.opens}' did not step back to ${name(here.page)}`); if (!O.menu) return false; }
    }
    return true;
  };
  walk([]);
  for (const p of pages) bad.push(...p.bad);
  return { pages, bad };
}

// ---------------------------------------------------------------- every menu of every scenario, walked

// ---------------------------------------------------------------- every row given by the keys the walk read

// ---------------------------------------------------------------- the four: the wait row and the word, from the left hand

// ---------------------------------------------------------------- the digits above the reach do nothing
export const orderkeysHigh = guarded(async () => {
  const { verdict, done } = family(`orderkeys-high: the digits above ${ORDER_DIGITS} (and 0) give nothing, with any page of any menu up or with none`);
  const HIGH = ['Digit0', ...Array.from({ length: 9 - ORDER_DIGITS }, (_, i) => `Digit${ORDER_DIGITS + 1 + i}`)];
  for (const scn of [LIST.find(s => s.squad === 2 && !s.level)!, LIST.find(s => s.squad === 4 && !s.level)!]) {
    const R = await rig(scn); const { O, D, game: g, input } = R; if (R.why !== null || !O.active) { verdict(false, `${scn.name}: staged`, String(R.why)); continue; }
    const s0 = [...g.player.char.pos]; let door: any = null; for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s0) - dist(b.frameCentre, s0))) { if (R.atDoor(d)) { door = d; break; } }
    const out: string[] = []; let ok = !!door; const l0 = O.log.length, slot0 = g.player.slot, ops0 = D.ops.length;
    const hit = (what: string) => { const M = O.menu, page = M?.page ?? null, sel = M?.sel, onWord = M?.onWord, rows = M?.rows.length;
      for (const c of HIGH) { input.pressed.add(c); R.step(); }
      const same = O.menu === M && (!M || (M.page === page && M.sel === sel && M.onWord === onWord && M.rows.length === rows)), quietLog = O.log.length === l0 && D.ops.length === ops0 && g.player.slot === slot0;
      ok &&= same && quietLog; out.push(`${what}: ${same ? 'as it was' : 'CHANGED'}${quietLog ? '' : ' · SOMETHING WAS GIVEN'}`); };
    R.aim = null; R.step(2); hit('no menu');
    R.onFloor(); R.openX(); hit('floor menu'); R.press(ORDERS.key);
    R.onSam(); R.openX(); hit('Sam menu'); R.press(ORDERS.key);
    if (door) { R.atDoor(door); const M = R.openX(); hit('door menu, first page'); const k = M?.rows.find((r: any) => r.opens)?.key; R.press(`Digit${k}`); ok &&= !!O.menu && O.menu.page !== null; hit('door menu, tools page');
      // (…and the page under the tools page, where the menu has one: the threshold's. Where 'clear from outside' is a plain row of the tools page
      //  — doortools.ts DOORTOOLS.outsidePlain — there is no page to turn to, and the line says so instead of pressing a key that opens nothing)
      const k2 = O.menu?.rows.find((r: any) => r.opens)?.key;
      if (k2 !== undefined) { R.press(`Digit${k2}`); ok &&= !!O.menu && O.menu.page !== null && O.menu.page !== 'tools'; hit('door menu, threshold page'); }
      else { const M2 = O.menu, four = M2?.rows[3], plain = !!M2 && M2.page === 'tools' && DOORTOOLS.outsidePlain === true && four?.id === 'thresholdHold' && String(four.key) === '4' && !M2.rows.some((r: any) => r.opens);
        ok &&= plain; out.push(plain ? 'no page under the tools page (outside is a plain row there, on 4)' : !M2 ? 'THE MENU WAS GONE FROM THE TOOLS PAGE' : `NO PAGE UNDER THE TOOLS PAGE (page ${M2.page}, rows ${M2.rows.map((r: any) => `${r.key} ${r.id}`).join(' · ')})`); }
      R.press(ORDERS.key); }
    verdict(ok && O.menu === null, `${scn.name} (${scn.query}): ${HIGH.map(nameOf).join(' ')} pressed with each menu page up and with none — no order, no page turned, Sam's slot as it was`, `${out.join(' · ')} · orders logged ${O.log.length - l0} · slot ${slot0} → ${g.player.slot}`);
  }
  return done();
});

// ---------------------------------------------------------------- the table and the code
export const orderkeysTable = guarded(async () => {
  const { verdict, done } = family('orderkeys-table: the order table deals no digit above the reach, and the keys the orders read are the left hand\'s');
  const keys: [string, string][] = [[ORDERS.key, 'the menu'], [ORDERS.whoKey, 'who'], [ORDERS.quickKey, 'the quick order'], [ORDERS.waitKey, 'wait for my word'], [ORDERS.waitAlt, 'wait for my word (second key)'], [DOORTOOLS.blowKey, 'the word / the trigger']];
  const okKeys = keys.every(([k]) => leftKey(nameOf(k)));
  verdict(ORDER_DIGITS === 5 && okKeys, `ORDER_DIGITS is 5, and every key an order is read from is one of ${LETTERS.join(' ')} or 1 to 5`, keys.map(([k, v]) => `${nameOf(k)} ${v}`).join(' · '));
  { // every row the game registers, for the pair and for the four
    const out: string[] = []; let ok = true;
    for (const scn of [LIST.find(s => s.squad === 2 && !s.level)!, LIST.find(s => s.squad === 4 && !s.level)!]) {
      const R = await rig(scn); const defs: any[] = R.O.defs;
      const bad = defs.filter(d => !(d.key === undefined ? !!d.hotKey : Number.isInteger(d.key) && d.key >= 1 && d.key <= ORDER_DIGITS));
      ok &&= bad.length === 0 && defs.length > 0; out.push(`${scn.squad === 4 ? 'four' : 'pair'}: ${defs.map(d => `${d.page ? `${d.page}/` : ''}${d.id}=${d.key ?? nameOf(d.hotKey ?? '—')}`).join(' ')}${bad.length ? ` · BAD: ${bad.map(d => d.id).join(' ')}` : ''}`);
      if (scn.squad === 4) {   // the word fits a door target (the console's give('go') with the reticle on one) though no door menu lists it
        const d0 = R.game.doors.list[0], dt = { kind: 'door', at: [d0.frameCentre[0], 0, d0.frameCentre[2]], door: d0, side: 1 }, listed = R.O.rowsFor(dt).some((d: any) => d.id === 'go'), why = String(R.O.give('go', dt));
        ok &&= !listed && !/does not fit|no such/.test(why); out.push(`four, 'go' on a door target: ${listed ? 'LISTED on its menu' : 'not on its menu'} · give() answers "${why}"`); }
    }
    verdict(ok, 'every row the game registers is on a digit from 1 to 5 or on a key of its own', out.join('\n    '));
  }
  { // register() itself: more rows than digits, on the first page and on a page — never a digit above the reach, never two rows of a page on one digit
    const R = await rig(LIST.find(s => s.squad === 2 && !s.level)!); const O = R.O; const ids: string[] = [];
    for (let i = 0; i < 4; i++) { const id = `t-first-${i}`; ids.push(id); O.register({ id, label: id, targets: ['floor'], issue: () => { } }); }
    for (let i = 0; i < 12; i++) { const id = `t-page-${i}`; ids.push(id); O.register({ id, page: 'tools', label: id, targets: ['door'], issue: () => { } }); }
    O.register({ id: 't-nine', key: 9, label: 't-nine', targets: ['none'], issue: () => { } }); ids.push('t-nine');
    O.register({ id: 't-nine-own', key: 9, page: 't-own', label: 't-nine-own', targets: ['none'], issue: () => { } }); ids.push('t-nine-own');   // (…and one that asks for 9 on a page of its own, where every digit is free)
    const defs: any[] = O.defs, over = defs.filter(d => d.key !== undefined && !(d.key >= 1 && d.key <= ORDER_DIGITS));
    const dup = (rows: any[]) => { const k = rows.filter(d => d.key !== undefined).map(d => d.key); return new Set(k).size !== k.length; };
    const floor = O.rowsFor({ kind: 'floor', at: [0, 0, 0] }), tools = O.rowsFor({ kind: 'door', at: [0, 0, 0], door: R.game.doors.list[0], side: 1 }, 'tools'), nine = defs.find(d => d.id === 't-nine');
    const keysOf = (rows: any[]) => rows.map(d => `${d.id}=${d.key ?? '·'}`).join(' ');
    // (a row that asks for 9 is dealt a digit in reach where one is free: 5 on the first page, or none where the menu of its target has a row of the game's on 5
    //  already (asked of the menu itself); 1 on a page of its own — orders.ts register)
    const fiveTaken = O.rowsFor({ kind: 'none', at: [0, 0, 0] }).some((d: any) => d.id !== 't-nine' && d.key === 5), nineOwn = defs.find(d => d.id === 't-nine-own');
    for (const id of ids) O.unregister(id);
    verdict(over.length === 0 && !dup(floor) && !dup(tools) && tools.filter((d: any) => d.key !== undefined).length === ORDER_DIGITS && floor.filter((d: any) => d.key !== undefined).length === ORDER_DIGITS && !!nine && nine.key === (fiveTaken ? undefined : 5) && nineOwn?.key === 1,
      'register() with more rows than digits (four more on the floor\'s page, twelve more on the tools page, two that ask for 9): no digit above 5 is dealt, no two rows of a page share one, a row that asks for 9 gets a digit in reach where one is free, the rest are left to the wheel',
      `floor: ${keysOf(floor)}\n    tools: ${keysOf(tools)}\n    asked for 9 on Sam's menu: got ${nine ? nine.key ?? 'none' : 'NO ROW'} (5 is ${fiveTaken ? 'a row of the game\'s there: none is right' : 'free there'}) · asked for 9 on a page of its own: got ${nineOwn?.key ?? 'none'} (want 1)`);
  }
  { // the code: the order path names no digit key above the reach (a literal, or a loop bound it would have to read)
    const PATH = ['src/game/orders.ts', 'src/game/doortools.ts', 'src/game/nightbreach.ts', 'src/game/roomclear.ts', 'src/game/rescue.ts', 'src/game/rescueflow.ts'];
    const hits: string[] = [];
    for (const f of PATH) readFileSync(resolve(ROOT, f), 'utf8').split('\n').forEach((ln, i) => { const code = ln.replace(/\/\/.*$/, ''); if (/['"`]Digit[06-9]['"`]/.test(code)) hits.push(`${f}:${i + 1}`); });
    verdict(hits.length === 0, `no line of the ${PATH.length} files on the order path names a digit key above 5, or 0`, `literals 'Digit0' / 'Digit6' … 'Digit9' in code: ${hits.length}${hits.length ? ` — ${hits.join(', ')}` : ''}`);
  }
  return done();
});
