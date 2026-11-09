// THE MISSION LIST (the level select; the maintainer, 10-07: 'a nice level select screen for all our different scenarios (both quiet and loud) … think like
// mgsv did'): the rows and the sheet the one starting page is built on (ui/startpage.ts, which lays them out and hands a pick on to the kit step, ui/loadout.ts):
// EVERY run the build has — the desk's rows (view/desk.ts `start`) in their groups, the highlighted mission's sheet beside them: a floor plan, title, place, its
// line, the team, whether it leans quiet or loud, 'untuned', and THE SETTINGS' ROWS, each drawn from the desk's `sheet` and pressed by its `set`. Every row is a link
// to its own URL (StartRow.link: what main.ts boots), so a pasted URL starts the same run.
//
// ITS OWN LAYER (#missions, over #menu): the boot card can open it while the shaders compile (it needs nothing of the engine), and Menu re-rendering the
// card under it does not touch it. KEYS (no held keys, nothing past 5): W S / ↑ ↓ move · A D / ← → Standard | Hard · H the next hour (Shift+H the one before) · J the next weather (Shift+J) · K the next way in (Shift+K), on a level with more than one ·
// Enter / Space on · Esc back — taken
// by ONE capture listener registered when this module loads, i.e. before main.ts's own Esc handler, so Esc here goes back instead of closing the card.
// Plain DOM, styled in index.html (#missions …). What is the page's here is the plan sheet, the pictures, the DOM, the keys, and the row last started on this machine.
import { desk } from '../view/desk';
import type { Setting, SheetView, StartRow, StartView } from '../view/read';
import { menuTheme } from './theme';
import officeData from '../../levels/office.json' with { type: 'json' };   // (the bundled office — scene/level.ts imports the same module: its plan needs no server)
import { artFrom, drawSheet, type ArtData, type PlanArt } from './planArt';   // (the map sheet's picture: the place as a model)
import { stillOf, drawStill } from './menuStills';   // (a card's picture: the mission's key art, where one was made)

/** one row of the list, as the desk says it */
export type Mission = StartRow;
/** the seams other screens set: `startSolo` is the start card's Start (set by ui/menu.ts only while the real start card is up) */
export const MISSIONS: { startSolo: ((card?: boolean) => void) | null;
  /** THE KEYS ARE ELSEWHERE ON THE LIST'S OWN PAGE (ui/startpage.ts: the loadout mounted in the sheet has them): while it says so the list takes no key */
  keysAway: (() => boolean) | null } = { startSolo: null, keysAway: null };
const LS = 'twopointsix.mission';   // the row last started on this machine: the list opens on it
/** a mission that draws an Hour row whatever its own: the hour is the list's, and a mission coming up may keep it from a row that does not draw it */
const ANY_HOUR = 'solo-night';

/** the classes a drawn row wears over `mrow` for it: `unplayed` (drawn quieter) on every unplayed row, and `first` on the one a group's unplayed rows begin
 *  at when a played row stands before it in that group (the break is drawn over it; a group of nothing else has nothing to part from) */
function unplayedClass(list: Mission[], i: number): string {
  if (!list[i].unplayed) return '';
  const g = list[i].group, k = list.findIndex(m => m.group === g && m.unplayed);
  return k === i && list.some(m => m.group === g && !m.unplayed) && i > 0 && list[i - 1].group === g ? ' unplayed first' : ' unplayed';
}

export interface MissionsOpts {
  /** the rows, as the desk's `start` said them for this page and the page chose among them */
  list: Mission[];
  /** the mission the page's address names ('' none) */
  named: string;
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
export interface MissionsScreen { root: HTMLElement; list: Mission[]; readonly selected: Mission; /** the highlighted row's rows of settings, as last asked */ readonly sheet: SheetView; readonly hourNow: string; /** the highlighted row's way in: 0 = the level's own start, k ≥ 2 = its k-th */ readonly way: number; select(i: number | string): void; move(d: number): void; /** one choice of one of its rows pressed */ set(key: string, to: string): void; /** the hour put on the list, whichever row is highlighted */ hour(id: string): void; pick(): void; close(): void }

let open: MissionsScreen | null = null;
export function currentMissions(): MissionsScreen | null { return open; }
/** the list is up: the start card under it is not idle (attract mode, the card's own focus) */
export function missionsBusy(): boolean { return !!open; }
export function closeMissions() { open?.close(); }

const settingOf = (s: SheetView, key: string): Setting | undefined => s.settings.find(r => r.key === key);
const litOf = (r: Setting | undefined): string => r?.choices.find(c => c.on)?.id ?? '';
/** the rows' own looks and what each button is known by; every other row is a lever's */
const ROW_CLASS: Record<string, string> = { difficulty: 'mdiff', hour: 'mdiff mhour', weather: 'mdiff mwx', way: 'mdiff mway' };
const BUTTON_ATTR: Record<string, string> = { difficulty: 'data-difficulty', hour: 'data-hour', weather: 'data-weather', way: 'data-way', cams: 'data-cams' };
interface SettingEls { el: HTMLElement; note: HTMLElement; btns: Map<string, HTMLElement>; add(id: string, label: string): HTMLElement }

export function openMissions(opts: MissionsOpts): MissionsScreen {
  menuTheme();
  open?.close();
  const list = opts.list, page = typeof location !== 'undefined' ? location.search ?? '' : '';
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
  const apart = list.map((_, i) => unplayedClass(list, i));   // (THE UNPLAYED ROWS — unplayedClass: ' unplayed', ' unplayed first' or nothing, worn beside whatever the paint gives the row)
  const secs: { h: HTMLElement; at: number }[] = [], secOf: number[] = [];   // (the folding headings, and which one each row is under)
  list.forEach((m, i) => {
    if (i === 0 || m.group !== list[i - 1].group) { const h = el('div', i ? 'msec rest' : 'msec'); h.setAttribute('data-head', m.groupTitle); h.title = m.groupLine; gcol.appendChild(h);
      const cv = el('canvas', 'mgart') as HTMLCanvasElement; cv.width = 336; cv.height = Math.round(cv.width * PLAN_D / PLAN_W); const nm = el('span', 'mgname'); nm.textContent = m.groupTitle; h.append(cv, nm); arts.push([cv, m]);
      secs.push({ h, at: i }); h.onclick = () => S.select(i); }   // (a click on a heading = its first row: the group opens round the cursor)
    secOf.push(secs.length - 1);
    const a = el('a', 'mrow') as HTMLAnchorElement; a.setAttribute('data-mission', m.id); a.setAttribute('tabindex', '-1');   // (W S walk the rows; Tab walks the buttons)
    a.setAttribute('data-group', m.groupTitle);
    { const cv = el('canvas', 'mthumb') as HTMLCanvasElement; cv.width = cardPx; cv.height = Math.round(cv.width * 5 / 8); a.appendChild(cv); cards.push(cv); }   // (8 : 5, the shape the page's CSS shows it at)
    const n = el('span', 'mn'); n.textContent = String(i + 1).padStart(2, '0'); a.appendChild(n);
    const t = el('span', 'mtitle'); t.textContent = m.title; a.appendChild(t);
    {   // (a card under 'Quiet, as two' does not say ', as two' again: the tile it stands under does; the whole name is its tooltip)
      const tail = /,\s*([^,]+)$/.exec(m.groupTitle)?.[1]?.toLowerCase();
      if (tail && m.title.toLowerCase().endsWith(`, ${tail}`)) { t.textContent = m.title.slice(0, -tail.length - 2); t.title = m.title; }
    }
    const team = el('span', `tag team ${m.teamTag}`); team.textContent = m.teamTag; a.appendChild(team);
    const ln = el('span', `mlean ${m.lean}`); ln.title = m.leanWords; a.appendChild(ln);
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
  // THE SETTINGS, every row by the one loop below: its label, a button per choice, the line that says what the lit one does. A row is made the first time the desk names it, and its buttons stay
  // (a button a mission does not offer is put away): the Way in row has one for every way the longest list on this screen names, and Dealt
  const setting = new Map<string, SettingEls>();
  const settingEls = (s: Setting): SettingEls => { const had = setting.get(s.key); if (had) return had;
    const r = el('div', ROW_CLASS[s.key] ?? 'mdiff mlever'); if (!ROW_CLASS[s.key]) r.setAttribute('data-lever', s.key);
    const dl = el('span', 'dl'); dl.textContent = s.label; r.appendChild(dl);
    const note = el('span', 'dn'), btns = new Map<string, HTMLElement>();
    const add = (id: string, label: string) => { const b = el('button', 'dbtn') as HTMLButtonElement; b.type = 'button'; b.textContent = label; b.setAttribute(BUTTON_ATTR[s.key] ?? 'data-rung', id); b.onclick = (e?: Event) => { e?.preventDefault?.(); S.set(s.key, id); }; btns.set(id, b); r.insertBefore(b, note); return b; };
    r.appendChild(note); sheet.appendChild(r);
    const E: SettingEls = { el: r, note, btns, add }; setting.set(s.key, E);
    if (s.key === 'way') { const most = Math.max(0, ...list.map(m => m.ways.length)); for (let k = 1; k <= most; k++) add(String(k), 'Dealt'); add('any', 'Dealt'); }
    return E; };
  const paintSetting = (s: Setting) => { const E = settingEls(s), offered = new Set<string>();
    E.el.className = `${ROW_CLASS[s.key] ?? 'mdiff mlever'}${s.live ? '' : ' na'}${s.moved ? ' moved' : ''}`; (E.el as HTMLElement & { style: { display: string } }).style.display = s.shown ? '' : 'none';
    for (const c of s.choices) { offered.add(c.id); const b = E.btns.get(c.id) ?? E.add(c.id, c.label); if (c.shown) b.textContent = c.label; b.title = c.title; (b as HTMLElement & { style: { display: string } }).style.display = c.shown ? '' : 'none'; const on = c.on && c.shown; b.className = on ? 'dbtn on' : 'dbtn'; b.setAttribute('aria-pressed', on ? 'true' : 'false'); }
    for (const [id, b] of E.btns) if (!offered.has(id)) { (b as HTMLElement & { style: { display: string } }).style.display = 'none'; b.className = 'dbtn'; b.setAttribute('aria-pressed', 'false'); }
    E.note.textContent = s.note; E.note.title = s.note; };

  let sel = 0, shown!: SheetView, planFile: string | null = null;
  /** the rows' links, as the desk says them now: they change with what is pressed */
  const paintLinks = (v: StartView) => { const at = new Map(v.rows.map(r => [r.id, r.link])); list.forEach((m, i) => { m.link = at.get(m.id) ?? m.link; rows[i].setAttribute('href', m.link); }); };
  const wayNow = () => { const lit = litOf(settingOf(shown, 'way')); return /^[2-9]/.test(lit) ? Number(lit) : 0; };
  const paint = () => {
    const m = list[sel]; shown = desk().sheet(m.id);
    for (const s of shown.settings) paintSetting(s);
    rows.forEach((r, i) => { r.className = (i === sel ? 'mrow sel' : secOf[i] !== secOf[sel] ? 'mrow shut' : 'mrow') + apart[i]; r.setAttribute('aria-current', i === sel ? 'true' : 'false'); });
    secs.forEach((x, k) => { const on = k === secOf[sel]; x.h.className = `msec${x.at ? ' rest' : ''}${on ? ' open' : ''}`; x.h.setAttribute('aria-expanded', on ? 'true' : 'false'); x.h.setAttribute('data-count', String((secs[k + 1]?.at ?? list.length) - x.at)); });
    title.textContent = m.title; untuned.style.display = m.unplayed ? '' : 'none';   // (a mission on the short list does not wear the hand-set label)
    place.textContent = m.place; blurb.textContent = m.blurb;
    fTeam.textContent = m.teamWords; fLean.textContent = m.leanWords; fLevel.textContent = m.levelWords;
    (rows[sel] as unknown as { scrollIntoView?: (o: unknown) => void }).scrollIntoView?.({ block: 'nearest' });
    const mark: PlanMark = { kind: m.kind, way: wayNow() }, pk = `${m.level}|${mark.kind}|${mark.way}`;
    if (pk !== planFile) { planFile = pk; drawPlan(plan, m.level, false, mark, true); }
    rows.forEach((_, i) => {   // the open group's cards: each its place, the highlighted one's with the way in on it
      if (secOf[i] !== secOf[sel]) return;
      const cv = cards[i], f = list[i].level, mk = i === sel ? mark : undefined, key = mk ? `${f}|${mk.kind}|${mk.way}` : f;
      if (cv.getAttribute('data-card') !== key) { cv.setAttribute('data-card', key); drawCard(cv, list[i], key, mk); } });   // (the map sheet marks where the team starts: the way in is SHOWN, and moves with its row)
    opts.onPaint?.();
  };
  const press = (id: string, key: string, to: string) => { if (desk().set(id, key, to).ok) paintLinks(desk().start(page)); paint(); };
  const S: MissionsScreen = {
    root, list,
    get selected() { return list[sel]; },
    get sheet() { return shown; },
    select(i) { const k = typeof i === 'string' ? list.findIndex(m => m.id === i) : i; if (k >= 0 && k < list.length) { sel = k; paint(); } },
    move(d) { sel = (sel + d + list.length) % list.length; paint(); },
    set(key, to) { press(list[sel].id, key, to); },
    get hourNow() { return litOf(settingOf(shown, 'hour')); },
    hour(id) { press(ANY_HOUR, 'hour', id); },
    get way() { return wayNow(); },
    pick() {
      try { globalThis.localStorage?.setItem(LS, list[sel].id); } catch { /* no store */ }
      opts.onPick();
    },
    close() { if (open !== S) return; open = null; root.classList.remove('on'); root.innerHTML = ''; under?.removeAttribute?.('inert'); try { had?.focus?.(); } catch { /* gone */ } opts.onClose?.(); },
  };
  { let kept: string | null = null; try { kept = globalThis.localStorage?.getItem(LS) ?? null; } catch { /* no store */ }
    const want = opts.select ?? (list.some(m => m.id === opts.named) ? opts.named : kept); const k = want ? list.findIndex(m => m.id === want) : -1; sel = k >= 0 ? k : Math.max(0, opts.first ? list.findIndex(m => m.id === opts.first) : 0); }
  open = S; root.classList.add('on');   // (shown BEFORE the first paint: its scrollIntoView needs a laid-out list)
  rows.forEach((r, i) => r.setAttribute('href', list[i].link));
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

/** the next choice a row offers, pressed; nothing where the row is not drawn */
function cycle(S: MissionsScreen, key: string, dir: number) {
  const r = settingOf(S.sheet, key); if (!r?.shown) return;
  const c = r.choices.filter(x => x.shown), i = c.findIndex(x => x.on), n = c.length; if (n) S.set(key, c[(i + dir + n) % n].id);
}
/** A D: a level, where a level changes something. (A machine on hand-tuned 'custom' defender rows keeps them under these two keys: only a click on the switch writes over them. A MIX OF THE LEVERS is
 *  kept the same way — the keys that sit under a resting hand do not throw away a mix set up row by row) */
function level(S: MissionsScreen, to: string) { const r = settingOf(S.sheet, 'difficulty'); if (r?.live && litOf(r)) S.set('difficulty', to); }
/** the list's keys — on the page it is the capture listener below. True = taken. */
export function missionsKey(e: { code: string; repeat?: boolean; ctrlKey?: boolean; metaKey?: boolean; altKey?: boolean; shiftKey?: boolean }): boolean {
  const S = open; if (!S || e.ctrlKey || e.metaKey || e.altKey) return false;
  const dir = e.shiftKey ? -1 : 1;
  switch (e.code) {
    case 'ArrowUp': case 'KeyW': S.move(-1); return true;
    case 'ArrowDown': case 'KeyS': S.move(1); return true;
    case 'ArrowLeft': case 'KeyA': if (!e.repeat) level(S, 'standard'); return true;
    case 'ArrowRight': case 'KeyD': if (!e.repeat) level(S, 'hard'); return true;
    case 'KeyH': if (!e.repeat) cycle(S, 'hour', dir); return true;
    case 'KeyJ': if (!e.repeat) cycle(S, 'weather', dir); return true;   // (the key beside H: the next weather, Shift+J the one before)
    case 'KeyK': if (!e.repeat) cycle(S, 'way', dir); return true;   // (the key beside J: the next way in, on a level that gives a choice)
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
  /** wall charges the level's own room clear ISSUES the squad (the file's scenarios.roomClear.wallCharges — rust/game_rules/src/issue raises the bag to it at staging); 0 = none */ issued: number;
  /** the block in which the level's file ISSUES the squad its own kit (scenarios.issue: every staging wears the issue cards with its fields written over them, whatever this machine keeps), as its text; '' = none */ kit: string;
  /** per way in (the level's own first), whether the file says where the team stands for the Team & kit picture (scenarios.kitView.ways — ui/kitView.ts) */ kv: boolean[];
  /** what the place asks of a kit, counted off the file (the Team & kit step's brief — ui/kitBrief.ts): doors found locked · those of them that are steel · the rescue's hostages */ needs: PlaceNeeds;
  /** the rescue's side door (scenarios.rescue.side.door) is one of the locked ones: its staging unlocks it (rust/game_rules/src/rescue), so a rescue finds one fewer */ sideLocked: boolean; sideSteel: boolean }
export interface PlaceNeeds { locked: number; steel: number; hostages: number; /** the map marks where the team goes in (a quiet mission's has no mark) */ marked: boolean }
interface PlanData extends ArtData { scenarios?: { roomClear?: { wallCharges?: unknown; team?: { sam?: number[] }; starts?: { sam?: number[] }[] }; nightBreach?: { team?: { sam?: number[] } }; rescue?: { team?: { sam?: number[] }; hostages?: unknown; side?: { door?: unknown } }; issue?: { members?: unknown }; kitView?: { ways?: { door?: unknown; span?: unknown }[] } }; mission?: { playerSpawn?: number[] } }
const OFFICE = 'office.json';
const PLANS = new Map<string, Promise<Plan | null>>();
const SKIP_GROUP = /^(ground|floors|ceiling|beacons|exterior\.lamps)/;
function planFrom(d: PlanData): Plan {
  const w = d.scenarios?.roomClear?.wallCharges;
  const k = d.scenarios?.issue?.members;
  let art: PlanArt | null = null;   // (made when a picture of the place is first drawn: the sort of what stands is the dear part, and most files are never drawn on a visit)
  const P: Plan = { get art() { return art ??= artFrom(d); }, min: [Infinity, Infinity], max: [-Infinity, -Infinity], issued: typeof w === 'number' && Number.isInteger(w) && w > 0 ? w : 0, kit: k && typeof k === 'object' && !Array.isArray(k) ? JSON.stringify(d.scenarios?.issue) : '', starts: {}, kv: (Array.isArray(d.scenarios?.kitView?.ways) ? d.scenarios.kitView.ways : []).map(y => (typeof y?.door === 'string' && !!y.door) || Array.isArray(y?.span)),
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
/** the kit this level file issues the squad itself, as the text of its block ('' = none, or the file did not come): the desk shows no cards for such a mission and says who
 *  carries what — the same fetch, once per file */
export function issuedKit(file: string): Promise<string> { return planOf(file).then(P => P?.kit ?? ''); }
/** what this mission's place asks of a kit, AS THE MISSION IS STAGED (null: the file did not come) — the same fetch, once per file. The hostages are a rescue's, and
 *  a rescue finds its side door unlocked (rust/game_rules/src/rescue); the mark is the one the map sheet draws for this kind of mission */
export function placeNeeds(m: Mission): Promise<PlaceNeeds | null> { const kind = m.kind, rescue = kind === 'rescue';
  return planOf(m.level).then(P => (P ? { locked: P.needs.locked - (rescue && P.sideLocked ? 1 : 0), steel: P.needs.steel - (rescue && P.sideSteel ? 1 : 0), hostages: rescue ? P.needs.hostages : 0, marked: (P.starts[kind] ?? []).length > 0 } : null)); }
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
  const file = m.level, g = cv.getContext?.('2d') ?? null, mine = () => cv.getAttribute('data-card') === key;
  const drawn = () => { cv.removeAttribute?.('data-still'); void drawPlan(cv, file, true, mark); };
  if (!g) return drawn();
  if (cv.getAttribute('data-still') === m.id) return;   // (its picture is up already, and nothing is marked on a picture)
  void stillOf(m.id).then(pic => {
    if (!mine()) return; if (!pic) return drawn();
    cv.setAttribute('data-still', m.id); cv.setAttribute('data-level', file); drawStill(g, cv.width, cv.height, pic);
  });
}
/** THE MAP SHEET of a mission on a canvas of the caller's, `width` px wide (the sheet's own proportions) — the model of its place AS THE PAGE WILL LOAD IT
 *  (the level's file), with the way in its address asks for marked as this list marks it: the load brief's
 *  map (ui/loadbrief.ts). Resolves true when it was drawn; the canvas is hidden where the level's file did not come. */
export function drawMissionMap(cv: HTMLCanvasElement, level: string, kind: string, way: number, width: number): Promise<boolean> {
  cv.width = width; cv.height = Math.round(width * PLAN_D / PLAN_W);
  return drawPlan(cv, level, false, { kind, way }).then(() => cv.style?.display !== 'none' && !!cv.getContext?.('2d'));
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
