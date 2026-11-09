// WHAT TAB SHOWS: a card over a paused game with the objective in full and the squad — each teammate, what he is doing and what
// he carries, then what the squad carries between them. The maintainer, 10-23: "if you don't have a menu open, tab opens up an
// objectives modal that also has your squad equipment?". It is the home of what the corner panel no longer says (ui/corner.ts: the line per man, loud
// or quiet, the counts). Every word on it is one the game already makes (the objective's line, the view's squad and each man's card); this file lays them
// out. No page under it: `tabCard` is the card as data, `tabCardOf` the card of a view as it stands, `tabCardHtml` its
// markup. The card is called from the page's Tab key only (host/drawhud.ts), and `objectiveWords` from the HUD's row as well: nothing here runs at module load.
import { objectiveParts } from './corner';
import { capsPlain, keyText } from '../host/keymap';
import { locked, type TickView } from '../view/seatview';

export interface TabMan { name: string; doing: string; kit: string; down: boolean }
export interface TabCard {
  /** the objective line's own word ('objective', 'training 2/7'; '' at a run's end) · what stands in front of it (a par, a clock) */
  label: string; note: string;
  /** the objective: its sentence, and under it the things it lists, each done, to do, or the one being called now */
  goal: string; items: { text: string; done: boolean; now: boolean }[];
  men: TabMan[];
  /** the squad as the block's title has it: loud or quiet, who the next order goes to ('loud · orders to GOLD'; '' = not said) */
  who: string;
  /** what the squad carries between them and what waits on him, as the block's notes word it ('charges 2 · bangs 3') */
  carry: string[];
}
/** An objective that lists ('clear the structure: ✓ the guard hut · [the store] · the workshop' — rust/game_rules/src/roomclear: a tick is a room taken, brackets the one
 *  being called) as its sentence and its list; any other as its sentence alone. */
export function objectiveList(text: string): { goal: string; items: TabCard['items'] } {
  const cut = text.indexOf(': '), parts = cut < 0 ? [] : text.slice(cut + 2).split(' · ').map(x => x.trim()).filter(Boolean);
  if (parts.length < 2) return { goal: text, items: [] };
  return { goal: text.slice(0, cut), items: parts.map(p => { const done = p.startsWith('✓'), t = done ? p.slice(1).trim() : p, now = /^\[.*\]$/.test(t); return { text: now ? t.slice(1, -1) : t, done, now }; }) };
}
/** the card: `objective` the mission's line ('' = none) · `men` the squad in its order · `who` the block's title past its first word · `carry` the
 *  block's plain notes. Only a line under the word 'objective' is read for a list (a lesson's step or a run's end is its sentence). */
export function tabCard(objective: string, men: TabMan[], carry: string[], who = ''): TabCard {
  const O = objectiveParts(objective), L = O?.label === 'objective' ? objectiveList(O.text) : { goal: O?.text ?? '', items: [] };
  return { label: O?.label ?? '', note: O?.note ?? '', goal: L.goal, items: L.items, men, who, carry };
}
/** a kit line with its counted things said as what he set out with: the squad's throwables and charges are one pool once the run is on (rust/game_rules/src/doortools),
 *  and its count is the note under the men, so a man's own count is what his card gave him, not what he has left */
export const staged = (kit: string): string => kit.split(' · ').map(p => (/^\d+ stun\b/.test(p) ? `started with ${p}` : p)).join(' · ');
/** the mission's line in the player's keys */
export const objectiveWords = (view: TickView): string => { const m = view.mission; return m?.of === 'lesson' ? capsPlain(m.text) : keyText(m?.text ?? ''); };
/** the card of a view as it stands: the squad in its order, each man's line as the view words it (what he is doing and what he covers) and the card he was staged with in a line
 *  (a mission that issues its own kit stages its own cards); the block's title and its plain notes (view.squad; no notes where a lesson's frame keeps
 *  them back). Every key named is named as the page has it (keymap.ts keyText). */
export function tabCardOf(view: TickView): TabCard {
  const V = view.squad;
  const men = (V?.mates ?? []).map(M => ({ name: M.name, doing: keyText(M.doing), kit: M.kit ? staged(M.kit) : '', down: M.down }));
  const carry = V && !locked(view, 'menu') ? V.notes.filter(o => !o.key).map(o => keyText(o.text)) : [];
  return tabCard(objectiveWords(view), men, carry, V && !locked(view, 'who') ? `${V.posture ? `${V.posture} · ` : ''}orders to ${V.who}` : '');
}
const esc = (s: string): string => s.replace(/[&<>"]/g, c => (c === '&' ? '&amp;' : c === '<' ? '&lt;' : c === '>' ? '&gt;' : '&quot;'));
/** the card's markup (index.html #tabcard): a section for the objective and one for the squad, each only while it has something to say; a line with no
 *  word of its own (a run's end) stands under no heading */
export function tabCardHtml(C: TabCard): string {
  const sec = (head: string, note: string, body: string) => `<section>${head || note ? `<h4>${esc(head)}${note ? `<em>${esc(note)}</em>` : ''}</h4>` : ''}${body}</section>`;
  const obj = C.goal ? sec(C.label, C.note, `<p>${esc(C.goal)}</p>${C.items.length ? `<ul>${C.items.map(i => `<li class="${i.done ? 'done' : i.now ? 'now' : ''}">${esc(i.text)}</li>`).join('')}</ul>` : ''}`) : '';
  const squad = C.men.length ? sec('squad', C.who, C.men.map(m => `<div class="man${m.down ? ' down' : ''}"><b>${esc(m.name)}</b><span>${esc(m.doing)}</span>${m.kit ? `<i>${esc(m.kit)}</i>` : ''}</div>`).join('') + C.carry.map(c => `<p class="carry">${esc(c)}</p>`).join('')) : '';
  return `${obj}${squad}<footer>Press <kbd>Tab</kbd> to go back to the game.</footer>`;
}
