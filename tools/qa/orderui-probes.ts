// The 'orderui' family (game/orders.ts — the squad's command menu as a PRESS menu, and WHO an order goes to with four teammates in two pairs: NIGHTBREACH.squad
// = 4, the page's ?squad4), run through probes.ts:
//   bun run tools/qa/probes.ts orderui | orderui-four | orderui-keys
// PASS / FAIL each, all headless (the Game of headless.ts under the room clear's staging, Sam in god mode and planted dark, the defenders kept dazzled: this
// family is about who is ordered and how, not the fight). Every order is given THROUGH THE INPUT as a keyboard and mouse would write it — the harness cursor on a
// world point, then one-frame presses of X / the middle button / T / B / a digit / the wheel / a click / Space — and the rig counts the keys and buttons DOWN on
// every frame: where a probe says "nothing held" that count never left zero. Nothing here looks at a picture: the colours, the ring on the target and how the
// menu reads on the page are checked as data, never drawn.
//   orderui-four   'move there' on one spot given to the squad (X, then its digit): every man takes it and goes, nothing held; the HUD named who before the
//                  order was given
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
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
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
    status(): any { return O.hud().status[0] ?? null; },
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
      address: () => {}, give: R => { R.cursor(SPOT); R.press(K.key); R.step(20); R.press('Digit1'); } },
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

// ---------------------------------------------------------------- the frame's input while the menu is up
export const orderuiKeys = guarded(async () => {
  const { verdict, done } = family('orderui-keys: what the menu takes of the frame\'s input, and what it never takes');
  const R = await rig(25); const { O, game: g, input, W } = R; const K = ORDERS; const sam = g.player.char;
  // Sam walks on with the menu up (movement keys are his, not the menu's): it stays, frozen on its target
  R.cursor(SPOT); R.press(K.key); R.step(5); const t0 = O.menu?.target, at0 = [...sam.pos];
  let walked = 0; for (const code of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) { const from = [...sam.pos]; input.keys.add(code); R.step(40); input.keys.delete(code); R.step(10); walked = Math.max(walked, dist(sam.pos, from)); }
  const stillUp = !!O.menu && O.menu.target === t0 && O.menu.target.kind === 'floor' && dist(O.menu.target.at, SPOT) < 0.3;
  verdict(walked > 0.5 && stillUp && O.log.length === 0, 'Sam walks with the menu up (W A S D held in turn): it stays up, on the spot it was opened on, and no order is given', `Sam moved up to ${f2(walked)} m a key, ${f2(dist(sam.pos, at0))} m in all · menu up ${!!O.menu} on ${O.menu?.target.kind} · orders logged ${O.log.length}`);
  // a digit with no row: nothing, and not his weapon slot; B: nothing while it is up
  const slot0 = g.player.slot; R.press('Digit7'); const up7 = !!O.menu; R.press(K.quickKey); const upB = !!O.menu;
  verdict(up7 && upB && O.log.length === 0 && g.player.slot === slot0, 'with the menu up a digit that has no row does nothing (and is not Sam\'s slot), and the quick key is dead', `after 7: up ${up7} · after B: up ${upB} · orders logged ${O.log.length} · slot ${slot0} → ${g.player.slot}`);
  // the left button is Sam's trigger with the menu up: the menu goes away, no order, he fires
  const gun = g.player.gunInHand(), n0 = gun?.shotsFired ?? NaN;
  input.clicked |= 1; input.buttons |= 1; R.step(); input.buttons &= ~1; R.step(20); const n1 = gun?.shotsFired ?? NaN, gone1 = O.menu === null, log1 = O.log.length;
  verdict(!!gun && gone1 && log1 === 0 && n1 === n0 + 1, 'a left click with the menu up is Sam\'s shot: the menu goes away, no order is given, a round is fired', gun ? `menu gone ${gone1} · orders logged ${log1} · rounds fired ${n0} → ${n1}` : 'Sam has no gun in hand: the check did not run');
  // a right click is used up by the menu
  R.cursor(SPOT); R.press(K.key); R.step(5); input.clicked |= 4; O.input(DT, input, W.cam, W.canvas); const rGone = O.menu === null, rBit = (input.clicked & 4) !== 0; R.step();
  verdict(rGone && !rBit, 'a right click with the menu up closes it and is taken out of the frame (not the OCP, not the ring)', `menu gone ${rGone} · the click still in the frame's input: ${rBit}`);
  // the frame it opens on is not the menu's; two digits in one frame are one order and no slot
  const nA = O.log.length; R.cursor(SPOT); input.pressed.add(K.key); input.pressed.add('Space'); R.step(); const upA = !!O.menu, logA = O.log.length - nA; R.step(10);
  const slotB = g.player.slot; input.pressed.add('Digit4'); input.pressed.add('Digit3'); R.step(); const lB = R.last(), logB = O.log.length - nA;
  verdict(upA && logA === 0 && O.menu === null && logB === 1 && lB?.order === 'fallIn' && g.player.slot === slotB, 'X and Space in one frame: the list comes up and nothing is given; then 3 and 4 in one frame: one order (the lower digit), and neither is Sam\'s slot', `on the press: up ${upA}, orders ${logA} · two digits: ${line(lB)} (${logB} order) · slot ${slotB} → ${g.player.slot}`);
  // the men a menu was for go down: the menu goes, the order never lands on everybody else
  // (blue is nobody's here: with START.fallIn it holds the fall in it started on throughout — the same assignment at the end, never the digit's order)
  { const R2 = await rig(26); const O2 = R2.O, ops = R2.ops; const was2 = [2, 3].map(i => O2.of(ops[i]));
    R2.on(ops[0]); R2.press(K.key); R2.step(5); const up = O2.menu?.men.length === 1;
    killGuard(R2.game, ops[0], [1, 0, 0], false, 'shot', 'player'); R2.step(2); R2.press('Digit4'); const a = { up: !!O2.menu, log: O2.log.length }; R2.step(70); const gone = O2.menu === null;
    R2.cursor(SPOT); R2.press(K.whoKey); const who = O2.who; R2.press(K.key); R2.step(5); const up2 = !!O2.menu;
    killGuard(R2.game, ops[1], [1, 0, 0], false, 'shot', 'player'); R2.step(2); const gone2 = O2.menu === null, who2 = O2.who; R2.press('Digit1'); R2.step(5);
    verdict(up && a.log === 0 && gone && who === 'red' && up2 && gone2 && who2 === 'all' && O2.log.length === 0 && [2, 3].every((i, k) => kept(O2, ops[i], was2[k])),
      'a menu opened on Red One, he goes down, 4 is pressed: nobody is ordered and the menu is gone; red addressed with one man left, he goes down: the menu goes at once and the digit orders nobody', `on him: up ${up} · after his death + 4: orders ${a.log}, menu ${gone ? 'gone' : 'up'} · red's menu up ${up2} → after the last red man: ${gone2 ? 'gone' : 'up'}, who ${who2} · orders logged ${O2.log.length} · blue ordered: ${START.fallIn ? `${[2, 3].some((i, k) => !kept(O2, ops[i], was2[k]))} (each still on the fall in he started on: ${[2, 3].map(i => `"${O2.statusOf(ops[i])}"`).join(', ')})` : [2, 3].some(i => O2.has(ops[i]))}`);
  }
  // the one pair: T on a man, T on him again = all (never round to the other man and back)
  { const R3 = await rig(27, { squad: 2 }); const O3 = R3.O, ops = R3.ops; const seq: any[] = [];
    R3.on(ops[0]); for (let i = 0; i < 4; i++) { R3.press(K.whoKey); seq.push(O3.who); } const tLine = R3.status()?.opts.find((o: any) => o.key === 'T')?.text;
    R3.press(K.whoKey); const tLine2 = R3.status()?.opts.find((o: any) => o.key === 'T')?.text;
    verdict(ops.length === 2 && seq.join() === '0,all,0,all' && tLine === 'who · Blue One (under the cursor)' && tLine2 === 'who · back to all', 'the pair, the cursor resting on Blue One: T is him, T again is all — and the HUD\'s T line says which it will be', `who after each T: ${seq.join(' ')} · T line "${tLine}" → "${tLine2}"`);
  }
  // a restart with it up
  NIGHTBREACH.squad = 4;   // (the pair's rig above set the knob: the restart below stages what it says)
  const nLog = O.log.length; R.cursor(SPOT); R.press(K.whoKey); R.press(K.key); R.step(5); const upR = !!O.menu, whoR = O.who; g.restartEncounter(); R.step(3);
  verdict(upR && whoR === 'red' && O.menu === null && O.who === 'all' && R.ops.length === 4 && O.log.length <= nLog, 'a restart with the menu up and red addressed: the menu is gone and the choice is back to everyone', `before: up ${upR}, who ${whoR} · after: menu ${O.menu ? 'up' : 'gone'}, who ${O.who}, ${R.ops.length} teammates`);
  return done();
});
