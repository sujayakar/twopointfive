// PRICE EVERY LEVEL (`?pricelevels` on the look page's address): one address, hands off, and the machine it ran on has said what the look costs it on
// every level the mission list draws (the maintainer, 11-03: "we'll want to dress up all the levels and check their budget"; 11-02: "the main thing i'm concerned
// about before merging is performance", "we're going to need to run the tests across more than just the pass"). A level is only ever loaded at boot
// (scene/scenarioList.ts), so a run is a chain of page loads, as the benchmark's is a page a scene (ui/benchrun.ts): a small record in the tab's own session store
// carries the rows measured so far from one page to the next. The last page shows the table in a box with a Copy button, and the whole run is ONE file
// (price-levels-<date>.json) that leaves by the page's one door (host/savefile.ts): to whoever listens there, and to the browser's downloads where the
// address that asked says `downloads=1` (the run carries that word). Where neither took it the card says so and holds the file's own text under the table,
// so Copy keeps the run.
//   WHAT A LEVEL'S TWO PAGES DO. First its mission's own address ON THE LOOK PAGE, at the hour its row opens at, the die pinned: it plays a few seconds so the
//   men stand where the mission puts them (by the page's own loop, the last quarter second by its scripted step: PRICE.playS), then HOLDS THE WORLD STILL and times that one picture with the look's layers off and on in turn (off, on, off, on):
//   the frame's time, and where the browser stamps the passes, the GPU's. Then the same mission on the page WITHOUT the look (no `look` on the address), held at
//   the same step of the game's own clock and timed as it stands. Then the next level. Both are a run's pages (host/runpage.ts): the kit and the fog of war
//   as the mission deals them, no sandbox row of this machine's, so the two pictures are of one scene. AND ONE RUNG OF THE QUALITY LADDER: a page of a run
//   has its preset pinned by its address (main.ts: `quality=` also keeps the climb off) and boots with the ladder's own switch off (`quality.adaptive`, the
//   menu's "Hold frame rate"; main.ts asks host/runpage.ts heldPage before the first frame is sampled). With both, the ladder does nothing: the scale, the
//   cap on rows, the gather's and the volumetrics' resolution and the sharp pass are the preset's own on both pages, whatever this machine's frame rate. Each
//   page writes what it stood on (`rung`), and a level whose two pages differ in any of it says so. (First run on a real GPU, 11-04: left to itself the
//   ladder cut the look page of two levels from High's own 0.77 of the window to 0.75 and left their pages without the look at 0.77.)
//   TWO MEANINGS OF "LOOK OFF", both in the file:
//     · THE PAGE WITHOUT THE LOOK is the budget's own ("GPU time within 10% of look off"): the game as it is drawn today. It is another page: other shader
//       text, and the level as its file has it (none of the look's round things and broken wall tops). Its row is `page`; the table's OVER is read against it.
//     · THE LAYERS OFF is the same page, shaders, shapes, camera, quality and scale, with every row the look panel lists as a layer with a cost of its own
//       (ui/lookpanel.ts hands them to the sweep as `look.<row>`) at its own off value, all at once. It says what the layers cost where they run, and it is NOT
//       the panel's "plain boxes", which leaves the lighting's, the smoke's and the eye's layers on.
//   WHAT IT DOES NOT SAY. A fight's frame (the picture is held at the mission's start) and a first visit's compile (ui/boot.ts times that; each row keeps the
//   boot's own line as its page had it). AND THE GPU'S TIME IS THE COMPUTE PASSES' ONLY, as everywhere on the page (gpu/timer.ts unionMs: a render pass's stamps
//   span the wait for the display): the look's SURFACE layers (paint, relief, finish, cavities, weather, edges, marks, the men's kit) are shaded in a render
//   pass, the g-buffer's, so their cost is not in that number, and the time between two frames shows it only once the display's rate is lost. So the run
//   ALSO takes the GPU's busy span over THE COMPUTE PASSES AND THE G-BUFFER (`all`: the same union with only the passes that draw to the display left out, so
//   passes the driver ran side by side are not counted twice), and says what the look ADDS to it, as a share of the baseline's GPU time: the verdict reads
//   that beside the game's own number. WHY THOSE PASSES: the stamps that span the wait for the drawable are the ones of the passes that draw to the display
//   (post, fxaa, a drawn composite). The g-buffer draws into the page's own textures; on an M4 Max under Metal its stamp was read against 50,000 recorded
//   frames (2026-11-04) and follows the g-buffer's work, not the frame's length — one chip, so the run still CHECKS each state (`spansWait`): where the span
//   reads about the whole time between frames and far more than the compute passes, it says so on the row and judges nothing by it. (Until 11-04 `all` was
//   every stamped pass, which under Metal read the display's interval, and the run said the surface layers could not be read there at all.)
//   AND HOW FAR BEHIND THE GPU IS (`lag`): each frame is timed from its start to the moment the GPU has finished everything the frame handed it (`work`: the
//   queue's own "submitted work done"), in frames. It is NOT a cost: the answer comes back after the next frame has begun even when the GPU keeps up (about
//   1.2 frames at 60 Hz, measured), and later when it does not. AND A FRAME'S TIME IS THE TIME BETWEEN TWO FRAMES THE PAGE DREW (`ms`), by the pacer's own
//   count of the display frames it worked (host/pace.ts, `window.__pace`): a priced page waits for the GPU as a player's does (main.ts `flight`), so where
//   the GPU cannot finish a frame in a display frame the page passes display frames over, and the time between two frame CALLBACKS would still read the
//   display's. (Two older ways and why they went, both 11-04: a frame's "cost" as the lesser of `work` and the time between callbacks read the display's
//   interval in every state. And a priced page that did not wait for the GPU queued hundreds of frames ahead on a level the card could not hold sixty on:
//   a switch of the layers then reached the stamps ten seconds late, inside the other state's turn, and the two states read alike.)
//   The frame's time and the lag are read AGAINST THE BASELINE too: a machine that does not hold sixty at this preset without the look is not the look's doing.
//   THE WORDS. `?pricelevels` = every level; `?pricelevels=pass,block` = these; `&pricesecs=N` = N seconds measured a turn (default 3); `&quality=<preset>` =
//   the preset every page stands at (default high: the budget is said for High; `auto` lets each page settle its own, and the row says where it settled);
//   `&pricepage=0` = the look page only, half the loads. THE LOOK'S OWN WORDS on the address that asked (`look=walls:0`, `geo=0` …) ride on every look page
//   of the run, with the trials this machine's look panel keeps off, as the benchmark carries them (ui/benchrun.ts): the file says which look was priced.
//   A reload in the middle does not start it again (the page finds its own mark and is a plain mission page); Esc stops a run.
// THE PAGE'S HALF IS ONE LINE (main.ts, before the boot): `if (priceLevelsPage(q)) return;`. Everything else is asked of what the page already puts on `window`
// for its checks (__ready, __engine, __game, __quality, __perfsweep, __boot): this file adds no hook and no state to the game.
import { listedLevels } from '../scene/scenarioList';
import { scenarioQuery } from '../scene/scenarios';
import { lookPageWord, lookTrialsKept } from '../scene/look';
import { PRICE_ASK_WORDS, PRICE_LEVELS_WORD, PRICE_RUN_WORD } from '../host/runpage';
import { PRESENT_PASSES, RENDER_PASSES, SURFACE_PASS } from '../gpu/timer';
import { saveFile, savedWords } from '../host/savefile';
import { BENCH_SEND_MS, benchView, machineFacts, type BenchMachine, type BenchWent } from './bench';
import { carriedWords } from './benchrun';

const KEY = 'twopointsix.pricelevels';
/** the word that asks for a run · the word on a level's page of one: its place in the run */
export const PRICE_KEY = PRICE_LEVELS_WORD, PRICE_RUN_KEY = PRICE_RUN_WORD;
/** the render passes, which the GPU's busy span leaves out everywhere on the page · the one of them the look's surface layers are shaded in: its stamp is
 *  printed beside and is inside `all` · and the ones that draw to the display, which `all` leaves out on every machine (gpu/timer.ts has all three and why) */
const DRAWN = [...RENDER_PASSES], SURFACE = SURFACE_PASS, PRESENTS = [...PRESENT_PASSES];
export const PRICE = {
  /** seconds OF THE GAME'S OWN CLOCK the mission plays before its picture is held, and how it is played. BY THE PAGE'S OWN LOOP, as a player's page plays
   *  it, up to `stepS` short of that; THE LAST PART IS STEPPED: the loop is paused and the mission played in fixed sixtieths by the page's own scripted step
   *  (main.ts `__runFrames`), `stepsAFrame` of them a display frame, to exactly the step asked. So a level's two pages stand on the same step whatever their
   *  frames did (watched from outside to the end, a page on a cold machine was once held 0.43 s late, 11-04), and the picture is still one the loop drew:
   *  PLAYED BY STEPS ALONE, A SNOW LEVEL HAD NO SNOW IN ITS AIR (11-04, a rented card: the page's first frame of its own clears the particles' ring, and
   *  with the clock held nothing files a flake again; its fog pass read 1.4 ms too cheap). For that the page also draws `warmFrames` frames of its own with
   *  the clock held before anything is played, which is all that stands between a fully stepped play (`stepS` ≥ `playS`: a check's) and that fault.
   *  A mission whose clock does not run by itself on a page nobody plays (a lesson's card holds it) is seen to stand for `stuckS` and stepped the whole way */
  playS: 4, stepS: 0.25, stepsAFrame: 2, warmFrames: 3, stuckS: 2,
  /** THE GPU CATCHES UP BEFORE ANYTHING IS TIMED: a boot hands it seconds of work (its pipelines, its first frames), and for a while after the queue's "done"
   *  comes many frames late whatever a frame costs (11-04, a rented card on a long run: hundreds of frames behind on a page with 6 ms of work, with the look
   *  and without). The held picture is drawn until a frame's work is done within `lagFrames` frames three times running, for at most this many seconds; a
   *  page still behind then says so on its row, and there the lag is the page's own. 0 = no wait (a check on an adapter whose frame is seconds long) */
  catchS: 30,
  /** a turn: the start that is dropped, then the part that is measured (seconds; `pricesecs` on the address is the second) */ settleS: 1.2, measureS: 3,
  /** … and each wants this many frames, and gives up waiting for them after `capS` (a machine at two frames a second) */ settleFrames: 12, measureFrames: 30, capS: 25,
  /** turns of each state */ rounds: 2,
  /** a frame longer than this (seconds) is the tab having been away: the turn starts over, at most `retries` times */ awayS: 1.5, retries: 3,
  /** THE BAR the table sets each level against (the look task's, 11-02; `ruled` below says which of its rows the maintainer has ruled on): the GPU's time with the layers on within this of off */ gpuMore: 0.10,
  /** which rows stand by the maintainer's word and which are still a proposal: written into the file and the report, so neither claims more than was ruled */ ruled: 'GPU time within 10% of no look and sixty frames a second held were ruled on 2026-11-04; the same with the g-buffer counted and "no program made by the switch" are still proposed',
  /** … and sixty frames a second held on High. Between two frames of a 60 Hz display there are 16.67 ms, which the proposal's "under 16.6" would call over, so
   *  the bar on a frame's time is that with 2% of the clock's noise on it */ frameMs: 17,
  /** … and the GPU keeping up: its "done" for a frame no later than this many frames after the frame began (it is about 1.2 when it keeps up) */ lagFrames: 3,
  /** THE G-BUFFER'S STAMP HOLDS THE DISPLAY'S WAIT where the compute passes with the g-buffer take this much of the time between frames (`waitAll`) and the
   *  g-buffer puts this much of that time on the compute passes (`waitGap`). HALF A FRAME, on purpose: a g-buffer that is honest work must never trip it. On
   *  an M4 Max the look page's g-buffer was 1.8 to 2.3 ms; at 120 Hz that is over a quarter of the 8.3 ms between frames, and a page that fills its frame
   *  there would be called a wait at a quarter (the bar it was until 11-04). A wait smaller than half a frame goes unseen: the span then reads about the
   *  frame's whole length on the baseline too, and the look adds nothing to it */ waitAll: 0.85, waitGap: 0.5,
  /** THE LAG IS JUDGED ONLY WHERE THE STAMPS AGREE: the GPU's "done" coming late is the page's own cost where its stamped work fills this much of the time
   *  between frames. Late with less than that is not explained by the passes (seen on a long run in one tab: pages with 6 ms of work hundreds of frames
   *  late, with the look and without), so it is said on the row and nothing is judged by it */ fullFrame: 0.8,
};

/** one state's turns on one page */
export interface PriceWin { /** frames the page drew in the measured turns · the time between two of them, ms (the header) · the 95th centile of the time between two frame callbacks */ frames: number; ms: number; p95: number; /** the GPU's busy span over the compute passes, ms (null: the browser stamps none) */ gpu: number | null;
  /** EVERY STAMPED PASS, ms: the GPU's busy span with nothing left out (the header; null: no stamps) */ all: number | null;
  /** from a frame's start to the GPU having finished all the frame handed it, ms (null: no device to ask) … */ work: number | null; /** … and that in frames: how far behind the GPU is (the header: not a cost) */ lag: number | null;
  /** each pass's own mean, ms */ passes: Record<string, number> | null }
/** THE RUNG OF THE QUALITY LADDER a page was priced on: everything of the picture the ladder can move (ui/quality.ts) */
export interface PriceRung { preset: string; /** the ladder's switch as the turns found it (false: held) */ adaptive: boolean | null; scale: number; /** the cap on internal rows */ cap: number | null; gatherThird: boolean | null; volQuarter: boolean | null; sharp: boolean | null }
export interface PriceRow {
  level: string; mission: string; title: string; hour: string; query: string;
  quality: string; scale: number; internal: [number, number]; out: [number, number]; rung: PriceRung;
  off: PriceWin; on: PriceWin;
  /** the game's clock where the picture was held, seconds (null: the page shows no clock) */ heldAt: number | null;
  /** layers on over layers off, less one: of the GPU's span where there is one, of the time between frames; and what the layers add to the compute passes with the g-buffer, as a share of the layers-off GPU's span */ gpuMore: number | null; allMore: number | null; msMore: number;
  /** THE PAGE WITHOUT THE LOOK, the same mission held at the same step (absent: the run was asked for without it, or that page did not finish) */
  page?: { win: PriceWin; quality: string; scale: number; internal: [number, number]; rung: PriceRung; heldAt: number | null; programs: { boot: number; after: number }; boot: string };
  /** the look page with its layers on over the page without the look, less one (null: no such page, or no GPU times); and what the look adds to the compute passes with the g-buffer, as a share of that page's GPU's span */ gpuOverPage: number | null; allOverPage: number | null; msOverPage: number | null;
  /** how the layers were put off */ offBy: string;
  /** shader programs: made before the page came ready, after it, and by the switch itself (the budget's "shaders compile once": nothing) */ programs: { boot: number; after: number; bySwitch: number };
  /** the boot as the page timed it (ui/boot.ts line) */ boot: string;
  notes: string[];
  /** WHY THE ROW IS NOT ONE PICTURE, in words (none: it is): the picture moved between its turns, or its two pages stood at another size, on another rung of
   *  the ladder or on another step of the game's clock. Such a row keeps its numbers and is NOT JUDGED: `over` stays empty and the report says so */
  apart: string[];
  /** THE VERDICT, written by the run once the row is whole (priceOver): what puts it over the bar, in words; none = under it. The table of levels
   *  (tools/qa/level-table.ts) prints these and judges nothing itself */
  over: string[];
}
/** a page of the run: a level on the look page, or on the page without it */
interface Step { level: string; look: boolean }
interface Carry { v: 5; from: string; startedAt: string; steps: Step[]; i: number; live: number; quality: string; secs: number; /** the look page's words, as the address that asked wrote them */ look: string[]; /** the visitor's flags every page of the run carries (ui/benchrun.ts carriedWords) */ keep: string[]; rows: PriceRow[]; notes: string[] }
export interface PriceFile { version: 5; what: string; at: string; startedAt: string; address: string; /** the words every look page of the run carried */ look: string[]; userAgent: string; /** the adapter the page drew on */ gpu: string; machine: BenchMachine; screen: string; dpr: number; quality: string; secs: number; bar: { gpuMore: number; frameMs: number; lagFrames: number; ruled: string }; rows: PriceRow[]; notes: string[] }

const read = (): Carry | null => { try { const t = sessionStorage.getItem(KEY), c = t ? JSON.parse(t) as Carry : null; return c && c.v === 5 && Array.isArray(c.rows) && Array.isArray(c.steps) && Array.isArray(c.look) && Array.isArray(c.keep) ? c : null; } catch { return null; } };
const write = (c: Carry): boolean => { try { sessionStorage.setItem(KEY, JSON.stringify(c)); return true; } catch { return false; } };
const clear = (): void => { try { sessionStorage.removeItem(KEY); } catch { /* no store: nothing was kept */ } };

/** THE LEVELS A RUN WALKS: every level a drawn row of the mission list plays on, in the list's order, each with the first row that plays on it */
export function priceLevels(): { level: string; mission: string; title: string; hour: string; query: string }[] {
  return listedLevels().map(({ level, rows: [e] }) => ({ level, mission: e.id, title: e.title, hour: e.hour ?? 'night', query: scenarioQuery(e) }));   // (scene/scenarioList.ts: the one answer to which levels the list draws)
}
/** THE LOOK PAGE'S WORDS a run carries: the address's own look word as it was written (the bare `look` where it has none: a run prices the look), the other
 *  words that page reads once at boot, and the trials this machine's look panel keeps off said as the address would say them (scene/look.ts; the benchmark
 *  carries the same, ui/benchrun.ts) */
export function priceLookWords(search: string): string[] {
  const q = new URLSearchParams(search), own = search.replace(/^\?/, '').split('&').filter(p => { const kv = p ? [...new URLSearchParams(p)][0] : undefined; return !!kv && lookPageWord(kv[0], kv[1]); });   // (with or without a `look` word beside them: every look page of a run is one, so `?pricelevels&geo=0` prices the look without that trial)
  return [...(own.some(p => /^look(=|$)/.test(p)) ? [] : ['look']), ...own, ...lookTrialsKept(q)];
}
/** the address a step is priced on: its mission's own words (on the look page with that page's words, or with none of them), the die pinned, no verdict kept of
 *  the try and no replay filed (the run is nobody's play: as a benchmark's scene, ui/benchrun.ts), no sound, the preset, and its place in the run */
export function priceSearch(level: string, i: number, quality: string, look: string[] | null = ['look'], keep: string[] = []): string {
  const L = priceLevels().find(l => l.level === level); if (!L) throw new Error(`no level '${level}' on the mission list`);
  const own = `${L.query.slice(1)}&seed=1&noverdict&nokeep&mute${quality === 'auto' ? '' : `&quality=${quality}`}&${PRICE_RUN_KEY}=${i}`, has = new URLSearchParams(own);
  return `?${[...[...(look ?? []), ...keep].filter(p => !has.has(p.split('=')[0])), own].join('&')}`;   // (a word the page's own address says for itself — its preset, no sound — is not said twice)
}

/** THE WORDS OF THE ASKING ADDRESS NO PAGE OF THE RUN CARRIES (the benchmark names its own the same way, ui/benchrun.ts `left`): every word that is neither
 *  the run's own, nor the look page's, nor one of the visitor's flags a run carries, nor said by a level's own address. A mission's words are among them (an
 *  hour, a level file: each level is priced on its own row's). The file says them, so its `address` never claims a word its pages did not have */
export function priceLeft(search: string, level: string, quality: string, look: string[], keep: string[]): string[] {
  const own = new URLSearchParams(priceSearch(level, 0, quality, look, keep));
  return [...new Set([...new URLSearchParams(search).keys()])].filter(k => k && !PRICE_ASK_WORDS.includes(k) && !own.has(k));
}

/** THE PAGE'S ONE LINE, before its boot. True = this address asked for a run and the first level's address is loading: boot nothing. False = boot as ever; on a
 *  level's page of a run under way the page's part of it starts by itself once the page is ready. */
export function priceLevelsPage(q: URLSearchParams, loc: Location = location): boolean {
  if (q.has(PRICE_KEY) && !q.has(PRICE_RUN_KEY)) {
    const all = priceLevels().map(l => l.level), asked = (q.get(PRICE_KEY) ?? '').split(',').map(s => s.trim()).filter(Boolean), levels = asked.length ? asked.filter(l => all.includes(l)) : all;
    const secs = parseFloat(q.get('pricesecs') ?? ''), quality = q.get('quality') ?? 'high', both = q.get('pricepage') !== '0';
    const steps: Step[] = levels.flatMap(level => both ? [{ level, look: true }, { level, look: false }] : [{ level, look: true }]);
    const rest = loc.search.replace(/^\?/, '').split('&').filter(p => p && !PRICE_ASK_WORDS.includes(p.split('=')[0]));   // (the address that asked, with the run's own words off it: where the card's Close goes)
    const c: Carry = { v: 5, from: loc.pathname + (rest.length ? '?' + rest.join('&') : '') + loc.hash, startedAt: new Date().toISOString(), steps, i: 0, live: -1, quality, secs: isFinite(secs) && secs > 0 ? secs : PRICE.measureS, look: priceLookWords(loc.search), keep: carriedWords(loc.search), rows: [],
      notes: asked.length && levels.length < asked.length ? [`not on the mission list, left out: ${asked.filter(l => !all.includes(l)).join(', ')}`] : [] };
    if (levels.length) { const left = priceLeft(loc.search, levels[0], quality, c.look, c.keep); if (left.length) c.notes.push(`every page was priced without ${left.join(', ')}: ${left.length > 1 ? 'words' : 'a word'} of the address that asked which no page of a run carries`); }
    if (!levels.length || !write(c) || !read()) { console.info(levels.length ? 'price every level: this tab keeps no session store, so a run cannot be carried from page to page' : `price every level: none of ${asked.join(', ')} is on the mission list (${all.join(' ')})`); return false; }
    loc.replace(loc.pathname + priceSearch(steps[0].level, 0, quality, steps[0].look ? c.look : null, c.keep) + loc.hash);   // (the address that asked is not left in the tab's history, where Back would ask again)
    return true;
  }
  const at = q.get(PRICE_RUN_KEY); if (at === null) return false;
  const c = read(), i = Number(at);
  if (!c || c.i !== i || c.live === i || !c.steps[i]) { if (c) clear(); return false; }   // (another page's address, or a reload of a level already begun: a plain mission page)
  c.live = i; write(c);
  countPrograms();
  void leg(c, i, loc);
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
  __ready?: Promise<unknown>; __timeScale?: number; __pauseLoop?: boolean; __runFrames?: (n: number, dt?: number, keys?: string[], draw?: boolean | 'none') => void; __menu?: { isOpen?: boolean }; __game?: { time: number };
  __engine?: { width: number; height: number; outWidth: number; outHeight: number; settings: { renderScale: number; sharpDirect?: boolean; maxInternalHeight?: number; gatherThird?: boolean; volQuarter?: boolean }; gpu?: { adapterInfo?: string; adapterFallback?: boolean }; device?: { queue: { onSubmittedWorkDone(): Promise<unknown> } };
    timer?: { enabled: boolean; resolved: number; lastFrame: Map<string, number>; unionMs(exclude: string[]): number } };
  __quality?: { label: string; calibrating: boolean; displayHz?: number; displayHzPin?: number; /** the preset standing · auto: it moves by itself · the ladder's switch (the menu's "Hold frame rate") · a preset's rows put on */ current?: string; auto?: boolean; adaptive?: boolean; apply?: (name: string) => void; /** the ladder's sample of a frame (a method of its class: an own one put over it stands in until it is deleted) */ sample?: (dt: number, gpuMs?: number) => void };
  __perfsweep?: { extra?: { id: string; flip(): string | { apply(): () => void } }[] };
  __boot?: { line?: string }; __cut?: { programs?: { swap?: boolean; ready?: Promise<unknown> } };
  /** the page's pacer (host/pace.ts): display frames it WORKED since the page began (the others it passed over: no room with the GPU, a rate being held) */
  __pace?: () => { worked: number; skipped: number };
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

/** the wait of PRICE.catchS: null = the GPU has caught up (or cannot be asked); else how many frames behind it still was when the time ran out */
async function caughtUp(w: PageHooks, stop: () => boolean): Promise<number | null> {
  const dev = w.__engine?.device; if (!dev || PRICE.catchS <= 0) return null;
  let good = 0, lag = 0;
  for (const t0 = performance.now(); performance.now() - t0 < PRICE.catchS * 1000 && !stop();) {
    const a = await frame(), b = await frame(), dt = Math.min(100, Math.max(4, b - a));   // (two frames running: the time between frames, then a frame asked whole as a turn asks it)
    await new Promise<void>(r => { const ch = new MessageChannel(); ch.port1.onmessage = () => { ch.port1.close(); r(); }; ch.port2.postMessage(0); });
    await dev.queue.onSubmittedWorkDone().catch(() => { });
    lag = (performance.now() - b) / dt;
    if (lag <= PRICE.lagFrames) { if (++good >= 3) return null; } else good = 0;
  }
  return stop() ? null : lag;
}
/** one turn: the frames of `settle` dropped, then `measure` kept. Null = the tab was away through every try */
async function turn(w: PageHooks, secs: number, stop: () => boolean): Promise<Bag | null> {
  const T = w.__engine?.timer, dev = w.__engine?.device;
  for (let tries = 0; tries <= PRICE.retries; tries++) {
    const ms: number[] = [], gpu: number[] = [], all: number[] = [], work: number[] = [], asked: Promise<void>[] = [], passes = new Map<string, number[]>(); let away = false, last = await frame(), seen = T?.resolved ?? 0;
    // THE FRAMES THE PAGE DREW, by its pacer's own count: the page waits for the GPU (main.ts `flight`), so on a level the GPU cannot finish in a display
    // frame it passes display frames over, and the time between this file's own frame callbacks stays the display's. A frame's time is the measured part's
    // length over the frames worked in it; and the GPU is asked for "done" only on a frame that was worked (a passed-over frame handed it nothing)
    const worked = () => w.__pace?.().worked; let workedAt = worked(), workedLast = workedAt;
    // THE FRAME WHOLE: asked in a task of its own, which the browser runs after every frame callback of this frame (the page's, which hands the GPU its
    // work, is one of them, before this file's or after it); the queue then answers once the GPU has finished all of it. From the frame's own start
    const whole = (start: number) => { if (!dev) return; asked.push(new Promise<void>(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => { ch.port1.close();
      const n = worked(); if (n !== undefined && n === workedLast) { res(); return; } workedLast = n;   // (the page passed this display frame over)
      dev.queue.onSubmittedWorkDone().then(() => { work.push(performance.now() - start); res(); }, () => res()); }; ch.port2.postMessage(0); })); };
    const t0 = last; let kept = -1;   // (the moment the measured part began)
    for (;;) { const now = await frame(), dt = now - last; last = now; if (stop()) return null;
      if (dt > PRICE.awayS * 1000 || document.hidden) { away = true; break; }
      if (kept < 0) { if ((now - t0 >= PRICE.settleS * 1000 && ms.length >= PRICE.settleFrames) || now - t0 >= PRICE.capS * 1000) { kept = now; ms.length = 0; seen = T?.resolved ?? 0; workedAt = worked(); } else ms.push(dt); continue; }
      ms.push(dt); whole(now);
      if (T?.enabled && T.resolved !== seen) { seen = T.resolved; gpu.push(T.unionMs(DRAWN)); all.push(T.unionMs(PRESENTS)); for (const [k, v] of T.lastFrame) { let a = passes.get(k); if (!a) passes.set(k, a = []); a.push(v); } }
      if ((now - kept >= secs * 1000 && ms.length >= PRICE.measureFrames) || now - kept >= PRICE.capS * 1000) { const n = worked(), drew = n !== undefined && workedAt !== undefined && n > workedAt ? { secs: (now - kept) / 1000, frames: n - workedAt } : null;
        await Promise.race([Promise.all(asked), new Promise(r => setTimeout(r, PRICE.capS * 1000))]); return { ms, gpu, all, work, passes, drew }; }
    }
    if (away) { while (document.hidden) await new Promise(r => setTimeout(r, 500)); }
  }
  return null;
}
type Bag = { ms: number[]; gpu: number[]; all: number[]; work: number[]; passes: Map<string, number[]>; /** the frames the page worked in the measured part, and its length (null: the page has no pacer to ask) */ drew: { secs: number; frames: number } | null };
const winOf = ({ ms, gpu, all, work, passes, drew }: Bag): PriceWin => { const s = [...ms].sort((a, b) => a - b), every = drew && drew.frames > 0 ? 1000 * drew.secs / drew.frames : median(ms);   // (the time between two frames the page DREW; between two frame callbacks where it cannot be asked)
  return { frames: drew ? drew.frames : ms.length, ms: +every.toFixed(2), p95: +(s[Math.min(s.length - 1, Math.floor(s.length * 0.95))] ?? NaN).toFixed(2), gpu: gpu.length ? +median(gpu).toFixed(2) : null, all: all.length ? +median(all).toFixed(2) : null,
    work: work.length ? +median(work).toFixed(2) : null, lag: work.length && every > 0 ? +(median(work) / every).toFixed(2) : null,
    passes: passes.size ? Object.fromEntries([...passes].map(([k, v]) => [k, +(v.reduce((a, b) => a + b, 0) / v.length).toFixed(3)])) : null }; };
/** A RENDER PASS'S STAMPS SPAN THE DISPLAY'S WAIT in this state (the header): the compute passes with the g-buffer take about the whole time between frames and far more
 *  than the compute passes alone, so the g-buffer's stamp holds a wait on this machine. Nothing is judged by `all` then */
export const spansWait = (w: PriceWin): boolean => w.all !== null && w.gpu !== null && w.all >= PRICE.waitAll * w.ms && w.all - w.gpu >= PRICE.waitGap * w.ms;
/** the stamped work fills the frame in this state (PRICE.fullFrame), or nothing is stamped: a late "done" is then the page's own */
export const fullFrame = (w: PriceWin): boolean => w.gpu === null ? true   // (no stamps at all: the lag is the only sign there is, so it is judged — the header)
  : (spansWait(w) || w.all === null ? w.gpu : w.all) >= PRICE.fullFrame * w.ms;   // (a span that holds the display's wait fills the frame by construction: the compute passes alone answer there)
/** … and late without that: said, not judged */
const lateUnexplained = (w: PriceWin, what: string): string[] => w.lag !== null && w.lag > PRICE.lagFrames && !fullFrame(w) ? [`${what}: the GPU's "done" came ${w.lag.toFixed(0)} frames late with ${w.all ?? w.gpu ?? '—'} ms of stamped work in a ${w.ms} ms frame: the passes do not explain it, and nothing is judged by it`] : [];
/** what `on` adds to the compute passes with the g-buffer over `base`, as a share of base's GPU's span (null: no stamps, or a display's wait in them) */
const added = (on: PriceWin, base: PriceWin): number | null => on.all === null || base.all === null || !base.gpu || spansWait(on) || spansWait(base) ? null : +((on.all - base.all) / base.gpu).toFixed(4);
/** what a page stands on of the ladder's */
const rungOf = (w: PageHooks): PriceRung => { const S = w.__engine!.settings, Q = w.__quality, b = (v: boolean | undefined) => v === undefined ? null : !!v;
  return { preset: Q?.label ?? '', adaptive: b(Q?.adaptive), scale: +S.renderScale.toFixed(3), cap: S.maxInternalHeight ?? null, gatherThird: b(S.gatherThird), volQuarter: b(S.volQuarter), sharp: b(S.sharpDirect) }; };
/** the levers two pages differ in, in words (none: one rung) */
export const rungDiffers = (a: PriceRung, b: PriceRung): string[] => (['preset', 'scale', 'cap', 'gatherThird', 'volQuarter', 'sharp'] as const).filter(k => a[k] !== b[k]).map(k => `${k} ${a[k]} / ${b[k]}`);

/** THIS PAGE'S PART OF THE RUN. Whatever it changes of the page (the world held, the ladder's sample, the layers, its own key) goes on `undo` and is put back
 *  however the part ends: by its next page, by Esc, or by a throw, which also takes the veil down (the same view's, not a new one's) */
async function leg(c: Carry, i: number, loc: Location) {
  const view = benchView(), undo: (() => void)[] = [];
  try { await price(c, i, loc, view, undo); }
  catch (e) { clear(); view?.progress(null); console.error('price every level: the run stopped —', e); }
  finally { while (undo.length) try { undo.pop()!(); } catch { /* the page is on its way out */ } }
}
async function price(c: Carry, i: number, loc: Location, view: ReturnType<typeof benchView>, undo: (() => void)[]) {
  const w = window as unknown as PageHooks, S = c.steps[i], L = priceLevels().find(l => l.level === S.level)!, of = c.steps.length;
  Object.assign(PRICE, (window as unknown as { __priceOpts?: Partial<typeof PRICE> }).__priceOpts ?? {});   // (a check's own times: tools/qa/pricerun-page.cjs drives a run on an adapter whose frame is seconds long)
  let stopped = false; const onKey = (e: KeyboardEvent) => { if (e.code === 'Escape') stopped = true; }; addEventListener('keydown', onKey, true); undo.push(() => removeEventListener('keydown', onKey, true));
  const say = (t: string) => view?.progress(`pricing the look, page ${i + 1} of ${of}: ${L.title}${S.look ? '' : ', without the look'} — ${t}   (hands off; Esc stops)`);
  say('loading');
  // THE PAGE'S LOOP IS HELD FROM BEFORE ITS BOOT (this runs on the page's one line, before the boot): the mission's clock then runs only by the steps of the
  // play below, on both pages of a level. Held only from ready, a page played by itself for as long as its last shaders took to build behind the start
  // (a cold machine's first page was held at 4.2 s where the rest held at 4, 11-04; where a person sits the start does not wait for the later set at all)
  const loopWas = w.__pauseLoop; w.__pauseLoop = true; undo.push(() => { w.__pauseLoop = loopWas; });
  while (!w.__ready) await new Promise(r => setTimeout(r, 50));
  await w.__ready; const bootMade = made.n, notes: string[] = [], apart: string[] = [];
  let offBy = '', heldAt: number | null = null, afterMade = bootMade;
  // (every address of a run is a mission's own and opens straight in: the start card up at ready is a mission that was refused, and what stands behind it is
  //  the plain night, which is not priced under the mission's name)
  let failed: string | null = w.__menu?.isOpen ? 'the mission did not start: the start card came up on its address' : null;
  // ONE RUNG (the header): the page booted with its preset pinned and the ladder's switch off (main.ts). Looked at, not assumed: a page that stands otherwise
  // (quality=auto was asked for; a build whose main.ts does not hold a priced page) says so on its row, and its ladder is then at least kept from sampling the
  // held picture, as it takes no sample behind a card
  const Qh = w.__quality, held = !!Qh && Qh.auto === false && Qh.adaptive === false;
  if (!held) notes.push(!Qh || Qh.auto !== false ? 'the quality is not a pinned preset: its ladder was left to settle by itself, so a level\'s two pages may stand on different rungs' : 'the quality ladder\'s switch was on: it may have cut the scale before the picture was held');
  const E = w.__engine!, size = () => `${E.width}×${E.height}@${E.settings.renderScale.toFixed(3)}${E.settings.sharpDirect ? ' sharp' : ''}`;
  const bag = (): Bag => ({ ms: [], gpu: [], all: [], work: [], passes: new Map(), drew: null }), acc = { off: bag(), on: bag() };
  if (!failed) {
    say('the mission stands up');
    const G = w.__game, pace = w.__timeScale ?? 1; undo.push(() => { w.__timeScale = pace; });
    if (G && w.__runFrames) {   // (the page's loop has been paused since before its boot: the mission's clock stands at its start on both pages)
      // the page's first frames of its own, with the clock held (PRICE.playS: what they clear, they clear before anything is in the air)
      w.__timeScale = 0; w.__pauseLoop = loopWas; for (let k = 0; k < PRICE.warmFrames && !stopped; k++) await frame();
      // … then the mission by the page's own loop, up to `stepS` short of the step asked for
      w.__timeScale = pace;
      { const live = PRICE.playS - PRICE.stepS; let at = G.time, since = performance.now();
        while (!stopped && G.time < live - 1e-6) { await frame();
          if (G.time > at + 1e-9) { at = G.time; since = performance.now(); }
          else if (performance.now() - since > PRICE.stuckS * 1000) { notes.push(`the mission's clock does not run by itself on this page (it stood at ${G.time.toFixed(2)} s for ${PRICE.stuckS} s): played by the page's scripted step from there`); break; } } }
      // … and the rest by the scripted step, to exactly that step
      w.__pauseLoop = true;
      const most = 2 * Math.round(PRICE.playS * 60); let done = 0;
      // (as many steps as are left and no more: the loop's last frame may have carried the clock a step or two past where it was let go)
      while (!stopped && G.time < PRICE.playS - 1e-6 && done < most) { const left = Math.max(1, Math.round((PRICE.playS - G.time) * 60)), n = Math.min(PRICE.stepsAFrame, left, most - done); w.__runFrames(n, 1 / 60, [], true); done += n; await frame(); }
      if (G.time < PRICE.playS - 1e-6) notes.push(`the game's clock stood at ${G.time.toFixed(2)} s after ${done} steps: held there, not at ${PRICE.playS} s`);
      else if (G.time > PRICE.playS + 1 / 120) notes.push(`the mission had played ${G.time.toFixed(2)} s before the page could be held: held there, not at ${PRICE.playS} s`); }
    else notes.push('this page has no scripted step: the picture was held where the mission stood when the page came ready');
    w.__timeScale = 0; w.__pauseLoop = loopWas; heldAt = G ? +G.time.toFixed(4) : null;   // the world holds still and the page's own loop draws it again: one picture, every state
    for (let t0 = performance.now(); w.__quality?.calibrating && !stopped;) { await frame(); if (performance.now() - t0 > 60000) { notes.push('auto quality was still measuring after a minute: priced where it stood'); break; } }
    if (!held && Qh && typeof Qh.sample === 'function') { Qh.sample = () => { }; undo.push(() => { delete Qh.sample; }); }   // (a ladder left on takes no sample of the held picture: nothing of the size, the scale or what is shed moves between the turns. A held one samples nothing anyway)
    const cut = w.__cut?.programs; if (cut?.swap && cut.ready) await Promise.race([cut.ready, new Promise(r => setTimeout(r, 60000))]);   // (the second set of ray programs compiles after ready where the page has one: not charged to the switch)
    if (S.look) for (let t0 = performance.now(); !w.__perfsweep?.extra?.some(x => x.id.startsWith('look.')) && performance.now() - t0 < 5000;) await frame();   // (the look panel hands its rows to the sweep once the page's recorder is there: a few frames after ready)
    { const behind = await caughtUp(w, () => stopped); if (behind !== null) notes.push(`the GPU's "done" still came ${behind.toFixed(0)} frames late after ${PRICE.catchS} s of holding the picture`); }
    afterMade = made.n; const size0 = size();
    for (let r = 0; r < PRICE.rounds && !failed && !stopped; r++) for (const state of S.look ? ['off', 'on'] as const : ['on'] as const) {
      say(S.look ? `layers ${state}, turn ${r + 1} of ${PRICE.rounds}` : `turn ${r + 1} of ${PRICE.rounds}`);
      const sw = state === 'off' ? layersOff(w) : null; if (state === 'off' && !sw) { failed = 'this page has no way to put the look\'s layers off (its panel\'s rows are not on the sweep)'; break; }
      if (sw) { offBy = sw.how; undo.push(sw.back); }
      const got = await turn(w, c.secs, () => stopped); if (sw) { undo.pop(); sw.back(); }
      if (!got) { failed = stopped ? 'stopped' : 'the tab was away through every try'; break; }
      if (got.drew) { const d = acc[state].drew ?? (acc[state].drew = { secs: 0, frames: 0 }); d.secs += got.drew.secs; d.frames += got.drew.frames; }
      acc[state].ms.push(...got.ms); acc[state].gpu.push(...got.gpu); acc[state].all.push(...got.all); acc[state].work.push(...got.work); for (const [k, v] of got.passes) acc[state].passes.set(k, [...(acc[state].passes.get(k) ?? []), ...v]);
      if (size() !== size0 && !apart.length) apart.push(`the size, the scale or the sharp pass moved during the turns (${size0} → ${size()}): the columns are not the same picture`);
    }
  }
  if (stopped) { clear(); view?.progress(null); console.info('price every level: stopped'); return; }
  const next: Carry = { ...c, i: i + 1, live: -1 }, facts = { quality: w.__quality?.label ?? c.quality, scale: +E.settings.renderScale.toFixed(3), internal: [E.width, E.height] as [number, number], rung: rungOf(w), heldAt };
  const on = winOf(acc.on), ratio = (a: number | null, b: number | null) => a !== null && b ? +(a / b - 1).toFixed(4) : null;
  const bar = { gpuMore: PRICE.gpuMore, frameMs: PRICE.frameMs, lagFrames: PRICE.lagFrames, ruled: PRICE.ruled }, judged = (r: PriceRow): PriceRow => ({ ...r, over: r.apart.length ? [] : priceOver(r, bar) });   // (a row whose states or pages are not one picture is NOT JUDGED: an "under the bar" read off two different pictures would be a verdict on nothing)
  if (failed) next.notes = [...c.notes, `${L.level}${S.look ? '' : ', the page without the look'}: not priced — ${failed}`];
  else if (S.look) { const off = winOf(acc.off);
    if (off.gpu === null) notes.push('the browser stamps no GPU times here: the frame\'s time is all there is, and under the display\'s rate it says nothing');
    next.rows = [...c.rows, judged({ level: L.level, mission: L.mission, title: L.title, hour: L.hour, query: loc.search, ...facts, out: [E.outWidth, E.outHeight], off, on, gpuMore: ratio(on.gpu, off.gpu), allMore: added(on, off), msMore: ratio(on.ms, off.ms) ?? 0, gpuOverPage: null, allOverPage: null, msOverPage: null,
      offBy, programs: { boot: bootMade, after: afterMade - bootMade, bySwitch: made.n - afterMade }, boot: w.__boot?.line ?? '',
      apart, notes: [...notes, ...lateUnexplained(on, 'the layers on'), ...(spansWait(on) || spansWait(off) ? [`the g-buffer's stamp spans the wait for the display on this machine (with it ${on.all} ms, the compute passes alone ${on.gpu} ms, a frame every ${on.ms} ms): what the look's surface layers cost cannot be read here, and nothing is judged by "with the g-buffer"`] : [])], over: [] })]; }
  else { const at = c.rows.findIndex(r => r.level === L.level);   // (the page without the look: its turns go on the level's own row, beside the look page's)
    if (at < 0) next.notes = [...c.notes, `${L.level}: the page without the look was timed (GPU ${on.gpu ?? '—'} ms, with the g-buffer ${on.all ?? '—'} ms, a frame every ${on.ms} ms) but its look page was not`];
    else { const R = c.rows[at]; next.rows = c.rows.map((r, k) => k !== at ? r : judged({ ...R, page: { win: on, ...facts, programs: { boot: bootMade, after: made.n - bootMade }, boot: w.__boot?.line ?? '' }, gpuOverPage: ratio(R.on.gpu, on.gpu), allOverPage: added(R.on, on), msOverPage: ratio(R.on.ms, on.ms),
        notes: [...R.notes, ...notes.map(t => `the page without the look: ${t}`), ...lateUnexplained(on, 'the page without the look')],
        apart: [...R.apart, ...apart.map(t => `the page without the look: ${t}`),
          ...(facts.internal.join('×') !== R.internal.join('×') || facts.scale !== R.scale ? [`the page without the look drew at ${facts.internal.join('×')} ×${facts.scale}, the look page at ${R.internal.join('×')} ×${R.scale}: not the same size`] : []),
          ...(rungDiffers(R.rung, facts.rung).length ? [`the two pages stood on different rungs of the quality ladder (the look page / the page without it): ${rungDiffers(R.rung, facts.rung).join(', ')}`] : []),
          ...(heldAt !== R.heldAt ? [`the page without the look was held at ${heldAt ?? '—'} s of the game's clock, the look page at ${R.heldAt ?? '—'} s: not the same step`] : [])] })); } }
  if (i + 1 < of) { if (!write(next)) throw new Error('the session store took no more'); const N = c.steps[i + 1]; loc.replace(loc.pathname + priceSearch(N.level, i + 1, c.quality, N.look ? c.look : null, c.keep) + loc.hash); return; }
  // ---- the last page: the table, and the one file
  clear(); view?.progress(null);
  const M = machineFacts(E.gpu?.adapterInfo ?? '', E.gpu?.adapterFallback, w.__quality?.displayHzPin || w.__quality?.displayHz || 0); await Promise.race([M.more, new Promise(r => setTimeout(r, 1500))]);   // (the adapter the page drew on and the display's rate, as the benchmark's file has them)
  const file: PriceFile = { version: 5, what: 'twopointsix: what the look costs on every level, one held picture a level — the page without the look, the look page with its layers off, and with them on (ui/pricerun.ts)', at: new Date().toISOString(), startedAt: c.startedAt, address: c.from, look: c.look, userAgent: navigator.userAgent, gpu: M.now.webgpu, machine: M.now,
    screen: `${screen.width}×${screen.height}`, dpr: devicePixelRatio || 1, quality: c.quality, secs: c.secs, bar, rows: next.rows, notes: next.notes };
  // THE ONE FILE goes out by the page's one door (host/savefile.ts): to whoever listens, and to the browser as a download only where the address asks for the
  // door as it was (`?downloads=1`, a flag the run carries from the address that asked). The card says which, in the door's own words.
  const name = `price-levels-${file.at.replace(/[:.]/g, '-')}.json`, report = priceReport(file), text = JSON.stringify(file, null, 1); let local = false;
  // (a file a listener took is heard of again at its end: the card's line follows, and Close waits for one still on its way, as the benchmark's card does —
  //  for at most BENCH_SEND_MS; a second press leaves at once)
  let ended: () => void = () => { }; const end = new Promise<void>(r => { ended = r; });
  const went: BenchWent = { result: false, take: null, idle: () => Promise.race([end, new Promise(r => setTimeout(r, BENCH_SEND_MS))]) };
  try { const w2 = saveFile('price', name, new Blob([text], { type: 'application/json' }), { done: arrived => { went.result = arrived ? 'arrived' : false; ended(); try { view?.update?.(went); } catch { /* the card is gone */ } } }); went.result = w2.went; local = w2.local; }
  catch { /* a download was owed and the browser refused it: the file's text goes on the card */ }
  if (!went.result) ended();   // (nothing is on its way: Close has nothing to wait for)
  // WHERE THE FILE WENT, under the table. Taken by nobody (no listener, and the address did not ask for a download), the run would be gone with the tab:
  // the file's own text then stands under the table, so Copy keeps all of it
  const kept = went.result || local, said = `\n\n${local ? `The file: ${name}, in your downloads. ` : ''}${savedWords('The file', went.result)}${kept ? '' : ` Nothing took it, so its text is below: press Copy to keep the run, and save it as ${name}. Add downloads=1 to the address next time.\n\n${text}`}`;
  (window as unknown as { __priceLevels?: PriceFile }).__priceLevels = file;
  if (view) view.show(report + said, () => loc.assign(c.from || loc.pathname), went); else console.info(report);
}

/** what puts a row over the bar, in words (none = it is under it). Everything is read against the page without the look where the run has one (the
 *  budget's own "look off"), else against the layers off: the GPU's time by the game's own number, what the look adds over the compute passes with the g-buffer (where the
 *  stamps hold no display's wait), the frame's time and the GPU's lag — the last two are said with the baseline's beside them, as the machine's where the baseline is
 *  over the bar too. THE ONE PLACE a row is judged: the run writes these words on the row (`over`), and the table of levels prints them */
export function priceOver(r: PriceRow, bar: PriceFile['bar']): string[] {
  const pc = (v: number) => `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`, vs = r.page ? 'the page without the look' : 'its layers off', top = `the bar is +${(100 * bar.gpuMore).toFixed(0)}%`;
  const gpu = r.page ? r.gpuOverPage : r.gpuMore, all = r.page ? r.allOverPage : r.allMore, from = r.page ? r.page.win : r.off;
  return [gpu !== null && gpu > bar.gpuMore ? `on the GPU the look adds ${pc(gpu)} against ${vs} (${from.gpu} → ${r.on.gpu} ms; ${top})` : '',
    all !== null && all > bar.gpuMore ? `with the g-buffer counted the look adds ${((r.on.all ?? 0) - (from.all ?? 0)).toFixed(2)} ms, ${pc(all)} of the GPU's time on ${vs} (${from.all} → ${r.on.all} ms, its GPU ms ${from.gpu}; ${top})` : '',
    // (the frame's time and the lag are said with the baseline's beside them, and a cause is named only where the numbers carry it: both over is the machine's,
    //  and the look is said to put the GPU behind only where it is half as late again as the baseline)
    r.on.ms <= bar.frameMs ? '' : `a frame comes every ${r.on.ms.toFixed(1)} ms with the look on, ${from.ms.toFixed(1)} ms on ${vs}${from.ms > bar.frameMs ? ': neither holds sixty a second at this preset on this machine' : ''} (the bar is ${bar.frameMs} ms)`,
    r.on.lag === null || r.on.lag <= bar.lagFrames || !fullFrame(r.on) ? '' : `the GPU is done ${r.on.lag.toFixed(1)} frames after a frame began with the look on${from.lag !== null ? `, ${from.lag.toFixed(1)} on ${vs}` : ''}${
      from.lag !== null && from.lag > bar.lagFrames ? ': it keeps up on neither at this preset on this machine' : from.lag !== null && r.on.lag >= 1.5 * from.lag ? ': the look puts it behind, whatever the frame clock says' : ''} (the bar is ${bar.lagFrames})`,
    r.programs.bySwitch > 0 ? `switching the layers made ${r.programs.bySwitch} shader programs (the bar is none)` : ''].filter(Boolean);
}
/** THE TABLE, as text: a line a level, the levels over the bar marked and named under it */
export function priceReport(f: PriceFile): string {
  const pct = (v: number | null | undefined) => (v === null || v === undefined ? '—' : `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`).padStart(7), n = (v: number | null | undefined, d = 2) => (v === null || v === undefined ? '—' : v.toFixed(d)).padStart(7);
  const three = (a: number | null | undefined, b: number | null, c: number | null) => `${n(a)} ${n(b)} ${n(c)}`;
  // (a column is its title and its cell: both are padded to the wider, so the head and the lines stand in line whatever a column is called)
  const cols: [string, (r: PriceRow) => string, boolean?][] = [
    ['level', r => r.level, true], ['hour', r => r.hour, true], ['quality', r => r.quality, true], ['size', r => `${r.internal.join('×')} ×${r.scale.toFixed(2)}`, true],
    ['GPU ms: no look, layers off, on', r => three(r.page?.win.gpu, r.off.gpu, r.on.gpu)], ['on vs: no look, off', r => `${pct(r.gpuOverPage)} ${pct(r.gpuMore)}`],
    ['with the g-buffer ms: no look, off, on', r => three(r.page?.win.all, r.off.all, r.on.all)], ['look adds: vs no look, off', r => `${pct(r.allOverPage)} ${pct(r.allMore)}`],
    ['surface ms: off, on', r => `${n(r.off.passes?.[SURFACE])} ${n(r.on.passes?.[SURFACE])}`], ['GPU done, frames: no look, on', r => `${n(r.page?.win.lag, 1)} ${n(r.on.lag, 1)}`],
    ['a frame every: no look, on', r => `${n(r.page?.win.ms, 1)} ${n(r.on.ms, 1)}`], ['programs', r => `${r.programs.boot}+${r.programs.after}+${r.programs.bySwitch}${r.apart.length ? '   NOT JUDGED' : r.over.length ? '   OVER' : ''}`, true]];
  const cells = f.rows.map(r => cols.map(c => c[1](r))), wide = cols.map((c, k) => Math.max(c[0].length, ...cells.map(x => x[k].length)));
  const line = (x: string[]) => x.map((t, k) => cols[k][2] ? t.padEnd(wide[k]) : t.padStart(wide[k])).join(' | ').trimEnd();
  const head = line(cols.map(c => c[0])), lines = cells.map(line);
  const overs = f.rows.filter(r => r.over.length).map(r => `  ${r.level}: ${r.over.join('; ')}`), rowNotes = f.rows.flatMap(r => r.notes.map(t => `  ${r.level}: ${t}`));
  const aparts = f.rows.filter(r => r.apart.length).map(r => `  ${r.level}: ${r.apart.join('; ')}`);
  // the rungs the pages stood on, in words: one line where they are one, each named where they are not, and said plainly where a ladder was left on
  const rungs = f.rows.flatMap(r => [r.rung, ...(r.page ? [r.page.rung] : [])]), loose = rungs.filter(g => g.adaptive !== false).length;
  const rungWords = [...new Set(rungs.map(g => `${g.preset} ×${g.scale}${g.cap ? `, at most ${g.cap} rows` : ''}, gather at a ${g.gatherThird ? 'third' : 'half'}, volumetrics at a ${g.volQuarter ? 'quarter' : 'half'}, sharp ${g.sharp ? 'on' : 'off'}`))];
  const rungLine = !rungs.length ? '' : loose ? `THE QUALITY LADDER WAS LEFT ON on ${loose} of ${rungs.length} pages (see the notes): the pages stood on ${rungWords.length === 1 ? 'one rung' : `${rungWords.length} rungs`} — ${rungWords.join(' | ')}.`
    : rungWords.length === 1 ? `EVERY PAGE stood on its preset's own rows with the quality ladder off: ${rungWords[0]}.` : `THE PAGES STOOD ON ${rungWords.length} RUNGS though the ladder was off: ${rungWords.join(' | ')}.`;
  const waits = f.rows.some(r => spansWait(r.on) || spansWait(r.off) || (r.page ? spansWait(r.page.win) : false));
  return [`WHAT THE LOOK COSTS, a held picture a level  ·  ${f.at.slice(0, 16).replace('T', ' ')} UTC`, `${f.gpu}${f.machine.hz ? `  ·  a ${f.machine.hz} Hz display` : ''}  ·  screen ${f.screen} ×${f.dpr}  ·  asked for quality ${f.quality}, ${f.secs} s a turn  ·  the look page's words: ${f.look.join('&')}`, f.userAgent, '',
    head, ...lines, '',
    'no look    = the same mission on the page WITHOUT the look, held at the same step of the game\'s clock: the game as it is drawn today (the budget\'s own "look off").',
    'layers off = the look page with every layer of the look that has a cost of its own put off at once; on = the look page as it opens.',
    'GPU ms     = the GPU\'s busy span over the compute passes, as the page takes it everywhere: a render pass\'s stamps span the wait for the display, so they are left out,',
    '             and with them the look\'s surface layers (paint, relief, finish, weather, marks), which are shaded in one.',
    'with the g-buffer = the GPU\'s busy span over the compute passes AND the g-buffer\'s render pass: every stamped pass but the ones that draw to the display.',
    '             "look adds" is what the look puts on it, as a share of the baseline\'s GPU ms.',
    waits ? '             ON THIS MACHINE the g-buffer\'s stamp spans the wait for the display (the span reads about a whole frame): nothing is judged by it, and what the' : '             (Good where the g-buffer\'s stamp holds no wait for the display, as here. Where it does, the run says so and judges nothing by it.)',
    ...(waits ? ['             look\'s surface layers cost cannot be read here.'] : []),
    'surface ms = the stamp of the g-buffer\'s pass alone, where the surface layers are shaded.',
    'GPU done   = how many frames after a frame began the GPU said it had finished it. About 1.2 when it keeps up, 2 to 3 when the page is waiting for it. Not a cost.',
    'a frame every = the time between two frames the page DREW (it waits for the GPU, so a frame the GPU cannot finish in time costs display frames). On a display',
    '             it holds, that is the display\'s own (16.7 at 60 Hz, 8.3 at 120) and says only that it is held.',
    'programs   = shader programs the look page made before it was ready + in the seconds after + by switching the layers (that last should be 0).',
    'NOT JUDGED = the row\'s states or its two pages were not one picture (another size, rung or step of the game\'s clock): said under the table.',
    `OVER       = past the bar (${f.bar.ruled}): with the look on, GPU ms within ${(100 * f.bar.gpuMore).toFixed(0)}% of no look and no more than that added with the g-buffer counted, sixty frames a second held (a frame every ${f.bar.frameMs} ms or less, the GPU done within ${f.bar.lagFrames} frames), no program made by the switch.`,
    ...(rungLine ? [rungLine] : []),
    ...(overs.length ? ['', 'Over the bar:', ...overs] : f.rows.length ? ['', aparts.length === f.rows.length ? 'No level was judged.' : aparts.length ? 'No level that was judged is over the bar.' : 'No level is over the bar.'] : ['', 'No level was priced.']),
    ...(aparts.length ? ['', 'Not judged, because the row is not one picture (its numbers stand, no verdict is read off them):', ...aparts] : []), ...(rowNotes.length || f.notes.length ? ['', 'Notes:', ...f.notes.map(t => `  ${t}`), ...rowNotes] : [])].join('\n');
}
