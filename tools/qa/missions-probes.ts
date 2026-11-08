// The 'missions' family (the rows of the mission list as they START; the rows, the sheet and the keys are the desk's: tools/qa/cards-page.ts walks them, rust/game_run/tests/desk.rs holds their rules), through probes.ts:
//   bun run tools/qa/probes.ts missions | missions-start
//   missions-start   rows' URLs (six, spread over the list; all under QA_SWEEP=all) booted in a process of its own (scenario-picker-probes.ts --boot, the page's own startFromQuery): it starts unrefused as
//                    that row, with the row's team beside Sam and exactly the defenders the sheet says; ?solo boots the plain night — no scenario, the shift
//                    and the civilians the solo sheet counts
import { ROOT } from './headless.ts';
import { sample } from './sweep.ts';

const { ALL_MISSIONS, pickSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { setDifficulty } = await import(`${ROOT}/src/game/difficulty.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
// ───────────────────────────────────────────────────────────── start ─────────────────────────────────────────────────────────────
const TAG = 'PICKER-BOOT ';
async function bootChild(query: string): Promise<any> {
  const p = Bun.spawn(['bun', 'run', `${ROOT}/tools/qa/scenario-picker-probes.ts`, '--boot', query], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });
  const out = await new Response(p.stdout).text(), errText = await new Response(p.stderr).text(); await p.exited;
  const line = out.split('\n').find(l => l.startsWith(TAG));
  return line ? JSON.parse(line.slice(TAG.length)) : { query, threw: `the boot printed no report (exit ${p.exitCode}): ${(errText || out).trim().split('\n').slice(-3).join(' | ')}` };
}
export async function missionsStart(): Promise<boolean> {
  const F = family('missions-start: every mission\'s URL booted in a process of its own — it starts as that row, with the team and the defenders its sheet says');
  setDifficulty('standard', false);
  const FLAGS = '?quality=high&mute', SOLO_ROW = { id: 'solo-night', title: 'Solo night' }, L: any[] = sample([SOLO_ROW, ...ALL_MISSIONS.map((e: any) => ({ id: e.id, title: e.title, squad: e.squad, scenario: e }))], 6), hrefOf = (m: any): string => m.scenario ? pickSearch(m.scenario, FLAGS) : `?solo&${FLAGS.slice(1)}`;
  const boots = await Promise.all(L.map(m => bootChild(hrefOf(m))));
  const SOLO = L.indexOf(SOLO_ROW);
  for (let i = 0; i < L.length; i++) { if (i === SOLO) continue; const m = L[i], r = boots[i], s = r.start;
    // (a QUIET mission's defenders are the level's own shift — staff, not a staged side: one of its patrol sets, so 1 to the sheet's 'up to N', and nobody staged against the team)
    const men = m.scenario.kind === 'quiet' ? s?.hostiles === 0 && s?.staff >= 1 && s?.staff <= m.scenario.defenders : s?.hostiles === m.scenario.defenders;
    const ok = !r.threw && s && s.why === null && s.entry === m.id && s.operatives === m.squad && men && (r.errors ?? []).length === 0;
    F.verdict(!!ok, `${String(i + 1).padStart(2, '0')} ${m.title}: ${hrefOf(m)}`, r.threw ? `threw: ${r.threw}` : `started ${s?.why === null ? 'unrefused' : `REFUSED (${s?.why})`} as '${s?.entry}' · ${s?.operatives} teammates (its row: ${m.squad}) · ${m.scenario.kind === 'quiet' ? `${s?.staff} of the shift on` : `${s?.hostiles} defenders`}${s?.hostages ? ` · ${s.hostages} hostages` : ''}${(r.errors ?? []).length ? ` · errors: ${r.errors.join(' | ')}` : ''}`); }
  if (SOLO >= 0) { const m = L[SOLO], r = boots[SOLO], s = r.start, want = ['', '3', '2'];   // (the shift: three guards and two civilians)
    const ok = !r.threw && s && r.spec === null && s.active === null && s.operatives === 0 && s.units === 0 && !!want && s.staff === Number(want[1]) && s.civilians === Number(want[2]);
    F.verdict(!!ok, `01 ${m.title}: ${hrefOf(m)}`, r.threw ? `threw: ${r.threw}` : `no scenario named or started (${s?.active ?? 'none'}) · nobody beside Sam · the shift ${s?.staff}, civilians ${s?.civilians}`); }
  return F.done();
}

if (import.meta.main) {
  const want = process.argv.slice(2).filter(a => !a.startsWith('-')); const on = (n: string) => !want.length || want.includes(n) || want.includes('missions');
  let ok = true;
  if (on('missions-start')) ok = (await missionsStart()) && ok;
  process.exit(ok ? 0 : 1);
}
