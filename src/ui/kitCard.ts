// A MEMBER'S CARD (the desk answers what it says, src/view/desk.ts and read.ts CardView; this is its face, ui/kitDense.ts its lay-out): memberCard(host, i) fills `host` — the start page's kit step mounts one in
// an element with data-member="i", 0 = you, 1 to 4 = the teammates — with member i's rows: the two guns as tiles with pop-ups, the rest of the kit as small tiles,
// the colours, the reset and copy and paste, and the card in one line.
//
// POUCHES: the head says how many of the man's pouches are full, as pips and 'n / m', and for a teammate, while the table
// sets a squad limit, the squad's ('squad 7 / 7': what the four carry between them); every row that spends them opens its line with what it fills now and its rate ('2 pouches · a pouch
// each · …'); an arrow whose step has no room is dimmed and, pressed (or A / D), changes nothing and puts the reason on the row in amber until the next
// press; a row of choices that walked past one with no room says which and why the same way; a set the squad has no room for is dimmed and, pressed, says
// why under the rows.
//
// WHAT THE CARD WEIGHS: two bars beside the pouches, mobility and concealment — each
// full for a man with nothing on him, shorter for every thing on his card, redrawn as a row is stepped. Shown, not told: no number; a notch on each bar is
// where his issued card stands (the man the game was tuned on), grey up to it, GREEN for what this card has gained on it and AMBER for what it has lost
// under it; and the highlighted row is outlined on the bar — what its thing takes (leave it behind and the bar grows to there), or, dashed, what one more
// of a thing he carries none of would take.
//
// THE MOUSE does everything: a click on an arrow steps the row, on the reset presses it, anywhere on the card takes the keys to it.
// THE KEYS (no key is held, no digit is read): W / S or up / down move the highlighted row (the last 'row' is the sets), A / D or left / right change its
// value, Enter or Space confirm (a row: its next value; the sets: the reset), Esc is the card's only while a pop-up is open. The host routes keys through
// card.key(e) — true = the card took it; W on the top row and S on the sets come back false, so the screen walks on (ui/loadout.ts, and Esc is the screen's).
//
// COPY AND PASTE (the maintainer, 10-15: "we should have copy and paste for loadouts"): two buttons at the end of the sets' line and two keys, C and V (the
// loadout screen hands them over from any of its rows, and takes Ctrl / Cmd with them as the same press). Copy keeps the card as it stands (the desk's, one
// for the page, until the next copy or the page's next load); Paste puts it on this man — fitted to his own rows, his colours
// kept, refused with the reason under the rows where the squad has no room. SHOWN, NOT TOLD: Copy is lit on every card that wears the copied kit (a paste
// there would change nothing, so Paste is dim), the tiles a paste moved flash once, and the roster marks the men who wear it (TeamMan.clip).
// The two keys go by the LETTER on the key (a mnemonic: C and V sit elsewhere on Dvorak), by its place only where the layout has no Latin letter there.
//
// The card owns its own CSS (one <style> on first use), so index.html needs nothing for it.
import { desk } from '../view/desk';
import type { CardRow, CardView, Pressed } from '../view/read';
import { menuTheme } from './theme';
import { kitIcon } from './kitIcons';
import { denseStyle } from './kitDense';

/** which gun a tile is */
type GunSlot = 'primary' | 'secondary';
export interface MemberCardOpts {
  /** who the head says he is (the callsign and pair the size shown stages him with) */
  name: { title: string; sub: string };
  /** a row or a set changed member i's card (the desk has it, and it is kept) */
  onChange?: (i: number) => void;
  /** the card took the keys (a click on it, focus()) */
  onFocus?: (i: number) => void;
  /** the highlight moved to another row of member i's card (its field; null: the sets) — a move, not a change of value */
  onSelect?: (i: number, field: string | null) => void;
  /** member i's card was copied (no card changed) */
  onCopy?: (i: number) => void;
  /** how far down the window a pop-up of the card's may reach, in the window's pixels (the top of the host's foot: its buttons lie
   *  under the card and stay where they are when the page scrolls). Absent or 0 = the window's own foot */
  floor?: () => number;
}
/** what an idle tile says in its value's place, and its tooltip (CardRow.idle: carried, and of no use on the mission the card is shown for) */
const IDLE_WORD = 'no use here', IDLE_WHY = 'There\'s no fog of war on this mission, so every room is already shown and it has nothing to look into. It still takes up its pouch.';
/** the four bars on a gun's tile, in the order the card says them (CardBar, CardPick.bars) */
const GUN_STATS: readonly { id: string; label: string; tip: string }[] = [
  { id: 'reach', label: 'Reach', tip: 'reach is how far it can keep its shots on an enemy' },
  { id: 'volume', label: 'Volume', tip: 'volume is how many rounds it fires in ten seconds, reloads included' },
  { id: 'handling', label: 'Handling', tip: 'handling is how quickly you can swing it around' },
  { id: 'quiet', label: 'Quiet', tip: 'quiet is how little noise it makes' },
];
/** a key press as copy or paste (null: neither): the letter the key types, else — no Latin letter on it — the key's place */
export function clipKey(e: { key: string; code?: string }): 'copy' | 'paste' | null {
  const k = /^[a-z]$/i.test(e.key) ? e.key.toLowerCase() : e.code === 'KeyC' || e.key === 'KeyC' ? 'c' : e.code === 'KeyV' || e.key === 'KeyV' ? 'v' : '';
  return k === 'c' ? 'copy' : k === 'v' ? 'paste' : null;
}
/** every card on the page redraws its Copy and Paste when one of them copies (the copied card is the page's, not a card's) */
const liveCards = new Set<{ root: HTMLElement; draw: (fresh: boolean) => void }>();
/** the bits of a KeyboardEvent the card reads (a probe hands a plain object) */
export interface KeyLike { key: string; code?: string; repeat?: boolean; ctrlKey?: boolean; metaKey?: boolean; altKey?: boolean }
export interface MemberCard {
  el: HTMLElement;
  /** take the keys (the highlight shows) / give them up */
  focus(): void; blur(): void;
  /** the highlighted row: 0 … rows − 1 among those the card has as it stands, or rows = the sets */
  at(): number;
  /** put the highlight on a row by its field ('first': the first row, 'sets': the sets; a host walking in from above goes to the first, from below to the sets) */
  go(where: string): void;
  /** a key press: true = the card's (taken), false = not the card's (or an edge: W at the top, S at the bottom) */
  key(e: KeyLike): boolean;
  /** the field of the highlighted row (null: the sets) */
  field(): string | null;
  /** redraw from the desk (after it was changed elsewhere) */
  refresh(): void;
  /** keep this man's card as the copied one · put the copied card on him (false: nothing copied, or refused) */
  copy(): void; paste(): boolean;
  /** the gun whose pop-up is open (null: none) · open one's, or shut the open one (null) */
  pop(): GunSlot | null; open(slot: GunSlot | null): void;
  destroy(): void;
}

const STYLE_ID = 'kc-style';
const CSS = `
.kc { box-sizing: border-box; width: 250px; padding: 12px 12px 10px; font: 12px/1.35 ui-monospace, SFMono-Regular, Menlo, monospace; color: var(--m-text); background: rgba(var(--m-panel-rgb),0.9); border: 1px solid rgba(var(--m-edge),0.28); border-radius: 4px; outline: none; user-select: none; }
.kc.focus { border-color: var(--m-accent-hi); box-shadow: 0 0 0 1px rgba(var(--m-accent-rgb),0.25); }
.kc .kc-head { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
.kc .kc-fig { position: relative; flex: 0 0 34px; height: 46px; }
.kc .kc-fig i { position: absolute; display: block; }
.kc .kc-fig .h { left: 11px; top: 0; width: 12px; height: 12px; border-radius: 50%; }
.kc .kc-fig .b { left: 6px; top: 13px; width: 22px; height: 20px; border-radius: 4px 4px 2px 2px; }
.kc .kc-fig .l { top: 33px; width: 9px; height: 13px; border-radius: 0 0 2px 2px; } .kc .kc-fig .l1 { left: 7px; } .kc .kc-fig .l2 { left: 18px; }
.kc .kc-fig .a { top: 14px; width: 5px; height: 17px; border-radius: 2px; } .kc .kc-fig .a1 { left: 0; } .kc .kc-fig .a2 { left: 29px; }
.kc .kc-fig .j { width: 5px; height: 5px; border-radius: 50%; } .kc .kc-fig .j1 { left: 0; top: 13px; } .kc .kc-fig .j2 { left: 29px; top: 13px; } .kc .kc-fig .j3 { left: 9px; top: 32px; } .kc .kc-fig .j4 { left: 20px; top: 32px; }
.kc .kc-fig i.issued { background: rgba(var(--m-edge),0.16); box-shadow: inset 0 0 0 1px rgba(var(--m-edge),0.55); }
.kc .kc-fig .j.issued { background: var(--m-base); }
.kc .kc-title { font: 600 14px/1.2 ui-sans-serif, system-ui, -apple-system, 'Helvetica Neue', sans-serif; letter-spacing: 0.04em; color: var(--m-bright); }
.kc .kc-title.changed::after { content: ' •'; color: #f3d79a; }
.kc .kc-sub { font-size: 10px; letter-spacing: 0.12em; text-transform: uppercase; color: var(--m-dim); margin-top: 2px; }
.kc .kc-pouch { display: flex; flex-wrap: wrap; align-items: center; gap: 3px 6px; margin-top: 5px; font-size: 10px; letter-spacing: 0.1em; text-transform: uppercase; color: var(--m-soft); }
.kc .kc-pouch > * { white-space: nowrap; }
.kc .kc-pouch.off { display: none; }
.kc .kc-pips { display: flex; gap: 2px; flex: 0 0 100%; }
.kc .kc-pips i { display: block; width: 7px; height: 9px; border-radius: 1px; box-shadow: inset 0 0 0 1px rgba(var(--m-edge),0.6); }
.kc .kc-pips i.on { background: var(--m-accent-hi); box-shadow: none; }
.kc .kc-pips i.big { width: 19px; margin-left: 4px; }
.kc .kc-pouch b { font-weight: 600; color: var(--m-bright); letter-spacing: 0.04em; }
.kc .kc-load { display: flex; flex-direction: column; gap: 3px; flex: 0 0 100%; order: 10; margin-top: 2px; }
.kc .kc-bar { display: flex; align-items: center; justify-content: space-between; gap: 6px; }
.kc .kc-bar-t { position: relative; display: block; flex: 0 0 96px; height: 7px; box-shadow: inset 0 0 0 1px rgba(var(--m-edge),0.6); }
.kc .kc-bar-t i { position: absolute; top: 0; bottom: 0; display: block; }
.kc .kc-bar-t .fill { left: 0; background: rgba(var(--m-edge),0.9); }
.kc .kc-bar-t .gain { background: #8fc98a; }
.kc .kc-bar-t .gain.loss { background: #f0b76a; }
.kc .kc-bar-t .part { top: -1px; bottom: -1px; box-shadow: inset 0 0 0 1px var(--m-bright); }
.kc .kc-bar-t .part.more { box-shadow: none; border-left: 1px dashed var(--m-bright); border-bottom: 1px dashed var(--m-bright); border-top: 1px dashed var(--m-bright); }
.kc .kc-bar-t .ref { top: -3px; bottom: -3px; width: 2px; margin-left: -1px; background: var(--m-bright); }
.kc .kc-row { padding: 4px 6px 5px; margin: 0 -6px; border-left: 2px solid transparent; border-radius: 2px; cursor: default; }
.kc.focus .kc-row.sel, .kc.focus .kc-sets.sel { border-left-color: var(--m-accent-hi); background: rgba(var(--m-hover-rgb),0.45); }
.kc .kc-ico { display: none; }
.kc .kc-line { display: flex; align-items: center; gap: 4px; }
.kc .kc-label { flex: 1 1 auto; font-size: 10px; letter-spacing: 0.12em; text-transform: uppercase; color: var(--m-soft); white-space: nowrap; }
.kc .kc-val { flex: 0 1 auto; min-width: 74px; text-align: center; color: var(--m-bright); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.kc .kc-val.num { min-width: 44px; }
.kc .kc-row.changed .kc-val { color: #f3d79a; }
.kc .kc-step { flex: 0 0 auto; font: inherit; width: 20px; height: 20px; padding: 0; line-height: 18px; color: var(--m-btn-text); background: rgba(var(--m-btn-rgb),0.7); border: 1px solid rgba(var(--m-edge),0.35); border-radius: 3px; cursor: pointer; }
.kc .kc-step:hover { background: rgba(var(--m-hover-rgb),0.9); border-color: var(--m-accent-hi); }
.kc .kc-step.off { opacity: 0.32; }
.kc .kc-step.off:hover { background: rgba(var(--m-btn-rgb),0.7); border-color: rgba(var(--m-edge),0.35); }
.kc .kc-note { font-size: 10.5px; color: var(--m-dim); margin-top: 1px; min-height: 14px; }
.kc .kc-cost { color: var(--m-btn-text); }
.kc .kc-row.refused .kc-note, .kc .kc-words.refused { color: #f0b76a; }
.kc .kc-squad { color: var(--m-soft); }
.kc .kc-squad.full { color: #f3d79a; }
.kc .kc-set.off { opacity: 0.45; }
.kc .kc-sets { display: flex; gap: 4px; margin: 8px -6px 0; padding: 5px 6px; border-left: 2px solid transparent; border-radius: 2px; }
.kc .kc-set { flex: 1 1 0; font: inherit; font-size: 10px; letter-spacing: 0.08em; text-transform: uppercase; padding: 4px 2px; color: var(--m-btn-text); background: rgba(var(--m-btn-rgb),0.6); border: 1px solid rgba(var(--m-edge),0.3); border-radius: 2px; cursor: pointer; white-space: nowrap; }
.kc .kc-set:hover { border-color: var(--m-accent-hi); }
.kc.focus .kc-sets.sel .kc-set.pick { box-shadow: 0 0 0 1px var(--m-bright); }
.kc .kc-sets { flex-wrap: wrap; }
.kc .kc-clip { flex: 0 0 auto; font: inherit; font-size: 10px; letter-spacing: 0.08em; text-transform: uppercase; padding: 4px 8px; color: var(--m-btn-text); background: rgba(var(--m-btn-rgb),0.6); border: 1px solid rgba(var(--m-edge),0.3); border-radius: 2px; cursor: pointer; white-space: nowrap; }
.kc .kc-clip[data-clip="copy"] { margin-left: auto; }
.kc .kc-clip:hover { border-color: var(--m-accent-hi); }
.kc .kc-clip.on { color: var(--m-on-accent); background: var(--m-accent-hi); border-color: var(--m-accent-hi); }
.kc .kc-clip.off { opacity: 0.45; }
@keyframes kc-pasted { from { box-shadow: inset 0 0 0 99px rgba(var(--m-accent-rgb),0.34); } to { box-shadow: inset 0 0 0 99px rgba(var(--m-accent-rgb),0); } }
@keyframes kc-pasted2 { from { box-shadow: inset 0 0 0 99px rgba(var(--m-accent-rgb),0.34); } to { box-shadow: inset 0 0 0 99px rgba(var(--m-accent-rgb),0); } }
.kc .kc-row.pasted { animation: kc-pasted 0.8s ease-out; }
.kc .kc-row.pasted.pasted2 { animation-name: kc-pasted2; }   /* (the same flash under a second name: a paste straight after a paste plays again) */
@media (prefers-reduced-motion: reduce) { .kc .kc-row.pasted { animation: none; } }
.kc .kc-kids { margin: 0 0 3px 7px; padding-left: 5px; border-left: 1px solid rgba(var(--m-edge),0.3); }
.kc .kc-stats { display: flex; flex-direction: column; gap: 3px; margin-top: 4px; }
.kc .kc-stats.off { display: none; }
.kc .kc-stats .kc-stat { display: flex; align-items: center; justify-content: space-between; gap: 6px; font-size: 9.5px; letter-spacing: 0.1em; text-transform: uppercase; color: var(--m-soft); }
.kc .kc-stats .kc-bar-t { flex: 0 0 112px; height: 6px; }
.kc.kc-tree .kc-row.none .kc-val { color: var(--m-dim); }
.kc.kc-tree .kc-row.fixed .kc-step { visibility: hidden !important; }
.kc.kc-tree .kc-row.fixed .kc-val { color: var(--m-soft); }
.kc .kc-words { margin-top: 7px; padding-top: 6px; border-top: 1px solid rgba(255,255,255,0.06); font-size: 10.5px; color: var(--m-soft); min-height: 28px; }
`;
function style() {
  if (typeof document === 'undefined' || document.getElementById(STYLE_ID)) return;
  menuTheme(); const s = document.createElement('style'); s.id = STYLE_ID; s.textContent = CSS; document.head.appendChild(s);
}
const el = (tag: string, cls: string, text = ''): HTMLElement => { const e = document.createElement(tag); e.className = cls; if (text) e.textContent = text; return e; };

/** a mannequin tint (linear albedo) as a CSS colour, about as it reads under a lamp */
function tintCss(t: readonly number[]): string { const c = (x: number) => Math.round(255 * Math.pow(Math.min(1, Math.max(0, x * 1.6)), 1 / 2.2)); return `rgb(${c(t[0])},${c(t[1])},${c(t[2])})`; }
const LOAD_TIP = { mob: 'mobility (the notch shows his issued kit)', con: 'concealment (the notch shows his issued kit)' };
const POUCH_TIP = 'each pouch holds one grenade-sized thing, and the big slot holds one big thing like a shotgun, a hammer or a wall charge';
const UP = new Set(['KeyW', 'ArrowUp', 'w', 'W']), DOWN = new Set(['KeyS', 'ArrowDown', 's', 'S']), LEFT = new Set(['KeyA', 'ArrowLeft', 'a', 'A']), RIGHT = new Set(['KeyD', 'ArrowRight', 'd', 'D']);
const OK = new Set(['Enter', 'NumpadEnter', 'Space', ' ', 'Spacebar']), BACK = new Set(['Escape', 'Esc']);
/** by the key's PLACE on the board where the event says it (host/input.ts reads e.code: W A S D are where they are on any layout), else by its name */
const is = (set: Set<string>, e: KeyLike) => set.has(e.code || e.key);
/** events a card has already been handed (a host that calls key(e) AND lets the event reach the card's own listener: taken once) */
const seen = new WeakSet<object>();

/** Fill `host` with member i's card. */
export function memberCard(host: HTMLElement, i: number, opts: MemberCardOpts): MemberCard {
  style();
  denseStyle();
  /** what the desk last said of the card: asked again at each draw that can have changed it; a move of the highlight asks nothing */
  let view: CardView = desk().card(i);
  /** the gun whose pop-up is open */
  let popAt: GunSlot | null = null;
  const isGun = (f: string | null): f is GunSlot => f === 'primary' || f === 'secondary';
  const rowOf = (f: string | null): CardRow | null => view.rows.find(r => r.field === f) ?? null;
  /** the rows the keys walk as the card stands (a gun's things are in its pop-up — shut, the tiles as they lie, the kit before the colours; open, that gun and what rides on it) */
  const seq = (): string[] => (popAt ? view.rows.filter(r => r.field === popAt || r.under === popAt) : [...view.rows.filter(r => !r.under && !r.look), ...view.rows.filter(r => !r.under && r.look)]).map(r => r.field);
  const root = el('div', 'kc kc-tree kc-dense'); root.setAttribute('data-member', String(i)); root.tabIndex = 0;
  // the head: a little figure in his two colours, who he is
  const head = el('div', 'kc-head'), fig = el('div', 'kc-fig'), suitBits: HTMLElement[] = [], trimBits: HTMLElement[] = [];
  for (const c of ['a a1', 'a a2', 'l l1', 'l l2', 'b', 'h']) { const b = el('i', c); suitBits.push(b); fig.appendChild(b); }
  for (const c of ['j j1', 'j j2', 'j j3', 'j j4']) { const b = el('i', c); trimBits.push(b); fig.appendChild(b); }
  const who = el('div', 'kc-who'), title = el('div', 'kc-title', opts.name.title), sub = el('div', 'kc-sub', opts.name.sub);
  // … and his pouches: a pip each, the full ones lit, 'n / m'
  const pouch = el('div', 'kc-pouch'), pips = el('span', 'kc-pips'), pouchN = el('b', ''), squadN = el('span', 'kc-squad');
  squadN.title = 'what your teammates carry between them. One more on his card means one fewer on someone else\'s';
  pouch.append(el('span', '', 'Pouches'), pouchN, squadN, pips); pouch.title = POUCH_TIP;
  // … and what the card weighs: a bar each for mobility and concealment — its fill up to the issued card's notch in grey, what the card has
  // gained on the notch in green or lost under it in amber, and the highlighted row outlined: what it takes (drop it and the bar grows to there), or, on a
  // row that carries none, dashed, what one more would take
  const load = el('span', 'kc-load'), bars = (['mob', 'con'] as const).map(k => {
    const bar = el('span', 'kc-bar'), track = el('span', 'kc-bar-t'), fill = el('i', 'fill'), gain = el('i', 'gain'), part = el('i', 'part'), ref = el('i', 'ref');
    bar.setAttribute('data-load', k); bar.title = LOAD_TIP[k]; track.append(fill, gain, part, ref); bar.append(el('span', 'kc-bar-l', k === 'mob' ? 'Mobility' : 'Concealment'), track);
    load.appendChild(bar); return { k, bar, fill, gain, part, ref };
  });
  pouch.appendChild(load);
  who.append(title, sub, pouch); head.append(fig, who); root.appendChild(head);
  const gunsBox = el('div', 'kc-guns'), gearBox = el('div', 'kc-gear'), lookBox = el('div', 'kc-look');   // (the three boxes the tiles are laid in)
  root.append(gunsBox, gearBox, lookBox);
  // the rows
  /** the highlighted row's field (null: the sets) · the rows as last drawn, for what stands before one that has gone */
  let cur: string | null = view.rows[0]?.field ?? null, order: string[] = view.rows.map(r => r.field), focused = false, hung = '';
  /** the row whose last step was refused (null: the sets), and why (shown on it until the next press anywhere on the card) */
  let said: { k: string | null; why: string } | null = null;
  /** the rows the last paste moved: they flash once (cleared by the card's next change), under one of two names by turns so that the next paste plays again */
  let flash: ReadonlySet<string> = new Set(), flashes = 0;
  const changed = (moved?: ReadonlySet<string>) => { said = null; flash = moved ?? new Set(); if (moved) flashes++; draw(true); opts.onChange?.(i); };
  const step = (f: string, dir: 1 | -1) => { cur = f; const r = desk().step(i, f, dir); if (!r.ok) { said = { k: f, why: r.why }; draw(); } else { changed(); if (r.passed) { said = { k: f, why: `passed ${r.passed}` }; draw(); } } };
  /** a pop-up: one of a gun's choices pressed — that gun on the card at once, fitted and refused as a paste is, not stepped to */
  const choose = (f: string, id: string) => { cur = f; if (rowOf(f)?.value === id) { said = null; draw(); return; }
    const p = desk().choose(i, f, id); if (!p.ok) { said = { k: f, why: p.why }; draw(); } else changed(); };
  const shut = (e: Event) => { const t = e.target as Node | null, slot = popAt; if (slot && !(t && guns[slot]?.pop?.contains(t)) && !(t && els.get(slot)?.row.contains(t))) setPop(null); };   // (a press anywhere but on the open pop-up or its own tile)
  const setPop = (slot: GunSlot | null) => { if (slot === popAt) return; const D = root.ownerDocument as Document | undefined;
    if (slot) { if (!rowOf(slot)) return; cur = slot; }
    else if (rowOf(cur)?.under) cur = popAt;   // (the highlight comes out with it: onto its gun)
    popAt = slot; said = null; const W = D?.defaultView;
    if (slot) { D?.addEventListener?.('pointerdown', shut, true); D?.addEventListener?.('scroll', refit, true); W?.addEventListener?.('resize', refit); }
    else { D?.removeEventListener?.('pointerdown', shut, true); D?.removeEventListener?.('scroll', refit, true); W?.removeEventListener?.('resize', refit); } draw(); };   // (the page scrolled or the window sized under an open pop-up: fitted over the foot again)
  const refit = () => { const p = popAt && guns[popAt]?.pop; if (p) fit(p); };
  /** the reset pressed: taken, or refused with the reason under the rows (the squad has no room for it as the other cards stand) */
  const press = () => { cur = null; const r = desk().reset(i); if (r.ok) changed(); else { said = { k: null, why: `Reset: ${r.why}` }; draw(); } };
  const guns: Partial<Record<GunSlot, { kids: HTMLElement; pop: HTMLElement; why: HTMLElement }>> = {};
  type Stat = { bar: HTMLElement; fill: HTMLElement; gain: HTMLElement; ref: HTMLElement };
  type Pick = { id: string; b: HTMLElement; ico: HTMLElement; name: HTMLElement; bars: HTMLElement; fills: HTMLElement[]; drawn: string };
  type RowEls = { row: HTMLElement; val: HTMLElement; note: HTMLElement; prev: HTMLElement; next: HTMLElement; ico: HTMLElement; drawn: string; slot: GunSlot | null; stats: { box: HTMLElement; bars: Stat[] } | null; chips: HTMLElement | null; picks: Pick[] | null; chipsDrawn: string };
  /** each row's elements, by field: made the first time the card has the row */
  const els = new Map<string, RowEls>();
  const rowEls = (r: CardRow): RowEls => {
    const had = els.get(r.field); if (had) return had;
    const f = r.field, row = el('div', 'kc-row'); row.setAttribute('data-field', f);
    const line = el('div', 'kc-line'), label = el('span', 'kc-label', r.label), prev = el('button', 'kc-step', '‹'), val = el('span', 'kc-val'), next = el('button', 'kc-step', '›'), note = el('div', 'kc-note');
    if (r.kind === 'count') val.classList.add('num');   // (a count is a digit or a short word: its long label and both arrows then fit the card)
    prev.title = 'previous'; next.title = 'next'; prev.tabIndex = -1; next.tabIndex = -1;
    // (a press on an arrow is the arrow's alone: the row under it must not take the same click, or it would wipe what the arrow just said)
    prev.onclick = (ev) => { ev?.stopPropagation?.(); step(f, -1); }; next.onclick = (ev) => { ev?.stopPropagation?.(); step(f, 1); };
    row.onclick = () => { if (isGun(f)) { setPop(popAt === f ? null : f); return; }   // (a gun's tile opens its pop-up, and shuts it)
      if (cur !== f || said) { cur = f; said = null; draw(); } };
    const ico = el('div', 'kc-ico');   // (the thing's silhouette — ui/kitIcons.ts)
    line.append(label, prev, val, next); row.append(ico, line, note);
    // (a gun's tile wears four bars of what the gun is good at, and what rides on it as chips; its things are hung in its pop-up, since the light goes with the gun in his hands)
    const slot = isGun(f) ? f : null;
    let stats: RowEls['stats'] = null, chips: HTMLElement | null = null, picks: Pick[] | null = null;
    if (slot) { const box = el('div', 'kc-gun'), kids = el('div', 'kc-kids'), sb = el('div', 'kc-stats'); box.setAttribute('data-gun', slot);
      stats = { box: sb, bars: GUN_STATS.map(S => { const bar = el('span', 'kc-stat'), track = el('span', 'kc-bar-t'), fill = el('i', 'fill'), gain = el('i', 'gain'), ref = el('i', 'ref'); bar.setAttribute('data-stat', S.id); bar.title = S.tip; track.append(fill, gain, ref); bar.append(el('span', 'kc-bar-l', S.label), track); sb.appendChild(bar); return { bar, fill, gain, ref }; }) };
      row.appendChild(sb);
      chips = el('div', 'kc-chips'); row.appendChild(chips);
      const pop = el('div', 'kc-pop'), list = el('div', 'kc-picks'), key = el('span', 'kc-pick-key'); pop.setAttribute('data-pop', slot);
      for (const S of GUN_STATS) key.appendChild(el('span', '', S.label)); list.appendChild(key);
      picks = r.picks.map(p => { const b = el('button', 'kc-pick'), pico = el('span', 'kc-pick-ico'), name = el('span', 'kc-pick-name'), pb = el('span', 'kc-pick-bars'); b.setAttribute('data-pick', p.id); b.tabIndex = -1;
        const fills = GUN_STATS.map(S => { const t = el('span', 'kc-bar-t'), fl = el('i', 'fill'); t.setAttribute('data-stat', S.id); t.title = S.tip; t.appendChild(fl); pb.appendChild(t); return fl; });
        b.append(pico, name, pb); b.onclick = (ev) => { ev?.stopPropagation?.(); choose(f, p.id); }; list.appendChild(b); return { id: p.id, b, ico: pico, name, bars: pb, fills, drawn: '' }; });
      const why = el('div', 'kc-pop-note'); pop.append(list, kids, why); box.append(row, pop); gunsBox.appendChild(box); guns[slot] = { kids, pop, why }; }
    else if (!r.under) { if (r.look) lookBox.insertBefore(row, sets); else gearBox.appendChild(row); }
    const made: RowEls = { row, val, note, prev, next, ico, drawn: '', slot, stats, chips, picks, chipsDrawn: '' };
    els.set(f, made); return made;
  };
  // the sets
  const sets = el('div', 'kc-sets');
  // (not 'As issued' a second time: that is a kit of the squad's row)
  const reset = el('button', 'kc-set pick', 'Reset'); reset.setAttribute('data-set', 'issued'); reset.tabIndex = -1; reset.onclick = press; sets.appendChild(reset);
  // copy and paste: two more buttons at the end of the sets' line (not sets: the keys' pick never lands on them — theirs are C and V)
  const clipEls = (['copy', 'paste'] as const).map(id => { const b = el('button', 'kc-clip', id === 'copy' ? 'Copy' : 'Paste'); b.setAttribute('data-clip', id); b.tabIndex = -1; sets.appendChild(b); return b; });
  const copy = () => { desk().copy(i); said = null;
    for (const c of liveCards) if ((c.root as { isConnected?: boolean }).isConnected === false) liveCards.delete(c); else c.draw(true);   // (every card on the page: who wears it now)
    opts.onCopy?.(i); };
  const paste = (): boolean => { if (!view.copied) return false;
    const r: Pressed = desk().paste(i); if (!r.ok) { said = { k: null, why: `Paste: ${r.why}` }; draw(); return false; }
    if (r.changed.length) changed(new Set(r.changed)); return true; };
  clipEls[0].onclick = copy; clipEls[1].onclick = () => { paste(); };
  lookBox.appendChild(sets);
  const words = el('div', 'kc-words'); root.appendChild(words);
  const pct = (x: number) => `${Math.round(Math.min(1, Math.max(0, x)) * 1000) / 10}%`;

  let told: string | null = cur;   // (the row onSelect was last told of)
  function draw(fresh = false) {
    if (fresh) view = desk().card(i);
    for (const r of view.rows) rowEls(r);
    { const S = seq(); if (cur !== null && !S.includes(cur)) { const at = order.indexOf(cur); cur = order.slice(0, Math.max(0, at)).reverse().find(f => S.includes(f)) ?? S[0] ?? null; } }   // (a highlight left on a row the card no longer has goes to the row before it — before anybody is told of it)
    order = view.rows.map(r => r.field);
    if (cur !== told) { told = cur; opts.onSelect?.(i, cur); }
    {   // what hangs under which gun as the card stands
      const now = view.rows.filter(r => r.under).map(r => `${r.field}>${r.under}`).join();
      if (now !== hung) { hung = now;
        for (const [f, e] of els) if (!rowOf(f)) e.row.remove();   // (not on the card as it stands: not drawn, not hung anywhere)
        for (const slot of ['primary', 'secondary'] as const) { const kids = guns[slot]?.kids; if (!kids) continue; let any = false;
          for (const r of view.rows) if (r.under === slot) { kids.appendChild(rowEls(r).row); any = true; }
          kids.style.display = any ? '' : 'none'; } }
    }
    const suit = view.suit ? tintCss(view.suit) : null, trim = view.trim ? tintCss(view.trim) : null;   // (the figure's two colours: as issued is drawn as an outline — the scenario's own colours are the scenario's, the four's red and blue pairs, and not this card's to guess)
    for (const b of suitBits) { b.style.background = suit ?? ''; b.classList.toggle('issued', !suit); }
    for (const b of trimBits) { b.style.background = trim ?? ''; b.classList.toggle('issued', !trim); }
    title.classList.toggle('changed', view.changed);
    const c = view.pouches, big = view.big, sq = view.squad; pips.textContent = '';
    for (let n = 0; n < Math.max(c.total, c.used); n++) pips.appendChild(el('i', n < c.used ? 'on' : ''));
    for (let n = 0; n < Math.max(big.total, big.used); n++) pips.appendChild(el('i', n < big.used ? 'big on' : 'big'));   // (his one big slot, a wide pip after the pouches — lit when something rides in it)
    pouchN.textContent = `${c.used} / ${c.total}`;
    squadN.textContent = sq ? `squad ${sq.used} / ${sq.total}` : '';
    const lit = rowOf(cur), S = lit?.takes ?? null, M = lit?.would ?? null;
    for (const b of bars) { const v = view.load[b.k], r = view.loadIssued[b.k], lo = Math.min(v, r), s = S?.[b.k] ?? 0, more = M?.[b.k] ?? 0;
      b.fill.style.width = pct(lo); b.gain.style.left = pct(lo); b.gain.style.width = pct(Math.abs(v - r)); b.gain.classList.toggle('loss', v < r);
      b.part.classList.toggle('more', !s && more > 0); b.part.style.left = pct(s ? v : v - more); b.part.style.width = pct(s ? Math.min(s, 1 - v) : more); b.part.style.display = s || more ? '' : 'none';
      b.ref.style.left = pct(r); b.bar.setAttribute('data-score', v.toFixed(3)); b.bar.setAttribute('data-share', s.toFixed(3)); b.bar.setAttribute('data-more', more.toFixed(3)); }
    squadN.classList.toggle('full', !!sq && sq.used >= sq.total && c.used < c.total);   // (room in his own pouches and none in the squad's: the squad's count is why his arrows are dimmed, so it is the one lit)
    for (const r of view.rows) { const e = rowEls(r), no = said && said.k === r.field ? said.why : '', count = r.kind === 'count';
      e.row.classList.toggle('none', r.kind !== 'choice' && r.n === 0); e.row.classList.toggle('fixed', r.fixed);   // (a gun made with its suppressor, or one none fits: the tile has no arrows)
      if (e.stats) { e.stats.box.classList.toggle('off', !r.bars.length);   // (what this gun is good at, beside the gun he is issued: the notch)
        r.bars.forEach(({ now: a, issued: z }, n) => { const b = e.stats!.bars[n], lo = Math.min(a, z); b.fill.style.width = pct(lo); b.gain.style.left = pct(lo); b.gain.style.width = pct(Math.abs(a - z)); b.gain.classList.toggle('loss', a < z); b.ref.style.left = pct(z); b.bar.setAttribute('data-score', a.toFixed(3)); }); }
      e.val.textContent = r.idle ? (count ? `${r.n} · ${IDLE_WORD}` : IDLE_WORD) : r.word; e.note.textContent = '';   // (his own count stays in front where he carries some: an arrow pressed always moves what the tile reads)
      e.row.classList.toggle('idle', r.idle); e.row.classList.toggle('zero', (count ? r.n === 0 : r.none && (r.field === 'big' || r.field === 'wand' || r.field === 'primary'))); e.row.setAttribute('data-noise', String(r.noise));   // (none of the thing carried — a count of none, an empty big slot, no wand; never a fitting switched off —; how loud it is in use: both are drawn on a quiet mission's gear screen only)
      e.row.title = r.idle ? IDLE_WHY : '';
      { const key = `${r.field}:${isGun(r.field) || r.field === 'big' ? r.value : ''}${r.can ? ':can' : ''}`; if (e.drawn !== key) { e.drawn = key; e.ico.innerHTML = kitIcon(r.field, r.value, r.icon, r.can); } e.ico.classList.toggle('none', r.none); }
      if (no) e.note.textContent = no;
      else { if (r.cost) e.note.append(el('span', 'kc-cost', r.cost), ' · '); e.note.append(r.note); }
      e.row.classList.toggle('sel', cur === r.field); e.row.classList.toggle('changed', r.changed); e.row.classList.toggle('refused', !!no); e.row.classList.toggle('pasted', flash.has(r.field)); e.row.classList.toggle('pasted2', flash.has(r.field) && flashes % 2 === 0);
      // an arrow whose step would not fit his pouches is dimmed and says why (it can still be pressed: the reason goes on the row)
      for (const [b, why, word] of [[e.prev, r.less, 'previous'], [e.next, r.more, 'next']] as [HTMLElement, string, string][]) { b.classList.toggle('off', !!why); b.title = why || word; }
    }
    drawDense();
    sets.classList.toggle('sel', cur === null);
    reset.classList.toggle('off', !!view.resetWhy || view.issued); reset.title = view.resetWhy || 'go back to the issued kit';   // (dim while the card is as issued, as Paste is with nothing to change)
    { const C = view.copied, same = view.clip, why = view.pasteWhy, whose = i ? 'his' : 'your';   // (Copy lit: this card IS the copied kit; Paste dim: nothing copied, nothing it would change, or no room)
      clipEls[0].classList.toggle('on', same); clipEls[0].title = `copy ${whose} kit (C)`;
      clipEls[1].classList.toggle('off', !C || same || !!why); clipEls[1].title = why || (!C ? 'nothing copied yet' : same ? `${i ? 'he wears' : 'you wear'} the copied kit` : `paste the copied kit onto ${i ? 'him' : 'you'} and keep ${whose} colors (V)`); }
    const noSet = said && said.k === null ? said.why : said && !popAt && isGun(said.k) ? said.why : ''; words.classList.toggle('refused', !!noSet);   // (a gun's tile has no line of its own: a step it refused is said on the card's)
    words.textContent = noSet || view.words;
    root.classList.toggle('focus', focused);
  }
  /** each gun's tile wears what rides on it as chips (lit: fitted or carried, with a count; faint: not, or of no use on this
   *  mission), and the open pop-up draws the guns the tile could be — each with its drawing, its name and the tile's four bars, the one on the card lit,
   *  one the pouches have no room for dimmed */
  function drawDense() {
    for (const r of view.rows) { const e = rowEls(r); if (!e.slot || !e.chips) continue; const pop = guns[e.slot]?.pop, open = popAt === e.slot;
      pop?.classList.toggle('open', open); e.row.classList.toggle('open', open);
      // (a refusal while the pop-up is open — this gun's, or a set's or a paste's — is said in the pop-up, which lies over the card's own line)
      { const w = guns[e.slot]?.why, no = said && open && (said.k === r.field || said.k === null) ? said.why : ''; if (w) { w.textContent = no; w.classList.toggle('on', !!no); } }
      const bits = view.rows.filter(c => c.under === e.slot).map(c => ({ c, n: c.idle ? 0 : c.kind === 'count' ? c.n : c.n ? 1 : 0 }));
      const sig = bits.map(b => `${b.c.field}:${b.n}:${b.c.idle}`).join();
      if (sig !== e.chipsDrawn) { e.chipsDrawn = sig; e.chips.textContent = '';
        for (const b of bits) { const c = el('span', `kc-chip${b.n ? '' : ' off'}${b.c.idle ? ' idle' : ''}`); c.setAttribute('data-chip', b.c.field); c.title = `${b.c.label}: ${b.c.idle ? IDLE_WORD : b.c.word}`;
          c.innerHTML = kitIcon(b.c.field, b.c.value); if (b.c.kind === 'count' && b.n) c.appendChild(el('b', '', String(b.n))); e.chips.appendChild(c); } }
      if (!e.picks || !open) continue;
      for (const p of e.picks) { const P = r.picks.find(x => x.id === p.id); if (!P) continue; const key = `${P.id}:${P.can}`;
        p.b.classList.toggle('on', P.now); p.b.classList.toggle('off', !!P.why); p.b.title = P.why || P.note;
        if (p.drawn !== key) { p.drawn = key; p.ico.innerHTML = kitIcon(r.field, P.id, P.icon, P.can); } p.ico.classList.toggle('none', !P.icon);
        p.name.textContent = P.word; p.bars.classList.toggle('off', !P.bars.length);
        P.bars.forEach((x, n) => { p.fills[n].style.width = pct(x); p.fills[n].parentElement?.setAttribute('data-score', x.toFixed(3)); }); }
      if (pop) fit(pop); }
  }
  /** the open pop-up ends over the host's foot (MemberCardOpts.floor: Deploy is under it) — in a window too short for it under the tiles it is drawn
   *  that much higher, over the foot of the guns' tiles, whose guns it names itself. A page with no lay-out: nothing */
  function fit(pop: HTMLElement) {
    try { pop.style.marginTop = ''; const box = pop.getBoundingClientRect?.(); if (!box || !box.height) return;
      const floor = opts.floor?.() || (root.ownerDocument?.documentElement?.clientHeight ?? 0), over = box.bottom + 4 - floor;
      if (floor > 0 && over > 0) pop.style.marginTop = `${-Math.ceil(over * (pop.offsetHeight / box.height || 1))}px`;   // (a text size's zoom: the page's pixels, not the screen's)
    } catch { /* (no lay-out to ask) */ }
  }
  function key(e: KeyLike): boolean {
    if (seen.has(e)) return true;
    if (e.ctrlKey || e.metaKey || e.altKey) return false;
    { const clip = clipKey(e); if (clip) { seen.add(e); if (!e.repeat) { if (clip === 'copy') copy(); else paste(); } return true; } }
    const up = is(UP, e), down = is(DOWN, e), left = is(LEFT, e), right = is(RIGHT, e), ok = is(OK, e), back = is(BACK, e);
    if (!(up || down || left || right || ok || back)) return false;
    const S: (string | null)[] = popAt ? seq() : [...seq(), null], at = Math.max(0, S.indexOf(cur));   // (the rows the card has as it stands, then the sets — or, a pop-up open, what is in it)
    // (Enter on a gun's tile opens its pop-up; open, the keys are the pop-up's — W S walk the gun and its things and stop at the ends, A D change, Enter on the gun or Esc shuts it)
    if (popAt ? back || (ok && isGun(cur)) || (up && at === 0) || (down && at === S.length - 1) : ok && isGun(cur)) {
      seen.add(e); if (!e.repeat && (back || ok)) setPop(popAt ? null : cur as GunSlot); return true; }
    if (back) return false;
    if (up && at === 0 || down && at === S.length - 1) { if (said) { said = null; draw(); } return false; }   // (an edge: the host's to walk on from)
    seen.add(e);
    if (e.repeat) return true;   // (a key held down is one press: nothing here repeats)
    if (said) said = null;
    if (up) cur = S[at - 1]; else if (down) cur = S[at + 1];
    else if (cur !== null) { step(cur, left ? -1 : 1); return true; }
    else if (ok) { press(); return true; }
    draw(); return true;
  }
  const setFocus = (on: boolean, tell: boolean) => { if (focused === on) return; focused = on; draw(); if (on && tell) opts.onFocus?.(i); };
  root.onkeydown = e => { if (key(e)) { e.preventDefault(); e.stopPropagation(); } };
  root.onfocus = () => setFocus(true, true);
  root.onblur = e => { if (!(e.relatedTarget instanceof Node && root.contains(e.relatedTarget))) setFocus(false, false); };
  root.onmousedown = e => { if (e.target !== root) e.preventDefault(); card.focus(); };   // (a click on an arrow or a set presses it without taking the keys off the card)
  host.appendChild(root); draw();
  const me = { root, draw }; liveCards.add(me);
  const card: MemberCard = {
    el: root,
    focus() { setFocus(true, true); (root as { focus?: (o?: FocusOptions) => void }).focus?.({ preventScroll: true }); },
    blur() { said = null; setPop(null); setFocus(false, false); draw(); },
    at: () => (cur === null ? view.rows.length : view.rows.findIndex(r => r.field === cur)), field: () => cur, key,
    go(where) { setPop(null); cur = where === 'first' ? view.rows[0]?.field ?? null : where === 'sets' ? null : where; said = null; draw(); },
    pop: () => popAt, open: setPop,
    refresh() { said = null; draw(true); },
    copy, paste,
    destroy() { const D = root.ownerDocument as Document | undefined; D?.removeEventListener?.('pointerdown', shut, true); D?.removeEventListener?.('scroll', refit, true); D?.defaultView?.removeEventListener?.('resize', refit); liveCards.delete(me); host.innerHTML = ''; },
  };
  return card;
}
