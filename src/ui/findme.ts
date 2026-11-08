// FINDING YOUR OWN MAN ON THE SCREEN (and, by the outline, your teammates): what the page draws for host/findme.ts's two picture settings, and the rows Options > Game gets
// for the table.
//   ring     the gold ring an open order menu puts at his feet (ui/planfloor.ts `py`), drawn here whenever that one is not up
//   outline  not drawn here: findMeOutline() fills the renderer's frame for it (render/outline.ts — a rim round the pixels that are his, from the g-buffer's
//            id target), with where he stands, and the same for each teammate
// PICTURE ONLY: both read where the men stand and write nothing of the game's, and with a setting off its half returns before it has asked
// anything.
// THE FIGURE THAT WAS HERE (until 2026-10-20): the outline and his shape through a wall were drawn in this overlay from his joints, each limb a stroke, and
// that is not his silhouette — from a squad level's distance an arm was two pixels wide and the 'outline' a stick figure's. The maintainer: "it should be a
// proper silhouette". The outline is the renderer's now. His shape through a wall is SHELVED until that one has been seen on a GPU: it needs him drawn a
// second time without the walls, which is a pass of its own.
import type { Game } from '../game/game';
import { men, teamMen, seatOfMan, seatTone } from '../game/seats';
import type { FollowCamera } from '../game/camera';
import type { Vec3 } from '../math/vec';
import { PLAN } from '../game/floorplan';
import { camCap } from '../host/keymap';
import { FINDME, setFindMe, resetFindMe, outlineInk, OUTLINE_LOOK, type OutlineHue } from '../host/findme';
import { makeOutlineMan, type OutlineFrame, type OutlineMan } from '../render/outline';

/** where the ring would be drawn for the seat's own man (its centre on the floor), null = none: the row off, or he is down */
export function findMeRing(game: Game): Vec3 | null {
  const pl = game.player; if (pl.down || !FINDME.ring) return null;
  return [pl.char.pos[0], 0, pl.char.pos[2]];
}

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

const NS = 'http://www.w3.org/2000/svg';
const CSS = `
.findme { pointer-events: none; }
.findme .fr { fill: none; stroke: #f2cf6b; stroke-width: 1.5; stroke-linejoin: round; }
`;

export class FindMeViz {
  private g: SVGGElement; private ring: SVGPolygonElement; private shown = false; private points = '';
  constructor(svg: SVGSVGElement) {
    if (!document.getElementById('findme-css')) { const s = document.createElement('style'); s.id = 'findme-css'; s.textContent = CSS; document.head.appendChild(s); }
    this.g = document.createElementNS(NS, 'g') as SVGGElement; this.g.setAttribute('class', 'findme'); this.g.style.display = 'none';
    svg.insertBefore(this.g, svg.firstChild?.nextSibling ?? null);   // (over the squad's plan on the floor, under everything else the overlay draws)
    this.ring = document.createElementNS(NS, 'polygon') as SVGPolygonElement; this.ring.setAttribute('class', 'fr'); this.g.appendChild(this.ring);
  }
  private show(on: boolean) { if (on !== this.shown) { this.shown = on; this.g.style.display = on ? 'block' : 'none'; } }
  /** `planRing`: the order menu's own gold ring is drawn this frame (planfloor.ts `youOn`): ours stands down under it */
  update(game: Game, cam: FollowCamera, w: number, h: number, planRing = false) {
    const bc = document.body.classList;
    if (!FINDME.ring || planRing || bc.contains('blinded') || bc.contains('attract')) { this.show(false); return; }   // (nothing asked; and under a stun's white-out or attract mode as the plan on the floor)
    const at = findMeRing(game); if (!at) { this.show(false); return; }
    const r = PLAN.ringM * 1.35, N = 48, out: string[] = [];
    for (let i = 0; i < N; i++) { const a = (i / N) * Math.PI * 2, [x, y, v] = cam.project([at[0] + Math.sin(a) * r, 0.04, at[2] + Math.cos(a) * r], w, h, Infinity, 0.5); if (!v) { this.show(false); return; } out.push(`${x.toFixed(1)},${y.toFixed(1)}`); }
    const pts = out.join(' '); if (pts !== this.points) { this.points = pts; this.ring.setAttribute('points', pts); }   // (written only when it moved: a man standing still under a still camera costs the page nothing)
    this.show(true);
  }
}

// ---- the rows (ui/options.ts, GAME tab): the screen's own row makers handed in, so the rows look and key as every other row there
export interface RowKit {
  sec(parent: HTMLElement, text: string): unknown;
  flip(parent: HTMLElement, label: string, get: () => boolean, set: (v: boolean) => void, describe: string, words?: [string, string]): void;
  seg<T extends string>(parent: HTMLElement, label: string, opts: { v: T; label: string; line?: string }[], get: () => T, set: (v: T) => void, describe: () => string): void;
  range(parent: HTMLElement, label: string, min: number, max: number, step: number, get: () => number, set: (v: number) => void, fmt: (v: number) => string, describe: string): void;
}
const TURNS: { v: 'as' | 'rev'; label: string }[] = [{ v: 'as', label: 'Normal' }, { v: 'rev', label: 'Reversed' }];
/** the rows under their headings; returns what Reset does for them */
export function findMeRows(K: RowKit, body: HTMLElement): () => void {
  K.sec(body, 'Finding yourself · trying out');
  K.flip(body, 'Ring at your feet', () => FINDME.ring, v => setFindMe('ring', v), 'Keep the gold ring from the orders menu at your feet all the time.', ['On', 'Off']);
  K.flip(body, 'Outline in the dark', () => FINDME.outline, v => setFindMe('outline', v), 'Draw a rim around anyone on your team, including you, who\'s standing in the dark.', ['On', 'Off']);
  K.sec(body, 'Camera · trying out');
  K.range(body, 'Aim pan', 0, 100, 10, () => Math.round(FINDME.pan * 100), v => setFindMe('pan', v / 100), v => !v ? 'None' : `${v.toFixed(0)} %`, 'How far the view leans toward where you aim. At none you stay in the middle of the screen.');
  K.seg(body, 'Turn direction', TURNS, () => FINDME.turn < 0 ? 'rev' : 'as', v => setFindMe('turn', v === 'rev' ? -1 : 1), () => `Which way the view turns when you hold ${camCap()} and move the mouse.`);
  return resetFindMe;
}
