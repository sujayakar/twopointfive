// THE LIGHTS GO OUT AND SOMEBODY GOES TO PUT THEM BACK ON — the check behind src/game/powerout.ts (POWEROUT.on: on in the page, ?nopowerout; off on the frozen
// stand, TPS_POWEROUT=1).
//   bun run tools/qa/powerout-check.ts        (also `probes.ts powerout`; run it on BOTH stands: plain, and with TPS_PROFILE=shipped in front)
// A room clear staged as the mission deals it, the team left standing on its start marks, Sam out of everybody's sight, the mains thrown by hand. The switch is
// thrown by the check; so are the dialogue pass and the teammates' work rule (a frozen stand without them hands the stock errand to one of Sam's own men, who
// walks into the house: another check's business). Read on the first of LEVELS where a PAIR goes (the levels are rebuilt under this check: no floor is counted on):
//   (a) a calm garrison: their word is on the net before any man of theirs has left his post, and the first step comes `beat` after it; no more than `men` go;
//       every man who left his post is one the timeline sent; nobody else stirs (the posted men keep their posts); every room that had a man keeps one; the men
//       out file boots for the team's ears and burn their lamps while the mains are down; a man of theirs is at the lever when the mains come back, later than
//       the word by the lever's own fumble; every man out is back on the post he left, holding it as he held it;
//   (b) the lights go a second time: nobody walks out again — one line;
//   (c) the garrison already knows (gunfire written into one man's witness record): nobody leaves, one line, the word is out;
// POWEROUT_BREAK=1 = `beat` thrown to nothing for (a): (a) must fail.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: a lamp here is a light the CPU's estimate counts, a line on the net is a string. Nobody has seen or heard any of this in play.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');
const LEVELS = ['depot.json', 'mousehole.json', 'killhouse.json', 'hall.json', 'barracks.json', 'warehouse.json', 'compound.json', 'records.json'];

function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function poweroutCheck(): Promise<boolean> {
  const { standUp } = await import('./headless.ts');
  const { POWEROUT, powerOutTimeline, powerOutSnapshot, powerOutRestore } = await import(`${ROOT}/src/game/powerout.ts`);
  const { hasHim, OWN } = await import(`${ROOT}/src/game/owner.ts`);
  const { WORDS } = await import(`${ROOT}/src/game/words.ts`), { WORK } = await import(`${ROOT}/src/game/workthings.ts`);
  const { breakerResetNear } = await import(`${ROOT}/src/game/lighting.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const DT = 1 / 60, rnd0 = Math.random, was = { ...POWEROUT }, wordsWas = WORDS.on, workWas = { ...WORK }; let all = true;
  const BREAK = process.env.POWEROUT_BREAK === '1';
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== powerout: the lights go out and somebody goes to put them back on ===');
  const levelOf = (file: string) => loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const foes = (g: any): any[] => g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead');
  /** the switches as this check runs them (a stand-up writes the held ones again: thrown after every one) */
  const throwOn = (on = true, patch: object = {}) => { Object.assign(POWEROUT, was, { on }, patch); WORDS.on = true; Object.assign(WORK, { on: true, posted: true, ownFace: true }); };
  interface Stood { W: any; g: any; men: any[]; post: Map<any, number[]>; room: Map<any, any>; lines: { t: number; s: string }[]; file: string }
  const stand = async (file: string, seed: number, on = true, patch: object = {}): Promise<Stood> => {
    Math.random = seeded(seed); const W: any = await quiet(() => standUp({ uploadEvery: 4, level: levelOf(file) })); const g = W.game;
    g.playerInvisible = true; g.godMode = true; throwOn(on, patch);
    const why = await quiet(async () => g.roomClear.start(1)); if (why) throw new Error(`the room clear did not start on ${file}: ${why}`);
    await quiet(async () => { for (let i = 0; i < 90; i++) W.step(DT); });
    const men = foes(g), lines: Stood['lines'] = [], msg0 = g.msg.bind(g); g.msg = (s: string) => { lines.push({ t: g.time, s }); return msg0(s); };
    return { W, g, men, post: new Map(men.map(gd => [gd, [...gd.char.pos]])), room: new Map(men.map(gd => [gd, g.fight.defs.find((D: any) => D.gd === gd)?.room ?? null])), lines, file };
  };
  const step = (W: any, n = 1) => quiet(async () => { for (let i = 0; i < n; i++) W.step(DT); });
  const off = (S: Stood, gd: any) => flat(gd.char.pos, S.post.get(gd)!);
  const ours = (S: Stood) => S.lines.filter(l => /\(radio\): (power's out|breaker's wrecked|he's down at the breaker|nothing from the breaker)/.test(l.s));
  const TL = (g: any): any[] => powerOutTimeline(g);
  /** run a stood floor until `until` says so or `secs` have gone, with `each` asked after every frame */
  const runFor = async (S: Stood, secs: number, each?: (i: number) => void, until?: () => boolean) => { for (let i = 0; i < secs * 60; i++) { await step(S.W); each?.(i); if (until?.()) break; } };

  try {
    // ---- the floor: the first level where a pair goes
    let pick: { file: string; seed: number } | null = null, own: { file: string; seed: number } | null = null; const tried: string[] = [];
    for (const file of LEVELS) { if (pick && own) break;
      const S = await stand(file, 11); S.g.setBlackout(86400, 'hand'); await runFor(S, 8, undefined, () => TL(S.g).some(e => e.what === 'said' || e.what === 'stay' || e.what === 'nobody'));
      const said = TL(S.g).find(e => e.what === 'said'); tried.push(`${file}: ${said ? said.why.split(',')[0] : TL(S.g).at(-1)?.what ?? 'nothing'}`);
      if (said && / and /.test(said.why) && !pick) pick = { file, seed: 11 };
      if (said && /in his own room/.test(said.why) && !own) own = { file, seed: 11 }; }
    if (!pick) { line(false, 'a level of the list sends a pair to the breaker', tried.join(' · ')); return false; }

    // (a) a calm garrison
    { const S = await stand(pick.file, pick.seed, true, BREAK ? { beat: 0 } : {}), g = S.g, men = S.men; g.setBlackout(86400, 'hand'); const t0 = g.time;
      const calm = new Map(men.map(gd => [gd, { state: gd.state, hold: gd.hold }]));
      let wordT = -1, firstOffT = -1, restoredT = -1, atLever = false, leverPrev = false, emptied = 0, lampOut = 0, outFrames = 0, strayLamp = 0, boots = 0; const left = new Set<any>(), seenEv = new Set<any>();
      await runFor(S, 150, () => {
        if (wordT < 0 && ours(S).length) wordT = ours(S)[0].t;
        for (const gd of men) { const d = off(S, gd), out = hasHim(gd, OWN.breaker);
          if (d > 0.6) { left.add(gd); if (firstOffT < 0) firstOffT = g.time; }
          if (out && d > 1 && g.blackout.active) { outFrames++; if (gd.flashlight.enabled) lampOut++; }
          if (!out && !left.has(gd) && gd.flashlight.enabled) strayLamp++; }
        for (const e of g.events) if (e.kind === 'step' && !seenEv.has(e)) { seenEv.add(e); if (men.some(gd => gd.char.id === e.who && hasHim(gd, OWN.breaker))) boots++; }
        for (const R of new Set(S.room.values())) { if (!R) continue; const had = men.filter(gd => S.room.get(gd) === R); if (had.length && !had.some(gd => off(S, gd) <= 0.6)) emptied++; }
        if (g.blackout.active) leverPrev = men.some(gd => breakerResetNear(g, gd));   // (lighting.ts's own gate for the re-arm, asked while the mains are still down)
        else if (restoredT < 0) { restoredT = g.time; atLever = leverPrev; }
      }, () => restoredT >= 0 && g.time - restoredT > 2 && !men.some(gd => hasHim(gd, OWN.breaker)));
      const tl = TL(g), sent = tl.filter(e => e.what === 'go').map(e => e.who), homes = tl.filter(e => e.what === 'home').map(e => e.who), goT = tl.find(e => e.what === 'go')?.t ?? -1;
      const strangers = [...left].filter(gd => !sent.includes(gd.callsign)).map(gd => gd.callsign);
      const back = [...left].every(gd => off(S, gd) <= 0.6 && gd.hold === calm.get(gd)!.hold && gd.state === calm.get(gd)!.state && g.fight.defs.find((D: any) => D.gd === gd)?.mode === 'hold');
      line(wordT >= 0 && firstOffT > wordT && goT - wordT >= was.beat - 2 * DT, 'their word is on the net before any man of theirs has left his post, and the first step comes the beat after it',
        `${pick.file} · the lights go at 0 · "${ours(S)[0]?.s ?? '—'}" at ${(wordT - t0).toFixed(1)} s · the men step off at ${(goT - t0).toFixed(1)} s · the first man 0.6 m off his post at ${(firstOffT - t0).toFixed(1)} s`);
      line(sent.length >= 2 && sent.length <= was.men && strangers.length === 0 && left.size === sent.length && emptied === 0, 'no more than the rule\'s men go, every man who left his post is one it sent, and every room that had a man keeps one',
        `sent ${sent.join(', ')} · left their posts ${[...left].map(gd => gd.callsign).join(', ')} of ${men.length}${strangers.length ? ` · NOT SENT: ${strangers.join(', ')}` : ''} · frames a room stood empty ${emptied}`);
      line(boots >= 4 && outFrames > 30 && lampOut === outFrames && strayLamp === 0, 'the men out file their boots for the team\'s ears and burn their lamps while the mains are down; nobody else\'s lamp burns',
        `${boots} boots filed · ${lampOut} of ${outFrames} frames out in the dark with the lamp burning · lamp frames of men who stayed ${strayLamp}`);
      line(restoredT >= 0 && atLever && restoredT - wordT >= 2.2, 'the mains come back with a man of theirs at the lever, later than the word by the lever\'s own fumble',
        `mains back ${(restoredT - t0).toFixed(1)} s after they went, ${(restoredT - wordT).toFixed(1)} s after the word · a defender at the panel ${atLever}`);
      line(homes.length === sent.length && back && !men.some(gd => hasHim(gd, OWN.breaker)), 'every man out is back on the post he left, holding it as he held it',
        `home ${homes.join(', ') || 'nobody'} · ${[...left].map(gd => `${gd.callsign} ${off(S, gd).toFixed(1)} m off, ${gd.state}`).join(' · ')}`);

      // (b) the lights go a second time (the same floor)
      const n0 = ours(S).length; g.setBlackout(86400, 'hand'); let moved = 0;
      await runFor(S, 12, () => { for (const gd of men) if (off(S, gd) > 0.6) moved++; });
      const again = ours(S).slice(n0);
      line(again.length === 1 && /no fault|no accident/.test(again[0].s) && moved === 0 && g.blackout.active, 'the lights go a second time: nobody walks out again — one line, and the word is out',
        `"${again.map(l => l.s).join(' | ') || '—'}" · frames a man was off his post ${moved} · mains still down ${g.blackout.active}`); }

  } finally { Object.assign(POWEROUT, was); WORDS.on = wordsWas; Object.assign(WORK, workWas); Math.random = rnd0; }
  console.log(`=== powerout: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await poweroutCheck()) ? 0 : 1);
