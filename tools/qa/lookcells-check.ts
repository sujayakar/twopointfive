// A LATTICE'S FEATURE POINTS ARE HASHED ONCE A LAYER: the check behind render/lookTex.ts cells2 and its cellTable.
//   bun run tools/qa/probes.ts lookcells        (family `render`)
//   LOOKCELLS_BREAK=1   the plain way below is handed a jitter a ten-thousandth off: must FAIL
// Why: making a look map, every texel asked its nine neighbouring cells where their points lie (three hashes a cell and a square root a cell), which was
// most of what a layer cost at the boot. cells2 now reads a table a lattice fills once, tells the nine apart by their squared distances and roots only the
// two nearest. The claim is that nothing it answers moves by a bit, so no map does.
// WHAT THIS CAN AND CANNOT SAY. The plain way is written out below as it stood, and cells2 is held to it answer for answer (Object.is on all five numbers)
// at every texel centre of a map on the lattices the materials use, at seeded scattered points and on cell edges. Two points within a rounding of the
// same distance could in principle be told apart differently by a square than by a root; none is met here, and if a new lattice ever meets one this goes
// red. It does not make the maps: that they came out byte for byte the same was compared once, layer by layer, when the table went in.
import { ROOT } from './headless.ts';
export async function lookCellsCheck(): Promise<boolean> {
  const BRK = process.env.LOOKCELLS_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== lookcells: a lattice's feature points are hashed once a layer${BRK ? ' (the plain way a hair off: must fail)' : ''} ===`);
  const T = await import(`${ROOT}/src/render/lookTex.ts`); const cells2 = T.cells2 as (u: number, v: number, p: number, q: number, seed: number, jitter?: number) => { f1: number; f2: number; id: number; dx: number; dy: number };
  // the plain way, as it stood: the hash, the wrap, nine cells, a root each
  const hash = (ix: number, iy: number, seed: number) => { let h = (Math.imul(ix | 0, 73856093) ^ Math.imul(iy | 0, 19349663) ^ Math.imul(seed | 0, 83492791)) >>> 0; h = (Math.imul(h, 747796405) + 2891336453) >>> 0; h = Math.imul((h >>> ((h >>> 28) + 4)) ^ h, 277803737) >>> 0; return (((h >>> 22) ^ h) >>> 0) / 4294967296; };
  const wrap = (i: number, p: number) => ((i % p) + p) % p;
  const plain = (u: number, v: number, p: number, q: number, seed: number, jitter: number) => {
    if (BRK) jitter *= 1.0001;
    const x = u * p, y = v * q, ix = Math.floor(x), iy = Math.floor(y); let f1 = 9, f2 = 9, id = 0, bx = 0, by = 0;
    for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) {
      const cx = wrap(ix + i, p), cy = wrap(iy + j, q);
      const px = ix + i + 0.5 + (hash(cx, cy, seed) - 0.5) * jitter, py = iy + j + 0.5 + (hash(cx, cy, seed + 1) - 0.5) * jitter;
      const dx = px - x, dy = py - y, d = Math.hypot(dx, dy);
      if (d < f1) { f2 = f1; f1 = d; id = hash(cx, cy, seed + 2); bx = dx; by = dy; } else if (d < f2) f2 = d;
    }
    return [f1, f2, id, bx, by];
  };
  // the lattices the materials ask for today (size, seed, jitter), a wide one, a tiny one and one wider than tall
  const LAT: [number, number, number, number][] = [[96, 96, 414, 0.9], [150, 150, 419, 0.9], [40, 40, 421, 0.85], [5, 5, 422, 0.7], [170, 170, 398, 0.5], [44, 44, 440, 1.0], [77, 77, 461, 1.0], [1, 1, 7, 0.85], [32, 8, 77, 0.85], [8, 32, 77, 0.85]];
  let s = 12345; const rnd = () => { s = (Math.imul(s, 1664525) + 1013904223) >>> 0; return s / 4294967296; };
  const differs = (u: number, v: number, L: [number, number, number, number]) => { const a = plain(u, v, L[0], L[1], L[2], L[3]), c = cells2(u, v, L[0], L[1], L[2], L[3]); return !(Object.is(a[0], c.f1) && Object.is(a[1], c.f2) && Object.is(a[2], c.id) && Object.is(a[3], c.dx) && Object.is(a[4], c.dy)); };
  { let asked = 0, bad = 0, first = ''; const N = 512;
    for (const L of LAT.slice(0, 5)) for (let y = 0; y < N; y++) for (let x = 0; x < N; x++) { asked++; if (differs((x + 0.5) / N, (y + 0.5) / N, L)) { if (!bad++) first = `lattice ${L.join(' ')} at texel ${x}, ${y}`; } }
    verdict(bad === 0, 'at every texel centre of a 512 map, on five of the materials\' lattices, cells2 answers what the plain nine-cell way answers, to the bit', bad ? `${bad} of ${asked} differ, the first on ${first}` : `${asked} answers, none differs`); }
  { let asked = 0, bad = 0, first = '';
    for (let k = 0; k < 400000; k++) { const L = LAT[k % LAT.length]; let u = rnd(), v = rnd(); if (k % 7 === 0) u = Math.floor(u * L[0]) / L[0]; if (k % 11 === 0) v = Math.floor(v * L[1]) / L[1]; if (k % 97 === 0) { u = 0; } if (k % 101 === 0) { v = 1 - 2 ** -53; } asked++;   // (scattered, on cell edges, at the tile's own edges; the lattices taken in turns, so a table is asked for between two others)
      if (differs(u, v, L)) { if (!bad++) first = `lattice ${L.join(' ')} at ${u}, ${v}`; } }
    verdict(bad === 0, 'the same at scattered points, on cell edges and at the tile\'s edges, the ten lattices asked in turns (a table is never another lattice\'s)', bad ? `${bad} of ${asked} differ, the first on ${first}` : `${asked} answers, none differs`); }
  { const f: string[] = []; const refuses = (p: number, q: number, seed: number, jitter: number) => { try { cells2(0.3, 0.3, p, q, seed, jitter); return false; } catch { return true; } };
    if (!refuses(2.5, 2.5, 901, 0.85)) f.push('a lattice of 2.5 cells was made'); if (!refuses(4, 0, 902, 0.85)) f.push('a lattice of no cells was made'); if (!refuses(4, 4, NaN, 0.85)) f.push('a seed that is no number was taken'); if (!refuses(4, 4, 903, NaN)) f.push('a jitter that is no number was taken');
    verdict(f.length === 0, 'a lattice that is not whole cells, or a seed or jitter that is no number, is refused aloud, never answered from a table read between its rows', f.join('; ') || 'all four refused'); }
  { // A LAYER MADE IN PLAY BREATHES IN ITS LAST PASSES TOO. Made in slices, a layer stopped after every row of its material and then ran the evening, the two
    // blurs, the hollows and the normals in two unbroken stretches of 20 to 40 ms: on the page's own thread, in play, a dropped frame or two. They stop
    // every few rows now. Asked by count, not by the clock: with a slice of nothing every stop is a breath
    const f: string[] = []; const N = 256, l = (T.LOOK_NAMES as string[]).indexOf('plaster'); let stops = 0;
    const sliced = await T.lookLayerSliced(l, N, true, 0, async () => { stops++; }), whole = T.lookLayer(l, N, true);
    const same = (x: Uint8Array, y: Uint8Array) => x.length === y.length && x.every((v, k) => v === y[k]);
    if (!(same(sliced.a, whole.a) && same(sliced.b, whole.b) && Object.is(sliced.tone, whole.tone) && Object.is(sliced.hollow, whole.hollow))) f.push('made in slices, the layer is not the layer made in one go');
    const rowsOnly = N + 2;   // (as it was: a stop a row and one between each of the three passes after them)
    if (!(stops >= rowsOnly + 5 * (N >> 5) - 2)) f.push(`${stops} stops in a ${N} map (want the ${rowsOnly} of its rows and at least ${5 * (N >> 5) - 2} more in the five passes after them)`);
    verdict(f.length === 0, 'a layer made in slices is the layer made in one go, to the byte, and its last passes stop every few rows as its first does every row', f.join('; ') || `plaster at ${N}: ${stops} stops, ${rowsOnly} of them the rows'`); }
  console.log(`=== lookcells: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await lookCellsCheck()) ? 0 : 1);
