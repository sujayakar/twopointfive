// THE FOG OF WAR'S FRAME, from its section. The renderer's frame (render/fogofwar.ts FogFrame) is one object its passes hold; the two words that are the
// renderer's own (`sight`, `pass`) are not the game's to say.
import type { FogFrame } from '../render/fogofwar';
import type { Drawn } from './drawn';
import { Fog, Section } from './sections';

export function fogFrom(F: FogFrame, D: Drawn): void {
  const f = D.f, u = D.u, o = D.at[Section.fog], on = D.n[Section.fog] > 0 && u[o + Fog.on] !== 0;
  F.on = on;
  F.count = on ? u[o + Fog.count] : 0;
  F.full = on && u[o + Fog.full] !== 0;
  F.lit = on ? f[o + Fog.lit] : 0;
  F.lightCount = on ? u[o + Fog.lightCount] : 0;
  if (!on) return;
  for (let i = 0; i < F.eyes.length; i++) { F.eyes[i] = f[o + Fog.eyes + i]; F.looks[i] = f[o + Fog.looks + i]; }
  F.dim = f[o + Fog.dim]; F.desat = f[o + Fog.desat]; F.topPad = f[o + Fog.topPad]; F.edge = f[o + Fog.edge];
  F.softIn = f[o + Fog.softIn]; F.softOut = f[o + Fog.softOut]; F.nearSoft = f[o + Fog.nearSoft];
  F.flat = f[o + Fog.flat]; F.fullFlat = f[o + Fog.fullFlat];
  F.litFill = f[o + Fog.litFill]; F.litCut = f[o + Fog.litCut]; F.litAo = f[o + Fog.litAo]; F.litAoM = f[o + Fog.litAoM];
  for (let i = 0, n = F.lightCount * 16; i < n; i++) F.lightsU[i] = u[o + Fog.lights + i];
  F.air = u[o + Fog.air] !== 0; F.kept = u[o + Fog.kept] !== 0; F.thin = u[o + Fog.thin] !== 0;
  for (let i = 0; i < F.thinBits.length; i++) F.thinBits[i] = u[o + Fog.thinBits + i];
  F.thinSerial = u[o + Fog.thinSerial]; F.thinCount = u[o + Fog.thinCount];
}
