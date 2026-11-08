// The 'laststand' family (game/laststand.ts — what the last few defenders of a room clear do once most are down: the call out, which is settled and on, and
// the regroup, an open decision that the page has off and the regroup probes raise), run through
// probes.ts:   bun run tools/qa/probes.ts laststand | laststand-callout
// PASS / FAIL each, all headless — no picture and no sound here. Each stages levels/compound.json (five on eleven, three squads), leaves the team on its start
// mark and takes the defenders down by hand to the three the probe names, so the endgame starts on a known floor.
//   laststand-callout   the call out with no regroup: a radio check, answers naming the rooms the men are truly in, the count and the asker's room, the strip
//                       draws them, the objective brackets exactly those rooms and its marker sits on one; down to one, he says he is the last and where
//                       (a man at his post: the net names his room). LASTSTAND_BREAK=1 = a second man is left up in the last-man run: that line must fail.
//                       (The call out is settled: no line asks it off and nothing here writes it. Until 2026-11-07 the probe ran the floor twice, with
//                       it on and off, and held the two runs' digests together with the net-goes-quiet rule pinned off; the break word was call out off.)
//   laststand-few       'most are down' is never more than a third of the staging: on the block's eight (levels/block.json) three up says nothing, two
//                       up starts the check
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { LASTSTAND, lastStandReport } = await import(`${ROOT}/src/game/laststand.ts`);
const { digInPhase } = await import(`${ROOT}/src/game/digin.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { stripRows } = await import('./heard.ts');
const { placeIn } = await import(`${ROOT}/src/game/dialogue.ts`);
const { runnerBreak, runnerOf } = await import(`${ROOT}/src/game/runner.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
async function rig(seed: number, level = 'compound') {
  Math.random = seeded(seed);
  const log = console.log, err = console.error, warn = console.warn; let W: any;
  try { console.log = console.error = console.warn = () => { };
    W = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${level}.json`), 'utf8'))) });
    await startFromQuery(W.game, new URLSearchParams(`?levelFile=${level}.json&roomclear=${seed}&squad4`));
  } finally { console.log = log; console.error = err; console.warn = warn; }
  return W;
}
const up = (g: any) => g.nightBreach.hostiles.filter((h: any) => h.state !== 'dead');
const roomOf = (g: any, gd: any): string | null => g.roomClear.roomAt(gd.char.pos);
const nameOf = (g: any, id: string | null) => g.roomClear.rooms.find((r: any) => r.def.id === id)?.def.name ?? '?';
/** the team stays on its start mark, unseen; every defender but the first of each room in `keep` is taken down by hand (quietly: no shot for anybody to hear) */
function thin(W: any, keep: string[]) {
  const g = W.game, kept: any[] = [];
  for (const R of g.roomClear.rooms) { const stay = keep.includes(R.def.id) ? R.defenders[0] : null; if (stay) kept.push(stay); for (const h of R.defenders.slice()) if (h !== stay && h.state !== 'dead') killGuard(g, h, [1, 0, 0], true); }
  return kept;
}
function steps(W: any, secs: number, each?: (i: number) => void) { const g = W.game; for (let i = 0; i < Math.round(secs * 60); i++) { g.player.visibility = 0.05; W.step(DT); each?.(i); } }
/** the room clear's row on the Rust game's record of the mission's line */
const rowOf = (g: any, of: string) => g.view.mission?.of === of ? g.view.mission : null;
/** the rooms their net has named, as the row shows them: its [bracketed] names that are rooms of the structure, by id (a place or the response team is no room) */
const namedRooms = (g: any): string[] => { const on = new Set([...String(rowOf(g, 'roomclear')?.text ?? '').matchAll(/\[([^\]]+)\]/g)].map(m => m[1])); return g.roomClear.rooms.filter((r: any) => on.has(r.def.name)).map((r: any) => r.def.id); };
const said = (g: any) => lastStandReport(g).said as { t: number; who: string; kind: string; text: string }[];

export async function lastStandCallOut(): Promise<boolean> {
  const F = family('laststand-callout: the rooms on their net are the rooms they are in');
  const was = { ...LASTSTAND };
  try {
    const KEEP = ['office', 'bunk', 'back'];
    const W = await rig(5), g = W.game;
    let clean = 0, first = -1; const rows = new Set<string>();
    const frame = () => { for (const r of stripRows(g) as { text: string }[]) rows.add(r.text); };
    steps(W, 1, frame); clean = said(g).length + lastStandReport(g).stats.checks;
    const kept = thin(W, KEEP);
    steps(W, 20, i => { frame(); if (first < 0 && said(g).length) first = i / 60; });
    const A = { rep: lastStandReport(g), rows, obj: { ...rowOf(g, 'roomclear') }, hint: namedRooms(g).sort(), truth: kept.map((h: any) => roomOf(g, h)).sort() as string[], clean, first };
    // down to one from the start: nobody answers him, he says he is the last and where. The guard hut's man (a man is named by what he SAID):
    //   a squad of one, so no mate of his drops and he HOLDS HIS POST: he says the hut and the net names the hut.
    // LASTSTAND_BREAK=1 = a second man is left up with him (the bunk room's): he is not the last, and both lines must fail.
    for (const id of ['hut']) {
      const W = await rig(5), g = W.game; steps(W, 1); const [one] = thin(W, process.env.LASTSTAND_BREAK === '1' ? [id, 'bunk'] : [id]);
      let room: string | null = null, named = '', spot = '', hunting = false, got = false;   // (where he is, what he is doing and what the net has named, the frame he says it — he may move afterwards)
      steps(W, 16, () => { if (!got && said(g).some(l => l.kind === 'lastAt')) { got = true; room = roomOf(g, one); named = namedRooms(g).join(); spot = room || !LASTSTAND.places ? '' : placeIn(g.level, one.char.pos)?.name ?? ''; hunting = (g.fight.defs as any[]).find(D => D.gd === one)?.mode === 'free'; } });
      const tail = said(g), last = tail.find(l => l.kind === 'lastAt'), post = id === 'hut';
      // (no room of the structure: 'outside the …' on ground the level has no word for — or, LASTSTAND.places, 'in the corridor' by the level's own name for the floor he stands on)
      const says = !!last && last.who === one.callsign && (room ? last.text.includes(`in the ${nameOf(g, room)}`) : spot ? last.text.includes(`in ${spot}`) : /outside the /.test(last.text));
      F.verdict(tail[0]?.kind === 'check1' && says && named === (hunting ? '' : room ?? '') && (!post || (!hunting && room === id)),
        'the last man at his post: nobody answers his check, he says so and where he is, and the net names his room',
        `${tail.map(l => `${l.who} '${l.text}'`).join(' · ') || 'nothing said'} → ${!got ? 'he never says he is the last' : `${hunting ? 'out hunting' : 'holding'}, truly ${room ? 'in ' + room : 'outside the rooms'} then, named '${named}'${spot ? `, place '${spot}'` : ''}`}`);
    }
    const S = A.rep.said as { who: string; kind: string; text: string }[], ans = S.filter(l => l.kind === 'ansAt'), few = S.find(l => l.kind === 'fewAt');
    F.verdict(A.clean === 0 && A.first >= LASTSTAND.first[0] && A.first <= LASTSTAND.first[1] + 0.1, 'nothing is said with eleven up; the check goes out a few seconds after only three are left', `first line ${A.first.toFixed(1)} s after`);
    F.verdict(S[0]?.kind === 'check1' && ans.length === 2 && !!few && /3 of us/.test(few.text) && A.rep.known.length === 3, 'a radio check, two answers, and the asker says the count', S.slice(0, 5).map(l => `${l.who}: ${l.text}`).join(' · '));
    F.verdict(A.hint.join() === A.truth.join() && A.truth.length === 3, 'the rooms named on their net are the rooms the three are in', `named ${A.hint.join(', ')} · truly in ${A.truth.join(', ')}`);
    const drawn = [...A.rows].filter(t => /still up|still here|holding|of us/.test(t));
    F.verdict(drawn.length >= 2, "the answers reach the radio strip (their net's rows)", `${drawn.length} drawn: ${drawn.slice(0, 3).join(' · ')}`);
    const br = (A.obj.text.match(/\[[^\]]+\]/g) ?? []) as string[];
    F.verdict(br.length === 3 && !!A.obj.pos, 'the objective brackets the three rooms named and keeps a marker', A.obj.text);
  } finally { Object.assign(LASTSTAND, was); }
  return F.done();
}

export async function lastStandFew(): Promise<boolean> {
  const F = family('laststand-few: the last few are never more than a third of the men staged');
  const was = { ...LASTSTAND };
  try {
    const W = await rig(2, 'block'), g = W.game; steps(W, 1);
    const all = g.nightBreach.hostiles.slice(), down = (n: number) => { for (const h of all) if (up(g).length > n && h.state !== 'dead') killGuard(g, h, [1, 0, 0], true); };
    down(3); steps(W, 12); const at3 = said(g).length + lastStandReport(g).stats.checks;
    down(2); steps(W, 12); const at2 = said(g);
    F.verdict(all.length === 8 && at3 === 0, 'eight staged, three up: nothing is said (three is more than a third)', `${all.length} staged · ${at3} lines or checks with three up`);
    F.verdict(at2[0]?.kind === 'check1' && lastStandReport(g).stats.up0 === 2, 'two up: the radio check goes out', at2.slice(0, 3).map(l => `${l.who}: ${l.text}`).join(' · '));
  } finally { Object.assign(LASTSTAND, was); }
  return F.done();
}

/** A MAN WHO HAS DROPPED HIS GUN AND RUN IS NO MAN OF THE RADIO CHECK (laststand.ts alive / fitGd ask runner.ts `OWN.runner`). Found in a play session on the
 *  block: a green man saw his leader shot, dropped his gun, was cornered in his room — and then LED the last stand's check and closed it counting himself.
 *  Here: the block, eight staged, three left up by hand, one of the three made to run (runner.ts `OWN.runner`). Two fighting men are then the last few: the check goes out, and the man who ran neither calls it, nor is asked for by
 *  name, nor answers, nor is counted. */
export async function lastStandFled(): Promise<boolean> {
  const F = family('laststand-fled: a man who has dropped his gun and run is no man of the radio check');
  const was = { ...LASTSTAND };
  try {
    const W = await rig(2, 'block'), g = W.game; steps(W, 1);
    const all = g.nightBreach.hostiles.slice(); for (const h of all) if (up(g).length > 3 && h.state !== 'dead') killGuard(g, h, [1, 0, 0], true);
    steps(W, 2); const before = said(g).length + lastStandReport(g).stats.checks;
    const ran = up(g).find((h: any) => runnerBreak(g, h)) ?? null; steps(W, 40);
    const R = lastStandReport(g), lines = said(g), name = ran?.callsign ?? '?', counts = lines.flatMap(l => [...l.text.matchAll(/(\d+) of us/g)].map(m => Number(m[1])));
    const his = lines.filter(l => l.who === name), named = lines.filter(l => l.who !== name && l.text.includes(name));
    F.verdict(!!ran && ran.fled && ran.state !== 'dead' && before === 0, 'three up and nothing said; one of them is made to run: his gun is out of his hand and he is not dead', `${name}: ${ran ? `phase ${runnerOf(ran)?.phase}, fled ${ran.fled}, ${ran.state}` : 'nobody would run'} · ${before} lines or checks before`);
    F.verdict(R.stats.checks >= 1 && R.stats.up0 === 2 && lines[0]?.kind === 'check1' && lines[0].who !== name, 'the two who still fight are the last few: the check goes out, called by one of them', `checks ${R.stats.checks}, up at the start ${R.stats.up0} · ${lines.slice(0, 2).map(l => `${l.who}: ${l.text}`).join(' · ')}`);
    F.verdict(his.length === 0 && named.length === 0 && !R.known.includes(name), 'the man who ran says nothing on the net, is not asked for by name and is not among the men known up', `his lines ${his.length}${his[0] ? ` ("${his[0].text}")` : ''} · lines naming him ${named.length}${named[0] ? ` ("${named[0].text}")` : ''} · known up: ${R.known.join(', ') || 'nobody'}`);
    F.verdict(counts.every(n => n <= 2), 'no count spoken is more than the two who fight', counts.length ? `counts spoken: ${counts.join(', ')}` : 'no count was spoken in the forty seconds (nothing to hold)');
  } finally { Object.assign(LASTSTAND, was); }
  return F.done();
}
