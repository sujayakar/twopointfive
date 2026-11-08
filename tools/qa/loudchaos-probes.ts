// The 'loudchaos' probe family — destruction's CHAOS on the loud levels (game/destructionSwitch.ts on: destruction.ts's loud rules, the levels' dressing —
// tools/levels/dress.ts; glass is NOT this part's — it is left as the base has it), run through probes.ts:
//   bun run tools/qa/probes.ts loudchaos | loudchaos-levels | loudchaos-firefight | loudchaos-blast | loudchaos-fixtures | loudchaos-off [--verbose]
// Headless = the Proxy device, a recording smoke stand-in, a probe that never answers; what is asserted is every CPU-side promise:
//   (levels)     each loud level file carries its dressing LAST under the group 'dressing'; the file without that group is the generator's own level, box for
//                box; destruction's index finds paper, clutter, binders and screens on it — all under a roof, none of them a fixture;
//   (firefight)  per level, a burst over a dressed table into the wall behind it and into the things on it: paper up, things knocked off as debris, dust into the
//                smoke — every count inside its cap (puffs a second / a frame, knocks a second, the debris pool), nothing NaN, and restartEncounter puts it all back;
//   (blast)      per level, a charge beside a dressed surface: paper up in a ring, the ceiling over it answers (a tile down, or the light panel in that square out),
//                dust off the ceiling AND off the floor, fixtures within reach and in view dark; out of doors no tile and no ceiling dust; `power` scales the reach;
//   (fixtures)   a round that is not the player's puts out a fixture only when it is IN it (its box, or within 18 cm of its flight), a guard with a line to it notices
//                (the 'lightOut' event the OCP always filed); the per-encounter cap and the gap hold; breakers, devices and scenery never;
//   (sight)      the AI's sight through dust is the rule it always was: guards.ts reads ONE number per sight line off the Sensors segment query and blocks under
//                0.3 / scales to 0.8 — planted answers move it exactly so; the harness's CPU dust stand-in (dust-standin.ts) feeds that
//                same query (a cloud between two points dims the line, a cloud behind a wall does not, no puff = no answer);
//   (off)        with NO scenario up the loud rules stay down (destruction.ts loudNow): a round stokes the office's own heat, the player-only fixture rule
//                stands alone, a blast under a lit panel leaves it and raises no floor dust. LOUDOFF_BREAK=1 = the floor's scenario is started: must fail.
// 2026-11-07: the destruction switch is a settled rule (on), and a write of its other side is refused. Gone with that: every 'with the switch off' arm (the
// blast's power ignored, a teammate's round leaving the panel, the sight rule 'identically on and off', the whole of 'the switch off = destruction as it was'
// and the glass lines laid beside it), the stun canister's two arms on the knobs' old values (CHAOS.loud.stunPanels is settled too), and the switch put back
// off after every probe. RESTATED, the question kept: 'off = the generator's level' is asked of the FILE WITHOUT ITS DRESSING GROUP, loaded as it is, and the
// nav grid of the dressed floor is held against the floor stood from that same file without the group (level data changed before the stand-up).
import { sample } from './sweep.ts';
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { DRESS_GROUP } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { fireWeapon, detonateStun } = await import(`${ROOT}/src/game/combat.ts`);
const fx = await import(`${ROOT}/src/game/effects.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const D = await import(`${ROOT}/src/game/destruction.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { NOBODY_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { installDustStandin } = await import('./dust-standin.ts');

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
/** the loud levels: file, generator, the query that stages its scenario — tools/levels/dress.ts's own list (what export.ts dresses by) */
const { LOUD_LEVELS } = await import(`${ROOT}/tools/levels/dress.ts`);
export const LOUD: { file: string; gen: string; q: string }[] = LOUD_LEVELS;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => x.toFixed(2);
const p2 = (p: Vec3) => `(${f2(p[0])}, ${f2(p[1])}, ${f2(p[2])})`;
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { const ok = all; console.log(`=== ${title.split(':')[0]}: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) process.exitCode = 1; return ok; } };
}
const resetStats = () => { for (const k of Object.keys(D.DESTR_STATS)) (D.DESTR_STATS as any)[k] = 0; D.DESTR_LOG.length = 0; };
const dataOf = (file: string) => parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'));
async function quiet<T>(fn: () => Promise<T> | T): Promise<T> { if (VERBOSE) return await fn(); const l = console.log, w = console.warn, e = console.error; console.log = console.warn = console.error = () => {}; try { return await fn(); } finally { console.log = l; console.warn = w; console.error = e; } }

interface Stage { W: any; game: any; smokeLog: any[]; items: { i: number; kind: string; box: any }[] }
/** A loud level stood up at chaos 0.5 (the page's), god mode, the AI off, a smoke stand-in that RECORDS emits (stamped with the game clock) — and, unless `scn`
 *  is false, its SCENARIO started (the loud rules are live only under one: destruction.ts loudNow), everybody standing where he was staged. `data`: the level
 *  stood from this data in place of the file's own (the file without its dressing). */
async function stage(file: string, o: { scn?: boolean; data?: any } = {}): Promise<Stage> {
  const scn = o.scn ?? true; Math.random = seeded(4401); resetStats();
  const W: any = await quiet(() => standUp({ uploadEvery: 1, level: loadLevel(o.data ?? dataOf(file)) })); const game = W.game;
  game.tune.chaos = 0.5; game.godMode = true; game.aiEnabled = false; game.playerInvisible = true;
  const smokeLog: any[] = []; const emit0 = W.smoke.emit.bind(W.smoke); W.smoke.emit = (e: any) => { smokeLog.push({ ...e, t: game.time }); emit0(e); };
  const items: Stage['items'] = []; for (let i = 0; i < game.col.boxes.length; i++) { const d = game.destr.itemOf(i); if (d) items.push({ i, kind: d.kind, box: d.box }); }
  if (scn) { const L = LOUD.find(l => l.file === file)!; game.aiEnabled = true; await quiet(() => startFromQuery(game, new URLSearchParams(`levelFile=${file}&${L.q}`))); game.aiEnabled = false; }
  for (let f = 0; f < 5; f++) W.step(DT);
  return { W, game, smokeLog, items };
}
/** a floor point 2 to 4.5 m from `p` with a clear line (at gun height) to just over it, nothing solid where a man would stand: where the probe's shooter stands */
function standNear(game: any, p: Vec3, min = 2, max = 4.5, lift = 0.12): Vec3 | null {
  for (let r = min; r <= max; r += 0.5) for (let k = 0; k < 16; k++) {
    const a = k * Math.PI / 8 + r; const q: Vec3 = [p[0] + Math.cos(a) * r, 0, p[2] + Math.sin(a) * r];
    if (q[0] < 1 || q[0] > 39 || q[2] < 1 || q[2] > 27) continue;
    if (game.col.solidBlocked([q[0], 1.4, q[2]], [p[0], p[1] + lift, p[2]]) || game.col.solidBlocked([q[0], 0.2, q[2]], [q[0], 1.7, q[2]])) continue;
    let clear = true; for (const [dx, dz] of [[0.3, 0], [-0.3, 0], [0, 0.3], [0, -0.3]]) if (game.col.solidBlocked([q[0], 0.9, q[2]], [q[0] + dx, 0.9, q[2] + dz])) clear = false;
    if (clear) return q;
  }
  return null;
}
/** the most crowded dressed spot: the paper stack with the most other destructibles within 1.2 m of it */
function busiest(items: Stage['items']): Stage['items'][number] | null {
  let best: Stage['items'][number] | null = null, bn = -1;
  for (const it of items) { if (it.kind !== 'paper') continue; let n = 0; for (const o of items) if (o !== it && Math.hypot(o.box.c[0] - it.box.c[0], o.box.c[2] - it.box.c[2]) < 1.2) n += o.kind === 'clutter' || o.kind === 'binder' ? 2 : 1; if (n > bn) { bn = n; best = it; } }
  return best;
}
/** stand Sam at `at` facing `to` */
function face(S: Stage, at: Vec3, to: Vec3) { const { W, game } = S; game.teleportPlayer(at); const pc = game.player.char; pc.bodyYaw = pc.aimYaw = Math.atan2(to[0] - at[0], to[2] - at[2]); for (let f = 0; f < 30; f++) { W.cursorAt(to); W.step(DT); } }   // (the cursor on the thing: his gun comes round to it, not into the wall beside him)
/** where a charge beside the dressing does the most: the floor point 0.9 to 1.6 m from a paper stack with the most stacks in the blast's own view (from just over desk height, as the module judges it) */
function chargeSpot(game: any, items: Stage['items']): Vec3 | null {
  let best: Vec3 | null = null, bn = 0; const papers = items.filter(o => o.kind === 'paper');
  for (const it of papers) { const q = standNear(game, it.box.c, 0.9, 1.6); if (!q) continue; const eye: Vec3 = [q[0], 0.95, q[2]]; let n = 0; for (const o of papers) if (v3.dist(o.box.c, q) < 2.4 && !game.col.solidBlocked(eye, [o.box.c[0], o.box.c[1] + o.box.h[1] + 0.05, o.box.c[2]])) n++; if (n > bn) { bn = n; best = q; } }
  return best;
}
const capsHeld = (smokeLog: any[], game: any): string[] => {
  const bad: string[] = []; const puffs = smokeLog.filter(e => e.kind === 'puff').map(e => e.t);
  for (let i = 0; i < puffs.length; i++) { let n = 0; for (let j = i; j < puffs.length && puffs[j] - puffs[i] < 1 - 1e-9; j++) n++; if (n > D.CHAOS.puffsPerSec) { bad.push(`${n} puffs inside one second at t ${f2(puffs[i])}`); break; } }
  const perFrame = new Map<string, number>(); for (const t of puffs) perFrame.set(t.toFixed(4), (perFrame.get(t.toFixed(4)) ?? 0) + 1); for (const [t, n] of perFrame) if (n > D.CHAOS.puffsPerFrame) { bad.push(`${n} puffs in the frame at ${t}`); break; }
  if (game.destr.debris.list.length > game.destr.debris.cap()) bad.push(`debris ${game.destr.debris.list.length} > cap ${game.destr.debris.cap()}`);
  for (const c of game.destr.debris.list) if (![...c.box.c, ...c.v].every(Number.isFinite)) { bad.push('a NaN in a debris chunk'); break; }
  if (D.DESTR_STATS.tiles > D.CHAOS.tilesPerEncounter) bad.push(`tiles ${D.DESTR_STATS.tiles} > ${D.CHAOS.tilesPerEncounter}`);
  if (D.DESTR_STATS.fixtures > D.CHAOS.loud.fixturesPerEncounter) bad.push(`fixtures ${D.DESTR_STATS.fixtures} > ${D.CHAOS.loud.fixturesPerEncounter}`);
  return bad;
};


// ================================================================ firefight
/** THE MERGE'S SEAM with the wall engine (src/game/wallbreak.ts): the dressing is small things ON tables and shelves — none of it may be taken by the engine's
 *  partition / cover rule (it would then be chewed as cover and a man could be dealt it), and all of it stays the chaos lane's to knock about. */
export async function loudDressing() {
  const F = family('loudchaos-dressing: none of the dressing is the wall engine\'s cover or partition');
  for (const L of LOUD) {
    const data = dataOf(L.file); const dress: number[] = []; data.boxes.forEach((b: any, i: number) => { if (b.group === DRESS_GROUP) dress.push(i); });
    const S = await stage(L.file); const walls = S.game.destr.walls; const list: any[] = walls.list; const taken = dress.filter(i => walls.owner.has(i) || list.some(B => B.id === i));
    const inv = walls.inventory(); const items = new Set(S.items.map(d => d.i)); const found = dress.filter(i => items.has(i)).length;
    F.verdict(dress.length > 0 && taken.length === 0 && inv.walls + inv.cover > 0, `${L.file}: ${dress.length} dressing boxes, none a breakable of the wall engine`,
      `engine: ${inv.walls} partitions, ${inv.cover} cover · dressing taken as cover / wall: ${taken.length}${taken.length ? ' (' + taken.slice(0, 6).join(', ') + ')' : ''} · the chaos lane indexes ${found} of them`);
  }
  return F.done();
}
export async function loudFirefight() {
  const F = family('loudchaos-firefight: a burst over a dressed table on a sample of the loud levels — paper up, things knocked off, dust into the smoke, every count inside its cap');
  for (const L of sample(LOUD, 5)) {
    const S = await stage(L.file); const { W, game, smokeLog, items } = S; const it = busiest(items);
    if (!it) { F.verdict(false, `${L.file}: a dressed spot`, 'no paper stack in the index'); continue; }
    const at = standNear(game, it.box.c);
    if (!at) { F.verdict(false, `${L.file}: somewhere to stand`, `nowhere within 4.5 m of ${p2(it.box.c)} has a line to it`); continue; }
    face(S, at, it.box.c); const pc = game.player.char; const hidden0 = game.destr.inventory().hidden;
    // 0.6 s of automatic fire into the stack, a second over it into whatever stands behind …
    for (let r = 0; r < 6; r++) { fireWeapon(game, pc, [it.box.c[0] + ((r % 3) - 1) * 0.05, it.box.c[1], it.box.c[2] + ((r % 2) - 0.5) * 0.06], true, { weapon: WEAPONS.fn2000 }); for (let f = 0; f < 6; f++) W.step(DT); }
    const aim = v3.normalize(v3.sub([it.box.c[0], it.box.c[1] + 0.3, it.box.c[2]], [at[0], 1.4, at[2]])); const far: Vec3 = v3.mad(it.box.c, aim, 6);
    for (let r = 0; r < 10; r++) { fireWeapon(game, pc, [far[0] + ((r % 5) - 2) * 0.12, far[1] + ((r % 3) - 1) * 0.1, far[2] + ((r % 4) - 1.5) * 0.12], true, { weapon: WEAPONS.fn2000 }); for (let f = 0; f < 6; f++) W.step(DT); }
    // … then into the small things near it, each from where it can be seen (a thing is knocked off once its square is hot: the fifth round into it at chaos 0.5)
    const smalls = items.filter(o => o.kind === 'clutter' || o.kind === 'binder').sort((a, b) => v3.dist(a.box.c, it.box.c) - v3.dist(b.box.c, it.box.c)).slice(0, 4);
    for (const o of smalls) { const q = standNear(game, o.box.c, 2, 4.5, o.box.h[1] + 0.05); if (!q) continue; face(S, q, o.box.c); for (let r = 0; r < 7; r++) { fireWeapon(game, pc, v3.copy(o.box.c), true, { weapon: WEAPONS.fn2000 }); for (let f = 0; f < 6; f++) { W.cursorAt(o.box.c); W.step(DT); } } }
    for (let f = 0; f < 90; f++) W.step(DT);
    const s = { ...D.DESTR_STATS }; const fails = capsHeld(smokeLog, game);
    if (s.papers < 1) fails.push('no paper went up'); if (smalls.length && s.knocks < 1) fails.push(`nothing knocked off (${smalls.length} small things shot at)`);
    if (s.rounds < 20) fails.push(`only ${s.rounds} rounds reached the module`); if (s.puffs < 1) fails.push('no dust');
    const hid = game.destr.inventory().hidden; await quiet(() => game.restartEncounter()); const back = game.destr.inventory().hidden === 0 && game.destr.debris.list.length === 0;
    if (!back) fails.push('restartEncounter left something hidden / in the pool');
    F.verdict(!fails.length, `${L.file}: from ${p2(at)} over the stack at ${p2(it.box.c)} — ${s.rounds} rounds: ${s.papers} stacks up (${s.sheets} sheets), ${s.knocks} knocked off (${s.knocksDemoted} demoted), ${s.puffs} dust puffs (−${s.puffsRefused} refused by the caps), ${s.bursts} bursts, ${s.tiles} tiles`,
      fails.join('; ') || `${hid - hidden0} statics hidden then restored by the restart; debris peak ${game.destr.debris.stats.peakLive} of ${game.destr.debris.cap()}; puffs ≤ ${D.CHAOS.puffsPerSec}/s and ≤ ${D.CHAOS.puffsPerFrame}/frame held`);
  }
  return F.done();
}

// ================================================================ blast
export async function loudBlast() {
  const F = family('loudchaos-blast: a charge beside a dressed surface on each loud level — paper up, the ceiling answers, dust off ceiling and floor, fixtures in reach dark');
  for (const L of LOUD) {
    const S = await stage(L.file); const { W, game, smokeLog, items } = S; const it = busiest(items);
    const at0 = it && (chargeSpot(game, items) ?? standNear(game, it.box.c, 0.9, 1.6)); if (!it || !at0) { F.verdict(false, `${L.file}: a spot for the charge`, 'none found beside a dressed surface'); continue; }
    const at: Vec3 = [at0[0], 0.06, at0[2]];
    const lit0 = game.targets.filter((t: any) => !t.broken).length;
    game.destr.bang(at, NOBODY_ID); for (let f = 0; f < 120; f++) W.step(DT);
    const s = { ...D.DESTR_STATS }; const fails = capsHeld(smokeLog, game);
    const puffs = smokeLog.filter(e => e.kind === 'puff'); const high = puffs.filter(e => e.pos[1] > 2).length, low = puffs.filter(e => e.pos[1] < 0.5).length;
    if (s.papers < 1) fails.push('no paper went up'); if (high < 1) fails.push('no dust off the ceiling');
    if (s.tiles + s.fixtures < 1) fails.push('neither a tile nor a panel came down over it');
    if (low < D.CHAOS.loud.blastDustPuffs) fails.push(`${low} puff(s) off the floor, a charge raises ${D.CHAOS.loud.blastDustPuffs}`);
    const dark = lit0 - game.targets.filter((t: any) => !t.broken).length;
    // out of doors: the same charge under the open sky drops nothing and shakes no ceiling
    let outdoors = 'no open ground found';
    { let op: Vec3 | null = null; for (let x = 1.5; x < 39 && !op; x += 1.5) for (let z = 1.5; z < 27 && !op; z += 1.5) if (!game.col.raycast([x, 2.3, z], [0, 1, 0], 6) && !game.col.solidBlocked([x, 0.1, z], [x, 2.2, z])) op = [x, 0.06, z];
      if (op) { const t0 = D.DESTR_STATS.tiles, n0 = smokeLog.length; game.destr.bang(op, NOBODY_ID); W.step(DT); const up = smokeLog.slice(n0).filter(e => e.kind === 'puff' && e.pos[1] > 2).length; if (D.DESTR_STATS.tiles !== t0 || up) fails.push(`a charge in the open at ${p2(op)} dropped a tile / shook a ceiling`); outdoors = `in the open at ${p2(op)}: no tile, no ceiling dust`; } }
    F.verdict(!fails.length, `${L.file}: a charge at ${p2(at)} — ${s.papers} stacks up, ${s.knocks} knocked off, ${s.tiles} tile down, ${dark} fixture(s) dark (${s.fixtures} by the blast / the ceiling), dust: ${high} off the ceiling, ${low} off the floor`, fails.join('; ') || `${outdoors}; debris ${game.destr.debris.list.length} of ${game.destr.debris.cap()}`);
  }
  // dustBurst (the seam for whatever else breaks things): far more asked than the caps allow → let out at ≤ 2 a frame, ≤ 8 a second, the queue bounded, the rest refused; nothing at chaos 0
  { const S = await stage('warehouse.json'); const { W, game, smokeLog } = S; const at: Vec3 = [20, 0.06, 14]; const fails: string[] = [];
    game.destr.dustBurst(at, [0, 1, 0], 60, 1, 'concrete', 1.5); for (let f = 0; f < 60 * 5; f++) W.step(DT);
    const n = smokeLog.filter(e => e.kind === 'puff').length; fails.push(...capsHeld(smokeLog, game));
    if (n < 16 || n > D.CHAOS.loud.burstQueue) fails.push(`${n} puffs out of 60 asked (queue ${D.CHAOS.loud.burstQueue}, ${D.CHAOS.loud.burstSecs} s at ${D.CHAOS.puffsPerSec}/s)`);
    const last = Math.max(...smokeLog.filter(e => e.kind === 'puff').map(e => e.t)), first = Math.min(...smokeLog.filter(e => e.kind === 'puff').map(e => e.t));
    if (last - first > D.CHAOS.loud.burstSecs + 0.1) fails.push(`still puffing ${f2(last - first)} s on`);
    const far = smokeLog.filter(e => e.kind === 'puff').some(e => Math.hypot(e.pos[0] - at[0], e.pos[2] - at[2]) > 1.5 + 1e-6); if (far) fails.push('a puff outside the spread');
    game.tune.chaos = 0; const n0 = smokeLog.length; game.destr.dustBurst(at, [0, 1, 0], 10, 1, 'concrete', 1); for (let f = 0; f < 30; f++) W.step(DT); if (smokeLog.length !== n0) fails.push('a burst at chaos 0 raised dust');
    F.verdict(!fails.length, `dustBurst of 60 puffs: ${n} let out over ${f2(last - first)} s inside the per-frame and per-second caps, ${D.DESTR_STATS.puffsRefused} refused, all within the 1.5 m spread; nothing at chaos 0`, fails.join('; ') || `queue ≤ ${D.CHAOS.loud.burstQueue}, patience ${D.CHAOS.loud.burstSecs} s`); }
  // `power` reaches further
  { const S = await stage('warehouse.json'); const { game, items } = S; const papers = items.filter(o => o.kind === 'paper');
    // (THE stack is asked, by its own box: the floor's count of stacks taken was the same thing while this one stood alone, and a lived-in room — lived.ts,
    //  on in the page — puts others nearer the charge. LOUDSTACK_BREAK=1, the red path: the power-1 charge is set at the stack itself — the line must fail.
    //  The power-4 arm has no red path of its own here: 'takes it' is read off the same log line)
    const NEAR = process.env.LOUDSTACK_BREAK === '1';
    // (the stack is found again on each floor by where its box stands, not by its number; 'taken' is the module's own line of it in its log; and each arm
    //  must show its charge went off at all — an arm that 'left the stack' because nothing was set is no answer)
    const c0 = papers[0].box.c, same = (o: Stage['items'][number]) => o.kind === 'paper' && Math.hypot(o.box.c[0] - c0[0], o.box.c[1] - c0[1], o.box.c[2] - c0[2]) < 1e-6;
    const reachOf = async (power: number) => { const T = await stage('warehouse.json'); const it = T.items.find(same); if (!it) return null;
      const at = NEAR && power === 1 ? standNear(T.game, it.box.c, 0.4, 0.9, 0.02) : standNear(T.game, it.box.c, 3.2, 3.6); if (!at) return null;
      const bangs0: number = D.DESTR_STATS.bangs, log0: number = D.DESTR_LOG.length; T.game.destr.bang([at[0], 0.06, at[2]], NOBODY_ID, power);
      return { took: D.DESTR_LOG.slice(log0).some((l: string) => l.includes(`paper ${it.i} scattered`)), all: D.DESTR_STATS.papers as number, went: D.DESTR_STATS.bangs - bangs0 === 1, far: Math.hypot(at[0] - it.box.c[0], at[2] - it.box.c[2]) }; };
    void game; const a = await reachOf(1), b = await reachOf(4);
     }
  return F.done();
}

// ================================================================ fixtures
export async function loudFixtures() {
  const F = family('loudchaos-fixtures: anybody\'s round that is IN a fixture puts it out, a near miss does not, the caps hold, breakers / devices / scenery never; a stun canister is not a blast; switch off: none of it');
  const liveOf = (game: any) => game.targets.filter((t: any) => !t.broken && t.interactive && (t.kind === 'emissive' || t.kind === 'analytic') && (t.on || t.factor > 0));
  { const S = await stage('barracks.json'); const { W, game } = S; const ts = liveOf(game); const fails: string[] = [];
    const t = ts[0]; const below: Vec3 = [t.pos[0] + 0.6, 1.2, t.pos[2] + 0.4]; const dir = v3.normalize(v3.sub(t.pos, below)), L = v3.dist(t.pos, below);
    const ev0 = game.events.length;
    game.destr.fixtureRound(below, v3.normalize(v3.sub([t.pos[0] + 0.45, t.pos[1], t.pos[2]], below)), L, undefined); const miss = t.broken;
    game.destr.fixtureRound(below, dir, L, undefined); const hit = t.broken; W.step(DT);
    if (miss) fails.push('a round passing 0.4 m from the fixture put it out'); if (!hit) fails.push('a round through the fixture left it lit');
    if (!game.events.slice(ev0).some((e: any) => e.kind === 'lightOut')) fails.push('no lightOut event filed (nobody could notice)');
    if (t.factor > 0.01 && (W.step(DT), t.factor > 0.01)) fails.push(`its light is still up (factor ${t.factor})`);
    // the gap, then the cap: every other fixture shot dead-on, 1 s apart
    const t2 = liveOf(game)[0]; game.destr.fixtureRound([t2.pos[0], 1.2, t2.pos[2]], [0, 1, 0], t2.pos[1] - 1.2, undefined); if (t2.broken) fails.push(`a second fixture inside the ${D.CHAOS.loud.fixtureGapSecs} s gap went too`);
    for (const x of liveOf(game)) { for (let f = 0; f < 60; f++) W.step(DT); game.destr.fixtureRound([x.pos[0], 1.2, x.pos[2]], [0, 1, 0], Math.max(0.1, x.pos[1] - 1.2), undefined); }
    const dead = D.DESTR_STATS.fixtures; if (dead > D.CHAOS.loud.fixturesPerEncounter) fails.push(`${dead} fixtures out, the cap is ${D.CHAOS.loud.fixturesPerEncounter}`);
    if (ts.length > D.CHAOS.loud.fixturesPerEncounter && dead !== D.CHAOS.loud.fixturesPerEncounter) fails.push(`${dead} out of ${ts.length} shot dead-on, expected the cap exactly`);
    const never = game.targets.filter((x: any) => x.broken && (x.kind === 'breaker' || x.kind === 'device' || !x.interactive)); if (never.length) fails.push(`${never.map((x: any) => x.name)} broken`);
    await quiet(() => game.restartEncounter()); if (game.targets.some((x: any) => x.broken)) fails.push('a fixture still broken after the restart');
    F.verdict(!fails.length, `barracks: ${t.name} survives a round 0.45 m wide and dies to one through it (lightOut filed); then ${ts.length} live fixtures shot dead-on a second apart → ${dead} out, ${D.DESTR_STATS.fixturesRefused} refused by the cap / gap; a restart relights them all`, fails.join('; ') || `cap ${D.CHAOS.loud.fixturesPerEncounter} an encounter, gap ${D.CHAOS.loud.fixtureGapSecs} s`); }
  { // through fireWeapon: a staged teammate's round into a panel's own box (not the player's)
    const run = async () => { const S = await stage('barracks.json'); const { W, game } = S;
      const op = game.nightBreach.ops[0]; if (!op) return { ok: false, why: 'no teammate staged' };
      let best: any = null, bd = 1e9; for (const t of liveOf(game)) { const d = v3.dist(t.pos, op.char.pos); if (d < bd && t.boxes.length && !game.col.solidBlocked(v3.add(op.char.pos, [0, 1.5, 0]), [t.pos[0], t.pos[1] - 0.15, t.pos[2]])) { bd = d; best = t; } }
      if (!best) return { ok: false, why: 'no fixture in the teammate\'s view' };
      for (let r = 0; r < 3; r++) { fireWeapon(game, op.char, v3.copy(best.pos), false, { weapon: WEAPONS[op.weapon?.def?.id ?? 'aks74u'] ?? WEAPONS.aks74u }); W.step(DT); }
      return { ok: true, broken: best.broken, name: best.name, d: bd }; };
    const a: any = await run();
    F.verdict(a.ok && a.broken === true, 'a staged teammate\'s rounds at a panel he can see put it out', a.ok ? `${a.name} at ${f2(a.d)} m — broken ${a.broken}` : `${a.why}`); }
  { // A STUN CANISTER IS NOT A BLAST (CHAOS.loud.stunFixtureM / stunPanels): on the floor under a lit fixture it leaves the light on, however many go off there; right
    // beside the lamp it puts it out; a door or wall charge borrowing the canister's report (detonateStun's `borrowed`) puts it out from the floor, as every bang did
    const run = async (how: 'stun' | 'beside' | 'charge', cans = 1) => { const S = await stage('barracks.json'); const { W, game } = S;
      {
        let best: any = null, bd = 1e9; const H = 0.1 + fx.bang.height;   // (where a can lying on the floor goes off)
        for (const t of liveOf(game)) { const eye: Vec3 = [t.pos[0], 0.95, t.pos[2]], d = t.pos[1] - H; if (d < 1.2 || d > 2.9 || d >= bd || !t.boxes.length) continue;
          const hit = game.col.raycast(eye, [0, 1, 0], t.pos[1] - 0.95); if (hit && hit.t < t.pos[1] - 0.95 - 0.06 && !(hit.index !== undefined && t.boxes.includes(hit.index))) continue;
          if (game.col.solidBlocked([t.pos[0], 0.1, t.pos[2]], eye)) continue; bd = d; best = t; }
        if (!best) return { ok: false, why: 'no lit fixture with clear floor under it' };
        const dark0 = game.targets.filter((x: any) => x.broken).length, log0 = D.DESTR_LOG.length, tiles0 = D.DESTR_STATS.tiles;
        const at: Vec3 = how === 'beside' ? [best.pos[0], best.pos[1] - 0.45 - fx.bang.height, best.pos[2]] : [best.pos[0], 0.1, best.pos[2]];
        let n = 0; for (; n < cans && !best.broken; n++) { const it = game.items.spawn('flash', at, [0, 0, 0], { fuse: 9, heading: 0, who: NOBODY_ID }); it.fired = true; detonateStun(game, it, how === 'charge'); for (let f = 0; f < 6; f++) W.step(DT); }
        return { ok: true, name: best.name, d: bd, n, out: !!best.broken, dark: game.targets.filter((x: any) => x.broken).length - dark0, tiles: D.DESTR_STATS.tiles - tiles0, shaken: D.DESTR_LOG.slice(log0).some((l: string) => l.includes(`fixture ${best.name} out (shaken)`)) };
      } };
    const a: any = await run('stun', 6), b: any = await run('beside'), c: any = await run('charge');
    const all = a.ok && b.ok && c.ok;
    const ok = all && !a.out && a.dark === 0 && a.tiles >= 1 && b.out && c.out;
    F.verdict(ok, 'a stun canister on the floor under a lit fixture leaves every light on, six of them running (tiles beside the panel let go); one that goes off right beside the lamp puts it out; a door or wall charge there puts it out as a blast does',
      all ? `${a.name}, ${f2(a.d)} m over the can — stun ×${a.n}: out ${a.out}, ${a.dark} dark, ${a.tiles} tiles · beside (0.45 m): out ${b.out} · charge: out ${c.out}, ${c.dark} dark` : `${a.why ?? b.why ?? c.why}`); }
  return F.done();
}

// ================================================================ sight

