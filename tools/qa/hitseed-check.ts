// `bun run tools/qa/hitseed-check.ts` (probes.ts hitseed): A HIT'S SEED IS THE BOX'S OWN, NEVER ITS PLACE IN THE LEVEL'S LIST (src/game/glass.ts boxSeed, hitSeed).
//
// The fault it holds off: a pane's break, a round's turn through glass, its turn through a thin wall and a window bar's break were seeded by the box's index in the level. One box
// written into a level's file ahead of a pane moved every index after it, so the same shot rolled another star, another break and another turn, and every run
// that met glass or a thin wall read another digest with no change of play.
//
// THE STAGE: the office stood up twice from its file. Game A is the file. Game B is the file with ONE INERT BOX written first (a speck inside the ground slab
// that no ray, eye or body meets), so every box of B stands one place later in the list than its twin in A. Every line asks the two games the same thing and
// wants the same answer to the last bit. The check raises what it needs itself (rounds through glass, the fracture model, rounds through thin walls), so it
// says the same on the frozen stand and under TPS_PROFILE=shipped. The last line stands the barracks up the same two ways for a window's bar (the office has none).
//
// HITSEED_BREAK=1: B's glass and thin things stand a millimetre higher than A's, so each is another box to the seed. Every line must then FAIL, and by the
// seed alone: every blow is given from the struck box's own middle and every answer is read from it (to the thousandth of a millimetre), so a box that
// stands a millimetre off gives the same numbers wherever the seed is not in them.
type Vec3 = [number, number, number];
const BREAK = process.env.HITSEED_BREAK === '1';

export async function hitseedCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { readFileSync } = await import('node:fs');
  const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { GLASSFX, boxSeed, hitSeed, glassImpact, glassPanes, isSheet, crackNear } = await import(`${ROOT}/src/game/glass.ts`);
  const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
  const { GLASSPEN, roundEnergy } = await import(`${ROOT}/src/game/glasspen.ts`);
  const { WALLPEN, thinPath, thinThing } = await import(`${ROOT}/src/game/wallpen.ts`);
  const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
  const { WINBARS, barsStanding, barMeet, barsSettle } = await import(`${ROOT}/src/game/winbars.ts`);
  const DT = 1 / 60; const f2 = (x: number) => x.toFixed(2);
  let failed = false; const line = (ok: boolean, s: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  /** a record as one comparable line: every number to the thousandth of a millimetre (a seed is a whole number and stays what it is) */
  const said = (x: unknown): string => JSON.stringify(x, (_k, v) => typeof v === 'number' && !Number.isInteger(v) ? Math.round(v * 1e6) / 1e6 : v);
  const from = (b: any, p: ArrayLike<number>): Vec3 => [p[0] - b.c[0], p[1] - b.c[1], p[2] - b.c[2]];
  const was = { fx: GLASSFX.on, pen: GLASSPEN.on, wall: WALLPEN.on, bars: WINBARS.on }, random = Math.random;
  /** the seed as it was until this check: the box's place in the list, then the point */
  const seedByPlace = (bi: number, p: Vec3) => { let h = Math.imul(bi + 1, 0x9E3779B1) >>> 0; for (const x of p) { h = Math.imul(h ^ (Math.round(x * 1000) | 0), 0x85EBCA77) >>> 0; h = (h ^ (h >>> 13)) >>> 0; } return Math.imul(h, 0xC2B2AE3D) >>> 0; };

  // ------------------------------------------------------------ the two files
  /** a level's file, and the same file with the inert box written first (and, under the break word, its glass and thin things a millimetre up) */
  const twoFiles = (name: string) => { const A = JSON.parse(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8')), B = JSON.parse(JSON.stringify(A)), slab = B.boxes[0];   // (the ground: the speck stands in the middle of it, hidden from every ray as a painted line is)
    B.boxes.unshift({ id: 'box_inert_first', floor: slab.floor, group: slab.group, c: [slab.c[0], slab.c[1], slab.c[2]], h: [0.01, Math.min(0.01, slab.h[1] / 2), 0.01], material: slab.material, flags: ['NoShadow'] });
    if (BREAK) for (const b of B.boxes) if (b.flags?.includes('Glass') || (Math.min(b.h[0], b.h[2]) <= 0.12 && b.h[1] >= 0.5 && !b.flags?.includes('NoShadow'))) b.c = [b.c[0], b.c[1] + 0.001, b.c[2]];
    return [A, B]; };
  const [fileA, fileB] = twoFiles('office');
  /** a fresh game on one of the two files, the same die, the parts the lines ask raised, nobody acting */
  const world = async (file: unknown, opts: { chaos?: number } = {}) => {
    Math.random = mulberry32(4242);
    const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(JSON.stringify(file))) })); const g = W.game;
    GLASSFX.on = true; GLASSPEN.on = true; WALLPEN.on = true; WINBARS.on = true;   // AFTER standUp: the stand-up writes its own
    g.tune.glassBreak = true; g.tune.chaos = opts.chaos ?? 0.5; g.godMode = true; g.playerInvisible = true; g.aiEnabled = false;
    for (let i = 0; i < 3; i++) W.step(DT);
    // every fragment the game's glass lets go, as it goes (a bystander on the fracture model's one way out): the frame, the pane by where it stands on the
    // floor plan, and the piece from the pane's own middle
    { const out: string[] = W.pieces = [], gb = g.glassBreak, emit = gb.emit.bind(gb);
      gb.emit = (st: any, f: any, key: number) => { out.push(said([Math.round(g.time * 60), st.box.c[0], st.box.c[2], f.kind, f.cause, from(st.box, f.centre), f.velocity, f.spin, f.seed, key])); return emit(st, f, key); }; }
    return W;
  };
  /** B's box that is A's box `b`: the same middle and sizes (a millimetre up under the break word) and the same flags */
  const twinOf = (gB: any, b: any): number => gB.level.boxes.findIndex((x: any) => x.flags === b.flags && x.h.every((v: number, k: number) => v === b.h[k]) && x.c[0] === b.c[0] && x.c[2] === b.c[2] && Math.abs(x.c[1] - b.c[1]) < 0.002);
  const paneState = (g: any, bi: number) => { const st = g.glassBreak.panes.get(bi); return st ? said([st.type, st.sites, [...st.open], st.nOpen, st.run, st.gone, st.energy, st.damage, st.pending.map((p: any) => [p.fid, p.at, p.cause]), st.dice ? [st.dice.seed, st.dice.next, st.dice.clumps.length] : null]) : 'no state'; };
  const settle = (W: any, s: number) => { for (let i = 0; i < Math.round(s * 60); i++) W.step(DT); };

  try {
    console.log(`\n=== hitseed: a hit's seed is the box's own, never its place in the level's list${BREAK ? ' (HITSEED_BREAK: the twin game\'s glass and thin things stand a millimetre off, so each is another box)' : ''} ===`);
    // ------------------------------------------------------------ 1. the word itself
    const WA = await world(fileA), WB = await world(fileB); const gA = WA.game, gB = WB.game;
    const panesA: number[] = glassPanes(gA).filter((bi: number) => isSheet(gA.level.boxes[bi]));
    const pairs = panesA.map(a => ({ a, b: twinOf(gB, gA.level.boxes[a]) }));
    const pts = (b: any): Vec3[] => [[b.c[0], b.c[1], b.c[2]], [b.c[0] + 0.137, b.c[1] - 0.211, b.c[2] + 0.093], [b.c[0] - 0.052, b.c[1] + 0.308, b.c[2] - 0.171]];
    let same = 0, asked = 0, placeParts = 0, moved = 0;
    for (const { a, b } of pairs) { if (b < 0) continue; if (b !== a) moved++; const A = gA.level.boxes[a], B = gB.level.boxes[b];
      for (const p of pts(A)) { asked++; if (hitSeed(A, p) === hitSeed(B, p)) same++; if (seedByPlace(a, p) !== seedByPlace(b, p)) placeParts++; } }
    const words = new Set(panesA.map(a => boxSeed(gA.level.boxes[a])));
    line(panesA.length >= 10 && pairs.every(p => p.b >= 0) && moved === panesA.length && gB.level.boxes.length === gA.level.boxes.length + 1 && same === asked && words.size === panesA.length && placeParts === asked,
      `the word: ${panesA.length} sheets of glass, each found again in the twin game ${moved} of them one place later in a list of ${gB.level.boxes.length} for ${gA.level.boxes.length}; by the box ${same} of ${asked} points read the same seed in both games (by the place in the list, as it was, ${placeParts} of ${asked} read another); ${words.size} sheets say ${panesA.length} different words`);

    // ------------------------------------------------------------ 2. the star (the rule with the fracture model off: a decal seeded off the hit)
    const byType = (g: any, t: string) => panesA.filter(bi => g.glassBreak.typeOfPane(bi) === t);
    const annealed = byType(gA, 'annealed'), tempered = byType(gA, 'tempered'), laminated = byType(gA, 'laminated');
    const N = (b: any): Vec3 => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); return b.h[0] < b.h[2] ? [cs, 0, -sn] : [sn, 0, cs]; };
    const star = (g: any, bi: number) => { const b = g.level.boxes[bi], n = N(b), at: Vec3 = [b.c[0] + 0.11, b.c[1] + 0.07, b.c[2] + 0.11]; g.tune.glassBreak = false; const d0 = g.decals.list.length;
      glassImpact(g, bi, v3.mad(at, n, -v3.dot(v3.sub(at, b.c), n)), v3.scale(n, -1), 1, 0); g.tune.glassBreak = true; const d = g.decals.list[g.decals.list.length - 1];
      return g.decals.list.length === d0 + 1 ? said([d.kind, from(b, d.pos), d.seed, d.radius, d.t, d.param]) : 'no decal'; };
    { const a = panesA[0], b = pairs[0].b; const sA = star(gA, a), sB = star(gB, b);
      line(sA !== 'no decal' && sA === sB, `the star: one blow on the first sheet with the fracture model off leaves the same decal in both games (where on the pane, look seed, reach and turn): ${sA}${sA === sB ? '' : ` against ${sB}`}`); }

    // ------------------------------------------------------------ 3. the break, each kind of glass: rounds, then two seconds of what follows
    const blows: [number, number, number, boolean][] = [[0.12, 0.2, 1.8, true], [-0.2, -0.3, 1.0, false], [0.05, 0.45, 1.8, true], [-0.31, 0.1, 0.4, false], [0.3, -0.55, 2.0, true], [0.0, 0.0, 1.8, true]];
    const shoot = (W: any, bi: number) => { const g = W.game, b = g.level.boxes[bi], n = N(b), wide: Vec3 = b.h[0] < b.h[2] ? [Math.sin(b.yaw), 0, Math.cos(b.yaw)] : [Math.cos(b.yaw), 0, -Math.sin(b.yaw)], hu = Math.max(b.h[0], b.h[2]), hv = b.h[1];
      const res: string[] = [];
      for (const [fu, fv, e, through] of blows) { const at: Vec3 = [b.c[0] + wide[0] * fu * hu, b.c[1] + fv * hv, b.c[2] + wide[2] * fu * hu]; res.push(said(glassImpact(g, bi, at, v3.scale(n, -1), e, 0, 'shot', false, { through }))); settle(W, 0.1); }
      settle(W, 2); return res; };
    const kinds: [string, number[]][] = [['plain float', annealed], ['toughened', tempered], ['laminated', laminated]];
    for (const [name, list] of kinds) {
      if (!list.length) { line(false, `the break, ${name}: the office has no such sheet to ask`); continue; }
      const a = list[0], b = twinOf(gB, gA.level.boxes[a]); const a0 = WA.pieces.length, b0 = WB.pieces.length;
      const ra = shoot(WA, a), rb = shoot(WB, b); const sa = paneState(gA, a), sb = paneState(gB, b); const fa: string[] = WA.pieces.slice(a0), fb: string[] = WB.pieces.slice(b0);
      const diff = fa.findIndex((x, i) => x !== fb[i]);
      line(ra.join() === rb.join() && sa === sb && fa.length === fb.length && diff < 0 && sa !== 'no state' && (name === 'laminated' || fa.length >= 3),
        `the break, ${name} (sheet ${a} / ${b}): six blows and two seconds on, the glass answered each blow the same, the pane's record is the same (stars, open pieces, pieces waiting and when they go) and ${fa.length} pieces let go the same in both games, frame for frame (where, how fast, their spin and their own seeds)${diff >= 0 ? ` — the first to part is piece ${diff}: ${fa[diff]} against ${fb[diff]}` : sa === sb ? '' : ' — the records part'}`);
    }

    // ------------------------------------------------------------ 4. a blast and a stun beside fresh glass
    { const WA2 = await world(fileA), WB2 = await world(fileB); const far = annealed[annealed.length - 1];
      // (the charge half a metre under the far float sheet's middle and 0.8 m off its face, the stun 2.5 m along from it on the floor: both from the sheet's own middle)
      const touched = [WA2, WB2].map(W => { const g = W.game, b = g.level.boxes[W === WA2 ? far : twinOf(g, gA.level.boxes[far])], n = N(b); const at: Vec3 = [b.c[0] + n[0] * 0.8, b.c[1] - 0.5, b.c[2] + n[2] * 0.8];
        const k = g.glassBreak.blast(at, { radius: 4, power: 1 }); const s = crackNear(g, [at[0] + 2.5, b.c[1] - 1.2, at[2]], 6, 0); settle(W, 2.5); return [k, s]; });
      const fa: string[] = WA2.pieces, fb: string[] = WB2.pieces;
      const all = (g: any) => panesA.map(bi => paneState(g, g === WA2.game ? bi : twinOf(g, gA.level.boxes[bi]))).join('|');
      const decals = (g: any) => said(g.decals.list.filter((d: any) => d.kind === 'glassHit').map((d: any) => [from(g.level.boxes[d.pane], d.pos), d.seed, d.radius, d.t]));
      const diff = fa.findIndex((x, i) => x !== fb[i]);
      line(touched[0][0] >= 1 && touched[0][1] >= 1 && touched[0].join() === touched[1].join() && fa.length === fb.length && fa.length >= 3 && diff < 0 && all(WA2.game) === all(WB2.game) && decals(WA2.game) === decals(WB2.game),
        `a blast and a stun beside fresh glass: the blast touched ${touched[0][0]} sheets and the stun starred ${touched[0][1]} in both games; ${fa.length} pieces let go the same, every sheet's record is the same and so is every star left on the glass${diff >= 0 ? ` — the first piece to part is ${diff}: ${fa[diff]} against ${fb[diff]}` : ''}`); }

    // ------------------------------------------------------------ 5. the turn: flights swept over the floor, through glass, thin walls and door leaves
    { const WA3 = await world(fileA, { chaos: 0 }), WB3 = await world(fileB, { chaos: 0 }); const a = WA3.game, b = WB3.game; const rifle = roundEnergy(WEAPONS.fn2000);
      const rnd = mulberry32(0x5eed);
      const nav = a.col.nav; const cells: Vec3[] = []; for (let iz = 1; iz < nav.H - 1; iz++) for (let ix = 1; ix < nav.W - 1; ix++) if (!nav.blocked[nav.idx(ix, iz)]) cells.push([(ix + 0.5) * nav.cell, 0, (iz + 0.5) * nav.cell]);
      let flights = 0, panes = 0, walls = 0, leaves = 0, turned = 0, parted = 0, turnOnly = 0, eg = '';
      const sig = (P: any) => P ? said([P.cross.map((x: any) => [x.at, x.dirOut, x.eOut]), P.walls.map((x: any) => [x.index >= 1e6 ? 'leaf' : 'static', x.at, x.dirOut, x.eOut]), P.len]) : 'null';
      /** the first thing a flight crossed, with its turn left out: where it went in and what it had left */
      const first = (P: any) => { const x = P && [...P.cross, ...P.walls].sort((p: any, q: any) => p.t - q.t)[0]; return x ? said([x.at, x.eOut]) : 'null'; };
      for (let i = 0; i < 6000; i++) { const c = cells[(rnd() * cells.length) | 0], yaw = rnd() * Math.PI * 2, pitch = (rnd() - 0.5) * 0.5; const o: Vec3 = [c[0], 0.3 + rnd() * 1.5, c[2]], d: Vec3 = v3.normalize([Math.sin(yaw) * Math.cos(pitch), Math.sin(pitch), Math.cos(yaw) * Math.cos(pitch)]);
        const PA = thinPath(a, o, d, 30, rifle), PB = thinPath(b, o, d, 30, rifle); if (!PA && !PB) continue; flights++;
        if (PA) { panes += PA.cross.length; for (const x of PA.walls) { if (x.index >= 1e6) leaves++; else walls++; } for (const x of [...PA.cross, ...PA.walls]) if (v3.dot(x.dirIn, x.dirOut) < 1 - 1e-12) turned++; }
        if (sig(PA) !== sig(PB)) { parted++; if (first(PA) === first(PB)) turnOnly++; if (!eg) eg = `from (${o.map(f2)}) along (${d.map(f2)})`; } }
      const thin = a.col.boxes.filter((_: unknown, i: number) => thinThing(a, i)).length;
      line(flights > 200 && panes >= 20 && walls >= 100 && leaves >= 10 && turned > (panes + walls + leaves) / 2 && parted === 0,
        `the turn: 6000 rifle flights over the floor, ${flights} of them through something (${panes} sheets of glass, ${walls} thin walls and things of the office's ${thin}, ${leaves} door leaves; ${turned} crossings turned the round): ${parted} flights left the two games by another way${eg ? ` (${turnOnly} of them went into the first thing they met at the same spot with the same energy left: the turn alone parts them) — e.g. ${eg}` : ''}`); }

    // ------------------------------------------------------------ 6. a window's bar (the barracks: bars and no glass): one round, and the pieces it leaves as
    { const [barA, barB] = twoFiles('barracks'); const WA4 = await world(barA, { chaos: 0 }), WB4 = await world(barB, { chaos: 0 }); const a = WA4.game, b = WB4.game;
      const bars: number[] = barsStanding(a), twins = bars.map(i => twinOf(b, a.level.boxes[i])); let broke = 0;
      const pieces = (g: any, i: number) => { const X = g.col.boxes[i], n0 = g.destr.debris.list.length, dir: Vec3 = X.h[0] > X.h[2] ? [Math.cos(X.yaw), 0, -Math.sin(X.yaw)] : [Math.sin(X.yaw), 0, Math.cos(X.yaw)];
        const at: Vec3 = [X.c[0], X.c[1] + 0.21 * X.h[1], X.c[2]]; broke += barsSettle(g, [barMeet(g, i, 1, at, dir)], 1, 0, false);
        return said(g.destr.debris.list.slice(n0).map((c: any) => [from(X, c.box.c), c.box.h, c.v, c.w])); };
      let same = 0, tried = 0, eg = '', sets = new Set<string>();
      for (let k = 0; k < Math.min(6, bars.length); k++) { if (twins[k] < 0) continue; tried++; const pa = pieces(a, bars[k]), pb = pieces(b, twins[k]); sets.add(pa); if (pa === pb && pa !== '[]') same++; else if (!eg) eg = `bar ${bars[k]} / ${twins[k]}: ${pa.slice(0, 200)} against ${pb.slice(0, 200)}`; }
      line(bars.length >= 6 && twins.every(t => t >= 0) && tried === 6 && broke === 12 && same === 6 && sets.size > 1,
        `a window's bar: the barracks stands ${bars.length} bars, each found again in the twin game; one round into each of six broke ${broke} bars in the two games and ${same} of 6 left as the same pieces (their lengths, where each starts from, how fast and how it spins; ${sets.size} different sets of pieces among the six)${eg ? ` — ${eg}` : ''}`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { GLASSFX.on = was.fx; GLASSPEN.on = was.pen; WALLPEN.on = was.wall; WINBARS.on = was.bars; Math.random = random; }
  console.log(failed ? '=== hitseed-check: FAIL ===' : '=== hitseed-check: ALL PASS ===');
  return !failed;
}
if (import.meta.main) process.exit((await hitseedCheck()) ? 0 : 1);
