// 'words-overheard' (the Rust game's cues::cues_overheard), run through probes.ts:   bun run tools/qa/probes.ts words-overheard
// A defender the fog hides who SHOUTS a line (not on his net) where the player's ears or a living teammate's really get it is a quiet row of its own on the
// radio strip: his words alone. PASS / FAIL each, all headless: no picture and no sound here — 'drawn' is the strip's own row list (game.view.radio), not a screen.
//   2 · the team met on levels/block-squads.json under the fog: every hidden defender shouts in turn — a row exactly where the game's own hearing law
//       (hearPath, the path under the reach × its carry) says an ear gets it, from where the player stands and with his ear brought beside the man;
//       and the questions the row asks leave nothing in the route cache the AI's own hearing reads
//   3 · a hidden defender made to shout beside the player through the game's own say(): the row is on the strip
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { CUES, isShout } = await import(`${ROOT}/src/game/cues.ts`);
const { stripRows } = await import('./heard.ts');
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { hearPath } = await import(`${ROOT}/src/game/weather.ts`);
const { isHostile } = await import(`${ROOT}/src/game/teams.ts`);
const { DIGIN } = await import(`${ROOT}/src/game/digin.ts`);

const BREAK = process.env.OVERHEARD_BREAK === '1';
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
async function rig(seed: number) {
  Math.random = seeded(seed);
  const log = console.log, err = console.error, warn = console.warn; let W: any;
  try { console.log = console.error = console.warn = () => { };
    W = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'block-squads.json'), 'utf8'))) });
    await startFromQuery(W.game, new URLSearchParams(`?levelFile=block-squads.json&roomclear=${seed}&squad4`));
  } finally { console.log = log; console.error = err; console.warn = warn; }
  return W;
}

export async function wordsOverheard(): Promise<boolean> {
  const F = family("words-overheard: a hidden defender's shout that reaches an ear is a quiet row on the strip, his words alone");
  const was = { ...CUES }, rnd = Math.random, stoodTop = CUES.overheard;   // (the shipped stand writes its switches once, at load: what it stood on is read here, before the check writes any)
  try {
    // ---- 2 · the real floor, the game's own hearing law
    const earsOf = (g: any) => { const at = (p: number[]) => [p[0], p[1] + 1.6, p[2]]; const out: number[][] = []; if (!g.player.down) out.push(at(g.player.char.pos)); for (const o of g.guards) if (o.team === 'operative' && o.state !== 'dead' && o.char.alive) out.push(at(o.char.pos)); return out; };
    let routed = 0;   // (how many of the law's questions went round walls: the ones a kept answer would have left in the route cache)
    const law = (g: any, gd: any) => { const p = gd.char.pos, m = [p[0], p[1] + 1.2, p[2]]; return earsOf(g).some(e => { if (Math.hypot(m[0] - e[0], m[2] - e[2]) > DIGIN.shoutM) return false; const h = hearPath(g, m, e, false); if (h.routed) routed++; return h.pathLen < DIGIN.shoutM * h.carry; }); };
    const ROUND = [[2, 0], [-2, 0], [0, 2], [0, -2], [1.5, 1.5], [-1.5, 1.5], [1.5, -1.5], [-1.5, -1.5]];
    const stagePlayer = (W: any, g: any) => { for (let i = 0; i < 60; i++) { g.player.visibility = 0.05; W.step(DT); }   // (cues-run's own staging: the player stood where the first squad's first posted man is aiming, read off the level's marks)
      const rc = g.level.scenarios?.roomClear, aim = rc?.rooms?.find((r: any) => r.id === rc.squads?.[0]?.rooms?.[0])?.marks?.[0]?.aimAt;
      if (!aim) throw new Error('words-overheard: block-squads.json has no first squad room with a mark to stage the player in front of');
      g.teleportPlayer([aim[0], 0, aim[2]]); };
    {
      const W = await rig(2), g = W.game; g.godMode = true; CUES.on = true; CUES.overheard = !BREAK; stagePlayer(W, g);
      let near = 0, farN = 0, seenRow = 0, frames = 0, named = 0, grew = 0; const wrong: string[] = []; const cache = (g.col.sound as any).cache as Map<string, unknown>; routed = 0;
      const ask = (gd: any, hidden: boolean, i: number) => {
        const want = hidden && law(g, gd);
        const C = g.cues as any, keep = { rows: C.rows, row: C.heardRow, said: C.heardSaid, shoutT: C.shoutT, log: C.heardLog.length, stats: JSON.stringify(C.stats) };
        C.rows = []; C.heardRow = null; C.heardSaid = new Map(); C.shoutT = undefined;   // (a bare strip for the question; put back after it — the run itself is never told)
        const c0 = cache.size; const drew = g.cues.overheard(gd, 'over here!'); grew += cache.size - c0; const row = (stripRows(g) as { text: string }[]).map(r => r.text);
        if (drew !== want) wrong.push(`${gd.callsign} @${(i / 60).toFixed(1)}s hidden ${hidden} law ${want} drew ${drew}`);
        if (drew) { if (row.join('|') !== 'heard: over here!') wrong.push(`${gd.callsign}: [${row.join(' | ')}]`); if (row.some(t => t.includes(gd.callsign))) named++; }
        if (!hidden) seenRow += drew ? 1 : 0; else if (want) near++; else farN++;
        C.rows = keep.rows; C.heardRow = keep.row; C.heardSaid = keep.said; C.shoutT = keep.shoutT; C.heardLog.length = keep.log; C.stats = JSON.parse(keep.stats);
      };
      for (let i = 0; i < 60 * 12; i++) {
        g.player.visibility = 1; W.step(DT); frames++;
        if (i % 30) continue;
        for (const gd of g.guards as any[]) {
          if (gd.state === 'dead' || !isHostile(gd.team, 'player')) continue;
          const hidden = g.fogOfWar.hidden(gd); ask(gd, hidden, i);   // where the player stands
          if (!hidden) continue;
          const pp = g.player.char.pos, was = [pp[0], pp[1], pp[2]];   // …and with his ear brought two metres from the man, eight ways round (walls between or not: the law says)
          for (const [dx, dz] of ROUND) { pp[0] = gd.char.pos[0] + dx; pp[2] = gd.char.pos[2] + dz; ask(gd, true, i); }
          pp[0] = was[0]; pp[1] = was[1]; pp[2] = was[2];
        }
      }
      F.verdict(g.fogOfWar.on === true && near >= 20 && farN >= 20 && !wrong.length && seenRow === 0 && named === 0, 'the block under the fog: each defender shouts in turn, from where the player stands and with his ear brought beside the man — a row exactly where a path under the reach × its carry gets to an ear of the team\'s, none where it does not, none for a man in sight, never his name', `${frames} frames · ${near} shouts in earshot drew · ${farN} out of it drew none · ${seenRow} rows for a man in sight · wrong: ${wrong.length ? wrong.slice(0, 4).join(' ; ') : 'none'}`);
      F.verdict(routed >= 20 && grew === 0, 'the row\'s own questions leave nothing behind: with that many of them going round walls, the route cache the AI\'s hearing reads did not grow by one entry across them', `${routed} of the law's questions went round walls · the cache grew by ${grew} across every question the row asked`);
    }

    // ---- 3 · the say() road
    {
      const out: { moves: string; shouts: number; drawn: number; row: boolean; who: string; ring: string }[] = [];
      for (const on of [!BREAK]) {
        const W = await rig(2), g = W.game; g.godMode = true; CUES.on = true; CUES.overheard = on;   // (after the stand-up: on the frozen stand a stand-up writes every held switch down again)
        stagePlayer(W, g); let moves = '', row = false, who = '', ring = '';
        for (let i = 0; i < 60 * 30; i++) { g.player.visibility = 1; W.step(DT);
          if (i === 600) {   // ten seconds in: the player is stood two metres from a hidden defender who then shouts through the game's own say()
            pick: for (const gd of g.guards as any[]) { if (gd.state === 'dead' || !isHostile(gd.team, 'player') || !g.fogOfWar.hidden(gd)) continue;
              for (const [dx, dz] of ROUND) { g.teleportPlayer([gd.char.pos[0] + dx, 0, gd.char.pos[2] + dz]);   // (the teleport stands him on free floor: the law is asked from where he ended up — the same tries, in the same order, switch down or up)
                if (law(g, gd)) { g.cues.rows = []; g.say(gd, 'over here!'); who = gd.callsign; ring = g.messages[g.messages.length - 1]?.text ?? ''; break pick; } } } }
          if (i % 30 === 0) moves += hashDigest(digestGame(g, true) as any) + ' ';
          if (i >= 600 && i < 600 + 60 * 3) for (const r of stripRows(g) as { text: string }[]) if (r.text === 'heard: over here!') row = true; }
        out.push({ moves, shouts: g.cues.stats.shouts, drawn: g.cues.stats.shoutsDrawn, row, who, ring });
      }
      const [b] = out;
      F.verdict(b.row === true && b.drawn >= 1 && b.ring === `${b.who}: over here!`, 'up: his shout through say() is on the strip as a heard row — and the message ring nobody draws has the same line under his name, as it always had', `row ${b.row} · ${b.shouts} hidden shouts offered, ${b.drawn} drawn · the ring: '${b.ring}'`);
    }
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { Object.assign(CUES, was); Math.random = rnd; }
  return F.done();
}
