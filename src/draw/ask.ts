// THE BOX WORLD, ASKED of a game the page does not hold (src/view/worldask.ts): the module answers, of the game shared.
import type { WorldAsk } from '../view/worldask';
import type { DrawGame } from '../host/drawgame';

export function askOf(game: DrawGame): WorldAsk {
  const F = game.face;
  return {
    blocked: (a, b) => F.sight(game.h, a, b),
    solid: (a, b) => F.solid(game.h, a, b),
    ray: (o, d, max) => F.ray(game.h, o, d, max),
    hear: (from, to) => F.hear(game.h, from, to),
    way: (a, b, maxIter) => F.way(game.h, a, b, maxIter),
    light: (p, owners, steady, leave) => F.light(game.h, p, owners, steady, leave),
  };
}
