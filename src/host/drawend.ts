// A CARD OF A RUN, on the page that draws a game it does not hold (view/runend.ts RunEnd): asked of the module as the card opens.
import { GEAR, gearText } from './gear';
import type { RunBooks } from '../ui/runBooks';
import type { RunEnd } from '../view/runend';
import type { DrawGame } from './drawgame';
import { shownAs } from './myname';

/** `names`: the players' picked names, one a seat (a networked game's) */
export function drawEnd(game: DrawGame, books: RunBooks, names: readonly string[] = []): RunEnd {
  const E = game.end(), F = E.floor, at = (x: number, z: number): number => F.rows.charCodeAt(z * F.w + x) - 48;
  GEAR.on = true;
  return {
    view: game.debrief(), verdict: E.verdict, kept: books.note, down: game.view().kit.down, par: E.par ? { ...E.par, shown: books.best ? `${E.par.line} · ${books.best}` : E.par.line } : null, holder: E.holder, marks: E.marks,
    floor: { w: F.w, h: F.h, cell: F.cell, blocked: (x, z) => (at(x, z) & 2) !== 0, indoors: (x, z) => (at(x, z) & 1) !== 0 },
    map: () => game.floor(), name: (_id, was) => shownAs(was, names), kit: gearText,
  };
}
