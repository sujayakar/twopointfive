// THE NAME A PLAYER PICKS (the maintainer, 2026-11-03, from his first game of two: "at least have numbers for players or maybe you can pick your name when
// you enter multiplayer (and we save it in a cookie)"). The numbers are game/seats.ts playerName ('Player 1', 'Player 2'); this is the name over them.
//   mine      what this browser's player typed on the title's Co-op page (ui/coop.ts), kept for the next visit (localStorage `twopointsix.name`, the page's
//             way of keeping a thing; one of the browser's settings, so where a build keeps a browser's settings with its backend — cloud/prefs.ts — it
//             goes there with them, and a seat's hello carries it to the game's server). '' = none: he is his seat's number
//   theirs    a networked page says its name in its hello, the server hands every client the seats' names with its state (net/wire.ts Hello.name,
//             State.names; cleaned at both ends, `cleanName`), and the page keeps them here by the game (`setSeatNames`)
//   shown     `shownName(g, id, was)`: what to call the man with that id where a page writes a player's name — the debrief's rows (ui/verdictCard.ts),
//             the death recap (ui/recap.ts) — his picked name, else what the game itself calls him (myname.ts shownAs, which the page that draws asks by the
//             game's own word: it holds no seats)
// NOT THE GAME'S STATE, on purpose: a name reaches each machine at its own moment, and the game's own names ('Player 1' in the verdict's tracks, the
// recap's cast) are what every machine holds to the bit. Nothing a tick does reads this file; a page with no names reads as it did.
import type { Game } from '../game/game';
import { rawGame, seatOfId, playerName } from '../game/seats';
import { shownAs } from './myname';

export { myName, setMyName } from './myname';

const NAMES = new WeakMap<object, readonly string[]>();
/** the seats' names as the server last said them, one a seat ('' = that player picked none), cleaned by whoever heard them (net/client.ts) */
export function setSeatNames(g: Game, names: readonly string[]): void { NAMES.set(rawGame(g), names); }
/** what a page calls the man with this id: a player's picked name, else his number (seats.ts playerName); for anybody else `was`, the game's own word */
export function shownName(g: Game, id: number, was = ''): string {
  const s = seatOfId(g, id);
  return s ? shownAs(playerName(g, s.view.player), NAMES.get(rawGame(g)) ?? []) : was;
}
