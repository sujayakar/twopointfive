// The 'squadorders' family (game/orders.ts — Sam's live orders to the AI squad: the quick key = the target's own order at once), run through probes.ts:
//   bun run tools/qa/probes.ts squadorders | orders-off | orders-move | orders-cover | orders-stack | orders-fallin | orders-nightbreach | orders-seams | orders-edges   [--verbose]
// PASS / FAIL each, all headless (the Game of headless.ts). The squad is the night breach's pair (game.nightBreach.start(), as ?nightbreach stages it) unless a
// probe says otherwise; orders are given through the quick key with the harness cursor on a world point, or give() where the row has no key. The defenders in the break
// room are kept dazzled (and blind fire off) wherever a probe is about WHERE the men go: this family is not about the fight. Sam's visibility is planted dark,
// as in the nightbreach family (no light meter headless).
// NOT held here: how the menu reads and feels on the page, the men's carriage as a picture, sound.
//   orders-off         no squad on the floor: the keys are dead (ten seeded seconds digest the same with and without them), no menu, no HUD block, give() refuses
//   orders-move        move there: both men on marks round the point, 'set.' called once; hold position mid-run stops them where they are and they stay
//   orders-cover       cover that way / this door: each man turns his gun onto it where he stands, and stays on it
//   orders-stack       stack up, two cases (the break room's west door, a man on each jamb; its north door from inside, both in file on the free one): off the opening,
//                      on the side Sam is on, and they stay; 'set.' called once
//   orders-fallin      fall in: the pair closes up; Sam walks on his own feet from the cubicles' aisle through the west door, across the break room and out of its north
//                      door into the corridor: both men stay within a few metres the whole way and end beside him on the far side; turning on the spot does not move them
//   orders-nightbreach the two systems together: an order while the pair is set on the breach calls the breach off (no kick), F stacks them again; once the
//                      breach is going an order is refused and the breach runs to 'clear'; a recall once they are inside ends the drill ('recalled'); an order that
//                      cannot be carried out never calls the breach off
//   orders-seams       what the later parts build on: a man the fight AI has (script ended, alert, eyes on) is left to it on his post and taken back once cold; a show
//                      taking the floor drops every order
//   orders-edges       what cannot be done is refused aloud; the men keep clear of Sam; the file closes up; the fight seam's clock
import { blockOf, doing, listedOf, ringUp, ringDown, sameWord } from './squadwords';
import { standUp, ROOT } from './headless.ts';
import { CMDRING } from '../../src/game/cmdring';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS, doorWords } = await import(`${ROOT}/src/game/orders.ts`);   // (doorWords: a door's name as the HUD says it — its id's words, or the rooms it joins where the door names rule is on — so the expected words follow the stand)
const { WORDS } = await import(`${ROOT}/src/game/words.ts`);
const { DOORSTAY, DOORWAY } = await import(`${ROOT}/src/game/doors.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { endGuardScript } = await import(`${ROOT}/src/game/squad.ts`);
const { START } = await import(`${ROOT}/src/game/startfile.ts`);

const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${f2(p[0])}, ${f2(p[2])})`;
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
const yawTo = (a: number[], b: number[]) => Math.atan2(b[0] - a[0], b[2] - a[2]);

/** One game under a seed. `nb`: the night breach staged (its pair = the squad). `quiet`: the defenders kept dazzled every frame (they neither see nor fire). */
async function rig(seed: number, o: { nb?: boolean; quiet?: boolean; knobs?: Record<string, unknown> } = {}) {
  Object.assign(NIGHTBREACH, NB_DEFAULTS, o.knobs ?? {});
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, NB = game.nightBreach, O = game.orders, input = W.input;
  const why = o.nb === false ? null : NB.start();
  const R = { W, game, NB, O, input, why, quiet: o.quiet !== false, samHit: false, said: [] as { who: string; text: string; t: number }[],
    /** run before every frame (Sam's walk) */
    pre: null as null | (() => void),
    step(n = 1) {
      for (let i = 0; i < n; i++) {
        game.player.visibility = 0.05;
        if (R.quiet) for (const h of NB.hostiles) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
        R.pre?.();
        W.step(DT);
        for (const gd of game.guards) { const b = gd.bubble, k = b ? `${b.text}@${b.t}` : ''; if (b && heard.get(gd.callsign) !== k) { heard.set(gd.callsign, k); R.said.push({ who: gd.callsign, text: b.text, t: game.time }); } }   // (a line is known by its words and its time: the copy's bubble object is the same one from line to line)
        if (game.player.down || game.player.hitFlash > 0) R.samHit = true;
      }
    },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** the reticle on a world point (the harness cursor → the game's aim ray on the next frame) */
    aim(p: number[]) { return W.cursorAt([p[0], p[1], p[2]]); },
    /** a one-frame key (a tap) */
    key(code: string) { input.pressed.add(code); R.step(); },
    /** the quick key pressed with the cursor on p: the target's own order, at once */
    quick(p: number[]) { R.aim(p); input.pressed.add(ORDERS.quickKey); R.step(); },
    /** X PRESSED (one frame — nothing is held) with the cursor on p: the menu comes up there; then `secs` more of frames with every key up */
    openX(p: number[], secs = 0.4) { R.aim(p); ringUp(R.input, R); const n = Math.round(secs * 60) - 1; for (let i = 0; i < n; i++) R.step(); },
    /** Esc pressed: the menu goes down with no order */
    esc() { input.pressed.add('Escape'); R.step(); },
    /** a left click as the page's mouse writes it: the press bit for one frame, the held bit until the button comes up a few frames on */
    click() { input.clicked |= 1; input.buttons |= 1; R.step(); input.buttons &= ~1; R.step(); },
    /** press X on p, then the row's digit a third of a second later (X long since up): the n-th row — more digits walk the menu's pages (5, 1 = the door
     *  tools' page, its first row) */
    pick(p: number[], ...n: number[]) { R.openX(p, 0.35); for (const k of n) { input.pressed.add(`Digit${k}`); R.step(); } },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    lines(text: RegExp, since = 0) { return R.said.filter(s => s.t >= since && text.test(s.text)); },
  };
  const heard = new Map<string, string>();
  return R;
}
/** every probe leaves the knob tables as it found them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); } };
const lastOrder = (O: any) => O.log[O.log.length - 1] ?? null;
const logLine = (l: any) => l ? `${l.order} → ${l.who.join(' + ')} on ${l.target}` : 'nothing';

// open floor on the east–west aisle that runs from the pair's start to the break room's west door, in plain view of the camera (nothing between it and the
// floor there: orders-move checks the reticle's point is the point asked for) — and the same for a spot back west of the pair, in the aisle they start in
const AISLE: number[] = [15, 0, 17.6], AISLE_FAR: number[] = [21, 0, 17.6], WEST: number[] = [7.5, 0, 17.6];

export const ordersOff = guarded(async () => {
  const { verdict, done } = family('orders-off: with no squad on the floor X, B, T and the middle button are dead');
  const run = async (keys: boolean) => { const R = await rig(5, { nb: false }); Math.random = seeded(77);
    for (let i = 0; i < 600; i++) { if (keys) { if (i === 100) { R.input.keys.add(CMDRING.key); R.input.pressed.add(CMDRING.key); } if (i === 125) R.input.keys.delete(CMDRING.key); if (i === 160) R.input.pressed.add(ORDERS.whoKey); if (i === 220) R.input.pressed.add(ORDERS.quickKey); if (i === 280) R.input.clicked |= 2; if (i === 340) ringDown(R.input); } R.step(); }
    return R; };
  const A = await run(false), B = await run(true); const O = B.O;
  const hud = { menu: listedOf(O), status: blockOf(O) };
  verdict(!O.active && O.menu === null && O.log.length === 0 && O.who === 'all' && hud.menu === null && hud.status.length === 0, 'inert: no squad, no menu, no order logged, no HUD block after X, T, B, a middle click and X again',
    `active ${O.active} · menu ${O.menu} · log ${O.log.length} · hud rows ${hud.status.length}`);
  const why = O.give('fallIn');
  verdict(why === 'no squad', 'give() refuses with the reason', `give('fallIn') → ${why}`);
  const da = JSON.stringify(digestGame(A.game, true)), db = JSON.stringify(digestGame(B.game, true));
  verdict(da === db, 'ten seeded seconds digest the same with and without the keys', `digest ${da.length} chars · ${da === db ? 'equal' : 'DIFFERENT'}`);
  return done();
});


export const ordersMove = guarded(async () => {
  const { verdict, done } = family('orders-move: move there puts the pair on the spot, hold stops them where they are');
  const R = await rig(11); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2]; const sam = g.player.char;
  R.step(30);
  const from = men.map((m: any) => [...m.char.pos]); const t0 = g.time;
  // the staging the step-off line was written on: each man stands facing the way he is about to be sent, as the pair does where a staging leaves it with no
  // order. Where the team starts fallen in on Sam (START.fallIn, as the page has it) they stand facing wherever the file left them, and a man walks along his
  // body while it comes round (guards.ts followPath), so the same third of a second measured his turn, not his lean into the move. Staged as the Director
  // stages a man's yaw: body and gun written, then a dt-0 update so that the yaw he is drawn at (character.ts TURN) is the one written and not the old one
  // coming round. Not his order, his place or a table; with the switch off nothing here runs.
  const stood = men.map((m: any) => m.char.bodyYaw);
  if (START.fallIn) for (const m of men) { const c = m.char; c.bodyYaw = c.aimYaw = yawTo(c.pos, AISLE); c.update(0); }
  R.quick(AISLE); const aimed = [...g.aimPoint];
  const ack = R.said.filter(s => s.t >= t0);
  R.step(2); const first = men.map((m: any, i: number) => dist(m.char.pos, from[i]));   // (the order's own tick and two more: he is already off his spot)
  R.step(18); const early = men.map((m: any, i: number) => dist(m.char.pos, from[i]));
  const set = R.until(() => men.every((m: any) => O.of(m)?.arrived), 12); const tSet = g.time - t0;
  const face = yawTo(sam.pos, AISLE);
  const dPost = men.map((m: any) => dist(m.char.pos, O.of(m).post)), dAt = men.map((m: any) => dist(m.char.pos, AISLE)), apart = dist(b1.char.pos, b2.char.pos);
  const dYaw = men.map((m: any) => Math.abs(wrap(m.char.bodyYaw - face)));
  verdict(dist(aimed, AISLE) < 0.1 && lastOrder(O)?.order === 'move' && lastOrder(O).target === `floor:${AISLE[0].toFixed(1)},${AISLE[2].toFixed(1)}`, 'the cursor on a floor point, the quick key: "move there", to that point', `reticle ${at(aimed)} for ${at(AISLE)} · ${logLine(lastOrder(O))}`);
  verdict(ack.length === 1 && /moving|on it/.test(ack[0].text), 'the order is acknowledged in a word by one man', `said: ${ack.map(s => `${s.who} "${s.text}"`).join(', ') || 'nothing'}`);
  verdict(early.every(d => d > 0.1) && first.every(d => d > 0), 'they step off at once (a third of a second on, both are moving — a man leans into a move now, guards.ts PACE.accel: it was a quarter metre gone by then)', `moved after 0.33 s: ${early.map(f2).join(' / ')} m${START.fallIn ? ` · off his spot three ticks in ${first.map(d => (d * 1000).toFixed(1)).join(' / ')} mm · each turned to the heading first (they stood fallen in, at ${stood.map(f2).join(' / ')} rad)` : ''}`);
  verdict(set && dPost.every(d => d < 0.35) && dAt.every(d => d < 2.55) && apart > 0.8, 'both arrive: each on his own mark round the point, a body\'s room and more apart (roomfit.ts deals the marks from the room; it was two abreast at the point)', `in ${f2(tSet)} s · off his spot ${dPost.map(f2).join(' / ')} m · off the point ${dAt.map(f2).join(' / ')} m · apart ${f2(apart)} m · ${men.map((m: any) => at(m.char.pos)).join(' ')}`);
  R.step(60);
  verdict(men.every((m: any) => { const A = O.of(m); return !!A.watch && Math.abs(wrap(m.char.aimYaw - yawTo(m.char.pos, A.watch))) < 0.3; }) && men.every((m: any) => doing(O.g, m).startsWith('in position · ')), 'each has his gun on what the room dealt him and the HUD says "in position" and what he is on (it was: every man facing the way they were sent)', `off the heading ${dYaw.map(f2).join(' / ')} rad · ${men.map((m: any) => doing(O.g, m)).join(' / ')}`);
  const sets = R.lines(/^set\.$/, t0);
  verdict(sets.length === 1, '"set." is called once, when both are on their marks', `${sets.map(s => `${s.who} @${f2(s.t - t0)} s`).join(', ') || 'never'}`);
  const here = men.map((m: any) => [...m.char.pos]); R.step(60 * 6);
  const drift = men.map((m: any, i: number) => dist(m.char.pos, here[i]));
  verdict(drift.every(d => d < 0.1) && R.lines(/^set\.$/, t0).length === 1, 'six seconds on they have not shifted, and nobody said it again', `drift ${drift.map(f2).join(' / ')} m`);
  // hold, mid-run
  R.quick(AISLE_FAR); R.step(55);
  const mid = men.map((m: any) => [...m.char.pos]); const speed = men.map((m: any) => m.speed);
  R.pick(sam.pos, 4);   // the reticle on Sam himself: fall in (3) / hold (4) — the digits are the same on every target
  const held = lastOrder(O); const atOrder = men.map((m: any) => [...m.char.pos]);
  R.step(60 * 2); const after2 = men.map((m: any, i: number) => dist(m.char.pos, atOrder[i]));
  R.step(60 * 6); const after8 = men.map((m: any, i: number) => dist(m.char.pos, atOrder[i])); const toFar = men.map((m: any) => dist(m.char.pos, AISLE_FAR));
  verdict(held?.order === 'hold' && speed.every((s: number) => s > 1) && after2.every(d => d < 0.6) && after8.every(d => d < 0.6) && toFar.every(d => d > 1.5) && men.every((m: any) => doing(O.g, m).startsWith('holding') && m.speed < 0.1),
    'hold position while they run to a far spot: both stop within a stride of where they were and stay there', `${logLine(held)} · speed at the order ${speed.map(f2).join(' / ')} m/s · ${mid.map(at).join(' ')} · off it after 2 s ${after2.map(f2).join(' / ')} m, after 8 s ${after8.map(f2).join(' / ')} m · still ${toFar.map(f2).join(' / ')} m short of the far spot`);
  verdict(!R.samHit && R.NB.state === 'staged', 'Sam untouched, the breach still only staged', `hit ${R.samHit} · night breach ${R.NB.state}`);
  return done();
});

export const ordersCover = guarded(async () => {
  const { verdict, done } = family('orders-cover: cover turns a man onto the direction or the door, and he stays on it');
  const R = await rig(11); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2];
  R.step(30); R.quick(AISLE); R.until(() => men.every((m: any) => O.of(m)?.arrived), 12); R.step(30);
  const offAim = (m: any, p: number[]) => Math.abs(wrap(m.char.aimYaw - yawTo(m.char.pos, p)));
  const cover = (label: string, p: number[], row: number, wantWords: RegExp, ackSince: number) => {
    const here = men.map((m: any) => [...m.char.pos]); const before = men.map((m: any) => offAim(m, p));
    O.give('cover', O.targetAt(p)); R.step(); const l = lastOrder(O); const watch = men.map((m: any) => O.of(m)?.watch ?? [NaN, NaN, NaN]);
    const on = R.until(() => men.every((m: any) => O.of(m)?.arrived), 4); const tOn = g.time - l.t;
    const off = men.map((m: any) => offAim(m, p)), moved = men.map((m: any, i: number) => dist(m.char.pos, here[i]));
    let worst = 0; for (let i = 0; i < 60 * 8; i++) { R.step(); for (const m of men) worst = Math.max(worst, offAim(m, p)); }
    const moved8 = men.map((m: any, i: number) => dist(m.char.pos, here[i]));
    const words = men.map((m: any) => doing(O.g, m)); const up = men.map((m: any) => m.script?.upper); const body = men.map((m: any) => Math.abs(wrap(m.char.bodyYaw - yawTo(m.char.pos, p))));
    verdict(l.order === 'cover' && watch.every((w: number[]) => dist(w, p) < 0.1) && before.every(b => b > 0.5) && on && off.every(d => d < 0.2) && moved.every(d => d < 0.3), `${label}: each man turns his gun onto it where he stands`,
      `${logLine(l)} · watching ${at(watch[0])} for ${at(p)} · off it before ${before.map(f2).join(' / ')} rad → ${off.map(f2).join(' / ')} rad in ${f2(tOn)} s · moved ${moved.map(f2).join(' / ')} m`);
    verdict(worst < 0.2 && body.every(d => d < 0.4) && moved8.every(d => d < 0.3) && words.every((w: string) => wantWords.test(w)) && up.every((u: string) => u === 'aim'), `${label}: eight seconds on he is still on it — gun and body — and the HUD says where`,
      `worst off it ${f2(worst)} rad · body off it ${body.map(f2).join(' / ')} rad · moved ${moved8.map(f2).join(' / ')} m · upper ${up.join(' / ')} · "${words.join('" / "')}"`);
    const ack = R.said.filter(s => s.t >= ackSince && /covering|got it/.test(s.text));
    return ack;
  };
  const tA = g.time;
  const ackA = cover('cover that way (a floor point behind them, to the west)', WEST, 2, WORDS.on ? /^covering (?!.*\b(north|south|east|west)\b)\S/ : /^covering west$/, tA);   // (the dialogue pass on: the place by its name or 'that way', never a bearing — a row on the HUD has no compass)
  const tB = g.time;
  const W = R.door('breakroom_w');
  const ackB = cover('cover this door (the break room\'s west door)', W.frameCentre, 2, new RegExp(`^covering ${doorWords(W)}$`), tB);
  verdict(ackA.length === 1 && ackB.length === 1, 'each cover order is acknowledged once', `${[...ackA, ...ackB].map(s => `${s.who} "${s.text}"`).join(', ')}`);
  verdict(!R.samHit, 'Sam was not hit', `hit ${R.samHit}`);
  return done();
});

export const ordersStack = guarded(async () => {
  const { verdict, done } = family('orders-stack: stack up puts the men on the door\'s jambs, on Sam\'s side');
  // three doors, each from the side Sam is put on: [door, where Sam stands, where the pair starts]
  // [door, where Sam is put, the words for that side, both men in file on one jamb?]
  const cases: [string, number[], string, boolean][] = [['breakroom_w', [24.5, 0, 17.6], 'the cubicles\' side', false],
    ['breakroom_n', [30.75, 0, 14.4], 'inside the break room (the counter runs off the hinge side: no floor at that jamb)', true]];
  let k = 0;
  for (const [name, samAt, sideWords, file] of cases) {
    const R = await rig(11 + k++); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2];
    R.step(30); g.teleportPlayer(samAt); R.step(40);
    const d = R.door(name); const th = d.def.closedDir, u = [Math.cos(th), Math.sin(th)], H = d.def.hinge, w = d.def.width;
    const along = (p: number[]) => (p[0] - H[0]) * u[0] + (p[2] - H[1]) * u[1];           // metres down the closed leaf's line from the hinge: the opening is 0‥w
    const off = (p: number[]) => -(p[0] - H[0]) * u[1] + (p[2] - H[1]) * u[0];            // signed distance off the wall line
    const samSide = Math.sign(off(g.player.char.pos)); const t0 = g.time;
    R.quick(d.frameCentre); const l = lastOrder(O);
    const set = R.until(() => men.every((m: any) => O.of(m)?.arrived), 25); const tSet = g.time - t0;
    R.step(30);
    const a = men.map((m: any) => along(m.char.pos)), o = men.map((m: any) => off(m.char.pos) * samSide);
    const clear = a.every(x => x < -0.15 || x > w + 0.15), jamb = a.map(x => x < 0 ? -x : x - w);   // how far past the nearer jamb
    const split = (a[0] < 0) !== (a[1] < 0);
    const dPost = men.map((m: any) => O.of(m) ? dist(m.char.pos, O.of(m).post) : NaN);
    verdict(l?.order === 'stack' && l.target === `door:${name}` && set && dPost.every(x => x < 0.35), `${name} #${k}: the quick key on the door from ${sideWords} — both men reach their marks`, `${logLine(l)} · in ${f2(tSet)} s · off his mark ${dPost.map(f2).join(' / ')} m · ${men.map((m: any) => at(m.char.pos)).join(' ')}`);
    verdict(clear && split === !file && o.every(x => x > 0.25 && x < 0.9) && jamb.every(x => x < (split ? 0.8 : 1.7)) && dist(b1.char.pos, b2.char.pos) > 0.6, `${name}: ${file ? 'in file on the one free jamb' : 'a man on each jamb'} — out of the opening, against the wall, on Sam's side of it`,
      `${split ? 'one each side of the door' : 'both in file on one side'} · past the jamb ${jamb.map(f2).join(' / ')} m · off the wall (Sam's side +) ${o.map(f2).join(' / ')} m · opening 0‥${f2(w)}, the men at ${a.map(f2).join(' / ')}`);
    // ('the door untouched': with every leaf on its closer (the harness's older stand) the door they stack on is shut, and stays shut. Where doors stay as they
    //  are left (DOORSTAY.on, as the page has it) a pair that came THROUGH it to reach Sam's side has left it open behind them: there the leaf is where their
    //  walk left it, and what the hold must not do is move it.)
    const leaf0 = DOORSTAY.on ? d.angle : 0;
    const here = men.map((m: any) => [...m.char.pos]); R.step(60 * 6); const drift = men.map((m: any, i: number) => dist(m.char.pos, here[i]));
    const sets = R.lines(/^set\.$/, t0); const words = men.map((m: any) => doing(O.g, m));
    verdict(drift.every(x => x < 0.1) && sets.length === 1 && words.every((s: string) => sameWord(s, `stacked · ${doorWords(d)}`)) && Math.abs(d.angle - leaf0) < 0.05, `${name}: they hold the stack (six seconds, the door untouched), "set." called once, the HUD says "stacked"`,
      `drift ${drift.map(f2).join(' / ')} m · set. ${sets.map(s => `${s.who} @${f2(s.t - t0)} s`).join(', ') || 'never'} · "${words[0]}" · leaf angle ${f2(d.angle)}${leaf0 ? ` (${f2(leaf0)} as the hold began)` : ''}`);
    if (!R.samHit) continue; verdict(false, `${name}: Sam was hit`, '');
  }
  return done();
});

export const ordersFallIn = guarded(async () => {
  const { verdict, done } = family('orders-fallin: fall in keeps the pair with Sam across two rooms and through the doors');
  const R = await rig(11); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2]; const sam = g.player.char;
  R.step(30);
  // away first, so that falling in is a real leg
  R.quick(AISLE_FAR); R.until(() => men.every((m: any) => O.of(m)?.arrived), 12);
  const far = men.map((m: any) => dist(m.char.pos, sam.pos)); const t0 = g.time;
  R.quick(sam.pos); const l = lastOrder(O);
  R.until(() => men.every((m: any) => O.of(m)?.arrived), 12); R.step(60);
  const near = men.map((m: any) => dist(m.char.pos, sam.pos));
  verdict(l?.order === 'fallIn' && far.every(d => d > 6) && near.every(d => d > 0.7 && d < 3.2) && dist(b1.char.pos, b2.char.pos) > 0.6 && men.every((m: any) => doing(O.g, m).startsWith('with you')), 'the quick key on Sam: the pair comes back from a far spot and stops short of him, in file',
    `${logLine(l)} · from ${far.map(f2).join(' / ')} m to ${near.map(f2).join(' / ')} m of Sam in ${f2(g.time - t0 - 1)} s · apart ${f2(dist(b1.char.pos, b2.char.pos))} m`);
  // turning on the spot: the men do not shuffle round him
  const here = men.map((m: any) => [...m.char.pos]);
  for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; R.aim([sam.pos[0] + Math.sin(a) * 4, 0, sam.pos[2] + Math.cos(a) * 4]); R.step(30); }
  const shuffled = men.map((m: any, i: number) => dist(m.char.pos, here[i]));
  verdict(shuffled.every(d => d < 0.15), 'Sam looks all the way round on the spot: the men stay where they stood', `moved ${shuffled.map(f2).join(' / ')} m while his reticle went round the compass`);
  // the walk: the aisle → the west door → across the break room → out of the north door → along the corridor
  const Wd = R.door('breakroom_w'), Nd = R.door('breakroom_n');
  const legs: number[][] = [[26.6, 0, 17.6], [29.6, 0, 17.6], [30.75, 0, 13.6], [30.75, 0, 11.0], [24, 0, 11.0]];
  const path: number[][] = [[sam.pos[0], 0, sam.pos[2]]]; let noPath = '';
  for (const p of legs) { const leg = g.col.nav.findPath(path[path.length - 1], p); if (!leg) { noPath = `no nav path to ${at(p)}`; break; } for (const q of leg) path.push([q[0], 0, q[2]]); path.push(p); }
  let seg = 0, t = 0; const SPEED = 1.55; let walked = 0;
  const worst = [0, 0], sum = [0, 0]; let frames = 0, far4 = 0; let nOpenedAt = NaN, firstManNorthAt = NaN;
  // Where doors open one way (as the page has it) Sam walks the path on his own feet: the walk keys turned onto the next point of the path each frame, his reticle 4 m ahead of him.
  const feet = !!DOORWAY.oneWay; let lastAt = [...sam.pos];
  R.pre = feet ? () => {
    walked += dist(sam.pos, lastAt); lastAt = [...sam.pos];
    while (seg < path.length - 1 && dist(sam.pos, path[seg + 1]) < 0.3) seg++;
    if (seg >= path.length - 1) { for (const k of ['KeyW', 'KeyS', 'KeyA', 'KeyD']) R.input.keys.delete(k); return; }
    const b = path[seg + 1], dx = b[0] - sam.pos[0], dz = b[2] - sam.pos[2], l = Math.hypot(dx, dz) || 1; const { fwd, right } = R.W.cam.planarBasis(), f = (dx * fwd[0] + dz * fwd[2]) / l, r = (dx * right[0] + dz * right[2]) / l;
    for (const [k, v] of [['KeyW', f], ['KeyS', -f], ['KeyD', r], ['KeyA', -r]] as [string, number][]) { if (v > 0.38) R.input.keys.add(k); else R.input.keys.delete(k); }
    R.aim([sam.pos[0] + dx / l * 4, 0, sam.pos[2] + dz / l * 4]);
  } : () => {
    if (seg >= path.length - 1) return;
    let stepLeft = SPEED * DT;
    while (stepLeft > 0 && seg < path.length - 1) { const a = path[seg], b = path[seg + 1], L = dist(a, b); if (L - t <= stepLeft) { stepLeft -= L - t; seg++; t = 0; } else { t += stepLeft; stepLeft = 0; } }
    const a = path[seg], b = path[Math.min(seg + 1, path.length - 1)], L = dist(a, b) || 1, f = seg >= path.length - 1 ? 0 : t / L;
    g.teleportPlayer([a[0] + (b[0] - a[0]) * f, 0, a[2] + (b[2] - a[2]) * f]); walked += SPEED * DT;
  };
  const tWalk = g.time; let nAtSamCross = NaN;
  while (seg < path.length - 1 && g.time - tWalk < 90) {
    const zBefore = sam.pos[2]; R.step();
    if (zBefore >= 12.2 && sam.pos[2] < 12.2) nAtSamCross = Nd.angle;
    men.forEach((m: any, i: number) => { const d = dist(m.char.pos, sam.pos); worst[i] = Math.max(worst[i], d); sum[i] += d; if (d > 4) far4++; if (isNaN(firstManNorthAt) && m.char.pos[2] < 12.0) firstManNorthAt = g.time - tWalk; });
    if (isNaN(nOpenedAt) && Math.abs(Nd.angle) > 0.3) nOpenedAt = g.time - tWalk;
    frames++;
  }
  R.pre = null; const walkSecs = g.time - tWalk;
  if (feet) { for (const k of ['KeyW', 'KeyS', 'KeyA', 'KeyD']) R.input.keys.delete(k); walked += dist(sam.pos, lastAt); }
  R.until(() => men.every((m: any) => O.of(m)?.arrived), 8); R.step(60 * 2);
  const end = men.map((m: any) => dist(m.char.pos, sam.pos));
  const room = (p: number[]) => p[2] < 12.2 ? 'corridor' : p[0] > 28 ? 'break room' : 'cubicles';
  verdict(!noPath && seg >= path.length - 1, 'Sam\'s walk: 1.55 m/s along a nav path from the aisle, in at the west door, across the break room, out at the north door, along the corridor', noPath || `${f2(walked)} m in ${f2(walkSecs)} s · ended ${at(sam.pos)} (${room(sam.pos)})${feet ? ` · on his own feet, W held (${f2(walked / walkSecs)} m/s)` : ''}`);
  verdict(worst.every(d => d < 5) && far4 / (2 * frames) < 0.1, 'both men stay within a few metres of him the whole way', `furthest ${worst.map(f2).join(' / ')} m · mean ${sum.map(s => f2(s / frames)).join(' / ')} m · further than 4 m for ${f2(100 * far4 / (2 * frames))} % of the man-frames`);
  // (3.8 m, was 3.2: Sam stops beside the north door, and a place in the file that falls in its leaf's swing is slid back along the file — doorway.ts `swing`)
  verdict(end.every(d => d < 3.8) && men.every((m: any) => room(m.char.pos) === 'corridor') && men.every((m: any) => m.state !== 'dead'), 'they end beside him on the far side of both doors', `${end.map(f2).join(' / ')} m from Sam · ${men.map((m: any) => `${m.callsign} ${at(m.char.pos)} ${room(m.char.pos)}`).join(' · ')}`);
  verdict(!isNaN(firstManNorthAt) && (Math.abs(nAtSamCross) < 0.3 ? nOpenedAt > 0 : true), 'the north door: ' + (Math.abs(nAtSamCross) < 0.3 ? 'still shut behind Sam (the harness carried him through the leaf) — the men opened it themselves' : 'already open when the men reached it (Sam\'s body pushed it)'),
    `leaf angle as Sam crossed ${f2(nAtSamCross)} · open past 0.3 rad at ${f2(nOpenedAt)} s · first man through at ${f2(firstManNorthAt)} s · west door ${f2(Wd.angle)} rad`);
  verdict(!R.samHit, 'Sam was not hit', `hit ${R.samHit}`);
  if (VERBOSE) console.log('    path ' + path.filter((_, i) => i % 6 === 0).map(at).join(' '));
  return done();
});


export const ordersNightBreach = guarded(async () => {
  const { verdict, done } = family('orders-nightbreach: the order menu and the night breach\'s F hand the pair to each other');
  { // ---- set → an order calls it off → F again → going: refused → clear → fall in
    const R = await rig(11, { quiet: false }); const { game: g, O, NB } = R; const [b1, b2] = O.squad(); const men = [b1, b2]; const sam = g.player.char;
    const beat = (w: string) => { const b = NB.drill?.beats.find((x: any) => x.what === w); return b ? b.t : NaN; };
    R.step(5);
    R.key('KeyF'); const set = R.until(() => NB.state === 'set', 30); const stS = men.map((m: any) => doing(O.g, m));
    verdict(set && stS.every((s: string) => /^set · press F/.test(s)), 'F stacks the pair as before, and the HUD\'s squad block says so', `night breach ${NB.state} · "${stS.join('" / "')}"`);
    const dS = R.door('server_manager'); const tS = { kind: 'door', at: [dS.frameCentre[0], 0, dS.frameCentre[2]], door: dS, side: 1 };   // (the keypad door, from the far side of the floor: nobody can get to that face — and with ORDERS.workOut off, no floor at either jamb on it)
    const greyS = O.whyNot(O.defs.find((d: any) => d.id === 'stack'), tS); const whyS = O.give('stack', tS); R.step(30);
    verdict(!!greyS && !!whyS && NB.state === 'set' && O.log.length === 0, 'an order that cannot be carried out (a stack with no room at the door) is greyed on the menu with the reason, and trying it does not call the breach off', `the stack row on that door: "${greyS}" · give('stack') → ${whyS} · night breach ${NB.state}`);
    R.quick(AISLE); const l = lastOrder(O); const stateOff = NB.state;
    R.until(() => men.every((m: any) => O.of(m)?.arrived), 14); R.step(60);
    verdict(l?.order === 'move' && stateOff === 'staged' && NB.log.some((x: any) => x.what === 'order:off') && isNaN(beat('kick')) && men.every((m: any) => dist(m.char.pos, AISLE) < 2.55) && Math.abs(R.door('breakroom_w').angle) < 0.05,
      'an order while they are set calls the breach off: the pair leaves the door for the spot, nothing is kicked', `${logLine(l)} · night breach ${stateOff} · kick ${f2(beat('kick'))} · ${men.map((m: any) => f2(dist(m.char.pos, AISLE))).join(' / ')} m off the point · ${NB.summary()}`);
    R.key('KeyF'); R.step(5); const owned = men.map((m: any) => O.of(m));
    const set2 = R.until(() => NB.state === 'set', 30);
    verdict(set2 && owned.every((a: any) => a === null), 'F again: the drill takes the pair back (their orders dropped) and stacks them', `night breach ${NB.state} · orders left ${owned.filter(Boolean).length}`);
    R.key('KeyF'); R.step(20); const going = NB.state; const n = O.log.length;
    R.quick(sam.pos); const refusedLog = O.log.length === n; const stillOwned = men.every((m: any) => O.of(m) === null);
    const ended = R.until(() => NB.drill.phase === 'done', 45);
    verdict(going === 'going' && refusedLog && stillOwned && ended && NB.state === 'clear' && !isNaN(beat('kick')) && !isNaN(beat('bang')), 'once the breach is going an order is refused, and the breach runs on to "clear"', `at the order: ${going}, logged ${!refusedLog}, taken ${!stillOwned} · ended ${NB.state} · kick @${f2(beat('kick'))} bang @${f2(beat('bang'))} clear @${f2(beat('clear'))} · ${NB.summary()}`);
    verdict(!R.samHit, 'Sam was not hit', `hit ${R.samHit}`);
  }
  { // ---- a recall once they are inside
    const R = await rig(11, { quiet: false }); const { game: g, O, NB } = R; const [b1, b2] = O.squad(); const men = [b1, b2]; const sam = g.player.char;
    R.step(5); R.key('KeyF'); R.until(() => NB.state === 'set', 30); R.key('KeyF');
    const inside = R.until(() => NB.state === 'inside', 30);
    R.quick(sam.pos); const l = lastOrder(O); const st = NB.state; let threw = '';
    try { R.step(60 * 12); } catch (e: any) { threw = String(e?.message ?? e); }
    const alive = men.filter((m: any) => m.state !== 'dead');
    verdict(inside && l?.order === 'fallIn' && st === 'recalled' && NB.drill.phase === 'done' && !threw && alive.every((m: any) => !!O.of(m)), 'an order once they are inside recalls them: the drill ends ("recalled"), the men are on Sam\'s order, the floor runs on',
      `${logLine(l)} · night breach ${st} · ${threw || 'no throw in 12 s'} · ${men.map((m: any) => `${m.callsign} ${m.state === 'dead' ? 'down' : `"${doing(O.g, m)}" ${f2(dist(m.char.pos, sam.pos))} m`}`).join(' · ')}`);
  }
  return done();
});

export const ordersSeams = guarded(async () => {
  const { verdict, done } = family('orders-seams: what the later parts build on');
  const R = await rig(11); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2];
  R.step(30); R.quick(AISLE); R.until(() => men.every((m: any) => O.of(m)?.arrived), 12); R.step(30);
  // the fight layer's seam: Blue One's script ended 'alert' with eyes on somebody for two seconds (emulated: lastSeenT stamped), then nothing
  const post = [...O.of(b1).post]; endGuardScript(b1, 'alert');
  let scripted = 0, pinned = 0, n = 0; const words = new Set<string>();
  for (let i = 0; i < 120; i++) { b1.lastSeenT = g.time; R.step(); n++; if (b1.script) scripted++; if (b1.pinned) pinned++; words.add(doing(O.g, b1).split(' · on ')[0]); }
  const dHot = dist(b1.char.pos, post), stateHot = b1.state;
  const retaken = R.until(() => !!b1.script, 6); const tCold = g.time - b1.lastSeenT; R.until(() => !!O.of(b1)?.arrived, 6); R.step(30);
  verdict(scripted === 0 && pinned === n && words.size === 1 && words.has('in contact') && dHot < 0.5 && stateHot === 'alert', 'a man the fight AI has (script ended, alert, eyes on): orders leave him to it, rooted on his post, the HUD says "in contact"',
    `two seconds: scripted ${scripted}/${n} frames, pinned ${pinned}/${n} · "${[...words].join('", "')}" · ${f2(dHot)} m off his post · ${stateHot}`);
  verdict(retaken && tCold >= ORDERS.coldSecs - 0.05 && tCold < ORDERS.coldSecs + 0.2 && O.of(b1)?.order === 'move' && dist(b1.char.pos, post) < 0.35 && doing(O.g, b1).startsWith('in position') && O.of(b2)?.arrived === true,
    'his contact cold: he is back on his order, on his post', `re-taken ${f2(tCold)} s after the last sighting (coldSecs ${ORDERS.coldSecs}) · ${f2(dist(b1.char.pos, post))} m off his post · "${doing(O.g, b1)}" · state ${b1.state}`);
  // a show takes the floor
  O.give('hold', O.targetAt(g.player.char.pos), 'all'); R.step(5); const had = men.filter((m: any) => O.of(m)).length;
  g.touring = true; ringUp(R.input, R); R.step(3); const left = men.filter((m: any) => O.of(m)).length; const hud = { menu: listedOf(O), status: blockOf(O) }; g.touring = false;
  verdict(had === 2 && left === 0 && !hud.menu && hud.status.length === 0, 'a show takes the floor (touring): every order is dropped, X is dead, the HUD block is gone', `orders ${had} → ${left} · hud rows ${hud.status.length}`);
  return done();
});


export const ordersEdges = guarded(async () => {
  const { verdict, done } = family('orders-edges: what cannot be done is refused aloud, and the men keep clear of Sam');
  const R = await rig(11); const { game: g, O } = R; const [b1, b2] = O.squad(); const men = [b1, b2]; const sam = g.player.char;
  const def = (id: string) => O.defs.find((d: any) => d.id === id);
  R.step(30);
  // (a) a floor spot no path leads to (found by scanning the nav grid — called directly: the cursor cannot be put on a spot chosen this way)
  let island: number[] | null = null;
  for (let z = 4.75; z < 24 && !island; z += 0.5) for (let x = 4.75; x < 36 && !island; x += 0.5) if (!g.col.nav.isBlocked(x, z) && !g.col.nav.findPath(b1.char.pos, [x, 0, z])) island = [x, 0, z];
  if (island) {
    // ('nobody sets off' = each man is left exactly as the refused order found him: with no order where a staging gives none, and where the team starts
    //  fallen in — START.fallIn, as the page has it — still on that same fall-in, not one dealt anew)
    const had = men.map((m: any) => O.of(m));
    const tI = { kind: 'floor', at: island }; const t0 = g.time; const grey = O.whyNot(def('move'), tI); const why = O.give('move', tI); R.step(20);
    verdict(/can't get there/.test(grey ?? '') && !!why && O.log.length === 0 && R.lines(/can't get there/, t0).length === 1 && men.every((m: any, i: number) => had[i] ? O.of(m) === had[i] && had[i].order === 'fallIn' : !O.has(m)), 'move there onto floor no path reaches: the row is greyed, the order is refused, a man says why, nobody sets off',
      `free floor with no way to it at ${at(island)} · row: "${grey}" · give → ${why} · said: ${R.lines(/./, t0).map(l => `${l.who} "${l.text}"`).join(' ')}`);
  } else verdict(false, 'a free floor cell with no path to it exists in the office', 'none found');
  // (e) Sam in the line of a man's gun: the muzzle comes down until he is clear
  const p1 = b1.char.pos; const through: number[] = [p1[0] + (sam.pos[0] - p1[0]) * 3, 0, p1[2] + (sam.pos[2] - p1[2]) * 3];
  O.give('cover', { kind: 'floor', at: through }, 0); R.step(60); const upIn = b1.script?.upper, stIn = b1.script?.stance, lowIn = O.of(b1)?.low;
  const samWas = [...sam.pos]; g.teleportPlayer([samWas[0] + 2.2, 0, samWas[2] - 1.5]); R.step(60); const upOut = b1.script?.upper, lowOut = O.of(b1)?.low;
  verdict(lowIn === true && upIn === 'none' && stIn === 'lowReady' && lowOut === false && upOut === 'aim', 'cover ordered straight through Sam: the man faces it with his muzzle down, and brings the gun up once Sam has stepped out of the line', `Sam in the line: upper ${upIn}, stance ${stIn} · Sam clear: upper ${upOut}`);
  g.teleportPlayer(samWas); R.step(10);
  // (f) fall in, then Sam walks out and doubles straight back along his own trail: the file gets out of his way
  O.give('fallIn', { kind: 'none', at: [...sam.pos] }, 'all'); R.until(() => men.every((m: any) => O.of(m)?.arrived), 10); R.step(30);
  const out = g.col.nav.findPath(sam.pos, AISLE_FAR) ?? [];
  const route: number[][] = [[sam.pos[0], 0, sam.pos[2]], ...out.map((q: number[]) => [q[0], 0, q[2]]), AISLE_FAR];
  const back = [...route].reverse().slice(1); const whole = [...route, ...back.slice(0, Math.max(1, back.length - 2))];
  let seg = 0, t = 0; const SPEED = 1.55; let turnedAt = -1; let minBack = Infinity, minOut = Infinity;
  R.pre = () => { if (seg >= whole.length - 1) return; let left = SPEED * DT;
    while (left > 0 && seg < whole.length - 1) { const a = whole[seg], b = whole[seg + 1], L = dist(a, b); if (L - t <= left) { left -= L - t; seg++; t = 0; } else { t += left; left = 0; } }
    const a = whole[seg], b = whole[Math.min(seg + 1, whole.length - 1)], L = dist(a, b) || 1, f = seg >= whole.length - 1 ? 0 : t / L; g.teleportPlayer([a[0] + (b[0] - a[0]) * f, 0, a[2] + (b[2] - a[2]) * f]); };
  const tW = g.time;
  while (seg < whole.length - 1 && g.time - tW < 60) { R.step(); const d = Math.min(...men.map((m: any) => dist(m.char.pos, sam.pos))); if (seg >= route.length - 1) { if (turnedAt < 0) turnedAt = g.time - tW; minBack = Math.min(minBack, d); } else minOut = Math.min(minOut, d); }
  R.pre = null; R.step(90); const endD = men.map((m: any) => dist(m.char.pos, sam.pos));
  verdict(turnedAt > 0 && minBack > 0.55 && endD.every(d => d < 3.5), 'Sam walks 11 m out along the aisle and straight back the way he came: no man is ever within arm\'s reach of him on the way back, and the file is still with him at the end', `nearest a man came to Sam on the way out ${f2(minOut)} m · on the way back ${f2(minBack)} m · at the end ${endD.map(f2).join(' / ')} m`);
  // (g) the file closes up when the man ahead is taken off it
  const first = men.find((m: any) => O.of(m)?.follow === 0), second = men.find((m: any) => m !== first);
  const dSecond0 = dist(second.char.pos, sam.pos); O.give('hold', { kind: 'none', at: [...first.char.pos], man: first }); R.step(1);
  const rank = O.of(second)?.follow; g.teleportPlayer([sam.pos[0] - 3, 0, sam.pos[2]]); R.step(60 * 5); const dSecond1 = dist(second.char.pos, sam.pos);
  verdict(rank === 0 && O.of(first)?.order === 'hold' && dSecond1 < 2.0, 'the first man in the file is told to hold: the man behind him takes the first place behind Sam', `${second.callsign}'s place in the file ${rank} · he was ${f2(dSecond0)} m from Sam, after Sam moved on ${f2(dSecond1)} m`);
  // (g2) Sam leans 1.4 m out and comes back: nobody follows the lean
  O.give('fallIn', { kind: 'none', at: [...sam.pos] }, 'all'); R.until(() => men.every((m: any) => O.of(m)?.arrived), 10); R.step(60);
  { const o = [...sam.pos]; const away = men.reduce((a: number[], m: any) => [a[0] + o[0] - m.char.pos[0], 0, a[2] + o[2] - m.char.pos[2]], [0, 0, 0]); const L = Math.hypot(away[0], away[2]) || 1; const u = [away[0] / L, 0, away[2] / L];   // straight away from the men
    const along = (q: number[]) => (q[0] - o[0]) * u[0] + (q[2] - o[2]) * u[2]; const before = men.map((m: any) => along(m.char.pos)); let x = 0, dirn = 1, peak = -Infinity;
    R.pre = () => { if (dirn === 0) return; x += dirn * 1.55 * DT; if (x >= 1.4) dirn = -1; if (x <= 0 && dirn < 0) { x = 0; dirn = 0; } g.teleportPlayer([o[0] + u[0] * x, 0, o[2] + u[2] * x]); };
    for (let i = 0; i < 60 * 6; i++) { R.step(); peak = Math.max(peak, ...men.map((m: any) => along(m.char.pos))); } R.pre = null;
    const after = men.map((m: any) => along(m.char.pos));
    verdict(peak < 0.5 && after.every(a => a < 0.1), 'Sam steps 1.4 m out from where he stood and back again: no man ends up out past him (the trail of the lean is not somewhere to stand)', `along the lean (Sam's spot = 0): the men before ${before.map(f2).join(' / ')} m · furthest out any man got ${f2(peak)} m · after ${after.map(f2).join(' / ')} m`);
  }
  // (g3) the men take turns answering, each through both lines
  { const t0 = g.time; for (let k = 0; k < 4; k++) { O.give('fallIn', { kind: 'none', at: [...sam.pos] }, 'all'); R.step(20); }
    const by = (who: string) => [...new Set(R.lines(/on you|with you/, t0).filter(l => l.who === who).map(l => l.text))].sort().join(' + ');
    verdict(men.every((m: any) => by(m.callsign) === 'on you. + with you.'), 'four orders in a row: each man answers twice, once with each line', men.map((m: any) => `${m.callsign}: ${by(m.callsign)}`).join(' · ')); }
  // (g4) a mark no path reaches, handed over directly (the menu would have refused it): he stops trying, says so, and the mark comes to him
  if (island) { const t0 = g.time; O.newGroup(); O.assign(b1, { order: 'move', post: [...island], face: 0, going: 'moving', there: 'in position' }); const got = R.until(() => !!O.of(b1)?.arrived, 20);
    verdict(got && doing(O.g, b1).startsWith('stopped short') && R.lines(/can't get through/, t0).length === 1 && g.time - t0 < ((await import(`${ROOT}/src/game/teampace.ts`)).TEAMPACE.on ? 16 : 12), 'a man sent at a mark he cannot reach gives up within seconds, says so once, and stands where he got to', `"${doing(O.g, b1)}" after ${f2(g.time - t0)} s · said: ${R.lines(/can't/, t0).map(l => `${l.who} "${l.text}"`).join(' ')} · ${f2(dist(b1.char.pos, island))} m from the mark`); }
  // (h) the fight seam's clock: handed over with no sighting on record he is still left alone for the cold time, and gunfire in his ear keeps him in it
  O.give('hold', { kind: 'none', at: [...b2.char.pos], man: b2 }); R.step(30);
  b2.lastSeenT = -1000; endGuardScript(b2, 'alert'); const tY = g.time; R.until(() => !!b2.script, 6); const leftFor = g.time - tY;
  endGuardScript(b2, 'alert'); const tS = g.time; R.pre = () => { if (g.time - tS < 4) b2.witness.heard.lastShotT = g.time; }; R.until(() => !!b2.script, 12); R.pre = null; const withShots = g.time - tS;
  verdict(leftFor > ORDERS.coldSecs - 0.2 && leftFor < ORDERS.coldSecs + 0.5 && withShots > 4 + ORDERS.coldSecs - 0.3 && O.of(b2)?.order === 'hold', 'a man handed to the fight AI with no sighting on record is left to it for the cold time, and for as long as shots keep reaching his ear', `no sighting: taken back after ${f2(leftFor)} s (cold time ${ORDERS.coldSecs}) · shots heard for 4 s: after ${f2(withShots)} s`);
  return done();
});
