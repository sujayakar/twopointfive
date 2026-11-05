// TWO PLAYERS, TWO TEAMS TO THE EYE (game/seats.ts seatTone, playerName; ui/findme.ts findMeOutline; ui/planfloor.ts plansToDraw; game/verdict.ts): in a game
// of two seats, does each player's page show the WHOLE team — every man rimmed, in the colour that says whose he is, the same on both screens — the other
// player's squad's plan on the floor, fainter than its own, and the two players told apart by number? The maintainer, 11-03, from his first game of two:
// "no outlines for the other player character (maybe we should have yellow + red team and then purple + blue team)", "seeing your teammates squad path
// overlays (but probably fainter than yours) would be great", "at least have numbers for players".
//   bun run tools/qa/teams-check.ts                  every line below; exit 0 = all pass, 1 = one failed. Run it on both stands (TPS_PROFILE=shipped).
//   TEAMS_BREAK=1 bun run tools/qa/teams-check.ts    the red path: two seats stood as ever, and the rule put off by its own tables — every seat given the
//                                                    first seat's tone (`SEAT_TONES` cut to one) and another's plan put to none (`PLAN.others` 0):
//                                                    `rims`, `both`, `plans`, `faint`, `once`, `names` and `picked` (the page handed no names) must FAIL
//                                                    (`down`, `one` and `pure` hold); and the fog's row is left off while `fog` asks about it on: it must FAIL too
// The mission is the room clear with the four (`?roomclear&squad4`), every man put in the dark. Nothing here draws: the rims are the frame the page hands the
// outline's pass, the plans the list the overlay walks. Never seen on a GPU: the purple and how faint the other plan is were picked by number.
// WHAT IS HELD:
//   rims    shown from the first seat: his own man's rim is his seat's yellow; five more are rimmed — the second player's man in purple, the red pair red, the
//           blue pair blue — each the teammates' row's share of his own
//   both    shown from the second seat: his own man's rim is purple, the first player's among the others in yellow, and every man has the ink he had on the
//           first seat's screen
//   down    a player who is down gives his place up on the other's screen, and on HIS OWN the team is still rimmed (no rim of his own) while the other
//           player is up; with both down the frame is off; the teammates' row at none leaves the shown player's own rim alone
//   plans   an order the second player gives his pair is on the FIRST player's floor as another's plan — its men, their ways and marks, in their pair's
//           colour — and not in his own; on the second player's own page it is his own plan, and the first player's order is the other's there; with
//           the looker's own menu up another's standing orders are listed too
//   faint   another's plan is drawn weaker than the seat's own (`PLAN.others`, under 1) — held on what the overlay itself writes, on a page stood in for
//           (elements that keep what was written on them): the same men's groups at that share of their own page's strength; at none it is not asked for
//   once    a wait both seats' plans list (both pairs stacked on one door) is on a page once, as its own with its key; another's alone has no key
//   names   the debrief's rows: 'Player 1' in the first seat's tone and 'Player 2' in purple, and `playerName` says the same
//   picked  a name a player picked (host/names.ts: said by the server, kept by the page, never the game's state) is what the page calls him; the other
//           player keeps his number and a teammate his callsign; the verdict's own rows and the game's state word do not move; a name is cleaned
//   fog     under the fog of war a flash made by EITHER seat's own man is the team's (what a wall charge's blast hands the fog is its seat's man:
//           blastLook.ts), nobody's is not, and neither is with the row off. No charge is blown: the afterglow light is off in every build
//   one     a game of ONE seat: 'Player', the tone it always had, one plan to draw, the four rimmed red and blue as ever
//   pure    none of the asking — rims filled, plans listed, the overlay drawn, from both seats — moved the game's state word
import { standUp, ROOT } from './headless.ts';

const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { stateHash } = await import(`${ROOT}/src/game/rollback.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const SE = await import(`${ROOT}/src/game/seats.ts`) as typeof import('../../src/game/seats.ts');
const F = await import(`${ROOT}/src/host/findme.ts`) as typeof import('../../src/host/findme.ts');
const { findMeOutline } = await import(`${ROOT}/src/ui/findme.ts`) as typeof import('../../src/ui/findme.ts');
const { makeOutlineFrame } = await import(`${ROOT}/src/render/outline.ts`) as typeof import('../../src/render/outline.ts');
const PF = await import(`${ROOT}/src/ui/planfloor.ts`) as typeof import('../../src/ui/planfloor.ts');
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`) as typeof import('../../src/game/doortools.ts');
const FP = await import(`${ROOT}/src/game/floorplan.ts`) as typeof import('../../src/game/floorplan.ts'), { PLAN } = FP;

const BREAK = process.env.TEAMS_BREAK === '1', DT = 1 / 60;
let all = true;
const say = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
const hush = async <T,>(f: () => T | Promise<T>): Promise<T> => { const log = console.log; console.log = () => {}; try { return await f(); } finally { console.log = log; } };
const stand = async (seats: number) => { Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: 9, seats }); const r: any = await hush(() => startFromQuery(W.game, '?roomclear&squad4')); return { W, g: W.game, why: r?.why as string | undefined }; };
const INK = F.OUTLINE_INK, mix = (hue: keyof typeof INK) => INK.pale.map((v, i) => v + (INK[hue][i] - v) * F.FINDME.colour);
const inkIs = (m: { ink: number[] }, hue: keyof typeof INK) => m.ink.every((v, i) => Math.abs(v - mix(hue)[i]) < 1e-9);
const hueOf = (m: { ink: number[] }) => (['you', 'purple', 'red', 'blue'] as const).find(h => inkIs(m, h)) ?? '?';

/** just enough of a page for the overlay to draw into (ui/planfloor.ts PlanFloor): every element keeps what was written on it */
interface FakeEl { tag: string; attrs: Record<string, string>; style: Record<string, string>; kids: FakeEl[]; textContent: string; firstChild: FakeEl | null; setAttribute(k: string, v: string): void; appendChild(e: FakeEl): FakeEl; insertBefore(e: FakeEl, at: FakeEl | null): FakeEl }
const fakeEl = (tag: string): FakeEl => { const e: FakeEl = { tag, attrs: {}, style: {}, kids: [], textContent: '', firstChild: null, setAttribute(k, v) { e.attrs[k] = v; }, appendChild(c) { e.kids.push(c); e.firstChild = e.kids[0]; return c; }, insertBefore(c, at) { const i = at ? e.kids.indexOf(at) : -1; if (i < 0) e.kids.push(c); else e.kids.splice(i, 0, c); e.firstChild = e.kids[0]; return c; } }; return e; };
/** the men's groups the overlay shows after one update from `g`'s shown seat: how strong each is drawn and whether its way was written */
function drawn(g: any): { k: number; way: boolean }[] {
  const G = globalThis as any, had = G.document;
  G.document = { createElementNS: (_ns: string, tag: string) => fakeEl(tag), body: { classList: { contains: () => false } } };
  try {
    const svg = fakeEl('svg'), floor = new PF.PlanFloor(svg as never);
    floor.update(g, { project: (p: number[], w: number, h: number) => [w / 2 + p[0] * 10, h / 2 + p[2] * 10, true] } as never, 800, 600);
    const shown = (e: FakeEl) => e.style.display === 'block';
    return svg.kids[0].kids.filter(e => e.tag === 'g' && shown(e) && e.kids.some(c => c.attrs.class?.startsWith('pm') && shown(c))).map(e => ({ k: Number(e.style.opacity), way: e.kids.some(c => c.attrs.class === 'pp' && shown(c) && !!c.attrs.d) }));
  } finally { G.document = had; }
}
console.log(`teams-check: two players, two teams to the eye${BREAK ? '  (TEAMS_BREAK: one tone for every seat, another\'s plan at none, no names handed over, the fog\'s row left off: rims, both, plans, faint, once, names, picked and fog must fail)' : ''}`);
const found = { ...F.FINDME }, tuned = F.FINDTUNED.on, plan0 = { on: PLAN.on, menuOnly: PLAN.menuOnly, others: PLAN.others }, tones0 = [...SE.SEAT_TONES], TONES = SE.SEAT_TONES as unknown as string[];
const others0 = PLAN.others;
try {
  F.FINDTUNED.on = false; F.findMeStarts(); F.setFindMe('outline', true); PLAN.on = true; PLAN.menuOnly = true;
  if (BREAK) { TONES.length = 1; PLAN.others = 0; }
  const { W, g, why } = await stand(2);
  for (let i = 0; i < 60; i++) W.step(DT);
  const two = g.seats.length === 2, v0 = g.seats[0].view, v1 = (g.seats[1] ?? g.seats[0]).view, p0 = v0.player, p1 = v1.player;
  const team: any[] = SE.teamMen(g), dark = () => { for (const p of SE.men(g)) { p.visibility = 0.04; p.nvAmount = 0; } for (const gd of team) gd.visibility = 0.04; };
  const show = (i: number) => { g.localSeat = g.seats[Math.min(i, g.seats.length - 1)]; };
  const fill = () => { const M = makeOutlineFrame(); dark(); findMeOutline(M, g, 0); return M; };

  // ---- rims · both
  show(0); const A = fill(), a = A.mates.filter(m => m.on), byId = (L: typeof a, id: number) => L.find(m => m.owner === id);
  const pairHue = (gd: any) => gd.unit?.pair === 'red' ? 'red' : 'blue', reds = team.filter(gd => pairHue(gd) === 'red').length;
  const matesOk = (L: typeof a) => team.every(gd => { const m = byId(L, gd.char.id); return !!m && inkIs(m, pairHue(gd)) && m.times === F.FINDME.mates && m.strength === 1; });
  const rims = two && !why && A.on && A.owner === p0.char.id && inkIs(A, 'you') && a.length === 5 && team.length === 4 && reds === 2 && matesOk(a) && !!byId(a, p1.char.id) && inkIs(byId(a, p1.char.id)!, 'purple') && byId(a, p1.char.id)!.times === F.FINDME.mates;
  say(rims, 'rims: from the first seat every man of the team is rimmed — his own yellow, the other player purple, the red pair red, the blue pair blue',
    `seats ${g.seats.length}${why ? ` · refused: ${why}` : ''} · his own: ${A.on ? hueOf(A) : 'OFF'} · others rimmed ${a.length} (5 wanted): ${a.map(m => `#${m.owner} ${hueOf(m)}`).join(' ')}`);
  show(1); const B = fill(), b = B.mates.filter(m => m.on);
  const sameInk = [A, ...a].every(m => { const o = m.owner === B.owner ? B : byId(b, m.owner); return !!o && o.ink.every((v, i) => v === m.ink[i]); });
  const both = two && B.on && B.owner === p1.char.id && inkIs(B, 'purple') && b.length === 5 && matesOk(b) && !!byId(b, p0.char.id) && inkIs(byId(b, p0.char.id)!, 'you') && sameInk;
  say(both, 'both: from the second seat his own rim is purple, the first player is among the others in yellow, and every man has the ink he had on the other screen',
    `his own: ${B.on ? `#${B.owner} ${hueOf(B)}` : 'OFF'} · others ${b.map(m => `#${m.owner} ${hueOf(m)}`).join(' ')} · the same inks on both screens: ${sameInk}`);

  // ---- down
  show(0); const was = p1.down; p1.down = true; const C = fill().mates.filter(m => m.on);
  show(1); const E = fill(), e = E.mates.filter(m => m.on), mineStill = E.on && E.strength === 0 && e.length === 5 && !!byId(e, p0.char.id);   // (his own page: he is down, the team is not)
  const was0 = p0.down; p0.down = true; const allOff = !fill().on; p0.down = was0; p1.down = was; show(0);
  F.setFindMe('mates', 0); const D = fill(), alone = D.on && D.mates.every(m => !m.on); F.setFindMe('mates', F.FINDME_START.mates);
  say(two && C.length === 4 && !byId(C, p1.char.id) && mineStill && allOff && alone, 'down: a player who is down gives his place up on the other screen and keeps the team\'s rims on his own; both down, none; the teammates\' row at none leaves his own rim alone',
    `the second player down: ${C.length} others rimmed on the first's screen (4 wanted), his among them ${!!byId(C, p1.char.id)}; on his own screen his rim's strength ${E.strength} and ${e.length} others (5 wanted) · both down: the frame ${allOff ? 'off' : 'ON'} · teammates' row at none: his own ${D.on ? 'on' : 'OFF'}, others ${D.mates.filter(m => m.on).length}`);

  // ---- plans · faint
  // (a floor spot some way off that the seat's own order book takes: the first one it does not refuse)
  const send = (v: any): string => { const O = v.orders, s = v.player.char.pos; let why = 'no floor found';
    for (let d = 6; d <= 12; d += 2) for (let k = 0; k < 16; k++) { const y = k * Math.PI / 8, p = [s[0] + Math.sin(y) * d + 0.25, 0, s[2] + Math.cos(y) * d + 0.25], t = O.targetAt(p); if (t.kind !== 'floor') continue; why = O.give('move', t, 'all') || ''; if (!why) return ''; }
    return why; };
  const why1 = send(v1); for (let i = 0; i < 6; i++) W.step(DT);
  const mine = (v: any) => new Set(v.orders.squad().map((gd: any) => gd.char.id)), ids = (P: any) => (P.men as any[]).map(m => m.gd.char.id);
  show(0); const L0 = PF.plansToDraw(g), own0 = L0[0], oth0 = L0[1], seen0 = drawn(g);
  show(1); const L1 = PF.plansToDraw(g), own1 = L1[0], oth1 = L1[1], seen1 = drawn(g);
  const his = mine(v1), listed = oth0 ? ids(oth0.P) : [], walked = oth0 ? (oth0.P.men as any[]).filter(m => m.path.length > 1).length : 0, tones = oth0 ? (oth0.P.men as any[]).map(m => m.tone) : [];
  const why0 = send(v0); for (let i = 0; i < 6; i++) W.step(DT);
  show(1); const L2 = PF.plansToDraw(g), oth2 = L2[1], mine0 = mine(v0);
  // (three seconds on both orders have gone out with their age; with the looker's own menu up the other squad's standing orders are listed again)
  for (let i = 0; i < 200; i++) W.step(DT); show(0); const aged = PF.plansToDraw(g)[1], agedN = aged ? ids(aged.P).length : -1;
  const menu0 = v0.orders.menu; v0.orders.menu = { men: [], rows: [], sel: 0 } as never; const read = PF.plansToDraw(g)[1], readN = read ? ids(read.P).length : -1; v0.orders.menu = menu0;
  const plans = two && !why1 && !why0 && L0.length === 2 && listed.length === his.size && listed.every(id => his.has(id)) && walked > 0 && tones.every(t => t === 'blue') && !ids(own0.P).some(id => his.has(id))
    && ids(own1.P).length === his.size && ids(own1.P).every(id => his.has(id)) && (own1.P.men as any[]).every(m => m.tone === 'blue') && !!oth1 && ids(oth1.P).length === 0
    && !!oth2 && ids(oth2.P).length === mine0.size && ids(oth2.P).every(id => mine0.has(id)) && (oth2.P.men as any[]).every(m => m.tone === 'red') && agedN === 0 && readN === his.size;
  say(plans, 'plans: the second player\'s order is on the first player\'s floor as another\'s plan, in its pair\'s colour, and not in his own; and the other way round; standing orders show while the looker\'s menu is up',
    `orders: ${why1 || 'given'}, ${why0 || 'given'} · first seat's page: ${L0.length} plans, its own lists ${ids(own0.P).length} men, the other's ${listed.length} (${his.size} wanted, ${walked} with a way, ${tones.join(' ') || 'no tone'}) · second seat's page: its own ${ids(own1.P).length}, the other's ${oth1 ? ids(oth1.P).length : '—'} before the first player's order and ${oth2 ? ids(oth2.P).length : '—'} after · 200 ticks on: ${agedN} listed, with the looker's menu up ${readN}`);
  PLAN.others = 0; const none = PF.plansToDraw(g).length; PLAN.others = BREAK ? 0 : others0;
  // (as the overlay writes it: the same two men's groups, each with his way, from the other player's page at `PLAN.others` of what their own page gives them)
  const top = (L: { k: number }[]) => Math.max(0, ...L.map(m => m.k)), drawnOk = seen0.length === his.size && seen1.length === his.size && seen0.every(m => m.way && m.k > 0) && seen1.every(m => m.way) && top(seen1) > 0.5 && Math.abs(top(seen0) - top(seen1) * others0) < 0.011;
  say(two && !!oth0 && oth0.k === others0 && others0 > 0 && others0 < 1 && own0.k === 1 && none === 1 && drawnOk, 'faint: another\'s plan is drawn weaker than the seat\'s own, by the overlay\'s own hand; at none it is not asked for',
    `its own ${own0.k}, the other's ${oth0 ? oth0.k : '—'} · the overlay, the second player's two men: on his own page ${seen1.map(m => m.k.toFixed(2)).join(' ') || 'none'}, on the first player's ${seen0.map(m => m.k.toFixed(2)).join(' ') || 'none'}, each with his way ${seen0.every(m => m.way) && seen1.every(m => m.way)} · at none: ${none} plan${none === 1 ? '' : 's'} to draw`);

  // ---- once
  // (one or two teammates on a plain stack wait on the key under DOORTOOLS.wordSmall: the page's rule, held off on the frozen stand, raised here for this line)
  { const small0 = DOORTOOLS.wordSmall; DOORTOOLS.wordSmall = true;
    const door = [...g.doors.list].map((d: any) => ({ d, t: v0.orders.targetAt(d.frameCentre) })).filter(x => x.t.kind === 'door').sort((x, y) => Math.hypot(x.d.frameCentre[0] - p0.char.posX, x.d.frameCentre[2] - p0.char.posZ) - Math.hypot(y.d.frameCentre[0] - p0.char.posX, y.d.frameCentre[2] - p0.char.posZ));
    let w0 = 'no door', w1 = 'no door', at: any = null;
    for (const x of door) { w0 = v0.orders.give('stack', x.t, 'all') || ''; if (w0) continue; w1 = v1.orders.give('stack', v1.orders.targetAt(x.d.frameCentre), 'all') || ''; at = x.d; break; }
    let both2 = false, n0 = -1, n1 = -1, keyed = false, bare = false;
    const lists = (v: any) => (FP.floorPlan(v).doors as any[]).some(d => d.door === at);
    for (let i = 0; i < 60 * 25 && !both2; i++) { W.step(DT); if (i % 20 === 19) both2 = lists(v0) && lists(v1); }
    if (both2) { show(0); const L = PF.plansToDraw(g), all2 = L.flatMap(x => x.P.doors).filter((d: any) => d.door === at); n0 = all2.length; keyed = L[0].P.doors.some((d: any) => d.door === at && !!d.key);
      // …and a wait that is another's alone: the first seat's men called off, the second's still on the door
      v0.orders.give('fallIn', v0.orders.targetAt(p0.char.pos), 'all'); for (let i = 0; i < 90; i++) W.step(DT);
      const M = PF.plansToDraw(g), theirs = (M[1]?.P.doors ?? []).filter((d: any) => d.door === at); n1 = M.flatMap(x => x.P.doors).filter((d: any) => d.door === at).length; bare = theirs.length === 1 && theirs[0].key === null && theirs[0].now === false; }
    say(two && both2 && n0 === 1 && keyed && n1 === 1 && bare, 'once: a door both squads wait on is on a page once, as its own with the key; another\'s alone has no key',
      `stack on one door: ${w0 || 'given'}, ${w1 || 'given'} · both plans list it ${both2} · on the first seat's page ${n0} time${n0 === 1 ? '' : 's'}, with its key ${keyed} · the first seat's men called off: ${n1} time${n1 === 1 ? '' : 's'}, as another's with no key ${bare}`); DOORTOOLS.wordSmall = small0; }

  // ---- names
  const tracks = [...((g.verdict as any).tracks as Map<number, any>).values()].filter(T => T.human);
  const names = two && tracks.length === 2 && tracks[0].name === 'Player 1' && tracks[0].tone === 'you' && tracks[1].name === 'Player 2' && tracks[1].tone === 'purple' && SE.playerName(g, p0) === 'Player 1' && SE.playerName(g, p1) === 'Player 2';
  say(names, 'names: the debrief has Player 1 in the first seat\'s tone and Player 2 in purple', tracks.map(T => `${T.name} (${T.tone})`).join(' · ') || 'no human row');
  // ---- picked
  { const N = await import(`${ROOT}/src/host/names.ts`) as typeof import('../../src/host/names.ts'), WR = await import(`${ROOT}/src/net/wire.ts`) as typeof import('../../src/net/wire.ts'), { cleanName } = WR;
    const { NetServer } = await import(`${ROOT}/src/net/server.ts`) as typeof import('../../src/net/server.ts');
    const over = (t: unknown) => cleanName((WR.netParse(WR.netText({ n: t })) as { n: unknown }).n);   // (as a name crosses the wire)
    const clean = cleanName('  Grace\u0007   Hopper the Second ') === 'Grace Hopper' && cleanName('<b>x</b>') === '<b>x</b>' && cleanName(7) === '' && cleanName(null) === '' && cleanName(' \n\t ') === ''
      && cleanName('a\u200bb\u202e') === 'ab' && cleanName('\u3164\u2800') === '' && cleanName('...') === '' && over('nan') === 'nan' && over('inf') === 'inf' && over('Ada') === 'Ada' && cleanName('\u{1F468}\u200D\u{1F469}\u200D\u{1F467}') === '\u{1F468}\u200D\u{1F469}\u200D\u{1F467}';
    // the server's own cleaning: a hello written by hand, never through a client's
    const got: string[] = [], conn = { send(t: string) { got.push(t); return 1; }, close() { /* */ } };
    const srv = new NetServer({ now: () => 0, step() { /* */ }, word: () => '', ended: () => null } as never, { game: 'g', build: '', recipe: {} as never, seatKeys: ['a', 'b'] });
    srv.message(conn, JSON.stringify({ k: 'hello', proto: WR.NET.proto, build: '', game: 'g', key: 'b', name: '  Grace\u0007   Hopper the Second ' }));
    const said = got.map(t => JSON.parse(t)).filter(m => m.k === 'state').pop(), served = JSON.stringify(said?.names) === JSON.stringify(['', 'Grace Hopper']);
    const w0 = stateHash(g), mate = team[0];
    const before = N.shownName(g, p0.char.id) === 'Player 1' && N.shownName(g, p1.char.id) === 'Player 2';
    N.setSeatNames(g, BREAK ? [] : ['Ada', '']);
    const a = N.shownName(g, p0.char.id), b = N.shownName(g, p1.char.id), c = N.shownName(g, mate.char.id, mate.callsign), kept = tracks[0].name === 'Player 1' && stateHash(g) === w0;
    // a name that would not tell the two apart is not used: the same one twice, or one that reads as a seat's number
    N.setSeatNames(g, ['Ada', 'ada']); const twin = N.shownName(g, p0.char.id) === 'Player 1' && N.shownName(g, p1.char.id) === 'Player 2';
    N.setSeatNames(g, ['Player 2', 'Grace']); const posed = N.shownName(g, p0.char.id) === 'Player 1' && N.shownName(g, p1.char.id) === 'Grace';
    say(clean && served && before && a === 'Ada' && b === 'Player 2' && c === mate.callsign && kept && twin && posed, 'picked: a name a player picked is what the page calls him, the other keeps his number, a teammate his callsign; the game\'s own names and its state word are as they were; a name is cleaned, by the server too',
      `cleaned as wanted ${clean} · a raw hello's dirty name, as the server says it: ${JSON.stringify(said?.names ?? null)}${served ? '' : ' ✗'} · before any name: ${before ? 'Player 1, Player 2' : 'NOT the numbers'} · the first seat named Ada: ${a}, ${b}, ${c} · the verdict's row still ${tracks[0].name}, the state word ${stateHash(g) === w0 ? 'equal' : 'MOVED'} · the same name twice: ${twin ? 'both keep their numbers' : '✗'} · a name that reads as the other's number: ${posed ? 'not used' : '✗'}`);
    N.setSeatNames(g, []); }

  // ---- fog
  // (whose light a flash is under the fog of war, asked of each seat's own man: what blastLook.ts hands the fog for a wall charge and every stage of its blast
  // is `g.player.char.id` on the SEAT's view. The afterglow light itself, BLAST.glow, is off in every build, so no charge is blown here to light one.)
  { const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`) as typeof import('../../src/game/fogofwar.ts'), FW = g.fogOfWar, L = g.engine.lights, was = { ours: FOG.ours, full: FOG.full, on: FW.on };
    FW.on = true; FOG.full = true; FOG.ours = !BREAK; for (let i = 0; i < 40; i++) W.step(DT);   // (the red path: the row left off while it is asked about on)
    const made = (id: number) => { const l = FW.lightBy(L.flash([p0.char.posX, 1.2, p0.char.posZ], [1, 1, 1], 5, 5, 0.3, 0), id); const c = L.fogClass(l); l.ttl = 0; l.age = 1; return c; };
    const ids = g.seats.map((s: any) => s.view.player.char.id as number), ours = ids.map((id: number) => FW.oursId(id) as boolean), cls = ids.map(made), nobody = made(0);
    FOG.ours = false; const off = ids.map(made); FOG.ours = was.ours; FOG.full = was.full; FW.on = was.on;
    say(two && FW.active !== false && ids.length === 2 && ids[0] !== ids[1] && ours.every(Boolean) && cls.every((c: number) => c === 0) && nobody === 1 && off.every((c: number) => c === 1),
      'fog: a flash made by either seat\'s own man is the team\'s under the fog of war, nobody\'s is not, and with the row off neither is',
      `the seats' men ${ids.join(' and ')} · the team's own: ${ours.join(' / ')} · class of a flash by each ${cls.join(' / ')} (0 = lights the fog's picture), by nobody ${nobody} · with the row off ${off.join(' / ')}`); }

  // ---- one
  const S1 = await stand(1), g1 = S1.g; for (let i = 0; i < 60; i++) S1.W.step(DT);
  for (const gd of SE.teamMen(g1) as any[]) gd.visibility = 0.04; g1.player.visibility = 0.04; g1.player.nvAmount = 0;
  const M1 = makeOutlineFrame(); findMeOutline(M1, g1, 0); const m1 = M1.mates.filter(m => m.on), t1 = [...((g1.verdict as any).tracks as Map<number, any>).values()].filter(T => T.human);
  const one = g1.seats.length === 1 && SE.playerName(g1, g1.player) === 'Player' && t1.length === 1 && t1[0].name === 'Player' && t1[0].tone === 'you' && PF.plansToDraw(g1).length === 1
    && M1.on && inkIs(M1, 'you') && m1.length === 4 && m1.filter(m => inkIs(m, 'red')).length === 2 && m1.filter(m => inkIs(m, 'blue')).length === 2;
  say(one, 'one: a game of one seat is as it was — Player, one plan to draw, the four rimmed red and blue', `${t1.map(T => `${T.name} (${T.tone})`).join(' ')} · plans ${PF.plansToDraw(g1).length} · his own ${hueOf(M1)}, others ${m1.map(hueOf).join(' ')}`);

  // ---- pure
  // (the orders given above are the game's own and move its word: what is held is the asking — every way this file asks, from both seats, with plans up)
  const ask = () => { findMeOutline(makeOutlineFrame(), g, 0); findMeOutline(makeOutlineFrame(), g, DT); PF.plansToDraw(g); drawn(g); };
  send(v0); send(v1); for (let i = 0; i < 6; i++) W.step(DT);
  dark(); show(0); const h0 = stateHash(g); show(1); ask(); show(0); ask(); const h1 = stateHash(g);
  say(h0 === h1 && typeof h0 === 'string' && h0.length > 0, 'pure: rims filled, plans listed and the overlay drawn from both seats leave the game\'s state word where it was', `${h0 === h1 ? 'equal' : 'MOVED'}`);
} finally { F.FINDTUNED.on = tuned; F.findMeStarts(); Object.assign(F.FINDME, found); PLAN.on = plan0.on; PLAN.menuOnly = plan0.menuOnly; PLAN.others = plan0.others; TONES.length = 0; TONES.push(...tones0); }
console.log(all ? '\nteams: ALL PASS' : '\nteams: FAIL');
process.exit(all ? 0 : 1);
