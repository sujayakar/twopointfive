// A stack's file with no room to pass in it fills from its far end (game/orders.ts ORDERS.fileQueue, fileCross).
//   bun run tools/qa/filequeue-check.ts [--verbose]      (probes.ts filequeue, in family squadorders; FILEQUEUE_BREAK=1 must FAIL)
// Found on compound-four as shipped, in a replay of the maintainer's: the team inside the garage, 'stack up' on its forecourt door. The marks run along
// the wall into the lane between the parked car and the wall, a man wide. The deal that walks the least put the nearest man on the mark by the door: he
// stood in the mouth of the lane, and the two bound for the marks beyond him pushed him down it — twenty seconds of three men edging along a wall, and
// again when 'clear from outside' moved every mark a step.
//   1. THE FOUND CASE, staged the same way on either stand: the four where the replay had them, Sam inside the garage, 'stack up' on that door, 30 s.
//      Rule off: a man is dealt a mark beyond a mate who is ahead of him, and a man who had his mark is pushed off it. Rule on: the lane's marks go to the
//      men in the order they come to it (the far one to the nearest man), nobody is pushed off a mark he had, one pair still holds the two marks nearest
//      the door, and all four are on their marks sooner than with the rule off.
//   2. … AND THE NEXT ORDER: 'clear from outside' given 5.2 s after the stack, as he gave it (every mark moves a step). Rule on: nobody pushed, everybody
//      set sooner than with the rule off.
//   3. EVERY OTHER DOOR: on three squad levels, every door and face that has a stack for four, the men where the level starts them: some have a tight
//      file, most have none, and where none is tight the deal is the very same with the rule on (nothing is priced).
//   4. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The doorway manners are raised for the staging on either stand (the frozen stand holds them off; the page has them), and the stack is dealt the face the
// men are on (ORDERS.knownFace off: the staged team has not 'been in' the garage it is put in).
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DOORMANNERS } = await import(`${ROOT}/src/game/doorway.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const fmt = (p: number[]) => `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`;

/** where the replay had the four when the order came (x, z), and the mouth of the lane between the car and the wall */
const AT: Record<string, number[]> = { 'Red One': [5.09, 19.32], 'Red Two': [5.99, 19.88], 'Blue One': [6.83, 20.06], 'Blue Two': [5.26, 20.11] }, MOUTH = [9.85, 0, 21.0];
interface Case { why: string | null; why2: string | null; deal: string; behind: number; pushed: number; allT: number; nearPair: boolean }
/** 1 and 2: the garage stack — `behind`: men dealt a lane mark beyond that of a mate who starts nearer the lane · `pushed`: frames a man stands more than
 *  0.45 m off a mark he had reached (a mark his order moved while he stood on it is a new mark) · `allT`: seconds until all four are on their marks and stay (30 = never) */
async function garage(on: boolean, thr: boolean): Promise<Case> {
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; DOORMANNERS.on = true; ORDERS.knownFace = false; ORDERS.fileQueue = on;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const men: any[] = g.guards.filter((x: any) => AT[x.callsign]), d = g.doors.list.find((x: any) => x.def.name === 'g_e');
  g.teleportPlayer([4.6, 0, 20.0]); for (const m of men) R.put(m, [AT[m.callsign][0], 0, AT[m.callsign][1]]); R.step(5);
  const T = O.targetAt(d.frameCentre), why = O.give('stack', T); R.step(1);
  // the deal, as dealt: the men on the wall's file (the marks past the door along the wall the lane runs by), nearest the lane first
  const file = men.filter(m => O.of(m)?.post && O.of(m).post[2] > 20.8 && O.of(m).post[0] > 9.5).sort((a, b) => flat(a.char.pos, MOUTH) - flat(b.char.pos, MOUTH));
  let behind = 0; for (let i = 0; i < file.length; i++) for (let j = i + 1; j < file.length; j++) if (O.of(file[j]).post[2] > O.of(file[i]).post[2] + 0.3) behind++;
  const deal = men.map(m => `${m.callsign} ${O.of(m)?.post ? fmt(O.of(m).post) : '—'}`).join(' · ');
  const byZ = [...file].sort((a, b) => O.of(a).post[2] - O.of(b).post[2]), nearPair = byZ.length >= 2 && byZ[0].callsign.split(' ')[0] === byZ[1].callsign.split(' ')[0];
  const had = new Map<any, number[]>(); let pushed = 0, allT = 30, ok = -1, why2: string | null = null;
  for (let f = 1; f <= 30 * 60; f++) {
    R.step(1);
    if (thr && f === 312) { why2 = O.give('thresholdHold', T); had.clear(); ok = -1; allT = 30; }
    let n = 0; for (const m of men) { const a = O.of(m); if (!a?.post) continue; { const h0 = had.get(m); if (h0 && flat(a.post, h0) > 0.1 && flat(m.char.pos, h0) <= 0.45) had.delete(m); }   // (his order gave him another mark while he stood on this one: a step he was told to take, not a push)
      if (a.arrived) { n++; if (!had.has(m)) had.set(m, [a.post[0], 0, a.post[2]]); } const h = had.get(m); if (h && flat(m.char.pos, h) > 0.45) pushed++; }
    if (n === men.length) { if (ok < 0) { ok = f; allT = f / 60; } } else { ok = -1; allT = 30; }
  }
  if (thr && allT < 30) allT -= 312 / 60;
  return { why, why2, deal, behind, pushed, allT, nearPair };
}

/** 3: every door and face of `scn` with a stack for the four where they start — [asked, tight, changed where not tight] */
async function doors(scn: string): Promise<[number, number, number]> {
  const R = await rig(scn, 5), g = R.game, O = R.O as any, men: any[] = O.squad().filter((x: any) => x.state !== 'dead'); let asked = 0, tight = 0, changed = 0;
  for (const d of g.doors.list) for (const side of [1, -1]) {
    const posts = O.stackPosts(d, side, men); if (!posts || posts.length < men.length) continue; asked++;
    const spots = posts.map((p: any) => p.pos), groups = posts.map((p: any) => p.group ?? 0), key = (x: [any, number[]][]) => x.map(([m, p]) => `${m.callsign}${fmt(p)}`).join('|');
    ORDERS.fileQueue = false; const a = key(O.pair(men, spots, groups, d.frameCentre));
    ORDERS.fileQueue = true; const b = key(O.pair(men, spots, groups, d.frameCentre)), t = !!O.fileCross(men, spots);
    if (t) tight++; else if (a !== b) changed++;
  }
  return [asked, tight, changed];
}

export const filequeueCheck = guarded(async () => {
  const F = family('filequeue: a stack — a file with no room to pass fills from its far end');
  const keep = { m: DOORMANNERS.on, q: ORDERS.fileQueue, k: ORDERS.knownFace, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1 and 2. the found case, and the next order
    for (const thr of [false, true]) {
      const off = await garage(false, thr), on = await garage(true, thr), how = thr ? "'stack up', then 'clear from outside' 5.2 s on" : "'stack up' on the garage's forecourt door, from inside";
      const say = (c: Case) => `${c.why ? `refused: ${c.why} · ` : ''}${c.why2 ? `'clear from outside' refused: ${c.why2} · ` : ''}${c.behind} dealt beyond a mate ahead of him · ${c.pushed} frames a man stood off a mark he had · all set ${c.allT >= 30 ? 'never in 30 s' : `after ${c.allT.toFixed(1)} s`}`;
      if (VERBOSE) { console.log(`  off: ${off.deal}`); console.log(`  on:  ${on.deal}`); }
      F.verdict(!off.why && !off.why2 && (thr || off.behind > 0) && off.pushed >= 30, `the found case, rule off, ${how}: ${thr ? 'a man' : 'a man is dealt a mark beyond a mate who is ahead of him, and a man'} who had his mark is pushed off it (half a second or more)`, say(off));
      F.verdict(!on.why && !on.why2 && on.behind === 0 && on.pushed === 0 && on.nearPair && on.allT <= off.allT - 2, `the found case, rule on, ${how}: the lane fills from its far end, nobody is pushed off a mark he had, one pair holds the two marks nearest the door, and all four are set two seconds or more sooner`, `${say(on)} (rule off: ${off.allT >= 30 ? 'never in 30 s' : `${off.allT.toFixed(1)} s`})`);
    }
    // ---- 3. every other door
    { let asked = 0, tight = 0, changed = 0;
      for (const scn of ['compound-four', 'room-clear-four', 'depot-four']) { const r = await doors(scn); asked += r[0]; tight += r[1]; changed += r[2]; console.log(`  ${scn}: ${r[0]} stacks asked · ${r[1]} with a tight file · ${r[2]} dealt differently where none is tight`); }
      F.verdict(asked > 20 && tight > 0 && tight < asked / 2 && changed === 0, 'every door and face of three squad levels with a stack for four: some have a tight file, most have none, and where none is tight the deal is the very same with the rule on', `${asked} stacks · ${tight} with a tight file · ${changed} changed where none is tight`); }
    // ---- 4. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      ORDERS.fileQueue = true; const old = snap(); if (old.tables.ORDERS) delete old.tables.ORDERS.fileQueue; if (old.flags) delete old.flags.fileQueue; J.applyKnobs(R.game, old); const a = ORDERS.fileQueue;
      ORDERS.fileQueue = true; const yes = snap(); ORDERS.fileQueue = false; J.applyKnobs(R.game, yes); const b = ORDERS.fileQueue;
      ORDERS.fileQueue = false; const no = snap(); ORDERS.fileQueue = true; J.applyKnobs(R.game, no); const c = ORDERS.fileQueue;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { DOORMANNERS.on = keep.m; ORDERS.fileQueue = keep.q; ORDERS.knownFace = keep.k; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await filequeueCheck()) ? 0 : 1);
