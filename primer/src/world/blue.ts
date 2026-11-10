// A TILE OF BLUE NOISE, by void and cluster: each rank goes where it is emptiest, so that the first so
// many ranks are always spread out evenly, and neighbors are always unlike.

export function blueNoise(size = 64, seed = 1): Float32Array {
  const count = size * size;
  const REACH = 7;
  const kernel: number[] = [];
  for (let y = -REACH; y <= REACH; y++) {
    for (let x = -REACH; x <= REACH; x++) kernel.push(Math.exp(-(x * x + y * y) / (2 * 1.9 * 1.9)));
  }
  let state = seed >>> 0 || 1;
  const random = () => (state = (Math.imul(state, 1664525) + 1013904223) >>> 0) / 4294967296;

  const energy = new Float32Array(count);
  const splat = (i: number, sign: number) => {
    const cx = i % size;
    const cy = Math.floor(i / size);
    let k = 0;
    for (let y = -REACH; y <= REACH; y++) {
      for (let x = -REACH; x <= REACH; x++) {
        const at = ((cy + y + size) % size) * size + ((cx + x + size) % size);
        energy[at]! += sign * kernel[k++]!;
      }
    }
  };
  const on = new Uint8Array(count);
  /** the tightest cluster among points that are on, or the largest void among those that are off */
  const extreme = (wanted: 0 | 1) => {
    let best = -1;
    let value = wanted ? -Infinity : Infinity;
    for (let i = 0; i < count; i++) {
      if (on[i] !== wanted) continue;
      if (wanted ? energy[i]! > value : energy[i]! < value) {
        value = energy[i]!;
        best = i;
      }
    }
    return best;
  };
  const first = Math.floor(count * 0.1);
  for (let placed = 0; placed < first; ) {
    const i = Math.floor(random() * count);
    if (on[i]) continue;
    on[i] = 1;
    splat(i, 1);
    placed++;
  }
  for (let k = 0; k < 4000; k++) {
    const cluster = extreme(1);
    on[cluster] = 0;
    splat(cluster, -1);
    const hole = extreme(0);
    on[hole] = 1;
    splat(hole, 1);
    if (cluster === hole) break;
  }
  const rank = new Float32Array(count);
  const onAtFirst = on.slice();
  const energyAtFirst = energy.slice();
  for (let r = first - 1; r >= 0; r--) {
    const i = extreme(1);
    on[i] = 0;
    splat(i, -1);
    rank[i] = r;
  }
  on.set(onAtFirst);
  energy.set(energyAtFirst);
  for (let r = first; r < count; r++) {
    const i = extreme(0);
    on[i] = 1;
    splat(i, 1);
    rank[i] = r;
  }
  return rank.map((r) => (r + 0.5) / count);
}
