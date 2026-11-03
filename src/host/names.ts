// THE NAME A PLAYER PICKS (the maintainer, 2026-11-03, from his first game of two: "at least have numbers for players or maybe you can pick your name when
// you enter multiplayer (and we save it in a cookie)"). The numbers are game/seats.ts playerName ('Player 1', 'Player 2'); this is the name over them.
//   mine      what this browser's player typed on the title's Co-op page (ui/coop.ts), kept for the next visit (localStorage `twopointsix.name`, the page's
//             way of keeping a thing; one of the browser's settings, so where a build keeps a browser's settings with its backend — cloud/prefs.ts — it
//             goes there with them, and a seat's hello carries it to the game's server). '' = none: he is his seat's number
//   theirs    a networked page says its name in its hello, the server hands every client the seats' names with its state (net/wire.ts Hello.name,
//             State.names; cleaned at both ends, `cleanName`), and the page keeps them here by the game (`setSeatNames`)
//   shown     `shownName(g, id, was)`: what to call the man with that id where a page writes a player's name — the debrief's rows (ui/verdictCard.ts),
//             the death recap (ui/recap.ts) — his picked name, else what the game itself calls him. A picked name that would not tell the players
//             apart is not used: one another seat picked too, or one that reads as a seat's own number
// NOT THE GAME'S STATE, on purpose: a name reaches each machine at its own moment, and the game's own names ('Player 1' in the verdict's tracks, the
// recap's cast) are what every machine holds to the bit. Nothing a tick does reads this file; a page with no names reads as it did.
import type { Game } from '../game/game';
import { rawGame, seatOfId, playerName } from '../game/seats';
import { cleanName } from '../net/wire';

const LS = 'twopointsix.name';
let mem: string | null = null;   // (a browser with no store: the name lasts the page)
/** this browser's player's name ('' = none) */
export function myName(): string { if (mem !== null) return mem; try { return cleanName(localStorage.getItem(LS)); } catch { return ''; } }
/** …as he types it: cleaned, kept, and handed back as kept */
export function setMyName(raw: string): string { const n = cleanName(raw); try { if (n) localStorage.setItem(LS, n); else localStorage.removeItem(LS); mem = null; } catch { mem = n; } return n; }

const NAMES = new WeakMap<object, readonly string[]>();
/** the seats' names as the server last said them, one a seat ('' = that player picked none), cleaned by whoever heard them (net/client.ts) */
export function setSeatNames(g: Game, names: readonly string[]): void { NAMES.set(rawGame(g), names); }
/** what a page calls the man with this id: a player's picked name, else his number (seats.ts playerName); for anybody else `was`, the game's own word */
export function shownName(g: Game, id: number, was = ''): string {
  const s = seatOfId(g, id); if (!s) return was;
  const all = NAMES.get(rawGame(g)), mine = all?.[s.index] ?? '', own = playerName(g, s.view.player), fold = (t: string) => t.toLowerCase();
  if (!mine || /^player( \d+)?$/i.test(mine) || all!.some((n, i) => i !== s.index && fold(n) === fold(mine))) return own;
  return mine;
}
