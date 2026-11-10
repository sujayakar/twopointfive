// THE BOX WORLD, ASKED of a game the page does not hold (src/view/worldask.ts): the module answers, of the game shared.
import { heardOf, wayOf, type Heard, type WorldAsk } from '../view/worldask';
import type { Vec3 } from '../math/vec';
import type { DrawGame } from '../host/drawgame';

export function askOf(game: DrawGame): WorldAsk {
  const X = game.X, doubles = (n: number): Float64Array => { const p = X.face_out_ptr() >>> 0; return new Float64Array(game.memory.buffer.slice(p, p + n * 8)); };
  return {
    blocked: (a, b) => X.face_line(game.h, 0, a[0], a[1], a[2], b[0], b[1], b[2]) !== 0,
    solid: (a, b) => X.face_line(game.h, 1, a[0], a[1], a[2], b[0], b[1], b[2]) !== 0,
    ray: (o, d, max): { t: number; n: Vec3 } | null => { if (!X.face_ray(game.h, o[0], o[1], o[2], d[0], d[1], d[2], max)) return null; const r = doubles(5); return { t: r[0], n: [r[1], r[2], r[3]] }; },
    hear: (from, to): Heard => { X.face_hear(game.h, from[0], from[1], from[2], to[0], to[1], to[2]); return heardOf(doubles(10)); },
    way: (a, b, maxIter) => { const n = X.face_way(game.h, a[0], a[2], b[0], b[2], maxIter); return n < 0 ? null : wayOf(n ? doubles(n * 2) : []); },
    light: (p, owners, steady, leave): number => { const n = owners.length * 8, q = n ? X.face_alloc(n) >>> 0 : 0; if (n) new Float64Array(game.memory.buffer, q, owners.length).set(owners); try { return X.face_light(game.h, p[0], p[1], p[2], steady ? 1 : 0, leave, q, n); } finally { if (n) X.face_free(q, n); } },
  };
}
