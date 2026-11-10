// THE SKY, THE HOUR AND THE WEATHER onto the engine, from their section.
import type { Engine } from '../engine';
import type { Drawn } from './drawn';
import { Section, Sky } from './sections';

/** a section's three floats at `o`, into `v` */
const into = (v: { [i: number]: number }, f: Float32Array, o: number, n: number): void => { for (let i = 0; i < n; i++) v[i] = f[o + i]; };
export const HOURS_OF = ['night', 'dawn', 'midday', 'overcast', 'dusk'] as const;

/** what the section last said of what a hand may turn (the sky, the haze, the gloss, the exposure): the hand's own turn stands until it says another, and the exposure's is carried to that */
const last = new Float32Array(9);

export function skyFrom(E: Engine, D: Drawn): void {
  const f = D.f, u = D.u, o = D.at[Section.sky], S = E.settings, W = E.weather;
  let moved = false;
  for (let i = 0; i < 3; i++) if (last[i] !== f[o + Sky.zenith + i] || last[3 + i] !== f[o + Sky.horizon + i]) { last[i] = f[o + Sky.zenith + i]; last[3 + i] = f[o + Sky.horizon + i]; moved = true; }
  if (moved) { S.skyZenith = [last[0], last[1], last[2]]; S.skyHorizon = [last[3], last[4], last[5]]; }
  if (last[6] !== f[o + Sky.haze]) S.hazeDensity = last[6] = f[o + Sky.haze];
  const said = f[o + Sky.exposure]; if (said !== last[7]) { S.exposure = last[7] > 0 ? S.exposure * said / last[7] : said; last[7] = said; }
  if (last[8] !== 1 + u[o + Sky.gloss]) { last[8] = 1 + u[o + Sky.gloss]; S.gloss = u[o + Sky.gloss] !== 0; }
  if (!u[o + Sky.weather]) return;
  W.rain = f[o + Sky.rain]; W.snow = f[o + Sky.snow]; W.shearX = f[o + Sky.shear]; W.shearZ = f[o + Sky.shear + 1]; W.wet = f[o + Sky.wet]; W.rainTop = f[o + Sky.rainTop];
  into(W.flash, f, o + Sky.flash, 3); into(W.interior, f, o + Sky.interior, 4); into(W.annex, f, o + Sky.annex, 4); into(W.nearFade, f, o + Sky.nearFade, 4);
  W.roofC = u[o + Sky.roofC]; W.roofD = u[o + Sky.roofD]; W.fog = u[o + Sky.fog];
  if (f[o + Sky.phase + 3] !== 0) into(W.phase ??= [0, 0, 0, 0], f, o + Sky.phase, 4); else W.phase = undefined;
}
