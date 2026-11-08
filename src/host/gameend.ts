// A CARD OF A RUN, on the page that holds its game (view/runend.ts RunEnd). It goes with that page: the marks and their words are rust/game_face end.rs's too.
import type { Game } from '../game/game';
import { gearText } from './gear';
import { mapOf } from '../game/replaybuf';
import { indoors } from '../game/rooms';
import type { RunEnd, RunMark } from '../view/runend';
import { shownName } from './names';

export function runEndOf(game: Game): RunEnd {
  const P = game.par?.result?.() ?? null, Q = game.view.quiet, nav = game.col.nav, M = game.level.mission, marks: RunMark[] = [], mark = (glyph: string, cls: string, x: number, z: number, title: string): void => { marks.push({ glyph, cls, x, z, title }); };
  { const at = game.roomClear?.startAt ?? game.level.playerSpawn; mark('S', 'spawn', at[0], at[2], 'insertion point'); }   // (a room clear stands Sam on its team start — and on a level with two ways in, on the one this visit took)
  mark('◎', 'rack', M.rack.front[0], M.rack.front[2], Q ? `${Q.holder} — ${Q.thing}` : `rack ${M.rack.index + 1} — the drive`);
  if (Q) { for (const X of game.quiet.exits) { const ex = game.doors.byName(X.door)?.frameCentre ?? [X.x, 0, X.z]; mark('⇥', 'exit', ex[0], ex[2], X.name); } }
  else { const ex = game.doors.byName(M.exfilDoor)?.frameCentre; if (ex) mark('⇥', 'exit', ex[0], ex[2], 'the fire exit'); }
  const p: [number, number, number] = [0, 0, 0];
  return {
    view: game.debriefView(), verdict: game.verdict?.result ?? null, kept: game.verdict?.kept ?? null, down: !!game.player.down, par: P ? { medal: P.medal, line: P.line, shown: game.par.shown ?? P.line } : null, quiet: Q ? { holder: Q.holder } : null, marks,
    floor: { w: nav.W, h: nav.H, cell: nav.cell, blocked: (x, z) => nav.blocked[z * nav.W + x] === 1, indoors: (x, z) => { p[0] = (x + 0.5) * nav.cell; p[2] = (z + 0.5) * nav.cell; return indoors(p); } },
    map: () => mapOf(game), name: (id, was) => shownName(game, id, was), kit: gearText,
  };
}
