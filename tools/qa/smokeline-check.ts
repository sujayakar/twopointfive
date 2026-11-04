// A round through smoke is ONE LINE (src/smoke/smoke.ts `round` and `push` with `line`; smoke.wgsl emR2, emitter mode 3; game/combat.ts asks on the look page
// only). It used to be a row of soft shoves 7 cm across for one step, which moved nothing anyone could see. No GPU here: the cut of a line to a lattice's smoke is
// run as it stands, the solver's distance is mirrored as numbers and its text is read. What the smoke then DOES is never seen here.
//   bun run tools/qa/smokeline-check.ts   (exit 0 = all pass; SMOKELINE_BREAK=1 measures a line as a point, sends a leg uncut and wakes bricks by its middle: must fail)
import '../../src/host/gpuEnums';   // (first: the solver's module reads GPU enums as it loads)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { SmokeSystem } from '../../src/smoke/smoke';
import type { SmokePush } from '../../src/smoke/types';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.SMOKELINE_BREAK === '1', ROOT = resolve(import.meta.dir, '../..');
const read = (p: string) => readFileSync(resolve(ROOT, p), 'utf8');
const wgsl = read('src/smoke/smoke.wgsl'), ts = read('src/smoke/smoke.ts'), combat = read('src/game/combat.ts');
type V = [number, number, number];
type Sent = SmokePush & { level: number };
/** SmokeSystem.push and round as they stand, over a system that is only its lists (no device): `boxes` = the loose bounds of the smoke there is, a lattice each */
const system = (boxes: { lo: V; hi: V; level: number }[], wake = { speed: 9, radius: 0.12, push: 4 }) => {
  const P = SmokeSystem.prototype as unknown as Record<string, (...a: unknown[]) => unknown>;
  const me = { params: { enabled: true, frozen: false }, pushes: [] as Sent[], boxes, wake, levelOf: P.levelOf, inBox: P.inBox, push: P.push, round: P.round };
  if (BREAK) me.push = function (this: typeof me, p: unknown) { this.pushes.push({ ...(p as SmokePush), level: 0 }); };   // (the leg as combat gave it, uncut)
  return me as unknown as { pushes: Sent[]; push(p: SmokePush): void; round(from: V, dir: V, len: number): boolean };
};
/** smoke.wgsl emR2: how far a cell is from an emitter, in its radii, squared */
const emR2 = (e: { pos: V; dir: V; radius: number; tall: number; mode: number }, at: V) => {
  const d: V = [at[0] - e.pos[0], at[1] - e.pos[1], at[2] - e.pos[2]]; let q = d;
  if (e.mode > 2.5 && !BREAK) { const t = Math.min(e.tall, Math.max(-e.tall, d[0] * e.dir[0] + d[1] * e.dir[1] + d[2] * e.dir[2])); q = [d[0] - e.dir[0] * t, d[1] - e.dir[1] * t, d[2] - e.dir[2] * t]; }
  return (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]) / (e.radius * e.radius);
};
/** smoke.wgsl emInBox: does an emitter reach the box lo‥hi (a brick)? */
const emInBox = (e: { pos: V; dir: V; radius: number; tall: number; mode: number }, lo: V, hi: V) => {
  let c: V = [...e.pos], R = 3 * e.radius + e.tall;
  if (e.mode > 2.5 && !BREAK) {
    const m: V = [(lo[0] + hi[0]) / 2 - e.pos[0], (lo[1] + hi[1]) / 2 - e.pos[1], (lo[2] + hi[2]) / 2 - e.pos[2]];
    const t = Math.min(e.tall, Math.max(-e.tall, m[0] * e.dir[0] + m[1] * e.dir[1] + m[2] * e.dir[2]));
    c = [e.pos[0] + e.dir[0] * t, e.pos[1] + e.dir[1] * t, e.pos[2] + e.dir[2] * t]; R = 3 * e.radius + 0.5 * Math.hypot(hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]);
  }
  const q = [0, 1, 2].map(a => Math.min(hi[a], Math.max(lo[a], c[a])) - c[a]);
  return q[0] * q[0] + q[1] * q[1] + q[2] * q[2] <= R * R * 1.02;
};
/** how far a stretch is from a box, by walking it a centimetre at a time */
const segToBox = (e: { pos: V; dir: V; tall: number }, lo: V, hi: V) => {
  let best = Infinity;
  for (let t = -e.tall; t <= e.tall + 1e-9; t += 0.01) { let d2 = 0; for (let a = 0; a < 3; a++) { const x = e.pos[a] + e.dir[a] * t, q = Math.min(hi[a], Math.max(lo[a], x)) - x; d2 += q * q; } best = Math.min(best, d2); }
  return Math.sqrt(best);
};

// ---- the cut: a round's leg of 20 m through a plume 2 m across is sent as the stretch in the plume, once a lattice
{ const S = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }]);
  const took = S.round([0, 1.3, 11], [1, 0, 0], 20), p = S.pushes[0];
  check(took && S.pushes.length === 1 && p.level === 0 && Math.abs(p.pos[0] - 11) < 1e-9 && Math.abs((p.line ?? 0) - (1 + 3 * 0.12)) < 1e-9 && p.pos[1] === 1.3 && p.speed === 9 && p.push === 4,
    `a 20 m leg through a plume from x 10 to 12 is one line about x ${p?.pos[0].toFixed(2)} of half length ${(p?.line ?? 0).toFixed(2)} m (the plume's half width and three radii each end)`);
  const R = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }]); const miss = R.round([0, 1.3, 14], [1, 0, 0], 20);
  check(miss && R.pushes.length === 0, 'a leg that passes two metres beside the plume sends nothing, and still says the smoke took it (no dots either)');
  const T = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }]); T.round([10.5, 1.3, 10.5], [0.6, 0, 0.8], 1.5);
  check(T.pushes.length === 1 && Math.abs((T.pushes[0].line ?? 0) - (0.75 - 0.24)) < 1e-9 && Math.abs(T.pushes[0].pos[0] - 10.95) < 1e-9,
    'a leg that starts and ends inside the plume is sent but for two radii at each end: its round ends stay off what the round hit');
  const N = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }]); N.round([11, 1.3, 11], [0.6, 0, 0.8], 0.5);
  check(N.pushes.length === 0, 'a leg shorter than its two ends sends nothing');
  const U = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }, { lo: [11, 0, 10], hi: [15, 2, 12], level: 0 }, { lo: [10.5, 0, 10.5], hi: [11, 2, 11.5], level: 1 }]); U.round([0, 1.3, 11], [1, 0, 0], 20);
  const a = U.pushes.find(x => x.level === 0), b = U.pushes.find(x => x.level === 1);
  const l0 = U.pushes.filter(x => x.level === 0);
  check(U.pushes.length === 3 && !!a && !!b && l0.length === 2 && l0.every(x => Math.abs((x.line ?? 0) - (2.5 + 0.36) / 2) < 1e-9) && Math.abs(l0[0].pos[0] + l0[1].pos[0] - 25) < 1e-9 && Math.abs(b.pos[0] - 10.75) < 1e-9,
    'two bounds of one lattice that overlap are one stretch (here 5.7 m, so sent as two halves end to end), and the other lattice gets its own');
  const F = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }, { lo: [16, 0, 10], hi: [17, 2, 12], level: 0 }]); F.round([0, 1.3, 11], [1, 0, 0], 20);
  check(F.pushes.length === 2 && Math.abs(F.pushes[0].pos[0] - 11) < 1e-9 && Math.abs(F.pushes[1].pos[0] - 16.5) < 1e-9 && Math.abs((F.pushes[1].line ?? 0) - 0.86) < 1e-9,
    'two plumes of one lattice 4 m apart are two stretches: the clear air between them is not sent');
  const G = system([{ lo: [0, 0, 10], hi: [24, 2, 12], level: 0 }]); G.round([0.5, 1.3, 11], [1, 0, 0], 23);
  check(G.pushes.length === 6 && G.pushes.every(x => (x.line ?? 9) <= 2 + 1e-9), 'a stretch of 22 m is sent in pieces of 4 m at most');
  const O = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }], { speed: 0, radius: 0.12, push: 4 });
  check(O.round([0, 1.3, 11], [1, 0, 0], 20) === false && O.pushes.length === 0, 'with the row at 0 the smoke takes nothing and says so: the caller sends its dots, as it was');
  const D = system([{ lo: [10, 0, 10], hi: [12, 2, 12], level: 0 }]); D.push({ pos: [11, 1, 11], dir: [1, 0, 0], speed: 2, radius: 0.3 }); D.push({ pos: [13, 1, 11], dir: [1, 0, 0], speed: 2, radius: 0.3 });
  check(D.pushes.length === 1 && D.pushes[0].line === undefined, 'a plain shove is a point as it was: sent where it lies in the smoke, dropped where it does not');
}
// ---- the solver's distance: every cell along the path is as near as the cell at its middle, and the ends are round
{ const e = { pos: [11, 1.3, 11] as V, dir: [1, 0, 0] as V, radius: 0.12, tall: 1.36, mode: 3 };
  const mid = emR2(e, [11, 1.3, 11.06]), far = emR2(e, [12.2, 1.3, 11.06]), past = emR2(e, [12.36 + 0.12, 1.3, 11]), body = emR2({ ...e, mode: 2 }, [12.2, 1.3, 11.06]);
  check(Math.abs(mid - 0.25) < 1e-9 && Math.abs(far - mid) < 1e-9, `half a radius off the path is ${far.toFixed(2)} radii² at its middle and 1.2 m along it alike`);
  check(Math.abs(past - 1) < 1e-9, 'one radius past the end of the stretch is one radius away: the ends are round');
  check(body > 100, 'an emitter of any other mode is a point, as it was');
}
// ---- the bricks a line reaches: the ones along it, never a sphere of its length, and never fewer than its shove touches
{ const bs = 0.44, e = { pos: [11, 1.3, 11] as V, dir: [1, 0, 0] as V, radius: 0.12, tall: 2, mode: 3 };
  const brick = (x: number, y: number, z: number): [V, V] => [[x * bs, y * bs, z * bs], [(x + 1) * bs, (y + 1) * bs, (z + 1) * bs]];
  check(emInBox(e, ...brick(29, 2, 25)) && emInBox(e, ...brick(21, 2, 25)) && !emInBox(e, ...brick(25, 2, 29)) && !emInBox(e, ...brick(25, 7, 25)) && !emInBox(e, ...brick(33, 2, 25)),
    'a line 4 m long reaches the bricks at both its ends, and not one 1.5 m to its side, 2 m over it or 1 m past its end');
  let seed = 12345; const rnd = () => (seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff;
  let missed = 0, loose = 0, n = 0;
  for (let i = 0; i < 400; i++) {
    const d: V = [rnd() - 0.5, rnd() - 0.5, rnd() - 0.5], len = Math.hypot(...d); if (len < 0.05) continue;
    const E = { pos: [rnd() * 3, rnd() * 3, rnd() * 3] as V, dir: [d[0] / len, d[1] / len, d[2] / len] as V, radius: 0.06 + rnd() * 0.24, tall: 0.03 + rnd() * 2, mode: 3 };
    const [lo, hi] = brick(Math.floor(rnd() * 7), Math.floor(rnd() * 7), Math.floor(rnd() * 7)), dist = segToBox(E, lo, hi), inb = emInBox(E, lo, hi); n++;
    if (dist <= 3 * E.radius && !inb) missed++;
    if (inb && dist > 3 * E.radius + bs * Math.sqrt(3) + 0.02) loose++;
  }
  check(n > 300 && missed === 0 && loose === 0, `over ${n} lines and bricks at random: no brick within three radii of a line is left out (${missed}), and none further than a brick's diagonal past that is taken (${loose})`);
}
// ---- the texts
{ const fn = wgsl.match(/\nfn emR2\(e: Emitter, d: vec3f\) -> f32 \{ ([^\n]*) \}\n/)?.[1] ?? '';
  check(fn === 'var q = d; if (e.mode > 2.5) { q -= e.dir * clamp(dot(d, e.dir), -e.tall, e.tall); } return dot(q, q) / (e.radius * e.radius);', 'smoke.wgsl emR2 is the distance mirrored above');
  check((wgsl.match(/emR2\(e, /g) ?? []).length === 3 && !/dot\(d, d\) \/ \(e\.radius \* e\.radius\)/.test(wgsl), 'the shove, the swelling and the feed all measure an emitter by emR2, and no site is left measuring a point');
  check(/if \(e\.mode < 1\.5 \|\| e\.mode > 2\.5\) \{ continue; \}/.test(wgsl) && /if \(e\.mode > 1\.5 && e\.mode < 2\.5\) \{/.test(wgsl) && /if \(e\.mode > 0\.5 && e\.mode < 1\.5\) \{ jet = normalize/.test(wgsl), 'a line is not taken for a body or a burst');
  const box = wgsl.match(/\nfn emInBox\(e: Emitter, lo: vec3f, hi: vec3f\) -> bool \{\n([^}]*\}[^}]*)\}\n/)?.[1].replace(/\s+/g, ' ').trim() ?? '';
  check(box === 'var c = e.pos; var R = emReach(e); if (e.mode > 2.5) { c += e.dir * clamp(dot((lo + hi) * 0.5 - e.pos, e.dir), -e.tall, e.tall); R = 3.0 * e.radius + 0.5 * length(hi - lo); } let q = clamp(c, lo, hi) - c; return dot(q, q) <= R * R * 1.02;',
    'smoke.wgsl emInBox is the reach mirrored above');
  check(/if \(emInBox\(e, vec3f\(b\) \* bs, vec3f\(b \+ 1\) \* bs\)\)/.test(wgsl) && /if \(e\.level == k\.l && emInBox\(e, lo, hi\)\)/.test(wgsl) && /let R = emHalf\(e\);/.test(wgsl)
    && /fn emHalf\(e: Emitter\) -> vec3f \{ if \(e\.mode > 2\.5\) \{ return abs\(e\.dir\) \* e\.tall \+ vec3f\(3\.0 \* e\.radius\); \} return vec3f\(emReach\(e\)\); \}/.test(wgsl),
    'the bricks an emitter wakes and the emitters a brick lists are both found by emInBox, over the stretch\'s own box of bricks');
  check(wgsl.includes('u += e.dir * max(e.speed * clamp(exp(-r2) * e.push, 0.0, 1.0) - dot(u, e.dir), 0.0);') && /LINE_MOST = 4 /.test(ts) && /readonly wake = \{ speed: 9, radius: 0\.12, push: 4 \};/.test(ts),
    'a line brings the air along it up to its speed once: no frame length in it, never slower, never turned');
  check(/\(e\.line \?\? 0\) > 0 \? 3 : 0;/.test(ts) && /const j = \(e\.line \?\? 0\) > 0 \? 0 : /.test(ts), 'a line is packed as mode 3 with its half length, and its direction is not jittered');
  check(/if \(third && lookConfig\.on && lookConfig\.stir > 0 && g\.smoke\.round\)/.test(combat) && /if \(third && !lined\) for \(let d = 0\.3;/.test(combat), 'combat asks the smoke for a line on the look page only, with its Stirred smoke row up, and sends its dots when the smoke did not take it');
}
console.log(red ? `smokeline-check: ${red} FAILED` : 'smokeline-check: all passed'); process.exit(red ? 1 : 0);
