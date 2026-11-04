// THE PRICING RUN'S JUDGEMENT, held to made-up rows (src/ui/pricerun.ts): the one place a GPU number is judged (priceOver), what counts as a display's wait in
// the stamps, a row that is not one picture or whose frames were queued ahead of the GPU left unjudged, the report's text, the addresses a run builds and the words it names
// as left out, and the words a mission's link sheds. No browser, no GPU: tools/qa/pricerun-page.cjs proves the chain of pages, this proves what is said of
// the numbers.   bun run tools/qa/pricerun-check.ts   (exit 0 = all pass; PRICERUN_BREAK=1 puts the bar out of reach and must fail)
import { PRICE, priceOver, priceReport, priceLeft, priceLevels, priceLookWords, priceSearch, rungDiffers, spansWait, queuedWords, type PriceFile, type PriceRow, type PriceWin, type PriceRung } from '../../src/ui/pricerun';
import { quitSearch } from '../../src/scene/scenarioList';
const BREAK = (globalThis as any).process?.env?.PRICERUN_BREAK === '1';
if (BREAK) PRICE.gpuMore = 9;
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
const f: PriceFile = { version: 6, what: 'made up', at: '2026-11-04T06:00:00.000Z', startedAt: '2026-11-04T05:50:00.000Z', address: '/?look', look: ['look'], userAgent: 'unit', gpu: 'made up', machine: { webgpu: 'made up' } as never, screen: '1920×1080', dpr: 1, quality: 'high', secs: 3, bar, rows, notes: [] };
const rep = priceReport(f);
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
check(priceLevels().length > 20 && new Set(priceLevels().map(l => l.level)).size === priceLevels().length, `the levels a run walks are the list's, none twice (${priceLevels().length})`);
{ const s = priceSearch(first, 3, 'high', ['look', 'geo=0'], ['mute', 'downloads=1', 'quality=low']), q = new URLSearchParams(s);
  check(q.has('look') && q.get('geo') === '0' && q.get('downloads') === '1' && q.get('quality') === 'high' && q.getAll('mute').length === 1 && q.getAll('quality').length === 1 && q.get('pricerun') === '3' && q.get('seed') === '1' && q.has('noverdict') && q.has('nokeep'),
    `a page's address carries the look page's words and the visitor's flags once each, and its own preset wins (${s})`); }
{ const w = priceLookWords('?pricelevels&geo=0'), w2 = priceLookWords('?look=walls:0&pricelevels');
  check(w.includes('look') && w.includes('geo=0') && w2.includes('look=walls:0') && !w2.includes('look'), `the look page's words ride with or without a look word beside them (${w.join('&')} · ${w2.join('&')})`); }
{ const search = '?look&pricelevels=pass&pricesecs=2&nosharp&hour=dusk&mute&downloads=1', left = priceLeft(search, first, 'high', priceLookWords(search), ['mute', 'downloads=1']);
  check(left.join(',') === 'nosharp,hour', `the words of the asking address that no page carries are named, the run's own are not (${left.join(', ')})`); }
check(quitSearch('?pricelevels=nosuch&pricesecs=2&pricepage=0&pricerun=4&mute') === '?mute', `a mission's link sheds every word of a pricing run (${quitSearch('?pricelevels=nosuch&pricesecs=2&pricepage=0&pricerun=4&mute')})`);

console.log(red ? `\n=== pricerun-check: ${red} FAILED${BREAK ? ' (PRICERUN_BREAK=1: as it must)' : ''} ===` : '\n=== pricerun-check: ALL PASS ===');
process.exit(red ? 1 : 0);
