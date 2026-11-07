// 'noknife' — NO KNIFE (CLOSE.noKnife, src/game/close.ts). The maintainer's design: "remove the knife, only rmb for non lethal CQC when nothing equipped … then
// we also remove the knife from interrogation. you can still select nothing equipped or the pistol, but there's only rmb to choke out".
//   bun run tools/qa/noknife-check.ts        (also `probes.ts noknife`)
// The key is settled on: the check asks the page's side alone and writes no switch (it used to hold the stalk off as well, STALK.on, which is settled on
// too: the left click is asked as the page has it). The key-down control of `draw`, the lesson lines read with the key down, with the stalk off and with
// the knife back, the `off` part and the harness part went with that.
//   blow      empty hands in reach of a man who stands, from behind, from a side and from the front: a left click starts nothing and he is on his feet a second
//             and a half later; a right click puts him down, out cold (the chronicle files a knockout, no kill).
//   hold      a man in the arm: a left click is no kill (no kill phase, he breathes a second later); a right click starts the choke and he goes down choked.
//   draw      with him in the arm no box of Sam's is the knife or its sheath, and his rows name no knife; in reach with empty hands the list says the right
//             click's knockout and no knife.
//   words     the three lesson lines that teach the knife say the knockout instead; a line that names the move by a token says the page's key and no
//             knife, and the lesson's own check refuses a bad token.
// NOKNIFE_BREAK=1 = Sam is stood 3 m from the man, out of reach, where blow asks for him in reach: blow must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here says how the bare-handed hold or the tell without a hilt LOOKS.
import { standUp, ROOT } from './headless.ts';

const { startGrab } = await import(`${ROOT}/src/game/player.ts`);
const { handProps } = await import(`${ROOT}/src/game/combat.ts`);
const { KNIFE_TAG, SHEATH_TAG } = await import(`${ROOT}/src/game/rigProps.ts`);
const { lessonLine } = await import(`${ROOT}/src/game/lesson.ts`);

const BREAK = process.env.NOKNIFE_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, AT = [10.2, 0, 19.5];   // (the lobby's open floor)
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
/** one man stood on the lobby's floor facing east, the rest parked, Sam with empty hands `gap` m from him `bearing` degrees off his facing (180 = behind) */
async function world(bearing: number, gap = 0.8) {
  const W: any = await quiet(() => standUp({ seed: 11 } as any)), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; g.player.slot = 'pistol'; g.player.holstered = true;
  const [A, ...rest] = g.guards; for (const [i, gd] of rest.entries()) { gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; }
  A.char.pos = [...AT]; A.char.bodyYaw = A.char.aimYaw = EAST; A.char.vel = [0, 0, 0]; A.path = []; A.pathGoal = null; A.hold = true; A.pinned = true; A.awareness = 0; A.heardUpTo = 1e12;
  const a = EAST + bearing * Math.PI / 180; W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([AT[0] + Math.sin(a) * gap, 0, AT[2] + Math.cos(a) * gap]);
  const step = (n = 1) => { for (let i = 0; i < n; i++) { W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; } };
  const click = (bit: number) => { W.input.buttons |= bit; W.input.clicked |= bit; step(); W.input.buttons &= ~bit; step(); };
  const said = () => g.nearby.map((r: any) => Object.values(r).filter(v => typeof v === 'string').join(' ')).join(' | ');
  const ev0 = g.chronicle.events.length, filed = () => g.chronicle.events.slice(ev0).map((e: any) => `${e.kind}${e.data?.weapon ? ':' + e.data.weapon : ''}${e.data?.method ? ':' + e.data.method : ''}`);
  step(40);
  return { W, g, A, step, click, said, filed };
}

async function blow() {
  console.log('\n=== blow: the left click is no blow, the right click the knockout, from every side ===');
  const rows: string[] = []; let ok = true;
  for (const [name, bearing] of [['behind', 180], ['a side', 90], ['the front', 0]] as const) {
    const C = await world(bearing, BREAK ? 3 : 0.8), { g, A } = C; const offered = /RMB {2}knock him out/.test(C.said()) && !/knife/i.test(C.said());
    C.click(1); const begun = !!g.player.takedown; C.step(90); const stood = A.state !== 'dead';
    C.click(4); let t = Infinity; for (let f = 0; f < 240; f++) { C.step(); if (A.state === 'dead') { t = (f + 1) * DT; break; } }
    const F = C.filed(), ko = F.some((e: string) => e.startsWith('ko')), kill = F.some((e: string) => e.startsWith('kill'));
    ok &&= offered && !begun && stood && t < 4 && ko && !kill;
    rows.push(`${name}: the list says the knockout and no knife ${offered} · the left click began a blow ${begun}, he stood a second and a half on ${stood} · the right click: ${isFinite(t) ? `down in ${t.toFixed(2)} s` : 'still up'}, filed ${F.join(', ') || 'nothing'}`);
  }
  verdict(ok, 'empty hands in reach of a man who stands, from behind, a side and the front: the left click starts nothing, the right click puts him out cold and no kill is filed', rows.join('\n    '));
}
async function hold() {
  console.log('\n=== hold: in the arm the left click is no kill, the right click the choke ===');
  const C = await world(180), { g, A } = C; startGrab(g, A); C.step(120);
  const H0 = g.player.holding, held = !!H0 && H0.phase === 'held';
  C.click(1); C.step(60); const H1 = g.player.holding, killing = !H1 || H1.phase === 'kill' || A.state === 'dead';
  C.click(4); let choking = false; for (let f = 0; f < 60 && !choking; f++) { C.step(); const H = g.player.holding; choking = !!H && (H.phase === 'choke' || H.windT >= 0); }
  let t = Infinity; for (let f = 0; f < 15 * 60; f++) { C.step(); if (A.state === 'dead') { t = (f + 1) * DT; break; } }
  const F = C.filed();
  verdict(held && !killing && choking && t < 15 && F.some((e: string) => e.startsWith('ko')) && !F.some((e: string) => e.startsWith('kill')),
    'a man in the arm: the left click is no kill and he breathes a second later; the right click starts the choke and he goes down choked, no kill filed',
    `held ${held} · after the left click: killed or killing ${killing} · the right click began the choke ${choking} · ${isFinite(t) ? `down in ${t.toFixed(2)} s` : 'never down'} · filed ${F.join(', ') || 'nothing'}`);
}
async function draw() {
  console.log('\n=== draw: nothing of his is a knife ===');
  const boxes = (g: any) => { const out: any[] = []; handProps(g, out); const mine = out.filter(b => b.owner === (g.player.char.id & 0xff)); return { knife: mine.filter(b => b.name === KNIFE_TAG).length, sheath: mine.filter(b => b.name === SHEATH_TAG).length }; };
  const C = await world(180), reach = C.said(), near = boxes(C.g); startGrab(C.g, C.A); C.step(120); const inArm = boxes(C.g), rows = C.said(), flag = !!C.g.player.char.anim.noKnife;
  verdict(flag && inArm.knife === 0 && inArm.sheath === 0 && near.knife === 0 && near.sheath === 0 && !/knife/i.test(rows) && !/knife/i.test(reach),
    'in reach and with him in the arm no box of Sam\'s is the knife or its sheath and no row names one',
    `the animator told ${flag} · in reach: knife boxes ${near.knife}, sheath ${near.sheath} · in the arm: knife ${inArm.knife}, sheath ${inArm.sheath} · rows name a knife: in reach ${/knife/i.test(reach)}, in the arm ${/knife/i.test(rows)}`);
}
async function words() {
  console.log('\n=== words: the lessons teach the knockout ===');
  const LINES = ['Right click to knock him out, or left click to use the knife', 'Stay low so he doesn\'t hear you, then left click to knife him', 'When the guard comes through, step in and left-click to kill him with the knife'];
  const now = LINES.map(l => lessonLine(l));
  verdict(now.every((l: string) => !/knife/i.test(l) && /knock him out/.test(l)),
    'each of the three lesson lines that teach the knife says the knockout and no knife', now.map((l: string) => `'${l}'`).join(' · '));
  // …and a line that names the move by a token (lesson.ts TOKENS) says the page's key however the sentence around it is worded
  const { LESSON_TOKENS, lessonToken, lessonTokenFaults } = await import(`${ROOT}/src/game/lesson.ts`);
  const T = 'When he lights up, creep in and {grab him from behind}. {Take him out} if he turns, or {knock him out}. {Grab him} now. {open the door}', page = lessonLine(T) as string;
  const known = LESSON_TOKENS.every((n: string) => lessonToken(n)) && lessonToken('Grab the near guard from behind') && !lessonToken('open the door') && !lessonToken('him');
  // …and the lesson's own check (checkLesson) refuses a name in braces that is no token, a brace left open, and a line that fits as written and not as read
  const pad = 'x'.repeat(70), faults = (l: string) => (lessonTokenFaults('t', l, 96) as string[]).length, refused = faults('{grab him from behnd}') + faults('{knock-him-out} now') + faults('grab him}') + faults(`${pad} {grab him from behind}`), clean = faults('{Grab yours}, then wait.') + faults('No braces here.') + faults('Creep in and {grab him}.'), far = faults('{grab guard 2}') + faults('{grab him }') + faults('{take the  man out}');
  const over = `${'x'.repeat(90)} {grab him from behind}`, row = lessonTokenFaults('t', `${pad} {grab him from behind}`, 96, 'a card') as string[], named = row.length === 1 && /: a card holds about 96$/.test(row[0]) && faults(over) === 0 && (lessonTokenFaults('t', over, 96, 'a card', true) as string[]).length === 1;   // (the row's name is the caller's; a line over the cap as WRITTEN is said only when the caller has no cap of its own for it; far: a digit, a space at the end, two spaces)
  verdict(/press \[Space\] behind him to grab him/.test(page) && !/knife/.test(page) && /\{open the door\}$/.test(page) && LESSON_TOKENS.length === 4 && known && refused === 4 && far === 3 && clean === 0 && named,
    'a line that names the move by its token says the grab\'s key and the knockout in the page\'s words and no knife, and leaves a name it does not know as written — which the lesson\'s check refuses, with a brace left open and a line too long once written out (the message naming the row the caller says the cap is of)',
    `the page: '${page}' · tokens: ${LESSON_TOKENS.join(', ')}, known ${known} · refused ${refused} of 4 bad lines and ${far} of 3 more, faulted ${clean} of 3 good ones · the message names the caller's row, and a line over the cap as written is said only when asked: ${named}`);
}

/** the whole card (probes.ts 'noknife'): true = every check passed */
export async function noknifeCheck(): Promise<boolean> {
  allPass = true;
  await blow(); await hold(); await draw(); await words();
  console.log(`\n=== noknife: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit((await noknifeCheck()) ? 0 : 1);
