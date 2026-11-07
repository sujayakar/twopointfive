// The 'holdglass' family (game/silent.ts roundHeldGlass — the maintainer, 10-08: "for shooting through conference glass it should depend if they're loud or
// not. accept if loud, hold fire if they're trying to stay quiet"). Headless: the whole Game over the fake device, the room clear's pair on the office with
// rounds through glass live, as on the page (a settled rule, as the unfound squad's hold and the silent hold are: nothing here writes them), the two teammates posted in the corridor on 'cover that way' with their guns on one
// defender the far side of the conference partition (z 10: plain float glass), that defender's state written back every frame so the only thing that differs
// between the cases is what the rule is asked about.
//
//   (until 2026-11-07 a probe `holdglass-loud` staged a loud pair nobody had found with the unfound squad's hold written off, to shoot "as before the rule":
//    under the page's hold such a pair holds, so its premise is gone and it is deleted; `other` lost its line for holdFire off)
//   holdglass-quiet     a silent pair: not one round through the glass at a man who has not noticed (already so before the rule: the rifle's report), NOR at
//                       one who has made them out but raised no alarm (before the rule: the hold let go at awareness 0.5 and the first round went through the
//                       pane — a 'glass' event at 0.85 the floor hears at 30 m), NOR from a quiet gun (before the rule: never held at all); they say so
//   holdglass-release   what ends the hold: the man ALERT or SEARCHING · something loud in the teammate's own ears since the order · the captor's last warning ·
//                       told loud
//   holdglass-other     while they hold on him an alert man on their side of the glass is shot, promptly and without the muzzle going to and fro (the held man
//                       is not one to stay on); two men behind the glass: no round, the muzzle settles; their guns on a calm man their side, the man behind the
//                       glass goes alert: he is the one shot
//   holdglass-open      no glass on the line: a quiet gun on a silent man shoots as ever (the rule is about the pane)
//
//   bun run tools/qa/holdglass-probes.ts [name…]         (also `probes.ts holdglass`)
// On the build before the rule (no SILENT.glass) the noticed-man and quiet-gun lines of holdglass-quiet are the red ones.
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { MOVES } = await import(`${ROOT}/src/game/character.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const GPM: any = await import(`${ROOT}/src/game/glasspen.ts`);
const KITM: any = await import(`${ROOT}/src/game/kit.ts`);
const SILENT: any = SLM.SILENT;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const clone = (o: any) => JSON.parse(JSON.stringify(o));
const DEFAULTS = { NIGHTBREACH: { ...NIGHTBREACH }, SILENT: clone(SILENT), KIT: clone(KITM.KIT.members), hook: GPM.GLASSPEN_HOOKS.crossed };
const reset = () => { Object.assign(NIGHTBREACH, DEFAULTS.NIGHTBREACH); Object.assign(SILENT, clone(DEFAULTS.SILENT)); KITM.KIT.members = clone(DEFAULTS.KIT); GPM.GLASSPEN_HOOKS.crossed = DEFAULTS.hook; };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };

type Vec3 = [number, number, number];
/** where the pair stands in the corridor, the defender behind the conference glass, the defender down the open corridor, and Sam out of every line */
const POST: Vec3[] = [[6.6, 0, 12.4], [8.0, 0, 12.4]], BEHIND: Vec3 = [7.2, 0, 6.2], SAM: Vec3 = [3.2, 0, 12.6];
const POST_OPEN: Vec3[] = [[6.6, 0, 10.9], [8.0, 0, 12.2]], OPEN: Vec3 = [16, 0, 11.2], SECOND: Vec3 = [8.6, 0, 10.6];
/** against the partition itself (a man's radius off the pane), and a second place behind the glass */
const AT_PANE: Vec3 = [7.2, 0, 9.66], BEHIND2: Vec3 = [5.6, 0, 6.8];
type Foe = 'calm' | 'noticed' | 'alert' | 'search';
interface Stage { silent?: boolean; foe?: Foe; open?: boolean; quietGun?: boolean; secs?: number;
  /** where the defender stands (default: BEHIND the glass, or OPEN down the corridor) */ foeAt?: Vec3;
  /** a second defender — ALERT on the pair's side of the glass (SECOND) unless said otherwise — from this many seconds in */ second?: number; secondAt?: Vec3; secondMode?: Foe;
  /** run after the posture is given and the men are posted, before the clock starts */ then?: (R: any) => void }

async function stage(seed: number, o: Stage = {}) {
  reset(); Object.assign(NIGHTBREACH, { squad: 2 });
  if (o.quietGun) for (const i of [1, 2]) KITM.KIT.members[i] = KITM.kitPresetCard(i, 'quiet');   // (kit.ts: the Quiet set — the suppressed Five-seveN, a report under a gunshot's)
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game;
  Object.assign(SILENT, { on: true });
  const RC = game.roomClear, NB = game.nightBreach, O = game.orders;
  game.godMode = true;
  const said: string[] = []; { const say = game.say.bind(game); game.say = (gd: any, t: string, ...rest: any[]) => { said.push(t); return say(gd, t, ...rest); }; }
  RC.start(seed);
  const crossed: { bi: number; who: number }[] = []; GPM.GLASSPEN_HOOKS.crossed = (_g: any, x: any, who: number) => { crossed.push({ bi: x.bi, who }); };
  const team: any[] = O.squad(); const hostiles: any[] = NB.hostiles; const foe = hostiles[0], at = o.foeAt ?? (o.open ? OPEN : BEHIND);
  const park = (h: any, p: Vec3, yaw: number) => { h.char.pos = [...p]; h.char.vel = [0, 0, 0]; h.speed = 0; h.char.bodyYaw = h.char.aimYaw = yaw; h.path = []; h.pathGoal = null; h.task = null; h.script = null;
    h.hold = true; h.pinned = true; h.dazzledUntil = 1e9; h.heardUpTo = 1e12; h.flashlight.enabled = false; h.char.update(0); };
  const other = hostiles[1]; let otherIn = false;
  hostiles.forEach((h: any, i: number) => { if (h !== foe) park(h, [37.5, 0, 25 + i * 1.2], 0); });   // the rest of the room's men: out in the lot, seeing and hearing nothing
  const mode: { foe: Foe; other: Foe } = { foe: o.foe ?? 'calm', other: o.secondMode ?? 'alert' };
  const freeze = () => { for (const h of hostiles) { if (h.state === 'dead') continue;
    const m: Foe = h === foe ? mode.foe : h === other && otherIn ? mode.other : 'calm';
    if (m === 'calm') { h.state = 'patrol'; h.awareness = 0; } else if (m === 'noticed') { h.state = 'suspicious'; h.awareness = 0.7; } else { h.state = m; h.awareness = 1; } } };
  // per teammate: how often his muzzle went to a DIFFERENT man (roomfight's own target), and when the second defender went down
  const watch = { last: new Map<any, any>(), switches: 0, otherDownAt: -1 };
  // (the man behind the glass is kept inert by a standing dazzle — park, below. The motion pass the page ships (character.ts MOVES.dazzle) folds a dazzled man at the
  //  waist, and behind the conference partition that takes his chest and head under the pair's sight line: nobody sees him and no rule here is asked anything. So the
  //  stage's own frames run with that one pose off — he stands as the stage means him to — and the knob is put back after every call: nothing leaks to the next probe.)
  const fold = MOVES.dazzle; const upright = <T>(fn: () => T): T => { MOVES.dazzle = 0; try { return fn(); } finally { MOVES.dazzle = fold; } };
  const step = (n = 1) => upright(() => { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); freeze();
    for (const X of game.fight.ops ?? []) { if (!X.tgt) continue; if (watch.last.has(X.gd) && watch.last.get(X.gd) !== X.tgt) watch.switches++; watch.last.set(X.gd, X.tgt); }
    if (other.state === 'dead' && watch.otherDownAt < 0) watch.otherDownAt = game.time; } });
  park(foe, at, o.open ? Math.PI / 2 : Math.PI);        // his back to the pair
  game.teleportPlayer([...SAM]); freeze(); step(2);
  team.forEach((m: any, i: number) => { m.char.pos = [...(o.open ? POST_OPEN : POST)[i]]; m.char.vel = [0, 0, 0]; m.path = []; m.pathGoal = null; m.char.update(0); });
  const err = [o.silent ? O.give('silent', O.targetAt([10, 0, 12.5])) : null, O.give('cover', O.targetAt([at[0], 0, at[2]]))].filter(Boolean);
  const fired = () => team.reduce((n: number, m: any) => n + m.weapon.shotsFired, 0);
  const chest = (c: any): Vec3 => [c.pos[0], c.pos[1] + 1.25, c.pos[2]];
  const panes: { bi: number; t: number }[] = []; game.col.segmentBlocked(chest(team[0].char), chest(foe.char), 0.02, 0, panes);
  const R = { W, game, team, foe, other, hostiles, said, crossed, mode, step, fired, err, watch, panesOnLine: panes.length, solid: game.col.solidBlocked(chest(team[0].char), chest(foe.char)) as boolean,
    guns: team.map((m: any) => m.weapon.def.id as string), silent: () => SLM.squadSilent(game) as boolean, label: () => (O.hud().status[0]?.label ?? '') as string };
  o.then?.(R);
  const f0 = fired(), h0 = SLM.SILENT_STATS.held, g0 = SLM.SILENT_STATS.heldGlass ?? 0;
  watch.switches = 0; let secondT = -1;
  if (o.second !== undefined) { step(Math.round(o.second * 60)); park(other, o.secondAt ?? SECOND, 0); otherIn = true; secondT = game.time; }
  step(Math.round((o.secs ?? 9) * 60));
  const eyes = team.filter((m: any) => (game.fight.ops ?? []).some((X: any) => X.gd === m && X.tgt === foe)).length;
  return { ...R, shots: fired() - f0, through: crossed.filter(x => team.some((m: any) => m.char.id === x.who)).length, held: SLM.SILENT_STATS.held - h0, heldGlass: (SLM.SILENT_STATS.heldGlass ?? 0) - g0,
    down: foe.state === 'dead', eyes, switches: watch.switches, otherAfter: watch.otherDownAt < 0 ? -1 : watch.otherDownAt - secondT, holding: said.filter(t => /holding/.test(t)).length, glassLine: said.filter(t => /behind glass/.test(t)).length };
}
const row = (a: any) => `rounds ${a.shots} · through glass ${a.through} · he is ${a.down ? 'down' : 'up'} · frames held ${a.held} (for the glass ${a.heldGlass}) · "holding" ×${a.holding}`;

export const holdglassQuiet = guarded(async () => {
  const { verdict, done } = family('holdglass-quiet: a silent pair holds its fire while the glass is between them and a man who has raised no alarm');
  const calm = await stage(11, { silent: true, foe: 'calm' });
  verdict(calm.silent() && calm.shots === 0 && calm.through === 0 && !calm.down && calm.held > 0 && calm.holding > 0 && calm.eyes === 2, 'silent, rifles, a man who has not noticed: not one round, both guns stay on him, and they say they are holding', `${row(calm)} · guns on him ${calm.eyes} of 2`);
  const seen = await stage(11, { silent: true, foe: 'noticed' });
  verdict(seen.silent() && seen.shots === 0 && seen.through === 0 && !seen.down && seen.heldGlass > 0 && seen.glassLine > 0, 'silent, rifles, a man who has made them out but raised no alarm: still not one round through the pane (red before the rule)', `${row(seen)} · "behind glass" ×${seen.glassLine}`);
  const gun = await stage(11, { silent: true, foe: 'calm', quietGun: true });
  verdict(gun.guns.every((id: string) => id === 'fiveseven') && gun.shots === 0 && gun.through === 0 && !gun.down && gun.heldGlass > 0 && gun.glassLine > 0, 'silent, the quiet pistols, a man who has not noticed: the round is quiet but the pane is not — held (red before the rule)', `${row(gun)} · guns ${gun.guns.join(', ')}`);
  const near = await stage(11, { silent: true, foe: 'noticed', foeAt: AT_PANE });
  verdict(near.panesOnLine > 0 && near.shots === 0 && near.through === 0 && !near.down && near.heldGlass > 0, 'the same man stood right against the pane, looking out: the glass is still in the way — held', row(near));
  const seeds = [12, 13]; const more = []; for (const sd of seeds) more.push(await stage(sd, { silent: true, foe: 'noticed' }));
  verdict(more.every(a => a.shots === 0 && a.through === 0 && !a.down && a.heldGlass > 0), `other dice (seeds ${seeds.join(', ')}): the same hold`, more.map(row).join(' | '));
  return done();
});

export const holdglassRelease = guarded(async () => {
  const { verdict, done } = family('holdglass-release: what lets a silent teammate shoot through the glass');
  const alert = await stage(11, { silent: true, foe: 'alert' });
  verdict(alert.silent() && alert.shots > 0 && alert.through > 0 && alert.down, 'the man is ALERT (the alarm is his already): shot through the glass, the order still standing', row(alert));
  const hunt = await stage(11, { silent: true, foe: 'search' });
  verdict(hunt.silent() && hunt.shots > 0 && hunt.through > 0 && hunt.down, 'the man is SEARCHING (an alarm raised and he is hunting them): shot through the glass, as a searching man is anywhere', row(hunt));
  const turn = await stage(11, { silent: true, foe: 'noticed', secs: 7 }); const before = { shots: turn.shots, down: turn.down }; turn.mode.foe = 'alert'; const f1 = turn.fired(); turn.step(60 * 6);
  verdict(before.shots === 0 && !before.down && turn.fired() - f1 > 0 && turn.foe.state === 'dead', 'held while he only looks; the moment he goes alert the rounds go', `7 s suspicious: rounds ${before.shots} · then alert: rounds ${turn.fired() - f1}, he is ${turn.foe.state === 'dead' ? 'down' : 'up'}`);
  const hot = await stage(11, { silent: true, foe: 'noticed', then: R => { R.step(30); R.game.fight.hotT = R.game.time; } });
  verdict(hot.shots > 0 && hot.through > 0 && hot.down && /compromised/.test(hot.label()), 'something loud in their ears since the order (compromised): they shoot as a loud pair does', `${row(hot)} · ${hot.label()}`);
  const last = await stage(11, { silent: true, foe: 'noticed', then: R => { R.game.rescue.lastWarning = () => true; } });
  verdict(last.shots > 0 && last.through > 0 && last.down, 'a captor at his last warning: nothing is held (silent never costs the hostage)', row(last));
  const loud = await stage(11, { silent: true, foe: 'noticed', secs: 6 }); const l0 = { shots: loud.shots }; loud.game.orders.give('loud', loud.game.orders.targetAt([10, 0, 12.5])); const f2 = loud.fired(); loud.step(60 * 6);
  verdict(l0.shots === 0 && !loud.silent() && loud.fired() - f2 > 0 && loud.foe.state === 'dead', 'told loud: the hold is gone with the order', `silent 6 s: rounds ${l0.shots} · told loud: rounds ${loud.fired() - f2}, he is ${loud.foe.state === 'dead' ? 'down' : 'up'}`);
  return done();
});

export const holdglassOther = guarded(async () => {
  const { verdict, done } = family('holdglass-other: a man held for the glass is not a man to stay on — anybody else they may shoot takes the muzzle');
  // the quiet pistols (a round at the second man is then nothing loud in their ears: the hold on the first must still stand afterwards)
  const A = await stage(11, { silent: true, foe: 'noticed', quietGun: true, second: 4, secs: 8 });
  verdict(A.shots > 0 && A.other.state === 'dead' && !A.down && A.through === 0 && A.heldGlass > 0, 'holding on a man behind glass, an alert man steps out on their side of it: he is shot, the man behind the glass still is not', `${row(A)} · the alert man is ${A.other.state === 'dead' ? 'down' : 'up'}`);
  verdict(A.otherAfter >= 0 && A.otherAfter < 2.5 && A.switches <= 4, '… promptly, and no muzzle goes to and fro between the two (each man: off the held man, onto the alert one, back when he is down)', `the alert man down ${A.otherAfter.toFixed(2)} s after he appeared · muzzle changes of man, both teammates: ${A.switches}`);
  const two = await stage(11, { silent: true, foe: 'noticed', second: 2, secondAt: BEHIND2, secondMode: 'noticed', secs: 12 });
  verdict(two.shots === 0 && two.through === 0 && !two.down && two.other.state !== 'dead' && two.heldGlass > 0 && two.switches <= 10, 'two men behind the glass, neither has raised an alarm: not a round, and the muzzles settle (a look at the other man every few seconds at most)', `${row(two)} · muzzle changes of man, both teammates, 12 s: ${two.switches}`);
  const B = await stage(11, { silent: true, foe: 'noticed', second: 2, secondMode: 'calm', secs: 5 }); const b0 = { shots: B.shots, through: B.through }; B.mode.foe = 'alert'; const tB = B.game.time; let downB = -1; for (let i = 0; i < 60 * 5 && downB < 0; i++) { B.step(1); if (B.foe.state === 'dead') downB = B.game.time - tB; }
  verdict(b0.shots === 0 && downB >= 0 && downB < 3 && B.crossed.length > 0, 'rifles: a calm man on their side (held for the report) and a man behind the glass (held for the pane) — he goes ALERT: he is the one shot, through the glass, at once', `5 s: rounds ${b0.shots} · he goes alert: down ${downB < 0 ? 'never' : downB.toFixed(2) + ' s later'} · panes crossed ${B.crossed.length}`);
  return done();
});

export const holdglassOpen = guarded(async () => {
  const { verdict, done } = family('holdglass-open: no glass on the line — the silent rules as they were');
  const gun = await stage(11, { silent: true, foe: 'calm', open: true, quietGun: true });
  verdict(gun.panesOnLine === 0 && !gun.solid && gun.guns.every((id: string) => id === 'fiveseven') && gun.shots > 0 && gun.through === 0 && gun.down && gun.heldGlass === 0, 'silent, the quiet pistols, a man down the open corridor who has not noticed: shot (a quiet gun is not held)', `${row(gun)} · panes on the line ${gun.panesOnLine}`);
  const seen = await stage(11, { silent: true, foe: 'noticed', open: true });
  verdict(seen.shots > 0 && seen.down && seen.heldGlass === 0, 'silent, rifles, he has made them out and nothing but air between: shot, as before the rule', row(seen));
  return done();
});

if (import.meta.main) {
  const all: Record<string, () => Promise<boolean>> = { 'holdglass-quiet': holdglassQuiet, 'holdglass-release': holdglassRelease, 'holdglass-other': holdglassOther, 'holdglass-open': holdglassOpen };
  const want = process.argv.slice(2).filter(a => !a.startsWith('--')); let ok = true;
  for (const [name, fn] of Object.entries(all)) if (!want.length || want.includes(name)) ok = (await fn()) && ok;
  process.exit(ok ? 0 : 1);
}
