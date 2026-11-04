// STEADY BOUNCE (src/render/diceRing.ts ringPoints / ringSlot / ringMask, the kernel shaders/dice_ring.wgsl; engine.ts runs the cache's dice through it; the
// look page's row `steady`). The maintainer on his GPU, 11-01, the pass at midday with doors open: "lots of temporal instability for midday, also strange
// shimmering on metallic barrels", "on the guy's back too", "overall night, dawn, and dusk look pretty good. midday and overcast are the broken ones". The
// radiance cache's directions are shifted a frame and its dice was a running mean of about three frames. By day a room's light is what a door or a window
// lets in, small and bright, caught by one frame's directions and missed by the next, and everything that reads the cache pulsed. By day the shift is one of
// a few points in turn and the dice is the mean of the last bake at each.
// No GPU: what is read is how many points each hour gets, which slot a frame takes, what is held as a ring fills and starts over, and a model of one probe
// whose bake depends on the frame's point alone (a still scene): its mean frame after frame, under the ring and under the game's running mean.
// The picture is never seen here; the ring's program is compiled and run only by a page.
//   bun run tools/qa/steadybounce-check.ts   (exit 0 = all pass; STEADYBOUNCE_BREAK=1 leaves the row at nothing and must fail)
import { RING_MAX, ringPoints, ringSlot, ringMask } from '../../src/render/diceRing';
import { LOOK_DEFAULTS } from '../../src/render/gbuffer';
import { HOURS } from '../../src/game/hours';
import { hourExposure } from '../../src/render/photometry';
const BRK = process.env.STEADYBOUNCE_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== steadybounce: the cache's mean over a few points by day${BRK ? ' (the row at nothing: must fail)' : ''} ===`);
const ROW = BRK ? 0 : LOOK_DEFAULTS.steady;
const share = (h: string) => h === 'night' ? 1 : hourExposure(h, (HOURS as Record<string, { sun: { intensity: number } }>)[h].sun.intensity);
const hours = ['night', 'dawn', 'dusk', 'overcast', 'midday'].filter(h => h === 'night' || h in HOURS), at = Object.fromEntries(hours.map(h => [h, ringPoints(share(h), ROW)])) as Record<string, number>;
check(at.night === 0 && ringPoints(1 / 5000, 0) === 0 && ringPoints(1, 1) === 0 && ringPoints(0.5, 1) === 0, 'at night, with the row at nothing and at an hour whose lamps are near their night strength there is no ring: the cache keeps its own mean',
  `night ${at.night}, the row at nothing at midday ${ringPoints(1 / 5000, 0)}, lamps at a half ${ringPoints(0.5, 1)}`);
check(at.midday === RING_MAX && at.overcast === RING_MAX && at.dusk === 4 && at.dawn === 4 && ringPoints(1e-9, 4) === RING_MAX && [0.5, 0.05, 0.01, 0.001, 1e-5].every(x => [0, 4, 8].includes(ringPoints(x, 1))),
  'by day the mean is over more points the further the lamps are under the daylight: eight at midday and on a grey day, four at dusk and dawn, and never a count between (a round is a whole number of the cache\'s own refresh rounds)', hours.map(h => `${h} ${at[h]} (lamps at 1/${Math.round(1 / share(h))})`).join(', '));
{ const n = at.midday || 8, seen = new Set<number>(); for (let f = 0; f < n; f++) seen.add(ringSlot(f, n));
  check(seen.size === n && ringSlot(n, n) === 0 && ringSlot(3 * n + 2, n) === 2 && ringSlot(5, 0) === 0, 'the frames take the points in turn and come round', `${n} points: frames 0 to ${n - 1} take ${[...seen].join(' ')}, frame ${n} takes ${ringSlot(n, n)}`); }
{ const n = at.midday || 8; let m = 0; const held: number[] = [];
  for (let f = 0; f < n + 2; f++) { m = ringMask(m, ringSlot(f, n), n, f === 0); held.push(m.toString(2).split('1').length - 1); }
  const again = ringMask(m, 3, n, true), fewer = ringMask(0b11111111, 1, 4, false);
  check(held.slice(0, n).every((c, i) => c === i + 1) && held[n] === n && again === 0b1000 && fewer === 0b1111, 'a ring fills a point a frame, holds them all from then on, drops what it held when the cache starts over, and never counts a slot past its points',
    `held after each frame: ${held.join(' ')}; after a start over at point 3: ${again.toString(2)}`); }
// one probe in a still scene: its bake is what the frame's point finds (a door, 1000 against the room's 1, found from one point in the ring's n; under the
// game's own shift, which never repeats, from one frame in n). The ring's mean, frame after frame, against the game's running mean of 0.3 a frame.
{ const n = at.midday || 8, bake = (k: number) => (k === 2 ? 1000 : 1), slots = new Array<number>(n).fill(0); let m = 0; const ring: number[] = [];
  for (let f = 0; f < 6 * n; f++) { const k = ringSlot(f, n); slots[k] = bake(k); m = ringMask(m, k, n, f === 0); let s = 0, c = 0; for (let i = 0; i < n; i++) if (m >> i & 1) { s += slots[i]; c++; } ring.push(s / c); }
  let v = 1; const ema: number[] = []; for (let f = 0; f < 6 * n; f++) { v += (bake(f % n) - v) * 0.3; ema.push(v); }
  const sw = (a: number[]) => { const t = a.slice(2 * n); return Math.max(...t) / Math.min(...t); };
  check(sw(ring) === 1 && sw(ema) > 5, 'in a still scene the ring\'s mean is the same number every frame once it has filled, where the game\'s running mean swings many times over',
    `highest over lowest after two rounds: the ring ${sw(ring).toFixed(3)}, the running mean ${sw(ema).toFixed(1)}`);
  // a real change (the door shuts at frame 3n: every point now finds 1) is all in after one round, a share a frame
  const slots2 = new Array<number>(n).fill(0), after: number[] = []; for (let f = 0; f < 5 * n; f++) { const k = ringSlot(f, n); slots2[k] = f >= 3 * n ? 1 : bake(k); if (f >= 3 * n) after.push(slots2.reduce((a, b) => a + b, 0) / n); }
  check(after[n - 1] === 1 && after[0] >= 1 && after.every((x, i) => i === 0 || x <= after[i - 1]), 'a real change of light is all in after one round of the points', `the door shuts: the mean over the next ${n} frames ${after.slice(0, n).map(x => x.toFixed(0)).join(' ')}`); }
process.exit(red ? 1 : 0);
