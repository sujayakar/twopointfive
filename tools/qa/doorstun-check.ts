// A BURST THAT TAKES A DOOR OFF STUNS THE MEN BEHIND IT (src/game/frag.ts; the maintainer, 10-11, a 40 mm on the compound's guard hut door:
// "40mm on door should maybe stun around the blast?"). Standalone and through probes.ts:
//   bun run tools/qa/doorstun-check.ts          bun run tools/qa/probes.ts doorstun
// The blast judges everybody on the frame it goes off, and the leaf it has just torn off still stands in its frame on that frame: the man straight behind the
// door was 'shielded' and shot back a second later. Headless, PASS / FAIL each; the 40 mm's burst is asked as he40.ts asks it (frag.ts blast() a hand off the
// leaf), on the compound's hut door with the man where the maintainer's replay had him, and on an office door for the player.
//   (1) his door, his man: the leaf is off, the man 2.5 m behind it is 'stunned' — folded for the wound band's seconds, alive, nobody down
//   (2) a stride behind the door, well inside the lethal radius: stunned, never killed (the leaf took the fragments)
//   (4) a burst too far off to take the leaf: it hangs, and he is shielded, as ever
//   (5) the player behind a door somebody else's burst takes off: stunned (his trigger dead for the seconds, a line saying why), never hurt
//   (7) BEFORE the burst: a man reckoning his cover (frag.ts exposedTo — who runs from a grenade, a thrower's 'none of mine in it') counts a door the burst
//       will take as no cover; a door out of its reach is cover as ever
// DOORSTUN_BREAK=1 = every burst is put 4.5 m off its door, too far to take the leaf: the lines are asked all the same and must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
type V3 = [number, number, number];
const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

export async function doorstunCheck(): Promise<boolean> {
  const { standUp } = (await import('./headless.ts')) as any;
  const { FRAG, FRAG_STATS, blast, exposure, exposedTo } = await import(`${ROOT}/src/game/frag.ts`);
  const { HE40 } = await import(`${ROOT}/src/game/he40.ts`);
  const { doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const BREAK = process.env.DOORSTUN_BREAK === '1'; let all = true;
  const was = { frag: JSON.parse(JSON.stringify(FRAG)), rnd: Math.random };
  const line = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`\n=== doorstun: a burst that takes a door off stuns the men behind it${BREAK ? ' (DOORSTUN_BREAK: every burst too far off)' : ''} ===`);
  /** a fresh floor with blasts taking doors off: the compound's room clear (his mission) or the office's plain night */
  const rig = async (compound: boolean) => {
    Math.random = seeded(7);
    const W: any = await standUp(compound ? { level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8'))) } : {});
    const g = W.game; g.tune.chaos = 0.5;
    if (compound) startFromQuery(g, 'levelFile=compound.json&roomclear&squad4');
    g.aiEnabled = false; g.godMode = false;
    const said: string[] = []; { const msg = g.msg.bind(g); g.msg = (t: string) => { said.push(t); msg(t); }; }
    const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } };
    step(10);
    const door = (name: string) => { const d = g.doors.list.find((x: any) => x.def.name === name); d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.place(); return d; };
    const put = (gd: any, p: number[]) => { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; };
    /** the 40 mm's burst on the middle of the leaf's `side` face, `off` metres from it (he40.ts: its own standoff) */
    const shell = (d: any, side: 1 | -1, who: number, off = BREAK ? 4.5 : HE40.standoff + 0.03) => { const pt = doorPt(d, d.def.width / 2, off, side, 1.0); return blast(g, [pt[0], pt[1] - FRAG.lift, pt[2]], { who, name: '40 mm grenade' }); };
    return { W, g, said, step, door, put, shell };
  };
  const words = (rep: any, gd: any, g: any) => `${rep.hits.filter((h: any) => h.id === gd.char.id).map((h: any) => `${h.name} ${h.result} at ${f2(h.d)} m, exposed ${h.exposed}`).join('') || 'not in the report'} · ${gd.state} · folded ${f2(Math.max(0, gd.staggerUntil - g.time))} s · report ${rep.killed} down ${rep.wounded} hit ${rep.stunned} stunned ${rep.shielded} behind cover`;
  const hit = (rep: any, gd: any) => rep.hits.find((h: any) => h.id === gd.char.id);
  /** his door: the compound's guard hut, the burst on its OUTSIDE face (the side the maintainer fired from), the hut's own man inside. Which face is which is
   *  read off the level: the hut's man is the defender staged nearest the door, and the burst goes on the face he is not on. */
  const hut = async (where: 'tape' | 'near', off?: number, fire = true) => {
    const R = await rig(true), g = R.g, d = R.door('hut_w'), mid = doorPt(d, d.def.width / 2, 0, 1, 0);
    const T = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').sort((a: any, b: any) => Math.hypot(a.char.pos[0] - mid[0], a.char.pos[2] - mid[2]) - Math.hypot(b.char.pos[0] - mid[0], b.char.pos[2] - mid[2]))[0];
    const inside: 1 | -1 = g.doors.side(d, T.char.pos), side = -inside as 1 | -1;
    { const s = doorPt(d, d.def.width / 2, 12, side, 0); g.teleportPlayer([s[0], 0, s[2]]); }   // (the player square on to the door from the yard, as he was)
    const p: number[] | null = where === 'tape' ? doorPt(d, d.def.width / 2, 2.4, inside, 0) : doorPt(d, d.def.width / 2, 1.0, inside, 0);   // ('tape' = where the replay had him: 2.5 m straight in from the leaf)
    if (!p) return { R, g, d, T, side, rep: null as any, st0: 0, counted: 0, inView: false };
    for (const o of g.guards) if (o !== T && o.team === 'hostile') R.put(o, [4.8, 0, 5.8]);   // (the rest of the yard: out of this burst)
    R.put(T, p); R.step(3); R.said.length = 0;
    const st0 = T.staggerUntil, n0 = FRAG_STATS.stunned; const rep = fire ? R.shell(d, side, PLAYER_ID, off) : null as any;
    // (does the player have a line to the man through the opening? the log names a man folded behind the door only then)
    const dyn = g.col.dynamicBoxes; g.col.dynamicBoxes = dyn.filter((b: any) => b !== d.box); let inView = false;
    try { inView = !g.col.segmentBlocked(g.player.char.bones.head ?? [g.player.char.pos[0], 1.6, g.player.char.pos[2]], T.char.bones.chest ?? [T.char.pos[0], 1.2, T.char.pos[2]]); } finally { g.col.dynamicBoxes = dyn; }
    return { R, g, d, T, side, rep, st0, counted: FRAG_STATS.stunned - n0, inView };
  };
  try {
    { const c = await hut('tape'), h = c.rep && hit(c.rep, c.T), secs = c.T.staggerUntil - c.g.time, aware = c.T.awareness; c.R.step(1);   // (what the game said reaches the check with the next step)
      line(!!h && c.d.off && h.result === 'stunned' && c.T.state !== 'dead' && secs > 2 && secs <= FRAG.stagger[0] && c.rep.killed === 0 && c.rep.stunned === 1 && c.counted === 1 && aware >= 0.92 && c.inView === c.R.said.some((s: string) => /1 stunned behind the door/.test(s)),
        '(1) his door, his man: the 40 mm takes the hut door off and the man 2.5 m behind it is stunned — folded, alive, and the log says so', c.rep ? `door off ${c.d.off} · ${words(c.rep, c.T, c.g)} · the player ${c.inView ? 'sees' : 'does not see'} him through the opening · counted ${c.counted} · awareness ${f2(aware)} · said "${c.R.said.join(' / ')}"` : 'NO BLAST'); }
    { const c = await hut('near'), h = c.rep && hit(c.rep, c.T);
      line(!!h && c.d.off && h.d < FRAG.lethalM && h.result === 'stunned' && c.T.state !== 'dead' && c.T.staggerUntil - c.g.time > 2.5,
        '(2) a stride behind the door, inside the lethal radius: stunned, never killed — the leaf took the fragments', c.rep ? `door off ${c.d.off} · ${words(c.rep, c.T, c.g)}` : 'NO BLAST'); }
    { const c = await hut('tape', 4.5), h = c.rep && hit(c.rep, c.T);
      line(!!h && !c.d.off && h.result === 'shielded' && c.T.staggerUntil === c.st0,
        '(4) a burst too far off to take the leaf: the door hangs and he is shielded as ever', c.rep ? `door off ${c.d.off} · ${words(c.rep, c.T, c.g)}` : 'NO BLAST'); }
    { const R = await rig(false), g = R.g, d = R.door('manager'), pl = g.player; for (const o of g.guards) R.put(o, [8, 0, 17.6]);
      const p = doorPt(d, d.def.width / 2, 1.8, -1, 0); g.teleportPlayer([p[0], 0, p[2]]); R.step(3); R.said.length = 0;
      const hits0 = pl.hitsLeft; const rep = R.shell(d, 1, 0); const secs = pl.staggerUntil - g.time, cd = pl.fireCd; R.step(1);   // (what the game said reaches the check with the next step)
      line(d.off && rep.player === 'stunned' && rep.stunned === 0 && !pl.down && pl.hitsLeft === hits0 && secs > 2 && cd > 2 && R.said.some((s: string) => /came through the door, stunned/.test(s)),
        '(5) the player behind a door a burst takes off: stunned like anybody — trigger dead for the seconds, not hurt, a line saying why', `door off ${d.off} · player ${rep.player} · down ${pl.down} · hits left ${pl.hitsLeft} of ${hits0} · stagger ${f2(secs)} s · fireCd ${f2(cd)} · "${R.said[0] ?? ''}"`); }
    { const c = await hut('near', undefined, false), g = c.g, c2 = c.T.char, head = [c2.pos[0], 1.65, c2.pos[2]];
      const at = (o: number) => { const pt = doorPt(c.d, c.d.def.width / 2, o, c.side, 1.0); return [pt[0], pt[1], pt[2]] as V3; };
      const near = at(HE40.standoff + 0.03), far = at(4.5);
      const plain = exposure(g, near, c2.pos, head), wary = exposedTo(g, near, c2.pos, head), out = exposedTo(g, far, c2.pos, head);
      line(!c.d.off && plain === 0 && wary > 0 && out === 0,
        '(7) before the burst: a door it will take is no cover to a man reckoning his own; one out of its reach is',
        `behind the shut hut door, a stride in · plain cover test ${plain} · a grenade at the leaf ${wary} · one 4.5 m off ${out}`); }
  } catch (e) { all = false; console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); }
  finally { Object.assign(FRAG, was.frag); Math.random = was.rnd; }
  console.log(`=== doorstun: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await doorstunCheck()) ? 0 : 1);
