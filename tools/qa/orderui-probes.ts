// The 'orderui' family (game/orders.ts — the squad's command menu as a PRESS menu, and WHO an order goes to with four teammates in two pairs: NIGHTBREACH.squad
// = 4, the page's ?squad4), run through probes.ts:
//   bun run tools/qa/probes.ts orderui | orderui-four | orderui-block | orderui-half | orderui-keys
// PASS / FAIL each, all headless (the Game of headless.ts under the room clear's staging, Sam in god mode and planted dark, the defenders kept dazzled: this
// family is about who is ordered and how, not the fight). Every order is given THROUGH THE INPUT as a keyboard and mouse would write it — the harness cursor on a
// world point, then one-frame presses of X / the middle button / T / B / a digit / the wheel / a click / Space — and the rig counts the keys and buttons DOWN on
// every frame: where a probe says "nothing held" that count never left zero. Nothing here looks at a picture: the colours, the ring on the target and how the
// menu reads on the page are checked as data, never drawn.
//   orderui-four   'move there' on one spot given to the squad (X, then its digit): every man takes it and goes, nothing held; the HUD named who before the
//                  order was given
//   orderui-block  the squad block with everyone addressed: the header says GOLD, each man has a row in his pair's colour with what he is doing, and an order
//                  given to everyone changes what each row says (ORDERBLOCK_BREAK=1: the order is never given)
//   orderui-half   a door tool to one half, as the page plays: Shift and the squad's key kept down on a door (the ring is one whole pair's), its
//                  'open and clear' wedge: that pair alone is in the door order and goes in, the other two keep the order they had and stay on Sam's side of the
//                  door (ORDERHALF_BREAK=1: the key kept down without Shift)
//   orderui-keys   what the menu takes of the frame's input, and what it never takes: Sam walks on with it up (frozen on its target), a digit with no row does
//                  nothing, B is dead while it is up; THE LEFT BUTTON IS ALWAYS HIS TRIGGER (a click with the menu up: the menu goes, no order, he fires); a right click is used up; the frame the menu opens on takes
//                  nothing else (X with a digit: the list is up, no order); two digits in one frame give one order and neither is a slot; a menu opened
//                  on one man goes away when he goes down, and so does a pair's when the pair is down — never on to everybody else; T on a man twice in
//                  the one-pair squad is back to all; a restart takes it down
// DELETED 2026-11-07, AS LINES THAT ASKED A BUILD FROM BEFORE TWO SETTLED RULES (the rig held CMDRING.wholeSquad and CMDRING.only off to ask them: an order
// to a part of the squad stepped with T, the menu's own keys and the middle button, "as the menu was built"; both rules are settled on and took those away;
// `probes.ts qonly` and wholesquad-check have the page's):
//   orderui-four, for the red pair, the blue pair and one man each: 'before the order the HUD says who it will go to' · '"move there" goes to … and to nobody
//     else' · 'they go to the spot; the rest stay with Sam' · 'no key or button was down on any frame' (the three stagings went with them)
//   orderui-hud, the whole probe: 'no menu, everyone addressed … T's choices' · 'T steps GOLD → RED → BLUE → GOLD' · 'the cursor on a door: B would stack on
//     it, X opens the menu on it; on a teammate … T would address him' · 'the menu up on a door with red addressed: a who row in red' · 'the mark for the
//     page's ring sits on that door, in red' · 'T with the menu up steps who it goes to' · 'a man addressed with T stays the choice after the menu closes' ·
//     'no key or button was down on any frame of this'
//   orderui-door, the whole probe: 'T, X on the door, 5, 1: "open and clear" is taken by the red pair' · 'red works the door and goes in; blue was never in
//     the door order' · 'no key or button was down on any frame' · 'blue stacked on the door (T twice, B), then "open and clear" (X, 5, 1) to red'
// 2026-11-07: the rig no longer holds CMDRING.wholeSquad and CMDRING.only off (both settled on: the lines are asked of the menu as the page has it), and the
// two lines behind knobs that are settled off went: 'ORDERS.wheelElement on' (orderui-hud) and 'ORDERS.clickGives on' (orderui-keys).
import { blockOf, menuKey } from './squadwords';
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { SILENT } = await import(`${ROOT}/src/game/silent.ts`);
const { START } = await import(`${ROOT}/src/game/startfile.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS)), RC_DEFAULTS = { ...ROOMCLEAR }, NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
const reset = () => { Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); };
/** every probe leaves the knob tables as it found them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { reset(); } };
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const names = (l: any[]) => l.map((g: any) => g.callsign).join(' + ');

// open floor on the aisle east of where the squad starts, in plain view of the camera · where Sam stands to work the break room's west door (off its line)
const SPOT: number[] = [15, 0, 17.6], SAM_W: number[] = [25.0, 0, 18.9];

/** One game under a seed: the room clear staged with four teammates (`squad: 2` = the pair), Sam in god mode and dark, the defenders dazzled every frame. */
async function rig(seed: number, o: { squad?: 2 | 4; sam?: number[] } = {}) {
  reset(); Object.assign(NIGHTBREACH, { squad: o.squad ?? 4 });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, input = W.input; const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true;
  const why = RC.start(seed);
  const R = { W, game, input, RC, NB, O, D, why,
    /** the most keys + mouse buttons DOWN at the start of any frame so far (a one-frame press is not a key down: `pressed` / `clicked`, never `keys` / `buttons`) */
    held: 0, post: null as null | (() => void),
    get ops(): any[] { return NB.ops; },
    step(n = 1) { for (let i = 0; i < n; i++) {
      game.player.visibility = 0.05; for (const h of NB.hostiles) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
      R.held = Math.max(R.held, input.keys.size + (input.buttons ? 1 : 0));
      W.step(DT); R.post?.(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    /** the cursor on a world point */
    cursor(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); },
    on(m: any) { R.cursor([m.char.pos[0], 1.2, m.char.pos[2]]); },
    /** one-frame presses, as the page's listeners write them */
    press(code: string) { input.pressed.add(code); R.step(); },
    mmb() { input.clicked |= 2; R.step(); },
    click() { input.clicked |= 1; R.step(); },
    rclick() { input.clicked |= 4; R.step(); },
    wheel(d: number) { input.wheel += d; R.step(); },
    last(): any { return O.log[O.log.length - 1] ?? null; },
    status(): any { return blockOf(O)[0] ?? null; },
  };
  R.step(30);
  if (o.sam) { game.teleportPlayer(o.sam); R.step(5); }
  return R;
}
const line = (l: any) => l ? `${l.order} → ${l.who.join(' + ')} on ${l.target}` : 'nothing';
/** the squad block's header for who the next order goes to. SILENT.on, as the page has it (src/game/silent.ts): the squad's posture is a word of the header,
 *  between the two — 'loud' in this family, where nobody says 'go silent' — and a row of a floor's menu and of one man's. With the switch off, as on the
 *  harness's older stand, the header has no such word and the menus no such row. */
//  (asked for once a floor is staged, never before: the stand's switches are set as the game stands up)
const head = (tag: string) => `squad · ${SILENT.on ? 'loud · ' : ''}orders to ${tag}`;
/** THE MEN AN ORDER IS NOT FOR. START.fallIn, as the page has it (src/game/startfile.ts): the four start on Sam's 'fall in', unspoken, so such a man has an
 *  order before anything is pressed — the one he started on. What the page promises of him is that he KEEPS it (the very same assignment after the press as
 *  before it, never the new order) and stays with Sam: on his own mark in the file, wherever Sam is. With the switch off, as on the harness's older stand, he
 *  has no order before or after, and stands where he stood. `kept`: his order now is the one he held then, and that one is what this stand starts him on. */
const kept = (O: any, m: any, was: any) => O.of(m) === was && (START.fallIn ? was?.order === 'fallIn' : was === null);
/** fallen in and there: on 'fall in', arrived, on the mark it gives him behind Sam */
const inFile = (O: any, m: any) => { const a = O.of(m); return a?.order === 'fallIn' && a.arrived && dist(m.char.pos, a.goal) < 0.3; };

// ---------------------------------------------------------------- the same order to the squad, to red, to blue, to one man
export const orderuiFour = guarded(async () => {
  const { verdict, done } = family('orderui-four: one order to the squad — a press, nothing held, and every man goes');
  const K = ORDERS;
  const CASES: { name: string; tag: string; tone: string; want: number[]; address(R: any): void; give(R: any): void; how: string }[] = [
    { name: 'the squad', tag: 'GOLD', tone: 'gold', want: [0, 1, 2, 3], how: 'X pressed on the spot, then 1',
      address: () => {}, give: R => { R.cursor(SPOT); menuKey(R.O ?? R.game.orders); R.step(20); R.press('Digit1'); } },
  ];
  for (const c of CASES) {
    const R = await rig(21); const { O } = R; const ops = R.ops; const p0 = ops.map((m: any) => [...m.char.pos]);
    if (ops.length !== 4) { verdict(false, `${c.name}: four teammates staged`, `${ops.length} staged (${R.why ?? 'started'})`); continue; }
    c.address(R); R.step(5);
    const st = R.status(), marks = (st?.opts ?? []).slice(0, 4).map((o: any) => o.text.slice(0, 1) === '▸' ? 1 : 0), wantMarks = ops.map((_: any, i: number) => c.want.length < 4 && c.want.includes(i) ? 1 : 0);
    verdict(st?.label === head(c.tag) && st.tone === c.tone && marks.join() === wantMarks.join(), `${c.name}: before the order the HUD says who it will go to`, `"${st?.label}" in ${st?.tone} · ${(st?.opts ?? []).slice(0, 4).map((o: any) => o.text).join(' | ')}`);
    const wantMen = c.want.map(i => ops[i]), rest = ops.filter((_: any, i: number) => !c.want.includes(i));
    const fallen = !!START.fallIn, was = rest.map((m: any) => O.of(m)), keeps = () => rest.every((m: any, i: number) => kept(O, m, was[i]));
    c.give(R); const l = R.last(); const menuGone = O.menu === null;
    const took = wantMen.every((m: any) => O.of(m)?.order === 'move'), restFree = keeps();
    R.until(() => wantMen.every((m: any) => O.of(m)?.arrived), 16); R.step(30);
    const got = wantMen.map((m: any) => dist(m.char.pos, SPOT)), stayed = rest.map((m: any) => dist(m.char.pos, p0[ops.indexOf(m)]));
    verdict(l?.order === 'move' && l.who.join(' + ') === names(wantMen) && O.log.length === 1 && menuGone && took && restFree && keeps(),   // (… as the order is given, and still once it is carried out)
      `${c.name}: "move there" goes to ${names(wantMen)} and to nobody else (${c.how})`, `${line(l)} · orders logged ${O.log.length} · the others: ${rest.length ? rest.map((m: any, i: number) => `${m.callsign} ${O.has(m) ? (fallen && kept(O, m, was[i]) ? 'keeps the fall in he started on' : 'HAS an order') : 'no order'}`).join(', ') : '—'}`);
    // (the rest, fallen in: they close up on Sam as the file re-forms without the men who left it — a step or so, he is standing still — so the promise there is
    //  each man on his own mark in the file and none of them inside the spot's spread; with no order they have nowhere to go: under 0.9 m from where they stood)
    const away = rest.map((m: any) => dist(m.char.pos, SPOT)), withSam = rest.every((m: any, i: number) => inFile(O, m) && away[i] > 3.55);
    verdict(got.every(d => d < 3.55) && (fallen ? withSam : stayed.every(d => d < 0.9)),   // (roomfit.ts: marks round the spot, within its spread for four)
      fallen ? `${c.name}: they go to the spot; the rest stay with Sam, each on his mark in the file and none of them at the spot` : `${c.name}: they go to the spot; the rest stay where they stood`,
      `off the spot: ${wantMen.map((m: any, i: number) => `${m.callsign} ${f2(got[i])} m`).join(', ')} · moved: ${rest.length ? rest.map((m: any, i: number) => `${m.callsign} ${f2(stayed[i])} m`).join(', ') : '—'}`
        + (fallen && rest.length ? ` · in the file: ${rest.map((m: any, i: number) => `${m.callsign} ${inFile(O, m) ? 'on his mark' : 'OFF his mark'}, ${f2(away[i])} m from the spot`).join(', ')}` : ''));
    verdict(R.held === 0, `${c.name}: no key or button was down on any frame`, `most keys + buttons down at once, over ${Math.round(R.game.time * 60)} frames: ${R.held}`);
  }
  return done();
});

// ---------------------------------------------------------------- what the squad block says with everyone addressed (the page's Q ring; 2026-11-07)
/** ORDERBLOCK_BREAK=1, the red path: the order of the second line is never given, so no row can come to say it */
export const orderuiBlock = guarded(async () => {
  const { verdict, done } = family('orderui-block: with everyone addressed the squad block names each man in his pair\'s colour with what he is doing');
  const BREAK = process.env.ORDERBLOCK_BREAK === '1', K = ORDERS;
  const R = await rig(22); const ops = R.ops;
  const txt = (st: any) => st ? `"${st.label}" (${st.tone}) · ${st.opts.slice(0, 4).map((o: any) => `${o.text}${o.tone ? ` (${o.tone})` : ''}`).join(' | ')}` : 'no block';
  const rows = (st: any): any[] => (st?.opts ?? []).slice(0, 4), named = (r: any[]) => r.length === 4 && r.every((o: any, i: number) => o.text.startsWith(`${ops[i].callsign} · `) && o.text.length > `${ops[i].callsign} · `.length);
  const tones = ops.map((m: any) => /^Red/.test(m.callsign) ? 'red' : 'blue').join();
  R.cursor(SPOT); R.step(2); const s0 = R.status(), r0 = rows(s0);
  verdict(ops.length === 4 && s0?.label === head('GOLD') && s0.tone === 'gold' && named(r0) && r0.map((o: any) => o.tone).join() === tones,
    'no ring up, everyone addressed: the header says the orders go to GOLD, in gold, and each of the four has a row in his pair\'s colour that names him and says what he is doing', txt(s0));
  const was = r0.map((o: any) => o.text);
  if (!BREAK) { R.cursor(SPOT); menuKey(R.O ?? R.game.orders); R.step(20); R.press('Digit1'); } R.step(30);
  const s1 = R.status(), r1 = rows(s1), l = R.last();
  verdict(l?.order === 'move' && l.who.length === 4 && named(r1) && r1.every((o: any, i: number) => o.text !== was[i]) && r1.map((o: any) => o.tone).join() === tones && s1?.label === head('GOLD'),
    '"move there" given to everyone: each man\'s row now says something else than it did, in the same colour, and the header still says GOLD', `${line(l)} · before: ${was.join(' | ')} · after: ${txt(s1)}`);
  return done();
});

// ---------------------------------------------------------------- a door tool to one half (the page's Q ring; 2026-11-07)
/** ORDERHALF_BREAK=1, the red path: the key is kept down WITHOUT Shift, so the ring is everyone's and the door order is all four's */
export const orderuiHalf = guarded(async () => {
  const { verdict, done } = family('orderui-half: a door tool given to one half of the squad is worked by that half alone');
  const BREAK = process.env.ORDERHALF_BREAK === '1';
  const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`); const KEY: string = CMDRING.key;
  const R = await rig(23, { sam: SAM_W }); const { O, D, input: I, game: g } = R; const ops = R.ops, d = R.door('breakroom_w'), sam = g.player.char;
  // the cursor where the squad's key reads that door
  const c = d.frameCentre, th = d.def.closedDir; let on = false;
  search: for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { R.cursor([c[0] - Math.sin(th) * k, y, c[2] + Math.cos(th) * k]); R.step(2); const t = O.targetUnder(I, R.W.cam, R.W.canvas); if (t.kind === 'door' && t.door === d) { on = true; break search; } }
  const was = new Map<any, any>(ops.map((m: any) => [m, O.of(m)])), side0 = sideOf(d, sam.pos);
  // Shift and the key kept down until the ring is up: it is one whole pair's (the nearer); its 'open and clear' wedge by its digit; then every key let go
  if (!BREAK) I.keys.add('ShiftLeft'); I.keys.add(KEY); I.pressed.add(KEY); const ring = R.until(() => !!O.cmdRing, 1.5);
  const part: any[] = [...(O.menu?.part ?? [])], pair = part.length === 2 && O.pairOf(part[0]) !== null && O.pairOf(part[0]) === O.pairOf(part[1]);
  const pick = (re: RegExp): string | null => { const x = (O.g.view.ring?.wedges ?? []).find((y: any) => re.test(y.text)); if (!x) return null; I.pressed.add(`Digit${x.key}`); R.step(); R.step(3); return x.text; };
  const wedges = (O.g.view.ring?.wedges ?? []).map((y: any) => `${y.key} ${y.text}`).join(' | '), w = ring ? pick(/^open and clear/) : null; I.keys.clear(); R.step(4); if (O.cmdRing) { O.cmdRing = null; O.close(); R.step(2); }
  const l = R.last(), op = D.ops[0] ?? null, rest = ops.filter((m: any) => !part.includes(m));
  verdict(on && ring && pair && !!w && l?.order === 'openClear' && l.who.join(' + ') === names(part) && !!op && op.men.length === 2 && op.men.every((m: any) => part.includes(m)),
    'Shift and the squad\'s key kept down on a door: the ring is one whole pair\'s; its "open and clear" wedge: the order is that pair\'s, and the door order that starts has that pair in it and nobody else',
    `the cursor on the door ${on} · the ring ${ring ? 'up' : 'NOT up'} for ${names(part) || 'nobody'} · the wedges: ${wedges} · given '${w}' · ${line(l)} · the door order's men: ${op ? names(op.men) : 'no door order'}`);
  // worked by that half alone: the other two keep the order they had and are on no frame across the door
  let other = 0, across = 0, inOp = 0;
  R.post = () => { for (const m of rest) { if (O.of(m) !== was.get(m)) other++; if (sideOf(d, m.char.pos) !== side0) across++; } for (const x of D.ops) if (rest.some((m: any) => x.men.includes(m))) inOp++; };
  const ended = !!op && R.until(() => !D.ops.includes(op), 60); R.step(60); R.post = null;
  const inRoom = ops.map((m: any) => m.state !== 'dead' && sideOf(d, m.char.pos) !== side0);
  verdict(ended && op.ended === 'in' && part.length === 2 && part.every((m: any) => inRoom[ops.indexOf(m)]) && rest.length === 2 && other === 0 && across === 0 && inOp === 0,
    'that pair works the door and goes in; the other two were never in a door order, keep the order they had, and are on no frame on the room\'s side of the door',
    `ended '${op?.ended ?? '—'}' · through the door: ${ops.map((m: any, i: number) => `${m.callsign} ${inRoom[i] ? 'in' : 'out'}`).join(', ')} · the other two (${names(rest) || 'nobody'}): frames on another order ${other}, frames across the door ${across}, frames in a door order ${inOp}`);
  return done();
});

// ---------------------------------------------------------------- the frame's input while the menu is up
