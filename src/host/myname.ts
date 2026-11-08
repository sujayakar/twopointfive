// THE NAME THIS BROWSER'S PLAYER PICKED: typed on the title's Co-op page, kept for the next visit. '' = none, he is his seat's number. And what a page writes for a player, `shownAs`.
import { cleanName } from '../net/wire';

const LS = 'twopointsix.name';
let mem: string | null = null;   // (a browser with no store: the name lasts the page)
export function myName(): string { if (mem !== null) return mem; try { return cleanName(localStorage.getItem(LS)); } catch { return ''; } }
/** …as he types it: cleaned, kept, and handed back as kept */
export function setMyName(raw: string): string { const n = cleanName(raw); try { if (n) localStorage.setItem(LS, n); else localStorage.removeItem(LS); mem = null; } catch { mem = n; } return n; }

/** what a page writes for the man the game calls `was` ('Player 2': his seat counted from one), by the names the game's server last said, one a seat: his picked name, or `was`. A picked name that would not
 *  tell the players apart is not used: one another seat picked too, or one that reads as a seat's own number */
export function shownAs(was: string, names: readonly string[]): string {
  const m = /^Player(?: (\d+))?$/.exec(was), seat = m ? m[1] ? +m[1] - 1 : names.length === 1 ? 0 : -1 : -1, mine = names[seat] ?? '', fold = (t: string): string => t.toLowerCase();
  return !mine || /^player( \d+)?$/i.test(mine) || names.some((n, i) => i !== seat && fold(n) === fold(mine)) ? was : mine;
}
