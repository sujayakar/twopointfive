// THE UNPLAYED ROWS STAND APART ON THE MISSION LIST (src/ui/missions.ts: unplayed, unplayedClass, missionList).
// A row is UNPLAYED when it is `untuned` and the short list does not name it (scene/picks.ts onShortList). A group's unplayed rows come after
// its others and are drawn quieter (`.mrow.unplayed`), the first of them under a break (`.mrow.unplayed.first`).
//   bun run tools/qa/unplayed-check.ts   ·   bun run tools/qa/probes.ts unplayed (family missions)
// Nothing here looks at a picture: the lists are the page's own functions (missionList, the start page's startPageList) and the screens the real openMissions on
// the picker probes' fake DOM. What it holds:
//   1. the rule: unplayed(m) is `untuned` and not on the short list, on every row; no short-list row is unplayed
//   2. the order (both lists): every mission once, the groups in one run each, in no group a played row after an unplayed one, the short list's rows
//      still first in their group, and the whole order = the table's order (written out below, not asked of the code) after a stable partition of each group
//   3. the screens: the rows stand in the list's order and the keys walk them in it; every unplayed row wears `unplayed` and no other
//      does; `first` is worn by the first unplayed row of each group that holds a played one before it, and by no other; the classes outlive the cursor
//   UNPLAYED_BREAK=1  the table's own order is read as the lists': the check must FAIL (exit 1) — the proof that it can
// exit 0 = all hold · 1 = one does not
import { ROOT } from './headless.ts';
import { groupsFit } from './listed.ts';

export async function unplayedCheck(): Promise<boolean> {
  const { El, withDom } = await import('./scenario-picker-probes.ts');
  const M = await import(`${ROOT}/src/ui/missions.ts`), SP = await import(`${ROOT}/src/ui/startpage.ts`);
  const { PICKS, onShortList } = await import(`${ROOT}/src/scene/picks.ts`);
  const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  // the two orders before the unplayed rows go back, WRITTEN OUT HERE AGAIN and not asked of the code under test: the solo night and the
  // table's rows in the groups' order (ui/missions.ts missionList), and on the start page the short list's rows first inside their group (ui/startpage.ts
  // startPageList). No literal: a row added to the table, or taken off it, moves nothing here.
  const { MISSION_GROUPS } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const gRank = (g: string) => { const i = (MISSION_GROUPS as { id: string }[]).findIndex(x => x.id === g); return i < 0 ? MISSION_GROUPS.length : i; };
  const pRank = (id: string) => { const k = [...new Set((PICKS as { mission: string }[]).map(p => p.mission))].indexOf(id); return k < 0 ? 1e9 : k; };
  const flat: { id: string; group: string }[] = [M.SOLO_NIGHT, ...(ALL_MISSIONS as unknown[]).map(e => M.missionOf(e))];
  const LIST_OFF = flat.map((m, i) => ({ m, i })).sort((a, b) => gRank(a.m.group) - gRank(b.m.group) || a.i - b.i).map(x => x.m.id);
  const START_OFF = LIST_OFF.map((id, i) => ({ id, i, g: gRank(flat.find(m => m.id === id)!.group) })).sort((a, b) => a.g - b.g || pRank(a.id) - pRank(b.id) || a.i - b.i).map(x => x.id);
  const brk = process.env.UNPLAYED_BREAK === '1';
  type Mi = { id: string; group: string; untuned: boolean };

  let failed = false, checks = 0;
  const line = (ok: boolean, s: string, detail = '') => { checks++; if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const ids = (L: Mi[]) => L.map(m => m.id), same = (a: string[], b: string[]) => a.length === b.length && a.every((x, i) => x === b[i]);
  const groupsOf = (L: Mi[]) => [...new Set(L.map(m => m.group))];
  const inRuns = (L: Mi[]) => L.filter((m, i) => !i || L[i - 1].group !== m.group).length === groupsOf(L).length;   // (each group stands in one run)
  const rule = (m: Mi) => m.untuned && !onShortList(m.id);   // (the rule as the owner said it, asked of the tables themselves, not of the helper under test)
  /** a list after a STABLE partition of each group: its played rows, then its unplayed, both in the order they stood */
  const parted = (L: Mi[]) => groupsOf(L).flatMap(g => { const of = L.filter(m => m.group === g); return [...of.filter(m => !rule(m)), ...of.filter(rule)]; });
  const playedFirst = (L: Mi[]) => groupsOf(L).every(g => { const of = L.filter(m => m.group === g), k = of.findIndex(rule); return k < 0 || of.slice(k).every(rule); });
  /** the rows that should wear the break: the first unplayed of each group that holds a played row before it */
  const breaksOf = (L: Mi[]) => groupsOf(L).flatMap(g => { const of = L.filter(m => m.group === g), k = of.findIndex(rule); return k > 0 ? [of[k].id] : []; });
  const lay = (L: Mi[]) => groupsOf(L).map(g => `${g}: ${L.filter(m => m.group === g).map(m => (rule(m) ? '~' : '') + m.id).join(' ')}`).join('\n    ');
  /** the real screen over `list` on the fake DOM: what its rows wear, at the open and after the keys have walked every row */
  const drawn = (list: Mi[] | undefined, search = '?mute') => withDom(search, () => {
    const host = new El('DIV'), S = M.openMissions({ root: host as any, list: list as any, select: 0, onPick: () => { } }), L: Mi[] = S.list;
    const rows = () => host.all((e: any) => e.cls.has('mrow')).map((a: any) => ({ id: a.attrs['data-mission'] as string, quiet: a.cls.has('unplayed') as boolean, first: a.cls.has('first') as boolean, sel: a.cls.has('sel') as boolean }));
    const at0 = rows(), walked: string[] = [], worn: boolean[] = [];
    S.select(0); for (let i = 0; i < L.length; i++) { const r = rows(), k = r.findIndex((x: { sel: boolean }) => x.sel); walked.push(r[k]?.id ?? '?'); worn.push(!!r[k] && r[k].quiet === at0[k].quiet && r[k].first === at0[k].first); M.missionsKey({ code: 'KeyS' }); }
    const tag = (id: string) => { S.select(id); return host.all((e: any) => e.cls.has('untuned'))[0]?.style.display !== 'none'; };
    const tags = L.map(m => tag(m.id)); S.close();
    return { L, rows: at0, walked, kept: worn.every(Boolean), tags };
  });
  const screenOk = (D: Awaited<ReturnType<typeof drawn>>) => {
    const want = D.L.filter(rule).map(m => m.id), wantFirst = breaksOf(D.L);
    return same(D.rows.map(r => r.id), ids(D.L)) && same(D.walked, ids(D.L)) && D.kept && same(D.rows.filter(r => r.quiet).map(r => r.id), want) && same(D.rows.filter(r => r.first).map(r => r.id), wantFirst) && D.rows.every(r => !r.first || r.quiet)
      && D.tags.every((t, i) => t === rule(D.L[i]));   // (the sheet's tag is the helper's rule too)
  };
  const wears = (D: Awaited<ReturnType<typeof drawn>>) => `${D.rows.length} rows · ${D.rows.filter(r => r.quiet).length} wear unplayed · first: ${D.rows.filter(r => r.first).map(r => r.id).join(', ') || 'none'} · walked in order ${same(D.walked, ids(D.L))}`;

  try {
    const all = 1 + (ALL_MISSIONS as unknown[]).length, top: string[] = [...new Set((PICKS as { mission: string }[]).map(p => p.mission))];
    const off: Mi[] = LIST_OFF.map(id => flat.find(m => m.id === id)!) as Mi[], offStart: Mi[] = START_OFF.map(id => flat.find(m => m.id === id)!) as Mi[];
    // (UNPLAYED_BREAK=1: the unmoved orders stand in for the lists: every line below that wants a row moved then fails — which takes a table with an
    // unplayed row in it; no line here asks the table for one, so the day every row has been played the check still passes, with nothing left to set apart)
    const on: Mi[] = brk ? off : M.missionList(), onStart: Mi[] = brk ? offStart : SP.startPageList();
    line(on.every(m => M.unplayed(m) === rule(m)) && top.every(id => { const m = on.find(x => x.id === id); return !!m && !M.unplayed(m); }),
      'the rule: a row is unplayed when it is untuned and the short list does not name it; no short-list row is unplayed', `${on.filter(m => m.untuned).length} untuned · ${on.filter(rule).length} unplayed · short list: ${top.join(', ')}`);
    for (const [name, L, offL] of [['the list (missionList)', on, off], ['the start page (startPageList)', onStart, offStart]] as [string, Mi[], Mi[]][]) {
      const once = L.length === all && new Set(ids(L)).size === all, moved = ids(L).filter((id, i) => id !== offL[i].id).length;
      line(once && inRuns(L) && playedFirst(L) && same(ids(L), ids(parted(offL))) && groupsFit(M, L, groupsOf(L)),
        `${name}: every mission once, each group in one run, in no group a played row after an unplayed one, the order = the table's order after a stable partition of each group, and every group draws a row and no more than eight (listed.ts: a row cut from the list is not counted)`, `${L.length} of ${all} · ${moved} rows stand elsewhere than in the table's order\n    ${lay(L)}`);
    }
    line(groupsOf(onStart).every(g => { const of = onStart.filter(m => m.group === g), n = of.filter(m => top.includes(m.id)).length; return of.slice(0, n).every(m => top.includes(m.id)) && same(of.slice(0, n).map(m => m.id), top.filter(id => of.some(m => m.id === id))); }),
      'the start page: the short list\'s rows still stand first in their group, in play order', top.join(', '));
    { const A = await drawn(onStart), B = await drawn(undefined), want = breaksOf(onStart);
      line(screenOk(A), 'the start page\'s cards: every unplayed card wears `unplayed` and no other does; `first` is on the first unplayed card of each group that holds a played one, and on no other; the cards stand and are walked in the list\'s order, and keep their classes under the cursor', `${wears(A)} (wanted first: ${want.join(', ')})`);
      line(screenOk(B) && same(ids(B.L), ids(on)), 'a screen handed no list: the same on its rows, in missionList\'s order', wears(B)); }
  } catch (e) { failed = true; console.log(`- FAIL — threw: ${e instanceof Error ? e.stack ?? e.message : String(e)}`); }
  const ok = !failed && checks > 0;
  console.log(ok ? '=== unplayed-check: ALL PASS ===' : '=== unplayed-check: FAILED ===');
  return ok;
}

if (import.meta.main) process.exit(await unplayedCheck() ? 0 : 1);
