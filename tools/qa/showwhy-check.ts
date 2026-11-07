// A TEAMMATE SHOWS WHY HE IS NOT ACTING (src/game/showwhy.ts SHOWWHY / showWhyTick; the wave is character.ts signalLayer 'aside') — the check.
//
//   bun run tools/qa/showwhy-check.ts [--verbose] [--only <case>]      exit 0 = all pass, 1 = a FAIL line
//   SHOWWHY_BREAK=1 …   = no defender is left on the floor where a case needs one: `line` must fail
//   bun run tools/qa/probes.ts showwhy
// 2026-11-07: the rule is settled (on), and a write of its other side is refused. Gone with that: the stand's word set before every rig and the switch
// written each step, `off` (the stack and the line staged with the switch off), `same` (those runs on against off by digest), `old` (a recording from
// before the key), and the digest half of `plain`.
//
// The harness has no GPU: what is proven HERE is what the game asks of each man's hand and puts over his head, and where the animator then puts the wrist —
// never a gesture seen. A squad of four in the warehouse, its defenders taken off the floor unless a line needs one:
//   plain   fallen in behind a walking player, then sent to a spot and stood there: nobody signals, nobody says a line of this file's, every man reads
//           'nothing holds him'
//   line    a defender the front man has his gun on, the player stood between them: that man — and not the three in file behind him — waves him off ('aside'), "you're in my line." is over him and still
//           over him seconds later having been said once, he waves again — and with the player out of the line the words go and nothing more is shown
//   quiet   … the same with the man unaware and the squad's fire held (nobody has found them): no wave, no line — no round was wanted
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';

const DT = 1 / 60, LINE_M = Number(process.env.LINE_M ?? 6);   // (how far off the defender of the 'line' case stands)
type V = [number, number, number];

export async function showWhyCheck(o: { verbose?: boolean; only?: string } = {}): Promise<boolean> {
  const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { SHOWWHY_LINES, showWhyOf, showWhyLog, showWhyStats } = await import(`${ROOT}/src/game/showwhy.ts`);
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const BREAK = process.env.SHOWWHY_BREAK === '1';
  const MINE = new Set<string>(Object.values(SHOWWHY_LINES) as string[]);
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const want = (name: string) => !o.only || o.only === name;
  const was = { rnd: Math.random };
  const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const d3 = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
  function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
  const E = (ALL_MISSIONS as any[]).find(x => x.id === 'warehouse-four');
  const levelOf = (): any => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', E.level), 'utf8')); return loadLevel(data); };
  const put = (m: any, p: ArrayLike<number>, yaw: number) => { m.char.pos = [p[0], 0, p[2]]; m.char.vel = [0, 0, 0]; m.speed = 0; m.char.bodyYaw = m.char.aimYaw = yaw; m.path = []; m.pathGoal = null; m.char.update(0); };

  /** the warehouse stood up with its four; `keepFoe`: one defender is left alive, whole and held (SHOWWHY_BREAK=1: never) */
  async function rig(keepFoe = false) {
    if (BREAK) keepFoe = false;
        Math.random = seeded(0x5709 + 7919);
    const W: any = await quiet(() => standUp({ uploadEvery: 4, level: levelOf() })); const g = W.game;
    g.godMode = true; g.playerInvisible = true;
    const { why } = await quiet(() => startFromQuery(g, new URLSearchParams(scenarioQuery(E) as string))); if (why) throw new Error(`start refused — ${why}`);
    const sig = new Map<any, number>(); let sigFrames = 0;
    const step = (n = 1, each?: () => void) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); for (const m of men) if (m.char.anim.signalling) { sigFrames++; sig.set(m, (sig.get(m) ?? 0) + 1); } each?.(); } };
    let men: any[] = [];
    step(20);
    const O = g.orders; men = O.squad().filter((x: any) => x.state !== 'dead'); for (const m of men) m.god = true;
    let foe: any = null;
    for (const x of g.guards) if ((x.team === 'hostile' || x.team === 'guard') && x.state !== 'dead') { if (keepFoe && !foe && x.team === 'hostile') { foe = x; x.god = true; continue; } killGuard(g, x, [0, 0, 1], true); }
    step(30);
    const notes = () => showWhyLog(g) as { t: number; who: string; why: string; signal: string | null; line: string | null }[];
    const said = (m: any) => { const b = m.bubble; return b && g.time - b.t <= b.dur ? b.text as string : null; };
    return { W, g, O, men, foe, step, notes, said, sigFrames: () => sigFrames, sig };
  }
  type Rig = Awaited<ReturnType<typeof rig>>;

  /** the longest straight run of free floor from `p` (16 ways), at least `need` long: its far end, or null */
  function runway(g: any, p: ArrayLike<number>, need: number): V | null {
    let best: V | null = null, bl = 0;
    for (let k = 0; k < 16; k++) { const a = k * Math.PI / 8, ux = Math.sin(a), uz = Math.cos(a); let L = 0;
      for (; L < 14; L += 0.25) { const q: V = [p[0] + ux * (L + 0.25), 0, p[2] + uz * (L + 0.25)]; if (g.col.nav.isBlocked(q[0], q[2]) || g.col.solidBlocked([p[0], 0.45, p[2]], [q[0], 0.45, q[2]]) || g.doors.list.some((d: any) => flat(q, d.frameCentre) < 1.2)) break; }
      if (L > bl) { bl = L; best = [p[0] + ux * L, 0, p[2] + uz * L]; } }
    return bl >= need ? best : null;
  }

  try {
    // ---- plain: nothing shown
    if (want('plain')) {
      const play = async () => {
        const A = await rig(), g = A.g, s0: V = [...g.player.char.pos] as V, far = runway(g, s0, 5); if (!far) throw new Error('plain: no free run of floor from the start');
        const refused = A.O.give('fallIn', { kind: 'none', at: [s0[0], 0, s0[2]] }, 'all'); A.step(60);
        let read = 0; const each = () => { for (const m of A.men) if (showWhyOf(g, m)) read++; };
        const L = flat(s0, far), n = Math.floor(L / 0.03); for (let i = 1; i <= n; i++) { g.teleportPlayer([s0[0] + (far[0] - s0[0]) * i / n, 0, s0[2] + (far[2] - s0[2]) * i / n]); A.step(1, each); }   // a walk at 1.8 m/s
        A.step(60, each);
        const sent = A.O.give('move', A.O.targetAt(s0), 'all'); A.step(8 * 60, each);
        return { A, refused, sent, read, lines: A.men.filter(m => MINE.has(m.bubble?.text ?? '')).length };
      };
      const X = await play(), T = showWhyStats(X.A.g);
      verdict(!X.refused && X.A.notes().length === 0 && X.read === 0 && X.lines === 0 && X.A.sigFrames() === 0,
        'plain — fallen in behind a walking player, then sent to a spot: nobody signals or says a line of this file\'s',
        `fall in ${X.refused ? `refused (${X.refused})` : 'given'}, move ${X.sent ? `refused (${X.sent})` : 'given'} · ${X.A.notes().length} notes, ${X.read} frames a man read as held, ${X.A.sigFrames()} frames a hand was signalling, ${T.lines} lines`);
    }

    // ---- line, quiet
    const linePlay = async (aware: boolean) => {
      const A = await rig(true), g = A.g, h = A.foe; if (!h) throw new Error('line: no defender to stand in front of a man');
      const m = A.men[0], p0: V = [...m.char.pos] as V, end = runway(g, p0, 7); if (!end) throw new Error('line: no free run of floor in front of a man');
      const yaw = Math.atan2(end[0] - p0[0], end[2] - p0[2]), far: V = [p0[0] + Math.sin(yaw) * LINE_M, 0, p0[2] + Math.cos(yaw) * LINE_M], back = yaw + Math.PI, mid: V = [(p0[0] + far[0]) / 2, 0, (p0[2] + far[2]) / 2];
      A.men.forEach((x, i) => { if (i > 0) put(x, [p0[0] - Math.sin(yaw) * (1.2 * i), 0, p0[2] - Math.cos(yaw) * (1.2 * i)], yaw); });   // the others in file behind him: he is the gun in front
      put(m, p0, yaw); const refused = A.O.give('cover', A.O.targetAt(far), 'all');   // ('cover that way': each from where he stands, his eyes down the run)
      const pin = () => { put(h, far, aware ? back : yaw); h.hold = true; if (aware) { h.state = 'alert'; h.awareness = 1; } else { h.state = 'patrol'; h.awareness = 0; } h.weapon && (h.weapon.cooldown = 9); h.visibility = 1; };   // (he stands in light: the frozen stand's teammates have no goggles to find him in the warehouse's dark)
      let on1 = 0, gap = 9, restGap = 0, sigM = 0, read = 0, rounds = 0, early = 0;
      let lat = 0; const each = () => { const c = m.char, a = c.bones.handL, b = c.bones.handR; if (a && b) gap = Math.max(gap === 9 ? 0 : gap, d3(a, b)); if (a) lat = Math.max(lat, Math.abs((a[0] - c.pos[0]) * Math.cos(c.bodyYaw) - (a[2] - c.pos[2]) * Math.sin(c.bodyYaw))); if (c.anim.signalling) sigM++; if (showWhyOf(g, m) === 'line') read++; if (A.men.some(x => A.said(x) === SHOWWHY_LINES.line)) on1++; };   // (one voice: whichever of the masked men has it up)
      { const off: V = [p0[0] - Math.sin(yaw) * 3 + Math.cos(yaw) * 2, 0, p0[2] - Math.cos(yaw) * 3 - Math.sin(yaw) * 2]; g.teleportPlayer(g.col.nav.isBlocked(off[0], off[2]) ? [p0[0] - Math.sin(yaw) * 5, 0, p0[2] - Math.cos(yaw) * 5] : off); }   // (the player behind them, out of every line, while they find him)
      for (let i = 0; i < 60; i++) { pin(); A.step(1, () => { if (A.men.some(x => A.said(x) === SHOWWHY_LINES.line)) early++; }); } { const c = m.char; if (c.bones.handL && c.bones.handR) restGap = d3(c.bones.handL, c.bones.handR); } gap = 9;
      const shots0 = g.fight.stats.opRounds;
      for (let i = 0; i < (aware ? 7 : 4) * 60; i++) { pin(); g.teleportPlayer(mid); A.step(1, each); }
      rounds = g.fight.stats.opRounds - shots0;
      const all = A.notes().filter(n => n.why === 'line'), N = all.filter(n => n.who === m.callsign), still = A.men.some(x => A.said(x) === SHOWWHY_LINES.line), voices = all.filter(n => n.line).map(n => n.who), wavers = new Set(all.filter(n => n.signal === 'aside').map(n => n.who)).size, tgt = g.fight.opOf(m)?.tgt === h, lineT = g.fight.opOf(m)?.lineT ?? 0;
      if (!aware) return { A, m, refused, lat, early, free: 0, N, still, on1, gap, restGap, sigM, read, rounds, tgt, lineT, gone: -1, later: 0, voices, wavers, all: all.length };   // (the quiet line asks nothing of the player stepping out)
      const aside: V = [mid[0] + Math.cos(yaw) * 3.5, 0, mid[2] - Math.sin(yaw) * 3.5]; g.teleportPlayer(g.col.nav.isBlocked(aside[0], aside[2]) ? [mid[0] - Math.cos(yaw) * 3.5, 0, mid[2] + Math.sin(yaw) * 3.5] : aside);
      h.god = true; let gone = -1, n0 = A.notes().length; const shots1 = g.fight.stats.opRounds; for (let i = 0; i < 4 * 60; i++) { pin(); A.step(); if (gone < 0 && !A.men.some(x => A.said(x) === SHOWWHY_LINES.line || showWhyOf(g, x) === 'line')) gone = i / 60; }
      return { A, m, refused, lat, early, free: g.fight.stats.opRounds - shots1, N, still, on1, gap, restGap, sigM, read, rounds, tgt, lineT, gone, later: A.notes().slice(n0).filter(n => n.why === 'line').length, voices, wavers, all: all.length };
    };
    if (want('line')) {
      const L = await linePlay(true), waves = L.N.filter(n => n.signal === 'aside').length, lines = L.voices.length;
      verdict(L.tgt && L.early === 0 && waves >= 2 && L.wavers === 1 && lines >= 1 && lines <= 2 && L.voices.every(v => v === L.m.callsign) && L.still && L.on1 > 240 && L.gap - L.restGap > 0.1 && L.lat > 0.45 && L.sigM > 60 && L.read > 240 && L.gone >= 0 && L.gone < 2.5 && L.later === 0,
        'line — the player between the squad and the defender the front man\'s gun is on: that man alone waves him off, "you\'re in my line." stands while it is true, and goes when it is not',
        `his gun on the defender: ${L.tgt} (round held ${L.lineT.toFixed(1)} s, ${L.rounds} rounds from the team meanwhile) · ${L.early} frames the line was up before the player stepped in · ${waves} waves from him, ${L.wavers} of the four waved (three stand in file behind him: they wait on him) · the line said ${lines}× (${[...new Set(L.voices)].join(', ') || 'nobody'}), up ${L.on1} of 420 frames and still at the end: ${L.still} · his hands ${L.restGap.toFixed(2)} m apart at rest, ${L.gap.toFixed(2)} m at the widest of a wave, the wrist ${L.lat.toFixed(2)} m out to the side of his spine (${L.sigM} frames signalling) · read as 'line' ${L.read} frames · the player out of the line: gone in ${L.gone < 0 ? 'never' : L.gone.toFixed(2) + ' s'}, ${L.later} notes in the four seconds after (${L.free} rounds from the team in them)`);
    }
    if (want('quiet')) {
      const L = await linePlay(false);
      verdict(L.tgt && L.lineT > 0 && L.all === 0 && L.on1 === 0 && L.read === 0,
        'quiet — the same with the defender unaware and the squad\'s fire held: no wave and no line (no round was wanted, so none is masked)',
        `his gun on the defender: ${L.tgt} (round held ${L.lineT.toFixed(1)} s, ${L.rounds} rounds) · ${L.all} notes, the line up ${L.on1} frames, read as 'line' ${L.read} frames`);
    }

  } finally { Math.random = was.rnd; }
  console.log(`\nshowwhy-check: ${checks - fails}/${checks} pass${BREAK ? ' (SHOWWHY_BREAK=1: no defender kept — this run must fail)' : ''}`);
  return fails === 0;
}

if (import.meta.main) { const a = process.argv; const ok = await showWhyCheck({ verbose: a.includes('--verbose'), only: a.includes('--only') ? a[a.indexOf('--only') + 1] : undefined }); process.exit(ok ? 0 : 1); }
