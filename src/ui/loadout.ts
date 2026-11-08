// THE KIT STEP OF THE ONE STARTING PAGE (ui/startpage.ts mounts it in the mission list's sheet; the maintainer, 10-07: "a "loadout" screen or something like that
// for picking how many squad members you want … think like mgsv did!"): you and a slot per teammate, how many come — none to four — and DEPLOY, which loads the
// mission's own URL with the size on it (game/loadout.ts squadSearch: at the mission's own size it is the URL its row always loaded). The team and its rules are the desk's
// (src/view/desk.ts: `for`, then `team` at each paint that can have changed, and the presses `size` and `named`; read.ts TeamView); this file is the DOM and the keys. THE SOLO NIGHT comes through here too (kind 'solo': the team row is greyed at solo,
// your own card is the only one) and Deploy loads `?solo`, where main.ts puts your card on the plain night's player.
//
// WHAT IS ON IT, top to bottom: TEAM (the size) · the five member slots, each with its man's kit in a line — one of those who come is PICKED ·
// the picked member's KIT AND UNIFORM CARD (src/ui/kitCard.ts memberCard), mounted in the one `[data-member="i"]` host under the
// slots (0 = you, 1‥4 = the teammates in staging order) · what the squad carries between them (stun
// canisters and door charges, the very number the run's staging sets) · Deploy.
//
// CONTROLS (his rules: no held keys, no digit above 5): the mouse for everything (a click on a filled slot picks that man, on an empty one brings the team up
// to him, on a card's arrows and sets as the card says) · W S / ↑ ↓ move between rows: team → members → down through the picked man's card → the buttons ·
// A D / ← → change the highlighted row (the team size; which member; a card row's value) · Enter / Space press (on the members row: into his
// card; in the card: the row's next value, or the reset; elsewhere Deploy) · Esc back to the rows, from anywhere.
// C copies the picked man's kit and V pastes the copied kit onto him, from any row (ui/kitCard.ts; Ctrl / Cmd with either is the same press while
// no text on the page is selected and no text field has the keys): pick a man, C, step along the roster, V on each. A man who wears the copied kit
// carries its mark on his chip. The two go by the letter on the key (kitCard.ts clipKey), the rest of the screen's keys by their place.
// While the page's keys are on it every key is this screen's: the listener below is registered when this module loads, before main.ts's own; the card never
// listens for itself here (it is handed its keys: card.key). The screen's first squad-wide row is the SQUAD KIT: a button for the
// issued kit and one for each of the cost table's named loadouts this build can carry — one press puts it on all five cards, colours kept (by the keys: A D move
// the pick along the row, Enter puts it on); the lit one is what the cards are now (none: a mix of your own).
import type { ScenarioEntry } from '../scene/scenarios';
import { SCENARIO_KEYS } from '../scene/scenarioList';
import { MISSIONS, SOLO_KEY, issuedWalls, issuedKit, type Mission } from './missions';
import { leversHref, fogOffNow } from '../game/difficulty';
import { sameFog } from '../game/scenarioRun';
import { quietFogHref, quietFogRow, quietFogAside, soloFogRow } from '../game/quietfog';
import { hourLive, hourIdOf } from '../game/hours';
import { weatherOfSearch } from '../game/weatherKinds';
import { squadSearch, startSize, storeSquad, type SquadSize, type LoadoutKind } from '../game/loadout';
import { memberCard, clipKey, type MemberCard } from './kitCard';
import { desk } from '../view/desk';
import { Commands } from '../draw/commands';
import type { TeamView } from '../view/read';
import { menuTheme } from './theme';
import { typedInto } from '../host/keymap';
import { gunIcon, kitIcon } from './kitIcons';
/** a roster chip's rows whose drawing is a narrow thing, cut to a square board so a line of them starts flush */
const CHIP_NARROW = new Set(['smoke', 'bangs', 'frags', 'shells']);
const CHIP_GAP = 0.65, CHIP_IN = 0.18, CHIP_DIGIT = 0.46;   // a chip's icon row in icon heights: between two things, between a drawing and its count, one digit (startpage.ts's em values for the same)

/** what the screen preps: a scenario's row (scene/scenarios.ts ScenarioEntry fits as it is), or the solo night (`kind: 'solo'`, squad 0) */
export interface LoadoutEntry { id: string; title: string; kind: LoadoutKind; squad: number; level?: string; group?: string; packs?: { frags?: number } }
export interface LoadoutOpts {
  /** Esc (after the screen has closed itself) */
  back?: () => void;
  /** Deploy, instead of the page load: the query string to load and the size (the co-op dress; the probes) */
  deploy?: (search: string, n: SquadSize) => void;
  /** the mission's own query string, with whatever else should ride along */
  search: string;
  /** the size to open on (absent: what was deployed last on this mission, else its own) */
  squad?: SquadSize;
  /** where the screen is put (ui/startpage.ts: in the mission list's sheet) */
  host: HTMLElement;
  /** the keys are the screen's right now: it takes a key only while this says so */
  keys(): boolean;
  /** a key, before any row of the screen has it: true = it was the host's (the map sheet — ui/kitBrief.ts) */
  key?(code: string, repeat: boolean): boolean;
  /** a key the highlighted row did not take, before the screen's own Enter / Esc / W A S D: true = it was the host's (the walkthrough — ui/kitGuide.ts: an open pop-up's Esc is the card's first) */
  rest?(code: string, repeat: boolean): boolean;
  /** Deploy is about to load `search` with `n` teammates (told before the load or `deploy`; nothing is replaced) */
  onDeploy?: (search: string, n: SquadSize) => void;
  /** told at the end of every paint (the one starting page's picture of the team follows it: ui/kitView.ts) */
  onPaint?: (s: LoadoutScreen) => void;
  /** the words on the block that deploys (absent: 'Deploy' and the team's size). The co-op dress says what its way is (ui/coop.ts) */
  deployWords?: string;
}
/** one keyboard row of the screen: the element that takes the highlight, and what A D and Enter do on it (absent: nothing / Deploy). A row with keys of its
 *  own (the kit card) takes them first — `key`: true = taken, false = the screen's (W / S off its edge walk on) — and is told when the highlight walks onto
 *  it (`enter`: +1 from above, −1 from below). */
export interface LoadoutRow { el: HTMLElement; change?: (dir: 1 | -1) => void; confirm?: () => void; key?: (code: string, repeat: boolean) => boolean; enter?: (dir: 1 | -1) => void }
export interface LoadoutScreen { readonly entry: LoadoutEntry; readonly root: HTMLElement; readonly squad: SquadSize; readonly open: boolean; rows: LoadoutRow[]; sel: number;
  /** the picked member (0 = you, 1‥squad) and his card (null: no cards) */
  readonly member: number; readonly card: MemberCard | null;
  /** the team as the desk last said it */
  readonly team: TeamView;
  /** the kit the picked member will be staged with, as a kept text */
  worn(): string;
  /** the NAMED kit that is on (the Kit row's lit one): '' = a mix of the player's own, or a screen with no such row */
  readonly kitName: string;
  setSquad(n: SquadSize): boolean; /** gone, with no word to anybody (Back's callback not called) */ close(): void; step(dir: 1 | -1): void; move(dir: 1 | -1): void; pick(i: number): void; deploy(): void; back(): void; paint(): void; search(): string; key(code: string, repeat?: boolean): boolean }
let current: LoadoutScreen | null = null;
/** the desk has been told of the fog lever at its bottom rung (moved on the setting rows, which are the page's until the levers are asked of it) */
let fogTold = false;
/** whether the keys are the screen's own right now (LoadoutOpts.keys) */
let currentKeys: (() => boolean) | null = null;
/** the screen that is up (null: none) */
export function loadoutOpen(): LoadoutScreen | null { return current; }

function el(tag: string, cls?: string, text?: string) { const e = document.createElement(tag); if (cls) e.className = cls; if (text !== undefined) e.textContent = text; return e; }

/** Open the kit step for a mission. One at a time: a screen already up is closed first, silently. */
export function openLoadout(entry: LoadoutEntry, opts: LoadoutOpts): LoadoutScreen {
  current?.close();
  injectStyle();
  const base = opts.search;
  const pageSearch = typeof location !== 'undefined' ? location.search ?? '' : '';
  const quietRow = entry.kind === 'solo' ? soloFogRow() : quietFogRow(entry as ScenarioEntry);   // (a quiet mission may be played under the fog: the fog lever is live on its screen with a bottom rung of its own, off; the solo night's link is read as a quiet row's)
  // WHAT THE DESK IS TOLD OF THE MISSION: its id, the link as it would deploy now (the fog lever is on this screen), and once its file has come the kit and the wall charges its level hands out
  const linkNow = () => quietFogHref(base, quietRow);
  let forLink = '', issueText = '', wallsNow = 0;
  const tellFor = () => { if (fogOffNow() !== fogTold) { fogTold = fogOffNow(); const C = new Commands(); C.lever('fog', fogTold ? 'off' : ''); desk().do(C); }
    forLink = linkNow(); desk().for(entry.id, forLink, issueText, wallsNow); };
  tellFor();
  let team: TeamView = desk().team();
  const own = team.sizes.find(c => c.own)?.n ?? 2;
  const asked = opts.squad;   // (a row's click opens on what this machine deployed that mission with last, else its own size: the page's own ?squad=N is another run's)
  let squad = (asked !== null && asked !== undefined && team.sizes[asked].ok ? asked : startSize(entry)) as SquadSize;
  if (!team.sizes[squad].ok) squad = (team.sizes.find(c => c.ok)?.n ?? own) as SquadSize;
  desk().size(squad); team = desk().team();
  const asIssued = team.asIssued;   // (a Training row: played with the kit as issued whatever this machine keeps — no cards, no squad kit row, and a line saying so)
  const readTeam = () => { team = desk().team(); };
  let open = true;

  const root = el('div', 'lo'); root.id = 'loadout'; root.setAttribute('role', 'dialog'); root.setAttribute('aria-label', `Loadout: ${entry.title}`);
  const shell = el('div', 'lo-card'); root.appendChild(shell);
  const card = el('div', 'lo-body'); shell.appendChild(card);   // (the rows)
  // the team size: a button per size
  const teamRow = el('div', 'lo-row lo-team'); teamRow.setAttribute('data-row', 'squad');
  const words = el('div', 'lo-words');
  teamRow.append(words);
  const sizes = el('div', 'lo-sizes'); const sizeBtns: HTMLButtonElement[] = [];
  for (const c of team.sizes) {
    const b = el('button', 'lo-size', c.n === 0 ? 'solo' : `+${c.n}`) as HTMLButtonElement; b.type = 'button'; b.setAttribute('data-squad', String(c.n));
    b.title = c.ok ? `${c.words}${c.own ? ' — this mission\'s own' : ''}` : c.note;
    if (!c.ok) { b.setAttribute('aria-disabled', 'true'); b.classList.add('off'); }
    b.onclick = () => { S.sel = 0; S.paint(); S.setSquad(c.n as SquadSize); };   // (painted first: a refused size leaves its reason on the note line)
    sizeBtns.push(b); sizes.appendChild(b);
  }
  teamRow.appendChild(sizes);
  card.appendChild(teamRow);
  const squadRows = el('div', 'lo-squadrows'); card.appendChild(squadRows);
  // the squad kit: one press puts a whole loadout of the cost table on the cards (as issued first)
  const kitsRow = el('div', 'lo-row lo-kits'); kitsRow.setAttribute('data-row', 'squadkit');   // (the one list of named kits — shown, and live, for a man alone too: a card has no sets of its own but the reset)
  kitsRow.appendChild(el('div', 'lo-label', 'Kit')); kitsRow.classList.add('lo-kits-one');
  const kitBtns: { id: string; b: HTMLButtonElement }[] = []; let kitPick = 0;   // (by the keys: A D move the pick along the row, Enter puts it on — a whole squad's kit is not changed by a stray D)
  const putKit = (id: string) => { const p = desk().named(id); readTeam(); mountCard(true); drawSlots(); S.paint(); if (!p.ok) { note.textContent = p.why; note.className = 'lo-note off'; } };
  for (const L of team.kits) {
    const b = el('button', 'lo-set', L.name) as HTMLButtonElement; b.type = 'button'; b.setAttribute('data-kit', L.id); b.title = L.id === 'issued' ? 'give everyone their issued kit (colors kept)' : `put the ${L.name} loadout on all five cards (colors kept)`;
    b.onclick = () => { S.sel = Math.max(0, S.rows.findIndex(r => r.el === kitsRow)); kitPick = kitBtns.findIndex(k => k.id === L.id); putKit(L.id); };
    kitBtns.push({ id: L.id, b }); kitsRow.appendChild(b);
  }
  const kitsOwn = el('div', 'lo-kits-own'); kitsRow.appendChild(kitsOwn);
  if (kitBtns.length > 1) squadRows.appendChild(kitsRow);
  const noteLine = el('div', 'lo-noteline'), note = el('div', 'lo-note'), pool = el('div', 'lo-pool'); pool.title = 'the stun canisters and door charges that the door tools use up, counted from what your teammates carry (see their cards)';
  const squadP = el('div', 'lo-squadp'); squadP.title = 'the pouches the teammates who come fill between them (the squad\'s budget; blank while the table sets none)';
  noteLine.append(note, squadP, pool); card.appendChild(noteLine);
  // the members: you and four slots; under them the picked member's kit and uniform card
  const slots = el('div', 'lo-slots'); slots.setAttribute('data-row', 'members'); card.appendChild(slots);
  const kitHost = el('div', 'lo-kit'); kitHost.setAttribute('data-row', 'kit'); card.appendChild(kitHost);
  let member = 0, kitCard: MemberCard | null = null;
  // the block that deploys
  const btnRow = el('div', 'lo-row lo-actions'); btnRow.setAttribute('data-row', 'actions');
  const goBtn = el('button', 'lo-btn primary', 'Deploy') as HTMLButtonElement; goBtn.type = 'button'; goBtn.setAttribute('data-act', 'deploy');
  btnRow.append(goBtn); shell.appendChild(btnRow);
  let keyed = false;    // inside a key press (paint scrolls the highlighted row into sight only then)

  const nameOf = (i: number) => ({ title: team.men[i].title, sub: team.men[i].sub });
  const drawSlots = () => {
    slots.innerHTML = '';
    let widest = 1;   // (the fullest man's kit row, in icon heights: every chip draws its things at the ONE size at which that row fits — startpage.ts '.lo-ics')
    team.men.forEach((man, i) => {
      const s = el('div', `lo-slot ${man.here ? 'in' : 'out'} p-${man.here ? man.pair : 'none'}`); s.setAttribute('data-slot', String(i));
      const name = el('div', 'lo-name', man.title); s.appendChild(name);
      s.appendChild(el('div', 'lo-role', man.sub));
      if (man.here && team.cards) {   // his kit in a line
        name.classList.toggle('changed', man.changed); { const g = el('div', 'lo-gun'); g.innerHTML = gunIcon(man.icon, man.can, man.gun); s.appendChild(g); } s.appendChild(el('div', 'lo-sum', man.words));
        { const ics = el('div', 'lo-ics');   // (the same kit as drawings: the one page shows these in place of the words)
          let wide = 0;
          for (const t of man.things) { const svg = kitIcon(t.field, t.value, t.icon, t.can); if (!svg) continue;   // (what differs from man to man: his gun, what he carries a count of, the wand)
            const narrow = CHIP_NARROW.has(t.field), count = t.n > 0, c = el('span', narrow ? 'lo-ic n' : 'lo-ic'); c.innerHTML = narrow ? svg.replace('viewBox="0 0 64 32"', 'viewBox="16 0 32 32"') : svg; c.title = t.label; if (count) c.appendChild(el('b', undefined, t.value));
            wide += (ics.childElementCount ? CHIP_GAP : 0) + (narrow ? 1 : 2) + (count ? CHIP_IN + CHIP_DIGIT * t.value.length : 0); ics.appendChild(c); }
          widest = Math.max(widest, wide); s.appendChild(ics); }
        s.appendChild(el('div', 'lo-pouch', `pouches ${man.pouches.used} / ${man.pouches.total}`));   // (what he carries of the tools and the throwables comes out of his pouches)
        if (man.clip) { s.classList.add('clip'); const c = el('span', 'lo-clip'); c.innerHTML = CLIP_MARK; c.title = 'wears the copied kit'; s.appendChild(c); }   // (the man copied from, and each man it was pasted onto — or who wears the same anyway)
        s.classList.toggle('pick', i === member); s.title = `${man.title}: kit and uniform`;
        s.onclick = () => { S.sel = Math.max(0, S.rows.findIndex(r => r.el === slots)); S.pick(i); };
      } else if (man.here && i > 0) {   // (no cards — a lesson, a mission's own issue: a click on a man who comes sends him, and those after him, home)
        s.title = `fewer: ${team.sizes[i - 1].words}`; s.onclick = () => { S.sel = 0; S.paint(); S.setSquad((i - 1) as SquadSize); };
        if (man.words) s.appendChild(el('div', 'lo-sum', man.words));
      } else if (man.here && man.words) { s.appendChild(el('div', 'lo-sum', man.words));   // (you, on a mission that issues its own kit)
      } else if (!man.here) {
        const c = team.sizes[i]; s.title = c.ok ? `more: ${c.words}` : c.note;   // (a size, not a man: the roster of two is the blue pair, of three or four the red pair first)
        s.onclick = () => { S.sel = 0; S.paint(); S.setSquad(i as SquadSize); };
      }
      slots.appendChild(s);
    });
    slots.style.setProperty?.('--u', (widest * 1.03).toFixed(2));   // (?.: the headless probes' fake DOM has a bare style object)
  };
  /** the picked member's card, made afresh (his name is the size's: 'Blue One' of a pair is 'Red One' of a four); the highlighted row is kept */
  const mountCard = (keep = false) => {   // (`keep`: the same man's card made again — its highlighted row stays; another man's card opens on its first row: the two kinds of card have different rows)
    const at = keep && kitCard ? kitCard.field() ?? 'sets' : 'first'; kitCard?.destroy(); kitCard = null; kitHost.innerHTML = ''; kitHost.setAttribute('data-member', String(member));
    if (!team.cards) return;
    try {
      kitCard = memberCard(kitHost, member, { name: nameOf(member), floor: () => btnRow.getBoundingClientRect?.().top ?? 0,   // (its pop-ups end over this screen's foot)
        onChange: () => { readTeam(); drawSlots(); S.paint(); },
        onCopy: () => { readTeam(); drawSlots(); },   // (the roster's marks move to whoever wears the copied kit; no card changed, so nothing is painted or told on)
        onSelect: () => opts.onPaint?.(S),   // (ui/kitView.ts: the picture plays the highlighted tile's thing — told of a move, which changes no value)
        onFocus: () => { const k = S.rows.findIndex(r => r.el === kitHost); if (k >= 0 && S.sel !== k) { S.sel = k; S.paint(); } } });
      kitCard.go(at);
    } catch (e) { console.error('loadout: memberCard', e); }
  };
  const stepTo = (dir: 1 | -1): SquadSize | null => { for (let n = squad + dir; n >= 0 && n <= 4; n += dir) if (team.sizes[n].ok) return n as SquadSize; return null; };

  const S: LoadoutScreen = {
    entry, root, rows: [], sel: 0,
    get squad() { return squad; }, get open() { return open; }, get member() { return member; }, get card() { return kitCard; }, get team() { return team; },
    get kitName() { return kitBtns.length > 1 ? team.kits.find(k => k.now)?.id ?? '' : ''; },
    worn: () => team.men[member]?.worn ?? '',
    setSquad(n) { const c = team.sizes[n]; if (!c || !c.ok) { note.textContent = c?.note ?? ''; note.className = 'lo-note off'; return false; } const was = squad; squad = n; if (was !== n) { desk().size(n); readTeam(); const same = member <= n; member = Math.min(member, n); mountCard(same); drawSlots(); } S.paint(); return true; },
    step(dir) { const n = stepTo(dir); if (n !== null) S.setSquad(n); },
    move(dir) { const n = S.rows.length, drawn = (k: number) => (S.rows[k].el as unknown as { offsetParent?: unknown }).offsetParent !== null;   // (a row the page it sits in does not draw — the one starting page hides Squad kit for a man alone — is stepped over)
      let k = S.sel; for (let i = 0; i < n; i++) { k = (k + dir + n) % n; if (drawn(k)) break; }
      S.sel = k; S.rows[S.sel].enter?.(dir); S.paint(); },
    pick(i) { const to = Math.min(squad, Math.max(0, Math.round(i))); if (to !== member) { member = to; mountCard(); drawSlots(); } S.paint(); },
    close,
    search: () => squadSearch(quietFogHref(quietFogAside(() => leversHref(base)), quietRow), squad, own),   // (a lever moved by hand: the link carries it only where this machine could not keep it — game/difficulty.ts)
    deploy() {
      if (!open || !team.sizes[squad].ok) return;
      storeSquad(entry.id, squad);
      const to = S.search(); close(); opts.onDeploy?.(to, squad);
      if (opts.deploy) { opts.deploy(to, squad); return; }
      // the solo night off the real start card over the plain night: Start, as it has always been — no load (ui/missions.ts's own shortcut, taken here now that
      // this screen stands between the list and the run); `startSolo(true)` closes the card and then puts your kit on this game's player (ui/menu.ts)
      if (entry.kind === 'solo' && MISSIONS.startSolo && hourIdOf(new URLSearchParams(to).get('hour')) === hourLive() && weatherOfSearch(to) === weatherOfSearch(pageSearch) && !SCENARIO_KEYS.some(k => k !== SOLO_KEY && new URLSearchParams(pageSearch).has(k)) && sameFog(to, pageSearch)) { MISSIONS.startSolo(true); return; }   // (…at the hour and in the weather the page stands at — game/hours.ts, game/weatherKinds.ts: another of either is a load — and in the fog it stands in: the solo night under the fog off a plain page is a load of its own link, game/quietfog.ts QUIETFOG.solo)
      const url = location.pathname + to + location.hash;
      if (url === location.pathname + location.search + location.hash) location.reload(); else location.assign(url);
    },
    back() {
      if (!open) return; close();
      try { const q = new URLSearchParams(location.search); if (q.has('loadout')) { q.delete('loadout'); const s = q.toString().replace(/=(?=&|$)/g, ''); history.replaceState(null, '', location.pathname + (s ? '?' + s : '') + location.hash); } } catch { /* no history here (a probe, a sandboxed frame): the flag stays */ }
      opts.back?.();
    },
    paint() {
      if (linkNow() !== forLink || fogOffNow() !== fogTold) { tellFor(); readTeam(); if (team.cards) drawSlots(); kitCard?.refresh(); }   // (the fog lever moved: the optical wand and the sticky cameras look into the fog of war, so the card and what the mission hands over it are asked again)
      words.textContent = team.sizes[squad].words;
      note.textContent = team.note; note.className = 'lo-note';
      sizeBtns.forEach((b, n) => { b.classList.toggle('on', n === squad); b.classList.toggle('own', team.sizes[n].own); b.setAttribute('aria-pressed', n === squad ? 'true' : 'false'); });
      squadP.textContent = team.squad;
      pool.textContent = team.pool;
      if (team.kits.length && kitBtns.length > 1) { const of = (id: string) => team.kits.find(k => k.id === id), now = team.kits.find(k => k.now)?.id ?? null; if (!of(kitBtns[kitPick].id)?.offered) kitPick = 0;
        kitBtns.forEach((k, n) => { k.b.style.display = of(k.id)?.offered ? '' : 'none'; k.b.classList.toggle('on', k.id === now); k.b.classList.toggle('pick', n === kitPick); k.b.setAttribute('aria-pressed', k.id === now ? 'true' : 'false'); }); kitsOwn.textContent = now ? '' : 'a mix of your own'; }
      S.rows.forEach((r, i) => r.el.classList.toggle('sel', i === S.sel));
      if (keyed) { if (S.sel === 0) (card as unknown as { scrollTop: number }).scrollTop = 0; (S.rows[S.sel]?.el as unknown as { scrollIntoView?: (o: unknown) => void } | undefined)?.scrollIntoView?.({ block: 'nearest' }); }   // (a short window scrolls the rows: the highlighted row is kept in sight, the first row with the mission's head over it — by the KEYS only: a click's focus that scrolled the rows moved the button from under the cursor and lost the click)
      if (kitCard) { if (S.rows[S.sel]?.el === kitHost) kitCard.focus(); else kitCard.blur(); }   // (the card's own highlight shows only while the keys are in it)
      goBtn.classList.toggle('sel', S.rows[S.sel]?.el === btnRow);
      goBtn.textContent = opts.deployWords ?? `Deploy · ${team.sizes[squad].words}`;
      opts.onPaint?.(S);
    },
    key(code, repeat = false) { if (!open) return false; keyed = true; try { return keys(code, repeat); } finally { keyed = false; } },
  };
  const keys = (code: string, repeat: boolean): boolean => {
      if (code && opts.key?.(code, repeat)) return true;
      const row = S.rows[S.sel];
      if (row?.key?.(code, repeat)) return true;
      if (code && opts.rest?.(code, repeat)) return true;
      if (repeat && (code === 'Enter' || code === 'NumpadEnter' || code === 'Space' || code === 'Escape')) return true;   // (a held Enter or Esc acts once)
      switch (code) {
        case 'KeyW': case 'ArrowUp': S.move(-1); return true;
        case 'KeyS': case 'ArrowDown': S.move(1); return true;
        case 'KeyA': case 'ArrowLeft': row?.change?.(-1); return true;
        case 'KeyD': case 'ArrowRight': row?.change?.(1); return true;
        case 'Enter': case 'NumpadEnter': case 'Space': if (row?.confirm) row.confirm(); else S.deploy(); return true;
        case 'Escape': S.back(); return true;
        case 'KeyC': if (!repeat) kitCard?.copy(); return true;    // (copy / paste the picked man's kit from any row: ui/kitCard.ts)
        case 'KeyV': if (!repeat) kitCard?.paste(); return true;
      }
      return false;
  };
  const rowsNow = (): LoadoutRow[] => [
    { el: teamRow, change: dir => S.step(dir) },
    ...(kitBtns.length > 1 && team.cards ? [{ el: kitsRow, change: (dir: 1 | -1) => { for (let k = kitPick + dir; k >= 0 && k < kitBtns.length; k += dir) if (team.kits.find(x => x.id === kitBtns[k].id)?.offered) { kitPick = k; break; } S.paint(); }, confirm: () => putKit(kitBtns[kitPick].id),
      enter: () => { const at = kitBtns.findIndex(k => k.id === team.kits.find(x => x.now)?.id); if (at >= 0) kitPick = at; } }] as LoadoutRow[] : []),
    ...(team.cards ? [
      { el: slots, change: (dir: 1 | -1) => S.pick(member + dir), confirm: () => S.move(1) },
      { el: kitHost, key: (code: string, repeat: boolean) => kitCard?.key({ key: code, code, repeat }) ?? false, enter: (dir: 1 | -1) => kitCard?.go(dir > 0 ? 'first' : 'sets') },
    ] as LoadoutRow[] : []),
    { el: btnRow },
  ];
  S.rows = rowsNow();
  // A MISSION WHOSE LEVEL FILE HANDS OUT ITS OWN WALL CHARGES OR KIT (the plan's own fetch): the desk is told once the file has come, and the screen asks again. A kit of its own takes the cards away: no squad kit row, the note says so and each man who comes shows what the mission hands him (not when the link this screen deploys carries ?noissue: the desk reads that)
  if (entry.kind === 'roomclear' && entry.level && !asIssued) void issuedWalls(entry.level).then(n => { if (!n || !open) return; wallsNow = n; tellFor(); readTeam(); pool.title += ' · this mission issues wall charges of its own, whatever the cards carry'; S.paint(); });
  if (!asIssued && entry.level) void issuedKit(entry.level).then(text => {
    if (!text || !open) return;
    issueText = text; tellFor(); readTeam(); if (team.cards) return;
    const at = S.rows[S.sel]?.el;
    (kitsRow as unknown as { remove?: () => void }).remove?.();
    S.rows = rowsNow(); S.sel = Math.max(0, S.rows.findIndex(r => r.el === at));
    mountCard(); drawSlots(); S.paint();
  });
  goBtn.onclick = () => S.deploy();
  function close() {
    if (!open) return; open = false; if (current === S) current = null; kitCard?.destroy(); kitCard = null; desk().done();   // (…and your own cards stand again)
    const p = root.parentElement as any; if (p?.removeChild) p.removeChild(root); else if (p?.children) { const k = p.children.indexOf(root); if (k >= 0) p.children.splice(k, 1); }
  }
  root.addEventListener?.('mousedown', (e: Event) => { e.stopPropagation(); const t = e.target as HTMLElement | null; if (!t?.closest?.('button, .kc')) e.preventDefault(); });   // (never the game canvas's click; and a click on bare card keeps the keys where they were — in a kit card, say)
  root.addEventListener?.('contextmenu', (e: Event) => e.preventDefault());

  mountCard(); drawSlots(); S.paint();
  opts.host.appendChild(root);
  current = S; currentKeys = opts.keys;
  root.setAttribute('tabindex', '-1');
  return S;
}

/** a mission of the list as this screen takes it: its scenario's row, or the solo night's stand-in */
export function entryOfMission(m: Mission): LoadoutEntry { return m.scenario ?? { id: m.id, title: m.title, kind: 'solo', squad: 0 }; }
// every key is the screen's while it has them (registered at module load: ahead of main.ts's Esc — which would close the start card under it — and of the game's Input)
const eaten = new Set<string>();
if (typeof window !== 'undefined' && typeof window.addEventListener === 'function') {
  window.addEventListener('keydown', e => {
    const S = current; if (!S || !S.open || !currentKeys?.()) return;
    const chord = e.ctrlKey || e.metaKey, clip = e.altKey || (chord && e.shiftKey) || !S.card ? null : clipKey(e);   // (C or V by the letter on the key — kitCard.ts — bare or with Ctrl / Cmd: the kit's copy and paste)
    if (chord && clip) { const t = e.target as { tagName?: string; isContentEditable?: boolean } | null;   // (… the chord only while it is not the browser's own: no text field has the keys, no text on the page is selected)
      if (typedInto(e.target) || t?.isContentEditable || String(window.getSelection?.() ?? '')) return; }   // (the hidden text box of a framed page is no field: host/keymap.ts)
    else if (e.ctrlKey || e.metaKey || e.altKey) return;   // (the browser's own: reload, devtools)
    if (/^F\d+$/.test(e.code)) return;   // (the recorders' keys and the browser's stay theirs, and BEFORE the stop below: F7 to F9 never reached their listeners behind this one)
    e.stopImmediatePropagation(); if (!chord) eaten.add(e.code);   // (a chord's release is not waited for: macOS sends no keyup for a key let go under Cmd, and one left here would eat the game's next release of that key)
    (window as unknown as { __attract?: { poke?: () => void } }).__attract?.poke?.();   // (main.ts's own idle listener is behind this one: somebody is here)
    e.preventDefault();
    S.key(clip === 'copy' ? 'KeyC' : clip === 'paste' ? 'KeyV' : e.code === 'KeyC' || e.code === 'KeyV' ? '' : e.code, e.repeat);   // (a held Enter or Esc acts once, and nothing in a kit card repeats: key() and the card see to it; copy and paste by their letter, so the two places are nothing on a layout that types another letter there)
  }, { capture: true });
  window.addEventListener('keyup', e => { if (eaten.delete(e.code)) { e.stopImmediatePropagation(); if (e.code === 'Space' || e.code === 'Enter' || e.code === 'NumpadEnter') e.preventDefault(); } }, { capture: true });   // (only the releases of presses taken above: a key held since before the screen opened is let go of in the game's Input too)
}

/** a roster chip's mark for a man who wears the copied kit (TeamMan.clip): two sheets, one over the other */
const CLIP_MARK = '<svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M5.5 5.5V2.8h7.7v7.7h-2.7"/><rect x="2.8" y="5.5" width="7.7" height="7.7"/></svg>';
const STYLE = `
#loadout { position: fixed; inset: 0; z-index: 30; display: flex; align-items: center; justify-content: center; background: radial-gradient(ellipse at 50% 45%, rgba(var(--m-base-rgb),0.86), rgba(var(--m-base-rgb),0.96) 75%); color: var(--m-text); font: 13px/1.45 ui-sans-serif, system-ui, -apple-system, 'Helvetica Neue', sans-serif; }
#loadout .lo-card { box-sizing: border-box; width: min(1040px, calc(100% - 32px)); max-height: calc(100% - 16px); display: flex; flex-direction: column; overflow: hidden; padding: 14px 0 10px; background: rgba(var(--m-panel-rgb),0.9); border: 1px solid rgba(var(--m-edge),0.35); border-radius: 6px; box-shadow: 0 10px 40px rgba(0,0,0,0.6); }
#loadout .lo-body { flex: 1 1 auto; min-height: 0; overflow: auto; padding: 0 28px 8px; border-bottom: 1px solid rgba(var(--m-edge),0.14); }   
#loadout .lo-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; margin-top: 8px; padding: 7px 10px; border: 1px solid transparent; border-radius: 4px; }
#loadout .lo-row.sel { border-color: rgba(var(--m-accent-rgb),0.55); background: rgba(var(--m-btn-rgb),0.45); }
#loadout .lo-label { flex: 0 0 auto; font-size: 10px; letter-spacing: 0.24em; text-transform: uppercase; color: var(--m-accent); margin-right: 6px; }
#loadout .lo-words { flex: 0 0 148px; text-align: center; font-size: 18px; letter-spacing: 0.04em; color: var(--m-bright); }
#loadout button { font: inherit; cursor: pointer; }
#loadout .lo-sizes { display: flex; gap: 6px; margin-left: auto; }
#loadout .lo-size { min-width: 46px; padding: 5px 8px; font-size: 11px; letter-spacing: 0.14em; text-transform: uppercase; color: var(--m-btn-text); background: rgba(var(--m-btn-rgb),0.55); border: 1px solid rgba(var(--m-edge),0.3); border-radius: 2px; }
#loadout .lo-size.own { border-style: double; border-width: 3px; padding: 3px 6px; }
#loadout .lo-kits { row-gap: 6px; }
#loadout .lo-set { padding: 5px 8px; font-size: 11px; letter-spacing: 0.1em; text-transform: uppercase; color: var(--m-btn-text); background: rgba(var(--m-btn-rgb),0.55); border: 1px solid rgba(var(--m-edge),0.3); border-radius: 2px; }
#loadout .lo-set.on { color: var(--m-on-accent); background: var(--m-accent-hi); border-color: var(--m-accent-hi); }
#loadout .lo-kits.sel .lo-set.pick { box-shadow: 0 0 0 1px var(--m-bright); }
#loadout .lo-set:hover:not(.on):not(:disabled) { border-color: var(--m-accent-hi); background: rgba(var(--m-hover-rgb),0.8); }
#loadout .lo-kits-own { font-size: 11px; color: var(--m-soft); font-style: italic; }
#loadout .lo-kits-own:empty { display: none; }
#loadout .lo-size.on { color: var(--m-on-accent); background: var(--m-accent-hi); border-color: var(--m-accent-hi); }
#loadout .lo-size.off { opacity: 0.35; cursor: not-allowed; text-decoration: line-through; }
#loadout .lo-size:hover:not(.on):not(.off), #loadout .lo-btn:hover { border-color: var(--m-accent-hi); background: rgba(var(--m-hover-rgb),0.8); }
#loadout .lo-noteline { display: flex; align-items: baseline; gap: 16px; margin: 6px 10px 0; }
#loadout .lo-note { flex: 1 1 auto; min-height: 1.45em; font-size: 12px; color: var(--m-soft); }
#loadout .lo-note.off { color: #e8b46a; }
#loadout .lo-ics { display: none; }
#loadout .lo-kit .kc-row.idle .kc-ico { opacity: 0.28; }   /* (carried, and of no use on this mission — the wand and the cameras with no fog of war: the thing drawn as faint as one not carried, its value 'no use here' in the note's grey; the label stays lit and the arrows work. It was the whole tile at 40 %, still reading 'carried': it looked broken) */
#loadout .lo-kit .kc-row.idle .kc-val, #missions.one #loadout .lo-kit .kc-row.idle .kc-val { color: var(--m-dim, #8f887a); }   /* (the second selector: the one start page colours a value under two ids — ui/startpage.ts) */
#loadout .lo-squadp { flex: 0 0 auto; margin-right: 14px; font: 11px ui-monospace, Menlo, monospace; letter-spacing: 0.06em; color: var(--m-btn-text); white-space: nowrap; }
#loadout .lo-squadp:empty { display: none; }
#loadout .lo-pool { flex: 0 0 auto; font: 11px ui-monospace, Menlo, monospace; letter-spacing: 0.06em; color: var(--m-text); white-space: nowrap; }
#loadout .lo-slots { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 10px; margin-top: 6px; padding: 4px; border: 1px solid transparent; border-radius: 6px; }
#loadout .lo-slots.sel { border-color: rgba(var(--m-accent-rgb),0.55); background: rgba(var(--m-btn-rgb),0.45); }
#loadout .lo-slot { display: grid; grid-template-columns: 30px minmax(0, 1fr); column-gap: 8px; align-content: start; padding: 8px 9px 8px; border: 1px solid rgba(var(--m-edge),0.3); border-top-width: 3px; border-radius: 4px; background: rgba(var(--m-panel2-rgb),0.7); color: var(--m-accent-hi); cursor: pointer; }
#loadout .lo-slot.p-you { border-top-color: var(--m-accent-hi); color: var(--m-accent-hi); }
#loadout .lo-slot.p-red { border-top-color: #e2685c; color: #f0958b; }
#loadout .lo-slot.p-blue { border-top-color: #5c9be2; color: #8fbdf2; }
#loadout .lo-slot.out { border-style: dashed; border-top-width: 1px; background: none; color: var(--m-faint); }
#loadout .lo-slot.out:hover, #loadout .lo-slot.in:hover { border-color: rgba(var(--m-accent-rgb),0.6); }
#loadout .lo-slot.out:hover { color: var(--m-dim); }
#loadout .lo-slot.pick { border-color: var(--m-accent-hi); background: rgba(var(--m-sel-rgb),0.8); box-shadow: 0 0 0 1px rgba(var(--m-accent-rgb),0.25); }
#loadout .lo-name { align-self: end; font-size: 14px; font-weight: 600; letter-spacing: 0.04em; color: var(--m-bright); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
#loadout .lo-name.changed::after { content: ' •'; color: #f3d79a; }
#loadout .lo-slot.out .lo-name { color: var(--m-faint); font-weight: 400; }
#loadout .lo-role { font-size: 10px; letter-spacing: 0.16em; text-transform: uppercase; color: inherit; }
#loadout .lo-sum { grid-column: 1 / span 2; margin-top: 6px; min-height: 2.6em; font: 10.5px/1.3 ui-monospace, Menlo, monospace; color: var(--m-soft); }
#loadout .lo-pouch { grid-column: 1 / span 2; margin-top: 3px; font: 10px/1.3 ui-monospace, Menlo, monospace; letter-spacing: 0.1em; text-transform: uppercase; color: var(--m-btn-text); }
#loadout .lo-slot.clip { position: relative; }
#loadout .lo-clip { position: absolute; top: 8px; right: 9px; line-height: 0; color: var(--m-soft); }
#loadout .lo-slot.pick .lo-clip { color: var(--m-text); }
#loadout .lo-kit { margin-top: 8px; }
#loadout .lo-kit:empty { display: none; }
#loadout .lo-kit .kc { width: 100%; display: grid; grid-template-columns: 200px repeat(3, minmax(0, 1fr)); grid-auto-columns: minmax(0, 1fr); grid-template-rows: repeat(4, auto) auto; grid-auto-flow: column; column-gap: 22px; padding: 12px 16px 10px; }   /* (three columns of rows, each a share of the width: a ninth row's note is as long as it likes and squeezes nobody) */
#loadout .lo-kit .kc-head { grid-column: 1; grid-row: 1 / span 2; margin: 0; align-self: start; }
#loadout .lo-kit .kc-words { grid-column: 1; grid-row: 3 / span 3; margin: 0; padding-top: 8px; align-self: start; }
#loadout .lo-kit .kc-row { margin: 0; }
#loadout .lo-kit .kc-sets { grid-column: 2 / -1; grid-row: 5; margin: 6px 0 0; }
#loadout .lo-kit .kc-set { flex: 0 1 150px; padding: 5px 2px; }
#loadout .lo-kit .kc-sets { flex-wrap: nowrap; }
#loadout .lo-kit .kc-clip { padding: 5px 12px; }   /* (copy and paste — ui/kitCard.ts: the pair at the far end of the sets' line, in the sets' own look on either page) */
#loadout .lo-actions { flex: 0 0 auto; justify-content: flex-end; margin: 6px 28px 0; }
#loadout .lo-btn { font-size: 12px; letter-spacing: 0.12em; text-transform: uppercase; padding: 8px 14px; background: rgba(var(--m-btn-rgb),0.7); color: var(--m-btn-text); border: 1px solid rgba(var(--m-edge),0.45); border-radius: 3px; }
#loadout .lo-btn.primary { min-width: 220px; background: var(--m-accent); color: var(--m-on-accent); border-color: var(--m-accent); font-weight: 600; }
#loadout .lo-btn.primary:hover { background: var(--m-accent-hi); }
#loadout .lo-btn.sel { outline: 2px solid var(--m-bright); outline-offset: 2px; }
/* THE ROSTER AND THE KIT TABLE: each man a card with his gun on it, the picked man's kit as tiles — a silhouette, the thing's name, how many — in columns */
#loadout .lo-label { font: 700 12px/1 var(--m-head); letter-spacing: 0.22em; }
#loadout .lo-words { font: 700 20px/1 var(--m-head); letter-spacing: 0.08em; text-transform: uppercase; }
#loadout .lo-set, #loadout .lo-size { font: 600 12px/1.4 var(--m-head); letter-spacing: 0.12em; border-radius: 0; }
#loadout .lo-set.on { background: var(--m-accent); border-color: var(--m-accent); }
#loadout .lo-slots { gap: 8px; }
#loadout .lo-slot { padding: 8px 10px 9px; border-radius: 0; border-top-width: 5px; background: rgba(var(--m-panel2-rgb),0.8); }
#loadout .lo-slot.out { border-top-width: 1px; background: none; }
#loadout .lo-slot.pick { background: rgba(var(--m-sel-rgb),0.95); border-left-color: var(--m-accent); border-right-color: var(--m-accent); border-bottom-color: var(--m-accent); box-shadow: 0 0 0 1px var(--m-accent); }
#loadout .lo-name { font: 700 17px/1.05 var(--m-head); letter-spacing: 0.07em; text-transform: uppercase; }
#loadout .lo-role { font: 600 10.5px/1.3 var(--m-head); letter-spacing: 0.18em; }
#loadout .lo-gun { grid-column: 1 / span 2; height: 34px; margin: 7px 0 0; color: var(--m-btn-text); opacity: 0.9; }
#loadout .lo-gun svg { display: block; height: 100%; width: auto; max-width: 100%; }
#loadout .lo-sum { margin-top: 5px; font: 11px/1.35 var(--m-font); }
#loadout .lo-pouch { font: 600 10.5px/1.3 var(--m-head); letter-spacing: 0.16em; color: var(--m-soft); }
#loadout .lo-kit .kc { grid-template-columns: 190px repeat(3, minmax(0, 1fr)); column-gap: 8px; row-gap: 8px; padding: 12px; font-family: var(--m-font); border-radius: 0; background: rgba(var(--m-base-rgb),0.6); }
#loadout .lo-kit .kc-head { padding: 2px 4px 0; }
#loadout .lo-kit .kc:not(:has(.kc-row ~ .kc-row ~ .kc-row ~ .kc-row ~ .kc-row ~ .kc-row ~ .kc-row ~ .kc-row ~ .kc-row)) { grid-template-columns: 190px repeat(2, minmax(0, 1fr)); }   /* (eight rows or fewer fill two columns of four: no empty third) */
#loadout .lo-kit .kc-title { font: 700 22px/1 var(--m-head); letter-spacing: 0.06em; text-transform: uppercase; }
#loadout .lo-kit .kc-sub, #loadout .lo-kit .kc-pouch { font-family: var(--m-head); font-weight: 600; font-size: 11px; letter-spacing: 0.16em; }
#loadout .lo-kit .kc-pips i { width: 12px; height: 12px; border-radius: 0; }
#loadout .lo-kit .kc-pips i.on { background: var(--m-accent); }
#loadout .lo-kit .kc-pips i.big { width: 30px; margin-left: 5px; }   /* (the big slot: game/kit.ts BIGSLOT) */
#loadout .lo-kit .kc-words { padding: 8px 4px 0; font-size: 11.5px; line-height: 1.4; border-top: 0; }
#loadout .lo-kit .kc-row { display: grid; grid-template-columns: 64px minmax(0, 1fr); column-gap: 10px; align-content: start; padding: 7px 8px 7px 9px; background: rgba(var(--m-panel2-rgb),0.85); border: 1px solid rgba(var(--m-edge),0.16); border-left: 3px solid rgba(var(--m-edge),0.3); border-radius: 0; }
#loadout .lo-kit .kc.focus .kc-row.sel { background: rgba(var(--m-sel-rgb),0.95); border-color: var(--m-accent); border-left-color: var(--m-accent); }
#loadout .lo-kit .kc-ico { display: block; grid-column: 1; grid-row: 1 / span 2; align-self: center; height: 32px; color: var(--m-btn-text); }
#loadout .lo-kit .kc-ico svg { display: block; width: 100%; height: 100%; }
#loadout .lo-kit .kc-ico.none { opacity: 0.28; }
#loadout .lo-kit .kc-row.changed .kc-ico { color: #f3d79a; }
#loadout .lo-kit .kc-line { grid-column: 2; gap: 3px 5px; flex-wrap: wrap; }
#loadout .lo-kit .kc-label { order: 1; flex: 1 1 0; min-width: 0; font: 600 11px/1.2 var(--m-head); letter-spacing: 0.1em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
#loadout .lo-kit .kc-val, #loadout .lo-kit .kc-val.num { order: 4; flex: 1 0 100%; min-width: 0; text-align: left; font: 700 16px/1.1 var(--m-head); letter-spacing: 0.05em; text-transform: uppercase; }
#loadout .lo-kit .kc-line .kc-step { order: 2; }
#loadout .lo-kit .kc-step { width: 22px; height: 22px; border-radius: 0; font-family: var(--m-font); }
#loadout .lo-kit .kc-note { grid-column: 2; margin-top: 3px; font-size: 11px; line-height: 1.3; min-height: 0; }
#loadout .lo-kit .kc-sets { margin: 0; padding: 0; gap: 6px; }
#loadout .lo-kit .kc-set, #loadout .lo-kit .kc-clip { font: 600 12px/1.4 var(--m-head); letter-spacing: 0.12em; border-radius: 0; }
#loadout .lo-kit .kc-clip.on { background: var(--m-accent); border-color: var(--m-accent); }
#missions.one #loadout .lo-kit .kc-clip { min-width: 0; padding: 7px 13px 6px; font: 600 12px/1.3 var(--m-head); letter-spacing: 0.12em; text-transform: uppercase; color: var(--m-soft); background: none; border: 0; border-radius: 0; box-shadow: none; }
#missions.one #loadout .lo-kit .kc-clip:hover:not(.on):not(.off) { color: var(--m-bright); background: rgba(var(--m-edge),0.1); border: 0; }
#missions.one #loadout .lo-kit .kc-clip.on { color: var(--m-bright); background: none; box-shadow: inset 0 -2px 0 var(--m-soft); }
/* the card as a tree (game/kittree.ts; the one start page lays it out in full — ui/startpage.ts): a gun's tile over what rides on it */
#loadout .lo-kit .kc-tree .kc-gun { display: flex; flex-direction: column; gap: 4px; min-width: 0; }
#loadout .lo-kit .kc-tree .kc-kids { display: flex; flex-direction: column; gap: 4px; margin: 0 0 0 12px; padding: 0 0 0 8px; border-left: 2px solid rgba(var(--m-edge),0.28); }
#loadout .lo-kit .kc-tree .kc-kids:empty { display: none; }
#loadout .lo-kit .kc-tree .kc-stats { grid-column: 1 / -1; }
#loadout .lo-btn { font: 700 14px/1 var(--m-head); letter-spacing: 0.16em; padding: 11px 20px 10px; border-radius: 0; }
#loadout .lo-btn.primary { font-size: 17px; }
@media (max-width: 1150px) { #loadout .lo-kit .kc { grid-template-columns: 170px repeat(3, minmax(0, 1fr)); } #loadout .lo-kit .kc-row { grid-template-columns: minmax(0, 1fr); } #loadout .lo-kit .kc-ico { display: none; } #loadout .lo-kit .kc-line, #loadout .lo-kit .kc-note { grid-column: 1; } }
@media (max-width: 700px) { #loadout .lo-slots { grid-template-columns: repeat(2, minmax(0, 1fr)); } #loadout .lo-sizes { margin-left: 0; } #loadout .lo-kit .kc { display: block; } }
`;
function injectStyle() {
  if (typeof document === 'undefined' || !document.head || document.getElementById('loadout-style')) return;
  menuTheme(); const s = document.createElement('style'); s.id = 'loadout-style'; s.textContent = STYLE; document.head.appendChild(s);
}
