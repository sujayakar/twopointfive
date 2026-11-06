// THE SETTLED ROWS FOLDED (tools/qa/fold.ts) AND THEIR LIST (tools/qa/switches.json), held to their word.
//   bun run tools/qa/fold-check.ts        exit 0 = all hold · 1 = one does not      (probes.ts fold; the same on both stands: every game here is stood in a
//                                          child on the page's stand, where the fold lives)
//   FOLD_BREAK=1 in front = the fold is laid to COUNT a write and let it through: lines (3), (5) and (6) must FAIL.
// (1) every settled row of the list is in the knob tables, a plain value, and at the list's value on a game the page's stand stood.
// (2) every yes/no row of the tables is on the list: a new switch says its kind, or this line is red. THE MEND for a new rule's switch:
//     `bun run tools/qa/switch-list.ts new` (it joins as an open decision at its page value; the port's list is told after).
// (3) under the fold a settled row answers its page value whatever is written; the write is counted once, by row, value and writer; writing the page's own
//     value counts nothing; a row that is not settled is still a switch.
// (4) the runner's fence (holds.ts asProbe: every table copied and put back round a probe) leaves the fold standing, and a branch a probe replaced whole
//     inside it is named in the probe's line with the value it was left at.
// (5) a branch of a table replaced whole loses its hold: that is found, named with the value it was left at, and laid again.
// (6) THE PAGE'S OWN CODE writes no settled row off its value: a game stood and stepped, then Hard and back, then each of the camera's modes, under the
//     fold, is refused nothing, and Hard reads as Hard. (A row the player's own settings write is NOT settled: a list that says so is red here.) What a
//     recording's reader, an address word and the debugger's panel write is NOT played here: those roads are refused by design.
// (7) the runner says one line for a probe, with the probe's own write and its child's; a word given by hand that pins a settled row is said in a line of
//     its own before the first probe, or, where a module read the word as it loaded, in the notice on stderr; on the old stand nothing is folded and that
//     is said.
// (8) who speaks: a check file's own main run alone says the probe's line itself, its child's write in it; a process nobody reads for says its own on
//     STDERR as it ends and nothing on stdout; an arm on the old stand under a folded parent is not folded and says nothing; a child given a word its
//     parent was not (one a module reads as it loads) has the row counted and says so; the words that ask for the fold are QA_FOLD=1 and TPS_GAME =
//     rust, audit or table; and `switch-list.ts new` refuses a run with a word of the stand by hand.
// WHAT THIS DOES NOT HOLD: `switch-list.ts make` and `same` (they write and read the list's own file), gate.sh's two lines, the `plain` line of a run
// without the fold; that the list is what the port's list makes (`bun run tools/qa/switch-list.ts same <the port's list>`: that list is not in the
// repository); a write through a road (6) does not walk (a menu's own row: the fold names it when a check walks it); a row read into a local before the
// fold is laid.
import { resolve } from 'node:path';
import { switchRows, settledOf, settledYesNo, unlistedYesNo, hostOf, sameValue, type Refused } from './fold.ts';

const ME = resolve(import.meta.dir, 'fold-check.ts'), mode = process.argv[2] ?? '';
const cell = (reg: Record<string, unknown>, row: string): { host: Record<string, unknown>; key: string } | null => { const [t, ...p] = row.split('.'), key = p[p.length - 1], host = hostOf(reg[t], p.slice(0, -1)); return host && key in host ? { host, key } : null; };
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } };

// ---------------------------------------------------------------- the children (each a fresh process on the page's stand)
if (mode === '--rows') {   // no fold: the list against a stood game
  const { standUp, ROOT } = await import('./headless.ts'), { knobRegistry, NOT_KNOBS } = await import(`${ROOT}/src/game/knobs.ts`);
  await quiet(() => standUp());
  const reg = knobRegistry() as Record<string, unknown>, L = switchRows(), missing: string[] = [], off: string[] = [], own: string[] = [];
  for (const r of L) { if (r.kind !== 'settled') continue; const c = cell(reg, r.row); if (!c) { missing.push(r.row); continue; }
    const d = Object.getOwnPropertyDescriptor(c.host, c.key); if (d?.get || d?.set) own.push(r.row); if (!sameValue(c.host[c.key], r.page)) off.push(`${r.row} (the game ${JSON.stringify(c.host[c.key])}, the list ${JSON.stringify(r.page)})`); }
  const unlisted = unlistedYesNo(reg, L, NOT_KNOBS).map(r => r.row), yesNo = unlistedYesNo(reg, [], NOT_KNOBS).length;
  console.log(JSON.stringify({ rows: { settled: L.filter(r => r.kind === 'settled').length, missing, off, own, yesNo, unlisted } })); process.exit(0);
}
if (mode === '--fold') {   // QA_FOLD=1: what the fold does
  const H = await import('./headless.ts'), F = await import('./fold.ts'), { knobRegistry, knobsCopy, knobsBack } = await import(`${H.ROOT}/src/game/knobs.ts`), { asProbe } = await import('./holds.ts');
  const reg = knobRegistry() as Record<string, any>, L = switchRows();
  const A = settledYesNo(L, 2)!, N = settledYesNo(L, 3)!, O = L.find(r => r.kind === 'open decision' && typeof r.page === 'boolean' && cell(reg, r.row))!;
  const a = cell(reg, A.row)!, n = cell(reg, N.row)!, o = cell(reg, O.row)!, out: Record<string, unknown> = { laid: F.FOLD.rows, skipped: F.FOLD.skipped, missing: F.FOLD.missing, A: A.row, N: N.row, O: O.row };
  F.foldTake();
  a.host[a.key] = !A.page; out.reads = a.host[a.key] === A.page; a.host[a.key] = !A.page; a.host[a.key] = A.page; out.counted = F.foldTake();                        // (3) twice off, once the page's own
  const o0 = o.host[o.key]; o.host[o.key] = !o0; out.open = o.host[o.key] === !o0 && F.foldTake().length === 0; o.host[o.key] = o0;                                    // (3) not settled: a switch
  { const copy = knobsCopy(); a.host[a.key] = !A.page; n.host[n.key] = !N.page; knobsBack(copy); F.foldTake(); out.fenceLost = F.foldLost(); }                         // (4) the copy and put-back alone
  // (this process runs its probes with nobody above it, so the fence itself says each probe's line: read here off what it printed)
  const spoken = async (fn: () => void): Promise<{ probe?: string; refused: Refused[]; lost?: string[] }[]> => { const lines: string[] = [], log = console.log; console.log = (x: unknown) => { lines.push(String(x)); };
    try { await asProbe('fold', fn); } finally { console.log = log; } return lines.filter(l => l.trim().startsWith('{"fold"')).map(l => (JSON.parse(l) as { fold: { probe?: string; refused: Refused[]; lost?: string[] } }).fold); };
  { const said = await spoken(() => { a.host[a.key] = !A.page; n.host[n.key] = !N.page; }); out.fenceReads = a.host[a.key] === A.page && n.host[n.key] === N.page; out.fenceLost2 = F.foldLost();
    out.fenceSaid = said.length === 1 && said[0].probe === 'fold' && said[0].refused.length === 2 && !said[0].lost; }
  { const [t, branch] = N.row.split('.'), old = reg[t][branch], said = await spoken(() => { reg[t][branch] = { ...old, [n.key]: !N.page }; });                        // (4) …and a branch replaced whole inside the fence
    out.fenceNamed = said[said.length - 1]?.lost ?? []; reg[t][branch] = old; F.foldLost(); F.foldLine('', false);
    await spoken(() => { delete reg[t][branch]; });                                                                                                                  // (4) …and a branch DELETED inside it: the put-back makes it again, plain
    out.fenceRemade = !!Object.getOwnPropertyDescriptor(reg[t][branch], n.key)?.get && reg[t][branch][n.key] === N.page; reg[t][branch] = old; F.foldLost(); F.foldLine('', false); }
  { const [t, branch] = N.row.split('.'), old = reg[t][branch]; reg[t][branch] = { ...old, [n.key]: !N.page }; out.plainAgain = reg[t][branch][n.key] === !N.page;   // (5) a branch replaced whole
    out.lost = F.foldLost(); out.laidAgain = reg[t][branch][n.key] === N.page; reg[t][branch] = old; out.lostAfter = F.foldLost(); }
  if (process.env.FOLD_BREAK === '1') F.foldSettled(reg, { 'PRESSURE.second': false }, true);   // (the red path of (6): the list as it stood before 2026-11-06, with a row Hard writes called settled)
  { let threw = '', hard = '', standard = ''; F.foldTake();                                                                                                           // (6) the page's own roads
    try { const W = await quiet(() => H.standUp()); for (let i = 0; i < 120; i++) W.step(1 / 60);
      const D = await import(`${H.ROOT}/src/game/difficulty.ts`), C = await import(`${H.ROOT}/src/host/autocam.ts`);
      D.setDifficulty('hard', false); hard = String(D.difficultyNow()); for (let i = 0; i < 30; i++) W.step(1 / 60); D.setDifficulty('standard', false); standard = String(D.difficultyNow());
      for (const m of Object.keys(C.AUTO_MODES)) { C.setAutoMode(m); for (let i = 0; i < 10; i++) W.step(1 / 60); } C.resetAutoTune();
    } catch (e: any) { threw = String(e?.message ?? e); }
    out.stood = { threw, hard, standard, refused: F.foldTake(), lost: F.foldLost() }; }
  console.log(JSON.stringify({ result: out })); process.exit(0);
}
if (mode === '--main') {   // a check file's own main, run alone: its probe writes a settled row off and so does its child
  const { ROOT } = await import('./headless.ts'), { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`), { asProbes, exitCode } = await import('./holds.ts'), A = settledYesNo(switchRows(), 2)!;
  process.exit(exitCode(await asProbes({ 'selftest-fold': async () => { const c = cell(knobRegistry() as Record<string, unknown>, A.row)!; c.host[c.key] = !A.page;
    return (await Bun.spawn(['bun', 'run', ME, '--write'], { stdout: 'inherit', stderr: 'inherit', env: process.env }).exited) === 0; } })));
}
if (mode === '--write') {   // a child of a probe: one settled row written off, handed to the runner as this process ends
  const { ROOT } = await import('./headless.ts'), { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`), A = settledYesNo(switchRows(), 2)!;
  const c = cell(knobRegistry() as Record<string, unknown>, A.row)!; c.host[c.key] = !A.page; process.exit(0);
}

// ---------------------------------------------------------------- the check
const BREAK = process.env.FOLD_BREAK === '1';
let fails = 0; const line = (ok: boolean, label: string, detail = ''): void => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}${detail ? ` — ${detail}` : ''}`); if (!ok) fails++; };
const few = (xs: readonly string[], n = 6): string => xs.length ? `${xs.slice(0, n).join(' · ')}${xs.length > n ? ` … (${xs.length})` : ''}` : 'none';
const base: Record<string, string> = { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', ...(process.env.TMPDIR ? { TMPDIR: process.env.TMPDIR } : {}), ...(BREAK ? { FOLD_BREAK: '1' } : {}) };
async function child(args: string[], env: Record<string, string>): Promise<{ out: string; err: string; code: number }> {
  const p = Bun.spawn(['bun', 'run', ...args], { stdout: 'pipe', stderr: 'pipe', env: { ...base, ...env } }), [out, err, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]);
  return { out, err, code };
}
/** every line of `text` that is `{"<key>": …}`, parsed */
const jsons = <T>(text: string, key: string): T[] => { const out: T[] = []; for (const l of text.split('\n')) { const s = l.trim(); if (s.startsWith(`{"${key}"`)) { try { out.push((JSON.parse(s) as Record<string, T>)[key]); } catch { /* not one */ } } } return out; };
const said = (r: { err: string; code: number }): string => `the child said nothing (exit ${r.code}): ${r.err.trim().split('\n').slice(-2).join(' / ')}`;
const refusedText = (xs: readonly Refused[]): string => few(xs.map(x => `${x.row}=${String(x.value)} @${x.writer}${x.n > 1 ? ` ×${x.n}` : ''}`));
if (BREAK) console.log('FOLD_BREAK=1: the fold counts a write and lets it through — lines (3), (5) and (6) must FAIL');

const L = switchRows(), settled = Object.keys(settledOf(L));
// (the rows this check names itself: two a word of the harness writes, DOORWAY.oneWay by ONEWAY as it loads and GLASSFX.on by GLASSFX as its module loads,
// must be settled; the red path of (6) lays PRESSURE.second, which must NOT be. A list that changes one of them asks for another row here, and says so.)
{ const kind = (row: string): string => L.find(r => r.row === row)?.kind ?? 'not on the list', wrong = [['DOORWAY.oneWay', 'settled'], ['GLASSFX.on', 'settled'], ['PRESSURE.second', "player's or run-time"]].filter(([r, k]) => kind(r) !== k);
  if (wrong.length) line(false, `(0) the rows this check itself names are of the kind it needs`, wrong.map(([r, k]) => `${r} is '${kind(r)}', the check wants '${k}': pick another row in fold-check.ts`).join(' · ')); }
{ const r = await child([ME, '--rows'], {}), R = jsons<{ settled: number; missing: string[]; off: string[]; own: string[]; yesNo: number; unlisted: string[] }>(r.out, 'rows')[0];
  line(!!R && R.settled === settled.length && R.settled > 0 && !R.missing.length && !R.off.length && !R.own.length, `(1) the list's ${settled.length} settled rows are in the knob tables, plain values, and at the list's value on a game the page's stand stood`,
    R ? `not in the tables: ${few(R.missing)} · not at the list's value: ${few(R.off)} · with a getter or setter of their own: ${few(R.own)}` : said(r));
  line(!!R && !R.unlisted.length, `(2) every yes/no row of the knob tables is on the list (${R?.yesNo ?? '?'} rows), so a new switch says its kind`,
    R ? `on no list: ${few(R.unlisted, 12)}${R.unlisted.length ? ' — a new rule\'s switch joins by `bun run tools/qa/switch-list.ts new`' : ''}` : 'no reading'); }

{ const r = await child([ME, '--fold'], { QA_FOLD: '1' }), R = jsons<any>(r.out, 'result')[0], c: Refused[] = R?.counted ?? [];
  if (!R) { for (const n of [3, 4, 5, 6]) line(false, `(${n}) the fold, in a child`, said(r)); }
  else {
    line(R.laid === settled.length && !R.skipped.length && !R.missing.length && R.reads === true && c.length === 1 && c[0].row === R.A && c[0].n === 2 && /^tools\/qa\/fold-check\.ts:\d+$/.test(c[0].writer) && R.open === true,
      `(3) under the fold ${R.A} answers its page value whatever is written, two writes of the other value are one count of 2 by this file, the page's own value counts nothing, and ${R.O} (not settled) is still a switch`,
      `laid ${R.laid} of ${settled.length} · reads its page value ${R.reads} · counted ${JSON.stringify(c)} · the open row flips ${R.open}`);
    line(!R.fenceLost.length && !R.fenceLost2.length && R.fenceReads === true && R.fenceSaid === true && (R.fenceNamed as string[]).some(x => x.startsWith(`${R.N} (was at`)) && R.fenceRemade === true, `(4) the runner's fence round a probe (every table copied and put back) leaves the fold standing, and says the probe's line where nobody above does; a branch replaced whole inside it is named in that line; one deleted inside it is held again once the fence has made it again`,
      `after a copy and put-back, holds lost: ${few(R.fenceLost)} · after asProbe: ${few(R.fenceLost2)} · the rows read their page value ${R.fenceReads} · the fence said the probe's line with its two writes ${R.fenceSaid} · a branch replaced inside the fence: ${few(R.fenceNamed)} · a branch deleted inside it is held again ${R.fenceRemade}`);
    const lost: string[] = R.lost, branch = String(R.N).slice(0, String(R.N).lastIndexOf('.') + 1);
    line(R.plainAgain === true && lost.some(x => x.startsWith(`${R.N} (was at`)) && lost.every(x => x.startsWith(branch)) && R.laidAgain === true && !R.lostAfter.length, `(5) ${R.N}'s branch replaced whole: the row is plain again, that is found and named with the value it was left at, and it is laid again`,
      `plain after the replace ${R.plainAgain} · found: ${few(lost)} · reads its page value after ${R.laidAgain} · with the old branch back: ${few(R.lostAfter)}`);
    line(!R.stood.threw && R.stood.hard === 'hard' && R.stood.standard === 'standard' && !R.stood.refused.length && !R.stood.lost.length, `(6) the page's own code writes no settled row off its value: a game stood and stepped, Hard and back, each of the camera's modes, under the fold, is refused nothing`,
      `${R.stood.threw ? `threw: ${R.stood.threw} · ` : ''}Hard reads '${R.stood.hard}', back to '${R.stood.standard}' · refused: ${refusedText(R.stood.refused)} · holds lost: ${few(R.stood.lost)}`); } }

{ const run = resolve(import.meta.dir, 'probes.ts'), A = settledYesNo(L, 2)!;
  const page = await child([run, 'selftest-fold'], { QA_FOLD: '1' }), hand = await child([run, 'selftest-pass'], { QA_FOLD: '1', ONEWAY: '0', GLASSFX: 'off' }), old = await child([run, 'selftest-fold'], { QA_FOLD: '1', TPS_PROFILE: 'frozen' }), plain = await child([run, 'selftest-fold'], {});
  type Line = { probe?: string; outside?: string; red?: boolean; refused: Refused[] };
  const P = jsons<Line>(page.out, 'fold'), own = P.find(x => x.probe === 'selftest-fold'), w = (own?.refused ?? []).map(x => `${x.row} @${x.writer.replace(/:\d+$/, '')} ×${x.n}`);
  const H = jsons<Line>(hand.out, 'fold'), before = H.find(x => x.outside === 'before selftest-pass'), hw = (before?.refused ?? []).map(x => `${x.row}=${String(x.value)} @${x.writer.replace(/:\d+$/, '')}`);
  const ok = P.length === 1 && !!own && own.red === false && w.length === 2 && w[0] === `${A.row} @tools/qa/fold-check.ts ×1` && w[1] === `${A.row} @tools/qa/probes.ts ×1` && /^THE SETTLED ROWS ARE FOLDED: /m.test(page.err);
  const laid = (e: string): string => /^THE SETTLED ROWS ARE FOLDED: .*$/m.exec(e)?.[0] ?? '', okHand = H.length === 2 && hw.length === 1 && hw[0] === 'DOORWAY.oneWay=false @tools/qa/headless.ts' && !H.find(x => x.probe === 'selftest-pass')!.refused.length && /GLASSFX\.on=false/.test(laid(hand.err)) && !/GLASSFX/.test(laid(page.err));
  line(ok && okHand && !jsons(old.out, 'fold').length && /^THE SETTLED ROWS ARE NOT FOLDED: /m.test(old.err) && !jsons(plain.out, 'fold').length && !/SETTLED ROWS/.test(plain.err),
    `(7) the runner under QA_FOLD=1 says one line for the probe with its own write and its child's; a word given by hand over a settled row is said before the first probe, or in the notice where its module read it before the fold; on the old stand nothing is folded and that is said; with no word nothing is said`,
    `the probe: ${own ? few(w) : 'no line'} · ONEWAY=0 by hand: ${before ? few(hw) : 'no line before the probe'} · GLASSFX=off by hand (read as its module loads): ${/GLASSFX\.on=false/.test(laid(hand.err)) ? 'named in the notice' : 'NOT named ✗'} · the old stand: ${jsons(old.out, 'fold').length ? 'a line ✗' : 'no line'}, ${/NOT FOLDED/.test(old.err) ? 'said' : 'NOT said ✗'} · no word: ${jsons(plain.out, 'fold').length ? 'a line ✗' : 'no line'}`); }

{ const F = await import('./fold.ts'), top = await child([ME, '--write'], { QA_FOLD: '1' }), offNow = /reads it from now on: (.*)$/m.exec(top.err)?.[1] ?? '';
  type Line = { probe?: string; process?: string; refused: Refused[] };
  const main = await child([ME, '--main'], { QA_FOLD: '1' }), M = jsons<Line>(main.out, 'fold'), mw = (M[0]?.refused ?? []).map(x => x.writer.replace(/:\d+$/, ''));
  const alone = jsons<Line>(top.err, 'fold'), other = await child([ME, '--write'], { QA_FOLD: '1', QA_FOLD_SAID: 'folded', TPS_PROFILE: 'frozen' });
  const word = await child([ME, '--write'], { QA_FOLD: '1', QA_FOLD_SAID: 'folded', QA_FOLD_OFF: offNow, GLASSFX: 'off' }), W = jsons<Line>(word.err, 'fold')[0];
  const fresh = await child([resolve(import.meta.dir, 'switch-list.ts'), 'new'], { TPS_GEAR: '0' });
  const asks = (e: Record<string, string>): string => { try { return String(F.foldWanted(e)); } catch { return 'stops'; } };
  const words = [asks({}), asks({ QA_FOLD: '0' }), asks({ QA_FOLD: '1' }), asks({ QA_FOLD: 'on' }), asks({ TPS_GAME: 'rust' }), asks({ TPS_GAME: 'audit' }), asks({ TPS_GAME: 'table' }), asks({ TPS_GAME: 'ts' })].join(' ');
  line(M.length === 1 && M[0].probe === 'selftest-fold' && mw.length === 2 && mw.every(x => x === 'tools/qa/fold-check.ts') && !jsons(main.err, 'fold').length
      && !jsons(top.out, 'fold').length && alone.length === 1 && alone[0].process === 'tools/qa/fold-check.ts' && alone[0].refused.length === 1
      && !/SETTLED ROWS/.test(other.err) && !jsons(other.err, 'fold').length && other.code === 0 && fresh.code === 2
      && /this child was given a word its parent was not: GLASSFX\.on=false/.test(word.err) && !!W && W.refused.some(x => x.row === 'GLASSFX.on' && x.writer === '(a word read before the fold)')
      && words === 'false false true stops true true true false',
    `(8) who speaks: a check file's own main run alone says its probe's line with its child's write in it; a process alone says its own on stderr; an arm on the old stand under a folded parent is not folded and says nothing; a child given a word its parent was not has the row counted; the words that ask are QA_FOLD=1 and TPS_GAME = rust, audit or table; switch-list.ts new refuses a word by hand`,
    `a main alone: ${M.length} line(s), writers ${few(mw)} · a process alone: ${alone.length} line(s) on stderr, ${jsons(top.out, 'fold').length} on stdout · an old-stand arm of a folded parent: ${!/SETTLED ROWS/.test(other.err) && !jsons(other.err, 'fold').length ? 'silent, nothing counted' : 'spoke ✗'} · switch-list.ts new under TPS_GEAR=0: exit ${fresh.code} · GLASSFX=off given to a child: ${W ? refusedText(W.refused.filter(x => x.row === 'GLASSFX.on')) : 'no line'} · nothing / QA_FOLD=0 / 1 / on / TPS_GAME=rust / audit / table / ts: ${words}`); }

console.log(fails ? `=== fold-check: ${fails} FAILED ===` : '=== fold-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
