// WHAT THE LEVEL ITSELF SAYS OF HOW IT LOOKS, handed over where Rust left it: the traffic map its floors read (rust/game_draw traffic.rs) and how its puffs of dust look (dust.rs).
import { WORLD } from '../scene/world';
import { WEAR_RES } from '../render/wear';
import type { GBuffer } from '../render/gbuffer';
import type { Particles } from '../render/particles';
import type { Drawn } from './drawn';
import { DustLook, Section } from './sections';

/** once, as the level opens */
export function trafficFrom(gbuffer: GBuffer, D: Drawn): void {
  D.fresh();
  gbuffer.setWearMap(new Uint8Array(D.u.buffer, D.at[Section.traffic] * 4, D.n[Section.traffic] * 4), Math.round(WORLD.gridW * WORLD.gridCell * WEAR_RES), Math.round(WORLD.gridH * WORLD.gridCell * WEAR_RES));
}
/** the kernels read the kind's numbers every frame: a change of the table is taken at the next */
export function dustFrom(P: Particles, D: Drawn): void {
  const f = D.f, o = D.at[Section.dust], d = P.params.dust;
  d.brightOrOpacity = f[o + DustLook.opacity]; d.shutterOrGrowth = f[o + DustLook.growth]; d.sleepV = f[o + DustLook.fadeIn]; d.holdOrFade = f[o + DustLook.soft]; d.restitution = f[o + DustLook.wrap]; d.keep = f[o + DustLook.ambient];
}
