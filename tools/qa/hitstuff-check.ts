// WHAT A HIT LOOKS LIKE GOES BY WHAT THE THING SAYS IT IS MADE OF (src/game/particleFx.ts, HITSTUFF) — a rule of the picture only.
// A settled rule, as the rule for a mark on a cut face is (render/decals.ts MARKCUT): nothing here asks the game with either down, or writes them. Until
// 2026-11-07 every line had a "rule down" half, line 5 held that the run is the same run up as down, and line 0 and half of line 6 asked the stands.
// On the office, by hand, with real rounds: a car's body that says nothing is read by the colour of its paint; the same body saying 'carpaint' gives
// sparks, no chips and no dust; a tyre saying 'rubber' is not the metal its dark paint is taken for; the car's glass saying 'glass' — a solid thing, not
// a pane — takes the crack in place of the hole, every round and with the chaos lane off, and stands; a thing that is metal by its paint throws no
// chips; the words and the lint; a mark on a cut face; the vehicles as the levels have them.
//   bun run tools/qa/hitstuff-check.ts          (probes.ts hitstuff; HITSTUFF_BREAK=1 must FAIL line 2 — one round is fired at the car's body where the
//                                                line asks two; MARKCUT_BREAK=1 must FAIL line 7 — the box is handed over with its cut taken off)
// WHAT A ROUND KNOCKS OUT IS READ OFF A STAND-IN for the GPU particles (the harness
// has none): the calls the game makes at the hit, their kind and count; a mark is the call that lays it. Nothing here is a picture, and no spark, chip or
// crack has been seen on a GPU by this check.
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { HIT_STUFFS, saidSurf, solidGlass } = await import(`${ROOT}/src/game/particleFx.ts`);
const { doesOf } = await import(`${ROOT}/src/scene/materials.ts`);
const surfOf = (b: { does?: string }): string => doesOf(b).surf;   // (what a hit goes by where the box's `stuff` is not asked: what the box does)
const { BOX_STUFFS, stuffOfName } = await import(`${ROOT}/src/scene/stuff.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60, BREAK = process.env.HITSTUFF_BREAK === '1', CUT_BREAK = BREAK || process.env.MARKCUT_BREAK === '1', VERBOSE = !!process.env.HITSTUFF_VERBOSE, FILE = 'office.json';
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
async function hushed<T>(f: () => T | Promise<T>): Promise<T> { const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.warn = warn; } }
const levelOf = (file = FILE) => loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8')));
async function stand(file = FILE): Promise<any> {
  Math.random = seeded(33);
  return await hushed(() => standUp({ level: levelOf(file), seed: 1 }));
}
const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
const sized = (b: any, h: number[]) => near(b.h[0], h[0]) && near(b.h[1], h[1]) && near(b.h[2], h[2]);

/** what one stood-up game lets the check do: put the man where a round of his meets box `i`, fire `n` of them, and say what each knocked out */
function range(W: any) {
  const { game, input } = W, pl = game.player, c = pl.char, st: any[] = game.engine.world.statics;
  
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
  console.log(`\n=== hitstuff: a round reads what a thing says it is made of${BREAK ? ' (HITSTUFF_BREAK: one round at the car\'s body where two are asked)' : ''} ===`);
  const rnd = Math.random;
  try {
    const W: any = await stand();
    const R = range(W), st = R.st; R.draw();
    const body = st.findIndex((b: any) => sized(b, [2.2, 0.34, 0.86])), glass = st.findIndex((b: any, i: number) => sized(b, [1.25, 0.27, 0.8]) && body >= 0 && Math.hypot(b.c[0] - st[body].c[0] + 0, b.c[2] - st[body].c[2]) < 0.6 && i !== body);
    if (body < 0 || glass < 0) throw new Error(`hitstuff-check: ${FILE} has no car where the kit writes one (body ${body}, glass ${glass})`);
    const B = st[body], G = st[glass];

    // (1) the body saying nothing (the word its level gives it — the office's cars say 'carpaint' since 11-05 — taken off): read by its paint
    const LOW = 0.5, pB = R.spot(body, LOW), levelSays: string | undefined = B.stuff; B.stuff = undefined;
    const plainUp = pB ? R.fire(body, 2, LOW) : null;
    const paint = surfOf(B);
    line(!!plainUp && plainUp.met === 2 && B.stuff === undefined && saidSurf(B) === undefined && plainUp.chips > 0 && paint !== 'metal',
      '1 a car\'s body that says nothing of what it is made of (the word its level gives it taken off): two rounds into it are read by the colour of its paint — chips, as a wall would give',
      `as the level has it the body says ${levelSays ?? 'nothing'} · by its paint: ${paint} · ${plainUp ? say(plainUp) : 'no spot'}`);

    // (2) the same body, saying it is car paint; and a tyre, saying it is rubber
    B.stuff = 'carpaint';
    const saidUp = pB ? R.fire(body, BREAK ? 1 : 2, LOW) : null;   // (the break word: one round where the line asks two)
    const tyre = st.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12]) && Math.hypot(b.c[0] - B.c[0], b.c[2] - B.c[2]) < 2), T = st[tyre], pT = tyre >= 0 ? R.spot(tyre) : null; if (T) T.stuff = 'rubber';
    const tyreUp = pT ? R.fire(tyre, 1) : null;
    line(!!saidUp && saidUp.met === 2 && saidUp.spark > 0 && saidUp.chips === 0 && saidUp.dust === 0 && count(saidUp, 'bullet') === 2
      && !!tyreUp && tyreUp.met === 1 && tyreUp.chips > 0 && count(tyreUp, 'damage 0') === 0 && count(tyreUp, 'bullet') === 1 && count(tyreUp, 'chip') === tyreUp.marks.length - 1,
      '2 the same body saying \'carpaint\': two rounds give sparks, no chips and no dust, and each leaves its hole. And a tyre saying \'rubber\' (by its dark paint it would be metal): it gives the chips plastic gives, and where the chaos lane would draw the dent of sheet metal it draws a chip',
      `body: ${saidUp ? say(saidUp) : 'no spot'}\n    tyre (by what it does ${T ? surfOf(T) : '-'}): ${tyreUp ? say(tyreUp) : 'no spot'}`);

    // (3) the car's glass, saying it is glass: a solid thing, not a pane — it cracks, every round, and stands
    G.stuff = 'glass'; const flags0 = G.flags, pG = R.spot(glass);
    const solid = solidGlass(G), glassUp = pG ? R.fire(glass) : null, glassCold = pG ? R.fire(glass, 2, 0, 0) : null;
    const stands = G.flags === flags0 && !!pG && R.game.col.raycast([pG[0], 1.35, pG[2]], v3.normalize(v3.sub(G.c, [pG[0], 1.35, pG[2]])), 60)?.index === glass;
    line(!!glassUp && !!glassCold && solid && glassUp.met === 2 && count(glassUp, 'damage 9') === glassUp.marks.length && glassUp.marks.length >= 2 && glassUp.dust === 0 && glassUp.chips > 0 && glassUp.chips <= 2 * 3
      && count(glassCold, 'damage 9') === 2 && count(glassCold, 'bullet') === 0 && stands && (G.flags & BoxFlag.Glass) === 0,
      '3 the car\'s glass saying \'glass\' is a solid thing of glass and not a pane: each of two rounds leaves the crack on that box in place of the hole, and where the chaos lane would draw the dent of the metal its dark paint was taken for it draws a crack too — a pinch of glass (three chips a round, no more) and no dust — with the chaos lane off too, round for round — and the box stands as it stood (the next round meets it)',
      `solid glass ${solid} · ${glassUp ? say(glassUp) : 'no spot'} · the chaos lane off: ${glassCold ? say(glassCold) : 'no spot'} · stands ${stands}`);

    // (4) a thing that is metal by its paint, and says it is or says nothing (every box of a level file says what its row is made of since 11-04: a cabinet
    //     says steel): sparks and no chips
    const METAL_WORDS: readonly (string | undefined)[] = [undefined, 'steel', 'metal', 'sheet'];
    const metalOf = (Q: ReturnType<typeof range>) => Q.st.map((b: any, i: number) => i).filter((i: number) => { const b = Q.st[i]; return METAL_WORDS.includes(b.stuff) && !(b.flags & (BoxFlag.Glass | BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Dynamic)) && !b.rot && surfOf(b) === 'metal' && b.h[1] > 0.5 && b.c[1] - b.h[1] < 0.6 && b.c[1] + b.h[1] > 1.5 && Math.max(b.h[0], b.h[2]) > 0.25 && !Q.game.destr.itemOf(i); });
    const first = (Q: ReturnType<typeof range>, cand: number[]): number => { for (const i of cand.slice(0, 40)) if (Q.spot(i)) return i; return -1; };
    { const cand = metalOf(R), m = first(R, cand);
      const metalUp = m >= 0 ? R.fire(m, 1) : null;
      line(!!metalUp && metalUp.met === 1 && metalUp.chips === 0 && metalUp.spark > 0,
        '4 a thing that is metal by its paint, and says so or says nothing (a cabinet, a rack): a round knocks sparks out of it and no chips',
        `box ${m} of ${cand.length} · ${metalUp ? say(metalUp) : 'no spot'}`); }

    // (6) the words and the lint
    { const stale = (HIT_STUFFS as string[]).filter(s => !(BOX_STUFFS as string[]).includes(s));
      const d = JSON.parse(readFileSync(`${ROOT}/levels/${FILE}`, 'utf8')); const gi = d.boxes.findIndex((b: any) => sized(b, [1.25, 0.27, 0.8])), ti = d.boxes.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12])); d.boxes[gi].stuff = 'glass'; d.boxes[ti].stuff = 'rubber';
      const errs = validateLevelData(parseLevelData(JSON.stringify(d))).filter((e: any) => (e.level ?? 'error') === 'error'), again = levelToData(loadLevel(parseLevelData(JSON.stringify(d))), {}), back = `${again.boxes[gi]?.stuff}, ${again.boxes[ti]?.stuff}`;
      const odd = { flags: 0, stuff: 'constructor' } as any, guarded = saidSurf(odd) === undefined && saidSurf({ flags: 0, stuff: 'toString' } as any) === undefined;
      line(stale.length === 0 && ['glass', 'rubber', 'carpaint', 'steel'].every(w => (HIT_STUFFS as string[]).includes(w)) && !(HIT_STUFFS as string[]).includes('paint') && !(HIT_STUFFS as string[]).includes('enamel') && errs.length === 0 && back === 'glass, rubber' && guarded,
        '6 every word the picture of a hit reads is a stuff a box may say — \'glass\' and \'rubber\' among them, and never \'paint\' or \'enamel\', which are a coat on anything — the level lint takes boxes that say the two and writes them back, and a word that is no stuff at all is read as none',
        `${(HIT_STUFFS as string[]).length} words${stale.length ? `, not a box's: ${stale.join(', ')}` : ''} · lint errors ${errs.length}${errs.length ? ` (${errs[0].msg ?? errs[0]})` : ''} · written back ${back} · a word that is none ${guarded}`); }

    // (7) a mark on a cut face, or round a round thing, lies where the round met it (render/decals.ts MARKCUT): on a box with a leaning face, a mark laid at a point
    //     of that face stays there (MARKCUT_BREAK=1: the box handed over with its cut taken off, the mark is pulled out to the square face)
    { const { cutPlane, cutRecord } = await import(`${ROOT}/src/scene/cut.ts`), { makeBox } = await import(`${ROOT}/src/scene/boxes.ts`), D = W.game.decals;
      const h: Vec3 = [0.6, 0.27, 0.8], t = 12 * Math.PI / 180, nl: Vec3 = [0, Math.sin(t), Math.cos(t)], dl = Math.cos(t) * h[2] - Math.sin(t) * h[1];   // (the +z face leaned in 12° about its bottom edge)
      const b: any = makeBox({ c: [2, 1.2, 3], h, yaw: 0, albedo: [0.07, 0.08, 0.1] }); b.cut = cutRecord(h, [cutPlane(nl, dl)]).cut;
      const lp: Vec3 = [0.1, 0.15, (dl - nl[1] * 0.15) / nl[2]], p: Vec3 = [b.c[0] + lp[0], b.c[1] + lp[1], b.c[2] + lp[2]], flat: Vec3 = [b.c[0] + 0.1, b.c[1] - 0.26, b.c[2] + h[2]];
      const drum: any = makeBox({ c: [5, 0.45, 3], h: [0.3, 0.45, 0.3], yaw: 0, albedo: [0.2, 0.2, 0.22] }); drum.round = true; const a30 = Math.PI / 6, nr: Vec3 = [Math.cos(a30), 0, Math.sin(a30)], pr: Vec3 = [drum.c[0] + 0.3 * nr[0], drum.c[1] + 0.1, drum.c[2] + 0.3 * nr[2]];   // (a drum's side, 30° round from the box's own axis)
      const was7 = { en: D.enabled, n: D.list.length }; D.enabled = true; let up7: any, face7: any, rUp: any, rTop: any;
      try { if (CUT_BREAK) b.cut = undefined; up7 = D.add('bullet', p, nl, 0.03, { clampTo: b }); face7 = D.add('bullet', flat, [0, 0, 1], 0.03, { clampTo: b }); rUp = D.add('bullet', pr, nr, 0.03, { clampTo: drum }); rTop = D.add('bullet', [drum.c[0] + 0.1, drum.c[1] + 0.45, drum.c[2]], [0, 1, 0], 0.03, { clampTo: drum }); }
      finally { D.enabled = was7.en; D.list.length = was7.n; }
      const off = (d: any) => d ? Math.abs((d.pos[0] - b.c[0]) * nl[0] + (d.pos[1] - b.c[1]) * nl[1] + (d.pos[2] - b.c[2]) * nl[2] - dl) : 9;
      line(!!up7 && !!face7 && off(up7) < 1e-6 && v3.dist(up7.pos, p) < 1e-6 && Math.abs(face7.pos[2] - (b.c[2] + h[2])) < 1e-9 && cutRecord(h, [cutPlane(nl, dl)]).cut.length === 1
        && !!rUp && !!rTop && v3.dist(rUp.pos, pr) < 1e-6 && Math.abs(rTop.pos[1] - 0.9) < 1e-9,
        '7 a mark on a cut face lies where the round met it: laid at a point of a pane\'s leaning face it stays on that face; a mark on a face the cut left alone is laid on that face; and round a drum\'s side the same, where the round met it, and on its flat top as ever',
        `off the leaning face: ${(off(up7) * 1000).toFixed(2)} mm · on the plain face ${face7 ? Math.abs(face7.pos[2] - (b.c[2] + h[2])).toExponential(1) : '-'} · a drum's side: ${rUp ? (v3.dist(rUp.pos, pr) * 1000).toFixed(2) : '-'} mm from the hit`); }
    // (8) THE VEHICLES AS THE LEVELS HAVE THEM say what they are made of, and a round reads it, nothing put on them by this check: the office's car on a
    //     fresh stand; the kits' vehicles in the showroom yard, a round into a body, a window and a tyre there; and every level file's plain car (its
    //     default-look twin's too, whose body the skin repaints) and every shaped box in a truck enamel
    { const V: any = await stand(), Q = range(V), s8 = Q.st; Q.draw();
      const bd = s8.findIndex((b: any) => sized(b, [2.2, 0.34, 0.86])), gl = s8.findIndex((b: any, i: number) => i !== bd && sized(b, [1.25, 0.27, 0.8]) && Math.hypot(b.c[0] - s8[bd].c[0], b.c[2] - s8[bd].c[2]) < 0.6);
      const ty = s8.findIndex((b: any) => sized(b, [0.32, 0.3, 0.12]) && Math.hypot(b.c[0] - s8[bd].c[0], b.c[2] - s8[bd].c[2]) < 2);
      const car = `${s8[bd]?.stuff}, ${s8[gl]?.stuff}, ${s8[ty]?.stuff}`;
      const b8 = Q.spot(bd, LOW) ? Q.fire(bd, 2, LOW) : null, g8 = Q.spot(gl) ? Q.fire(gl) : null, t8 = ty >= 0 && Q.spot(ty) ? Q.fire(ty, 1) : null;
      // the truck park: the kit's vehicles stand there
      const Y: any = await stand('tut-pair.json'), QY = range(Y), sy = QY.st; QY.draw();
      const hit = (b: any) => !(b.flags & (BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Dynamic | BoxFlag.Glass)) && !b.rot && !b.piece, idx = (w: string, tall: number) => sy.map((b: any, i: number) => i).filter((i: number) => sy[i].stuff === w && hit(sy[i]) && sy[i].h[1] >= tall).sort((a: number, b: number) => sy[b].h[1] * Math.min(sy[b].h[0], sy[b].h[2]) - sy[a].h[1] * Math.min(sy[a].h[0], sy[a].h[2]));   // (the biggest face first: a round aimed at its middle meets it)
      const yb = first(QY, idx('carpaint', 0.3)), kb = yb >= 0 ? QY.fire(yb, 2) : null, yg = first(QY, idx('glass', 0.15)), kg = yg >= 0 ? QY.fire(yg) : null, yt = first(QY, idx('rubber', 0.2)), kt = yt >= 0 ? QY.fire(yt, 1) : null;   // (each fired from the spot found for it: finding a spot moves the man)
      // every level file: the plain car (by its boxes' sizes) and the kits' vehicles (a shaped box in a truck enamel)
      const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')), wrong: string[] = []; let cars = 0, shaped = 0, built = 0, panes = 0;
      const TRUCK = ['truckWhite', 'truckTan', 'truckRed'];
      for (const f of files) { const d = JSON.parse(readFileSync(`${ROOT}/levels/${f}`, 'utf8')), bs = loadLevel(parseLevelData(JSON.stringify(d))).boxes;
        for (const b of bs as any[]) { if (b.stuff === 'glass' && (b.flags & BoxFlag.Glass)) panes++;
          if (sized(b, [2.2, 0.34, 0.86]) && b.shape) { cars++; const g = (bs as any[]).find(q => sized(q, [1.25, 0.27, 0.8]) && Math.hypot(q.c[0] - b.c[0], q.c[2] - b.c[2]) < 0.6), t = (bs as any[]).find(q => sized(q, [0.32, 0.3, 0.12]) && Math.hypot(q.c[0] - b.c[0], q.c[2] - b.c[2]) < 2);
            if (b.stuff !== 'carpaint' || g?.stuff !== 'glass' || t?.stuff !== 'rubber') wrong.push(`${f} car: ${b.stuff}, ${g?.stuff}, ${t?.stuff}`); } }
        const word = (r: any) => r.stuff ?? d.materials[r.material]?.stuff, drawn = (r: any) => (r.flags ?? []).some((x: string) => x === 'NoShadow' || x === 'NoPrimary');
        const wins = (d.boxes as any[]).filter(r => word(r) === 'glass' && !drawn(r) && r.h[1] > 0.15);   // (a vehicle's window: solid glass a round can meet, taller than a counter's top)
        for (const r of d.boxes as any[]) if (TRUCK.includes(r.material) && !r.shape && !drawn(r) && Math.max(r.h[0], r.h[1], r.h[2]) > 0.2 && wins.some(w => Math.hypot(w.c[0] - r.c[0], w.c[2] - r.c[2]) < 2.6 && r.c[1] - r.h[1] < w.c[1])) { built++; if (word(r) !== 'carpaint') wrong.push(`${f} ${r.id} (built by hand): ${r.material} says ${word(r)}`); }   // (a vehicle a level builds by hand: a panel in a truck enamel beside and under a window)
        for (const r of d.boxes as any[]) if (TRUCK.includes(r.material) && r.shape && Math.max(r.h[0], r.h[1], r.h[2]) > 0.2) { shaped++; const w = r.stuff ?? d.materials[r.material]?.stuff; if (w !== 'carpaint') wrong.push(`${f} ${r.id}: ${r.material} says ${w}`); } }   // (a body panel, by the file's own words: a tail lamp's lens in the red enamel is 15 cm and stays paint)
      line(car === 'carpaint, glass, rubber' && !!b8 && b8.met === 2 && b8.spark > 0 && b8.chips === 0 && b8.dust === 0 && count(b8, 'bullet') === 2
        && !!g8 && g8.marks.length >= 2 && count(g8, 'damage 9') === g8.marks.length && (s8[gl].flags & BoxFlag.Glass) === 0
        && !!t8 && t8.met === 1 && t8.chips > 0 && count(t8, 'damage 0') === 0
        && !!kb && kb.met === 2 && kb.spark > 0 && kb.chips === 0 && kb.dust === 0 && !!kg && kg.met >= 1 && kg.marks.length >= 1 && count(kg, 'damage 9') === kg.marks.length && !!kt && kt.met === 1 && kt.chips > 0 && count(kt, 'damage 0') === 0
        && wrong.length === 0 && cars >= 20 && shaped >= 40 && built >= 6 && panes === 0 && stuffOfName('truckWhite') === 'paint',
        '8 the vehicles as the levels have them say what they are made of, nothing put on them here: a round into the office car\'s body gives sparks and no chips or dust, one into its window the crack on glass that stands, one into a tyre plastic\'s chips and no dent; the same of a body, a window and a tyre of the kits\' vehicles in the truck park; and in every level file each plain car says car paint, glass and rubber (a default-look twin\'s repainted car too) and each shaped panel in a truck enamel says car paint, and so does each plain panel in one beside a window (a vehicle a level builds by hand), while the enamel\'s own row stays paint (a cot, a locker, a floor line); no pane says glass',
        `the office's car says ${car} · body: ${b8 ? say(b8) : 'no spot'} · window: ${g8 ? say(g8) : 'no spot'} · tyre: ${t8 ? say(t8) : 'no spot'}\n    the yard's vehicles · body (box ${yb}): ${kb ? say(kb) : 'no spot'} · window (box ${yg}): ${kg ? say(kg) : 'no spot'} · tyre (box ${yt}): ${kt ? say(kt) : 'no spot'}\n    ${files.length} level files: ${cars} plain cars, ${shaped} shaped panels in a truck enamel, ${built} plain ones beside a window, ${panes} panes saying glass${wrong.length ? ` · WRONG: ${wrong.slice(0, 6).join(' · ')}${wrong.length > 6 ? ` … ${wrong.length}` : ''}` : ''}`); }
  } catch (e) { console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); all = false; }
  finally { Math.random = rnd; }
  console.log(`=== hitstuff: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
if (import.meta.main) process.exit((await hitstuffCheck()) ? 0 : 1);
