// STANDING OFF A WALL CHARGE OUT OF KNOWN FIRE — the check behind WALLCHARGE.outOfFire (src/game/wallcharge.ts underFire / knownFoes; a settled
// rule since 2026-11-07, as is the safety radius it stands on: every tool stands the game with both on).
//   bun run tools/qa/standfire-check.ts        (also `probes.ts standfire`)
// The maintainer's own case, staged from his tape (The pass, 10-14: "the two guys move away from the wall charge to a place that doesn't make sense (isn't safe
// since there are known enemies that can shoot them from there)"): a charge on the bunk room's long wall from the yard, three men to stand off, and the team
// knowing what it knew then — a man in the bunk room called nine seconds before, another by its west window and one at the tea room's window, each where he was
// last seen. The marks as they were ran along the wall past its end, across the bunk room window's lane.
//   (3) Sam where he stood: every mark dealt is out of the blast;
//   (4) Sam elsewhere: the first mark dealt (the setter's) is the least under fire of them, and all are out of the blast;
//   (6) a sighting counts fireKnownS and no longer: a man last seen 29 s ago is known and the marks are dealt for him, one last seen 31 s ago is not;
//   (7) the fog off (the picture shows everybody): a living enemy counts where he stands;
//   (7b) a man the squad is shooting at counts where he stands (he is in the picture) · (7c) the wall about to go is no cover where it will open;
//   (8) the order itself deals those marks, and the men walk to them and stand.
// 2026-11-07, what went with the rule's off side (it cannot be staged any more): (1) the stand's line; (2) 'the marks as they were, one in the middle of a known
// man's lane' and every half of a line that laid the marks beside those ('less under fire than the worst of those', 'a place those never took'); (5) 'nobody
// known: the marks as they were'; (9) the recording from before the key and the page's flag. What is left of (3), (4), (6), (7) and (8) holds numbers of its own.
// STANDFIRE_BREAK=1 = the team is told of nobody (`know` writes nothing): the lines that ask who is known must fail.   exit 0 = all hold · 1 = one does not
// Headless: lines are the collision's own (glass passes). Never seen on a GPU; nobody has played it.
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
const { WALLCHARGE, WALL_ORDER, wallAt, wallTarget, wallPt, safeRadius } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
type Vec3 = [number, number, number];

/** the tape's moment: the charge's place on the wall, where Sam and the three stood when the order was given, and where the team had last seen the three it knew of */
const CHARGE: Vec3 = [31.28, 0, 18.14],   // (on the wall's yard face: wallAt takes a point within WALLCHARGE.pickM of it)
  SAM: Vec3 = [26.4, 0, 20.8], ELSEWHERE: Vec3 = [22.0, 0, 20.5];
const MEN: Record<string, Vec3> = { 'Red Two': [28.8, 0, 19.9], 'Blue One': [27.3, 0, 20.4], 'Blue Two': [27.6, 0, 19.3] };
const BUNK: Vec3 = [30.5, 0, 11.9], KNOWN: [Vec3, number][] = [[BUNK, 9.5], [[31.0, 0, 15.2], 14.0], [[19.2, 0, 9.6], 14.3]];

export async function standfireCheck(): Promise<boolean> {
  const BREAK = process.env.STANDFIRE_BREAK === '1';
  console.log(`\n=== standfire: the men who stand off a wall charge keep out of the fire of the enemies the team knows of${BREAK ? ' — STANDFIRE_BREAK: the team told of nobody' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const destr0 = DESTRUCTION.on, rnd0 = Math.random;
  const log = console.log, quiet = async <T>(fn: () => Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
  const data = parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'));
  try {
    /** The pass, the four in the yard as the tape had them (Red One down), every defender out of the way: what the team knows is what `know` writes */
    const stage = async () => {
      DESTRUCTION.on = true; WALLCHARGE.on = true;   // (the destruction switch raised as the wall charge's own probes raise it)
      const W: any = await quiet(() => standUp({ level: loadLevel(data), seed: 1 })); const g = W.game; g.godMode = true; g.tune.chaos = 0.5;
      const why = await quiet(async () => startFromQuery(g, '?levelFile=pass.json&roomclear&squad4').why); if (why) throw new Error(`start refused — ${why}`);
      const step = (secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) W.step(1 / 60); };
      const put = (gd: any, p: number[]) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.path = []; gd.pathGoal = null; gd.speed = 0; };
      const O = g.orders, C = g.wallCharge; if (C.carried !== null) C.setCarried(2);   // (the shipped stand's kit: a wall charge is carried or there is none)
      const fog = g.fogOfWar, foes: any[] = g.guards.filter((h: any) => h.team === 'hostile');
      for (const h of foes) put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
      const men: any[] = O.men('all'), one = men.find(m => m.callsign === 'Red One'); if (one) killGuard(g, one, [1, 0, 0], true);
      for (const m of men) if (MEN[m.callsign]) put(m, MEN[m.callsign]);
      g.teleportPlayer(SAM); step(0.5); for (const m of men) if (MEN[m.callsign]) put(m, MEN[m.callsign]);
      const Wl = wallAt(g, CHARGE), spot: Vec3 | null = Wl ? C.setSpot(Wl) : null;
      /** what the team knows: the fog on, and of the living enemies only these — each last seen `age` s ago at his place (he stands 400 m off) */
      const know = (list: [Vec3, number][]) => { fog.on = true; for (const h of foes) fog.recs.delete(h);
        if (!BREAK) list.forEach(([p, age], i) => fog.recs.set(foes[i], { seen: false, t: g.time - age, at: [p[0], 0, p[2]], body: false, smoked: false, smokeF: -1e9 })); };
      const sam = (p: Vec3) => { g.player.char.pos[0] = p[0]; g.player.char.pos[2] = p[2]; };
      /** the three marks the stand-off search deals */
      const deal = (): Vec3[] | null => C.safePosts(Wl, 3, spot);
      const fire = (q: Vec3): number => C.fireAt(q, Wl), words = (P: Vec3[] | null) => P ? P.map(q => `${at(q)} fire ${fire(q)}`).join(' · ') : 'none';
      const clear = (P: Vec3[]) => P.every(q => C.blastShare(Wl, q, undefined, false) === 0);
      return { W, g, O, C, fog, foes, men, Wl, spot, step, put, know, sam, deal, fire, words, clear };
    };
    const max = (S: any, P: Vec3[]): number => Math.max(...P.map(q => S.fire(q) as number)), same = (a: Vec3[] | null, b: Vec3[] | null) => JSON.stringify(a) === JSON.stringify(b);

    const S = await stage();
    if (!S.Wl || !S.Wl.ok || !S.spot) { line(false, 'the bunk room\'s long wall takes a charge from the yard', `wall ${S.Wl ? `${S.Wl.index} ok ${S.Wl.ok} ${S.Wl.why ?? ''}` : 'none'} · spot ${S.spot ? at(S.spot) : 'none'}`); return false; }
    S.know(KNOWN); S.sam(SAM);
    const on = S.deal();
    // (3)
    line(!!on && S.clear(on), 'Sam where he stood: three marks are dealt and every one is out of the blast',
      `charge ${at(wallPt(S.Wl, 0, 0))}, stand-off ${safeRadius(S.g, S.Wl).standoff} m · known at ${S.C.knownFoes().map(at).join(' ')} · marks ${S.words(on)} · worst ${on ? max(S, on) : '–'}`);
    // (4)
    { S.sam(ELSEWHERE); const n2 = S.deal();
      line(!!n2 && n2.every(q => S.fire(n2[0]) <= S.fire(q)) && S.clear(n2), 'Sam elsewhere: the first mark dealt — the setter\'s — is the least under fire of them, and every mark is out of the blast',
        `marks ${S.words(n2)}`); S.sam(SAM); }
    // (6)
    { S.know([[BUNK, WALLCHARGE.fireKnownS - 1]]); const fresh = S.deal(), kA = S.C.knownFoes().length, wA = S.words(fresh);
      S.know([[BUNK, WALLCHARGE.fireKnownS + 1]]); const stale = S.deal(), kB = S.C.knownFoes().length;
      line(kA === 1 && kB === 0 && !!fresh && !!stale && !same(fresh, stale), `a sighting counts ${WALLCHARGE.fireKnownS} s and no longer: the man in the bunk room last seen ${WALLCHARGE.fireKnownS - 1} s ago is stood off from, last seen ${WALLCHARGE.fireKnownS + 1} s ago he is not`,
        `${WALLCHARGE.fireKnownS - 1} s: known ${kA}, marks ${wA} · ${WALLCHARGE.fireKnownS + 1} s: known ${kB}, marks ${stale ? stale.map(at).join(' · ') : 'none'}`); }
    // (7)
    { S.know([]); S.fog.on = false; S.put(S.foes[0], BUNK); const k = S.C.knownFoes(), n5 = S.deal();
      line(!S.fog.active && k.length === S.foes.filter((h: any) => h.state !== 'dead').length && k.some((p: Vec3) => flat(p, BUNK) < 0.01) && !!n5, 'the fog off (the picture shows everybody): a living enemy counts where he stands',
        `known ${k.length} of ${S.foes.length} (the rest stand 400 m off, past fireM), one at ${at(BUNK)} · marks ${S.words(n5)}`);
      S.put(S.foes[0], [BUNK[0] + 400, 0, BUNK[2]]); }
    // (7b) a man the squad is shooting at is in the picture: he counts where he stands, seen or not
    { const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`); const sf0 = FOG.squadFire; FOG.squadFire = true; S.know([]); S.put(S.foes[0], BUNK); const k0 = S.C.knownFoes().length;
      S.fog.firedAt.set(S.foes[0], S.fog.fireClock); const k1 = S.C.knownFoes(), n6 = S.deal();
      line(S.fog.active && k0 === 0 && k1.length === 1 && flat(k1[0], BUNK) < 0.01 && !!n6, 'the fog on, a man nobody has seen: not known — until a teammate\'s aimed round goes at him (he is in the picture then): he counts where he stands',
        `known before ${k0} · after ${k1.map(at).join(' ')} · marks ${S.words(n6)}`);
      S.fog.firedAt.delete(S.foes[0]); FOG.squadFire = sf0; S.put(S.foes[0], [BUNK[0] + 400, 0, BUNK[2]]); }
    // (7c) the wall about to go is no cover where it will open
    { const BEHIND: Vec3 = [wallPt(S.Wl, 0, 0)[0], 0, 15.2]; S.know([[BEHIND, 5]]); const front: Vec3 = wallPt(S.Wl, 0, 2.4), aside: Vec3 = wallPt(S.Wl, -3, 2.4);
      const fWall = S.C.fireAt(front, S.Wl), fNone = S.C.fireAt(front), aWall = S.C.fireAt(aside, S.Wl), n7 = S.deal();
      line(fWall > 0 && fNone === 0 && aWall === 0 && !!n7 && n7.every(q => S.fire(q) === 0), 'a man known behind the wall, square behind the charge: the floor square in front of it is under his fire (the wall is about to go), three metres along it is not — and no mark is dealt in that lane',
        `${at(front)}: ${fWall} with the wall counted as the hole it will be, ${fNone} as it stands · ${at(aside)}: ${aWall} · marks ${S.words(n7)}`); }
    // (8) the order
    { S.know(KNOWN); S.sam(SAM); const want = S.C.safePosts(S.Wl, 3, S.spot, (q: Vec3) => !S.O.reach(S.men.find((m: any) => m.callsign === 'Red Two'), q));
      const why = S.O.give(WALL_ORDER, wallTarget(S.g, CHARGE, { kind: 'floor', at: S.O.freeSpot?.(CHARGE) ?? CHARGE }), 'all'), op = S.C.ops[0];
      const posts: Vec3[] = op ? [...op.post.values()] : [], dealt = !!op && !!want && posts.length === 3 && posts.every(p => want.some((q: Vec3) => flat(p, q) < 0.01));
      const worst = posts.length ? max(S, posts) : -1, marks = S.words(posts);   // (read now: the sightings age as the men walk)
      S.step(14); const there = op ? [...op.post].map(([m, p]: [any, Vec3]) => `${m.callsign} ${m.state === 'dead' ? 'DOWN' : flat(m.char.pos, p).toFixed(1) + ' m off'}`) : [];
      const stood = !!op && [...op.post].every(([m, p]: [any, Vec3]) => m.state !== 'dead' && flat(m.char.pos, p) < 0.8) && S.C.tooClose(S.C.set[0] ?? null).length === 0;
    }
  } finally { console.log = log; DESTRUCTION.on = destr0; Math.random = rnd0; }
  console.log(`=== standfire: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await standfireCheck()) ? 0 : 1);
