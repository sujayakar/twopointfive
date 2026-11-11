// WHAT THE LEVEL ITSELF SAYS OF HOW IT LOOKS, handed over where Rust left it: the traffic map its floors read (rust/game_draw traffic.rs) and how its puffs of dust look (dust.rs).
import { WORLD } from '../scene/world';
import type { GBuffer } from '../render/gbuffer';
import type { Particles } from '../render/particles';
import type { Drawn } from './drawn';
import { DustLook, Section } from './sections';

/** texels a metre of the traffic map (rust/game_draw/src/traffic.rs makes it) */
const WEAR_RES = 8;

/** once, as the level opens: the module's map (`DrawGame.traffic`) */
export function trafficFrom(gbuffer: GBuffer, map: Uint8Array): void {
  const w = Math.round(WORLD.gridW * WORLD.gridCell * WEAR_RES), h = Math.round(WORLD.gridH * WORLD.gridCell * WEAR_RES);
  if (map.length !== w * h * 2) throw new Error(`a traffic map of ${map.length} bytes, and a floor of ${w} x ${h} texels is ${w * h * 2}`);
  gbuffer.setWearMap(map, w, h);
}
/** the kernels read the kind's numbers every frame: a change of the table is taken at the next */
export function dustFrom(P: Particles, D: Drawn): void {
  const f = D.f, o = D.at[Section.dust], d = P.params.dust;
  d.brightOrOpacity = f[o + DustLook.opacity]; d.shutterOrGrowth = f[o + DustLook.growth]; d.sleepV = f[o + DustLook.fadeIn]; d.holdOrFade = f[o + DustLook.soft]; d.restitution = f[o + DustLook.wrap]; d.keep = f[o + DustLook.ambient];
}
