// PRICE EVERY LEVEL (`?pricelevels` on the look page's address): one address, hands off, and the machine it ran on has said what the look costs it on
// every level the mission list draws (the maintainer, 11-03: "we'll want to dress up all the levels and check their budget"; 11-02: "the main thing i'm concerned
// about before merging is performance", "we're going to need to run the tests across more than just the pass"). A level is only ever loaded at boot
// (scene/scenarioList.ts), so a run is a chain of page loads, as the benchmark's is a page a scene (ui/benchrun.ts): a small record in the tab's own session store
// carries the rows measured so far from one page to the next. Nothing is sent anywhere: the last page shows the table in a box with a Copy button and the whole
// run lands in the browser's downloads as ONE file (price-levels-<date>.json).
//   WHAT A LEVEL'S TWO PAGES DO. First its mission's own address ON THE LOOK PAGE, at the hour its row opens at, the die pinned: it plays a few seconds so the
//   men stand where the mission puts them, then HOLDS THE WORLD STILL and times that one picture with the look's layers off and on in turn (off, on, off, on):
//   the frame's time, and where the browser stamps the passes, the GPU's. Then the same mission on the page WITHOUT the look (no `look` on the address), held at
//   the same moment and timed as it stands. Then the next level.
//   TWO MEANINGS OF "LOOK OFF", both in the file:
//     · THE PAGE WITHOUT THE LOOK is the budget's own ("GPU time within 10% of look off"): the game as it is drawn today. It is another page: other shader
//       text, and the level as its file has it (none of the look's round things and broken wall tops). Its row is `page`; the table's OVER is read against it.
//     · THE LAYERS OFF is the same page, shaders, shapes, camera, quality and scale, with every row the look panel lists as a layer with a cost of its own
//       (ui/lookpanel.ts hands them to the sweep as `look.<row>`) at its own off value, all at once. It says what the layers cost where they run, and it is NOT
//       the panel's "plain boxes", which leaves the lighting's, the smoke's and the eye's layers on.
//   WHAT IT DOES NOT SAY. A fight's frame (the picture is held at the mission's start) and a first visit's compile (ui/boot.ts times that; each row keeps the
//   boot's own line as its page had it).
//   THE WORDS. `?pricelevels` = every level; `?pricelevels=pass,block` = these; `&pricesecs=N` = N seconds measured a turn (default 3); `&quality=<preset>` =
//   the preset every page stands at (default high: the budget is said for High; `auto` lets each page settle its own, and the row says where it settled);
//   `&pricepage=0` = the look page only, half the loads.
//   A reload in the middle does not start it again (the page finds its own mark and is a plain mission page); Esc stops a run.
// THE PAGE'S HALF IS ONE LINE (main.ts, before the boot): `if (priceLevelsPage(q)) return;`. Everything else is asked of what the page already puts on `window`
// for its checks (__ready, __engine, __quality, __perfsweep, __boot): this file adds no hook and no state to the game.
import { ALL_MISSIONS } from '../scene/scenarioList';
import { scenarioQuery } from '../scene/scenarios';
import { benchView } from './bench';

const KEY = 'twopointsix.pricelevels';
/** the word that asks for a run · the word on a level's page of one: its place in the run */
export const PRICE_KEY = 'pricelevels', PRICE_RUN_KEY = 'pricerun';
/** the render passes: their stamps span the wait for the drawable, so the GPU's busy span is taken over the others (as ui/perfrec.ts takes it) */
const DRAWN = ['gbuffer', 'composite', 'post', 'fxaa', 'pdraw', 'present'];
export const PRICE = {
  /** seconds the mission plays before its picture is held */ playS: 4,
  /** a turn: the start that is dropped, then the part that is measured (seconds; `pricesecs` on the address is the second) */ settleS: 1.2, measureS: 3,
  /** … and each wants this many frames, and gives up waiting for them after `capS` (a machine at two frames a second) */ settleFrames: 12, measureFrames: 30, capS: 25,
  /** turns of each state */ rounds: 2,
  /** a frame longer than this (seconds) is the tab having been away: the turn starts over, at most `retries` times */ awayS: 1.5, retries: 3,
  /** THE PROPOSED BAR the table sets each level against (the look task's, 11-02; the maintainer has not ruled): the GPU's time with the layers on within this of off */ gpuMore: 0.10,
  /** … and a frame under this on High (ms: sixty frames a second) */ frameMs: 16.6,
};

/** one state's turns on one page */
export interface PriceWin { frames: number; ms: number; p95: number; /** the GPU's busy span over the compute passes, ms (null: the browser stamps none) */ gpu: number | null; /** each pass's own mean, ms */ passes: Record<string, number> | null }
export interface PriceRow {
  level: string; mission: string; title: string; hour: string; query: string;
  quality: string; scale: number; internal: [number, number]; out: [number, number];
  off: PriceWin; on: PriceWin;
  /** layers on over layers off, less one: of the GPU's span where there is one, of the frame's time */ gpuMore: number | null; msMore: number;
  /** THE PAGE WITHOUT THE LOOK, the same mission held at the same moment (absent: the run was asked for without it, or that page did not finish) */
  page?: { win: PriceWin; quality: string; scale: number; internal: [number, number]; programs: { boot: number; after: number }; boot: string };
  /** the look page with its layers on over the page without the look, less one (null: no such page, or no GPU times) */ gpuOverPage: number | null; msOverPage: number | null;
  /** how the layers were put off */ offBy: string;
  /** shader programs: made before the page came ready, after it, and by the switch itself (the budget's "shaders compile once": nothing) */ programs: { boot: number; after: number; bySwitch: number };
  /** the boot as the page timed it (ui/boot.ts line) */ boot: string;
  notes: string[];
}
/** a page of the run: a level on the look page, or on the page without it */
interface Step { level: string; look: boolean }
interface Carry { v: 1; from: string; startedAt: string; steps: Step[]; i: number; live: number; quality: string; secs: number; rows: PriceRow[]; notes: string[] }
export interface PriceFile { version: 1; what: string; at: string; startedAt: string; address: string; userAgent: string; gpu: string; screen: string; dpr: number; quality: string; secs: number; bar: { gpuMore: number; frameMs: number; ruled: false }; rows: PriceRow[]; notes: string[] }

const read = (): Carry | null => { try { const t = sessionStorage.getItem(KEY), c = t ? JSON.parse(t) as Carry : null; return c && c.v === 1 && Array.isArray(c.rows) && Array.isArray(c.steps) ? c : null; } catch { return null; } };
const write = (c: Carry): boolean => { try { sessionStorage.setItem(KEY, JSON.stringify(c)); return true; } catch { return false; } };
const clear = (): void => { try { sessionStorage.removeItem(KEY); } catch { /* no store: nothing was kept */ } };

/** THE LEVELS A RUN WALKS: every level a drawn row of the mission list plays on, in the list's order, each with the first row that plays on it */
export function priceLevels(): { level: string; mission: string; title: string; hour: string; query: string }[] {
  const seen = new Set<string>(), out: ReturnType<typeof priceLevels> = [];
  for (const e of ALL_MISSIONS) { if (e.offList) continue; const level = e.level ? e.level.replace(/\.json$/, '') : 'office'; if (seen.has(level)) continue; seen.add(level);
    out.push({ level, mission: e.id, title: e.title, hour: e.hour ?? 'night', query: scenarioQuery(e) }); }
  return out;
}
/** the address a step is priced on: its mission's own words (on the look page, or not), the die pinned, no verdict kept of the try and no replay filed (the run
 *  is nobody's play: as a benchmark's scene, ui/benchrun.ts), no sound, the preset, and its place in the run */
export function priceSearch(level: string, i: number, quality: string, look = true): string {
  const L = priceLevels().find(l => l.level === level); if (!L) throw new Error(`no level '${level}' on the mission list`);
  return `?${look ? 'look&' : ''}${L.query.slice(1)}&seed=1&noverdict&nokeep&mute${quality === 'auto' ? '' : `&quality=${quality}`}&${PRICE_RUN_KEY}=${i}`;
}

/** THE PAGE'S ONE LINE, before its boot. True = this address asked for a run and the first level's address is loading: boot nothing. False = boot as ever; on a
 *  level's page of a run under way the page's part of it starts by itself once the page is ready. */
export function priceLevelsPage(q: URLSearchParams, loc: Location = location): boolean {
  if (q.has(PRICE_KEY) && !q.has(PRICE_RUN_KEY)) {
    const all = priceLevels().map(l => l.level), asked = (q.get(PRICE_KEY) ?? '').split(',').map(s => s.trim()).filter(Boolean), levels = asked.length ? asked.filter(l => all.includes(l)) : all;
    const secs = parseFloat(q.get('pricesecs') ?? ''), quality = q.get('quality') ?? 'high', both = q.get('pricepage') !== '0';
    const steps: Step[] = levels.flatMap(level => both ? [{ level, look: true }, { level, look: false }] : [{ level, look: true }]);
    const rest = loc.search.replace(/^\?/, '').split('&').filter(p => p && !/^(pricelevels|pricesecs|pricepage)(=|$)/.test(p));   // (the address that asked, with the run's own words off it: where the card's Close goes)
    const c: Carry = { v: 1, from: loc.pathname + (rest.length ? '?' + rest.join('&') : '') + loc.hash, startedAt: new Date().toISOString(), steps, i: 0, live: -1, quality, secs: isFinite(secs) && secs > 0 ? secs : PRICE.measureS, rows: [],
      notes: asked.length && levels.length < asked.length ? [`not on the mission list, left out: ${asked.filter(l => !all.includes(l)).join(', ')}`] : [] };
    if (!levels.length || !write(c) || !read()) { console.info(levels.length ? 'price every level: this tab keeps no session store, so a run cannot be carried from page to page' : `price every level: none of ${asked.join(', ')} is on the mission list (${all.join(' ')})`); return false; }
    loc.replace(loc.pathname + priceSearch(steps[0].level, 0, quality, steps[0].look) + loc.hash);   // (the address that asked is not left in the tab's history, where Back would ask again)
    return true;
  }
  const at = q.get(PRICE_RUN_KEY); if (at === null) return false;
  const c = read(), i = Number(at);
  if (!c || c.i !== i || c.live === i || !c.steps[i]) { if (c) clear(); return false; }   // (another page's address, or a reload of a level already begun: a plain mission page)
  c.live = i; write(c);
  countPrograms();
  void leg(c, i, loc).catch(e => { clear(); console.error('price every level: the run stopped —', e); benchView()?.progress(null); });
  return false;
}

// ---- shader programs made, counted from before the device exists (the page's one line runs before its boot)
const made = { n: 0 };
function countPrograms() {
  const P = typeof GPUDevice === 'function' ? GPUDevice.prototype as unknown as Record<string, (...a: unknown[]) => unknown> : null; if (!P) return;
  for (const name of ['createComputePipeline', 'createRenderPipeline', 'createComputePipelineAsync', 'createRenderPipelineAsync']) { const f = P[name]; P[name] = function (this: unknown, ...a: unknown[]) { made.n++; return f.apply(this, a); }; }
}

/** what of the page this file asks (all of it on `window` already, for the page's checks) */
interface PageHooks {
  __ready?: Promise<unknown>; __timeScale?: number; __menu?: { isOpen?: boolean; close(): void };
  __engine?: { width: number; height: number; outWidth: number; outHeight: number; settings: { renderScale: number }; timer?: { enabled: boolean; resolved: number; lastFrame: Map<string, number>; unionMs(exclude: string[]): number } };
  __quality?: { label: string; calibrating: boolean };
  __perfsweep?: { extra?: { id: string; flip(): string | { apply(): () => void } }[] };
  __boot?: { line?: string }; __cut?: { programs?: { swap?: boolean; ready?: Promise<unknown> } };
}
const median = (a: number[]): number => { const s = [...a].sort((x, y) => x - y), n = s.length; return n ? (n % 2 ? s[(n - 1) / 2] : (s[n / 2 - 1] + s[n / 2]) / 2) : NaN; };
const frame = (): Promise<number> => new Promise(res => requestAnimationFrame(res));

/** THE LOOK'S LAYERS OFF: every row of the look that the page itself lists as a layer with a cost of its own (ui/lookpanel.ts hands them to the sweep as
 *  `look.<row>`, each with its own off value), all at once. NOT the panel's "plain boxes" (`look.all`): that puts the surfaces' layers off and leaves the
 *  lighting's, the smoke's and the eye's as they stand, so against it the look would read cheaper than it is. Returns how many rows, and the way back; null = the
 *  page has no such rows (it is not the look page, or its panel is not built) */
function layersOff(w: PageHooks): { how: string; back: () => void } | null {
  const rows = (w.__perfsweep?.extra ?? []).filter(x => x.id.startsWith('look.') && x.id !== 'look.all'), back: (() => void)[] = [];
  for (const row of rows) { const r = row.flip(); if (r && typeof r === 'object') back.push(r.apply()); }   // (a row that is off already says so in words: left as it is)
  return back.length ? { how: back.length === rows.length ? `${rows.length} layers of the look put off` : `${back.length} of the look's ${rows.length} layers put off (the rest were off already)`, back: () => { for (let i = back.length - 1; i >= 0; i--) back[i](); } } : null;
}

/** one turn: the frames of `settle` dropped, then `measure` kept. Null = the tab was away through every try */
async function turn(w: PageHooks, secs: number, stop: () => boolean): Promise<{ ms: number[]; gpu: number[]; passes: Map<string, number[]> } | null> {
  const T = w.__engine?.timer;
  for (let tries = 0; tries <= PRICE.retries; tries++) {
    const ms: number[] = [], gpu: number[] = [], passes = new Map<string, number[]>(); let away = false, last = await frame(), seen = T?.resolved ?? 0;
    const t0 = last; let kept = -1;   // (the moment the measured part began)
    for (;;) { const now = await frame(), dt = now - last; last = now; if (stop()) return null;
      if (dt > PRICE.awayS * 1000 || document.hidden) { away = true; break; }
      if (kept < 0) { if ((now - t0 >= PRICE.settleS * 1000 && ms.length >= PRICE.settleFrames) || now - t0 >= PRICE.capS * 1000) { kept = now; ms.length = 0; seen = T?.resolved ?? 0; } else ms.push(dt); continue; }
      ms.push(dt);
      if (T?.enabled && T.resolved !== seen) { seen = T.resolved; gpu.push(T.unionMs(DRAWN)); for (const [k, v] of T.lastFrame) { let a = passes.get(k); if (!a) passes.set(k, a = []); a.push(v); } }
      if ((now - kept >= secs * 1000 && ms.length >= PRICE.measureFrames) || now - kept >= PRICE.capS * 1000) return { ms, gpu, passes };
    }
    if (away) { while (document.hidden) await new Promise(r => setTimeout(r, 500)); }
  }
  return null;
}
const winOf = (ms: number[], gpu: number[], passes: Map<string, number[]>): PriceWin => { const s = [...ms].sort((a, b) => a - b);
  return { frames: ms.length, ms: +median(ms).toFixed(2), p95: +(s[Math.min(s.length - 1, Math.floor(s.length * 0.95))] ?? NaN).toFixed(2), gpu: gpu.length ? +median(gpu).toFixed(2) : null,
    passes: passes.size ? Object.fromEntries([...passes].map(([k, v]) => [k, +(v.reduce((a, b) => a + b, 0) / v.length).toFixed(3)])) : null }; };

/** THIS PAGE'S PART OF THE RUN */
async function leg(c: Carry, i: number, loc: Location) {
  const w = window as unknown as PageHooks, view = benchView(), S = c.steps[i], L = priceLevels().find(l => l.level === S.level)!, of = c.steps.length;
  Object.assign(PRICE, (window as unknown as { __priceOpts?: Partial<typeof PRICE> }).__priceOpts ?? {});   // (a check's own times: tools/qa/pricerun-page.cjs drives a run on an adapter whose frame is seconds long)
  let stopped = false; const onKey = (e: KeyboardEvent) => { if (e.code === 'Escape') stopped = true; }; addEventListener('keydown', onKey, true);
  const say = (t: string) => view?.progress(`pricing the look, page ${i + 1} of ${of}: ${L.title}${S.look ? '' : ', without the look'} — ${t}   (hands off; Esc stops)`);
  say('loading');
  while (!w.__ready) await new Promise(r => setTimeout(r, 50));
  await w.__ready; const bootMade = made.n, notes: string[] = [];
  if (w.__menu?.isOpen) w.__menu.close();   // (the plain night's start card: a mission's own address opens straight in)
  say('the mission stands up');
  for (let t0 = performance.now(); performance.now() - t0 < PRICE.playS * 1000 || w.__quality?.calibrating;) { await frame(); if (stopped) break; if (w.__quality?.calibrating && performance.now() - t0 > 60000) { notes.push('auto quality was still measuring after a minute: priced where it stood'); break; } }
  const cut = w.__cut?.programs; if (cut?.swap && cut.ready) await Promise.race([cut.ready, new Promise(r => setTimeout(r, 60000))]);   // (the second set of ray programs compiles after ready where the page has one: not charged to the switch)
  if (S.look) for (let t0 = performance.now(); !w.__perfsweep?.extra?.some(x => x.id.startsWith('look.')) && performance.now() - t0 < 5000;) await frame();   // (the look panel hands its rows to the sweep once the page's recorder is there: a few frames after ready)
  const afterMade = made.n, wasScale = w.__timeScale;
  w.__timeScale = 0;   // the world holds still: one picture, every state
  const E = w.__engine!, size = () => `${E.width}×${E.height}@${E.settings.renderScale.toFixed(3)}`, size0 = size();
  const bag = () => ({ ms: [] as number[], gpu: [] as number[], passes: new Map<string, number[]>() }), acc = { off: bag(), on: bag() };
  let offBy = '', failed: string | null = null;
  for (let r = 0; r < PRICE.rounds && !failed && !stopped; r++) for (const state of S.look ? ['off', 'on'] as const : ['on'] as const) {
    say(S.look ? `layers ${state}, turn ${r + 1} of ${PRICE.rounds}` : `turn ${r + 1} of ${PRICE.rounds}`);
    const sw = state === 'off' ? layersOff(w) : null; if (state === 'off' && !sw) { failed = 'this page has no way to put the look\'s layers off (its panel\'s rows are not on the sweep)'; break; }
    if (sw) offBy = sw.how;
    const got = await turn(w, c.secs, () => stopped); sw?.back();
    if (!got) { failed = stopped ? 'stopped' : 'the tab was away through every try'; break; }
    acc[state].ms.push(...got.ms); acc[state].gpu.push(...got.gpu); for (const [k, v] of got.passes) acc[state].passes.set(k, [...(acc[state].passes.get(k) ?? []), ...v]);
    if (size() !== size0) notes.push(`the size or scale moved during the turns (${size0} → ${size()}): the columns are not the same picture`);
  }
  w.__timeScale = wasScale ?? 1; removeEventListener('keydown', onKey, true);
  if (stopped) { clear(); view?.progress(null); console.info('price every level: stopped'); return; }
  const next: Carry = { ...c, i: i + 1, live: -1 }, facts = { quality: w.__quality?.label ?? c.quality, scale: +E.settings.renderScale.toFixed(3), internal: [E.width, E.height] as [number, number] };
  const on = winOf(acc.on.ms, acc.on.gpu, acc.on.passes), ratio = (a: number | null, b: number | null) => a !== null && b ? +(a / b - 1).toFixed(4) : null;
  if (failed) next.notes = [...c.notes, `${L.level}${S.look ? '' : ', the page without the look'}: not priced — ${failed}`];
  else if (S.look) { const off = winOf(acc.off.ms, acc.off.gpu, acc.off.passes);
    if (off.gpu === null) notes.push('the browser stamps no GPU times here: the frame\'s time is all there is, and under the display\'s rate it says nothing');
    next.rows = [...c.rows, { level: L.level, mission: L.mission, title: L.title, hour: L.hour, query: loc.search, ...facts, out: [E.outWidth, E.outHeight], off, on, gpuMore: ratio(on.gpu, off.gpu), msMore: ratio(on.ms, off.ms) ?? 0, gpuOverPage: null, msOverPage: null,
      offBy, programs: { boot: bootMade, after: afterMade - bootMade, bySwitch: made.n - afterMade }, boot: w.__boot?.line ?? '', notes }]; }
  else { const at = c.rows.findIndex(r => r.level === L.level);   // (the page without the look: its turns go on the level's own row, beside the look page's)
    if (at < 0) next.notes = [...c.notes, `${L.level}: the page without the look was timed (GPU ${on.gpu ?? '—'} ms, a frame ${on.ms} ms) but its look page was not`];
    else { const R = c.rows[at]; next.rows = c.rows.map((r, k) => k !== at ? r : { ...R, page: { win: on, ...facts, programs: { boot: bootMade, after: made.n - bootMade }, boot: w.__boot?.line ?? '' }, gpuOverPage: ratio(R.on.gpu, on.gpu), msOverPage: ratio(R.on.ms, on.ms),
        notes: [...R.notes, ...notes, ...(facts.internal.join('×') !== R.internal.join('×') || facts.scale !== R.scale ? [`the page without the look drew at ${facts.internal.join('×')} ×${facts.scale}, the look page at ${R.internal.join('×')} ×${R.scale}: not the same size`] : [])] }); } }
  if (i + 1 < of) { if (!write(next)) { clear(); throw new Error('the session store took no more'); } const N = c.steps[i + 1]; loc.replace(loc.pathname + priceSearch(N.level, i + 1, c.quality, N.look) + loc.hash); return; }
  // ---- the last page: the table, and the one file
  clear(); view?.progress(null);
  let gpu = 'unknown'; try { const a = await (navigator as unknown as { gpu?: { requestAdapter(): Promise<{ info?: Record<string, string> } | null> } }).gpu?.requestAdapter(), f = a?.info; if (f) gpu = [f.vendor, f.architecture, f.device, f.description].filter(Boolean).join(' · ') || 'unnamed'; } catch { /* no adapter to ask */ }
  const file: PriceFile = { version: 1, what: 'twopointsix: what the look costs on every level, one held picture a level — the page without the look, the look page with its layers off, and with them on (ui/pricerun.ts)', at: new Date().toISOString(), startedAt: c.startedAt, address: c.from, userAgent: navigator.userAgent, gpu,
    screen: `${screen.width}×${screen.height}`, dpr: devicePixelRatio || 1, quality: c.quality, secs: c.secs, bar: { gpuMore: PRICE.gpuMore, frameMs: PRICE.frameMs, ruled: false }, rows: next.rows, notes: next.notes };
  const name = `price-levels-${file.at.replace(/[:.]/g, '-')}.json`; let saved = false;
  try { const a = document.createElement('a'); a.href = URL.createObjectURL(new Blob([JSON.stringify(file, null, 1)], { type: 'application/json' })); a.download = name; document.body.appendChild(a); a.click(); setTimeout(() => { URL.revokeObjectURL(a.href); a.remove(); }, 2000); saved = true; } catch { /* no download here: the text is on the card */ }
  (window as unknown as { __priceLevels?: PriceFile }).__priceLevels = file;
  if (view) view.show(priceReport(file) + (saved ? `\n\nThe file: ${name}, in your downloads.` : ''), () => loc.assign(c.from || loc.pathname), saved); else console.info(priceReport(file));
}

/** what puts a row over the PROPOSED bar, in words (none = it is under it). The GPU's time is read against the page without the look where the run has one (the
 *  budget's own "look off"), else against the layers off */
export function priceOver(r: PriceRow, bar: PriceFile['bar']): string[] {
  const pc = (v: number) => `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`, more = r.page ? r.gpuOverPage : r.gpuMore;
  return [more !== null && more > bar.gpuMore ? `the GPU's time is ${pc(more)} against ${r.page ? 'the page without the look' : 'the layers off'} (the bar is +${(100 * bar.gpuMore).toFixed(0)}%)` : '',
    r.on.ms > bar.frameMs ? `a frame takes ${r.on.ms.toFixed(1)} ms with the look on (the bar is ${bar.frameMs} ms)` : '', r.programs.bySwitch > 0 ? `switching the layers made ${r.programs.bySwitch} shader programs (the bar is none)` : ''].filter(Boolean);
}
/** THE TABLE, as text: a line a level, the levels over the proposed bar marked and named under it */
export function priceReport(f: PriceFile): string {
  const pct = (v: number | null) => (v === null ? '—' : `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`).padStart(7), n = (v: number | null | undefined, d = 2) => (v === null || v === undefined ? '—' : v.toFixed(d)).padStart(6);
  const head = `${'level'.padEnd(13)}${'hour'.padEnd(9)}${'quality'.padEnd(13)}${'size'.padEnd(17)}| GPU ms: no look  layers off  layers on | on vs no look  on vs off | frame ms: no look     on | programs`;
  const lines = f.rows.map(r => `${r.level.padEnd(13)}${r.hour.padEnd(9)}${r.quality.padEnd(13)}${`${r.internal.join('×')} ×${r.scale.toFixed(2)}`.padEnd(17)}|        ${n(r.page?.win.gpu)}      ${n(r.off.gpu)}     ${n(r.on.gpu)} |       ${pct(r.gpuOverPage)}    ${pct(r.gpuMore)} |           ${n(r.page?.win.ms, 1)} ${n(r.on.ms, 1)} | ${r.programs.boot}+${r.programs.after}+${r.programs.bySwitch}${priceOver(r, f.bar).length ? '   OVER' : ''}`);
  const overs = f.rows.filter(r => priceOver(r, f.bar).length).map(r => `  ${r.level}: ${priceOver(r, f.bar).join('; ')}`), rowNotes = f.rows.flatMap(r => r.notes.map(t => `  ${r.level}: ${t}`));
  return [`WHAT THE LOOK COSTS, a held picture a level  ·  ${f.at.slice(0, 16).replace('T', ' ')} UTC`, `${f.gpu}  ·  screen ${f.screen} ×${f.dpr}  ·  asked for quality ${f.quality}, ${f.secs} s a turn`, f.userAgent, '',
    head, ...lines, '',
    'no look    = the same mission on the page WITHOUT the look, held at the same moment: the game as it is drawn today (the budget\'s own "look off").',
    'layers off = the look page with every layer of the look that has a cost of its own put off at once; layers on = the look page as it opens.',
    'programs   = shader programs the look page made before it was ready + in the seconds after + by switching the layers (that last should be 0).',
    `OVER       = past the PROPOSED bar, which nobody has ruled on: GPU time with the look on within ${(100 * f.bar.gpuMore).toFixed(0)}% of no look, a frame under ${f.bar.frameMs} ms, no program made by the switch.`,
    ...(overs.length ? ['', 'Over the proposed bar:', ...overs] : ['', 'No level is over the proposed bar.']), ...(rowNotes.length || f.notes.length ? ['', 'Notes:', ...f.notes.map(t => `  ${t}`), ...rowNotes] : [])].join('\n');
}
