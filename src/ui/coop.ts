// CO-OP ON THE PAGE: host a game or join one (the door, the words and the control plane's calls are net/lobby.ts; the page once it is in a game is
// net/page.ts). Four small things, each where a player would look:
//   the title's CO-OP row    (ui/menu.ts hands this file its column of buttons: `coopRows`) — 'Host a game', and a field for a code with 'Join'
//   hosting                  the ONE STARTING PAGE in its co-op dress (ui/startpage.ts `coop`): the missions two can play, the setup rail as ever, no kit step
//                            (both players play the kit as issued today), and its block makes the game instead of loading the mission (`host`)
//   the door                 `coopFromUrl`, asked by main.ts where it asks a replay's and a networked game's: an address with `?join=<CODE>` (the link, a
//                            page loaded again) is given its seat and sent on to the game's server; a line says what it is waiting for, or why it cannot.
//                            The title's own two ways in go through the same door with no page load of their own (`enter`): the one load is the game's
//   the invite               `coopInvite`, on the host's page while the server waits for the second player: the code, the link and a button that copies it
// Plain DOM. THE SWITCH is lobby.ts COOP.on (`?coop=0`: none of this is on the page). Nothing here is read by a tick.
import { COOP, Lobby, codeTyped, codeWhole, coopScenario, joinLink, netSearch, lobbyWhy, freshId, type GameView } from '../net/lobby';
import { SENDING_WORDS } from '../replay/engineplay';
import { VISIT_FLAGS } from './replaykey';
import { NET_JOIN, buildName } from '../net/page';
import { cloudFind, cloudUrl } from '../cloud/cloud';
import { wire } from '../cloud/wire';
import { keptLevers, leversNow } from '../game/difficulty';
import { seatsKind } from '../game/seats';
import { lessonGroup } from '../scene/scenarios';
import { openList } from './startpage';
import { listedMissions, type Mission } from './missions';

const TAB = 'tps.coop.tab', ID = /^[a-z0-9]{24,64}$/;
let mem = '';
/** this tab's id as a seat's holder (lobby.ts: the tab, not the browser), kept for the tab so a page loaded again is the same player */
function tabId(): string {
  try { const k = sessionStorage.getItem(TAB); if (k && ID.test(k)) return k; const id = freshId(); sessionStorage.setItem(TAB, id); return id; } catch { return mem ||= freshId(); }
}
/** the lobby of this visit's backend, or the words for why this page has none */
async function lobby(q: URLSearchParams): Promise<Lobby | string> {
  await cloudFind();
  const url = cloudUrl(q); if (!url) return 'You can’t play co-op here because this page isn’t connected to the game service.';
  const build = await buildName(); if (!build) return 'You can’t play co-op on this copy of the game.';
  return new Lobby(wire(url), tabId(), build);
}

// ---- the one line that says what the page is waiting for, or why not (the look of net/page.ts's line: this one is up before there is a game)
let lineEl: HTMLElement | null = null;
function say(text: string, passing = false): void {
  if (typeof document === 'undefined') return;
  if (!lineEl) { const d = lineEl = document.createElement('div'); d.id = 'coopline'; d.setAttribute('role', 'status'); d.style.cssText = 'position:fixed;left:50%;top:14%;transform:translateX(-50%);z-index:60;padding:8px 16px;border-radius:6px;background:rgba(10,12,14,.88);color:#e8e6df;font:500 15px/1.3 system-ui,sans-serif;pointer-events:none;max-width:80vw;text-align:center'; document.body.appendChild(d); }
  lineEl.textContent = text; lineEl.style.display = text ? 'block' : 'none';
  if (passing && text) { const off = () => { if (lineEl?.textContent === text) say(''); }; setTimeout(() => { window.addEventListener('keydown', off, { once: true, capture: true }); window.addEventListener('mousedown', off, { once: true, capture: true }); }, 1500); }   // (it stays until the player does something: a refusal read in passing is a refusal missed)
}

/** THE DOOR (main.ts): this address names a co-op game. 'moved' = the page is on its way to the game's server (stop booting); null = it is not a game this
 *  page can play: the words are up, the code is off the address, and the page boots as itself */
export async function coopFromUrl(q: URLSearchParams): Promise<'moved' | null> {
  const stay = (why: string): null => { say(why, true); try { const u = new URLSearchParams(location.search); u.delete(NET_JOIN); const s = u.toString(); history.replaceState(null, '', location.pathname + (s ? '?' + s : '') + location.hash); } catch { /* a sandboxed frame */ } return null; };
  const code = (q.get(NET_JOIN) ?? '').trim().toUpperCase();   // (an address's code is a code or it is not: nothing is read out of other letters, as a typed one is)
  if (!codeWhole(code)) return stay('There’s no game with that code. Check the code with your friend.');
  say(WAIT.join);
  const L = await lobby(q); if (typeof L === 'string') return stay(L);
  let v: GameView;
  try { v = await L.seat(code, s => say(waitWords(s))); } catch (e) { return stay(String((e as Error)?.message ?? e)); }
  location.replace(location.pathname + netSearch(q, v) + location.hash);
  return 'moved';
}
const WAIT = { host: 'Starting your game…', join: 'Joining the game…', theirs: 'Your friend’s game is starting…' };
/** what a tab that holds a seat is waiting for */
const waitWords = (v: GameView): string => v.state !== 'starting' ? WAIT.join : v.seat === 0 ? WAIT.host : WAIT.theirs;
/** THROUGH THE DOOR WITHOUT A PAGE LOAD (the title's own two ways in): this tab's seat in the game `code` names, then the one load that is the game's own.
 *  Resolves to why not ('' = the page is on its way) */
async function enter(L: Lobby, code: string, told: (words: string) => void): Promise<string> {
  try { const v = await L.seat(code, s => told(waitWords(s))); location.assign(location.pathname + netSearch(location.search, v) + location.hash); return ''; } catch (e) { return String((e as Error)?.message ?? e); }
}

/** the missions two can play: a kind the game seats two in (game/seats.ts), and not a lesson (on rails, for one) or a row built for its own size only */
export const coopCan = (m: Mission): boolean => !!m.scenario && seatsKind(m.scenario.kind) && !m.scenario.only && !lessonGroup(m.group);
/** HOST: the starting page, picking a mission for two. Its block hands the mission's address here */
export function coopHostList(select?: string): void {
  (window as unknown as { __coop?: unknown }).__coop = { hostable: () => listedMissions().filter(coopCan).map(m => ({ id: m.id, group: m.group, kind: m.scenario?.kind ?? '' })) };   // (a check's: which rows the page was handed)
  openList({ backLabel: '‹ Main menu', ...(select ? { select } : {}), coop: { title: 'Host a co-op game', words: 'Host game', can: coopCan, deploy: (search, m) => void host(search, m) } });
}
let hosting = false;   // (one game a press: a second press while the first is being made would make a second server)
async function host(search: string, m: Mission): Promise<void> {
  if (hosting) return; hosting = true;
  say(WAIT.host);
  const fail = (why: string): void => { hosting = false; say(why, true); coopHostList(m.id); };   // (the page closed its kit as it deployed: it is put up again on the same mission)
  const q = new URLSearchParams(location.search), L = await lobby(q); if (typeof L === 'string') return fail(L);
  const K = keptLevers() as Record<string, string | undefined>, kept = leversNow().flatMap(l => { const v = K[l.key]; return v === undefined ? [] : [[l.param, v] as [string, string]]; });
  // (the mission's address is the starting page's link, which carries this VISIT's words along — a picture's quality, a muted mixer, where it sends: those
  // are this window's, not the game's, and the second player's page must not take them from the recipe)
  const own = new URLSearchParams(search.replace(/^[^?]*\?/, '')); for (const k of [...VISIT_FLAGS, ...SENDING_WORDS, 'seed', 'player']) own.delete(k);
  let made: GameView; try { made = await L.make(coopScenario('?' + own.toString(), kept)); } catch (e) { return fail(lobbyWhy(e, 'host')); }
  const why = await enter(L, made.code, say); if (why) fail(why);
}

// ---- the title's CO-OP page: the title's own column of buttons, filled by this file (ui/menu.ts puts the row there and takes the column back on `back`)
let styled = false;
function styles(): void {
  if (styled || typeof document === 'undefined') return; styled = true;
  const s = document.createElement('style'); s.textContent = `
#menu.title .actions .corow { display: flex; gap: 4px; width: min(360px, 100%); }
#menu.title .actions .corow .btn { width: auto; flex: 0 0 auto; }
#menu.title .actions .cocode { flex: 1 1 0; min-width: 0; box-sizing: border-box; font: 600 24px/1 var(--m-head); letter-spacing: 0.3em; text-transform: uppercase; padding: 13px 14px 11px; color: var(--m-bright); background: rgba(var(--m-btn-rgb),0.4); border: 0; border-left: 4px solid rgba(var(--m-edge),0.3); border-radius: 0; outline: none; }
#menu.title .actions .cocode:focus { border-left-color: var(--m-accent); background: rgba(var(--m-hover-rgb),0.7); }
#menu.title .actions .cocode::placeholder { color: var(--m-dim); opacity: 0.6; }
#menu.title .actions .conote { min-height: 1.3em; margin: 4px 0 6px 4px; font: 500 14px/1.3 system-ui, sans-serif; color: var(--m-bright); max-width: 360px; }
#coopinvite { position: fixed; left: 50%; top: 24%; transform: translateX(-50%); z-index: 41; padding: 18px 24px 16px; border-radius: 6px; background: rgba(10,12,14,.9); color: #e8e6df; font: 500 15px/1.4 system-ui, sans-serif; text-align: center; max-width: min(520px, 86vw); }
#coopinvite .ck { font: 600 11px/1.2 system-ui, sans-serif; letter-spacing: 0.22em; text-transform: uppercase; opacity: 0.7; }
#coopinvite .cc { margin: 6px 0 10px; font: 700 44px/1 ui-monospace, Menlo, Consolas, monospace; letter-spacing: 0.24em; padding-left: 0.24em; user-select: all; }
#coopinvite .cl { display: flex; gap: 8px; align-items: center; justify-content: center; margin: 0 0 10px; }
#coopinvite .cl span { font: 500 13px/1.3 ui-monospace, Menlo, Consolas, monospace; opacity: 0.85; user-select: all; overflow-wrap: anywhere; text-align: left; }
#coopinvite button { flex: 0 0 auto; font: 600 12px/1 system-ui, sans-serif; letter-spacing: 0.08em; text-transform: uppercase; padding: 8px 12px; color: #111; background: #e8e6df; border: 0; border-radius: 3px; cursor: pointer; }
`; document.head.appendChild(s);
}
const el = (tag: string, cls?: string, text?: string): HTMLElement => { const e = document.createElement(tag); if (cls) e.className = cls; if (text !== undefined) e.textContent = text; return e; };

/** what the title's CO-OP page holds: kept here, not in its buttons, because the title is drawn twice (the boot card, then the live title over it) and a
 *  code half typed, or a join on its way, is the same one on both */
const PAGE = { code: '', note: '', busy: false, typing: false, noteEl: null as HTMLElement | null };
const tell = (words: string): void => { PAGE.note = words; if (PAGE.noteEl) PAGE.noteEl.textContent = words; };
/** the title's column, as the CO-OP page: host, or type a code. `back` = the title's own column again; `focus` = put the keyboard on its first button */
export function coopRows(actions: HTMLElement, back: () => void, focus = true): void {
  styles();
  actions.innerHTML = '';
  const btn = (text: string, run: () => void, cls = ''): HTMLButtonElement => { const b = el('button', 'btn ' + cls, text) as HTMLButtonElement; b.type = 'button'; b.onclick = run; return b; };
  const hostBtn = btn('Host a game', () => coopHostList()); hostBtn.dataset.coop = 'host';
  const head = el('div', 'xhead', 'Join a game');
  const row = el('div', 'corow'), code = el('input', 'cocode') as HTMLInputElement, note = el('div', 'conote'); note.setAttribute('aria-live', 'polite');
  code.type = 'text'; code.placeholder = 'Code'; code.maxLength = 12; code.autocomplete = 'off'; code.spellcheck = false; code.setAttribute('aria-label', 'The game’s code'); code.dataset.coop = 'code';
  code.value = PAGE.code; note.textContent = PAGE.note; PAGE.noteEl = note;
  const join = (): void => { const c = codeTyped(code.value); if (PAGE.busy) return; if (!codeWhole(c)) { tell('Type the whole code first.'); code.focus(); return; }
    PAGE.busy = true; tell(WAIT.join);   // (the seat is asked for from here: a wrong code is said under the field it was typed in, with no page load to learn it)
    void (async () => { const L = await lobby(new URLSearchParams(location.search)), why = typeof L === 'string' ? L : await enter(L, c, tell); if (why) { PAGE.busy = false; tell(why); const f = PAGE.noteEl?.parentElement?.querySelector('input'); f?.focus(); f?.select(); } })(); };
  const joinBtn = btn('Join', join); joinBtn.dataset.coop = 'join';
  code.addEventListener('input', () => { const c = codeTyped(code.value); if (c !== code.value) code.value = c; PAGE.code = c; if (!PAGE.busy) tell(''); });
  code.addEventListener('keydown', e => { if (e.code === 'Enter' || e.code === 'NumpadEnter') { e.preventDefault(); join(); } if (e.code !== 'Escape' && e.code !== 'Tab') e.stopPropagation(); });   // (a letter typed here is the code's, nobody else's key)
  row.append(code, joinBtn);
  const backBtn = btn('‹ Main menu', back, 'xback');   // (Esc is the title's own: ui/menu.ts presses this button)
  actions.append(hostBtn, head, row, note, backBtn);
  if (focus) hostBtn.focus(); else if (PAGE.typing) { code.focus(); code.setSelectionRange(code.value.length, code.value.length); }   // (the title drawn again under a player who is typing: he goes on typing)
  code.addEventListener('focus', () => { PAGE.typing = true; }); code.addEventListener('blur', () => { if (code.isConnected) PAGE.typing = false; });
}

/** THE INVITE (main.ts, once a networked page stands): on the host's page, while the server waits for the second player, the code and the link to send.
 *  Gone when the fight starts. `net.client` is the page's client (net/client.ts) */
export function coopInvite(net: { client: { state: string; seat: number; bye: string; broken: string } }, q: URLSearchParams): void {
  const code = q.get(NET_JOIN) ?? ''; if (!COOP.on || !codeWhole(code) || typeof document === 'undefined' || net.client.seat !== 0) return;
  styles();
  const link = joinLink(location.origin, location.pathname, code);
  const card = el('div'); card.id = 'coopinvite'; card.style.display = 'none'; card.addEventListener('mousedown', e => e.stopPropagation());
  const words = el('div', undefined, 'Send your friend this code or the link. The mission will start when they join.');
  const copy = el('button', undefined, 'Copy link') as HTMLButtonElement; copy.type = 'button'; copy.dataset.coop = 'copy';
  copy.onclick = () => { void (async () => { try { await navigator.clipboard.writeText(link); copy.textContent = 'Copied'; } catch { copy.textContent = 'Select the link to copy it'; } setTimeout(() => { copy.textContent = 'Copy link'; }, 2000); })(); };
  const cl = el('div', 'cl'), ln = el('span', undefined, link); ln.dataset.coop = 'link'; cl.append(ln, copy);
  const cc = el('div', 'cc', code); cc.dataset.coop = 'shown';
  card.append(el('div', 'ck', 'Co-op code'), cc, cl, words); document.body.appendChild(card);
  // (the second player is in and loading: the control plane knows — the server tells it who is connected — and the card says so instead of asking for him)
  let here = false, asked = 0, L: Lobby | string | null = null; void lobby(q).then(l => { L = l; });
  const began = Date.now();
  const tick = (): void => {
    const c = net.client, waiting = c.state === 'waiting' && !c.bye && !c.broken;
    if (!waiting) { clearInterval(t); card.remove(); return; }
    card.style.display = 'block';
    const now = Date.now();   // (a friend who is coming is here within the minute: after it the control plane is asked less often)
    if (L && typeof L !== 'string' && now - asked > (now - began < 60_000 ? 2000 : 10_000)) { asked = now; void L.get(code).then(v => { here = !!v?.connected[1]; }, () => { /* asked again */ }); }
    cc.style.display = cl.style.display = here ? 'none' : ''; words.textContent = here ? 'Your friend is joining. The mission will start in a moment.' : 'Send your friend this code or the link. The mission will start when they join.';
  };
  const t = setInterval(tick, 250); tick();   // (up at once: the page's first seconds are busy ones, and a timer's first turn can be a long way off)
}
