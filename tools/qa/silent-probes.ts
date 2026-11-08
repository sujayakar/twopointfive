// The 'silent' family (game/silent.ts — the squad's posture on Sam's word: one row on the squad's order menu puts the whole squad in silent, the row that
// stands in its place takes it out). Headless: the whole Game over the fake device, the room clear on the office with its squad of four (or the pair).
//
//   silent-order   the menu: X on the floor / on a man lists "go silent" under 'the whole squad' on 5 (never a digit above 5, nothing held), 5 gives it to
//                  the whole squad whoever was addressed, the row then reads "go loud"; a door's menu keeps 5 for the door tools; the men keep their
//                  orders across it; the pair has the row too
//   silent-hud     the squad block's header says which of the two, in both states — and nothing with the switch off
//   silent-walk    the same walk loud and silent: silent legs never above SILENT.pace, no light burning, every footfall at the silent level; a hot floor
//                  does not lift the pace (a knob does); the step level a listener — Hard's defenders among them — is handed
//   silent-doors   an unlocked door is a hand on the leaf, not a shoulder (a creak for the bash); a silent man's contact with a leaf is a quiet one
//   silent-pick    a silent squad at a locked door: the lead kneels at the keyway and picks it (the player's own pick: its seconds, its clicks, its snap),
//                  then the entry goes on as at an unlocked door; a steel door is refused as ever; told loud mid-pick he stands up and boots it; blinded
//                  mid-pick his hands come off and go back on; what the room's defenders made of the noise
//   silent-cans    a threshold entry with a can going in first at every door: loud spends one, silent keeps the kit whole — on the word too
//   silent-tools   the loud tools stay on the door's menu marked LOUD while silent; picking one (bang and clear at a locked door; bang, then step in on a
//                  held one) does it and takes the squad out of silent with one line
//   silent-fire    defenders who have not noticed the team: loud teammates shoot them, silent ones bring their guns on and hold ('eyes on. holding.');
//                  once the defenders notice, the silent men shoot — and the first loud thing in their ears is said once ('compromised.') and shown on
//                  the HUD's word, the order still standing
//   silent-set     told to stack beside men who have not noticed them, a silent pair on its marks with its rounds held on those men is SET ('set.', the
//                  row's 'stacked') and no hand goes to the handle while the guns are on them;
//                  SILENTSET_BREAK=1 = the pair is never told silent: the lines about SET must fail
//   silent-hush    a silent teammate's line is a radio line; a loud one's is called out
//   silent-start   SILENT.start stands a fresh squad up silent (and a restart again); the order still takes it out
//   silent-off     never given, the switch on or off: the same scripted minute digests the same every frame; with the switch off there is no row, no word
//                  on the HUD and no rule — even with the state left silent
// The harness stands the game up with SILENT.on false (src/host/standup.ts: the menu and the squad block as every older probe was written against); the rig
// here turns it on, as it ships.
// The rig runs on the page's door rules: an entry halts at a door found shut and 'clear from outside' is one order.
// The cross-commit identity (the order never given = the base build) is the play-throughs, run here and in a checkout of the base: the same lines.
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { FLOOR } = await import(`${ROOT}/src/game/floor.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const NVM: any = await import(`${ROOT}/src/game/nvg.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const { silentHoldsRound, silentHoldsTarget } = await import(`${ROOT}/src/game/silent.ts`);
const SILENT: any = SLM.SILENT;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const clone = (o: any) => JSON.parse(JSON.stringify(o));
const DEFAULTS = { DOORTOOLS: clone(DOORTOOLS), ROOMCLEAR: { ...ROOMCLEAR }, NIGHTBREACH: { ...NIGHTBREACH }, SILENT: clone(SILENT), NVG: clone(NVM.NVG), LIGHTS: clone(GDM.LIGHTS) };
const reset = () => { Object.assign(DOORTOOLS, clone(DEFAULTS.DOORTOOLS)); Object.assign(ROOMCLEAR, DEFAULTS.ROOMCLEAR); Object.assign(NIGHTBREACH, DEFAULTS.NIGHTBREACH); Object.assign(SILENT, clone(DEFAULTS.SILENT)); Object.assign(NVM.NVG, clone(DEFAULTS.NVG)); Object.assign(GDM.LIGHTS, clone(DEFAULTS.LIGHTS)); };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
const AISLE: number[] = [15, 0, 17.6], AISLE_FAR: number[] = [21, 0, 17.6];

interface RigOpts { squad?: 2 | 4; silent?: Record<string, unknown>; doortools?: Record<string, unknown>; empty?: boolean }
/** the room clear on the office, its squad stood up (`empty`: every hostile taken off the floor first — nothing but the orders decides what follows) */
async function rig(seed: number, o: RigOpts = {}) {
  reset(); Object.assign(NIGHTBREACH, { squad: o.squad ?? 4 }); Object.assign(DOORTOOLS, o.doortools ?? {});
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, input = W.input;
  Object.assign(DOORTOOLS, o.doortools ?? {});   // (again after the stand-up, which writes the table)
  Object.assign(SILENT, { on: true }, o.silent ?? {});   // (the harness stands the game up with the order off the menu — what the older probes were written against; this family is about it on, as shipped)
  const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true;
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(t); msg(t); }; }
  const why = RC.start(seed);
  const seen = new Set<any>();
  const R = { W, game, input, RC, NB, O, D, why, said, held: 0, kicks: 0, bangs: 0, opShots: 0, doorNoise: [] as { level: number; loud: boolean; who: number }[], post: null as null | (() => void),
    get team(): any[] { return O.squad(); }, get hostiles(): any[] { return NB.hostiles; },
    step(n = 1) { for (let i = 0; i < n; i++) {
      game.player.visibility = 0.05;
      R.held = Math.max(R.held, input.keys.size + (input.buttons ? 1 : 0));
      W.step(DT);
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e);
        if (e.kind === 'kick') R.kicks++; if (e.kind === 'bang') R.bangs++;
        if (e.kind === 'guardShot' && game.guards.some((x: any) => x.team === 'operative' && x.char.id === e.who)) R.opShots++;
        if (e.kind === 'door') R.doorNoise.push({ level: e.level ?? 0, loud: !!e.loud, who: e.who ?? -1 }); }
      R.post?.(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    cursor(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); },
    press(code: string) { input.pressed.add(code); R.step(); },
    last(): any { return O.log[O.log.length - 1] ?? null; },
    /** the newest posture change (silent.ts's own log: the order is not one of the orders' log's) */
    turn(): any { return SLM.SILENT_LOG[SLM.SILENT_LOG.length - 1] ?? null; },
    label(): string { return O.hud().status[0]?.label ?? ''; },
    /** X with the cursor on p: the menu's order rows as the panel draws them (the who row and the cancel row left out) — the menu left UP */
    menu(p: number[]) { R.cursor(p); R.press(ORDERS.key); R.step(2);
      const hud = O.hud().menu; return { kind: O.menu?.target.kind as string | undefined, rows: (hud?.rows ?? []).slice(1).filter((r: any) => r.kind !== 'cancel').map((r: any) => ({ head: r.label as string, key: r.opts[0]?.key as string, text: r.opts[0]?.text as string, off: !!r.off })) }; },
    esc() { for (let i = 0; i < 3 && O.menu; i++) R.press('Escape'); },
    give(id: string, at: number[], who: any = 'all'): string | null { return O.give(id, O.targetAt(at), who); },
    giveDoor(id: string, name: string): string | null { return O.give(id, O.targetAt(R.door(name).frameCentre)); },
    beats(op: any): string { return (op?.beats ?? []).map((b: any) => b.what).join(' | '); },
    silent(): boolean { return SLM.squadSilent(game); },
  };
  R.step(30);
  if (o.empty) { for (const h of game.guards) if (h.team === 'hostile' && h.state !== 'dead') killGuard(game, h, [0, 0, 1], true); R.step(60 * 4); }
  return R;
}
const digitAbove5 = (rows: { key: string }[]) => rows.some(r => /^[6-9]$|^0$/.test(r.key));
const rowOf = (rows: { key: string; text: string; head: string }[], text: string) => rows.find(r => r.text === text) ?? null;

// ---------------------------------------------------------------- the order
export const silentOrder = guarded(async () => {
  const { verdict, done } = family('silent-order: one row on the squad\'s menu, on 5, for the whole squad — and the row that takes it out');
  const R = await rig(7); const { O, game } = R; const sam = game.player.char.pos, spot = [sam[0] + 1.6, 0, sam[2]];
  verdict(R.why === null && R.team.length === 4 && !R.silent(), 'the room clear stands its four up loud', `start: ${R.why} · squad ${R.team.length} · ${SLM.silentSummary(game)}`);
  // the floor's menu
  const m1 = R.menu(AISLE), r1 = rowOf(m1.rows, 'go silent');
  verdict(m1.kind === 'floor' && !!r1 && r1.key === '5' && r1.head === 'the whole squad' && !(r1 as any).off && !digitAbove5(m1.rows) && !rowOf(m1.rows, 'go loud'),
    'X on the floor: "go silent" is on 5 under "the whole squad", no digit above 5 on the menu', m1.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | '));
  const n0 = O.log.length, s0 = R.said.length; R.press('Digit5'); R.step(2); const l1 = R.turn(), told = R.said.slice(s0).find((t: string) => /^you: /.test(t)) ?? '';
  verdict(l1?.to === 'silent' && l1.by === 'order' && /gold — go silent/i.test(told) && R.silent() && O.menu === null && R.held === 0,
    '5 gives it: the whole squad is silent, the menu is away, no key was ever held', `${l1?.to} by ${l1?.by} · "${told}" · silent ${R.silent()} · most keys down ${R.held} · said: ${R.said.slice(-2).join(' / ')}`);
  verdict(O.log.length === n0, 'it is not logged as one of Sam\'s orders to the men (a rescue that starts its clock on his first order does not start it on this)', `orders log +${O.log.length - n0}`);
  // … and the row that stands in its place
  const m2 = R.menu(AISLE), r2 = rowOf(m2.rows, 'go loud');
  verdict(!!r2 && r2.key === '5' && !rowOf(m2.rows, 'go silent') && !digitAbove5(m2.rows), 'silent: the same row reads "go loud", on the same digit', m2.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | '));
  R.press('Digit5'); R.step(2); const l2 = R.turn();
  verdict(l2?.to === 'loud' && l2.by === 'order' && !R.silent(), '5 again: loud', `${l2?.to} by ${l2?.by} · silent ${R.silent()}`);
  // a man's menu, and one man addressed
  const m3 = R.menu([sam[0], 1.0, sam[2]]), r3 = rowOf(m3.rows, 'go silent'); R.esc();
  verdict(m3.kind === 'none' && !!r3 && r3.key === '5' && !digitAbove5(m3.rows), 'X on Sam himself: the row is there too, on 5', m3.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | '));
  const s4 = R.said.length, w4 = R.give('silent', spot, 0); R.step(); const told4 = R.said.slice(s4).find((t: string) => /^you: /.test(t)) ?? '';
  verdict(w4 === null && /gold — go silent/i.test(told4) && R.silent() && R.team.every((x: any) => SLM.underSilence(game, x)), 'addressed to one man it is still the squad\'s: all four take it', `refusal ${w4} · "${told4}" · silent men ${R.team.filter((x: any) => SLM.underSilence(game, x)).length}`);
  // a door's menu keeps its 5
  const bw = R.door('breakroom_w'); const m5 = R.menu(bw.frameCentre), five = m5.rows.find((r: any) => r.key === '5'); R.esc();
  verdict(m5.kind === 'door' && !!five && /^door tools/.test(five.text) && !m5.rows.some((r: any) => /go (silent|loud)/.test(r.text)) && !digitAbove5(m5.rows), 'a door\'s menu is as it was: 5 is the door tools, no posture row',
    m5.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | '));
  verdict(R.give('loud', bw.frameCentre) === null && !R.silent(), '… though the order itself fits any target (give with the reticle on a door)', `silent ${R.silent()}`);
  // the men keep what they were doing
  R.give('hold', spot); R.step(20); const before = R.team.map((x: any) => O.of(x)); R.give('silent', spot); R.step(5); const after = R.team.map((x: any) => O.of(x));
  verdict(before.every((a: any, i: number) => !!a && a === after[i] && a.order === 'hold'), 'the men keep the orders they had: the posture moves nobody', after.map((a: any) => a?.order).join(', '));
  R.give('loud', spot);
  // the pair
  const P = await rig(7, { squad: 2 }); const mp = P.menu(AISLE), rp = rowOf(mp.rows, 'go silent'); P.press('Digit5'); P.step(2);
  verdict(P.team.length === 2 && rp?.key === '5' && P.silent() && P.team.every((x: any) => SLM.underSilence(P.game, x)), 'the pair has the same row on the same digit', `${mp.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | ')} · silent ${P.silent()}`);
  return done();
});

// ---------------------------------------------------------------- the HUD
export const silentHud = guarded(async () => {
  const { verdict, done } = family('silent-hud: the squad block says which of the two, at a glance');
  const R = await rig(7); const { game } = R;
  const l0 = R.label(); R.give('silent', AISLE); R.step(2); const l1 = R.label(); R.give('loud', AISLE); R.step(2); const l2 = R.label();
  verdict(/^squad · loud · orders to /.test(l0) && /^squad · SILENT · orders to /.test(l1) && l2 === l0, 'the header of the squad block: "loud" until told, "SILENT" while silent, "loud" again', `'${l0}' → '${l1}' → '${l2}'`);
  R.give('silent', AISLE); SILENT.on = false; R.step(3); const l3 = R.label(), m = R.menu(AISLE); R.esc();
  verdict(/^squad · orders to /.test(l3) && !m.rows.some((r: any) => /go (silent|loud)/.test(r.text)) && !R.silent() && R.give('silent', AISLE) !== null,
    'the switch off: no word on the header, no row, no such order — and a state left silent rules nothing', `'${l3}' · ${m.rows.map((r: any) => `${r.key || '·'} ${r.text}`).join(' | ')} · ${SLM.silentSummary(game)}`);
  SILENT.on = true; R.step(3); const l4 = R.label(), m2 = R.menu(AISLE); R.esc();
  verdict(/^squad · SILENT · /.test(l4) && rowOf(m2.rows, 'go loud')?.key === '5', '… and back on: the rows and the word are back (the squad as it was left)', `'${l4}'`);
  return done();
});

// ---------------------------------------------------------------- the walk
/** SILENTWALK_BREAK=1: the pace line asks a pace 0.6 m/s under the rule's own number — the silent men's legs are then over it, and it must FAIL */
const WALK_BREAK = process.env.SILENTWALK_BREAK === '1';
interface Walk { top: number; lit: number; frames: number; steps: number[]; gog: number; arrived: boolean; /** the most the game would give a teammate's legs on this walk, whatever was asked (guards.ts teamPace: the team's cap, a silent squad's under it) */ cap: number }
async function walk(silent: boolean, o: RigOpts = {}): Promise<{ R: any; w: Walk }> {
  const R = await rig(7, { empty: true, ...o }); const { game, O, W } = R; const team: any[] = R.team;
  if (silent) R.give('silent', AISLE);
  R.give('move', [11, 0, 17.6]); R.until(() => team.every(x => O.of(x)?.arrived), 25); R.step(30);
  W.audioLog = []; const w: Walk = { top: 0, lit: 0, frames: 0, steps: [], gog: 0, arrived: false, cap: 0 };
  R.post = () => { w.frames++; for (const x of team) { w.top = Math.max(w.top, x.speed); w.cap = Math.max(w.cap, GDM.teamPace(game, x, 99)); if (x.flashlight?.enabled) w.lit++; if (NVM.nvgDown(game, x)) w.gog++; } };
  R.give('move', AISLE_FAR); w.arrived = R.until(() => team.every(x => O.of(x)?.arrived), 30); R.post = null;
  w.steps = (W.audioLog as any[]).filter(c => c.name === 'footstep' && !c.opts?.isPlayer).map(c => c.gain / (FLOOR as any)[c.opts.mat].loud); W.audioLog = null;
  return { R, w };
}
export const silentWalk = guarded(async () => {
  const { verdict, done } = family('silent-walk: the same walk, loud and silent — the legs, the lights, the feet');
  const { w: L } = await walk(false), { R, w: S } = await walk(true); const { game } = R; const man = R.team[0];
  const mean = (a: number[]) => a.reduce((s, x) => s + x, 0) / Math.max(1, a.length), maxOf = (a: number[]) => a.reduce((m, x) => Math.max(m, x), 0);
  // (THE ORDER'S PACE IS THE GAME'S OWN: what guards.ts teamPace gives a teammate's legs on the walk. Where that is over the silent pace the two walks differ and
  //  the loud one must show it; where the squad's own walk is already under it — the page's squad walks at the player's pace — the silent pace has nothing to
  //  take, and the line holds the legs to the lower of the two.)
  const pace = SILENT.pace - (WALK_BREAK ? 0.6 : 0), faster = L.cap > SILENT.pace + 0.2;
  verdict(L.arrived && S.arrived && L.top <= L.cap + 0.02 && (!faster || L.top > SILENT.pace + 0.2) && S.top <= Math.min(pace, L.cap) + 0.02 && S.cap <= SILENT.pace + 1e-6, 'loud they move at the order\'s pace; silent no man\'s legs go above SILENT.pace — and they still get there',
    `top speed loud ${f2(L.top)} m/s (the order's pace ${f2(L.cap)}) · silent ${f2(S.top)} (the pace asked ${f2(pace)}, the game's own cap then ${f2(S.cap)}) · ${faster ? 'the loud walk is the faster' : 'the squad\'s own walk is under the silent pace: nothing for it to take'} · arrived ${L.arrived} / ${S.arrived} in ${L.frames} / ${S.frames} frames`);
  verdict(L.steps.length > 8 && S.steps.length > 8 && maxOf(S.steps) <= SILENT.step + 1e-6 && mean(L.steps) > SILENT.step + 0.1, 'every silent footfall is at SILENT.step (floor for floor), under the loud walk\'s',
    `footfalls loud ${L.steps.length}, base level mean ${f2(mean(L.steps))} · silent ${S.steps.length}, max ${f2(maxOf(S.steps))} (SILENT.step ${SILENT.step})`);
  // what a listener is handed, and the hot floor
  const stepS = SLM.silentStep(game, man), stepH = SLM.silentStep(game, R.hostiles[0] ?? { team: 'hostile', unit: {} });
  const calm = SLM.silentPace(game, man, 2.2); game.fight.hotT = game.time; const hot = SLM.silentPace(game, man, 2.2); game.fight.hotT = -100;
  verdict(stepS === SILENT.step && stepH === null && calm === SILENT.pace && hot === SILENT.pace && SLM.silentPace(game, man, 1.0) === 1.0,
    'the step a listener hears from a silent teammate is SILENT.step (nobody else\'s is touched); a hot floor does not lift his pace, and a slower walk is never sped up',
    `step: teammate ${stepS}, hostile ${stepH} · pace asked 2.2: calm ${calm}, hot ${hot} · asked 1.0: ${SLM.silentPace(game, man, 1.0)}`);
  R.give('loud', AISLE); R.step(90); const back = R.team.filter((x: any) => x.state !== 'dead').length > 0 && SLM.silentPace(game, man, 2.2) === 2.2 && SLM.silentStep(game, man) === null && !SLM.silentDark(game, man);
  verdict(back, 'told loud again: pace, step and light are the team\'s own again', `pace ${SLM.silentPace(game, man, 2.2)} · step ${SLM.silentStep(game, man)} · dark ${SLM.silentDark(game, man)}`);
  return done();
});

// ---------------------------------------------------------------- doors
export const silentDoors = guarded(async () => {
  const { verdict, done } = family('silent-doors: no boot by themselves, and a leaf eased open');
  // (the locked door's line, 'loud … the lead boots it (as ever)', went 2026-11-07: it asked the squad of before the handle is tried and before the big slot's tools.
  //  What a loud squad does at a lock is asked by doortools-probes.ts and frameholds-probes.ts.)
  // the unlocked door: the shove
  const shove = async (silent: boolean) => { const R = await rig(7, { empty: true }); const { D, W } = R; const d = R.door('breakroom_w');
    if (silent) R.give('silent', AISLE); W.audioLog = []; let top = 0; R.post = () => { top = Math.max(top, Math.abs(d.vel)); };
    const w = R.giveDoor('openClear', 'breakroom_w'); const op = D.ops[0]; R.until(() => D.ops.length === 0, 45); R.post = null;
    const bash = (W.audioLog as any[]).filter(c => c.name === 'doorBash').length, creak = (W.audioLog as any[]).filter(c => c.name === 'doorCreak').length; W.audioLog = null;
    return { R, w, op, top, bash, creak, loudNoise: R.doorNoise.filter((n: any) => n.loud).length, noise: R.doorNoise.reduce((m: number, n: any) => Math.max(m, n.level), 0), opened: Math.abs(d.angle) > 0.5 || op?.crossed?.size > 0, crossed: op?.crossed?.size ?? 0 }; };
  const L = await shove(false), S = await shove(true);
  verdict(L.w === null && S.w === null && L.bash > 0 && S.bash === 0 && S.creak > 0 && S.loudNoise === 0 && Math.abs(S.top - L.top) < 0.5 && S.crossed === L.crossed && S.crossed > 0,
    'an unlocked door: loud it is shouldered open (the bash); silent it is a hand on the leaf (a creak, no bash, nothing loud filed) — the leaf swings as it did and the men go in the same',
    `loud: ${L.bash} bash, leaf top ${f2(L.top)} rad/s, ${L.crossed} in · silent: ${S.bash} bash, ${S.creak} creak, leaf top ${f2(S.top)} rad/s, loud door events ${S.loudNoise}, ${S.crossed} in`);
  // a contact
  const man = S.R.team[0]; const q1 = SLM.silentEase(S.R.game, man); S.R.give('loud', AISLE); const q0 = SLM.silentEase(S.R.game, man);
  verdict(q1 && !q0, 'a silent man\'s own push on a leaf is a quiet contact (doors.ts: the creak at 0.12 for 0.3); a loud man\'s as it was', `quiet contact: silent ${q1} · loud ${q0}`);
  return done();
});

// ---------------------------------------------------------------- the lock picked
export const silentPick = guarded(async () => {
  const { verdict, done } = family('silent-pick: a silent squad picks the lock it would have booted');
  const PICK_SECS = 4.2;   // (doors.ts Doors.PICK_SECS — the player's own)
  const run = async (o: RigOpts & { mid?: (R: any, op: any) => void } = {}) => { const R = await rig(7, o); const { D, game } = R; const Md = R.door('manager'); const locked = Md.locked, k0 = SLM.SILENT_STATS.picked;
    R.give('silent', AISLE); const w = R.giveDoor('openClear', 'manager'); const op = D.ops[0], lead = op?.lead;
    let kneel = 0, off = 0, t0 = NaN, t1 = NaN, heard = 0, near = Infinity, did = false, at = NaN, stillHot = 0, aw0 = NaN, awSnap = 0; const top = () => R.hostiles.reduce((m: number, h: any) => h.state === 'dead' ? m : Math.max(m, h.awareness), 0);
    R.post = () => { const hands = R.team.filter((m: any) => m.char.anim.lockpick);
      if (hands.length) { kneel++; if (isNaN(t0)) { t0 = game.time; at = dist(hands[0].char.pos, Md.pos); aw0 = top(); } } else if (!isNaN(t0) && Md.locked) off++;
      if (!Md.locked && isNaN(t1)) t1 = game.time; if (!isNaN(t1) && game.time - t1 < 0.5) awSnap = Math.max(awSnap, top());
      if (Md.locked) for (const h of R.hostiles) if (h.state !== 'dead') { heard = Math.max(heard, h.awareness); near = Math.min(near, dist(h.char.pos, Md.pos)); if (h.state === 'alert' || h.state === 'search') stillHot++; }
      if (o.mid && !did && Md.pick > 0.4) { did = true; o.mid(R, op); } };
    R.until(() => D.ops.length === 0 || (op?.crossed?.size ?? 0) > 0, 60); R.step(45); R.post = null;
    const mine = R.doorNoise.filter((n: any) => n.who === lead?.char.id);
    return { R, Md, locked, w, op, lead, kneel, off, secs: t1 - t0, at, heard, near, stillHot, aw0, awSnap, picked: SLM.SILENT_STATS.picked - k0, picks: D.stats.picks as number, clicks: mine.filter((n: any) => Math.abs(n.level - 0.16) < 1e-6).length, snaps: mine.filter((n: any) => Math.abs(n.level - 0.3) < 1e-6).length,
      loudNoise: R.doorNoise.filter((n: any) => n.loud).length, handsLeft: R.team.filter((m: any) => m.char.anim.lockpick).length, crossed: op?.crossed?.size ?? 0, saidPick: R.said.filter((t: string) => /locked\. picking/.test(t)), ended: op?.ended as string | undefined }; };
  // picked, and entered
  const A = await run();
  verdict(A.locked && A.w === null && A.R.kicks === 0 && !A.Md.lockBroken && !A.Md.locked && A.picked === 1 && A.picks === 1 && A.saidPick.length === 1 && /radio/.test(A.saidPick[0]),
    'silent, a locked door, "open and clear": no boot — the lead says "locked. picking." over the net and the lock is picked, not broken', `kicks ${A.R.kicks} · lock broken ${!!A.Md.lockBroken} · locked now ${A.Md.locked} · picked ${A.picked} · said: ${A.saidPick.join(' / ')} · ${A.R.beats(A.op)}`);
  verdict(A.kneel > 200 && Math.abs(A.secs - PICK_SECS) < 0.3 && A.at < 1.2 && A.clicks >= 4 && A.clicks <= 9 && A.snaps === 1 && A.loudNoise === 0 && A.handsLeft === 0,
    'it is the player\'s own pick: hands on the lock at the door for its 4.2 s, a click every ~0.7 s at 0.16 and one snap at 0.3 filed for hearing under his id, nothing loud — and his hands come off it after',
    `hands on the lock ${A.kneel} frames, ${f2(A.at)} m from the leaf · ${f2(A.secs)} s to the snap · ${A.clicks} clicks, ${A.snaps} snap · loud door events ${A.loudNoise} · hands still on a lock after ${A.handsLeft}`);
  verdict(A.crossed > 0 && A.R.silent(), 'and the order carries on as at an unlocked door: the leaf is eased open and they go in, still silent', `over the line ${A.crossed} · ended: ${A.ended ?? '(still going in)'} · silent ${A.R.silent()}`);
  console.log(`    (informational) the room's defenders while the lock was worked: nearest ${f2(A.near)} m from the hinge, top awareness ${f2(A.aw0)} at the first touch → ${f2(A.heard)} at most while it was worked → ${f2(A.awSnap)} in the half second after the snap; frames any was alert / searching before it gave: ${A.stillHot}`);
  // the page's own door rule on (DOORTOOLS.testDoor: an entry halts at a door found shut, and steel is found at the handle, not read off the menu): a silent squad is not halted at a lock it can pick, and steel still gets no pick and no boot
  { const P = A;
    verdict(P.locked && P.w === null && P.R.D.stats.halted === 0 && P.R.kicks === 0 && P.picked === 1 && P.crossed > 0 && P.R.silent(),
      'with the door tried before entry (as the page ships): the silent lead finds it locked and picks it on the same order — no halt, no boot, and they go in silent', `halted ${P.R.D.stats.halted} · kicks ${P.R.kicks} · picked ${P.picked} · over the line ${P.crossed} · ${P.R.beats(P.op)}`);
    const R = await rig(7, {}); const { D, game } = R; const St = R.door('stairwell'); R.give('silent', AISLE); const w = R.giveDoor('openClear', 'stairwell'); const op = D.ops[0]; R.until(() => D.ops.length === 0, 45);
    verdict(St.locked && w === null && /steel/.test(op?.ended ?? '') && R.said.some((t: string) => /it's steel\. no way through/.test(t)) && D.stats.picks === 0 && St.pick === 0 && R.kicks === 0 && R.silent(),
      'and steel nobody has tried: the order is taken, the lead says it is steel at the door and the order ends there — nobody kneels, no boot, still silent', `given: ${w ?? 'ok'} · ended: ${op?.ended ?? '—'} · picks ${D.stats.picks} · kicks ${R.kicks} · said: ${R.said.filter((t: string) => /steel/.test(t)).join(' / ')}`); }
  // the fight has him mid-pick (blinded): hands off, then back on, and it still gives
  const C = await run({ mid: (R, op) => { op.lead.dazzledUntil = R.game.time + 1.0; } });
  verdict(C.w === null && C.off >= 50 && C.off <= 75 && C.picked === 1 && C.R.kicks === 0 && C.secs > PICK_SECS + 0.8 && C.secs < PICK_SECS + 1.6 && C.crossed > 0,
    'blinded for a second mid-pick: his hands come off the lock for it, go back on, and the lock still gives (the door kept what was done)', `hands off ${C.off} frames · ${f2(C.secs)} s from first touch to the snap · picked ${C.picked} · over the line ${C.crossed}`);
  return done();
});

// ---------------------------------------------------------------- the door no pick opens
export const silentBoot = guarded(async () => {
  const { verdict, done } = family('silent-boot: a door no pick opens (wedged from inside, a keypad\'s) — booted once compromised, and the squad stays silent');
  const wedge = (d: any) => { d.lockId = `wedge:${d.def.name}`; };                                    // (digin.ts wedge(): the door's own lock, to a lock no ring fits)
  const run = async (o: { prep: (d: any, R: any) => void; hot: boolean; silent?: boolean; again?: boolean; later?: number }) => {
    const R = await rig(7, { empty: true }); const { D, game } = R; const Md = R.door('manager'); o.prep(Md, R);
    const b0 = SLM.SILENT_STATS.booted, l0 = SLM.SILENT_STATS.locked, turns0 = SLM.SILENT_LOG.length, bangs0 = D.bangs;
    if (o.silent !== false) R.give('silent', AISLE);
    if (o.hot) { game.fight.hotT = game.time; R.step(3); }                                          // (a shot in the team's ears AFTER the order: silent-fire's own way of compromising them)
    const hotLabel = R.label(); if (o.later) R.step(60 * o.later); const orderLabel = R.label();   // (`later`: the noise that long gone before the door is given)
    const s0 = R.said.length, w = R.giveDoor('openClear', 'manager'); const op = D.ops[0], lead = op?.lead;
    let lit = 0, top = 0, kickLabel = '', regiven = false;
    R.post = () => { for (const m of R.team) { if (m.flashlight?.enabled) lit++; top = Math.max(top, m.speed); }
      if (R.kicks > 0 && !kickLabel) kickLabel = R.label();
      if (o.again && !regiven && op?.phase === 'kick' && R.kicks === 0) { regiven = true; R.giveDoor('openClear', 'manager'); } };   // (the same order given again while he walks to the kick spot)
    R.until(() => D.ops.length === 0 || (op?.crossed?.size ?? 0) > 0, 60); R.step(45); R.post = null;
    const man = R.team[0];
    return { R, Md, w, op, lead, lit, top, kickLabel, regiven, hotLabel, orderLabel, said: R.said.slice(s0), booted: SLM.SILENT_STATS.booted - b0, locked: SLM.SILENT_STATS.locked - l0, bangs: bangs0 - D.bangs,
      turns: SLM.SILENT_LOG.slice(turns0).map((x: any) => x.to), label: R.label(), dark: SLM.silentDark(game, man), pace: SLM.silentPace(game, man, 2.2), cans: SLM.silentNoCans(game), crossed: op?.crossed?.size ?? 0, ended: op?.ended as string | undefined }; };
  const line = (A: any, re: RegExp) => A.said.filter((t: string) => re.test(t));
  // wedged, compromised: the boot, said once, still silent
  const A = await run({ prep: wedge, hot: true });
  verdict(A.w === null && A.R.kicks === 1 && !!A.Md.lockBroken && A.booted === 1 && A.R.D.stats.picks === 0 && line(A, /wedged from inside\. booting it/).length === 1 && /radio/.test(line(A, /booting it/)[0] ?? '') && line(A, /kicking it|no boot/).length === 0 && A.crossed > 0,
    'silent and compromised at a door wedged from inside: the lead says "wedged from inside. booting it." once, over the net, boots it, and they go in', `kicks ${A.R.kicks} · lock broken ${!!A.Md.lockBroken} · booted ${A.booted} · said: ${line(A, /wedged|locked/).join(' / ')} · over the line ${A.crossed} · ${A.R.beats(A.op)}`);
  verdict(A.R.silent() && /SILENT · compromised/.test(A.kickLabel) && /SILENT · compromised/.test(A.label) && A.turns.join() === 'silent' && A.dark && A.lit === 0 && A.pace === SILENT.pace && A.top <= SILENT.pace + 0.25 && A.cans && line(A, /compromised\./).length === 0,
    'and the posture stands: still SILENT · compromised on the HUD at the boot and after, no light lit on the way in (a loud squad\'s burn: the last check), no man over the silent walk, the can rule still answering — and nobody says "compromised." a second time at their own boot', `HUD at the boot '${A.kickLabel}' · after '${A.label}' · posture turns: ${A.turns.join(', ')} · lit frames ${A.lit} · top speed ${f2(A.top)} (cap ${SILENT.pace}) · cans spent ${A.bangs}`);
  // wedged, NOT compromised: no boot, the line names the fix
  const B = await run({ prep: wedge, hot: false });
  verdict(B.w === null && B.R.kicks === 0 && B.Md.locked && !B.Md.lockBroken && B.booted === 0 && B.locked === 1 && /silent: no boot/.test(B.ended ?? '') && line(B, /wedged from inside\. no quiet way in\. go loud and we boot it/).length === 1 && B.R.silent() && !/compromised/.test(B.label),
    'silent and NOT compromised at the same door: no boot — "wedged from inside. no quiet way in. go loud and we boot it." and the order ends on the door, still locked', `kicks ${B.R.kicks} · locked ${B.Md.locked} · ended: ${B.ended} · said: ${line(B, /wedged|locked/).join(' / ')} · ${B.label}`);
  // … and his loud word after it does boot it
  { const { R, Md } = B; R.give('loud', AISLE); R.give('silent', AISLE); R.step(3); const again = SLM.silentBoot(R.game, Md);
    verdict(again === null, '(and a fresh "go silent" starts clean: not compromised until something loud comes after it)', `asked of a squad just told silent again: ${again}`); }
  return done();
});

// ---------------------------------------------------------------- cans
export const silentCans = guarded(async () => {
  const { verdict, done } = family('silent-cans: the canisters stay in the kit');
  const run = async (silent: boolean, then: boolean) => { const R = await rig(7, { doortools: { thrFirst: 2 } }); const { D } = R; const b0 = D.bangs;   // (thrFirst 2: a can goes in first at EVERY threshold — the loud side of the comparison is then certain)
    if (silent) R.give('silent', AISLE); const w = R.giveDoor(then ? 'thresholdHold' : 'threshold', 'breakroom_w'); const op = D.ops[0];
    if (then) { R.until(() => !!op?.thr?.told || D.ops.length === 0, 40); const held = D.bangs; const w2 = R.giveDoor('go', 'breakroom_w'); R.until(() => D.ops.length === 0, 60); return { R, w, w2, op, spent: b0 - D.bangs, heldSpent: b0 - held }; }
    R.until(() => D.ops.length === 0, 60); return { R, w, w2: null as string | null, op, spent: b0 - D.bangs, heldSpent: 0 }; };
  const T = await run(true, true);
  verdict(T.w === null && T.heldSpent === 0 && T.w2 === null && T.spent === 0 && T.R.bangs === 0 && (T.op?.crossed?.size ?? 0) > 0 && T.R.silent(), 'silent, holding the door, then the word: they step in on it, and the kit is still whole', `while holding ${T.heldSpent} spent · after the word ${T.spent} · blasts ${T.R.bangs} · over the line ${T.op?.crossed?.size} · ${T.R.beats(T.op)}`);
  return done();
});

// ---------------------------------------------------------------- the loud tools
export const silentTools = guarded(async () => {
  const { verdict, done } = family('silent-tools: picking a loud tool ends silent');
  // a bang and clear started any other way (a scenario's F: DoorTools.order)
  { const R = await rig(7); const { D, O } = R; const d = R.door('breakroom_w'); R.give('silent', AISLE); const t = O.targetAt(d.frameCentre); const r = D.order('bangClear', O.men(), d, t.side);
    verdict(typeof r !== 'string' && !R.silent() && R.turn()?.by === 'tool', 'a bang and clear given to the door tools directly (a scenario\'s own launch) ends silent the same way', `started ${typeof r !== 'string'} · silent ${R.silent()} · by ${R.turn()?.by}`); }
  // the same three under the page's own door rules (the door tried before entry; 'clear from outside' as one order — its second order is this page's rows 1 and 2)
  { const R = await rig(7, {}); const { D } = R; const Md = R.door('manager'); R.give('silent', AISLE); const w = R.giveDoor('bangClear', 'manager'); const flipped = !R.silent(); const op = D.ops[0];
    R.until(() => D.ops.length === 0 || R.kicks > 0, 45); const halted = D.stats.halted as number, k1 = R.kicks, said = R.said.filter((t: string) => /holding on it/.test(t));
    const w2 = R.giveDoor('bangClear', 'manager'); const op2 = D.ops[0]; R.until(() => D.ops.length === 0 || R.bangs > 0, 45); R.step(30);
    verdict(w === null && flipped && halted === 1 && k1 === 0 && said.length === 1 && w2 === null && R.kicks > 0 && !!Md.lockBroken && R.bangs > 0 && !R.silent(),
      'with the door tried before entry: silent, "bang and clear" on a locked door ends silent at the order, and the now loud squad does what a loud one does — halts on the lock, and the same row again is the boot, then the can',
      `loud at once ${flipped} · first order: halted ${halted}, kicks ${k1}, said: ${said.join(' / ')} | ${R.beats(op)} · again: kicks ${R.kicks}, blasts ${R.bangs} | ${R.beats(op2)}`); }
  { const R = await rig(7, { doortools: { thrFirst: 2 } }); const { D } = R; const b0 = D.bangs; R.give('silent', AISLE); R.giveDoor('thresholdHold', 'breakroom_w'); const op = D.ops[0];
    R.until(() => !!op?.thr?.told || D.ops.length === 0, 40); const held = b0 - D.bangs, was = R.silent(); const w2 = R.giveDoor('bangClear', 'breakroom_w'); const flipped = !R.silent(); R.until(() => R.bangs > 0, 30);
    verdict(held === 0 && was && w2 === null && flipped && b0 - D.bangs >= 1 && R.bangs >= 1, 'with "clear from outside" as one order: silent, holding a door, then "bang, then go in" — nothing thrown while they held; the row ends silent and its can goes in', `while holding ${held} spent, silent ${was} · after the row: ${w2 ?? 'given'}, loud ${flipped}, ${b0 - D.bangs} spent, blasts ${R.bangs}`); }
  return done();
});

// ---------------------------------------------------------------- fire
export const silentFire = guarded(async () => {
  const { verdict, done } = family('silent-fire: fire held on a man who has not noticed — and what changes once the squad is compromised');
  // the break room's two, kept unaware (state and awareness written back every frame) while the four go in on them
  const run = async (silent: boolean, o: RigOpts = {}) => { const R = await rig(7, o); const { game } = R; let frozen = true; const h0 = SLM.SILENT_STATS.held, c0 = SLM.SILENT_STATS.compromised;
    R.post = () => { if (frozen) for (const h of R.hostiles) if (h.state !== 'dead') { h.state = 'patrol'; h.awareness = 0; } };
    if (silent) R.give('silent', AISLE); R.giveDoor('openClear', 'breakroom_w'); R.step(60 * 25);
    const a = { shots: R.opShots, held: SLM.SILENT_STATS.held - h0, label: R.label(), eyesOn: R.said.filter((t: string) => /eyes on\. holding/.test(t)).length, down: R.hostiles.filter((h: any) => h.state === 'dead').length };
    frozen = false; for (const h of R.hostiles) if (h.state !== 'dead') { h.state = 'alert'; h.awareness = 1; } R.step(60 * 15); R.post = null;
    return { R, a, shots: R.opShots, comp: SLM.SILENT_STATS.compromised - c0, label: R.label(), said: R.said.filter((t: string) => /compromised/.test(t)), down: R.hostiles.filter((h: any) => h.state === 'dead').length, silent: R.silent(), top: 0 }; };
  const S = await run(true);
  verdict(S.a.shots === 0 && S.a.down === 0 && S.a.held > 0 && S.a.eyesOn > 0 && /SILENT/.test(S.a.label) && !/compromised/.test(S.a.label),
    'silent: guns on the same men, not one round — and they say so on the net', `rounds ${S.a.shots} · down ${S.a.down} · frames a round was held ${S.a.held} · "eyes on. holding." ×${S.a.eyesOn} · ${S.a.label}`);
  verdict(S.shots > 0 && S.down > 0 && S.silent, 'the men notice: the silent four shoot them — the order still stands', `rounds ${S.shots} · down ${S.down} · still silent ${S.silent}`);
  verdict(S.comp === 1 && S.said.length === 1 && /SILENT · compromised/.test(S.label), 'the first loud thing in a silent man\'s ears is said once, over the net, and the HUD\'s word shows it', `compromised ×${S.comp} · said: ${S.said.join(' / ')} · ${S.label}`);
  // noise from before the order is not a compromise; a captor at his last warning holds nothing back
  { const R = await rig(7); const { game } = R; const man = R.team[0], foe = R.hostiles.find((h: any) => h.state !== 'dead'); foe.state = 'patrol'; foe.awareness = 0;
    game.fight.hotT = game.time; R.step(3); const c0 = SLM.SILENT_STATS.compromised; R.give('silent', AISLE); R.step(30); const stale = { comp: SLM.SILENT_STATS.compromised - c0, label: R.label(), holds: silentHoldsRound(game, man, foe) };
    game.fight.hotT = game.time; R.step(3); const fresh = { comp: SLM.SILENT_STATS.compromised - c0, label: R.label(), holds: silentHoldsRound(game, man, foe) };
    verdict(stale.comp === 0 && !/compromised/.test(stale.label) && stale.holds && fresh.comp === 1 && /compromised/.test(fresh.label) && !fresh.holds,
      'told silent 3 frames after a loud fight: not compromised, fire still held — a loud thing AFTER the order is what compromises', `noise before the order: compromised ×${stale.comp}, holds ${stale.holds}, ${stale.label} · noise after: ×${fresh.comp}, holds ${fresh.holds}, ${fresh.label}`); }
  // told loud, the cue is gone
  S.R.give('loud', AISLE); S.R.step(5);
  verdict(!S.R.silent() && /squad · loud ·/.test(S.R.label()), 'told loud: the word on the HUD is "loud" again', S.R.label());
  return done();
});

// ---------------------------------------------------------------- lines
export const silentHush = guarded(async () => {
  const { verdict, done } = family('silent-hush: what he would have called out goes over the net');
  const R = await rig(7); const { game } = R;
  // (a line said by hand on the copy never reaches the game's own say: the lines asked are the men's answers to orders)
  const answer = (id: string, at: number[]) => { const t0 = game.time; R.give(id, at); let b: any = null; for (let i = 0; i < 120 && !b; i++) { R.step(1); b = R.team.map((x: any) => x.bubble).find((x: any) => x && x.t >= t0 - 0.02) ?? null; } const got = b && { ...b }; R.step(180); return got; };
  const loud = answer('hold', AISLE), ack = answer('silent', AISLE), after = answer('hold', AISLE_FAR);
  verdict(!!loud && loud.radio === false && !!ack && ack.radio === true && !!after && after.radio === true, 'loud: called out · silent: a radio line (the order\'s own answer included)',
    `loud '${loud?.text}' ${loud?.radio} · ack '${ack?.text}' ${ack?.radio} · silent '${after?.text}' ${after?.radio}`);
  return done();
});

// ---------------------------------------------------------------- the starting posture
export const silentStart = guarded(async () => {
  const { verdict, done } = family('silent-start: SILENT.start stands a fresh squad up silent');
  const R = await rig(7, { silent: { start: true } }); const { game, RC } = R; let lit = 0; R.post = () => { for (const x of R.team) if (x.flashlight?.enabled) lit++; };
  R.step(120); const a = R.silent(), la = R.label(), by = SLM.SILENT_LOG[SLM.SILENT_LOG.length - 1]?.by;
  verdict(a && /^squad · SILENT · /.test(la) && by === 'start', 'staged with start on: silent from the first frames, nobody told', `silent ${a} · '${la}' · by ${by} · man-frames lit over two seconds ${lit}`);
  R.give('loud', AISLE); R.step(5); const b = R.silent(); const cast0 = R.team[0];
  const why = RC.start(8); R.step(5); const c = R.silent(), fresh = R.team[0] !== cast0;
  verdict(!b && why === null && fresh && c, 'the order takes it out — and a restart stands the NEW squad up silent again', `after "go loud" ${b} · restart ${why} · new men ${fresh} · silent ${c}`);
  // … and clean: the last squad's noise is not the new one's (a squad compromised, then a restart that stands the next one up silent as well)
  { game.fight.hotT = game.time; R.step(3); const was = /compromised/.test(R.label()); const old = R.team[0]; RC.start(10); R.step(5); const Md = R.door('manager'); Md.lockId = `wedge:${Md.def.name}`;
    verdict(was && R.team[0] !== old && R.silent() && !/compromised/.test(R.label()) && SLM.silentBoot(game, Md) === null, 'a silent squad that was compromised, then a restart: the new squad is silent and NOT compromised (a wedged door is not booted on the last squad\'s noise)',
      `before '${was ? 'compromised' : 'clean'}' · after '${R.label()}' · a wedged door asked: ${SLM.silentBoot(game, Md)}`); Md.lockId = Md.def.lock ?? Md.lockId; RC.start(8); R.step(5); }
  // a man leaving the roster (a cast changed, not replaced) leaves Sam's word standing; a showcase's cast is never silent
  const gone = game.guards.indexOf(R.team[3]), man = game.guards.splice(gone, 1)[0]; R.step(3); const kept = R.silent(); game.guards.splice(gone, 0, man); R.step(3); const kept2 = R.silent();
  game.touring = true; const show = { squad: R.silent(), man: SLM.underSilence(game, R.team[0]), pace: SLM.silentPace(game, R.team[0], 2.2) }; game.touring = false;
  verdict(kept && kept2 && !show.squad && !show.man && show.pace === 2.2 && R.silent(), 'one man off the roster and back: still silent; under a showcase / the tour no rule applies (the squad is not under Sam\'s orders there)',
    `three men ${kept} · four again ${kept2} · touring: silent ${show.squad}, pace asked 2.2 → ${show.pace}`);
  SILENT.start = false; RC.start(9); R.step(5);
  verdict(!R.silent() && /^squad · loud · /.test(R.label()), 'start off (shipped): the next squad stands up loud, whatever the last one was', `silent ${R.silent()} · '${R.label()}' · ${SLM.silentSummary(game)}`);
  return done();
});

// ---------------------------------------------------------------- off / never given
/** a scripted minute — fall in, stack, open and clear with the room's men alive — hashed every frame: the digest, and every man's light, state, place, aim */
async function minute(o: RigOpts, pre?: (R: any) => void): Promise<string> {
  const R = await rig(11, o); const { game, O } = R; pre?.(R); let h = 0x811c9dc5 >>> 0;
  const eat = (s: string) => { for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } };
  const frame = (n: number) => { for (let f = 0; f < n; f++) { R.step(); eat(JSON.stringify(digestGame(game, true))); for (const x of game.guards) eat(`${x.flashlight?.enabled ? 1 : 0}${x.state[0]}${x.char.pos[0].toFixed(4)},${x.char.pos[2].toFixed(4)},${x.char.aimYaw.toFixed(4)},${x.bubble?.radio ? 1 : 0}`); } };
  const sam = game.player.char.pos; O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all'); frame(120);
  R.giveDoor('stack', 'breakroom_w'); frame(60 * 12); R.giveDoor('openClear', 'breakroom_w'); frame(60 * 25); R.giveDoor('threshold', 'manager'); frame(60 * 25);
  return `${(h >>> 0).toString(16).padStart(8, '0')} · ${game.guards.filter((x: any) => x.state !== 'dead').length} standing at ${game.time.toFixed(2)} s · kicks ${R.kicks} bangs ${R.bangs}`;
}

// ---------------------------------------------------------------- a silent stack beside unaware men is set
/** SILENTSET_BREAK=1: the pair is never told silent — the lines about the rule must fail */
const SET_BREAK = process.env.SILENTSET_BREAK === '1';
export const silentSet = guarded(async () => {
  const { verdict, done } = family('silent-set: a silent man on his stack mark whose round is held on a man is stacked');
  const off = (o: any, at: number[]) => { const e = Math.atan2(at[0] - o.char.pos[0], at[2] - o.char.pos[2]) - o.char.aimYaw; return Math.abs(Math.atan2(Math.sin(e), Math.cos(e))); };
  // the break room's west door stood open, Sam inside by the north door, its two men kept unaware: the silent pair is told to stack on the north door from inside
  const run = async (o: RigOpts = {}, locked = false) => { const R = await rig(7, { squad: 2, ...o }); const { game, O, D } = R;
    const d = R.door('breakroom_n'), dw = R.door('breakroom_w'), said: string[] = [], say0 = game.say; game.say = function (this: any, gd: any, text: string, ...r: any[]) { said.push(text); return say0.call(this, gd, text, ...r); };
    R.post = () => { for (const h of R.hostiles) if (h.state !== 'dead') { h.state = 'patrol'; h.awareness = 0; } };
    if (!SET_BREAK) R.give('silent', AISLE); dw.angle = (dw.swing ?? 1) > 0 ? dw.maxA : dw.minA; dw.latched = false; dw.vel = 0; dw.quietT = -3600; game.teleportPlayer([30.75, 0, 14.4]); R.step(60);
    if (locked) { d.locked = true; d.latched = true; d.keySide = game.doors.side(d, game.player.char.pos); }   // (locked against the side they stack on)
    const h0 = SLM.SILENT_STATS.held, t0 = D.stats.tested, up0 = R.hostiles.filter((h: any) => h.state !== 'dead').length, w = R.giveDoor('stack', 'breakroom_n'), men = () => R.team.filter((x: any) => x.state !== 'dead');
    const all = () => men().length === 2 && men().every((x: any) => O.of(x)?.order === 'stack' && O.of(x).arrived);
    let secs = 0; for (; secs < 30 && !all(); secs++) R.step(60);
    let left = 0, n = 0; for (let i = 0; i < 60 * 8; i++) { R.step(); for (const x of men()) { n++; if (!O.of(x)?.arrived) left++; } }
    const rows = men().map((x: any) => { const A = O.of(x), t = game.fight.opOf(x)?.tgt; return { cs: x.callsign as string, arrived: !!A?.arrived, mark: A?.post ? dist(x.char.pos, A.post) : NaN, man: t ? off(x, t.char.pos) : NaN, door: off(x, d.frameCentre), tgt: (t?.callsign ?? 'nobody') as string, calm: t?.state === 'patrol', rule: silentHoldsTarget(game, x) as boolean, word: O.statusOf(x) as string }; });
    const out = { R, w, set: all(), secs, left, n, rows, rounds: R.opShots, held: SLM.SILENT_STATS.held - h0, tested: D.stats.tested - t0, sets: said.filter(x => x === 'set.').length, lockedSaid: said.filter(x => /locked/.test(x)).length, leaf: Math.abs(d.angle), silent: R.silent(), up0, up: R.hostiles.filter((h: any) => h.state !== 'dead').length };
    game.say = say0; return out; };
  const show = (a: Awaited<ReturnType<typeof run>>) => `${a.w ? `refused: ${a.w}` : 'given'} · set ${a.set} after ${a.secs} s · man-frames not arrived in the 8 s after ${a.left}/${a.n} · 'set.' said ${a.sets} · team rounds ${a.rounds} · frames a round was held ${a.held} · handles tried ${a.tested} · 'locked' said ${a.lockedSaid} · leaf ${f2(a.leaf)} · still silent ${a.silent} · defenders up ${a.up}/${a.up0} · ${a.rows.map(m => `${m.cs}: ${f2(m.mark)} m off his mark, gun ${f2(m.man)} rad off ${m.tgt} and ${f2(m.door)} rad off the doorway, held on him by the order ${m.rule}, "${m.word}"`).join(' · ')}`;
  const onMarks = (a: Awaited<ReturnType<typeof run>>) => !a.w && a.rows.length === 2 && a.rows.every(m => m.mark < 0.35 && m.calm && m.man < 0.2 && m.door > 0.2) && a.rounds === 0 && a.held > 0 && a.silent && a.up === a.up0 && a.up > 0 && a.leaf < 0.05;
  const A = await run();
  verdict(onMarks(A) && A.set && A.left === 0 && A.sets === 1 && A.rows.every(m => m.arrived && m.rule && m.word.startsWith('stacked')),
    'a silent pair told to stack inside a room with unaware men in view: on its marks, each gun held on a man and not on the door, the pair is SET — \'set.\' said once, the rows say stacked, not a round fired', show(A));
  const C = await run({}, true);
  verdict(onMarks(C) && C.set && C.left === 0 && C.sets === 1 && C.tested === 0 && C.lockedSaid === 0 && C.rows.every(m => m.rule && m.word.startsWith('stacked')),
    'the door LOCKED and the handle tried before entry (as the page ships): set and held on a man, no hand goes to the handle — nobody says \'locked\', the pair stays stacked', show(C));
  // … and when the men they were held on are gone, the hand goes to the handle only once the gun is the door's again (the frame before each try, as testStacks sees it)
  { const { R } = C, { game, O, D } = R, t0 = D.stats.tested; let tried = -1, worst = 0, lockedSaid = 0;
    const say0 = game.say; game.say = function (this: any, gd: any, text: string, ...r: any[]) { if (/locked/.test(text)) lockedSaid++; return say0.call(this, gd, text, ...r); };
    R.post = null; for (const h of R.hostiles) if (h.state !== 'dead') killGuard(game, h, [0, 0, 1], true);
    for (let i = 0; i < 60 * 15 && tried < 0; i++) { const pre = R.team.filter((x: any) => x.state !== 'dead' && O.of(x)?.stance === 'stack' && O.of(x).watch).map((x: any) => off(x, O.of(x).watch));
      R.step(); if (D.stats.tested > t0) { tried = i; worst = Math.max(0, ...pre); } }
    game.say = say0;
    verdict(SET_BREAK || (tried > 0 && worst < 0.2 && lockedSaid > 0), 'the men they were held on are gone: the lead tries the handle only with his gun back on the door, and says what he found',
      `handle tried ${tried < 0 ? 'never in 15 s' : `${f2(tried / 60)} s after`} · the stacked lead's gun ${f2(worst)} rad off his door the frame before · 'locked' said ${lockedSaid}`); }
  // the rule's own edges, asked of the function on the set pair: a man who has noticed, a loud squad
  { const { R } = A, { game } = R, m = R.team[0], t = game.fight.opOf(m)?.tgt; const calm = silentHoldsTarget(game, m) as boolean;
    let noticed = true, loud = true;
    if (t) { t.state = 'alert'; noticed = silentHoldsTarget(game, m); t.state = 'patrol'; }
    R.give('loud', AISLE); loud = silentHoldsTarget(game, m);
    verdict(!!t && calm && !noticed && !loud, 'the question is the order\'s own hold: true on the unaware man; false once he is alert, and for a squad told loud',
      `on ${t?.callsign ?? 'nobody'}, unaware: ${calm} · him alert: ${noticed} · told loud: ${loud}`); R.post = null; }
  return done();
});

if (import.meta.main) {
  const all: Record<string, () => Promise<boolean>> = { 'silent-order': silentOrder, 'silent-hud': silentHud, 'silent-walk': silentWalk, 'silent-doors': silentDoors, 'silent-pick': silentPick, 'silent-boot': silentBoot, 'silent-cans': silentCans, 'silent-tools': silentTools, 'silent-fire': silentFire, 'silent-hush': silentHush, 'silent-start': silentStart, 'silent-set': silentSet };
  process.exit(exitCode(await asProbes(all, wanted())));
}
