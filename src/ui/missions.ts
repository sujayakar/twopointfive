// THE MISSION LIST (the level select; the maintainer, 10-07: 'a nice level select screen for all our different scenarios (both quiet and loud) … think like
// mgsv did'): the rows and the sheet the one starting page is built on (ui/startpage.ts, which lays them out and hands a pick on to the kit step, ui/loadout.ts):
// EVERY run the build has — the plain solo night (the office drive mission) and one row per entry of scene/scenarioList.ts SCENARIOS — in their groups, the
// highlighted mission's sheet beside them: a floor plan, title, place, its line, the team, whether it leans quiet or loud, 'untuned', and the settings' rows.
// Every row is a link to its own URL (scene/scenarioList.ts pickSearch — main.ts boots it through game/scenarioRun.ts startFromQuery), so a pasted URL starts the same run.
// The solo night's URL is `?solo`: the plain night with no start card (main.ts).
//
// ITS OWN LAYER (#missions, over #menu): the boot card can open it while the shaders compile (it needs nothing of the engine), and Menu re-rendering the
// card under it does not touch it. KEYS (no held keys, nothing past 5): W S / ↑ ↓ move · A D / ← → Standard | Hard · H the next hour (Shift+H the one before) · J the next weather (Shift+J) · K the next way in (Shift+K), on a level with more than one ·
// Enter / Space on · Esc back — taken
// by ONE capture listener registered when this module loads, i.e. before main.ts's own Esc handler, so Esc here goes back instead of closing the card.
// Plain DOM, styled in index.html (#missions …). Nothing at module level uses another module's exports (ui/menu.ts imports this beside the game).
import { pickLeaveSearch, onShortList } from '../scene/picks';
import { MISSION_GROUPS, lessonGroup, type MissionGroup, type ScenarioEntry } from '../scene/scenarios';
import { ALL_MISSIONS, SCENARIO_KEYS, pickSearch, quitSearch, entryOfQuery, levelWords, teamWords, teamTag } from '../scene/scenarioList';
import { lookLevelFile } from '../scene/look';   // (a leaf: which file of a level the page loads)
import { DIFFICULTIES, difficultyNow, setDifficulty, difficultyWords, difficultyHref, structureOfLevel, squadsHeld, type Difficulty } from '../game/difficulty';
import { leverRows, leversSig } from './levers';
import { fogOfLink } from '../game/scenarioRun';
import { quietFogHref, quietFogRow, quietFogAside, soloFogRow } from '../game/quietfog';
import { rowHour, rowWeather } from '../game/rowset';
import { WEATHER_KINDS, WEATHER_PRESETS, weatherTitle, weatherWords, weatherOfSearch, weatherSearch, snowLevel, type WeatherKind } from '../game/weatherKinds';
import { HOUR_IDS, HOURS, hourIdOf, hourHref, hourLabel, hourClock, hourWords, type HourId } from '../game/hours';
import { menuTheme } from './theme';
import officeData from '../../levels/office.json' with { type: 'json' };   // (the bundled office — scene/level.ts imports the same module: its plan needs no server)
import { artFrom, drawSheet, type ArtData, type PlanArt } from './planArt';   // (the map sheet's picture: the place as a model)
import { stillOf, drawStill } from './menuStills';   // (a card's picture: the mission's key art, where one was made)

export type Lean = 'quiet' | 'loud' | 'either';
/** one row of the list: a SCENARIOS entry, or the plain solo night (`scenario` null) */
export interface Mission {
  id: string; title: string;
  /** the scenario it starts (scene/scenarioList.ts); null = the plain night, alone */
  scenario: ScenarioEntry | null;
  /** teammates beside the player as the build stages it: 0 = alone (1: a quiet mission's one partner) */
  squad: 0 | 1 | 2 | 4;
  /** the group it is listed under (scene/scenarios.ts MISSION_GROUPS: the list draws a heading per group, in that table's order) */
  group: MissionGroup;
  place: string; blurb: string; lean: Lean; untuned: boolean;
}
/** the seams other screens set: `startSolo` is the start card's Start (set by ui/menu.ts only while the real start card is up) */
export const MISSIONS: { startSolo: ((card?: boolean) => void) | null;
  /** THE KEYS ARE ELSEWHERE ON THE LIST'S OWN PAGE (ui/startpage.ts: the loadout mounted in the sheet has them): while it says so the list takes no key */
  keysAway: (() => boolean) | null } = { startSolo: null, keysAway: null };
const LS = 'twopointsix.mission';   // the row last started on this machine: the list opens on it

/** THE SOLO NIGHT: the office after dark, alone — reach the server room, pull the drive, leave by the fire exit (game/mission.ts). Not a scenario: no flag
 *  but `?solo`, which only keeps the start card shut. The shift is three guards and two civilians (tools/qa missions probe counts them on a plain boot). */
export const SOLO_KEY = 'solo';
export const SOLO_NIGHT: Mission = { id: 'solo-night', title: 'Solo night', scenario: null, squad: 0, group: 'quiet-alone', place: 'The office, after dark', lean: 'quiet', untuned: false,
  blurb: 'You\'re alone on the office floor. Reach the server room, pull the drive and leave through the fire exit. Watch out for lights, noise and cameras.' };

const LEAN_OF_KIND: Record<ScenarioEntry['kind'], Lean> = { nightbreach: 'either', roomclear: 'loud', rescue: 'loud', quiet: 'quiet' };
/** THE GROUP A ROW IS LISTED UNDER: its own (`group`), else by what it is — a quiet mission alone or as two, a rescue on the clock, and any other squad's
 *  mission a raid (every row is under exactly one heading: there is no 'the rest') */
export function groupOf(e: ScenarioEntry): MissionGroup { return e.group ?? (e.kind === 'quiet' ? (e.squad ? 'quiet-two' : 'quiet-alone') : e.kind === 'rescue' ? 'clock' : 'raids'); }
const groupRank = (g: MissionGroup) => { const i = MISSION_GROUPS.findIndex(x => x.id === g); return i < 0 ? MISSION_GROUPS.length : i; };
/** the headings a list of rows is drawn under: one over the first row of each run of one group, from row `from` on (`{ index: title }` — openMissions' `heads`) */
export function groupHeads(list: Mission[], from = 0): Record<number, string> {
  const H: Record<number, string> = {};
  for (let i = from; i < list.length; i++) if (i === from || list[i].group !== list[i - 1].group) H[i] = MISSION_GROUPS.find(x => x.id === list[i].group)?.title ?? 'Missions';
  return H;
}
export function missionOf(e: ScenarioEntry): Mission {
  return { id: e.id, title: e.title, scenario: e, squad: e.squad, group: groupOf(e), place: e.place ?? (e.level ? levelWords(e) : 'The office'), blurb: e.blurb ?? '', lean: e.lean ?? LEAN_OF_KIND[e.kind], untuned: !!e.untuned };
}
/** the whole list IN ITS GROUPS (MISSION_GROUPS' order: training, raids, strongholds, on the clock, quiet alone, quiet as two): inside a group the solo night
 *  first, then the scenarios in their list's order, each group's unplayed rows after its others (unplayedLast) */
export function missionList(list: ScenarioEntry[] = ALL_MISSIONS): Mission[] {
  const flat = [SOLO_NIGHT, ...list.map(missionOf)];
  return unplayedLast(flat.map((m, i) => ({ m, i })).sort((a, b) => groupRank(a.m.group) - groupRank(b.m.group) || a.i - b.i).map(x => x.m));
}
/** THE SWITCH over the rows CUT FROM THE LIST (scene/scenarios.ts `offList`): on, no list draws them. ON IN THE PAGE (`?offlist=0` = every row of the table
 *  drawn, as before the cut), OFF in the module and so in every harness: every list probe there pins the whole table, as it always has (one bar apart: the cap of eight rows a group is asked of what the page draws, tools/qa/listed.ts). Drawing only,
 *  and which lesson a won lesson's end card offers next (ui/verdictNext.ts: the next one the lists draw): a cut row's own address, its `?loadout=` and its id
 *  read back to it either way (missionList keeps it). */
export const OFFLIST_UI = { on: false };   // (the page's entry puts it on — main.ts; nothing at this module's load does, so a probe's stand-in page leaves it off)
/** THE ROWS A LIST DRAWS: missionList without the rows cut from the list — what the Missions screen and the start page show when handed no list of their own.
 *  `keep` = a row the visit itself names (`?loadout=<id>`, a `select`): a cut row asked for by name is drawn that once, so its address still lands on it */
export function listedMissions(list: Mission[] = missionList(), keep?: string): Mission[] { return OFFLIST_UI.on ? list.filter(m => !m.scenario?.offList || m.id === keep) : list; }
/** IS THIS ROW UNPLAYED: it runs and nobody has balanced it (`untuned`), and the short list does not name it (scene/picks.ts). The sheet's tag asks it, and so
 *  can whatever a mission earns. */
export function unplayed(m: { id: string; untuned?: boolean }): boolean { return !!m.untuned && !onShortList(m.id); }
/** a list that stands IN ITS GROUPS, each group's unplayed rows put AFTER its others (stable: both halves keep the order they came in, no row leaves its
 *  group) */
function unplayedLast(list: Mission[]): Mission[] {
  const at = new Map<MissionGroup, number>(); list.forEach((m, i) => { if (!at.has(m.group)) at.set(m.group, i); });   // (a group's place = where its first row stands)
  return list.map((m, i) => ({ m, i, u: unplayed(m) ? 1 : 0 })).sort((a, b) => at.get(a.m.group)! - at.get(b.m.group)! || a.u - b.u || a.i - b.i).map(x => x.m);
}
/** the classes a drawn row wears over `mrow` for it: `unplayed` (drawn quieter) on every unplayed row, and `first` on the one a group's unplayed rows begin
 *  at when a played row stands before it in that group (the break is drawn over it; a group of nothing else has nothing to part from) */
export function unplayedClass(list: Mission[], i: number): string {
  if (!unplayed(list[i])) return '';
  const g = list[i].group, k = list.findIndex(m => m.group === g && unplayed(m));   // (the group's FIRST unplayed row, wherever the list put it: a list that did not come through unplayedLast still gets one break a group)
  return k === i && list.some(m => m.group === g && !unplayed(m)) && i > 0 && list[i - 1].group === g ? ' unplayed first' : ' unplayed';
}
/** does Standard | Hard change anything on this mission? Only a squad scenario's defenders read it (game/difficulty.ts): not the solo night, not a quiet mission */
export const diffLive = (m: Mission): boolean => !!m.scenario && m.scenario.kind !== 'quiet';
/** the query string a pick of `m` loads: the scenario's own (pickSearch) with the difficulty the tables stand at riding along, or `?solo` — the visitor's
 *  other flags kept either way */
export function missionHref(m: Mission, search = '', hour?: HourId): string {
  const href = quietFogHref(m.scenario ? quietFogAside(() => difficultyHref(pickSearch(m.scenario!, search))) : (rest => `?${SOLO_KEY}${rest ? '&' + rest.slice(1) : ''}`)(quitSearch(search)), m.scenario ? quietFogRow(m.scenario) : soloFogRow());   // (a quiet mission's rung of the fog lever — game/quietfog.ts: this machine's pick on a quiet row's link, a rung the visitor arrived with dropped from every other)
  return hour === undefined ? href : hourHref(href, hour);   // (THE HOUR — game/hours.ts: the sheet's row puts its pick on the link; none given = whatever the visitor's own flags carry)
}
/** the row a query string would boot (null: it names nothing on the list — a plain URL boots the start card, not a mission) */
export function missionOfQuery(search: string, list: Mission[] = missionList()): Mission | null {
  const e = entryOfQuery(search, list.flatMap(m => m.scenario ? [m.scenario] : [])); if (e) return list.find(m => m.scenario === e) ?? null;
  const q = new URLSearchParams(search); return q.has(SOLO_KEY) && !SCENARIO_KEYS.some(k => k !== SOLO_KEY && q.has(k)) ? list.find(m => !m.scenario) ?? null : null;
}
export function teamWordsOf(m: { squad: 0 | 1 | 2 | 4 }): string { return m.squad ? teamWords({ squad: m.squad }) : 'you, alone'; }
export function teamTagOf(m: { squad: 0 | 1 | 2 | 4 }): string { return m.squad ? teamTag({ squad: m.squad }) : 'solo'; }
export function leanWords(l: Lean): string { return l === 'quiet' ? 'quiet' : l === 'loud' ? 'loud' : 'quiet or loud'; }

export interface MissionsOpts {
  list?: Mission[];
  /** the visitor's query string (default: the page's): what a pick keeps, and which row the list opens on when it names one */
  search?: string;
  /** the layer to render into (default #missions) */
  root?: HTMLElement;
  /** Enter, Space or a double click on a row: the next step (ui/startpage.ts: the kit, mounted inside this list) */
  onPick: () => void;
  /** the row to open on (default: the URL's, else the one last started here, else `first`, else the list's first) */
  select?: string;
  /** the row a visit that names none and has started none opens on (the start page: the short list's first — its groups put Training on top) */
  first?: string;
  onClose?: () => void;
  /** after every paint (the highlighted row, the difficulty, the hour or the weather may have changed) */
  onPaint?: () => void;
}
/** the weather picked on the list this page last showed (kept while the page lives: the loadout's Back reopens the list; never stored) */
let wxPick: { search: string; kind: WeatherKind | null } | null = null;
/** …and the hour picked on it, kept the same way (a screen coming back hands its own over in opts.hour, which wins) */
let hourPick: { search: string; id: HourId } | null = null;
/** …and THE WAY IN picked per mission (game/roomclear.ts ROOMCLEAR.way, the sheet's Way in row): 0 or absent = the level's own start, k ≥ 2 = its k-th
 *  (`way=k` on the link), −1 = dealt (`way=any`). Only rows whose entry names its `ways` have the choice. */
const wayPick: Record<string, number> = {};
/** the way in on a link: the page's own `way` taken off, then this row's — nothing for the level's own start (a bare link is the level as it was made) */
export function waySearch(href: string, way: number): string {
  const [path, query = ''] = href.split('?'); const keys = query.split('&').filter(k => k && !/^way(=|$)/.test(k));
  if (way < 0) keys.push('way=any'); else if (way > 1) keys.push(`way=${way}`);
  return keys.length ? `${path}?${keys.join('&')}` : path;
}
/** the Way in row's line under its buttons */
function wayNote(m: Mission, way: number): string {
  const W = m.scenario?.ways; if (!W) return 'this mission has one way in';
  return way < 0 ? 'the game picks one for you each time, along with where the defenders start' : way > 1 ? 'this side has a different first room, so plan it again' : 'the level was built around this way in';
}
/** THE TWO ROWS TOGETHER (game/hours.ts, the weather over the hour): what the hour's line and the weather's say once the other row is not at its default */
export function hourNote(h: HourId, wx: WeatherKind | null, own = false): string {   // (own: the weather is the mission's own — no Weather row is drawn below to pick it on)
  return `${hourClock(h)} · ${h === 'night' && wx ? `the level as it was built, with the moon and the lights outside, in ${own ? 'the mission\'s own weather' : 'the weather picked below'}` : hourWords(h)}`;
}
export function weatherNote(wx: WeatherKind | null, h: HourId): string {
  if (h === 'night') return weatherWords(wx);
  if (!wx) return 'no weather picked, so it\'s a dry day (the level\'s own storm stays with its night)';
  const c = WEATHER_PRESETS[wx].cloud, own = HOURS[h].cloud ?? 0;
  const sky = c <= own || c < 0.15 ? '' : c >= 0.99 ? ' · by day there\'s no sun at all, just dim gray light and no shadows' : c >= 0.85 ? ' · by day the sun is behind clouds, with flat gray light and soft shadows' : c >= 0.5 ? ' · by day a pale sun comes through it' : ' · by day the sun is a little dimmer';
  return weatherWords(wx).replace(". At night it's the same as clear", '') + sky;
}
export interface MissionsScreen { root: HTMLElement; list: Mission[]; readonly selected: Mission; readonly hourNow: HourId; select(i: number | string): void; move(d: number): void; difficulty(lv: Difficulty): void; hour(id: HourId | number): void; readonly weather: WeatherKind | null; setWeather(k: WeatherKind | null): void; /** the highlighted row's way in: 0 = the level's own start, k ≥ 2 = its k-th, −1 = dealt */ readonly way: number; setWay(v: number | 'next' | 'prev'): void; pick(): void; close(): void; hrefOf(m: Mission): string }

let open: MissionsScreen | null = null;
export function currentMissions(): MissionsScreen | null { return open; }
/** the list is up: the start card under it is not idle (attract mode, the card's own focus) */
export function missionsBusy(): boolean { return !!open; }
export function closeMissions() { open?.close(); }

export function openMissions(opts: MissionsOpts): MissionsScreen {
  menuTheme();
  open?.close();
  const list = opts.list ?? listedMissions(undefined, opts.select), page = typeof location !== 'undefined' ? location.search ?? '' : '', search = opts.search ?? pickLeaveSearch(page);   // (off a short-list pick's page: without the pick's name and its own switches — scene/picks.ts)
  const root = opts.root ?? document.getElementById('missions')!;
  const had = typeof document !== 'undefined' ? (document as { activeElement?: Element | null }).activeElement as HTMLElement | null | undefined : null;
  const under = opts.root ? null : document.getElementById('menu') as HTMLElement | null;   // the card under the list: inert while it is up (Tab stays on the list's own buttons)
  root.innerHTML = '';
  const card = el('div', 'mcard'); root.appendChild(card);
  // ---- head: back, the title, the difficulty switch
  const head = el('div', 'mhead');
  const back = el('button', 'btn mback') as HTMLButtonElement; back.type = 'button'; back.textContent = '‹ Back'; back.onclick = () => S.close(); head.appendChild(back);
  const ht = el('div', 'mt'); ht.textContent = 'Missions'; head.appendChild(ht);
  head.appendChild(el('div', 'mcount'));
  card.appendChild(head);
  const body = el('div', 'mbody'); card.appendChild(body);
  // ---- the list, THE GROUPS AS TILES, THEIR MISSIONS AS CARDS: the headings stand together in a rail of their own (`.mgroups`), each a tile with a small picture of its
  // first mission's place and how many rows it holds (`data-count`), and only the one over the highlighted row shows its rows (the others' are `.mrow.shut`), in a grid beside it
  // (`.mgrid`). The keys walk every row, crossing into a group opens it, a click on a heading highlights its first row. Every row a link to its own URL (a click highlights it; a double click, Enter or Space goes on)
  const col = el('div', 'mlist tiles'); body.appendChild(col);
  const gcol = el('div', 'mgroups'), ccol = el('div', 'mgrid'); col.append(gcol, ccol);
  const cardPx = 456, cards: HTMLCanvasElement[] = [], arts: [HTMLCanvasElement, Mission][] = [];   // (a card's picture, twice the px it is shown at)
  const rows: HTMLElement[] = [];
  const heads = groupHeads(list);
  const apart = list.map((_, i) => unplayedClass(list, i));   // (THE UNPLAYED ROWS — unplayedClass: ' unplayed', ' unplayed first' or nothing, worn beside whatever the paint gives the row)
  const secs: { h: HTMLElement; at: number }[] = [], secOf: number[] = [];   // (the folding headings, and which one each row is under)
  list.forEach((m, i) => {
    if (heads[i]) { const h = el('div', i ? 'msec rest' : 'msec'); h.setAttribute('data-head', heads[i]); const g = MISSION_GROUPS.find(x => x.title === heads[i]); if (g) h.title = g.line; gcol.appendChild(h);
      const cv = el('canvas', 'mgart') as HTMLCanvasElement; cv.width = 336; cv.height = Math.round(cv.width * PLAN_D / PLAN_W); const nm = el('span', 'mgname'); nm.textContent = heads[i]; h.append(cv, nm); arts.push([cv, m]);
      secs.push({ h, at: i }); h.onclick = () => S.select(i); }   // (a click on a heading = its first row: the group opens round the cursor)
    secOf.push(secs.length - 1);
    const a = el('a', 'mrow') as HTMLAnchorElement; a.setAttribute('data-mission', m.id); a.setAttribute('tabindex', '-1');   // (W S walk the rows; Tab walks the buttons)
    a.setAttribute('data-group', heads[secs[secs.length - 1].at]);
    { const cv = el('canvas', 'mthumb') as HTMLCanvasElement; cv.width = cardPx; cv.height = Math.round(cv.width * 5 / 8); a.appendChild(cv); cards.push(cv); }   // (8 : 5, the shape the page's CSS shows it at)
    const n = el('span', 'mn'); n.textContent = String(i + 1).padStart(2, '0'); a.appendChild(n);
    const t = el('span', 'mtitle'); t.textContent = m.title; a.appendChild(t);
    {   // (a card under 'Quiet, as two' does not say ', as two' again: the tile it stands under does; the whole name is its tooltip)
      const tail = /,\s*([^,]+)$/.exec(heads[secs[secs.length - 1].at] ?? '')?.[1]?.toLowerCase();
      if (tail && m.title.toLowerCase().endsWith(`, ${tail}`)) { t.textContent = m.title.slice(0, -tail.length - 2); t.title = m.title; }
    }
    const team = el('span', `tag team ${teamTagOf(m)}`); team.textContent = teamTagOf(m); a.appendChild(team);
    const ln = el('span', `mlean ${m.lean}`); ln.title = leanWords(m.lean); a.appendChild(ln);
    a.onclick = (e?: Event) => { e?.preventDefault?.(); S.select(i); };
    (a as unknown as { ondblclick: unknown }).ondblclick = (e?: Event) => { e?.preventDefault?.(); S.select(i); S.pick(); };
    rows.push(a); ccol.appendChild(a);
  });
  // ---- the sheet of the highlighted row
  const sheet = el('div', 'msheet'); body.appendChild(sheet);
  const plan = el('canvas', 'mplan') as HTMLCanvasElement; plan.width = 640; plan.height = Math.round(plan.width * PLAN_D / PLAN_W); sheet.appendChild(plan);
  const top = el('div', 'mtop'); const title = el('span', 'mst'); const untuned = el('span', 'tag untuned'); untuned.textContent = 'untuned'; untuned.title = 'it runs, but nobody has balanced it yet'; top.append(title, untuned); sheet.appendChild(top);
  const place = el('div', 'mplace'); sheet.appendChild(place);
  const blurb = el('div', 'mblurb'); sheet.appendChild(blurb);
  const facts = el('div', 'mfacts nodef'); sheet.appendChild(facts);
  const fact = (label: string) => { const c = el('div', 'mfact'); const l = el('i'); l.textContent = label; const v = el('b'); c.append(l, v); c.setAttribute('data-fact', label.toLowerCase()); facts.appendChild(c); return v; };
  const fTeam = fact('Team'), fDef = fact('Defenders'), fLean = fact('Approach'), fLevel = fact('Level');
  fDef.parentElement!.style.display = 'none';   // (how many are inside is a mystery. No screen draws this cell: it stands empty only while the page's written walk, tools/qa/cards-page.ts, counts it)
  // the difficulty switch (game/difficulty.ts): Standard | Hard, kept per machine and put on every scenario link — it changes the squad scenarios' defenders only
  const diff = el('div', 'mdiff'); const dl = el('span', 'dl'); dl.textContent = 'Difficulty'; diff.appendChild(dl);   // (it was 'Defenders': the level sets the fog of war too — every lever under it)
  const dbtns: [Difficulty, HTMLElement][] = [];
  for (const lv of DIFFICULTIES) { const b = el('button', 'dbtn') as HTMLButtonElement; b.type = 'button'; b.textContent = lv; b.setAttribute('data-difficulty', lv); b.onclick = (e?: Event) => { e?.preventDefault?.(); S.difficulty(lv); }; dbtns.push([lv, b]); diff.appendChild(b); }
  const dnote = el('span', 'dn'); diff.appendChild(dnote); sheet.appendChild(diff);
  // THE LEVERS under it (ui/levers.ts, game/difficulty.ts LEVERS): what Standard | Hard is made of, each row its own to move — fog of war (three rungs), the
  // enemy's AI, the enemy's kit. A press on the switch above puts all three back on that level's rungs; a lever moved alone is kept on this machine
  const levers = leverRows(sheet, { row: 'mdiff mlever', label: 'dl', btn: 'dbtn', note: 'dn' }, () => paint());
  // the hour (game/hours.ts): the level at another time of day than the night it was made for — this visit's pick only (nothing is kept: a bare link is the
  // level's own night), put on every link like the difficulty
  const hrow = el('div', 'mdiff mhour'); const hl = el('span', 'dl'); hl.textContent = 'Hour'; hrow.appendChild(hl);
  const hbtns: [HourId, HTMLElement][] = [];
  for (const id of HOUR_IDS) { const b = el('button', 'dbtn') as HTMLButtonElement; b.type = 'button'; b.textContent = hourLabel(id); b.setAttribute('data-hour', id); b.onclick = (e?: Event) => { e?.preventDefault?.(); S.hour(id); }; hbtns.push([id, b]); hrow.appendChild(b); }
  const hnote = el('span', 'dn'); hrow.appendChild(hnote); sheet.appendChild(hrow);
  // the WEATHER the mission is started in (game/weatherKinds.ts): 'As it is' = the level's own, then the kinds to try. On the URL only (?weather=) — nothing is kept
  // per machine, so a plain link is always the level as it has always played. Any mission, the solo night too.
  const wrow = el('div', 'mdiff mwx'); const wl = el('span', 'dl'); wl.textContent = 'Weather'; wrow.appendChild(wl);
  const wbtns: [WeatherKind | null, HTMLElement][] = [];
  for (const k of [null, ...WEATHER_KINDS] as (WeatherKind | null)[]) { const b = el('button', 'dbtn') as HTMLButtonElement; b.type = 'button'; b.textContent = weatherTitle(k); b.setAttribute('data-weather', k ?? 'default'); b.onclick = (e?: Event) => { e?.preventDefault?.(); S.setWeather(k); }; wbtns.push([k, b]); wrow.appendChild(b); }
  const wnote = el('span', 'dn'); wrow.appendChild(wnote); sheet.appendChild(wrow);
  // THE WAY IN (game/roomclear.ts ROOMCLEAR.way): where the team starts, on the levels that give more than one start — the player's pick, on the link (`way=N`),
  // never a surprise; 'Dealt' = this visit's die picks (`way=any`) — offered only with game/samestart.ts off: while every mission starts the same way there is no dealt way. One button per way the longest list on this screen names, the rest hidden per row.
  const yrow = el('div', 'mdiff mway'); const yl = el('span', 'dl'); yl.textContent = 'Way in'; yrow.appendChild(yl);
  const ybtns: [number, HTMLElement][] = [];
  { const most = Math.max(0, ...list.map(m => m.scenario?.ways?.length ?? 0));
    for (const v of [...Array.from({ length: most }, (_, k) => k ? k + 1 : 0), -1]) { const b = el('button', 'dbtn') as HTMLButtonElement; b.type = 'button'; b.textContent = 'Dealt'; b.setAttribute('data-way', v < 0 ? 'any' : String(v || 1)); b.onclick = (e?: Event) => { e?.preventDefault?.(); S.setWay(v); }; ybtns.push([v, b]); yrow.appendChild(b); } }
  const ynote = el('span', 'dn'); yrow.appendChild(ynote); sheet.appendChild(yrow);

  let hourSel: HourId = (hourPick && hourPick.search === search ? hourPick.id : hourIdOf(new URLSearchParams(search).get('hour'))), hourShown: HourId | null = null;
  let wx: WeatherKind | null = wxPick && wxPick.search === search ? wxPick.kind : weatherOfSearch(search), wxShown: WeatherKind | null | undefined;   // (the pick made on this page's list earlier — Back from the loadout reopens the list — else the URL's)
  { const m0 = missionOfQuery(search, list), w = new URLSearchParams(search).get('way');   // (the page's own way in is its mission's pick until another is made)
    if (m0?.scenario?.ways && !(m0.id in wayPick) && w) wayPick[m0.id] = wayOfLink(m0, search); }
  const wayOf = (m: Mission) => { const w = m.scenario?.ways ? wayPick[m.id] ?? 0 : 0; return w < 0 ? 0 : w; }; let wayVer = 0, wayShown = -1;   // (wayOf: a link that came in with way=any = the level's own)
  /** a level BUILT FOR SNOW (game/weatherKinds.ts SNOW_LEVELS) has its own weather: no Weather row is drawn for it, and its link carries none */
  const ownWx = (m: Mission) => snowLevel(m.scenario?.level?.replace(/\.json$/, ''));
  // A ROW WITH ITS OWN HOUR OR WEATHER (game/rowset.ts — a tutorial mission) is always started in them: that row is not drawn for it (what does not apply is
  // not drawn), and the note of the row that IS drawn reads the mission's own for the other. Its link is any row's link: the visitor's picks pass through
  const startHour = (m: Mission) => rowHour(m.scenario) ?? hourSel, startWx = (m: Mission) => ownWx(m) ? null : rowWeather(m.scenario) ?? wx;
  function hrefOf(m: Mission) { return waySearch(weatherSearch(missionHref(m, search, hourSel), ownWx(m) ? null : wx), wayOf(m)); }
  let sel = 0, hrefs: string[] = [], now: Difficulty | 'custom' | null = null, sig = '', planFile: string | null = null;
  const paint = () => {
    const m = list[sel], d = quietFogAside(difficultyNow), lv = leversSig();   // (quietFogAside: the squad's level, not a quiet mission's rung held on the rows)
    if (d !== now || lv !== sig || hourSel !== hourShown || wx !== wxShown || wayVer !== wayShown) { now = d; sig = lv; hourShown = hourSel; wxShown = wx; wayShown = wayVer; hrefs = list.map(hrefOf); rows.forEach((r, i) => r.setAttribute('href', hrefs[i])); }   // (the links change with the difficulty, the hour, the weather and a way in picked alone)
    for (const [id, b] of hbtns) { b.className = hourSel === id ? 'dbtn on' : 'dbtn'; b.setAttribute('aria-pressed', hourSel === id ? 'true' : 'false'); }
    hnote.textContent = hourNote(hourSel, startWx(m), !!rowWeather(m.scenario));
    for (const [k, b] of wbtns) { b.className = wx === k ? 'dbtn on' : 'dbtn'; b.setAttribute('aria-pressed', wx === k ? 'true' : 'false'); }
    wnote.textContent = weatherNote(wx, startHour(m));
    (wrow as HTMLElement & { style: { display: string } }).style.display = ownWx(m) || rowWeather(m.scenario) ? 'none' : '';
    (hrow as HTMLElement & { style: { display: string } }).style.display = rowHour(m.scenario) ? 'none' : '';   // (what does not apply is not drawn)
    { const W = m.scenario?.ways, y = wayOf(m);
      for (const [v, b] of ybtns) { const k = v < 0 ? -1 : v ? v - 1 : 0, shown = !!W && v >= 0 && k < W.length; (b as HTMLElement & { style: { display: string } }).style.display = shown ? '' : 'none'; if (W && v >= 0 && k < W.length) b.textContent = W[k]; b.className = shown && y === v ? 'dbtn on' : 'dbtn'; b.setAttribute('aria-pressed', shown && y === v ? 'true' : 'false'); }
      yrow.className = W ? 'mdiff mway' : 'mdiff mway na'; (yrow as HTMLElement & { style: { display: string } }).style.display = W ? '' : 'none'; ynote.textContent = wayNote(m, y); }   // (a mission with one way in shows no row: the one-page start has no line to spare)
    rows.forEach((r, i) => { r.className = (i === sel ? 'mrow sel' : secOf[i] !== secOf[sel] ? 'mrow shut' : 'mrow') + apart[i]; r.setAttribute('aria-current', i === sel ? 'true' : 'false'); });
    secs.forEach((x, k) => { const on = k === secOf[sel]; x.h.className = `msec${x.at ? ' rest' : ''}${on ? ' open' : ''}`; x.h.setAttribute('aria-expanded', on ? 'true' : 'false'); x.h.setAttribute('data-count', String((secs[k + 1]?.at ?? list.length) - x.at)); });
    title.textContent = m.title; untuned.style.display = unplayed(m) ? '' : 'none';   // (a mission on the short list does not wear the hand-set label — scene/picks.ts)
    place.textContent = m.place; blurb.textContent = m.blurb;
    fTeam.textContent = teamWordsOf(m); fLean.textContent = leanWords(m.lean); fLevel.textContent = m.scenario?.level ? levelWords(m.scenario) : 'office';
    for (const [lv, b] of dbtns) { b.className = now === lv ? 'dbtn on' : 'dbtn'; b.setAttribute('aria-pressed', now === lv ? 'true' : 'false'); }
    diff.className = diffLive(m) ? 'mdiff' : 'mdiff na';   // (greyed where the pick changes nothing: the solo night, and a quiet mission — its shift is the level's own)
    dnote.textContent = m.scenario?.kind === 'quiet' ? 'a quiet mission uses the level\'s own shift on either setting' : m.scenario ? quietFogAside(() => difficultyWords(now!, squadsHeld(m.scenario))) : 'the night shift is the same on either setting';
    dnote.title = dnote.textContent ?? '';   // (the sheet shows two lines of it: index.html .mdiff > .dn)
    { const fogOn = !!m.scenario && fogOfLink(new URLSearchParams(hrefs[sel].replace(/^[^?]*\?/, '')));   // (the fog of war is up on a room clear and a rescue — game/scenarioRun.ts fogOfQuery: the lever is greyed where the mission has none; a Training lesson is taught on the level as it is — its page reads no lever, game/difficulty.ts initDifficulty)
      const quiet = m.scenario ? quietFogRow(m.scenario) : soloFogRow();   // (a quiet mission may be played under the fog — game/quietfog.ts: the lever is live there with a bottom rung of its own, off, and lit where a quiet mission starts — QUIETFOG.def — until it is moved)
      levers.paint(key => !lessonGroup(m.scenario?.group) && (key === 'fog' || key === 'told' ? fogOn || quiet : diffLive(m)),   /* (how much you're told — game/told.ts — is live wherever the fog's lever is, its off rung included: the lever is for more signs than the fog's mark) */ m.scenario?.kind === 'roomclear' ? structureOfLevel(m.scenario.level) : '', quiet); }
    (rows[sel] as unknown as { scrollIntoView?: (o: unknown) => void }).scrollIntoView?.({ block: 'nearest' });
    const file = m.scenario?.level ?? OFFICE, mark: PlanMark = { kind: m.scenario ? m.scenario.kind : 'solo', way: wayOf(m) }, pk = `${file}|${mark.kind}|${mark.way}`;
    if (pk !== planFile) { planFile = pk; drawPlan(plan, file, false, mark, true); }
    rows.forEach((_, i) => {   // the open group's cards: each its place, the highlighted one's with the way in on it
      if (secOf[i] !== secOf[sel]) return;
      const cv = cards[i], f = list[i].scenario?.level ?? OFFICE, mk = i === sel ? mark : undefined, key = mk ? `${f}|${mk.kind}|${mk.way}` : f;
      if (cv.getAttribute('data-card') !== key) { cv.setAttribute('data-card', key); drawCard(cv, list[i], key, mk); } });   // (the map sheet marks where the team starts: the way in is SHOWN, and moves with its row)
    opts.onPaint?.();
  };
  const S: MissionsScreen = {
    root, list, hrefOf,
    get selected() { return list[sel]; },
    select(i) { const k = typeof i === 'string' ? list.findIndex(m => m.id === i) : i; if (k >= 0 && k < list.length) { sel = k; paint(); } },
    move(d) { sel = (sel + d + list.length) % list.length; paint(); },
    difficulty(lv) { quietFogAside(() => setDifficulty(lv)); paint(); },   // (quietFogAside: the level's rows are written under a quiet mission's rung, not over it)
    get hourNow() { return hourSel; },
    hour(id) { hourSel = typeof id === 'number' ? HOUR_IDS[(HOUR_IDS.indexOf(hourSel) + id + HOUR_IDS.length) % HOUR_IDS.length] : id; hourPick = { search, id: hourSel }; paint(); },
    get weather() { return wx; },
    setWeather(k) { wx = k; wxPick = { search, kind: k }; paint(); },
    get way() { return wayOf(list[sel]); },
    setWay(v) { const m = list[sel], W = m.scenario?.ways; if (!W) return; const all = [0, ...W.slice(1).map((_, k) => k + 2)]; wayPick[m.id] = v === 'next' || v === 'prev' ? all[(all.indexOf(wayOf(m)) + (v === 'next' ? 1 : all.length - 1)) % all.length] : all.includes(v) ? v : 0; wayVer++; paint(); },
    pick() {
      try { globalThis.localStorage?.setItem(LS, list[sel].id); } catch { /* no store */ }
      opts.onPick();
    },
    close() { if (open !== S) return; open = null; root.classList.remove('on'); root.innerHTML = ''; under?.removeAttribute?.('inert'); try { had?.focus?.(); } catch { /* gone */ } opts.onClose?.(); },
  };
  { let kept: string | null = null; try { kept = globalThis.localStorage?.getItem(LS) ?? null; } catch { /* no store */ }
    const want = opts.select ?? missionOfQuery(search, list)?.id ?? kept; const k = want ? list.findIndex(m => m.id === want) : -1; sel = k >= 0 ? k : Math.max(0, opts.first ? list.findIndex(m => m.id === opts.first) : 0); }
  open = S; root.classList.add('on');   // (shown BEFORE the first paint: its scrollIntoView needs a laid-out list)
  paint();
  under?.setAttribute?.('inert', '');
  if (!(root as unknown as { __missionsBound?: boolean }).__missionsBound) {   // clicks in the list never reach the game canvas (as #menu's)
    (root as unknown as { __missionsBound?: boolean }).__missionsBound = true;
    root.addEventListener('mousedown', e => e.stopPropagation()); root.addEventListener('contextmenu', e => e.preventDefault());
  }
  root.setAttribute('tabindex', '-1'); try { root.focus?.(); } catch { /* a fake element */ }   // (focus leaves the card's button underneath: Enter / Space must not press IT)
  for (const [cv, m] of arts) { cv.setAttribute('data-card', m.id); drawCard(cv, m, m.id); }   // (the six tiles' pictures: each its first mission's)
  return S;
}

/** the list's keys — exported for the probes; on the page it is the capture listener below. True = taken. */
export function missionsKey(e: { code: string; repeat?: boolean; ctrlKey?: boolean; metaKey?: boolean; altKey?: boolean }): boolean {
  const S = open; if (!S || e.ctrlKey || e.metaKey || e.altKey) return false;
  switch (e.code) {
    case 'ArrowUp': case 'KeyW': S.move(-1); return true;
    case 'ArrowDown': case 'KeyS': S.move(1); return true;
    // (a machine on hand-tuned 'custom' defender rows keeps them under these two keys: only a click on the switch writes over them. A MIX OF THE LEVERS is
    // kept the same way — the keys that sit under a resting hand do not throw away a mix set up row by row)
    case 'ArrowLeft': case 'KeyA': if (!e.repeat && diffLive(S.selected) && quietFogAside(difficultyNow) !== 'custom') S.difficulty('standard'); return true;
    case 'ArrowRight': case 'KeyD': if (!e.repeat && diffLive(S.selected) && quietFogAside(difficultyNow) !== 'custom') S.difficulty('hard'); return true;
    case 'KeyH': if (!e.repeat && !rowHour(S.selected.scenario)) S.hour((e as { shiftKey?: boolean }).shiftKey ? -1 : 1); return true;
    case 'KeyJ': if (!e.repeat && !rowWeather(S.selected.scenario) && !snowLevel(S.selected.scenario?.level?.replace(/\.json$/, ''))) { const all = [null, ...WEATHER_KINDS] as (WeatherKind | null)[]; S.setWeather(all[(all.indexOf(S.weather) + ((e as { shiftKey?: boolean }).shiftKey ? all.length - 1 : 1)) % all.length]); } return true;   // (the key beside H: the next weather, Shift+J the one before)
    case 'KeyK': if (!e.repeat) S.setWay((e as { shiftKey?: boolean }).shiftKey ? 'prev' : 'next'); return true;   // (the key beside J: the next way in, on a level that gives a choice)
    case 'Enter': case 'NumpadEnter': case 'Space': if (!e.repeat) S.pick(); return true;
    case 'Escape': if (!e.repeat) S.close(); return true;
  }
  return false;
}
if (typeof window !== 'undefined' && typeof window.addEventListener === 'function') {
  // registered at module load = ahead of every listener main.ts adds at boot (its Esc handler, the Input class): a key the list takes stops here. Enter / Space
  // on one of the list's own BUTTONS (Tab put the focus there) is that button's, not a pick.
  const onButton = (e: KeyboardEvent) => (e.code === 'Space' || e.code === 'Enter' || e.code === 'NumpadEnter') && !!open && e.target instanceof HTMLElement && e.target.tagName === 'BUTTON' && open.root.contains(e.target);
  // Tab stays on the list while it is up: Back → Standard → Hard → the five hours → the seven weathers → the ways in (where the mission has them) → round and round (the card underneath is inert, and a Tab that reached
  // the page's body would be the game's key and go nowhere)
  const tab = (e: KeyboardEvent) => {
    const f = [...open!.root.querySelectorAll<HTMLElement>('button')].filter(b => b.offsetParent !== null && !(b as HTMLButtonElement).disabled);   // (only what the focus can land on: a hidden or disabled button would hold the walk where it stands)
    const i = f.indexOf(document.activeElement as HTMLElement);
    f[i < 0 ? (e.shiftKey ? f.length - 1 : 0) : (i + (e.shiftKey ? f.length - 1 : 1)) % f.length]?.focus(); };
  window.addEventListener('keydown', e => {
    if (!open || MISSIONS.keysAway?.()) return;
    if (e.code === 'Tab' && !e.ctrlKey && !e.metaKey && !e.altKey) { tab(e); e.preventDefault(); e.stopImmediatePropagation(); return; }
    if (!onButton(e) && missionsKey(e)) { e.preventDefault(); e.stopImmediatePropagation(); }
  }, { capture: true });
  window.addEventListener('keyup', e => { if (open && !MISSIONS.keysAway?.() && !onButton(e) && (e.code === 'Space' || e.code === 'Enter' || e.code === 'NumpadEnter')) e.preventDefault(); }, { capture: true });   // (a link still focused under the list is not followed by the key's release)
}

/** 'Quit to scenarios' across a page load (a scenario on a level file of its own quits by loading the bare URL — main.ts): ask BEFORE the load, and the start
 *  card that boots opens the page. One shot, this tab only. */
const REOPEN = 'twopointsix.missions.reopen';
export function reopenMissionsAfterLoad() { try { globalThis.sessionStorage?.setItem(REOPEN, '1'); } catch { /* no store */ } }
/** the same question asked by the boot card (ui/menu.ts) and the boot (main.ts), who put the page up: read once, nothing opened here */
export function missionsReopenAsked(): boolean { let asked = false; try { asked = globalThis.sessionStorage?.getItem(REOPEN) === '1'; globalThis.sessionStorage?.removeItem(REOPEN); } catch { /* no store */ } return asked; }

// ---- THE MAP SHEET: the level file's own boxes as a MODEL of the place — tilted, the walls cut, lit like a thing on a table (ui/planArt.ts) —, fitted to the canvas by the
// walls' own extent. The office is the bundle's; a level file comes off the dev server's /levels/ like the level itself does, once per file —
// a file that does not come = no plan this time (the sheet closes up; asked again on the next sight).
interface Plan { min: [number, number]; max: [number, number];
  /** the same file as the map sheet's model (ui/planArt.ts): its ground, what stands on it back to front, its lamps */ art: PlanArt;
  /** where the team starts, [x, z], by the mission's kind (a room clear: one per way in, the level's own first; 'solo' = the level's own spawn) */ starts: Record<string, number[][]>;
  /** wall charges the level's own room clear ISSUES the squad (the file's scenarios.roomClear.wallCharges — game/issue.ts raises the bag to it at staging); 0 = none */ issued: number;
  /** the kit the level's file ISSUES the squad itself (scenarios.issue.members — game/kit.ts kitIssue: every staging wears the issue cards with these fields written over them, whatever this machine keeps), by place on the roster ('0' = you); null = none */ kit: IssuedKit | null;
  /** per way in (the level's own first), whether the file says where the team stands for the Team & kit picture (scenarios.kitView.ways — ui/kitView.ts) */ kv: boolean[];
  /** what the place asks of a kit, counted off the file (the Team & kit step's brief — ui/kitBrief.ts): doors found locked · those of them that are steel · the rescue's hostages */ needs: PlaceNeeds;
  /** the rescue's side door (scenarios.rescue.side.door) is one of the locked ones: its staging unlocks it (game/rescue.ts), so a rescue finds one fewer */ sideLocked: boolean; sideSteel: boolean }
export interface PlaceNeeds { locked: number; steel: number; hostages: number; /** the map marks where the team goes in (a quiet mission's has no mark) */ marked: boolean }
/** a level file's own issue, as the file has it: per place on the roster, the fields that differ from that man's issue card */
export type IssuedKit = Record<string, Record<string, unknown>>;
interface PlanData extends ArtData { scenarios?: { roomClear?: { wallCharges?: unknown; team?: { sam?: number[] }; starts?: { sam?: number[] }[] }; nightBreach?: { team?: { sam?: number[] } }; rescue?: { team?: { sam?: number[] }; hostages?: unknown; side?: { door?: unknown } }; issue?: { members?: unknown }; kitView?: { ways?: { door?: unknown; span?: unknown }[] } }; mission?: { playerSpawn?: number[] } }
const OFFICE = 'office.json';
const PLANS = new Map<string, Promise<Plan | null>>();
const SKIP_GROUP = /^(ground|floors|ceiling|beacons|exterior\.lamps)/;
function planFrom(d: PlanData): Plan {
  const w = d.scenarios?.roomClear?.wallCharges;
  const k = d.scenarios?.issue?.members;
  let art: PlanArt | null = null;   // (made when a picture of the place is first drawn: the sort of what stands is the dear part, and most files are never drawn on a visit)
  const P: Plan = { get art() { return art ??= artFrom(d); }, min: [Infinity, Infinity], max: [-Infinity, -Infinity], issued: typeof w === 'number' && Number.isInteger(w) && w > 0 ? w : 0, kit: k && typeof k === 'object' && !Array.isArray(k) ? k as IssuedKit : null, starts: {}, kv: (Array.isArray(d.scenarios?.kitView?.ways) ? d.scenarios.kitView.ways : []).map(y => (typeof y?.door === 'string' && !!y.door) || Array.isArray(y?.span)),
    needs: { locked: 0, steel: 0, hostages: Array.isArray(d.scenarios?.rescue?.hostages) ? d.scenarios.rescue.hostages.length : 0, marked: false }, sideLocked: false, sideSteel: false };   // (anchored on a door, or on a span: a stretch of wall, a gap)
  { const at = (v?: number[]) => (Array.isArray(v) && v.length >= 3 && Number.isFinite(v[0]) && Number.isFinite(v[2]) ? [[v[0], v[2]]] : []), sc = d.scenarios;   // where the team starts, per kind of mission (a room clear: one per way in, the level's own first)
    P.starts = { roomclear: [...at(sc?.roomClear?.team?.sam), ...(sc?.roomClear?.starts ?? []).flatMap(t => at(t?.sam))], nightbreach: at(sc?.nightBreach?.team?.sam), rescue: at(sc?.rescue?.team?.sam), solo: at(d.mission?.playerSpawn) }; }
  for (const b of d.boxes ?? []) {
    if (SKIP_GROUP.test(b.group ?? '') || b.flags?.includes('NoPrimary')) continue;
    const yaw = b.yaw ?? 0, top = b.c[1] + b.h[1], bottom = b.c[1] - b.h[1];
    if (!b.flags?.includes('Glass') && !(top >= 1.9 && bottom < 1)) continue;
    const cs = Math.abs(Math.cos(yaw)), sn = Math.abs(Math.sin(yaw)), ex = cs * b.h[0] + sn * b.h[2], ez = sn * b.h[0] + cs * b.h[2];   // (the frame = what stands: walls and glazing)
    P.min = [Math.min(P.min[0], b.c[0] - ex), Math.min(P.min[1], b.c[2] - ez)]; P.max = [Math.max(P.max[0], b.c[0] + ex), Math.max(P.max[1], b.c[2] + ez)];
  }
  const side = d.scenarios?.rescue?.side?.door;
  for (const o of d.doors ?? []) {
    const k = o as { id?: unknown; locked?: unknown; steel?: unknown }; if (k.locked !== true) continue;
    P.needs.locked++; if (k.steel === true) P.needs.steel++; if (typeof side === 'string' && k.id === side) { P.sideLocked = true; P.sideSteel = k.steel === true; } }
  if (!(P.max[0] > P.min[0] && P.max[1] > P.min[1])) { P.min = [0, 0]; P.max = [PLAN_W, PLAN_D]; }
  return P;
}
function planOf(file: string): Promise<Plan | null> {
  let p = PLANS.get(file);
  if (!p) {
    p = (async () => {
      try {
        if (file === OFFICE) return planFrom(officeData as unknown as PlanData);   // (the bundled office: its file is in the build, block and all, so nothing is fetched)
        if (typeof fetch !== 'function') return null;
        const r = await fetch(`/levels/${file}`); if (!r.ok) return null;
        return planFrom(await r.json() as PlanData);
      } catch { return null; }
    })();
    PLANS.set(file, p); void p.then(P => { if (!P) PLANS.delete(file); });
  }
  return p;
}
/** the wall charges a room clear of this level file issues the squad whatever its cards carry (0 = none, or the file did not come) — the loadout's 'Squad carries'
 *  line; the same fetch as the plan's, once per file */
export function issuedWalls(file: string): Promise<number> { return planOf(file).then(P => P?.issued ?? 0); }
/** the kit this level file issues the squad itself (null = none, or the file did not come): the loadout screen shows no cards for such a mission and says who
 *  carries what — the same fetch, once per file */
export function issuedKit(file: string): Promise<IssuedKit | null> { return planOf(file).then(P => P?.kit ?? null); }
/** what this mission's place asks of a kit, AS THE MISSION IS STAGED (null: the file did not come) — the same fetch, once per file. The hostages are a rescue's, and
 *  a rescue finds its side door unlocked (game/rescue.ts); the mark is the one the map sheet draws for this kind of mission */
export function placeNeeds(m: Mission): Promise<PlaceNeeds | null> { const kind = m.scenario ? m.scenario.kind : 'solo', rescue = kind === 'rescue';
  return planOf(m.scenario?.level ?? OFFICE).then(P => (P ? { locked: P.needs.locked - (rescue && P.sideLocked ? 1 : 0), steel: P.needs.steel - (rescue && P.sideSteel ? 1 : 0), hostages: rescue ? P.needs.hostages : 0, marked: (P.starts[kind] ?? []).length > 0 } : null)); }
/** which of this level file's ways in have a Team & kit picture staged for them (none: the file has no scenarios.kitView block, or did not come) — the same fetch,
 *  once per file. No file = a mission on the BUNDLED office (its entry names no level): the block of its own file, levels/office.json */
export function kitViewWays(file?: string): Promise<boolean[]> { return planOf(file ?? OFFICE).then(P => P?.kv ?? []); }
const PLAN_W = 40, PLAN_D = 28;   // the canvas's proportions: the levels' footprint today (a plan of another shape is fitted inside it)
/** where the team starts, for the map sheet: the mission's kind and the way in picked (0 / 1 = the level's own, k = its k-th, −1 = dealt: every way, hollow) */
interface PlanMark { kind: string; way: number }
/** the starts a mark shows: the one of the way in picked, or (dealt, and more than one way) every one of them */
function startPts(P: Plan, mark?: PlanMark): { pts: number[][]; dealt: boolean } {
  const S = mark ? P.starts[mark.kind] ?? [] : [], dealt = !!mark && mark.way < 0 && S.length > 1;
  return { dealt, pts: dealt ? S : S.slice(Math.min(S.length - 1, Math.max(0, (mark?.way ?? 0) - 1))).slice(0, 1) };
}
/** A CARD'S PICTURE: the mission's key art, composed ahead of time (ui/menuStills.ts; `data-still` on the canvas says it is one; nothing is marked on it:
 *  the way in is on the map sheet) — and, where no picture came (the one-file build, a mission newer than the pictures), the menu's own drawn model of the
 *  place with the way in marked where `mark` is given, as before. `key` = the canvas's `data-card` at the call: a card asked for something else meanwhile is
 *  left to that. */
function drawCard(cv: HTMLCanvasElement, m: Mission, key: string, mark?: PlanMark): void {
  const file = m.scenario?.level ?? OFFICE, g = cv.getContext?.('2d') ?? null, mine = () => cv.getAttribute('data-card') === key;
  const drawn = () => { cv.removeAttribute?.('data-still'); void drawPlan(cv, file, true, mark); };
  if (!g) return drawn();
  if (cv.getAttribute('data-still') === m.id) return;   // (its picture is up already, and nothing is marked on a picture)
  void stillOf(m.id).then(pic => {
    if (!mine()) return; if (!pic) return drawn();
    cv.setAttribute('data-still', m.id); cv.setAttribute('data-level', file); drawStill(g, cv.width, cv.height, pic);
  });
}
/** the way in a link asks for, as this list keeps it: 0 = the level's own (and every mission with one way), k = its k-th, −1 = dealt */
function wayOfLink(m: Mission, search: string): number {
  const w = new URLSearchParams(search).get('way'), n = m.scenario?.ways?.length ?? 0;
  return !n || !w ? 0 : w === 'any' ? -1 : /^\d+$/.test(w) && +w >= 2 ? Math.min(+w, n) : 0;
}
/** THE MAP SHEET of a mission on a canvas of the caller's, `width` px wide (the sheet's own proportions) — the model of its place AS THE PAGE WILL LOAD IT
 *  (scene/look.ts lookLevelFile: the level's file in the look that is on), with the way in its address asks for marked as this list marks it (dealt = a
 *  ring at each): the load brief's map (ui/loadbrief.ts). Resolves true when it was drawn; the canvas is hidden where the level's file did not come. */
export function drawMissionMap(cv: HTMLCanvasElement, m: Mission, search: string, width: number): Promise<boolean> {
  const w = wayOfLink(m, search), file = lookLevelFile(m.scenario?.level ?? OFFICE);
  cv.width = width; cv.height = Math.round(width * PLAN_D / PLAN_W);
  return drawPlan(cv, file, false, { kind: m.scenario ? m.scenario.kind : 'solo', way: w < 0 ? 0 : w }).then(() => cv.style?.display !== 'none' && !!cv.getContext?.('2d'));
}
function drawPlan(cv: HTMLCanvasElement, file: string, thumb: boolean, mark?: PlanMark, small = thumb): Promise<void> {
  cv.setAttribute('data-level', file); cv.setAttribute('data-sheet', small ? 'small' : 'full'); cv.setAttribute('data-mark', mark ? `${mark.kind}|${mark.way}` : '');
  const g = cv.getContext?.('2d') ?? null; if (!g) { if (!thumb) cv.style.display = 'none'; return Promise.resolve(); }
  const paint = (P: Plan | null) => {
    if (cv.getAttribute('data-level') !== file || cv.getAttribute('data-mark') !== (mark ? `${mark.kind}|${mark.way}` : '')) return;   // (another row was highlighted while this one loaded)
    if (!thumb) cv.style.display = P ? '' : 'none'; if (!P) return;
    // the place as a model — the level's own boxes tilted as the game sees them, walls cut, ground as flat fills, a 10 m scale at the foot (ui/planArt.ts)
    // THE START: the page's accent, its third and last use — a dot where the team stands for the way in picked; dealt = a ring at every way
    const { pts, dealt } = startPts(P, mark);
    g.setTransform(1, 0, 0, 1, 0, 0); g.clearRect(0, 0, cv.width, cv.height);
    drawSheet(g, cv.width, cv.height, P.art, [P.min[0], P.min[1], P.max[0], P.max[1]], Object.values(P.starts).flat(), pts, dealt, small);   // (small: a card's picture, and the tiles' own sheet — a third the size of the old one)
  };
  return planOf(file).then(paint);
}

function el(tag: string, cls?: string) { const e = document.createElement(tag); if (cls) e.className = cls; return e; }
