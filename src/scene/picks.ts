// THE SHORT LIST (the maintainer, 10-08: 'keep polishing, optimizing, curating missions'): the few missions worth playing first, IN PLAY ORDER, each with the
// settings that show it at its best — and nothing else. THIS TABLE IS THE ONE PLACE: the start page's first missions (ui/startpage.ts), the 'next' button on the pause and
// debrief cards and `?pick=<id>` typed by hand (host/drawpage.ts) all read PICKS; to reorder, retune or swap a
// mission, edit a row here. Every mission the build has stays on the mission list (ui/missions.ts), with its own team, hour and
// weather rows: a pick is only a shortcut to one row of that list with its settings already on the URL.
//
// A PICK IS A PAGE LOAD, like every start on this build: pickHref(p) is the mission's own query (scene/scenarioList.ts scenarioQuery, or `?solo`) with the
// pick's settings written OUT on it — `squad=N` where it is not the mission's own size, `difficulty=` always on a scenario (so a machine that keeps Hard does
// not turn a Standard pick hard, and the other way round; nothing is stored) — and
// `pick=<id>` last, which names the pick to the page it loads (the pause card's 'next'). The visitor's other flags (?quality=, ?mute …) ride along;
// an older run's settings do not, and neither does a mission's own switch (a visitor's `?pair` is not carried into a pick). What a machine keeps in the Tab panel beyond the difficulty (the team feel rows, a stored pair switch) is the player's and is not pinned. `?pick=<id>` (or its place on the list, `?pick=3`) alone is a link that stays right when a row's settings change: host/drawpage.ts
// answers it with one redirect to pickHref.
//
// THE ONE CHOICE A ROW OFFERS IS STANDARD OR HARD (the maintainer, 10-08: no switches 'other than easy/hard'; the names are the build's own, rust/game_rules/src/difficulty):
// every other setting is this table's. A row opens on its own difficulty (pickDifficulty); the other one is the same
// URL with the other `difficulty=` on it (pickHref's last argument). Both read back as the same pick (pickOfQuery), so 'next' and the quit work on either,
// and 'next' carries a difficulty the player chose over the row's own on to the row after.
//
// Types and pure functions (the guarded read of the page's own query string aside): nothing here imports the game at run time or touches the DOM.
import { scenarioQuery, type ScenarioEntry } from './scenarios';
import { ALL_MISSIONS, SCENARIO_KEYS, quitSearch, specOfQuery } from './scenarioList';
import { squadSearch, type SquadSize } from './squadsize';   // (pure — scenarioList.ts takes the same module)
import type { Difficulty } from './words';

export interface Pick {
  /** its own name on the URL (`pick=<id>`): lower case, digits and dashes; never reused for another row */
  id: string;
  /** the mission it starts: a SCENARIOS id (scene/scenarioList.ts), or SOLO_ID for the plain solo night */
  mission: string;
  /** what the card calls it (absent = the mission's own title) */
  title?: string;
  /** THE ONE PLAIN LINE: what you do in it, in the player's words */
  line: string;
  /** teammates beside the player (absent = the mission's own size; the solo night takes none) */
  squad?: SquadSize;
  /** the defenders the row opens on (absent = standard; the solo night has no switch). The other one is a click away on the same row */
  difficulty?: Difficulty;
  /** RESERVED, NOT READ YET: a kit for this entry, by the name the loadout will give it. Kit is kept per machine today (rust/game_rules/src/loadout) and is on no URL,
   *  so no row sets this and nothing reads it */
  loadout?: string;
}

export const SOLO_ID = 'solo-night';   // (ui/missions.ts SOLO_NIGHT.id / SOLO_KEY)
const SOLO_KEY = 'solo';

// ─────────────────────────────────────────────────────────────── THE LIST, in play order ───────────────────────────────────────────────────────────────
export const PICKS: Pick[] = [
  { id: 'compound', mission: 'compound-four',
    line: 'Start here, at night. The gate beside you is dark, the main gate is lit, and nobody holds the pantry.' },
  { id: 'depot', mission: 'depot-four',
    line: 'A lit yard watched from the dark. The mains are behind the hut that has no door on the yard.' },
  { id: 'solo', mission: SOLO_ID,
    line: 'Alone in the office after dark. Reach the server room, pull the drive, leave by the fire exit.' },
];

// ─────────────────────────────────────────────────────────────── a pick and the query string ───────────────────────────────────────────────────────────────
const PICK_KEY = 'pick';
/** the settings a pick owns on the URL beside the scenario's own keys: whatever the visitor's URL carried for these is replaced, never kept */
const SETTING_KEYS = ['hard', 'difficulty', 'digin', 'pressure', 'hour', 'sun', 'weather', PICK_KEY];   // (digin / pressure: the two older single-switch difficulty flags)
const keyOf = (part: string) => part.split('=')[0];
/** switches that are a mission's own on a URL: a visitor's `?pair` never rides into a pick */
const ROW_SWITCHES = ['pair', 'silentstart'];
/** every key a pick writes or replaces */
export function pickKeys(): string[] { return [...SETTING_KEYS, ...ROW_SWITCHES]; }
const partsOf = (search: string) => search.replace(/^[^?]*\?/, '').split('&').filter(k => k);
/** `search` without the keys in `drop` — every other flag kept as it was typed, in order ('' or '?a&b=c') */
function without(search: string, drop: string[]): string { const keep = partsOf(search).filter(k => !drop.includes(decodeURIComponent(keyOf(k)))); return keep.length ? '?' + keep.join('&') : ''; }

/** the SCENARIOS row a pick starts (null: the solo night; undefined: the list names a mission the build does not have) */
export function pickEntry(p: Pick, scenarios: ScenarioEntry[] = ALL_MISSIONS): ScenarioEntry | null | undefined { return p.mission === SOLO_ID ? null : scenarios.find(e => e.id === p.mission); }
/** the difficulty a row opens on, and the other of the two */
export function pickDifficulty(p: Pick): Difficulty { return p.difficulty ?? 'standard'; }
/** the difficulty a URL spells out, where it is one of the two a row can be started at */
function difficultyOf(search: string): Difficulty | undefined { const d = new URLSearchParams(search).get('difficulty'); return d === 'standard' || d === 'hard' ? d : undefined; }
/** THE QUERY STRING A PICK LOADS (the head of this file), the visitor's other flags behind it — or null where its mission is not on the build.
 *  `difficulty`: the row started at that one instead of its own (the one choice the card offers on a row; nothing on the solo night) */
export function pickHref(p: Pick, search = '', list: Pick[] = PICKS, scenarios: ScenarioEntry[] = ALL_MISSIONS, difficulty?: Difficulty): string | null {
  const e = pickEntry(p, scenarios); if (e === undefined) return null;
  const own = e ? (p.squad !== undefined ? squadSearch(scenarioQuery(e), p.squad, e.squad) : scenarioQuery(e)) : `?${SOLO_KEY}`;
  const parts = partsOf(own);
  if (e) parts.push(`difficulty=${difficulty ?? pickDifficulty(p)}`);
  parts.push(`${PICK_KEY}=${p.id}`);
  const rest = partsOf(without(quitSearch(search), pickKeys()));
  return '?' + [...parts, ...rest].join('&');
}
/** a row by its id, or by its place on the list ('3' = the third) */
export function pickNamed(name: string | null, list: Pick[] = PICKS): Pick | null {
  if (!name) return null;
  return list.find(p => p.id === name) ?? (/^\d+$/.test(name) ? list[Number(name) - 1] ?? null : null);
}
/** `?pick=<id | place>` with NO mission on the URL: the query string to go to instead (host/drawpage.ts, one replace before anything boots) — null for every other URL,
 *  the pick's own included, so the answer to its own answer is always null */
export function pickRedirect(search: string, list: Pick[] = PICKS, scenarios: ScenarioEntry[] = ALL_MISSIONS): string | null {
  const q = new URLSearchParams(search); if (!q.has(PICK_KEY) || specOfQuery(q) || SCENARIO_KEYS.some(k => q.has(k))) return null;
  const p = pickNamed(q.get(PICK_KEY), list); return p ? pickHref(p, search, list, scenarios, difficultyOf(search)) : null;   // (`?pick=1&difficulty=hard` typed by hand: the row at that difficulty)
}
