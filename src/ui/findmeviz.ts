// THE OUTLINE (host/findme.ts FINDME.outline) as the page that holds a game fills it: not drawn here. findMeOutline() fills the renderer's frame for it (render/outline.ts: a
// rim round the pixels that are his, from the g-buffer's id target), with where he stands, and the same for each teammate. The page that draws the Rust game fills
// the same frame from its rows (draw/rims.ts). The ring, the setting's other half, is ui/findring.ts's.
// PICTURE ONLY: it reads where the men stand and writes nothing of the game's, and with the setting off it returns before it has asked anything. His shape
// through a wall is SHELVED until the outline has been seen on a GPU: it needs him drawn a second time without the walls.
import type { Game } from '../game/game';
import { men, teamMen, seatOfMan, seatTone } from '../game/seats';
import type { Vec3 } from '../math/vec';
import { FINDME, outlineInk, OUTLINE_LOOK, type OutlineHue } from '../host/findme';
import { makeOutlineMan, type OutlineFrame, type OutlineMan } from '../render/outline';

/** one man's part of the outline's frame: where he stands, how wide and in what ink */
function outlineMan(M: OutlineMan, pos: Vec3, times: number, hue: OutlineHue) {
  const L = OUTLINE_LOOK;
  M.on = true; M.at[0] = pos[0]; M.at[1] = pos[1]; M.at[2] = pos[2]; M.reach = L.reachM; M.top = L.topM; M.times = times; outlineInk(M.ink, hue);
}
/** the outline's frame for this frame (render/outline.ts), written in place: off unless the row is on, one of the players is up (a game of two: a player down keeps his own rim, as he has since the rim went by the picture) and `shown` (the page says no where no mark of his is drawn: a stun's white-out, attract mode, the kit's picture); else his own man — whose pixels,
 *  where, how wide, in what ink — and with him EVERY OTHER LIVING MAN OF THE TEAM (seats.ts: the other players' own men who are up, then
 *  `teamMen`, the roster of every seat, whatever an orders menu is doing), each rim covering OUTLINE_LOOK.mates of
 *  what the player's does. THE COLOURS SAY WHOSE MAN HE IS (game/seats.ts seatTone): a player's own man in his seat's (yellow,
 *  the second player's purple), a teammate in his pair's (red for the red pair; blue for the blue pair, a duo's partner and a squad not in pairs) —
 *  so the same man is the same colour on both players' screens. A man who has left the list gives his place up. */
export function findMeOutline(F: OutlineFrame, game: Game, shown = true) {
  const pl = game.player, pool = F.mates;
  if (!FINDME.outline || game.recap.allDown || !shown) { F.on = false; for (const m of pool) { m.on = false; m.owner = 0; } return; }
  F.whole = Math.max(0, Math.min(1, 1 - pl.nvAmount));   // (only the goggles take the rims out)
  F.owner = pl.char.id; outlineMan(F, pl.char.pos, 1, seatTone(seatOfMan(game, pl).index));
  for (const m of pool) m.on = false;
  const mate = (id: number, pos: Vec3, hue: OutlineHue): void => {
    let m: OutlineMan | undefined, free: OutlineMan | undefined;
    for (const x of pool) { if (x.owner === id) { m = x; break; } if (!x.owner) free ??= x; }
    if (!m) { m = free; if (!m) pool.push(m = makeOutlineMan()); m.owner = id; }
    outlineMan(m, pos, OUTLINE_LOOK.mates, hue);
  };
  for (const p of men(game)) if (p !== pl && !p.down) mate(p.char.id, p.char.pos, seatTone(seatOfMan(game, p).index));
  for (const gd of teamMen(game)) mate(gd.char.id, gd.char.pos, gd.unit?.pair === 'red' ? 'red' : 'blue');
  for (const m of pool) if (!m.on) m.owner = 0;
}
