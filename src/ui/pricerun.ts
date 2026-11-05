// PRICE EVERY LEVEL (`?pricelevels` on the look page's address): one address, hands off, and the machine it ran on has said what the look costs it on
// every level the mission list draws (the maintainer, 11-03: "we'll want to dress up all the levels and check their budget"; 11-02: "the main thing i'm concerned
// about before merging is performance", "we're going to need to run the tests across more than just the pass"). A level is only ever loaded at boot
// (scene/scenarioList.ts), so a run is a chain of page loads, as the benchmark's is a page a scene (ui/benchrun.ts): a small record in the tab's own session store
// carries the rows measured so far from one page to the next. The last page shows the table in a box with a Copy button, and the whole run is ONE file
// (price-levels-<date>.json) that leaves by the page's one door (host/savefile.ts): to whoever listens there, and to the browser's downloads where the
// address that asked says `downloads=1` (the run carries that word). Where neither took it the card says so and holds the file's own text under the table,
// so Copy keeps the run.
//   WHAT A LEVEL'S TWO PAGES DO. First its mission's own address ON THE LOOK PAGE, at the hour its row opens at, the die pinned: it plays a few seconds so the
//   men stand where the mission puts them (by the page's scripted step, a step a frame, after the page's own warm-up: PRICE.playS), then HOLDS THE WORLD STILL and times that one picture with the look's layers off and on in turn (off, on, off, on):
//   the frame's time, and where the browser stamps the passes, the GPU's. Then the same mission on the page WITHOUT the look (no `look` on the address), held at
//   the same step of the game's own clock and timed as it stands. Then the next level. Both are a run's pages (host/runpage.ts): the kit and the fog of war
//   as the mission deals them, no sandbox row of this machine's, so the two pictures are of one scene. AND ONE RUNG OF THE QUALITY LADDER: a page of a run
//   says the page's own words for it on its address: `quality=<preset>` pins the preset and keeps the climb off, `adapt=0` switches the ladder off (the
//   menu's "Hold frame rate") before the first frame is sampled, both read by the ladder's one reader (ui/quality.ts visit), and `giveway=0` keeps the
//   window from giving way to one in focus elsewhere (host/pace.ts). No rule of the page knows a priced page. With these, the ladder does nothing: the scale, the
//   cap on rows, the gather's and the volumetrics' resolution and the sharp pass are the preset's own on both pages, whatever this machine's frame rate. Each
//   page writes what it stood on (`rung`), and a level whose two pages differ in any of it says so. (First run on a real GPU, 11-04: left to itself the
//   ladder cut the look page of two levels from High's own 0.77 of the window to 0.75 and left their pages without the look at 0.77.)
//   TWO MEANINGS OF "LOOK OFF", both in the file:
//     · THE PAGE WITHOUT THE LOOK is the budget's own ("GPU time within 10% of look off"): the game as it is drawn today. It is another page: other shader
//       text, and the level as its file has it (none of the look's round things and broken wall tops). Its row is `page`; the table's OVER is read against it.
//     · THE LAYERS OFF is the same page, shaders, shapes, camera, quality and scale, with every row the look panel lists as a layer with a cost of its own
//       (ui/lookpanel.ts hands them to the sweep as `look.<row>`) at its own off value, all at once. It says what the layers cost where they run, and it is NOT
//       the panel's "plain boxes", which leaves the lighting's, the smoke's and the eye's layers on.
//   AND THE WORLD RUNNING (`run`, 11-05). A held picture steps nothing: on a step of no length the smoke solver encodes nothing, the wind over the floor and
//   the loose snow are not stepped, and a flake or a drop is handed back as it was. So a level with snow, rain, a fire or wind read cheaper held than
//   played (a walk of 29 levels had no stamp of the solver in any row, and its wind pass read 0). After a page's held turns the SAME scripted steps play on
//   to PRICE.runS of the game's clock, every step drawn, and one more turn is timed with the world still stepping: a step of the game's clock at the
//   game's own rate, paced by the page's own pacer (host/pace.ts) where the GPU has room. Both pages of a level take the same steps up to that second. The running read stands BESIDE the held one
//   (`run`, `page.run`), with its own verdict (`overRun`) and its own reasons not to be judged (`runApart`): the held read is one picture on both pages and
//   says what each layer costs; the running read is nearer what is played.
//   WHAT IT DOES NOT SAY. A fight's frame (nobody fires at a mission's start, held or running) and a first visit's compile (ui/boot.ts times that; each row keeps the
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
//   The frame's time is read AGAINST THE BASELINE too: a machine that does not hold sixty at this preset without the look is not the look's doing. The lag
//   is no line of the verdict (it reads 2 to 3 wherever the page waits for the GPU, and more on a fast display): far past that (PRICE.lagFrames) frames were
//   queued ahead of the GPU, a state's stamps may hold another's work, and the row is NOT JUDGED. So is a row where the GPU's answer came late for no work
//   of the state's own (PRICE.lateMs): the lag in frames cannot see that, because the page waits for the GPU and its frames slow with the answer.
//   THE WORDS. `?pricelevels` = every level; `?pricelevels=pass,block` = these; `&pricesecs=N` = N seconds measured a turn (default 3); `&quality=<preset>` =
//   the preset every page stands at (default high: the budget is said for High; `auto` lets each page settle its own, and the row says where it settled);
//   `&pricepage=0` = the look page only, half the loads; `&pricelive=N` = the world runs to second N before its read (default PRICE.runS; 0 = held pictures only). THE LOOK'S OWN WORDS on the address that asked (`look=walls:0`, `geo=0` …) ride on every look page
//   of the run, with the trials this machine's look panel keeps off, as the benchmark carries them (ui/benchrun.ts): the file says which look was priced.
//   A reload in the middle does not start it again (the page finds its own mark and is a plain mission page); Esc stops a run.
// THE PAGE'S HALF IS ONE LINE (main.ts, before the boot): `if (priceLevelsPage(q)) return;`. Everything else is asked of what the page already puts on `window`
// for its checks (__ready, __engine, __game, __quality, __perfsweep, __boot): this file adds no hook and no state to the game.
import { listedLevels } from '../scene/scenarioList';
import { pacer } from '../host/pace';
import { scenarioQuery } from '../scene/scenarios';
import { lookTrialsKept } from '../scene/look';
import { HELD_LADDER_WORD, HELD_WINDOW_WORD, PRICE_ASK_WORDS, PRICE_LEVELS_WORD, PRICE_RUN_WORD } from '../host/runpage';
import { PRESENT_PASSES, RENDER_PASSES, SURFACE_PASS } from '../gpu/timer';
import { saveFile, savedWords } from '../host/savefile';
import { BENCH_SEND_MS, benchView, machineFacts, type BenchMachine, type BenchWent } from './bench';
import { carriedWords, lookWord } from './benchrun';

const KEY = 'twopointsix.pricelevels';
/** the word that asks for a run · the word on a level's page of one: its place in the run */
export const PRICE_KEY = PRICE_LEVELS_WORD, PRICE_RUN_KEY = PRICE_RUN_WORD;
/** the render passes, which the GPU's busy span leaves out everywhere on the page · the one of them the look's surface layers are shaded in: its stamp is
 *  printed beside and is inside `all` · and the ones that draw to the display, which `all` leaves out on every machine (gpu/timer.ts has all three and why) */
const DRAWN = [...RENDER_PASSES], SURFACE = SURFACE_PASS, PRESENTS = [...PRESENT_PASSES];
export const PRICE = {
  /** seconds OF THE GAME'S OWN CLOCK the mission plays before its picture is held. ONE WAY, on every page: the page's loop is held from before its boot and
   *  the mission is played in fixed sixtieths by the page's own scripted step (main.ts `__runFrames`), ONE STEP A DISPLAY FRAME, EVERY STEP DRAWN, to exactly
   *  the step asked. So a level's two pages have run the same steps, drawn the same number of frames and stand on the same render clock (a lamp's flicker
   *  reads it), whatever their frames cost and whatever the display's rate; a lesson whose card holds the clock on a page nobody plays is stepped like the
   *  rest. The play WAITS FOR THE GPU as the page's own pacer does (host/pace.ts): no step is drawn while as many frames are still with the GPU as the
   *  pacer keeps there (`window.__pace().flight`; `flight` here where a build does not say), so what the game reads back from the GPU comes as late as on a
   *  player's page and no later.
   *  THE PAGE'S WARM-UP COMES FIRST: a page clears every particle and all smoke some frames after its first frame of its own (main.ts `__prewarm`: a
   *  throwaway can and spark that touch the pipelines once). Held from its boot and stepped, that clear came after the play, with the clock held, and A
   *  SNOW LEVEL WAS PRICED WITH NO SNOW IN ITS AIR (11-04, a rented card: its fog pass read 1.4 ms too cheap). So the run asks for the warm-up itself and
   *  lets the page draw with the clock held until the page says the clear is over (`window.__prewarmed`; `warmFrames` of its own frames on a build that
   *  does not say) before a step is played: the clear is over before a flake falls */
  playS: 4, flight: 2, warmFrames: 16,
  /** THE GPU CATCHES UP BEFORE ANYTHING IS TIMED: a boot hands it seconds of work (its pipelines, its first frames), and for a while after the queue's "done"
   *  comes many frames late whatever a frame costs (11-04, a rented card on a long run: hundreds of frames behind on a page with 6 ms of work, with the look
   *  and without). The held picture is drawn until a frame's work is done within `lagFrames` frames three times running, for at most this many seconds; a
   *  page still behind then says so on its row, and there the lag is the page's own. 0 = no wait (a check on an adapter whose frame is seconds long) */
  catchS: 30,
  /** a turn: the start that is dropped, then the part that is measured (seconds; `pricesecs` on the address is the second) */ settleS: 1.2, measureS: 3,
  /** … and each wants this many frames, and gives up waiting for them after `capS` (a machine at two frames a second) */ settleFrames: 12, measureFrames: 30, capS: 25,
  /** turns of each state */ rounds: 2,
  /** THE WORLD RUNNING (the header): the second of the game's clock a page is played on to after its held turns, by the same scripted steps, before one more
   *  turn is timed with the world still stepping. TWENTY, BY ONE MEASURE (11-05, a rented card, the snow road with three open fires near the player,
   *  stepped as this run steps for 60 s and read every 5 s): the smoke stood at 1,289 bricks after 5 s, 1,947 after 10 and 2,141 after 15, and from there
   *  to 60 s wandered between 1,850 and 2,310; the stamped passes read 21.7 ms over the first 5 s, 23.5 over the next 5, and 23.6 to 24.4 from 15 s on. So
   *  a plume has levelled by 15 s and 20 leaves a margin. One level, one card: a bigger fire may take longer. 0 = not asked (`pricelive` on the address);
   *  a second that is not past the one the picture is held at is no running read, and the row says so.
   *  THE TURN STEPS THE WORLD ITSELF, a sixtieth of the game's clock a frame, the frames paced to sixty a second by THE PAGE'S OWN PACER (host/pace.ts
   *  `pacer`: the display's period read off the callbacks, a display frame passed over where the rate asks or the GPU has no room, a frame the queue never
   *  answers given up). So the world runs at the game's rate on a 60 or a 120 Hz display (on a 144 Hz one the pacer's rule draws 72, and the world runs a
   *  fifth fast; with `?pace=0`, the page's word for no even pace, every display frame is stepped). NOT THE PAGE'S OWN LOOP, though that is what a player's page runs: a card on the page
   *  freezes that loop (main.ts `frozen`: a lesson's card on a tutorial nobody plays), and a scripted step is the one way the world runs on every level,
   *  as the play before the held picture already knows. What that costs in truth: a played page on a fast display draws between its steps too, and this
   *  turn does not.
   *  THE LATE ANSWER'S RULE READS THIS TURN AGAINST ITS OWN KIND: the least answer among the running states, not the held ones' (a running state is paced
   *  to sixty and its frame holds the game's step, so on a fast display it answers sooner than a held state and would make every held state look late).
   *  Its 12 ms was still set by held states of one walk. Not seen yet: no GPU has run it.
   *  A PERSON'S RUN IS ABOUT TWICE AS LONG WITH IT, in one tab, and a long walk tires (lateMs): the held read, which the ruled bar is read from, is then
   *  at more risk on the later levels. `pricelive=0` is the run as it was */
  runS: 20,
  /** a frame longer than this (seconds) is the tab having been away: the turn starts over, at most `retries` times */ awayS: 1.5, retries: 3,
  /** THE BAR the table sets each level against (the look task's, 11-02; `ruled` below says which of its rows the maintainer has ruled on): the GPU's time with the layers on within this of off */ gpuMore: 0.10,
  /** which rows stand by the maintainer's word and which are still a proposal: written into the file and the report, so neither claims more than was ruled */ ruled: 'GPU time within 10% of no look and sixty frames a second held were ruled on 2026-11-04; the same with the g-buffer counted and "no program made by the switch" are still proposed',
  /** … and sixty frames a second held on High. Between two frames of a 60 Hz display there are 16.67 ms, which the proposal's "under 16.6" would call over, so
   *  the bar on a frame's time is that with 2% of the clock's noise on it */ frameMs: 17,
  /** FRAMES QUEUED AHEAD OF THE GPU: its "done" for a frame later than this many drawn frames after the frame began. The page waits for the GPU with two
   *  frames in flight, so the lag is about 1.2 when it keeps up and 2 to 3 while the page waits for it, more on a fast display where the queue's answer
   *  is itself a few frames long. Past this the stamps of one state can hold another's work (11-04: 629 frames late, off and on read alike), so the row is
   *  NOT JUDGED. It is no bar of the budget: whether sixty a second are held is the frame's time */ lagFrames: 8,
  /** THE GPU'S ANSWER COMES LATE FOR NO WORK OF THE STATE'S OWN. From a frame's start to its "done" (`work`) a state may take as many frames of its own
   *  stamped passes as the page keeps with the GPU (two, its own and one ahead: host/pace.ts `flight`) and the machine's own time to answer, which no level
   *  changes and which is read off THE SAME RUN: the least any of its states took beyond its own passes (`answerOf`). A state over that sum by more than
   *  this many ms is late (`lateRows`). Read against the run's own least and not a fixed time, because a browser's answer is itself a frame or a few long,
   *  and longer on a fast display. Seen 11-05 on a rented card that walked 29 levels in one browser: from about the 15th level the answer came later and
   *  later (to 57 ms for 10 ms of passes) while the pages' boots kept their time, and the last seven rows read the look page CHEAPER than the page without
   *  it with every check green: the lag in drawn frames stayed near 1.7, because the frames slowed with the answer. The same seven levels alone drew a
   *  frame every 17 ms and read sane. THE NUMBER IS SET BY THAT ONE WALK: its least answer was 8 ms, the 22 rows that read right had no state over 8.4 by
   *  this measure (most stand below nought: two frames of passes explain more than a fresh browser needs) and each of the seven had one at 21 to 28. A
   *  walk of look pages only has two states a row to be caught by: there the walk's pass (4) would have slipped through and its bazaar (16) not. Something
   *  the stamps do not show holds the GPU, so the row is NOT JUDGED. 0 = the rule is not asked. A run of one level has little to be read against */ lateMs: 12,
  /** THE G-BUFFER'S STAMP HOLDS THE DISPLAY'S WAIT where the compute passes with the g-buffer take this much of the time between frames (`waitAll`) and the
   *  g-buffer puts this much of that time on the compute passes (`waitGap`). HALF A FRAME, on purpose: a g-buffer that is honest work must never trip it. On
   *  an M4 Max the look page's g-buffer was 1.8 to 2.3 ms; at 120 Hz that is over a quarter of the 8.3 ms between frames, and a page that fills its frame
   *  there would be called a wait at a quarter (the bar it was until 11-04). A wait smaller than half a frame goes unseen: the span then reads about the
   *  frame's whole length on the baseline too, and the look adds nothing to it */ waitAll: 0.85, waitGap: 0.5,
};

/** one state's turns on one page */
export interface PriceWin { /** frames the page drew in the measured turns · the time between two of them, ms (the header) · the 95th centile of the time between two frame callbacks */ frames: number; ms: number; p95: number; /** the GPU's busy span over the compute passes, ms (null: the browser stamps none) */ gpu: number | null;
  /** THE COMPUTE PASSES AND THE G-BUFFER, ms: the GPU's busy span over every stamped pass but the ones that draw to the display (the header; until version 3 it was every stamped pass; null: no stamps) */ all: number | null;
  /** from a frame's start to the GPU having finished all the frame handed it, ms (null: no device to ask) … */ work: number | null; /** … and that in frames: how far behind the GPU is (the header: not a cost) */ lag: number | null;
  /** each pass's own mean, ms */ passes: Record<string, number> | null }
/** a page's read with THE WORLD RUNNING (PRICE.runS): the turn, the game's clock where it ended, seconds (null: the page shows no clock), and the shader
 *  programs made from the run-up's first step to the turn's end (a first use while the world ran: no line of the bar, said on the row) */
export interface PriceRun { win: PriceWin; at: number | null; made: number }
/** THE RUNG OF THE QUALITY LADDER a page was priced on: everything of the picture the ladder can move (ui/quality.ts) */
export interface PriceRung { preset: string; /** the ladder's switch as the turns found it (false: held) */ adaptive: boolean | null; scale: number; /** the cap on internal rows */ cap: number | null; gatherThird: boolean | null; volQuarter: boolean | null; sharp: boolean | null }
export interface PriceRow {
  level: string; mission: string; title: string; hour: string; query: string;
  quality: string; scale: number; internal: [number, number]; out: [number, number]; rung: PriceRung;
  off: PriceWin; on: PriceWin;
  /** the game's clock where the picture was held, seconds (null: the page shows no clock) */ heldAt: number | null;
  /** layers on over layers off, less one: of the GPU's span where there is one, of the time between frames; and what the layers add to the compute passes with the g-buffer, as a share of the layers-off GPU's span */ gpuMore: number | null; allMore: number | null; msMore: number;
  /** THE PAGE WITHOUT THE LOOK, the same mission held at the same step (absent: the run was asked for without it, or that page did not finish) */
  page?: { win: PriceWin; quality: string; scale: number; internal: [number, number]; rung: PriceRung; heldAt: number | null; programs: { boot: number; after: number }; boot: string; /** … and with the world running */ run?: PriceRun };
  /** THE LOOK PAGE WITH THE WORLD RUNNING, its layers on (absent: not asked, a file from before 11-05, or the page has no scripted step) */
  run?: PriceRun;
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
  /** … and the same two of THE RUNNING READ (absent where the row has none): why it is not judged (frames queued, a late answer; a row whose held read is
   *  apart is not judged running either: its two pages are not one size or rung), and what puts it over the bar (priceOverRun) */
  runApart?: string[]; overRun?: string[];
}
/** a page of the run: a level on the look page, or on the page without it */
interface Step { level: string; look: boolean }
interface Carry { v: 6; from: string; startedAt: string; steps: Step[]; i: number; live: number; quality: string; secs: number; /** the second the world runs to before its read (0: held pictures only) */ run?: number; /** the look page's words, as the address that asked wrote them */ look: string[]; /** the visitor's flags every page of the run carries (ui/benchrun.ts carriedWords) */ keep: string[]; rows: PriceRow[]; notes: string[] }
export interface PriceFile { version: 6; what: string; at: string; /** the build the pages were (the page's own word for it, as a benchmark's file has it; none where the page has none) */ commit?: string; startedAt: string; address: string; /** the words every look page of the run carried */ look: string[]; userAgent: string; /** the adapter the page drew on */ gpu: string; machine: BenchMachine; screen: string; dpr: number; quality: string; secs: number; bar: { gpuMore: number; frameMs: number; lagFrames: number; /** (from 11-05: a file from before has none, nor has a run that did not ask the rule) */ lateMs?: number; /** the second the world ran to before its read (none: held pictures only) */ runS?: number; ruled: string }; /** the least any state of the run took to answer beyond its own passes, ms (answerOf: the machine's own; a state late against it by more than bar.lateMs put its row apart) */ leastAnswer?: number; /** … and the least any RUNNING state took, which the running states are read against (lateRows) */ leastAnswerRun?: number; rows: PriceRow[]; notes: string[] }

const read = (): Carry | null => { try { const t = sessionStorage.getItem(KEY), c = t ? JSON.parse(t) as Carry : null; return c && c.v === 6 && Array.isArray(c.rows) && Array.isArray(c.steps) && Array.isArray(c.look) && Array.isArray(c.keep) ? c : null; } catch { return null; } };
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
  const q = new URLSearchParams(search), own = search.replace(/^\?/, '').split('&').filter(p => !!p && lookWord(p));   // (with or without a `look` word beside them: every look page of a run is one, so `?pricelevels&geo=0` prices the look without that trial)
  return [...(own.some(p => /^look(=|$)/.test(p)) ? [] : ['look']), ...own, ...lookTrialsKept(q)];
}
/** the address a step is priced on: its mission's own words (on the look page with that page's words, or with none of them), the die pinned, no verdict kept of
 *  the try and no replay filed (the run is nobody's play: as a benchmark's scene, ui/benchrun.ts), no sound, the preset, and its place in the run */
export function priceSearch(level: string, i: number, quality: string, look: string[] | null = ['look'], keep: string[] = []): string {
  const L = priceLevels().find(l => l.level === level); if (!L) throw new Error(`no level '${level}' on the mission list`);
  // (the page's own words for a held picture: the preset pinned and the ladder off, the window giving way to nobody. `auto` leaves the ladder to the page)
  const own = `${L.query.slice(1)}&seed=1&noverdict&nokeep&mute${quality === 'auto' ? '' : `&quality=${quality}&${HELD_LADDER_WORD}`}&${HELD_WINDOW_WORD}&${PRICE_RUN_KEY}=${i}`, has = new URLSearchParams(own);
  return `?${[...[...(look ?? []), ...keep].filter(p => !has.has(p.split('=')[0])), own].join('&')}`;   // (a word the page's own address says for itself — its preset, no sound — is not said twice)
}

/** THE WORDS OF THE ASKING ADDRESS NO PAGE OF THE RUN CARRIES (the benchmark names its own the same way, ui/benchrun.ts `left`): every word that is neither
 *  the run's own, nor the look page's, nor one of the visitor's flags a run carries, nor said by a level's own address. A mission's words are among them (an
 *  hour, a level file: each level is priced on its own row's). The file says them, so its `address` never claims a word its pages did not have */
export function priceLeft(search: string, level: string, quality: string, look: string[], keep: string[]): string[] {
  const own = new URLSearchParams(priceSearch(level, 0, quality, look, keep));
  return [...new Set([...new URLSearchParams(search).keys()])].filter(k => k && !PRICE_ASK_WORDS.includes(k) && !own.has(k));
}

/** … and THE WORDS OF THE ASKING ADDRESS THE RUN SAYS FOR ITSELF WITH ANOTHER VALUE (its die, its preset's hold, its window's word), each as asked and as the
 *  pages have it: the file says these too, for the same reason */
export function priceOverruled(search: string, level: string, quality: string, look: string[], keep: string[]): string[] {
  const own = new URLSearchParams(priceSearch(level, 0, quality, look, keep)), asked = new URLSearchParams(search);
  return [...new Set([...asked.keys()])].filter(k => k && !PRICE_ASK_WORDS.includes(k) && k !== PRICE_RUN_KEY && own.has(k) && own.get(k) !== asked.get(k)).map(k => `${k}${asked.get(k) ? `=${asked.get(k)}` : ''} (the pages have ${k}${own.get(k) ? `=${own.get(k)}` : ''})`);
}

/** THE PAGE'S ONE LINE, before its boot. True = this address asked for a run and the first level's address is loading: boot nothing. False = boot as ever; on a
 *  level's page of a run under way the page's part of it starts by itself once the page is ready. */
export function priceLevelsPage(q: URLSearchParams, loc: Location = location): boolean {
  if (q.has(PRICE_KEY) && !q.has(PRICE_RUN_KEY)) {
    const all = priceLevels().map(l => l.level), asked = (q.get(PRICE_KEY) ?? '').split(',').map(s => s.trim()).filter(Boolean), levels = asked.length ? asked.filter(l => all.includes(l)) : all;
    const secs = parseFloat(q.get('pricesecs') ?? ''), quality = q.get('quality') ?? 'high', both = q.get('pricepage') !== '0', run = parseFloat(q.get('pricelive') ?? '');
    const steps: Step[] = levels.flatMap(level => both ? [{ level, look: true }, { level, look: false }] : [{ level, look: true }]);
    const rest = loc.search.replace(/^\?/, '').split('&').filter(p => p && !PRICE_ASK_WORDS.includes(p.split('=')[0]));   // (the address that asked, with the run's own words off it: where the card's Close goes)
    const c: Carry = { v: 6, from: loc.pathname + (rest.length ? '?' + rest.join('&') : '') + loc.hash, startedAt: new Date().toISOString(), steps, i: 0, live: -1, quality, secs: isFinite(secs) && secs > 0 ? secs : PRICE.measureS, ...(isFinite(run) && run >= 0 ? { run } : {}), look: priceLookWords(loc.search), keep: carriedWords(loc.search), rows: [],
      notes: [...(asked.length && levels.length < asked.length ? [`not on the mission list, left out: ${asked.filter(l => !all.includes(l)).join(', ')}`] : []),
        ...(q.has('pricelive') && !(isFinite(run) && run >= 0) ? [`pricelive=${q.get('pricelive')} is no second of the game's clock: the world ran to the run's own ${PRICE.runS} s`] : [])] };
    if (levels.length) { const left = priceLeft(loc.search, levels[0], quality, c.look, c.keep), over = priceOverruled(loc.search, levels[0], quality, c.look, c.keep);
      if (left.length) c.notes.push(`every page was priced without ${left.join(', ')}: ${left.length > 1 ? 'words' : 'a word'} of the address that asked which no page of a run carries`);
      if (over.length) c.notes.push(`the address that asked said ${over.join(', ')}: a run's pages say ${over.length > 1 ? 'these' : 'this'} for themselves`); }
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
interface PageHooks { __prewarm?: () => boolean; /** the page's warm-up has cleared what it threw in (main.ts; undefined: a build without the word) */ __prewarmed?: boolean; __perfrec?: { commit?: string };
  __ready?: Promise<unknown>; __timeScale?: number; __pauseLoop?: boolean; __runFrames?: (n: number, dt?: number, keys?: string[], draw?: boolean | 'none') => void; __menu?: { isOpen?: boolean }; __game?: { time: number; /** the mission's own word: its stage, whether the alarm is on, and its count of alarm episodes (someone went to alert while nobody was) */ mission?: { stage?: string; alarmOn?: boolean; stats?: { alerts?: number } } };
  __engine?: { width: number; height: number; outWidth: number; outHeight: number; settings: { renderScale: number; sharpDirect?: boolean; maxInternalHeight?: number; gatherThird?: boolean; volQuarter?: boolean }; gpu?: { adapterInfo?: string; adapterFallback?: boolean }; device?: { queue: { onSubmittedWorkDone(): Promise<unknown> } };
    timer?: { enabled: boolean; resolved: number; lastFrame: Map<string, number>; unionMs(exclude: string[]): number } };
  __quality?: { label: string; calibrating: boolean; displayHz?: number; displayHzPin?: number; /** the preset standing · auto: it moves by itself · the ladder's switch (the menu's "Hold frame rate") · a preset's rows put on */ current?: string; auto?: boolean; adaptive?: boolean; apply?: (name: string) => void; /** the ladder's sample of a frame (a method of its class: an own one put over it stands in until it is deleted) */ sample?: (dt: number, gpuMs?: number) => void };
  __perfsweep?: { extra?: { id: string; flip(): string | { apply(): () => void } }[] };
  __boot?: { line?: string }; __cut?: { programs?: { swap?: boolean; ready?: Promise<unknown> } };
  /** the page's pacer (host/pace.ts): display frames it WORKED since the page began (the others it passed over: no room with the GPU, a rate being held) */
  __pace?: () => { worked: number; skipped: number; /** frames it keeps with the GPU (0: it does not wait) */ flight?: number };
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

/** the wait of PRICE.catchS: null = the GPU has caught up (or cannot be asked); else how many DISPLAY frames behind its answer still came when the time ran
 *  out. Against the display's own frames, not the frames the page drew: the page waits for the GPU (the pacer's `flight`), so a backlog after the boot shows
 *  as few frames drawn, each answered about two of ITS frames late whatever the backlog. The display's frame is the clock a backlog shows against. (A
 *  turn's lag is in drawn frames: there it asks another question, whether frames were queued ahead) */
async function caughtUp(w: PageHooks, stop: () => boolean): Promise<number | null> {
  const dev = w.__engine?.device; if (!dev || PRICE.catchS <= 0) return null;
  const t0 = performance.now(); let good = 0, lag = 0;
  while (performance.now() - t0 < PRICE.catchS * 1000 && !stop()) {
    const a = await frame(), b = await frame(), dt = Math.min(100, Math.max(4, b - a));   // (two frames running: the display's frame, then a frame asked whole as a turn asks it)
    await new Promise<void>(r => { const ch = new MessageChannel(); ch.port1.onmessage = () => { ch.port1.close(); r(); }; ch.port2.postMessage(0); });
    await dev.queue.onSubmittedWorkDone().catch(() => { });
    lag = (performance.now() - b) / dt;
    if (lag <= PRICE.lagFrames) { if (++good >= 3) return null; } else good = 0;
  }
  return stop() ? null : lag;
}
const NO_FRAME = 'the page drew no frame through a turn though the display\'s frames came: its loop has stopped';
/** one turn: the frames of `settle` dropped, then `measure` kept. Null = the tab was away through every try. Throws where the page's loop has stopped (the
 *  page is then filed as not priced, with that cause: `leg`). `drive` = THE WORLD RUNNING (PRICE.runS): the page's loop is paused and the turn itself asks
 *  for a step of the world on each frame callback, handing over the callback's stamp (true: one was stepped and drawn; false: this display frame was
 *  passed over, for the rate or for the GPU), so the frames the page drew are the steps taken */
async function turn(w: PageHooks, secs: number, stop: () => boolean, drive?: (ts: number) => boolean): Promise<Bag | null> {
  const T = w.__engine?.timer, dev = w.__engine?.device;
  for (let tries = 0; tries <= PRICE.retries; tries++) {
    const ms: number[] = [], gpu: number[] = [], all: number[] = [], work: number[] = [], asked: Promise<void>[] = [], passes = new Map<string, number[]>(); let away = false, last = await frame(), seen = T?.resolved ?? 0;
    // THE FRAMES THE PAGE DREW, by its pacer's own count: the page waits for the GPU (main.ts `flight`), so on a level the GPU cannot finish in a display
    // frame it passes display frames over, and the time between this file's own frame callbacks stays the display's. A frame's time is the measured part's
    // length over the frames worked in it; and the GPU is asked for "done" only on a frame that was worked (a passed-over frame handed it nothing)
    let stepped = 0; const worked = drive ? () => stepped : () => w.__pace?.().worked, w0 = worked(); let workedAt = w0, workedLast = w0;
    const drawn = (since: number | undefined, callbacks: number) => { const n = worked(); return n !== undefined && since !== undefined ? n - since : callbacks; };   // (frames since a count of the pacer's; this file's own callbacks where there is no pacer to ask)
    // THE FRAME WHOLE: asked in a task of its own, which the browser runs after every frame callback of this frame (the page's, which hands the GPU its
    // work, is one of them, before this file's or after it); the queue then answers once the GPU has finished all of it. From the frame's own start
    const whole = (start: number) => { if (!dev) return; asked.push(new Promise<void>(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => { ch.port1.close();
      const n = worked(); if (n !== undefined && n === workedLast) { res(); return; } workedLast = n;   // (the page passed this display frame over)
      dev.queue.onSubmittedWorkDone().then(() => { work.push(performance.now() - start); res(); }, () => res()); }; ch.port2.postMessage(0); })); };
    const t0 = last; let kept = -1;   // (the moment the measured part began)
    for (;;) { const now = await frame(), dt = now - last; last = now; if (stop()) return null;
      if (dt > PRICE.awayS * 1000 || document.hidden) { away = true; break; }
      if (drive?.(now)) stepped++;
      if (kept < 0) { if (now - t0 >= PRICE.capS * 1000 && w0 !== undefined && drawn(w0, 0) <= 0) throw new Error(NO_FRAME);
        if ((now - t0 >= PRICE.settleS * 1000 && drawn(w0, ms.length) >= PRICE.settleFrames) || now - t0 >= PRICE.capS * 1000) { kept = now; ms.length = 0; seen = T?.resolved ?? 0; workedAt = workedLast = worked(); } else ms.push(dt); continue; }
      ms.push(dt); whole(now);
      if (T?.enabled && T.resolved !== seen) { seen = T.resolved; gpu.push(T.unionMs(DRAWN)); all.push(T.unionMs(PRESENTS)); for (const [k, v] of T.lastFrame) { let a = passes.get(k); if (!a) passes.set(k, a = []); a.push(v); } }
      if ((now - kept >= secs * 1000 && drawn(workedAt, ms.length) >= PRICE.measureFrames) || now - kept >= PRICE.capS * 1000) { const n = worked(), drew = n !== undefined && workedAt !== undefined ? { secs: (now - kept) / 1000, frames: n - workedAt } : null;
        if (drew && drew.frames <= 0) throw new Error(NO_FRAME);   // (the display's frames came and the page drew none of them: its loop has stopped. Not a tab that was away, and not a frame time of the display's)
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
/** FRAMES WERE QUEUED AHEAD OF THE GPU in this state (PRICE.lagFrames), in words (none: they were not): what makes its row one that is not judged */
export const queuedWords = (w: PriceWin, what: string, lagFrames = PRICE.lagFrames): string[] => w.lag !== null && w.lag > lagFrames
  ? [`${what}: the GPU's "done" came ${w.lag.toFixed(1)} frames after a frame began (past ${lagFrames}: frames were queued ahead of it, so this state's stamps may hold another's work)`] : [];
/** a state's stamped passes as the late answer's rule counts them, ms: the compute passes with the g-buffer, or the compute passes alone where the
 *  g-buffer's stamp holds the display's wait (`spansWait`: there `all` is about a whole frame whatever the work); null: no stamps */
const stampedOf = (w: PriceWin, waits = spansWait(w)): number | null => waits ? w.gpu : w.all;
/** the machine's time to answer in a state: from a frame's start to its "done", less the state's own stamped passes, ms (null: no stamps, or no device to
 *  ask). The least of a run is its machine's own (PRICE.lateMs). `waits` = the g-buffer's stamp holds the display's wait on this page (the state's own
 *  word; a running state takes its page's held state's, as its frames are paced to sixty and cannot show a faster display's wait) */
export const answerOf = (w: PriceWin, waits = spansWait(w)): number | null => { const g = stampedOf(w, waits); return w.work !== null && g !== null ? +(w.work - g).toFixed(1) : null; };
/** how far past what explains it a state's answer came, ms: its `work` less a frame of its own passes for each frame the page keeps with the GPU, and less
 *  the run's least answer (PRICE.lateMs) */
export const lateBy = (w: PriceWin, least: number, flight = 2, waits = spansWait(w)): number | null => { const g = stampedOf(w, waits); return w.work !== null && g !== null ? +(w.work - Math.max(1, flight) * g - least).toFixed(1) : null; };
/** THE RUN'S ROWS WITH THE LATE ONES SET APART (PRICE.lateMs): the least answer any HELD state of the run needed, and each row with a held state late against
 *  it said in its `apart` and left without a verdict: as before the running read was, to the digit. THE RUNNING STATES ARE READ AGAINST THE LEAST OF THEIR
 *  OWN KIND (`leastRun`): they are paced to sixty and hold the game's step, so they answer on another beat than a held state, and one least for both would
 *  make every held state of a fast display look late. A late running state puts the running read apart and leaves the held verdict standing. Asked once,
 *  of the whole run, when its file is made. `flight` = the frames the run's pages kept with the GPU (the page's own word, a flag a run carries); `lateMs`
 *  0 = not asked (the leasts null, the rows as they are) */
export function lateRows(rows: PriceRow[], lateMs = PRICE.lateMs, flight = 2): { rows: PriceRow[]; least: number | null; leastRun: number | null } {
  type State = [what: string, w: PriceWin, waits: boolean];
  const held = (r: PriceRow): State[] => [['the layers off', r.off, spansWait(r.off)], ['the layers on', r.on, spansWait(r.on)], ...(r.page ? [['the page without the look', r.page.win, spansWait(r.page.win)] as State] : [])];
  const live = (r: PriceRow): State[] => r.run ? [['the look page with the world running', r.run.win, spansWait(r.on)], ...(r.page?.run ? [['the page without the look with the world running', r.page.run.win, spansWait(r.page.win)] as State] : [])] : [];   // (a row's running read is its look page's: no row without one has running states)
  const asked = lateMs > 0 && isFinite(lateMs), leastOf = (of: (r: PriceRow) => State[]): number | null => { const a = asked ? rows.flatMap(r => of(r).map(([, w, waits]) => answerOf(w, waits))).filter((v): v is number => v !== null) : []; return a.length ? Math.min(...a) : null; };
  const least = leastOf(held), leastRun = leastOf(live);
  if (least === null && leastRun === null) return { rows, least, leastRun };
  const said = (states: State[], base: number | null) => base === null ? [] : states.flatMap(([what, w, waits]) => { const late = lateBy(w, base, flight, waits);
    return late !== null && late > lateMs ? [`${what}: the GPU's "done" came ${w.work!.toFixed(0)} ms after a frame began for ${stampedOf(w, waits)!.toFixed(1)} ms of passes, ${late.toFixed(0)} ms more than ${Math.max(1, flight) === 1 ? 'a frame' : `${Math.max(1, flight)} frames`} of its own passes and this run's least answer (${base.toFixed(0)} ms) explain (past ${lateMs.toFixed(0)}: something the stamps do not show held the GPU, so this state's times cannot be trusted)`] : []; });
  return { least, leastRun, rows: rows.map(r => { const h = said(held(r), least), l = said(live(r), leastRun);
    return h.length ? { ...r, apart: [...r.apart, ...h], over: [], ...(r.run ? { runApart: [...(r.runApart ?? []), ...l], overRun: [] } : {}) } : l.length ? { ...r, runApart: [...(r.runApart ?? []), ...l], overRun: [] } : r; }) };
}
/** `a` over `b`, less one (null: either is missing) */
const more = (a: number | null, b: number | null): number | null => a !== null && b ? +(a / b - 1).toFixed(4) : null;
/** A ROW WITH A PAGE'S RUNNING READ PUT ON IT, as the run writes it (`look`: the look page's, else the page without the look's, whose `page` the row must
 *  already have). `ran` undefined = that page has none; `why` = that page's own reasons its running read is not to be judged. The look page's read is
 *  the row's; the other page's goes on `page`, and where the look page has a read and the other has none the pair cannot be judged, which is said */
export function rowRun(r: PriceRow, look: boolean, ran: PriceRun | undefined, why: string[]): PriceRow {
  if (look) return ran ? { ...r, run: ran, runApart: [...why, ...queuedWords(ran.win, 'the look page with the world running')] } : r;
  // (the row's running read is the look page's. Where that page took none, the other page's has nothing to be read beside and is not kept: said in the notes)
  if (!r.run) return ran ? { ...r, notes: [...r.notes, 'the page without the look was read with the world running and the look page was not: that read is not kept'] } : r;
  const page = r.page && ran ? { ...r.page, run: ran } : r.page;
  return { ...r, page, runApart: [...(r.runApart ?? []), ...why.map(t => `the page without the look: ${t}`), ...(ran ? queuedWords(ran.win, 'the page without the look with the world running') : ['the page without the look has no running read'])] };
}
/** A ROW JUDGED, as the run judges it once it is whole: a row whose states or pages are not one picture is NOT JUDGED (an "under the bar" read off two
 *  different pictures would be a verdict on nothing), and its running read is not judged then either, nor where that read has a reason of its own */
export const judgeRow = (r: PriceRow, bar: PriceFile['bar']): PriceRow => ({ ...r, over: r.apart.length ? [] : priceOver(r, bar), ...(r.run ? { overRun: r.apart.length || r.runApart?.length ? [] : priceOverRun(r, bar) } : {}) });
/** the second a run's pages run the world to before their running read (0: held pictures only): the asking address's, else the run's own */
const runSOf = (c: Carry): number => c.run ?? PRICE.runS;
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
  catch (e) {
    // A PAGE WHOSE PART THREW (a step of the game, a lost device, a loop that stopped) IS ONE PAGE NOT PRICED, with its cause: the rows before it stand and
    // the run goes on to its next page, as from a page that failed any other way. Only where that cannot be done either is the run given up
    console.error('price every level: this page\'s part threw —', e);
    try { const S = c.steps[i]; await handOn(c, { ...c, i: i + 1, live: -1, notes: [...c.notes, `${S.level}${S.look ? '' : ', the page without the look'}: not priced — its part threw: ${String((e as Error)?.message ?? e).slice(0, 200)}`] }, i, loc, view); }
    catch (e2) { clear(); view?.progress(null); console.error('price every level: the run stopped —', e2); }
  }
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
  // ONE RUNG (the header): the page booted with its preset pinned and the ladder's switch off, by its address's own words. Looked at, not assumed: a page
  // that stands otherwise (quality=auto was asked for; a build whose ladder does not read `adapt=0`) says so on its row, and its ladder is then at least kept
  // from sampling the held picture, as it takes no sample behind a card
  const Qh = w.__quality, held = !!Qh && Qh.auto === false && Qh.adaptive === false;
  if (held && c.quality !== 'auto' && Qh!.current !== undefined && Qh!.current !== c.quality) notes.push(`quality ${c.quality} was asked for and is no preset of this page: it stands on ${Qh!.current}, held, and the row is that preset's`);
  if (!held) notes.push(!Qh || Qh.auto !== false ? 'the quality is not a pinned preset: its ladder was left to settle by itself, so a level\'s two pages may stand on different rungs, and its frames are paced to the ladder\'s own rate' : 'the quality ladder\'s switch was on: it may have cut the scale before the picture was held');
  const E = w.__engine!, size = () => `${E.width}×${E.height}@${E.settings.renderScale.toFixed(3)}${E.settings.sharpDirect ? ' sharp' : ''}`;
  const bag = (): Bag => ({ ms: [], gpu: [], all: [], work: [], passes: new Map(), drew: null }), acc = { off: bag(), on: bag() };
  let ran: PriceRun | undefined, heldMade = -1; const runApart: string[] = [];
  const factsNow = () => ({ quality: w.__quality?.label ?? c.quality, scale: +E.settings.renderScale.toFixed(3), internal: [E.width, E.height] as [number, number], rung: rungOf(w) }); let facts: ReturnType<typeof factsNow> | undefined;
  // THE SCRIPTED STEPS, one a display frame, every one drawn, no more frames with the GPU than the page's own pacer keeps there: to `until` of the game's
  // clock (the steps taken). The play before the held picture and the run-up to the running read are the same steps
  const dev = w.__engine?.device, room = w.__pace?.().flight ?? PRICE.flight, flying: Promise<unknown>[] = [];
  /** the game's fixed step, a second (host/loop.ts FIXEDSTEP.hz): what a scripted step of the play and of the running turn is */
  const STEP_HZ = 60;
  // (… and a wait for the GPU's queue has a limit, as every wait here has: a frame the queue never answers for is gone on without, PRICE.capS)
  const landedOr = (p: Promise<unknown> | undefined): Promise<unknown> => Promise.race([p, new Promise(r => setTimeout(r, PRICE.capS * 1000))]);
  const play = async (until: number): Promise<number> => { const G = w.__game!, most = Math.ceil((until - G.time) * STEP_HZ - 1e-6); let done = 0;
    while (!stopped && G.time < until - 1e-6 && done < most) {
      if (dev && room > 0 && flying.length >= room) await landedOr(flying.shift());
      w.__runFrames!(1, 1 / STEP_HZ, [], true); done++; await frame(); if (dev && room > 0) flying.push(dev.queue.onSubmittedWorkDone().catch(() => { })); }
    return done; };
  if (!failed) {
    say('the mission stands up');
    const G = w.__game, pace = w.__timeScale ?? 1; undo.push(() => { w.__timeScale = pace; });
    if (G && w.__runFrames) {   // (the page's loop has been paused since before its boot: the mission's clock stands at its start on both pages)
      // the page's warm-up, with the clock held (PRICE.playS: what it clears, it clears before anything is in the air). The page says when it has cleared
      // (`__prewarmed`); a build without that word is given `warmFrames` frames of its own
      const worked = () => w.__pace?.().worked, w0 = worked(); let seen = 0;
      const warm = () => w.__prewarmed === undefined ? ((n => n !== undefined && w0 !== undefined ? n - w0 : seen)(worked())) >= PRICE.warmFrames : w.__prewarmed;
      w.__timeScale = 0; w.__prewarm?.(); w.__pauseLoop = loopWas;
      for (const t0 = performance.now(); !stopped && !warm();) { await frame(); seen++;
        if (performance.now() - t0 > PRICE.capS * 1000) { notes.push(`the page's warm-up had not cleared ${PRICE.capS} s after it was asked for: it may have cleared the air after the play`); break; } }
      // … then the mission, a step a frame, every step drawn, no more frames with the GPU than the page's own pacer keeps there
      w.__pauseLoop = true; w.__timeScale = pace;
      const done = await play(PRICE.playS);
      if (!stopped && G.time < PRICE.playS - 1e-6) notes.push(`the game's clock stood at ${G.time.toFixed(2)} s after ${done} steps: held there, not at ${PRICE.playS} s`); }
    else notes.push('this page has no scripted step: the picture was held where the mission stood when the page came ready');
    w.__timeScale = 0; w.__pauseLoop = loopWas; heldAt = G ? +G.time.toFixed(4) : null;   // the world holds still and the page's own loop draws it again: one picture, every state
    for (let t0 = performance.now(); w.__quality?.calibrating && !stopped;) { await frame(); if (performance.now() - t0 > 60000) { notes.push('auto quality was still measuring after a minute: priced where it stood'); break; } }
    if (!held && Qh && typeof Qh.sample === 'function') { Qh.sample = () => { }; undo.push(() => { delete Qh.sample; }); }   // (a ladder left on takes no sample of the held picture: nothing of the size, the scale or what is shed moves between the turns. A held one samples nothing anyway)
    const cut = w.__cut?.programs; if (cut?.swap && cut.ready) await Promise.race([cut.ready, new Promise(r => setTimeout(r, 60000))]);   // (the second set of ray programs compiles after ready where the page has one: not charged to the switch)
    if (S.look) for (let t0 = performance.now(); !w.__perfsweep?.extra?.some(x => x.id.startsWith('look.')) && performance.now() - t0 < 5000;) await frame();   // (the look panel hands its rows to the sweep once the page's recorder is there: a few frames after ready)
    { const behind = await caughtUp(w, () => stopped); if (behind !== null) notes.push(`the GPU's "done" still came ${behind.toFixed(0)} display frames late after ${PRICE.catchS} s of holding the picture`); }
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
    // ---- THE HELD READ IS WHOLE HERE: what the page stood on and what the switch made are its own, taken before the world runs
    heldMade = made.n; facts = factsNow();
    // THE WORLD RUNNING (the header): the same steps on to PRICE.runS, then one turn with the world still stepping, the layers as the page opens with them.
    // Whatever goes wrong in it is said of the running read: the held read above stands
    const runS = runSOf(c);
    if (!failed && !stopped && runS > 0 && G && w.__runFrames && !(runS > G.time + 1e-6)) notes.push(`the world was asked to run to ${runS} s, which is not past the ${G.time.toFixed(2)} s the picture is held at: the row has no running read`);
    else if (!failed && !stopped && runS > 0 && G && w.__runFrames) {
      const stage0 = G.mission?.stage;
      try {
        say(`the world runs to ${runS} s of its clock`);
        w.__pauseLoop = true; w.__timeScale = pace;
        const done = await play(runS); await landedOr(Promise.all(flying.splice(0)));
        if (!stopped && G.time < runS - 1e-6) runApart.push(`the game's clock stood at ${G.time.toFixed(2)} s after ${done} more steps and did not reach ${runS} s: the world did not run on this page as it does on one that is played`);
        say('the world running, its turn');
        // (the page's own pacer, a second one of this turn's: sixty frames a second on the display's own beat, no more frames with the GPU than the page keeps there)
        const from = G.time, P = pacer(), landed = (word: number) => () => P.done(word, performance.now());
        const got = stopped ? null : await turn(w, c.secs, () => stopped, ts => { if (P.skip(ts, STEP_HZ, false, dev ? room : 0)) return false;
          w.__runFrames!(1, 1 / STEP_HZ, [], true); if (dev) { const word = P.sent(performance.now()); dev.queue.onSubmittedWorkDone().then(landed(word), landed(word)); } return true; });
        if (got) { ran = { win: winOf(got), at: +G.time.toFixed(4), made: made.n - heldMade };
          if (!(G.time > from)) runApart.push('the game\'s clock did not move through the running turn: what was timed is a held picture again');
          if (size() !== size0) runApart.push(`the size, the scale or the sharp pass moved before the running turn (${size0} → ${size()})`);
          // (NO FIGHT is what both reads say of themselves. The mission's own word on it, since it began: an alarm episode, the alarm on, or its stage moved)
          { const M = G.mission, n = M?.stats?.alerts ?? 0;
            if (n > 0 || M?.alarmOn) runApart.push(`someone went to alert before the running turn ended (${n} alarm ${n === 1 ? 'episode' : 'episodes'} since the mission began${M?.alarmOn ? ', the alarm on' : ''}): a fight may be in this read, and it is not a mission's quiet start`);
            if (M?.stage !== stage0) runApart.push(`the mission's stage moved from ${stage0 ?? '—'} to ${M?.stage ?? '—'} while the world ran: this read is not its start`); }
          if (ran.made > 0) notes.push(`${ran.made} shader ${ran.made === 1 ? 'program was' : 'programs were'} made while the world ran (a first use: no line of the bar, and not the switch's)`); }
        else if (!stopped) notes.push('the world running: the tab was away through every try, so the row has no running read');
      } catch (e) { ran = undefined; notes.push(`the world running: its part threw, so the row has no running read — ${String((e as Error)?.message ?? e).slice(0, 200)}`); console.error('pricing: the running read threw —', e); }   /* (not the run's own "price every level" word, which a driver reads as the run's end: the run goes on) */
      finally { w.__timeScale = 0; w.__pauseLoop = loopWas; }
    }
  }
  if (stopped) { clear(); view?.progress(null); console.info('price every level: stopped'); return; }
  if (heldMade < 0) heldMade = made.n;   // (a page that failed before its held read was whole: as it stands now)
  const next: Carry = { ...c, i: i + 1, live: -1 }, stood = { ...(facts ?? factsNow()), heldAt };
  const on = winOf(acc.on), bar = barOf(c), judged = (r: PriceRow): PriceRow => judgeRow(r, bar);
  if (failed) next.notes = [...c.notes, `${L.level}${S.look ? '' : ', the page without the look'}: not priced — ${failed}`];
  else if (S.look) { const off = winOf(acc.off);
    if (off.gpu === null) notes.push('the browser stamps no GPU times here: the frame\'s time is all there is, and under the display\'s rate it says nothing');
    next.rows = [...c.rows, judged(rowRun({ level: L.level, mission: L.mission, title: L.title, hour: L.hour, query: loc.search, ...stood, out: [E.outWidth, E.outHeight], off, on, gpuMore: more(on.gpu, off.gpu), allMore: added(on, off), msMore: more(on.ms, off.ms) ?? 0, gpuOverPage: null, allOverPage: null, msOverPage: null,
      offBy, programs: { boot: bootMade, after: afterMade - bootMade, bySwitch: heldMade - afterMade }, boot: w.__boot?.line ?? '',
      apart: [...apart, ...queuedWords(off, 'the layers off'), ...queuedWords(on, 'the layers on')], notes: [...notes, ...(spansWait(on) || spansWait(off) ? [`the g-buffer's stamp spans the wait for the display on this machine (with it ${on.all} ms, the compute passes alone ${on.gpu} ms, a frame every ${on.ms} ms): what the look's surface layers cost cannot be read here, and nothing is judged by "with the g-buffer"`] : [])], over: [] }, true, ran, runApart))]; }
  else { const at = c.rows.findIndex(r => r.level === L.level);   // (the page without the look: its turns go on the level's own row, beside the look page's)
    if (at < 0) next.notes = [...c.notes, `${L.level}: the page without the look was timed (GPU ${on.gpu ?? '—'} ms, with the g-buffer ${on.all ?? '—'} ms, a frame every ${on.ms} ms) but its look page was not`];
    else { const R = c.rows[at]; next.rows = c.rows.map((r, k) => k !== at ? r : judged(rowRun({ ...R, page: { win: on, ...stood, programs: { boot: bootMade, after: heldMade - bootMade }, boot: w.__boot?.line ?? '' }, gpuOverPage: more(R.on.gpu, on.gpu), allOverPage: added(R.on, on), msOverPage: more(R.on.ms, on.ms),
        notes: [...R.notes, ...notes.map(t => `the page without the look: ${t}`)],
        apart: [...R.apart, ...apart.map(t => `the page without the look: ${t}`), ...queuedWords(on, 'the page without the look'),
          ...(stood.internal.join('×') !== R.internal.join('×') || stood.scale !== R.scale ? [`the page without the look drew at ${stood.internal.join('×')} ×${stood.scale}, the look page at ${R.internal.join('×')} ×${R.scale}: not the same size`] : []),
          ...(rungDiffers(R.rung, stood.rung).length ? [`the two pages stood on different rungs of the quality ladder (the look page / the page without it): ${rungDiffers(R.rung, stood.rung).join(', ')}`] : []),
          ...(heldAt !== R.heldAt ? [`the page without the look was held at ${heldAt ?? '—'} s of the game's clock, the look page at ${R.heldAt ?? '—'} s: not the same step`] : [])] }, false, ran, runApart))); } }
  await handOn(c, next, i, loc, view, bar);
}
/** THE BUILD THE PAGE IS, by the one small file a dev server carries (tools/serve.ts /qa/commit: the page's recorders ask the same file when they start);
 *  none off a static build, and none after a second and a half */
const buildWord = (): Promise<string | undefined> => Promise.race([
  fetch('/qa/commit', { cache: 'no-store' }).then(r => r.ok ? r.text() : '').then(t => /^[0-9a-f]{7,40}$/.test(t.trim()) ? t.trim() : undefined, () => undefined),
  new Promise<undefined>(r => setTimeout(() => r(undefined), 1500))]);
/** the bar a run's file carries: the knobs as its pages had them */
const barOf = (c: Carry): PriceFile['bar'] => ({ gpuMore: PRICE.gpuMore, frameMs: PRICE.frameMs, lagFrames: PRICE.lagFrames, ...(PRICE.lateMs > 0 ? { lateMs: PRICE.lateMs } : {}), ...(runSOf(c) > 0 ? { runS: runSOf(c) } : {}), ruled: PRICE.ruled });
/** FROM A PAGE TO THE NEXT, or, from the last, the table and the one file. `next` = the run as this page leaves it */
async function handOn(c: Carry, next: Carry, i: number, loc: Location, view: ReturnType<typeof benchView>, bar0: PriceFile['bar'] = barOf(c)) {
  const bar = { ...bar0 };
  const w = window as unknown as PageHooks, E = w.__engine, of = c.steps.length;
  if (i + 1 < of) { if (!write(next)) throw new Error('the session store took no more'); const N = c.steps[i + 1]; loc.replace(loc.pathname + priceSearch(N.level, i + 1, c.quality, N.look ? c.look : null, c.keep) + loc.hash); return; }
  // ---- the last page: the table, and the one file
  clear(); view?.progress(null);
  const M = machineFacts(E?.gpu?.adapterInfo ?? '', E?.gpu?.adapterFallback, w.__quality?.displayHzPin || w.__quality?.displayHz || 0); await Promise.race([M.more, new Promise(r => setTimeout(r, 1500))]);   // (the adapter the page drew on and the display's rate, as the benchmark's file has them)
  const commit = w.__perfrec?.commit ?? await buildWord();
  const late = lateRows(next.rows, bar.lateMs ?? 0, w.__pace?.().flight ?? 2);   // (the whole run is here: a state whose answer came late against the run's own least puts its row apart, PRICE.lateMs)
  if (!late.rows.some(r => r.run)) delete bar.runS;   // (the second the world ran to is said only where a page took a running read: a run asked for one that no page could take has none)
  const file: PriceFile = { version: 6, what: `twopointsix: what the look costs on every level, one held picture a level — the page without the look, the look page with its layers off, and with them on${bar.runS ? `; then each page again with the world running, read after ${bar.runS} s of the game's clock` : ''} (ui/pricerun.ts)`, at: new Date().toISOString(), ...(commit ? { commit } : {}), startedAt: c.startedAt, address: c.from, look: c.look, userAgent: navigator.userAgent, gpu: M.now.webgpu, machine: M.now,
    screen: `${screen.width}×${screen.height}`, dpr: devicePixelRatio || 1, quality: c.quality, secs: c.secs, bar, ...(late.least !== null ? { leastAnswer: late.least } : {}), ...(late.leastRun !== null ? { leastAnswerRun: late.leastRun } : {}), rows: late.rows, notes: next.notes };
  // THE ONE FILE goes out by the page's one door (host/savefile.ts): to whoever listens, and to the browser as a download only where the address asks for the
  // door as it was (`?downloads=1`, a flag the run carries from the address that asked). The card says which, in the door's own words.
  const name = `price-levels-${file.at.replace(/[:.]/g, '-')}.json`, report = priceReport(file), text = JSON.stringify(file, null, 1); let local = false;
  // (a file a listener took is heard of again at its end: the card's line follows, and Close waits for one still on its way, as the benchmark's card does —
  //  for at most BENCH_SEND_MS; a second press leaves at once)
  let ended: () => void = () => { }; const end = new Promise<void>(r => { ended = r; });
  const went: BenchWent = { result: false, take: null, idle: () => Promise.race([end, new Promise(r => setTimeout(r, BENCH_SEND_MS))]) };
  try { const w2 = saveFile('price', name, new Blob([text], { type: 'application/json' }), { meta: { commit: file.commit }, done: arrived => { went.result = arrived ? 'arrived' : false; ended(); try { view?.update?.(went); } catch { /* the card is gone */ } } }); went.result = w2.went; local = w2.local; }
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
 *  stamps hold no display's wait), the frame's time, said with the baseline's beside it, as the machine's where the baseline is
 *  over the bar too. THE ONE PLACE a row is judged: the run writes these words on the row (`over`), and the table of levels prints them */
export function priceOver(r: PriceRow, bar: PriceFile['bar']): string[] {
  return [...barLines({ on: r.on, base: r.page ? r.page.win : r.off, vs: r.page ? 'the page without the look' : 'its layers off', gpu: r.page ? r.gpuOverPage : r.gpuMore, all: r.page ? r.allOverPage : r.allMore }, bar),
    r.programs.bySwitch > 0 ? `switching the layers made ${r.programs.bySwitch} shader programs (the bar is none)` : ''].filter(Boolean);
}
/** THE BAR'S THREE LINES OF A READ, for the held read and the running one alike: `on` against `base` (null: nothing to read it against, so the frame's time
 *  alone), `gpu` and `all` what the look adds by the game's own number and with the g-buffer counted (null: not judged), `lead` the read's own words in
 *  front. The frame's time is said with the baseline's beside it, and the machine is named where the baseline is over the bar too. The GPU's lag is no
 *  line of the budget: past PRICE.lagFrames the row is not judged at all (queuedWords) */
function barLines(x: { on: PriceWin; base: PriceWin | null; vs: string; gpu: number | null; all: number | null; lead?: string }, bar: PriceFile['bar']): string[] {
  const pc = (v: number) => `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`, top = `the bar is +${(100 * bar.gpuMore).toFixed(0)}%`, { on, base, vs, gpu, all, lead } = x;
  return [base && gpu !== null && gpu > bar.gpuMore ? `${lead ? `${lead}, ` : ''}on the GPU the look adds ${pc(gpu)} against ${vs} (${base.gpu} → ${on.gpu} ms; ${top})` : '',
    base && all !== null && all > bar.gpuMore ? `${lead ? `${lead} and ` : 'with '}the g-buffer counted the look adds ${((on.all ?? 0) - (base.all ?? 0)).toFixed(2)} ms, ${pc(all)} of the GPU's time on ${vs} (${base.all} → ${on.all} ms, its GPU ms ${base.gpu}; ${top})` : '',
    on.ms <= bar.frameMs ? '' : `${lead ? `${lead} ` : ''}a frame comes every ${on.ms.toFixed(1)} ms with the look on${base ? `, ${base.ms.toFixed(1)} ms on ${vs}${base.ms > bar.frameMs ? ': neither holds sixty a second at this preset on this machine' : ''}` : ''} (the bar is ${bar.frameMs} ms)`].filter(Boolean);
}
/** … and what puts THE RUNNING READ over the bar (none = under it, or the row has no such read): the same lines, read between the two pages with the
 *  world running; the frame's time alone where the run has no page without the look. Nothing is judged by "with the g-buffer" where a HELD state of
 *  either page says the g-buffer's stamp holds the display's wait: the running turn is paced to sixty and cannot show a faster display's wait itself.
 *  A switch's programs are the held read's line. NOBODY HAS RULED ON A RUNNING READ: the lines are the bar's, read off another picture */
export function priceOverRun(r: PriceRow, bar: PriceFile['bar']): string[] {
  const a = r.run?.win, b = r.page?.run?.win ?? null; if (!a) return [];
  return barLines({ on: a, base: b, vs: 'the page without the look', gpu: b ? more(a.gpu, b.gpu) : null, all: addedRun(r), lead: 'with the world running' }, bar);
}
/** the g-buffer's stamp holds the display's wait somewhere on this row: any state of it says so (spansWait) */
export const waitsOn = (r: PriceRow): boolean => [r.off, r.on, r.page?.win, r.run?.win, r.page?.run?.win].some(w => !!w && spansWait(w));
/** what the look adds with the world running, with the g-buffer counted, as a share of the other page's GPU's span (null: a page has no running read, or
 *  the row has a display's wait in its stamps). ONE NUMBER for the running verdict and the report's column */
export const addedRun = (r: PriceRow): number | null => r.run && r.page?.run && !waitsOn(r) ? added(r.run.win, r.page.run.win) : null;
/** THE TABLE, as text: a line a level, the levels over the bar marked and named under it */
export function priceReport(f: PriceFile): string {
  const pct = (v: number | null | undefined) => (v === null || v === undefined ? '—' : `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`).padStart(7), n = (v: number | null | undefined, d = 2) => (v === null || v === undefined ? '—' : v.toFixed(d)).padStart(7);
  const three = (a: number | null | undefined, b: number | null, c: number | null) => `${n(a)} ${n(b)} ${n(c)}`;
  const runs = f.rows.some(r => !!r.run), sub = (a: number | null | undefined, b: number | null | undefined) => a === null || a === undefined || b === null || b === undefined ? null : a - b;
  // (a column is its title and its cell: both are padded to the wider, so the head and the lines stand in line whatever a column is called)
  const cols: [string, (r: PriceRow) => string, boolean?][] = [
    ['level', r => r.level, true], ['hour', r => r.hour, true], ['quality', r => r.quality, true], ['size', r => `${r.internal.join('×')} ×${r.scale.toFixed(2)}`, true],
    ['GPU ms: no look, layers off, on', r => three(r.page?.win.gpu, r.off.gpu, r.on.gpu)], ['on vs: no look, off', r => `${pct(r.gpuOverPage)} ${pct(r.gpuMore)}`],
    ['with the g-buffer ms: no look, off, on', r => three(r.page?.win.all, r.off.all, r.on.all)], ['look adds: vs no look, off', r => `${pct(r.allOverPage)} ${pct(r.allMore)}`],
    ['surface ms: off, on', r => `${n(r.off.passes?.[SURFACE])} ${n(r.on.passes?.[SURFACE])}`], ['GPU done, frames: no look, on', r => `${n(r.page?.win.lag, 1)} ${n(r.on.lag, 1)}`],
    ['a frame every: no look, on', r => `${n(r.page?.win.ms, 1)} ${n(r.on.ms, 1)}`], ['programs', r => `${r.programs.boot}+${r.programs.after}+${r.programs.bySwitch}${r.apart.length ? '   NOT JUDGED' : r.over.length ? '   OVER' : ''}`, true],
    // (THE WORLD RUNNING, where the run has such a read: the same span, what the running adds to each page, what the look adds between them, and the frame)
    ...(runs ? [['RUNNING, with the g-buffer ms: no look, on', r => `${n(r.page?.run?.win.all)} ${n(r.run?.win.all)}`], ['running adds ms: no look, on', r => waitsOn(r) ? `${n(null)} ${n(null)}` : `${n(sub(r.page?.run?.win.all, r.page?.win.all))} ${n(sub(r.run?.win.all, r.on.all))}`],
      ['look adds, running', r => pct(addedRun(r))], ['a frame every, running: no look, on', r => `${n(r.page?.run?.win.ms, 1)} ${n(r.run?.win.ms, 1)}`],
      ['running', r => !r.run ? 'no read' : r.apart.length || r.runApart?.length ? 'NOT JUDGED' : r.overRun?.length ? 'OVER' : '', true]] as [string, (r: PriceRow) => string, boolean?][] : [])];
  const cells = f.rows.map(r => cols.map(c => c[1](r))), wide = cols.map((c, k) => Math.max(c[0].length, ...cells.map(x => x[k].length)));
  const line = (x: string[]) => x.map((t, k) => cols[k][2] ? t.padEnd(wide[k]) : t.padStart(wide[k])).join(' | ').trimEnd();
  const head = line(cols.map(c => c[0])), lines = cells.map(line);
  const overs = f.rows.filter(r => r.over.length).map(r => `  ${r.level}: ${r.over.join('; ')}`), rowNotes = f.rows.flatMap(r => r.notes.map(t => `  ${r.level}: ${t}`));
  const aparts = f.rows.filter(r => r.apart.length).map(r => `  ${r.level}: ${r.apart.join('; ')}`);
  const oversRun = f.rows.filter(r => r.overRun?.length).map(r => `  ${r.level}: ${r.overRun!.join('; ')}`), apartsRun = f.rows.filter(r => r.runApart?.length).map(r => `  ${r.level}: ${r.runApart!.join('; ')}`);
  // the rungs the pages stood on, in words: one line where they are one, each named where they are not, and said plainly where a ladder was left on
  const rungs = f.rows.flatMap(r => [r.rung, ...(r.page ? [r.page.rung] : [])]), loose = rungs.filter(g => g.adaptive !== false).length;
  const rungWords = [...new Set(rungs.map(g => `${g.preset} ×${g.scale}${g.cap ? `, at most ${g.cap} rows` : ''}, gather at a ${g.gatherThird ? 'third' : 'half'}, volumetrics at a ${g.volQuarter ? 'quarter' : 'half'}, sharp ${g.sharp ? 'on' : 'off'}`))];
  const rungLine = !rungs.length ? '' : loose ? `THE QUALITY LADDER WAS LEFT ON on ${loose} of ${rungs.length} pages (see the notes): the pages stood on ${rungWords.length === 1 ? 'one rung' : `${rungWords.length} rungs`} — ${rungWords.join(' | ')}.`
    : rungWords.length === 1 ? `EVERY PAGE stood on its preset's own rows with the quality ladder off: ${rungWords[0]}.` : `THE PAGES STOOD ON ${rungWords.length} RUNGS though the ladder was off: ${rungWords.join(' | ')}.`;
  const waits = f.rows.some(r => spansWait(r.on) || spansWait(r.off) || (r.page ? spansWait(r.page.win) : false));
  return [`WHAT THE LOOK COSTS, a held picture a level${runs ? ', and the world running' : ''}  ·  ${f.at.slice(0, 16).replace('T', ' ')} UTC${f.commit ? `  ·  build ${f.commit.slice(0, 8)}` : ''}`, `${f.gpu}${f.machine.hz ? `  ·  a ${f.machine.hz} Hz display` : ''}  ·  screen ${f.screen} ×${f.dpr}  ·  asked for quality ${f.quality}, ${f.secs} s a turn  ·  the look page's words: ${f.look.join('&')}`, f.userAgent, '',
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
    `GPU done   = how many drawn frames after a frame began the GPU said it had finished it. About 1.2 when it keeps up, 2 to 3 when the page is waiting for it. Not a cost and not judged; past ${f.bar.lagFrames} the row is not judged at all${f.bar.lateMs !== undefined ? `, nor where the answer came over ${f.bar.lateMs.toFixed(0)} ms later than the frames of its own passes the page keeps with the GPU and the run's least answer (${f.leastAnswer ?? '—'} ms) explain` : ''}.`,
    'a frame every = the time between two frames the page DREW (it waits for the GPU, so a frame the GPU cannot finish in time costs display frames). On a display',
    '             it holds, that is the display\'s own (16.7 at 60 Hz, 8.3 at 120) and says only that it is held. On a 120 Hz display it can read above 8.3 with',
    `             sixty still held: the page draws when the GPU has answered. It is judged against sixty a second (${f.bar.frameMs} ms) on every display.`,
    'programs   = shader programs the look page made before it was ready + in the seconds after + by switching the layers (that last should be 0).',
    ...(runs ? [`RUNNING    = the same page after its held turns: played on by the same scripted steps to ${f.bar.runS ?? '—'} s of the game's clock, then timed with the world still stepping`,
      '             (falling snow and rain, a fire\'s smoke and the solver that moves it, the wind over the floor). A held picture steps none of these. Lights in fog',
      '             are in the held read already. NO FIGHT is in either read (a running read in which someone went to alert is not judged and is named below):',
      '             a fight\'s smoke, grenades and charges are above both. Its verdict reads the bar\'s own lines off this other picture: NOBODY HAS RULED ON IT.',
      ...(f.leastAnswerRun !== undefined ? [`             A running state's answer is read for lateness against the least among the running states (${f.leastAnswerRun} ms), not the held ones' (${f.leastAnswer ?? '—'} ms).`] : [])] : []),
    'NOT JUDGED = the row\'s states or its two pages were not one picture (another size, rung or step of the game\'s clock), or frames were queued ahead of the GPU, or its answer came late for no work of the state\'s own: said under the table.',
    `OVER       = past the bar (${f.bar.ruled}): with the look on, GPU ms within ${(100 * f.bar.gpuMore).toFixed(0)}% of no look and no more than that added with the g-buffer counted, sixty frames a second held (a frame every ${f.bar.frameMs} ms or less), no program made by the switch.`,
    ...(rungLine ? [rungLine] : []),
    ...(overs.length ? ['', 'Over the bar:', ...overs] : f.rows.length ? ['', aparts.length === f.rows.length ? 'No level was judged.' : aparts.length ? 'No level that was judged is over the bar.' : 'No level is over the bar.'] : ['', 'No level was priced.']),
    ...(aparts.length ? ['', 'Not judged, because the row is not one picture, its frames were queued or the GPU\'s answer came late (its numbers stand, no verdict is read off them):', ...aparts] : []),
    ...(oversRun.length ? ['', 'Over the bar with the world running:', ...oversRun] : !runs ? [] : ['', f.rows.some(r => r.run && !r.apart.length && !r.runApart?.length) ? 'No level whose running read was judged is over the bar with the world running.' : 'No running read was judged.']),
    ...(apartsRun.length ? ['', 'The running read is not judged (its numbers stand):', ...apartsRun] : []), ...(rowNotes.length || f.notes.length ? ['', 'Notes:', ...f.notes.map(t => `  ${t}`), ...rowNotes] : [])].join('\n');
}
