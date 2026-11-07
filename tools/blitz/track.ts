// WHAT A SMOKE SOURCE RIDES: the Rust game says whose muzzle, whose ejection port, whose cigarette or which spark, and the solver asks each frame where that is
import { SMOKEBREAK } from '../../src/game/smokebreak';
export function trackOf(g: any, t: { kind: string; by?: any; slot?: number }): (() => { pos: number[]; dir: number[] } | null) | undefined {
  if (t.kind === 'muzzle' || t.kind === 'ejectPort') { const c = t.by; if (!c) return undefined;
    return t.kind === 'muzzle' ? () => ({ pos: c.muzzle, dir: c.gunDir })
      : () => { const from = c.bones?.handR ?? c.muzzle, d = c.gunDir; return { pos: [from[0] + d[0] * 0.06, from[1] + d[1] * 0.06 + 0.06, from[2] + d[2] * 0.06], dir: [0, 1, 0] }; }; }
  // (a cigarette: its lit end, while he is seen. The source lives a second and the game files the next while he smokes, so its stopping needs no word here)
  if (t.kind === 'cigarette') { const c = t.by; if (!c) return undefined; const K = SMOKEBREAK;
    return () => { const hand = c.bones?.handL, F = g.fogOfWar; if (!hand || (F && !F.bared && F.hiddenChars?.has?.(c))) return null;
      const e = c.bones?.forearmL; let d = [Math.sin(c.bodyYaw), 0, Math.cos(c.bodyYaw)]; if (e) { const x = hand[0] - e[0], y = hand[1] - e[1], z = hand[2] - e[2], l = Math.hypot(x, y, z); if (l > 1e-4) d = [x / l, y / l, z / l]; }
      return { pos: [hand[0] + d[0] * K.tipOut, hand[1] + d[1] * K.tipOut + K.tipUp, hand[2] + d[2] * K.tipOut], dir: [c.vel[0] * 0.5, K.thread.speed, c.vel[2] * 0.5] }; }; }
  const s = g.sparks?.pool?.[t.slot ?? -1]; if (!s) return undefined; let last = -1, gone = false;
  return () => { if (gone || !s.alive || s.age < last) { gone = true; return null; } last = s.age; return { pos: s.pos, dir: [0, 1, 0] }; }; }
