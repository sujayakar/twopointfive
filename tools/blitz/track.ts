// WHAT A SMOKE SOURCE RIDES: the Rust game says whose muzzle, whose ejection port or which spark, and the solver asks each frame where that is
export function trackOf(g: any, t: { kind: string; by?: any; slot?: number }): (() => { pos: number[]; dir: number[] } | null) | undefined {
  if (t.kind === 'muzzle' || t.kind === 'ejectPort') { const c = t.by; if (!c) return undefined;
    return t.kind === 'muzzle' ? () => ({ pos: c.muzzle, dir: c.gunDir })
      : () => { const from = c.bones?.handR ?? c.muzzle, d = c.gunDir; return { pos: [from[0] + d[0] * 0.06, from[1] + d[1] * 0.06 + 0.06, from[2] + d[2] * 0.06], dir: [0, 1, 0] }; }; }
  const s = g.sparks?.pool?.[t.slot ?? -1]; if (!s) return undefined; let last = -1, gone = false;
  return () => { if (gone || !s.alive || s.age < last) { gone = true; return null; } last = s.age; return { pos: s.pos, dir: [0, 1, 0] }; }; }
