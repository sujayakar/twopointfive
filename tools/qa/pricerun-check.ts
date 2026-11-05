// THE PRICING RUN'S JUDGEMENT, held to made-up rows (src/ui/pricerun.ts): the one place a GPU number is judged (priceOver), what counts as a display's wait in
// the stamps, a row that is not one picture or whose frames were queued ahead of the GPU left unjudged, the report's text, the addresses a run builds and the words it names
// as left out, and the words a mission's link sheds. No browser, no GPU: tools/qa/pricerun-page.cjs proves the chain of pages, this proves what is said of
// the numbers.   bun run tools/qa/pricerun-check.ts   (exit 0 = all pass; PRICERUN_BREAK=1 puts the bar out of reach and must fail; PRICERUN_BREAK=late puts
// the late answer's rule off and must fail on its cases alone)
import { PRICE, priceOver, priceReport, priceLeft, priceOverruled, priceLevels, priceLookWords, priceSearch, rungDiffers, spansWait, queuedWords, answerOf, lateRows, type PriceFile, type PriceRow, type PriceWin, type PriceRung } from '../../src/ui/pricerun';
import { quitSearch } from '../../src/scene/scenarioList';
const WORD = (globalThis as any).process?.env?.PRICERUN_BREAK, BREAK = WORD === '1' || WORD === 'late';
if (WORD === '1') PRICE.gpuMore = 9; if (WORD === 'late') PRICE.lateMs = 0;
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };

const win = (gpu: number | null, all: number | null, work: number, ms = 16.7): PriceWin => ({ frames: 360, ms, p95: ms, gpu, all, work, lag: +(work / ms).toFixed(2), passes: gpu === null || all === null ? null : { gather: 1.4, gbuffer: +(all - gpu - 0.09).toFixed(2), post: 0.05, fxaa: 0.04 } });
const rung = (o: Partial<PriceRung> = {}): PriceRung => ({ preset: 'high', adaptive: false, scale: 0.77, cap: 1080, gatherThird: false, volQuarter: false, sharp: true, ...o });
const bar: PriceFile['bar'] = { gpuMore: PRICE.gpuMore, frameMs: PRICE.frameMs, lagFrames: PRICE.lagFrames, ruled: 'made up' };
const ratio = (a: number | null, b: number | null) => a !== null && b ? +(a / b - 1).toFixed(4) : null;
const add = (on: PriceWin, base: PriceWin) => on.all === null || base.all === null || !base.gpu || spansWait(on) || spansWait(base) ? null : +((on.all - base.all) / base.gpu).toFixed(4);   // (as the run computes it)
const row = (level: string, plain: PriceWin, off: PriceWin, on: PriceWin, o: Partial<PriceRow> = {}): PriceRow => {
  const r: PriceRow = { level, mission: level, title: level, hour: 'night', query: '?x', quality: 'high', scale: 0.77, internal: [1368, 770], out: [1920, 1080], rung: rung(), off, on, heldAt: 4, gpuMore: ratio(on.gpu, off.gpu), allMore: add(on, off), msMore: 0,
    page: { win: plain, quality: 'high', scale: 0.77, internal: [1368, 770], rung: rung(), heldAt: 4, programs: { boot: 128, after: 24 }, boot: '' }, gpuOverPage: ratio(on.gpu, plain.gpu), allOverPage: add(on, plain), msOverPage: 0, offBy: '26 layers of the look put off', programs: { boot: 120, after: 4, bySwitch: 0 }, boot: '', notes: [], apart: [], over: [], ...o };
  if (!o.apart) r.apart = [...queuedWords(off, 'the layers off'), ...queuedWords(on, 'the layers on'), ...queuedWords(plain, 'the page without the look')];   // (as the run writes a row: frames queued ahead of the GPU in any state put it apart)
  return { ...r, over: r.apart.length ? [] : priceOver(r, bar) }; };   // (as the run judges a row: one that is apart is not judged)

// ---- the verdict
const under = row('under', win(10, 10.3, 19.5), win(10.2, 10.6, 19.6), win(10.5, 11.1, 19.8));
const gpuonly = row('gpuonly', win(10, 10.3, 19.5), win(11, 11.4, 19.6), win(11.5, 12, 19.8));
const surface = row('surface', win(10, 10.3, 19.5), win(10.2, 10.8, 19.6), win(10.6, 12.5, 19.8));          // +6 % GPU, but +2.2 ms with the g-buffer = +22 % of the baseline's GPU ms
const behind = row('behind', win(10, 10.3, 19.5), win(10.2, 10.6, 19.6), win(14.5, 15.1, 204.4));            // 12.2 frames late with the look on: frames were queued, the row is not judged
const lag3 = row('lag3', win(10, 10.3, 19.5), win(10.2, 10.6, 19.6), win(10.5, 11.1, 50.1));                  // 3.00 frames: a page waiting for the GPU, no line of the verdict
const weak = row('weak', win(15, 15.4, 60, 20.0), win(15.1, 15.5, 61, 20.2), win(15.3, 15.8, 62, 20.4));     // a machine that holds neither sixty nor the GPU even without the look
const metal = row('metal', win(5, 16.5, 19.5), win(5.2, 16.6, 19.6), win(5.4, 16.6, 19.8));                   // the g-buffer's stamp padded to the display's interval: +8 % GPU, the second span says nothing
const rows = [under, gpuonly, surface, lag3, weak, metal];
check(under.over.length === 0, `within the bar on both numbers: no verdict (${under.over.join('; ') || 'none'})`);
check(gpuonly.over.length === 2 && /on the GPU the look adds \+15\.0%/.test(gpuonly.over[0]) && /with the g-buffer counted the look adds 1\.70 ms, \+17\.0%/.test(gpuonly.over[1]), `over on the game's own number, and with the g-buffer counted: ${gpuonly.over.join('; ')}`);
check(surface.over.length === 1 && /with the g-buffer counted the look adds 2\.20 ms, \+22\.0% of the GPU's time/.test(surface.over[0]), `a cost that is all in the g-buffer is caught by the second span alone: ${surface.over.join('; ')}`);
check(behind.over.length === 0 && behind.apart.length === 1 && /the layers on: the GPU's "done" came 12\.2 frames after a frame began \(past 8: frames were queued ahead of it/.test(behind.apart[0]) && priceOver({ ...behind, apart: [] }, bar).length === 2, `a state whose "done" comes 12 frames late is not judged, though its numbers would be over the bar: ${behind.apart.join('; ')}`);
check(lag3.on.lag === 3 && lag3.over.length === 0 && lag3.apart.length === 0, `done ${lag3.on.lag} frames after, a page that waits for the GPU, is judged and is no line of the verdict (${lag3.over.join('; ') || 'no verdict'})`);
{ const fast = row('fast', win(5.0, 6.2, 50, 8.33), win(5.1, 6.4, 50, 8.33), win(5.2, 6.5, 50, 8.33));
  check(fast.on.lag === 6 && fast.over.length === 0 && fast.apart.length === 0, `a 120 Hz display whose "done" comes ${fast.on.lag} of its short frames late is judged, and is under the bar (${fast.over.join('; ') || 'no verdict'})`); }
check(weak.over.length === 1 && /a frame comes every 20\.4 ms with the look on, 20\.0 ms on the page without the look: neither holds sixty a second/.test(weak.over[0]), `a machine at the bar without the look: the frame time is said for both and the look is not blamed: ${weak.over.join('; ')}`);
// ---- the GPU's answer late for no work of the state's own (PRICE.lateMs): the numbers are a rented card's, 11-05, a walk of 29 levels in one browser
{ const early = row('early', win(6.27, 6.42, 14.7, 16.8), win(6.3, 6.52, 15.1, 17.0), win(6.35, 6.92, 19.1, 16.95)), mid = row('mid', win(9.24, 9.48, 26.3, 16.85), win(9.3, 9.62, 26.9, 16.85), win(9.32, 10.35, 27.6, 16.9)),
    busy = row('busy', win(14.94, 15.23, 26.3, 16.71), win(15.0, 15.28, 27.0, 16.9), win(22.43, 23.34, 43.6, 25.67)),          // the GPU full with the look on: two frames of its own passes explain its 44 ms
    tired = row('tired', win(8.89, 9.11, 47.55, 30.08), win(6.9, 7.54, 36.0, 25.35), win(7.99, 8.77, 37.5, 25.67));             // the look page reads CHEAPER than the page without it: the walk's 24th level
  const got = lateRows([early, mid, busy, tired]), T = got.rows[3];
  check(got.least === answerOf(early.page!.win) && got.rows.slice(0, 3).every((r, i) => r === [early, mid, busy][i]) && T.apart.length === 2 && T.over.length === 0 && /^the layers off: /.test(T.apart[0]) && /the page without the look: the GPU's "done" came 48 ms after a frame began for 9\.1 ms of passes, 21 ms more than 2 frames of its own passes and this run's least answer \(8 ms\) explain \(past 12:/.test(T.apart[1]),
    `a state whose answer comes late against the run's own least puts its row apart, and no other row (least answer ${got.least} ms): ${T.apart[1] ?? T.apart[0] ?? 'not set apart'}`);
  // a machine whose every answer is slow (a fast display, a browser whose "done" is a few frames long) is not a late state: nothing stands apart from its own least
  const slow = [row('a', win(5, 5.2, 41, 8.4), win(5.1, 5.3, 42, 8.4), win(5.4, 5.9, 44, 8.4)), row('b', win(9, 9.4, 50, 16.7), win(9.1, 9.5, 52, 16.7), win(9.6, 10.2, 55, 16.7))], gs = lateRows(slow);
  check(gs.rows.every((r, i) => r === slow[i]) && gs.least !== null && gs.least > 2 * (1000 / 60), `a machine whose answer takes ${gs.least} ms at the least, on every state, has no late row`);
  // … and so is a machine three and four frames behind on every state (the verdict's own made-up rows `weak` and `weaker`, each a machine of its own above)
  { const w3 = row('w3', win(15, 15.4, 60, 20.0), win(15.1, 15.5, 61, 20.2), win(15.3, 15.8, 62, 20.4)), w4 = row('w4', win(16.5, 17.0, 80, 20.0), win(16.6, 17.1, 81, 20.2), win(16.8, 17.3, 82, 20.4)), g = lateRows([w3]), g4 = lateRows([w4]);
    check(g.rows[0] === w3 && g4.rows[0] === w4, `a machine whose "done" is three or four frames behind on all its states is judged by its frame's time, as before (least answers ${g.least} and ${g4.least} ms)`); }
  // a walk of look pages only: two states a row. The tired level above is caught by them at this bar (its layers off stand 13 over), and says so with one line
  { const two = (r: PriceRow): PriceRow => ({ ...r, page: undefined }), g2 = lateRows([two(early), two(mid), two(tired)]);
    check(g2.rows[2].apart.length === 1 && /^the layers off: /.test(g2.rows[2].apart[0]) && g2.rows[0].apart.length === 0, `on a walk with no page without the look the tired level is still set apart: ${g2.rows[2].apart[0] ?? 'not set apart'}`); }
  // where the g-buffer's stamp holds the display's wait, the passes that count are the compute passes: such a state neither hides a late answer nor makes the run's least
  { const held = (work: number) => win(5, 16.5, work), fresh = row('fresh', held(19.5), held(19.6), held(19.8)), late = row('late', held(52), held(50), held(51)), g = lateRows([fresh, late]);
    check(spansWait(held(19.5)) && g.least === 14.5 && g.rows[0] === fresh && g.rows[1].apart.length === 3, `a display that pads the g-buffer's stamp: the least answer is read off the compute passes (${g.least} ms) and a late state is still late (${g.rows[1].apart.length} of 3 states)`); }
  check(lateRows([early, tired], PRICE.lateMs, 1).rows[1].apart.length === 3 && lateRows([early, tired], 0).least === null, 'a page that keeps one frame with the GPU is allowed one frame of its passes, and a rule that is not asked reads nothing');
  { const f: PriceFile = { version: 6, what: '', at: '2026-11-05T00:00:00.000Z', startedAt: '', address: '/?look&pricelevels', look: ['look'], userAgent: '', gpu: 'made up', machine: { hz: 60 } as any, screen: '1×1', dpr: 1, quality: 'high', secs: 3, bar: { ...bar, lateMs: 12 }, leastAnswer: got.least!, rows: got.rows, notes: [] }, text = priceReport(f);
    check(/nor where the answer came over 12 ms later than the frames of its own passes the page keeps with the GPU and the run's least answer \(8\.3 ms\) explain/.test(text) && /tired: the layers off: the GPU's "done" came 36 ms/.test(text) && /NOT JUDG/.test(text), 'the report says the late answer\'s bar and the run\'s least, and names the row it set apart under the table'); }
  const alone = lateRows([tired]); check(alone.rows[0] === tired, 'a run of one level is read against itself, so a level that is late in all three states says nothing: the rule needs a walk');
  const blind = row('blind', win(null, null, 30), win(null, null, 30), win(null, null, 30)); check(lateRows([blind]).least === null && lateRows([blind]).rows[0] === blind, 'no stamps, nothing to read: the rows are left as they are'); }
{ const weaker = row('weaker', win(16.5, 17.0, 80, 20.0), win(16.6, 17.1, 81, 20.2), win(16.8, 17.3, 82, 20.4));
  check(weaker.over.length === 1 && /neither holds sixty a second/.test(weaker.over[0]) && !/frames after/.test(weaker.over.join(' ')), `a GPU four frames behind on both pages: the frame's time is the verdict, the lag is none (${weaker.over.join('; ')})`); }
{ const late = row('late', win(6.4, 6.6, 19.5), win(7.0, 7.2, 19.6), win(6.5, 6.8, 4300));
  check(late.over.length === 0 && late.apart.length === 1, `a "done" that comes hundreds of frames late is a queue, not a cost: the row is not judged (${late.apart.join('; ')})`); }
{ const blind = row('blind', win(null, null, 19.5), win(null, null, 19.6), win(null, null, 700));
  check(blind.over.length === 0 && blind.apart.length === 1 && /41\.9 frames/.test(blind.apart[0]), `a browser that stamps nothing and answers 42 frames late: not judged (${blind.apart.join('; ')})`); }
{ const both = row('both', win(10, 10.3, 200), win(10.2, 10.6, 19.6), win(10.5, 11.1, 19.8));
  check(both.apart.length === 1 && /^the page without the look: /.test(both.apart[0]), `frames queued on the page without the look put the row apart too (${both.apart.join('; ')})`); }

// ---- a display's wait in the stamps, and what is not one
check(spansWait(metal.on) && metal.allOverPage === null && metal.over.length === 0, `a g-buffer stamp that spans the display's wait: the second span judges nothing (${metal.over.join('; ') || 'no verdict'}; look adds ${metal.allOverPage})`);
check(!spansWait(win(14.47, 15.89, 117.1)), 'a heavy state on a rented card (15.89 of 16.7 ms stamped, the compute passes 14.47) is NOT taken for a display\'s wait');
check(!spansWait(win(5.6, 7.9, 10, 8.33)) && !spansWait(win(6.0, 8.3, 10, 8.33)), 'a 120 Hz display with an honest 2.3 ms g-buffer on a page that fills its 8.3 ms frame is NOT taken for a display\'s wait');
{ const hz120 = row('hz120', win(5.0, 6.2, 10, 8.33), win(5.1, 6.4, 10, 8.33), win(5.6, 7.9, 10, 8.33));
  check(hz120.allOverPage !== null && hz120.over.some(t => /with the g-buffer counted the look adds 1\.70 ms/.test(t)), `… and its row is judged with the g-buffer counted (${hz120.over.join('; ')})`); }

// ---- a row that is not one picture is not judged
const apartRow = row('apart', win(10, 10.3, 19.5), win(11, 11.4, 19.6), win(13, 14, 19.8), { apart: ['the page without the look was held at 5.3 s of the game\'s clock, the look page at 4 s: not the same step'] });
check(apartRow.over.length === 0 && priceOver({ ...apartRow, apart: [] }, bar).length > 0, 'a row whose two pages stood on different steps has no verdict, though its numbers would be over the bar');
check(rungDiffers(rung(), rung({ scale: 0.75, gatherThird: true })).join(', ') === 'scale 0.77 / 0.75, gatherThird false / true' && rungDiffers(rung(), rung({ adaptive: null })).length === 0, 'two rungs are told apart lever by lever (the switch itself is not a lever of the picture)');

// ---- the report
const f: PriceFile = { version: 6, what: 'made up', at: '2026-11-04T06:00:00.000Z', commit: 'abcdef0123456789', startedAt: '2026-11-04T05:50:00.000Z', address: '/?look', look: ['look'], userAgent: 'unit', gpu: 'made up', machine: { webgpu: 'made up' } as never, screen: '1920×1080', dpr: 1, quality: 'high', secs: 3, bar, rows, notes: [] };
const rep = priceReport(f);
check(/UTC  ·  build abcdef01\n/.test(rep) && !/build/.test(priceReport({ ...f, commit: undefined }).split('\n')[0]), 'the report\'s first line names the build the pages were, where the page knew it');
{ const L = rep.split('\n'), h = L.findIndex(l => l.startsWith('level')), pipes = (l: string) => [...l].flatMap((c, i) => c === '|' ? [i] : []).join(',');
  check(h > 0 && rows.every((_, k) => pipes(L[h + 1 + k]) === pipes(L[h])), `the head and every line have their bars in the same places (${pipes(L[h])})`); }
check(/EVERY PAGE stood on its preset's own rows with the quality ladder off: high ×0\.77, at most 1080 rows, gather at a half, volumetrics at a half, sharp on\./.test(rep), 'the report says the one rung every page stood on');
check(/ON THIS MACHINE the g-buffer's stamp spans the wait for the display/.test(rep), 'a file with a wait in its stamps says so in the legend');
check(/Over the bar:\n  gpuonly: /.test(rep) && /past the bar \(made up\)/.test(rep), 'the levels over the bar are named under the table, and the legend says which rows of the bar are ruled');
check(/THE PAGES STOOD ON 2 RUNGS though the ladder was off/.test(priceReport({ ...f, rows: [{ ...under, page: { ...under.page!, rung: rung({ scale: 0.75 }) } }] })), 'two rungs in a file are called out');
{ const auto = priceReport({ ...f, rows: [{ ...under, rung: rung({ preset: 'auto (high)', adaptive: true }), page: { ...under.page!, rung: rung({ preset: 'auto (high)', adaptive: true }) } }] });
  check(/THE QUALITY LADDER WAS LEFT ON on 2 of 2 pages/.test(auto) && !/EVERY PAGE stood/.test(auto), 'a run whose ladder was left on says that, and does not also say every page was held'); }
{ const none = priceReport({ ...f, rows: [], notes: ['pass: not priced — the mission did not start'] }); check(/No level was priced\./.test(none) && !/RUNG|EVERY PAGE|No level is over/.test(none), 'a run that priced nothing says so, and says nothing of rungs or of the bar'); }
{ const some = priceReport({ ...f, rows: [under, apartRow] }), all = priceReport({ ...f, rows: [apartRow] });
  check(/NOT JUDGED/.test(some.split('\n').find(l => l.startsWith('apart')) ?? '') && /No level that was judged is over the bar\./.test(some) && /Not judged, because the row is not one picture[^\n]*\n  apart: the page without the look was held at 5\.3 s/.test(some) && !/No level is over the bar\./.test(some), 'a row that is not one picture is marked NOT JUDGED on its line and named under the table, and the report does not say "no level is over"');
  check(/No level was judged\./.test(all), 'a run whose every row is apart says no level was judged'); }

// ---- the addresses a run builds, and the words it names as left out
const first = priceLevels()[0]?.level ?? '';
check(priceLevels().length > 1 && new Set(priceLevels().map(l => l.level)).size === priceLevels().length, `the levels a run walks are the list's, none twice (${priceLevels().length})`);
{ const s = priceSearch(first, 3, 'high', ['look', 'geo=0'], ['mute', 'downloads=1', 'quality=low']), q = new URLSearchParams(s);
  check(q.has('look') && q.get('geo') === '0' && q.get('downloads') === '1' && q.get('quality') === 'high' && q.get('adapt') === '0' && q.get('giveway') === '0' && q.getAll('mute').length === 1 && q.getAll('quality').length === 1 && q.get('pricerun') === '3' && q.get('seed') === '1' && q.has('noverdict') && q.has('nokeep'),
    `a page's address carries the look page's words and the visitor's flags once each, and its own preset wins (${s})`); }
{ const a = new URLSearchParams(priceSearch(first, 0, 'auto', ['look'], ['adapt=0', 'giveway=1']));
  check(!a.has('quality') && a.getAll('adapt').length === 1 && a.get('giveway') === '0' && a.getAll('giveway').length === 1, `asked at auto, a page pins no preset, and the window's word is the run's own whatever the visitor's (${a.toString()})`); }
{ const w = priceLookWords('?pricelevels&geo=0'), w2 = priceLookWords('?look=walls:0&pricelevels');
  check(w.includes('look') && w.includes('geo=0') && w2.includes('look=walls:0') && !w2.includes('look'), `the look page's words ride with or without a look word beside them (${w.join('&')} · ${w2.join('&')})`); }
{ const search = '?look&pricelevels=pass&pricesecs=2&nosharp&hour=dusk&mute&downloads=1', left = priceLeft(search, first, 'high', priceLookWords(search), ['mute', 'downloads=1']);
  check(left.join(',') === 'nosharp,hour', `the words of the asking address that no page carries are named, the run's own are not (${left.join(', ')})`); }
{ const search = '?look&pricelevels&adapt=1&giveway=1&seed=7&mute', over = priceOverruled(search, first, 'high', priceLookWords(search), ['mute']);
  check(over.length === 3 && /^adapt=1 \(the pages have adapt=0\)$/.test(over[0]) && /^giveway=1 \(the pages have giveway=0\)$/.test(over[1]) && /^seed=7 \(the pages have seed=1\)$/.test(over[2]) && priceOverruled('?look&pricelevels&quality=high&mute', first, 'high', ['look'], ['mute']).length === 0, `a word the asking address says and the run says otherwise is named with both values, one the run says the same is not (${over.join('; ')})`); }
{ const from = priceSearch(first, 3, 'high', ['look'], ['mute']), left = quitSearch(from), q = new URLSearchParams(left);
  check(!q.has('adapt') && !q.has('giveway') && !q.has('pricerun') && quitSearch('?adapt=0&giveway=0&mute') === '?adapt=0&giveway=0&mute', `a mission picked from a priced page sheds the held picture's words with the run's own; a visitor's own stay on a page that is no run's (${left})`); }
check(quitSearch('?pricelevels=nosuch&pricesecs=2&pricepage=0&pricerun=4&mute') === '?mute', `a mission's link sheds every word of a pricing run (${quitSearch('?pricelevels=nosuch&pricesecs=2&pricepage=0&pricerun=4&mute')})`);

console.log(red ? `\n=== pricerun-check: ${red} FAILED${BREAK ? ` (PRICERUN_BREAK=${WORD}: as it must)` : ''} ===` : '\n=== pricerun-check: ALL PASS ===');
process.exit(red ? 1 : 0);
