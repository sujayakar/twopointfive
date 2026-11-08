// WHAT A MACHINE KEEPS OF A RUN (src/ui/runBooks.ts): the game says what there is to keep (view RunView), and the books keep it. A game stood by the harness is played to its end
// and the books told of it as either page tells them: a try counted as it begins, a win kept under its mission and difficulty, a load a try of its own, a cheat's run and a
// lesson's not kept, the course's best time.
//   TPS_GAME=rust bun run tools/qa/runbooks-check.ts     exit 0 all hold · 1 one does not
// RUNBOOKS_BREAK=1: the books are not told that another run stands, and the check must fail.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export async function runBooksCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { RunBooks } = await import(`${ROOT}/src/ui/runBooks.ts`), { recordOf } = await import(`${ROOT}/src/ui/verdictStore.ts`);
  let failed = 0; const line = (ok: boolean, s: string): void => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const mem = new Map<string, string>(); const G = globalThis as any, had = G.localStorage; G.localStorage = { getItem: (k: string) => mem.get(k) ?? null, setItem: (k: string, v: string) => { mem.set(k, v); }, removeItem: (k: string) => { mem.delete(k); } };
  const kept = (): Record<string, any> => JSON.parse(mem.get('twopointsix.verdicts') ?? '{}'), times = (): Record<string, number> => JSON.parse(mem.get('twopointsix.parbest') ?? '{}');
  const quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };

  async function stage(file: string | null, query: string) {
    const l = console.log; console.log = () => { }; let W: any; try { W = await standUp({ search: `?${query}`, seed: 5, ...(file ? { level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) } : {}) } as any); } finally { console.log = l; }
    const g = W.game, books = new RunBooks(); let of: unknown = null;
    if (query !== 'solo') { const r = quiet(() => startFromQuery(g, `?${query}`)); if (r?.why) throw new Error(`${query}: ${r.why}`); }
    // (as a page tells them: main.ts vBooks, drawpage.ts)
    const tell = (): void => { const V = g.verdict; if (V.current !== of) { of = V.current; if (!process.env.RUNBOOKS_BREAK) books.anew(); } if (!books.due(V.live && V.started, !!V.result)) return; books.tell(g.runView()); };
    const step = (n: number): void => quiet(() => { for (let i = 0; i < n; i++) { W.step(1 / 60); tell(); } });
    const nudge = (m: number): void => quiet(() => { const p = g.player.char.pos; g.teleportPlayer([p[0] + m, 0, p[2]]); });
    const win = (): void => { quiet(() => { for (const h of g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead')) killGuard(g, h, [1, 0, 0], false, 'shot', 'player'); }); step(30); };
    const lose = (): void => { for (let i = 0; i < 5 && !g.verdict.result; i++) { const p = g.player.char.pos; quiet(() => { for (let k = 0; k < 3; k++) g.dropThrowable('frag', [p[0], 0.3, p[2]]); }); step(480); } };
    step(6); return { W, g, books, step, nudge, win, lose };
  }

  { const A = await stage('killhouse.json', 'levelFile=killhouse.json&roomclear&squad4'), K = 'killhouse-four|standard';
    { const r = A.g.runView(); line(!r.begun && !r.over && r.mission === 'killhouse-four' && r.level === 'standard' && !r.lesson && r.par === null, `staged: not begun, the kill house at standard (${r.mission}, ${r.level}, ${r.course})`); }
    line(Object.keys(kept()).length === 0, 'nothing done is no try');
    A.nudge(2.5); A.step(4);
    line(recordOf(K).n === 1 && recordOf(K).s === 1, 'a stride: the try is counted as it begins');
    quiet(() => A.g.restartEncounter()); A.step(6); A.nudge(2.5); A.step(4);
    line(recordOf(K).n === 2 && recordOf(K).s === 2, 'given up and begun again: two tries');
    A.step(60); A.win();
    const r = A.g.runView(), way = recordOf(K).w[r.way!];
    line(r.over && r.won && !r.void && !!way && way.tries === 2 && way.wins === 1 && recordOf(K).s === 0, `won by ${r.way} at the second try, and kept so`);
    line(A.books.note?.streak === 2 && !!A.books.note.fresh && !A.books.note.gold && !!A.books.note.ways[r.way!], 'the card is told: the second try, a way that was dim, no gold yet');
    line(r.par !== null && times()[r.course] === Math.floor(r.par) && r.course === 'killhouse.json|way1|squad4|standard' && A.books.best === 'first time on this clock', `the course's time is kept (${r.course}: ${times()[r.course]} s)`);
    const n = JSON.stringify(kept()); A.step(120); line(JSON.stringify(kept()) === n, 'an end is kept once');
    quiet(() => A.g.restartEncounter()); A.step(6); A.g.godMode = true; A.nudge(2.5); A.step(8); A.win();
    line(JSON.stringify(kept()) === n && A.g.runView().void && A.books.note?.streak === 0 && A.books.best === '' && !!A.books.note?.ways[r.way!], 'a cheat\'s run is no try and no win; its card shows what was won before'); A.g.godMode = false;
    quiet(() => A.g.restartEncounter()); A.step(6); A.nudge(2.5); A.step(4); A.lose();
    line(!!A.g.verdict.result && !A.g.verdict.result.won && recordOf(K).n === 3 && recordOf(K).s === 1 && recordOf(K).w[r.way!]!.wins === 1, 'lost: the try stands, and nothing won is taken away'); }
  { const A = await stage(null, 'solo'), r0 = A.g.runView();
    line(r0.mission === 'solo-night' && r0.course.startsWith('office|'), `the plain night is the solo night's (${r0.mission}, ${r0.course})`); }
  { const A = await stage('tut-quiet.json', 'levelFile=tut-quiet.json&quiet'), n = JSON.stringify(kept()), r = A.g.runView();
    A.nudge(2.5); A.step(8); line(r.lesson && A.g.runView().begun && JSON.stringify(kept()) === n, `a lesson is no try (${r.mission})`);
    A.lose(); line(!A.g.verdict.result || (A.books.note?.lesson === true && JSON.stringify(kept()) === n), 'and its end is graded by nothing'); }
  console.log(`=== runbooks-check: ${failed ? `${failed} do not hold` : 'all hold'} ===`);
  if (had === undefined) delete G.localStorage; else G.localStorage = had;
  return !failed;
}

if (import.meta.main) { const ok = await runBooksCheck(); process.exit(ok ? 0 : 1); }
