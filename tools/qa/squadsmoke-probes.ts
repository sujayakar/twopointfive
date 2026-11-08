// The 'squadsmoke' family (game/squadsmoke.ts — teammates carry smoke canisters on their cards and throw one on the floor order "smoke there"; game/kit.ts
// a teammate's 'smoke' row, kitSmokes), run through probes.ts:
//   bun run tools/qa/probes.ts squadsmoke | squadsmoke-table | squadsmoke-issued | squadsmoke-order | squadsmoke-restart
// PASS / FAIL each, all headless: the whole Game over the fake device, the room clear on the office with its squad of four (or the pair). Nothing here
// looks at a picture, and NOTHING HERE SEES SMOKE: the solver is the renderer's, headless the air is clear — what is checked is the can (the item, its fuse,
// where it comes to rest, that it vents), the count and the menu.
//   squadsmoke-table    the card: a teammate has a 'Smoke canisters' row (0 to 2, none as issued); a hand-typed store is made safe
//   squadsmoke-issued   every card as issued: no row on any menu, no line on the HUD, H with the menu up is nobody's
//   squadsmoke-order    a card with two cans: the floor's menu lists 'smoke there · 2 left' on H under the floor's own four (their digits untouched, no
//                       digit above 5, nothing held); X then H makes the nearest man with a throw toss a 'smoke' item that comes to rest near the spot and
//                       vents; he keeps the order he was on and nobody else moves; the count drops; at 0 the row is greyed and a third order is refused; a
//                       spot nobody can reach is refused in the row's words; a silent squad stays silent; the throw is on the order log
//   squadsmoke-restart  a restart puts the count back and the row on the menu once; the cards back as issued (or the table off) and a restart takes the row
//                       and the HUD line away; the pair's count is its two men's
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const K: any = await import(`${ROOT}/src/game/kit.ts`);
/** slots.json as shipped (no squad limit since v7), and the same table with the squad limit it had through v6: the trade lines are held on that one */
const SHIPPED = JSON.parse(readFileSync(resolve(ROOT, 'src/game/slots.json'), 'utf8')), LIMIT7 = { ...SHIPPED, squad: { pouches: 7 } };
const SM: any = await import(`${ROOT}/src/game/squadsmoke.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { KIT, KIT_LS } = K, { SQUADSMOKE, SMOKE_ORDER } = SM;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DEFAULTS = { NIGHTBREACH: { ...NIGHTBREACH }, SILENT: JSON.parse(JSON.stringify(SLM.SILENT)), SQUADSMOKE: { ...SQUADSMOKE } };
/** every card back as issued, the table on, nothing kept anywhere; the knobs this family moves put back */
function issued() { K.kitLoad(null, ''); K.kitReset(); Object.assign(NIGHTBREACH, DEFAULTS.NIGHTBREACH); Object.assign(SLM.SILENT, JSON.parse(JSON.stringify(DEFAULTS.SILENT))); Object.assign(SQUADSMOKE, DEFAULTS.SQUADSMOKE); }
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { K.slotsSet(SHIPPED); issued(); } };
/** room in the squad's pouches for n more (under a table with a squad limit the issued squad fills them: smoke is a trade; under the shipped table, which
 *  sets none, the trade is not needed and is made all the same: the later probes stage the squad they always staged) — the frags first, then the door charges; nothing with the rule off */
function giveUp(n: number) { if (!K.SLOTS.on) return; for (const [i, f] of [[1, 'frags'], [2, 'frags'], [2, 'charges'], [1, 'charges']] as const) { if (n <= 0) break; if ((KIT.members as any[])[i][f] > 0 && K.kitSet(i, f, 0)) n--; } }
/** n smoke canisters onto teammate i's card, paid for (false = the card refused it) */
function bring(i: number, n: number): boolean { giveUp(n); return K.kitSet(i, 'smoke', n); }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const p2 = (p: number[]) => `(${f1(p[0])}, ${f1(p[2])})`;
function memStore(seed?: string) { const m = new Map<string, string>(); if (seed !== undefined) m.set(KIT_LS, seed); return { m, getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, v); } }; }

interface Row { head: string; key: string; text: string; off: boolean }
/** the room clear on the office, its squad stood up with the cards as they are NOW (`empty`: every hostile taken off the floor — nothing but the orders
 *  decides what follows) */
async function rig(seed: number, o: { squad?: 2 | 4; empty?: boolean; silent?: boolean } = {}) {
  Object.assign(NIGHTBREACH, { squad: o.squad ?? 4 });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, input = W.input;
  if (o.silent) Object.assign(SLM.SILENT, { on: true });
  const RC = game.roomClear, NB = game.nightBreach, O = game.orders;
  game.godMode = true;
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(t); msg(t); }; }
  const lines: { who: string; text: string }[] = []; { const say = game.say.bind(game); game.say = (gd: any, t: string, ...a: any[]) => { lines.push({ who: gd?.callsign ?? '?', text: t }); return say(gd, t, ...a); }; }
  const why = RC.start(seed);
  const R = { W, game, input, RC, NB, O, why, said, lines, held: 0,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; R.held = Math.max(R.held, input.keys.size + (input.buttons ? 1 : 0)); W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    cursor(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); },
    press(code: string) { input.pressed.add(code); R.step(); },
    /** X with the cursor on p: the menu's order rows as the panel draws them (the who row and the cancel row left out) — the menu left UP */
    menu(p: number[]): { kind: string | undefined; rows: Row[] } { R.cursor(p); R.press(ORDERS.key); R.step(2);
      const hud = O.hud().menu; return { kind: O.menu?.target.kind, rows: (hud?.rows ?? []).slice(1).filter((r: any) => r.kind !== 'cancel').map((r: any) => ({ head: r.label, key: r.opts[0]?.key, text: r.opts[0]?.text, off: !!r.off })) }; },
    esc() { for (let i = 0; i < 3 && O.menu; i++) R.press('Escape'); },
    /** the squad block's lines (the HUD's status rows' texts) */
    notes(): string[] { return (O.hud().status ?? []).flatMap((r: any) => r.opts.map((x: any) => x.text as string)); },
    cans(): any[] { return game.items.items.filter((x: any) => x.kind === 'smoke'); },
    restart() { const log = console.log; console.log = () => {}; try { game.restartEncounter(); } finally { console.log = log; } R.step(30); },
  };
  R.step(30);
  if (o.empty) { for (const h of game.guards) if (h.team === 'hostile' && h.state !== 'dead') killGuard(game, h, [0, 0, 1], true); R.step(60 * 4); }
  return R;
}
const smokeRow = (rows: Row[]) => rows.find(r => r.text.startsWith('smoke there')) ?? null;
const digitAbove5 = (rows: Row[]) => rows.some(r => /^[6-9]$|^0$/.test(r.key));
/** a spot of free floor `d` metres from a man that he has a throw to and that the menu, opened with the cursor on it, takes for that floor (not a door, a man,
 *  Sam, or the top of a wall the camera's line crosses first) — the first found on a ring */
function spotNear(R: any, gd: any, d: number): number[] | null {
  const p = gd.char.pos;
  for (let k = 0; k < 24; k++) { const a = k * Math.PI / 12, at = [p[0] + Math.sin(a) * d, 0, p[2] + Math.cos(a) * d], t = R.O.targetAt(at);
    if (t.kind !== 'floor' || dist(t.at, at) > 0.3 || !SM.squadSmokeThrower(R.game, [gd], t.at)) continue;
    R.cursor(t.at); const u = R.O.targetUnder(R.input, R.W.cam, R.W.canvas);   // (…and the cursor put on it finds it: no wall's top between the camera and the spot)
    if (u.kind === 'floor' && dist(u.at, t.at) < 0.3 && SM.squadSmokeThrower(R.game, [gd], u.at)) return t.at; }
  return null;
}

// ───────────────────────────────────────────────────────────── the card ────────────────────────────────
export const smokeTable = guarded(async () => {
  const F = family('squadsmoke-table: a teammate\'s card can carry smoke, none as issued, and the squad\'s count is the cards\'');
  issued();
  const M = KIT.members as any[], rowsYou = K.kitRows(0), rowsMate = K.kitRows(1), at = rowsMate.findIndex((x: any) => x.field === 'smoke'), r = rowsMate[at], mine = rowsYou.find((x: any) => x.field === 'smoke');
  F.verdict(K.SLOTS.on && !!r && r.label === 'Smoke canisters' && r.values.join(',') === '0,1,2' && K.SMOKE_MATE_MAX === 2 && M.slice(1).every(m => m.smoke === 0) && M[0].smoke === 1 && mine?.values.length === 5 && !M.some((_m, i) => K.kitChanged(i))
    && rowsMate[at - 1]?.field === 'bangs' && rowsMate[at + 1]?.field === 'charges' && [2, 3, 4].every(i => K.kitRows(i).some((x: any) => x.field === 'smoke')),
    'every teammate\'s card has a Smoke canisters row, 0 to 2, under his stun canisters; nobody on the squad is issued one; your own smoke row is your pouches\' (0 to 4, one issued: you carry what a teammate carries since slots.json v6 — it was 0 to 6, four issued)', `teammate rows: ${rowsMate.map((x: any) => x.label).join(', ')} · values ${r?.values.join('/')} · issued ${M.map(m => m.smoke).join(' ')}`);
  // the rule off (?noslots): the row is there all the same, free like the others
  K.kitLoad(null, '?noslots'); const free = K.kitSet(1, 'smoke', 2) && K.kitSet(4, 'smoke', 2), fp = K.kitPool(4), fs = K.kitSmokes(4); issued();
  F.verdict(free && fs === 4 && fp.bangs === 3 && fp.charges === 2 && fp.frags === 2, 'with the pouches rule off the row is a free one like the rest: smoke goes on with nothing given up', `set ${free} · smoke ${fs} · stun/charges/frags ${fp.bangs}/${fp.charges}/${fp.frags}`);
  // the store
  const st = memStore(); K.kitLoad(st, ''); bring(3, 1); const kept = st.m.get(KIT_LS) ?? ''; issued();
  K.kitLoad(memStore(kept), ''); const back = M[3].smoke === 1 && M[4].smoke === 0 && M[1].frags === 0;
  K.kitLoad(memStore(JSON.stringify({ v: 1, members: [{}, { smoke: 9, frags: 0 }, { smoke: -1, frags: 0 }, { smoke: '2' }, { smoke: 2 }] })), '');
  const safe = M[1].smoke === 0 && M[2].smoke === 0 && M[3].smoke === 0 && M[4].smoke === 2 && K.kitLegal() === null;
  K.kitLoad(memStore(JSON.stringify({ v: 1, members: [{}, { frags: 0 }, {}, {}, {}] })), ''); const old = M.slice(1).every(m => m.smoke === 0) && M[1].frags === 0 && M[0].smoke === 1;   // (yours: the one you are issued under the rule — it was four)
  F.verdict(back && safe && old, 'a smoke count is kept and read back; a hand-typed one out of the row\'s values comes back as none (one the row offers, with room made for it, is kept); a store written before the row existed reads as no smoke', `read back ${back} · made safe ${safe} · an older store ${old}`);
  return F.done();
});

// ───────────────────────────────────────────────────────────── as issued ───────────────────────────────
export const smokeIssued = guarded(async () => {
  const F = family('squadsmoke-issued: with every card as issued nothing of it is on the menu, on the HUD or in the run');
  issued();
  const R = await rig(11, { empty: true }); const { O, game } = R; const sam = game.player.char.pos, spot = [sam[0] + 1.8, 0, sam[2] + 0.4];
  const m = R.menu(spot), items0 = game.items.items.length; R.press(SQUADSMOKE.orderKey); R.step(30); const still = !!O.menu; R.esc();
  F.verdict(R.why === null && R.team.length === 4 && m.kind === 'floor' && !smokeRow(m.rows) && !O.defs.some((d: any) => d.id === SMOKE_ORDER) && SM.squadSmokeLeft(game) === 0 && SM.squadSmokeBrought(game) === 0 && !R.notes().some(t => /smoke/.test(t)) && game.items.items.length === items0 && still,
    'the room clear with four, cards as issued: the floor\'s menu has no smoke row, the squad block no smoke line, and H with the menu up is nobody\'s (the menu stays, nothing is thrown)',
    `start ${R.why} · floor menu: ${m.rows.map(r => `${r.key || '·'} ${r.text}`).join(' | ')} · ${SM.squadSmokeSummary(game)}`);
  return F.done();
});

// ───────────────────────────────────────────────────────────── the order ───────────────────────────────
export const smokeOrder = guarded(async () => {
  const F = family('squadsmoke-order: "smoke there" — a teammate throws a smoke canister at the spot, and it is counted');
  // (the squad as issued but for the trade `bring` makes below — the same two frags given up here: the floor's menu counts the squad's frags on its throw row
  //  where the squad's throws are on, the page's stand, and what is compared is the cans alone)
  issued(); giveUp(2); const base = await rig(7, { empty: true, silent: true }); const sam0 = base.game.player.char.pos, m0 = base.menu([sam0[0] + 1.8, 0, sam0[2] + 0.4]); base.esc();
  issued(); if (!bring(3, 2)) throw new Error('the card refused the smoke');
  const R = await rig(7, { empty: true, silent: true }); const { O, game } = R;
  const team = R.team, near = team.slice().sort((a: any, b: any) => dist(a.char.pos, game.player.char.pos) - dist(b.char.pos, game.player.char.pos));
  // a spot four metres from one of them that he has a throw to
  let thrower: any = null, spot: number[] | null = null;
  for (const gd of near) { const s = spotNear(R, gd, 4); if (s && SM.squadSmokeThrower(game, team, s)?.gd === gd) { thrower = gd; spot = s; break; } }
  if (!thrower || !spot) { F.verdict(false, 'a spot of floor four metres from a teammate, with a throw to it', `none found round ${near.map((g: any) => `${g.callsign} ${p2(g.char.pos)}`).join(' ')}`); return F.done(); }
  const block = R.notes();   // (the squad's block, read with the menu down: the panel is the menu's while it is up)
  const m = R.menu(spot), row = smokeRow(m.rows), plain = m.rows.filter(r => r !== row), headOfMove = m.rows.find(r => r.text === 'move there')?.head;
  const same = plain.length === m0.rows.length && plain.every((r, i) => r.key === m0.rows[i].key && r.text === m0.rows[i].text && r.off === m0.rows[i].off);
  F.verdict(R.why === null && SM.squadSmokeLeft(game) === 2 && m.kind === 'floor' && !!row && row.key === 'H' && row.text === 'smoke there · 2 left' && !row.off && row.head === headOfMove && m.rows.indexOf(row) === m.rows.findIndex(r => r.text === 'hold position') + 1 && same && !digitAbove5(m.rows) && block.includes('smoke 2'),
    'one teammate\'s card carries two: the floor\'s menu lists "smoke there · 2 left" on H, straight under hold position and under the same header; every other row keeps its key, its words and whether it is greyed (against the same squad with no can: the same two frags given up); no digit above 5; the squad block says what is left',
    `floor menu: ${m.rows.map(r => `${r.key || '·'} ${r.text}${r.off ? ' (off)' : ''}`).join(' | ')} · with no can it was: ${m0.rows.map(r => `${r.key || '·'} ${r.text}${r.off ? ' (off)' : ''}`).join(' | ')}`);
  // X is up: H gives it
  const asg0 = new Map(team.map((gd: any) => [gd, O.of(gd)])), log0 = O.log.length, pos0 = team.map((gd: any) => [...gd.char.pos]), items0 = R.cans().length, silent0 = (SLM.setSilent(game, true), SLM.squadSilent(game));
  R.press(SQUADSMOKE.orderKey);
  const atOnce = SM.squadSmokeThrower(game, [thrower], spot);   // (asked the frame after: the same man, the same spot)
  const can = R.cans()[0], who = can ? team.find((gd: any) => gd.char.id === can.who) : null, left1 = SM.squadSmokeLeft(game), menuGone = !O.menu, said = R.lines.some(l => l.who === thrower.callsign && l.text === 'smoke out.');
  const kept = team.every((gd: any) => O.of(gd) === asg0.get(gd)), logged = O.log.length === log0 + 1 && O.log.at(-1)?.order === SMOKE_ORDER && O.log.at(-1)?.who.join() === thrower.callsign;
  F.verdict(items0 === 0 && !!can && who === thrower && left1 === 1 && menuGone && said && kept && logged && R.held <= 1 && SM.SQUADSMOKE_LOG.at(-1)?.who === thrower.callsign,
    'X, then H (two presses, nothing held): the nearest man with a throw tosses a smoke canister — the item is his, he says so, the count is one fewer, the menu is away; every man keeps the order he was on, and the throw is on the squad\'s order log (an order was given: the rescue\'s clock starts on one)',
    `${thrower.callsign} at ${p2(thrower.char.pos)}, spot ${p2(spot)} (${f1(dist(thrower.char.pos, spot))} m) · item ${can ? `'${can.kind}' who ${can.who} fuse ${can.fuse.toFixed(2)}` : 'none'} · left ${left1} · said ${said} · orders kept ${kept} · logged ${logged} · most keys held at once ${R.held}`);
  R.until(() => !!can?.fired && can.settled, 6); R.step(30);
  const rest = can ? dist(can.pos, spot) : NaN, moved = Math.max(...team.map((gd: any, i: number) => dist(gd.char.pos, pos0[i])));
  F.verdict(!!can && can.fired && can.emitT > 0 && rest < 1.5 && R.said.includes('smoke deployed') && moved < 0.3 && silent0 && SLM.squadSilent(game),
    'the can comes to rest on the spot (within 1.5 m) and vents there (the game\'s own canister: its fuse from the hand, its pop); nobody walked for it; a silent squad is still silent',
    `at rest ${can ? p2(can.pos) : '—'}, ${f1(rest)} m from the spot · fired ${can?.fired} · venting ${can ? f1(can.emitT) : '—'} s left · the furthest any man moved ${moved.toFixed(2)} m · silent ${SLM.squadSilent(game)}`);
  // the second, then none left
  const again = O.give(SMOKE_ORDER, O.targetAt(spot), R.team.indexOf(thrower)); R.step(5); const again2 = O.give(SMOKE_ORDER, O.targetAt(spot)); R.step(5);
  const block2 = R.notes(), m2 = R.menu(spot), row2 = smokeRow(m2.rows), n2 = R.cans().length; R.press(SQUADSMOKE.orderKey); R.step(5); const n3 = R.cans().length; R.esc();
  F.verdict(atOnce === null && again === null && again2 === 'no smoke left.' && SM.squadSmokeLeft(game) === 0 && !!row2 && row2.off && /no smoke left/.test(row2.text) && n2 === 2 && n3 === 2 && block2.includes('smoke 0'),
    'the same man has no second throw in the same breath (the toss has to finish); once it has, told by name he throws the second; a third order is refused in words; at none left the row stays, greyed with its reason, and H throws nothing',
    `the frame after: ${atOnce ? 'a throw' : 'no throw'} · later, to him alone: ${again === null ? 'given' : `'${again}'`} · a third: '${again2}' · left ${SM.squadSmokeLeft(game)} · row '${row2?.text}'${row2?.off ? ' (off)' : ''} · cans on the floor ${n2} then ${n3}`);
  // nobody has a throw to a spot far from every man
  issued(); bring(4, 1);
  const Q = await rig(7, { empty: true }); const far = (() => { const men = Q.team; for (const c of [[6, 0, 10], [30, 0, 8], [34, 0, 20], [10, 0, 21], [20, 0, 6]]) { const t = Q.O.targetAt(c); if (t.kind === 'floor' && men.every((gd: any) => dist(gd.char.pos, t.at) > 12)) return t.at as number[]; } return null; })();
  if (!far) { F.verdict(false, 'a spot of floor more than 12 m from every man', 'none of the candidates is floor that far off'); return F.done(); }
  const mf = Q.menu(far), rf = smokeRow(mf.rows); Q.press(SQUADSMOKE.orderKey); Q.step(5);
  F.verdict(!!rf && rf.off && /no throw from here/.test(rf.text) && Q.cans().length === 0 && SM.squadSmokeLeft(Q.game) === 1,
    'a spot nobody has a throw to (every man more than 12 m off): the row is greyed with its reason, H throws nothing and the can stays in the kit', `spot ${p2(far)}, nearest man ${f1(Math.min(...Q.team.map((gd: any) => dist(gd.char.pos, far))))} m · row '${rf?.text}'${rf?.off ? ' (off)' : ''} · left ${SM.squadSmokeLeft(Q.game)}`);
  // the reach, measured: the furthest spot on open floor a standing man's can comes to rest on
  { const gd = Q.team[0]; let reach = 0; for (let d = 2; d <= 16; d += 0.25) if (spotNear(Q, gd, d)) reach = d;
    F.verdict(reach >= 8 && reach <= 14, 'the reach of the throw, measured on this floor (the page quotes it): somewhere between 8 and 14 m', `${gd.callsign} at ${p2(gd.char.pos)}: the furthest ring with a clear throw is ${reach} m`); }
  return F.done();
});

// ───────────────────────────────────────────────────────────── restarts ────────────────────────────────
export const smokeRestart = guarded(async () => {
  const F = family('squadsmoke-restart: the count is this staging\'s — a restart puts it back, cards as issued take the row away');
  issued(); bring(3, 1); bring(4, 2);
  const R = await rig(5, { empty: true }); const { O, game } = R;
  let spot: number[] | null = null; for (const d of [3.5, 4.5, 2.5]) { for (const gd of R.team) if ((spot = spotNear(R, gd, d))) break; if (spot) break; }
  if (!spot) { F.verdict(false, 'a spot with a throw', 'none'); return F.done(); }
  const b0 = SM.squadSmokeLeft(game), g1 = O.give(SMOKE_ORDER, O.targetAt(spot)); R.step(10); const b1 = SM.squadSmokeLeft(game);
  R.restart(); for (const h of game.guards) if (h.team === 'hostile' && h.state !== 'dead') killGuard(game, h, [0, 0, 1], true);
  const rows = O.defs.filter((d: any) => d.id === SMOKE_ORDER).length, lines = R.notes().filter(t => /^smoke /.test(t));
  F.verdict(b0 === 3 && g1 === null && b1 === 2 && SM.squadSmokeLeft(game) === 3 && rows === 1 && lines.join('|') === 'smoke 3' && R.cans().length === 0,
    'three cans on two cards, one thrown, then a restart: the count is three again, the row is on the menu once, the squad block has one line, the thrown can is off the floor', `brought ${b0} · after one ${b1} · after the restart ${SM.squadSmokeLeft(game)} · rows ${rows} · lines ${JSON.stringify(lines)}`);
  // the cards back as issued, then a restart
  K.kitReset(); R.restart();
  const sam = game.player.char.pos, m = R.menu([sam[0] + 1.8, 0, sam[2] + 0.4]); R.esc();
  F.verdict(!O.defs.some((d: any) => d.id === SMOKE_ORDER) && !smokeRow(m.rows) && SM.squadSmokeLeft(game) === 0 && !R.notes().some(t => /smoke/.test(t)) && O.give(SMOKE_ORDER, O.targetAt(spot)) !== null,
    'the cards put back as issued and the run restarted: no row, no line, no count — and the order no longer exists', `rows ${O.defs.filter((d: any) => d.id === SMOKE_ORDER).length} · left ${SM.squadSmokeLeft(game)} · ${SM.squadSmokeSummary(game)}`);
  // smoke on the cards, the table off
  bring(3, 2); KIT.on = false; R.restart(); const offLeft = SM.squadSmokeLeft(game), offRow = O.defs.some((d: any) => d.id === SMOKE_ORDER); KIT.on = true; R.restart(); const onLeft = SM.squadSmokeLeft(game);
  F.verdict(offLeft === 0 && !offRow && onLeft === 2, 'smoke on a card with the kit table switched off: a staging brings none; switched on again, the next staging brings them', `table off ${offLeft} (row ${offRow}) · on again ${onLeft}`);
  // the pair: its two men's cards only
  issued(); bring(1, 1); bring(3, 2);
  const P = await rig(5, { squad: 2, empty: true });
  F.verdict(P.team.length === 2 && SM.squadSmokeLeft(P.game) === 1 && K.kitSmokes(2) === 1, 'the pair: the count is its two men\'s (a card of a man who did not come brings nothing)', `squad ${P.team.length} · left ${SM.squadSmokeLeft(P.game)} · kitSmokes(2) ${K.kitSmokes(2)} kitSmokes(4) ${K.kitSmokes(4)}`);
  return F.done();
});

if (import.meta.main) {
  const all: Record<string, () => Promise<boolean>> = { 'squadsmoke-table': smokeTable, 'squadsmoke-issued': smokeIssued, 'squadsmoke-order': smokeOrder, 'squadsmoke-restart': smokeRestart };
  process.exit(exitCode(await asProbes(all, wanted())));
}
