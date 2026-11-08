// A tile of blue noise, by void and cluster: each rank goes where it is emptiest.
export function blueNoise(N = 64, seed = 1): Float32Array {
  const n = N * N, out = new Float32Array(n), e = new Float32Array(n), R = 7, g: number[] = []; let s = seed >>> 0 || 1; const rnd = () => (s = (Math.imul(s, 1664525) + 1013904223) >>> 0) / 4294967296;
  for (let y = -R; y <= R; y++) for (let x = -R; x <= R; x++) g.push(Math.exp(-(x * x + y * y) / (2 * 1.9 * 1.9)));
  const splat = (i: number, k: number) => { const X = i % N, Y = (i / N) | 0; let q = 0; for (let y = -R; y <= R; y++) for (let x = -R; x <= R; x++) e[((Y + y + N) % N) * N + ((X + x + N) % N)]! += k * g[q++]!; };
  const on = new Uint8Array(n), first = Math.floor(n * 0.1); for (let k = 0; k < first;) { const i = Math.floor(rnd() * n); if (!on[i]) { on[i] = 1; splat(i, 1); k++; } }
  const worst = (v: number) => { let b = -1, best = v ? -1e9 : 1e9; for (let i = 0; i < n; i++) if (on[i] === v && (v ? e[i]! > best : e[i]! < best)) { best = e[i]!; b = i; } return b; };
  for (let k = 0; k < 4000; k++) { const a = worst(1); on[a] = 0; splat(a, -1); const b = worst(0); on[b] = 1; splat(b, 1); if (a === b) break; }
  const on0 = on.slice(), e0 = e.slice(); for (let r = first - 1; r >= 0; r--) { const a = worst(1); on[a] = 0; splat(a, -1); out[a] = r; }
  on.set(on0); e.set(e0); for (let r = first; r < n; r++) { const b = worst(0); on[b] = 1; splat(b, 1); out[b] = r; }
  for (let i = 0; i < n; i++) out[i] = (out[i]! + 0.5) / n; return out;
}
