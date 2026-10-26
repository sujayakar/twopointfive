// THE ROWS CUT FROM THE MISSION LIST ARE DRAWN ON NO LIST, AND STILL OPEN BY THEIR OWN ADDRESS — scene/scenarios.ts `offList`, ui/missions.ts OFFLIST_UI /
// listedMissions. The maintainer's tap, 2026-10-13 ('Three'): the four-man office room clear and both yard rows leave the list, and one of the two block rows
// when the block's rebuild says which (it said: the three squads); their rows, level files and
// every check that names them stay. The night breach joined them 2026-10-24 (the maintainer: "let's also remove "night breach" since it's unnecessary now"):
// its row, its address and the checks that stage it stay, and it is on the short list no more (line 7). Headless: the lists as data and the two screens on the fake DOM (tools/qa/scenario-picker-probes.ts El / withDom).
//   bun run tools/qa/probes.ts offlist        (family missions)        bun run tools/qa/offlist-check.ts
//   1. the table: the rows that carry `offList` are exactly the ones his tap named, and each is still a row of the table with its own address
//   2. the module's default is OFF (the page's entry puts it on — main.ts): in a harness the lists hold every row, as every list probe pins them
//   3. on: listedMissions and the start page's list hold every other row once, in the order they stood, and none of the cut ones; so do both screens' rows
//   4. on: a cut row still reads back from its own address (missionOfQuery), and a visit that names one (`?loadout=<id>`, a `select`) has it drawn that once
//   5. on: a won lesson's end card leads to the next lesson the lists draw (ui/verdictNext.ts), a cut one is nobody's next, and one played by its own address
//      leads to the listed one after it; off: every lesson leads to the next row of the table (2026-10-26, the maintainer's reorder of the tutorial: plan
//      mode's mission and the tactics mission leave the list "for now"; their rows, levels and checks stay)
//   6. off (?offlist=0): the lists are the whole table again
//   7. no row of the short list (scene/picks.ts PICKS) starts a cut row: a card would offer what the list has dropped
//   OFFLIST_BREAK=1  the switch is left off where the lines above want it on: the check must FAIL (exit 1) — the proof that it can
// exit 0 = all hold · 1 = one does not
import { ROOT } from './headless.ts';

/** his tap: these and no other (the block's row joined when its rebuild said which of the two it was for) */
const CUT = ['night-breach', 'room-clear-four', 'block-squads', 'quiet-yard-alone', 'quiet-yard-two', 'tut-plan', 'tut-squad'];

export async function offlistCheck(): Promise<boolean> {
  const { El, withDom } = await import('./scenario-picker-probes.ts');
  const M = await import(`${ROOT}/src/ui/missions.ts`), SP = await import(`${ROOT}/src/ui/startpage.ts`);
  const { ALL_MISSIONS, TUTORIAL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { PICKS } = await import(`${ROOT}/src/scene/picks.ts`);
  const brk = process.env.OFFLIST_BREAK === '1', was = M.OFFLIST_UI.on;
  type Mi = { id: string; scenario: { offList?: boolean } | null };
  let failed = false, checks = 0;
  const line = (ok: boolean, s: string, detail = '') => { checks++; if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const ids = (L: Mi[]) => L.map(m => m.id), same = (a: string[], b: string[]) => a.length === b.length && a.every((x, i) => x === b[i]);
  const drawn = (tiles: boolean, select?: string) => withDom('?mute', () => {
    const host = new El('DIV'), list = tiles ? SP.startPageList(M.listedMissions(undefined, select)).list : undefined;
    const S = M.openMissions({ root: host as any, list: list as any, fold: tiles, tiles, select }), got = host.all((e: any) => e.cls.has('mrow')).map((a: any) => a.attrs['data-mission'] as string);
    S.close(); return got as string[];
  });
  try {
    const all: Mi[] = M.missionList(), cutRows = (ALL_MISSIONS as { id: string; offList?: boolean }[]).filter(e => e.offList).map(e => e.id);
    line(same([...cutRows].sort(), [...CUT].sort()) && CUT.every(id => all.some(m => m.id === id)), 'the table: the rows cut from the list are the ones his tap named, and each is still a row of the table', cutRows.join(', '));
    line(was === false && same(ids(M.listedMissions()), ids(all)), 'the module\'s default is off: in a harness the list holds every row (the page\'s entry puts the switch on)', `${all.length} rows`);

    M.OFFLIST_UI.on = !brk;
    const want = ids(all).filter(id => !CUT.includes(id));
    { const L: Mi[] = M.listedMissions(), S: Mi[] = SP.startPageList().list;
      line(same(ids(L), want) && same([...ids(S)].sort(), [...want].sort()), 'on: the list and the start page\'s list hold every other row once, the list in the order it stood, and none of the cut ones', `${L.length} of ${all.length} rows · start page ${S.length}`);
      const A = await drawn(true), B = await drawn(false);
      line(same([...A].sort(), [...want].sort()) && same(B, want), 'on: the start page\'s cards and the older list\'s rows are those rows and no other', `cards ${A.length} · list ${B.length}`); }
    { const back = CUT.map(id => { const m = all.find(x => x.id === id)!; return M.missionOfQuery('?' + String(M.missionHref(m, '')).replace(/^[^?]*\?/, ''))?.id; });
      const named = await Promise.all(CUT.map(async id => { const A = await drawn(true, id), B = await drawn(false, id); return A.includes(id) && B.includes(id) && A.length === want.length + 1 && B.length === want.length + 1; }));
      line(same(back as string[], CUT) && (brk || named.every(Boolean)), 'on: a cut row still reads back from its own address, and a visit that names one has it drawn that once, beside the listed rows and no other cut row', `read back: ${back.join(', ')} · named: ${named.join(', ')}`); }

    // THE TUTORIAL'S NEXT LESSON (ui/verdictNext.ts): a won lesson's end card leads to the next lesson THE LISTS DRAW, in the tutorial's own order — a cut one
    // is nobody's next lesson, and one played by its own address leads to the listed lesson after it. What the lists draw is asked of listedMissions here,
    // not of the rows' own flag: the card and the list must agree.
    const { VERDICT } = await import(`${ROOT}/src/game/verdict.ts`), { nextLessonOf } = await import(`${ROOT}/src/ui/verdictNext.ts`);
    const vWas = VERDICT.on, tut = (TUTORIAL_MISSIONS as { id: string }[]).map(e => e.id), nextOf = (id: string) => nextLessonOf(id, '')?.id ?? null;
    const after = (id: string, among: string[]) => tut.slice(tut.indexOf(id) + 1).find(x => among.includes(x)) ?? null;
    VERDICT.on = true;
    try {
      const drawnT = tut.filter(id => ids(M.listedMissions()).includes(id)), cutT = tut.filter(id => !drawnT.includes(id));
      const chain = drawnT.map(nextOf), fromCut = cutT.map(nextOf);
      line(drawnT.length >= 2 && cutT.length > 0 && chain.every((x, i) => x === (drawnT[i + 1] ?? null)) && fromCut.every((x, i) => x === after(cutT[i], drawnT)),
        'on: a won lesson leads to the next lesson the list draws and the last to none; a cut lesson is nobody\'s next, and played by its own address it leads to the listed one after it',
        `${drawnT.map((id, i) => `${id} → ${chain[i] ?? 'none'}`).join(' · ')} · by its own address: ${cutT.map((id, i) => `${id} → ${fromCut[i] ?? 'none'}`).join(' · ') || '(no tutorial row is cut)'}`);
      M.OFFLIST_UI.on = false;
      const whole = tut.map(nextOf);
      line(whole.every((x, i) => x === (tut[i + 1] ?? null)), 'off (?offlist=0): every lesson leads to the next row of the table, as before the cut', tut.map((id, i) => `${id} → ${whole[i] ?? 'none'}`).join(' · '));
    } finally { VERDICT.on = vWas; }

    M.OFFLIST_UI.on = false;
    line(same(ids(M.listedMissions()), ids(all)) && same(await drawn(false), ids(all)), 'off (?offlist=0): the lists are the whole table again', `${all.length} rows`);
    { const picks = (PICKS as { id: string; mission: string }[]).map(p => p.mission), onCut = (brk ? [...picks, CUT[0]] : picks).filter(id => CUT.includes(id));   // (OFFLIST_BREAK: the night breach's pick stood back on the short list)
      line(onCut.length === 0, 'the short list starts no row that is cut from the list', onCut.length ? `STARTS A CUT ROW: ${onCut.join(', ')}` : `${picks.length} picks: ${picks.join(', ')}`); }
  } catch (e) { failed = true; console.log(`- FAIL — threw: ${e instanceof Error ? e.stack ?? e.message : String(e)}`); }
  finally { M.OFFLIST_UI.on = was; }
  const ok = !failed && checks > 0;
  console.log(ok ? '=== offlist-check: ALL PASS ===' : '=== offlist-check: FAILED ===');
  return ok;
}

if (import.meta.main) process.exit(await offlistCheck() ? 0 : 1);
