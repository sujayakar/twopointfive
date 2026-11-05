// WHAT A HIT LOOKS LIKE GOES BY WHAT THE THING SAYS IT IS MADE OF (src/game/particleFx.ts, HITSTUFF) — a rule of the picture only.
// On the office, by hand, with real rounds: a car's body as the level has it is read by the colour of its paint, rule up or down; the same body saying
// 'carpaint' gives sparks, no chips and no dust; a tyre saying 'rubber' is not the metal its dark paint is taken for; the car's glass saying 'glass' — a
// solid thing, not a pane — takes the crack in place of the hole, every round and with the chaos lane off, and stands; a thing that is metal by its paint
// throws no chips with the rule up; AND THE RUN IS THE SAME RUN with the rule up as down — by digest, by the chaos lane's die and counters and by the
// count of marks — rounds into boxes that say a word included (a window pane among them: it is the pane engine's whatever it says); the words, the lint, the stands, the registry and a recording from before the table.
//   bun run tools/qa/hitstuff-check.ts          (probes.ts hitstuff; HITSTUFF_BREAK=1 must FAIL — the rule is left down where the check raises it)
// Both stands: the check gives the stand its word (TPS_HITSTUFF) itself. WHAT A ROUND KNOCKS OUT IS READ OFF A STAND-IN for the GPU particles (the harness
// has none): the calls the game makes at the hit, their kind and count; a mark is the call that lays it. Nothing here is a picture, and no spark, chip or
// crack has been seen on a GPU by this check.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { HITSTUFF, HIT_STUFFS, saidSurf, solidGlass, surfOf } = await import(`${ROOT}/src/game/particleFx.ts`);
const { BOX_STUFFS } = await import(`${ROOT}/src/scene/stuff.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60, BREAK = process.env.HITSTUFF_BREAK === '1', VERBOSE = !!process.env.HITSTUFF_VERBOSE, FILE = 'office.json';
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
async function hushed<T>(f: () => T | Promise<T>): Promise<T> { const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.warn = warn; } }
const levelOf = () => loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${FILE}`, 'utf8')));
let stood: boolean | null = null;   // (the switch as a stand with no word of the check's left it, read once)
async function stand(on: boolean): Promise<any> {
  Math.random = seeded(33);
  const word = process.env.TPS_HITSTUFF;
  try {
    if (stood === null) { await hushed(() => standUp({ level: levelOf(), seed: 1 })); stood = !!HITSTUFF.on; }
    process.env.TPS_HITSTUFF = on && !BREAK ? '1' : '0';
    return await hushed(() => standUp({ level: levelOf(), seed: 1 }));
  } finally { if (word === undefined) delete process.env.TPS_HITSTUFF; else process.env.TPS_HITSTUFF = word; }
}
const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
const sized = (b: any, h: number[]) => near(b.h[0], h[0]) && near(b.h[1], h[1]) && near(b.h[2], h[2]);

/** what one stood-up game lets the check do: put the man where a round of his meets box `i`, fire `n` of them, and say what each knocked out */
function range(W: any) {
  const { game, input } = W, pl = game.player, c = pl.char, st: any[] = game.engine.world.statics;
  const eye = (p: number[]) => [p[0], 1.35, p[2]];
  const draw = () => { for (let i = 0; i < 3 && !pl.pistolUp; i++) { input.pressed.add(i === 0 ? 'Digit1' : 'KeyE'); for (let k = 0; k < 50; k++) W.step(DT); } };
  /** where a round is aimed in box i: its middle, `low` of its half height under that (a car's body under its glass: the gun is held higher than the body's top) */
  const aimAt = (i: number, low = 0): number[] => { const b = st[i]; return [b.c[0], b.c[1] - low * b.h[1], b.c[2]]; };
  /** floor from which a line to that point meets box i first — from his eye and from where a pistol is held — and where he really stands once put there */
  const spot = (i: number, low = 0): number[] | null => { const b = st[i], to = aimAt(i, low);
    for (const d of [4, 3, 5, 6, 2.5]) for (let k = 0; k < 48; k++) { const a = k / 48 * Math.PI * 2, p = [b.c[0] + Math.sin(a) * d, 0, b.c[2] + Math.cos(a) * d];
      if (game.col.nav.isBlocked(p[0], p[2])) continue;
      if ([1.35, 1.2, 1.05].some(y => { const o = [p[0], y, p[2]], h = game.col.raycast(o, v3.normalize(v3.sub(to, o)), 60); return !h || h.index !== i; })) continue;
      game.teleportPlayer(p); for (let s = 0; s < 4; s++) W.step(DT); pl.aim.clear();
      if (v3.distXZ(c.pos, p) < 0.1) return p; }
    return null; };
  /** `n` rounds at box i's middle from where he stands. Returns what the game asked of the particles (kind → how many, summed), the damage marks it laid
   *  on that box ('bullet' = a round's hole; 'damage 9' = the crack, 'damage 0' the chaos lane's dent), and how many of the rounds met box i. `chaos`: the
   *  chaos lane's dial for the volley (0 = the lane does nothing with the rounds it is told of); `cold` false = on the floor's heat as it stands */
  const fire = (i: number, n = 2, low = 0, chaos = 1, cold = true) => { const b = st[i], to = aimAt(i, low), e = game.engine as any, was = { p: e.particles, s: e.settings.gpuParticles, add: game.decals.add, round: game.destr.round, en: game.decals.enabled, chaos: game.tune.chaos };
    const emits: { kind: string; at: number[]; n: number }[] = [], hits: number[][] = [], marks: string[] = []; let met = 0;
    // (a stand-in for the GPU particles, there only while the round is fired — what a hit knocks out is asked for inside fireWeapon: it answers every call
    //  and keeps the ones that emit)
    const fake: any = new Proxy(function () { return 0; }, { get: (_t, k) => k === 'emit' ? (kind: string, at: number[], _d: unknown, cnt: number) => { emits.push({ kind, at: [at[0], at[1], at[2]], n: cnt }); } : fake, set: () => true });
    const shot = () => { e.particles = fake; e.settings.gpuParticles = true; try { fireWeapon(game, c, to, true, { spread: 0 }); } finally { e.particles = was.p; e.settings.gpuParticles = was.s; } };
    game.decals.enabled = true; game.decals.add = function (this: any, kind: string, ...a: any[]) { if (a[3]?.clampTo === b) marks.push(kind === 'damage' ? `damage ${Math.floor(a[3]?.param ?? -1)}` : kind); return was.add.call(this, kind, ...a); };
    game.destr.round = function (this: any, sh: any, p: any, d: any, hit: any, def: any) { hits.push([p[0], p[1], p[2]]); if (hit.index === i) met++; return was.round.call(this, sh, p, d, hit, def); };
    game.tune.chaos = chaos; if (cold) game.destr.heat.fill(0);   // (each volley on a cold floor: what a hot zone adds to a hit grows with every round that lands in it, and two volleys are compared)
    try { for (let k = 0; k < n; k++) { for (let s = 0; s < 3; s++) { W.cursorAt(to); W.step(DT); } shot(); for (let s = 0; s < 12; s++) W.step(DT); } }
    finally { e.particles = was.p; e.settings.gpuParticles = was.s; game.decals.add = was.add; game.destr.round = was.round; game.decals.enabled = was.en; game.tune.chaos = was.chaos; }
    // (what was knocked out AT THE HIT: the muzzle's own sparks and the case are the gun's)
    const of = (kind: string) => emits.filter(m => m.kind === kind && hits.some(h => v3.dist(h, m.at) < 0.3)).reduce((a, m) => a + m.n, 0);
    return { spark: of('spark'), chips: of('debris'), dust: of('dust'), marks, met };
  };
  return { game, st, draw, spot, fire, aimAt };
}
type Shot = { spark: number; chips: number; dust: number; marks: string[]; met: number };
const say = (r: Shot) => `${r.met} met it · ${r.spark} sparks, ${r.chips} chips, ${r.dust} dust · marks: ${r.marks.join(', ') || 'none'}`;
const count = (r: Shot, m: string) => r.marks.filter(x => x === m).length;

export async function hitstuffCheck(): Promise<boolean> {
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log(`\n=== hitstuff: a round reads what a thing says it is made of${BREAK ? ' (HITSTUFF_BREAK: the rule left down)' : ''} ===`);
  const was = { ...HITSTUFF }, rnd = Math.random;
  const up = (v: boolean) => { HITSTUFF.on = v && !BREAK; };
  try {
    const W: any = await stand(true);
    { const shipped = (process.env.TPS_PROFILE ?? '') === 'shipped', said = process.env.TPS_HITSTUFF;
      line(stood === (said !== undefined ? said === '1' : shipped), '0 the stand: down on the frozen stand, up on the shipped one, either as its word says where the run gives one', `stood up ${stood} · ${shipped ? 'shipped' : 'frozen'}${said !== undefined ? ` with TPS_HITSTUFF=${said}` : ''}`); }
    const R = range(W), st = R.st; R.draw();
    const body = st.findIndex((b: any) => sized(b, [2.2, 0.34, 0.86])), glass = st.findIndex((b: any, i: number) => sized(b, [1.25, 0.27, 0.8]) && body >= 0 && Math.hypot(b.c[0] - st[body].c[0] + 0, b.c[2] - st[body].c[2]) < 0.6 && i !== body);
    if (body < 0 || glass < 0) throw new Error(`hitstuff-check: ${FILE} has no car where the kit writes one (body ${body}, glass ${glass})`);
    const B = st[body], G = st[glass];

    // (1) the body saying nothing (the word its level gives it — the office's cars say 'carpaint' since 11-05 — taken off): read by its paint, rule up or down
    const LOW = 0.5, pB = R.spot(body, LOW), levelSays: string | undefined = B.stuff; B.stuff = undefined;
    up(true); const plainUp = pB ? R.fire(body, 2, LOW) : null; up(false); const plainDown = pB ? R.fire(body, 2, LOW) : null;
    const paint = surfOf(B.albedo);
    line(!!plainUp && !!plainDown && plainUp.met === 2 && plainDown.met === 2 && B.stuff === undefined && saidSurf(B) === undefined && plainUp.spark === plainDown.spark && plainUp.chips === plainDown.chips && plainUp.dust === plainDown.dust && plainUp.chips > 0 && paint !== 'metal',
      '1 a car\'s body that says nothing of what it is made of (the word its level gives it taken off): two rounds into it are read by the colour of its paint, and knock out the same with the rule up as down — chips, as a wall would give',
      `as the level has it the body says ${levelSays ?? 'nothing'} · by its paint: ${paint} · up: ${plainUp ? say(plainUp) : 'no spot'} · down: ${plainDown ? say(plainDown) : 'no spot'}`);

    // (2) the same body, saying it is car paint; and a tyre, saying it is rubber
    B.stuff = 'carpaint';
    up(true); const saidUp = pB ? R.fire(body, 2, LOW) : null; up(false); const saidDown = pB ? R.fire(body, 2, LOW) : null;
    const tyre = st.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12]) && Math.hypot(b.c[0] - B.c[0], b.c[2] - B.c[2]) < 2), T = st[tyre], pT = tyre >= 0 ? R.spot(tyre) : null; if (T) T.stuff = 'rubber';
    up(true); const tyreUp = pT ? R.fire(tyre, 1) : null; up(false); const tyreDown = pT ? R.fire(tyre, 1) : null;
    line(!!saidUp && !!saidDown && saidUp.met === 2 && saidUp.spark > 0 && saidUp.chips === 0 && saidUp.dust === 0 && count(saidUp, 'bullet') === 2
      && saidDown.chips === plainDown!.chips && saidDown.dust === plainDown!.dust && saidDown.spark === plainDown!.spark
      && !!tyreUp && !!tyreDown && tyreUp.met === 1 && tyreUp.chips > 0 && tyreUp.spark < tyreDown.spark && tyreDown.chips <= 3 && count(tyreUp, 'damage 0') === 0 && tyreUp.marks.length === tyreDown.marks.length && count(tyreUp, 'bullet') === 1 && count(tyreUp, 'chip') === tyreUp.marks.length - 1,
      '2 the same body saying \'carpaint\': with the rule up two rounds give sparks, no chips and no dust, and each leaves its hole; with the rule down the word is not heard and they give what the paint gave. And a tyre saying \'rubber\': by its dark paint it was metal, a shower of sparks; with the rule up it gives the chips and the few sparks plastic gives, and where the chaos lane drew the dent of sheet metal it draws a chip — as many marks as with the rule down',
      `body up: ${saidUp ? say(saidUp) : 'no spot'} · down: ${saidDown ? say(saidDown) : 'no spot'}\n    tyre (by its paint ${T ? surfOf(T.albedo) : '-'}) up: ${tyreUp ? say(tyreUp) : 'no spot'} · down: ${tyreDown ? say(tyreDown) : 'no spot'}`);

    // (3) the car's glass, saying it is glass: a solid thing, not a pane — it cracks, every round, and stands
    G.stuff = 'glass'; const flags0 = G.flags, pG = R.spot(glass);
    up(true); const solid = solidGlass(G), glassUp = pG ? R.fire(glass) : null, glassCold = pG ? R.fire(glass, 2, 0, 0) : null;
    const stands = G.flags === flags0 && !!pG && R.game.col.raycast([pG[0], 1.35, pG[2]], v3.normalize(v3.sub(G.c, [pG[0], 1.35, pG[2]])), 60)?.index === glass;
    up(false); const glassDown = pG ? R.fire(glass) : null;
    line(!!glassUp && !!glassCold && !!glassDown && (solid || BREAK) && glassUp.met === 2 && count(glassUp, 'damage 9') === glassUp.marks.length && glassUp.marks.length === glassDown.marks.length && glassUp.marks.length >= 2 && glassUp.dust === 0 && glassUp.chips > 0 && glassUp.chips <= 2 * 3
      && count(glassCold, 'damage 9') === 2 && count(glassCold, 'bullet') === 0 && stands && (G.flags & BoxFlag.Glass) === 0
      && count(glassDown, 'damage 9') === 0 && count(glassDown, 'bullet') === 2,
      '3 the car\'s glass saying \'glass\' is a solid thing of glass and not a pane: with the rule up each of two rounds leaves the crack on that box in place of the hole, and where the chaos lane drew the dent of the metal its dark paint was taken for it draws a crack too — as many marks as with the rule down — a pinch of glass (three chips a round, no more) and no dust — with the chaos lane off too, round for round — and the box stands as it stood (the next round meets it); with the rule down each leaves its hole and no crack',
      `solid glass ${solid} · up: ${glassUp ? say(glassUp) : 'no spot'} · the chaos lane off: ${glassCold ? say(glassCold) : 'no spot'} · stands ${stands} · down: ${glassDown ? say(glassDown) : 'no spot'}`);

    // (4) a thing that is metal by its paint, and says it is or says nothing (every box of a level file says what its row is made of since 11-04: a cabinet
    //     says steel): no chips with the rule up, the two flecks a round threw with it down
    const METAL_WORDS: readonly (string | undefined)[] = [undefined, 'steel', 'metal', 'sheet'];
    const metalOf = (Q: ReturnType<typeof range>) => Q.st.map((b: any, i: number) => i).filter((i: number) => { const b = Q.st[i]; return METAL_WORDS.includes(b.stuff) && !(b.flags & (BoxFlag.Glass | BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Dynamic)) && !b.rot && surfOf(b.albedo) === 'metal' && b.h[1] > 0.5 && b.c[1] - b.h[1] < 0.6 && b.c[1] + b.h[1] > 1.5 && Math.max(b.h[0], b.h[2]) > 0.25 && !Q.game.destr.itemOf(i); });
    const first = (Q: ReturnType<typeof range>, cand: number[]): number => { for (const i of cand.slice(0, 40)) if (Q.spot(i)) return i; return -1; };
    { const cand = metalOf(R), m = first(R, cand);
      up(true); const metalUp = m >= 0 ? R.fire(m, 1) : null; up(false); const metalDown = m >= 0 ? R.fire(m, 1) : null;
      line(!!metalUp && !!metalDown && metalUp.met === 1 && metalDown.met === 1 && metalUp.chips === 0 && metalUp.spark > 0 && metalUp.spark === metalDown.spark && metalDown.chips > 0 && metalDown.chips <= 3,
        '4 a thing that is metal by its paint, and says so or says nothing (a cabinet, a rack): a round knocks the same sparks out of it either way, no chips with the rule up, and with it down the two flecks it threw before',
        `box ${m} of ${cand.length} · up: ${metalUp ? say(metalUp) : 'no spot'} · down: ${metalDown ? say(metalDown) : 'no spot'}`); }

    // (5) A RULE OF THE PICTURE: the run is the same run with the rule up as down — by digest, by the chaos lane's own die and counters, and by how many
    //     marks lie on the level — over four rounds a spot at the chaos dial's top (the harness holds the dial at 0, and a hot zone is where the lane's dice
    //     fall), with the particles' stand-in in, into: the car's body, glass and a tyre saying their words; a box of every word the level itself says that a
    //     round can reach; a wall and a thing of metal that say nothing; and a window pane that says 'glass' as well, which stays the pane engine's
    { const run = async (on: boolean) => { const V: any = await stand(on); const Q = range(V); Q.draw(); up(on); V.game.decals.onCut = on;   // (the page's word for a mark on a cut face rides with the rule: both are the picture's)
        let fired = 0, panes = 0; const words = new Set<string>(), DS: any = (await import(`${ROOT}/src/game/destruction.ts`)).DESTR_STATS, ds0 = { ...DS };   // (a tally of the whole process: what THIS run adds to it is compared)
        const bd = Q.st.findIndex((b: any) => sized(b, [2.2, 0.34, 0.86])), gl = Q.st.findIndex((b: any, i: number) => i !== bd && sized(b, [1.25, 0.27, 0.8]) && Math.hypot(b.c[0] - Q.st[bd].c[0], b.c[2] - Q.st[bd].c[2]) < 0.6);
        const ty = Q.st.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12]) && Math.hypot(b.c[0] - Q.st[bd].c[0], b.c[2] - Q.st[bd].c[2]) < 2);
        const vis = (b: any) => !(b.flags & (BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Dynamic)) && !b.rot;
        const pane = Q.st.findIndex((b: any) => (b.flags & BoxFlag.Glass) !== 0 && b.h[1] > 0.4), wall = Q.st.findIndex((b: any) => b.stuff === undefined && surfOf(b.albedo) === 'drywall' && b.h[1] > 1 && vis(b)), metal = first(Q, metalOf(Q));
        Q.st[bd].stuff = 'carpaint'; Q.st[gl].stuff = 'glass'; if (ty >= 0) Q.st[ty].stuff = 'rubber'; if (pane >= 0) Q.st[pane].stuff = 'glass';
        { const soft = first(Q, Q.st.map((b: any, i: number) => i).filter((i: number) => ![bd, gl, ty, pane, wall, metal].includes(i) && Q.st[i].stuff === 'paint' && vis(Q.st[i]) && !(Q.st[i].flags & BoxFlag.Glass) && Q.st[i].h[1] > 0.3)); if (soft >= 0) Q.st[soft].stuff = 'cloth'; }   // (a soft thing: the word most boxes of the levels say, and one the office says of nothing a round reaches — a thing that said only 'paint' says it here)
        const said: number[] = []; for (const w of HIT_STUFFS as string[]) { const c = Q.st.map((b: any, i: number) => i).filter((i: number) => ![bd, gl, ty, pane].includes(i) && Q.st[i].stuff === w && vis(Q.st[i]) && !(Q.st[i].flags & BoxFlag.Glass) && Q.st[i].h[1] > 0.3); const i = first(Q, c); if (i >= 0) { said.push(i); words.add(w); } }
        const car: number[] = []; let paneSaid = true;
        await hushed(() => { for (const [i, low] of [[bd, LOW], [gl, 0], [ty, 0], ...said.map(i => [i, 0]), [wall, 0], [metal, 0], [pane, 0]] as [number, number][]) { if (i < 0 || !Q.spot(i, low)) { if (i === bd || i === gl || i === ty) car.push(0); continue; }
            if (i === pane) { panes++; paneSaid = saidSurf(Q.st[pane]) === undefined && !solidGlass(Q.st[pane]); }
            const r = Q.fire(i, 4, low, 1, false); fired += 4; if (i === bd || i === gl || i === ty) car.push(r.met); }
          for (let s = 0; s < 120; s++) V.step(DT); });
        return { h: hashDigest(digestGame(V.game, true)) as string, fired, panes, paneSaid, car, words: [...words], die: V.game.destr.serial as number, marks: V.game.decals.list.length as number, stats: Object.keys(DS).some(k => DS[k] !== ds0[k]) ? JSON.stringify(Object.fromEntries(Object.keys(DS).filter(k => typeof DS[k] === 'number').map(k => [k, DS[k] - ds0[k]]))) : 'unmoved' }; };
      const a = await run(false), b = await run(true);
      line(a.h === b.h && a.die === b.die && a.stats === b.stats && a.stats !== 'unmoved' && a.marks === b.marks && a.fired >= 24 && b.fired === a.fired && a.car.length === 3 && a.car.every(n => n >= 3) && b.car.join() === a.car.join() && a.panes === 1 && b.paneSaid && a.words.length >= 6 && a.words.includes('cloth'),
        '5 a rule of the picture: the run is the same run with the rule up as down — by digest, by the chaos lane\'s own die and its counters, and by how many marks lie on the level — the page\'s word for a mark on a cut face up with it and down with it — over four rounds a spot at the top of the chaos dial, with the particles\' stand-in in: into the car\'s body, glass and a tyre saying their words (each met), a box of every word the level itself says that a round can reach (six or more) and a soft thing saying \'cloth\', a wall and a thing of metal that say nothing, and a window pane that says \'glass\' too and stays the pane engine\'s',
        `down ${a.h} · up ${b.h} · the lane's die ${a.die} / ${b.die} · marks ${a.marks} / ${b.marks} · its counters ${a.stats === b.stats ? 'the same' : 'DIFFER'}${a.stats === 'unmoved' ? ' (UNMOVED: the lane was never in)' : ''} · ${a.fired} rounds · the car's body, glass, tyre met ${a.car.join(', ')} · the level's own words met: ${a.words.join(', ') || 'none'} · a pane among them ${a.panes === 1}, heard as a pane ${b.paneSaid}`); }

    // (6) the words, the lint, the stands' list, the registry and a recording from before the table
    { const J: any = await import(`${ROOT}/src/game/journal.ts`), PR: any = await import(`${ROOT}/src/host/profile.ts`), SE: any = await import(`${ROOT}/src/host/standEnv.ts`), KN: any = await import(`${ROOT}/src/game/knobs.ts`);
      const stale = (HIT_STUFFS as string[]).filter(s => !(BOX_STUFFS as string[]).includes(s));
      const d = JSON.parse(readFileSync(`${ROOT}/levels/${FILE}`, 'utf8')); const gi = d.boxes.findIndex((b: any) => sized(b, [1.25, 0.27, 0.8])), ti = d.boxes.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12])); d.boxes[gi].stuff = 'glass'; d.boxes[ti].stuff = 'rubber';
      const errs = validateLevelData(parseLevelData(JSON.stringify(d))).filter((e: any) => (e.level ?? 'error') === 'error'), again = levelToData(loadLevel(parseLevelData(JSON.stringify(d))), {}), back = `${again.boxes[gi]?.stuff}, ${again.boxes[ti]?.stuff}`;
      const odd = { flags: 0, stuff: 'constructor' } as any; up(true); const guarded = saidSurf(odd) === undefined && saidSurf({ flags: 0, stuff: 'toString' } as any) === undefined; up(false);
      const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(W.game)));
      HITSTUFF.on = true; const old = snap(); delete old.tables.HITSTUFF; J.applyKnobs(W.game, old); const before = HITSTUFF.on;
      HITSTUFF.on = true; const rec = snap(); HITSTUFF.on = false; J.applyKnobs(W.game, rec); const kept = HITSTUFF.on;
      line(stale.length === 0 && ['glass', 'rubber', 'carpaint', 'steel'].every(w => (HIT_STUFFS as string[]).includes(w)) && !(HIT_STUFFS as string[]).includes('paint') && !(HIT_STUFFS as string[]).includes('enamel') && errs.length === 0 && back === 'glass, rubber' && guarded && before === false && kept === true && PR.FROZEN_HOLDS['HITSTUFF.on'] === 'TPS_HITSTUFF=1' && SE.STAND_ENV.includes('TPS_HITSTUFF') && KN.JOURNAL_KNOBS.includes('HITSTUFF'),
        '6 every word the picture of a hit reads is a stuff a box may say — \'glass\' and \'rubber\' among them, and never \'paint\' or \'enamel\', which are a coat on anything — the level lint takes boxes that say the two and writes them back, and a word that is no stuff at all is read as none; a recording from before the table plays with the rule down and one made with it up keeps it; the stands\' list has the row and the word, and the journal the table',
        `${(HIT_STUFFS as string[]).length} words${stale.length ? `, not a box's: ${stale.join(', ')}` : ''} · lint errors ${errs.length}${errs.length ? ` (${errs[0].msg ?? errs[0]})` : ''} · written back ${back} · a word that is none ${guarded} · applyKnobs without the table: ${before} · recorded up: ${kept} · FROZEN_HOLDS ${PR.FROZEN_HOLDS['HITSTUFF.on'] ?? 'no row'} · STAND_ENV ${SE.STAND_ENV.includes('TPS_HITSTUFF')} · journal ${KN.JOURNAL_KNOBS.includes('HITSTUFF')}`); }

    // (7) a mark on a cut face, or round a round thing, lies where the round met it (render/decals.ts onCut, the page's word): on a box with a leaning face, a mark laid at a point
    //     of that face stays there with the word up, and with it down is pulled out to the uncut box's face, centimetres off the surface
    { const { cutPlane, cutRecord } = await import(`${ROOT}/src/scene/cut.ts`), { makeBox } = await import(`${ROOT}/src/scene/boxes.ts`), D = W.game.decals;
      const h: Vec3 = [0.6, 0.27, 0.8], t = 12 * Math.PI / 180, nl: Vec3 = [0, Math.sin(t), Math.cos(t)], dl = Math.cos(t) * h[2] - Math.sin(t) * h[1];   // (the +z face leaned in 12° about its bottom edge)
      const b: any = makeBox({ c: [2, 1.2, 3], h, yaw: 0, albedo: [0.07, 0.08, 0.1] }); b.cut = cutRecord(h, [cutPlane(nl, dl)]).cut;
      const lp: Vec3 = [0.1, 0.15, (dl - nl[1] * 0.15) / nl[2]], p: Vec3 = [b.c[0] + lp[0], b.c[1] + lp[1], b.c[2] + lp[2]], flat: Vec3 = [b.c[0] + 0.1, b.c[1] - 0.26, b.c[2] + h[2]];
      const drum: any = makeBox({ c: [5, 0.45, 3], h: [0.3, 0.45, 0.3], yaw: 0, albedo: [0.2, 0.2, 0.22] }); drum.round = true; const a30 = Math.PI / 6, nr: Vec3 = [Math.cos(a30), 0, Math.sin(a30)], pr: Vec3 = [drum.c[0] + 0.3 * nr[0], drum.c[1] + 0.1, drum.c[2] + 0.3 * nr[2]];   // (a drum's side, 30° round from the box's own axis)
      const was7 = { on: D.onCut, en: D.enabled, n: D.list.length }; D.enabled = true; let up7: any, down7: any, face7: any, rUp: any, rDown: any, rTop: any;
      try { D.onCut = !BREAK; up7 = D.add('bullet', p, nl, 0.03, { clampTo: b }); face7 = D.add('bullet', flat, [0, 0, 1], 0.03, { clampTo: b }); rUp = D.add('bullet', pr, nr, 0.03, { clampTo: drum }); rTop = D.add('bullet', [drum.c[0] + 0.1, drum.c[1] + 0.45, drum.c[2]], [0, 1, 0], 0.03, { clampTo: drum }); D.onCut = false; down7 = D.add('bullet', p, nl, 0.03, { clampTo: b }); rDown = D.add('bullet', pr, nr, 0.03, { clampTo: drum }); }
      finally { D.onCut = was7.on; D.enabled = was7.en; D.list.length = was7.n; }
      const off = (d: any) => d ? Math.abs((d.pos[0] - b.c[0]) * nl[0] + (d.pos[1] - b.c[1]) * nl[1] + (d.pos[2] - b.c[2]) * nl[2] - dl) : 9;
      line(!!up7 && !!down7 && !!face7 && off(up7) < 1e-6 && v3.dist(up7.pos, p) < 1e-6 && off(down7) > 0.02 && Math.abs(face7.pos[2] - (b.c[2] + h[2])) < 1e-9 && cutRecord(h, [cutPlane(nl, dl)]).cut.length === 1
        && !!rUp && !!rDown && !!rTop && v3.dist(rUp.pos, pr) < 1e-6 && v3.dist(rDown.pos, pr) > 0.03 && Math.abs(rTop.pos[1] - 0.9) < 1e-9,
        '7 a mark on a cut face lies where the round met it: laid at a point of a pane\'s leaning face, with the page\'s word up it stays on that face, and with it down it is pulled out to the uncut box\'s face, centimetres off the glass, where the shader drops it; a mark on a face the cut left alone is laid on that face either way; and round a drum\'s side the same — where the round met it with the word up, pulled out to the square\'s face with it down, and on its flat top as ever',
        `off the leaning face: ${(off(up7) * 1000).toFixed(2)} mm with the word up, ${(off(down7) * 1000).toFixed(1)} mm with it down · on the plain face ${face7 ? Math.abs(face7.pos[2] - (b.c[2] + h[2])).toExponential(1) : '-'} · a drum's side: ${rUp ? (v3.dist(rUp.pos, pr) * 1000).toFixed(2) : '-'} mm from the hit with the word up, ${rDown ? (v3.dist(rDown.pos, pr) * 1000).toFixed(1) : '-'} mm with it down`); }
  } catch (e) { console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); all = false; }
  finally { Object.assign(HITSTUFF, was); Math.random = rnd; }
  console.log(`=== hitstuff: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
if (import.meta.main) process.exit((await hitstuffCheck()) ? 0 : 1);
