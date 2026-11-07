#![allow(unused_variables)]
//! Every function and method of `shards`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/shards.ts:145 shardsLive)
pub fn shards_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/shards.ts:154 u32)
pub fn u32(x: f64) -> f64 { todo!() }

/// (src/game/shards.ts:156 cellName)
pub fn cell_name(ix: f64, iz: f64) -> f64 { todo!() }

/// (src/game/shards.ts:161 GlassShards.pending)
pub fn glass_shards_pending(g: &mut crate::game::Game) -> Vec<crate::shards::Pending> { todo!() }

/// (src/game/shards.ts:167 GlassShards.litter)
pub fn glass_shards_litter(g: &mut crate::game::Game) -> JsMap<crate::Num, f64> { todo!() }

/// (src/game/shards.ts:180 GlassShards.constructor)
pub fn glass_shards_new(g: &mut crate::game::Game) -> crate::shards::GlassShards { todo!() }

/// (src/game/shards.ts:186 GlassShards.release)
pub fn glass_shards_release(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike) -> bool { todo!() }

/// (src/game/shards.ts:245 GlassShards.spall)
pub fn glass_shards_spall(g: &mut crate::game::Game, at: V3, n: V3, dir: Option<V3>, thickness: f64, tint: V3, seed: f64, far: Option<bool>) { todo!() }

/// (src/game/shards.ts:255 GlassShards.take)
pub fn glass_shards_take(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64) -> Option<DebrisChunkId> { todo!() }

/// (src/game/shards.ts:283 GlassShards.paneAt)
pub fn glass_shards_pane_at(g: &mut crate::game::Game, c_2: V3) -> Option<crate::glassbreak::PaneState> { todo!() }

/// (src/game/shards.ts:295 GlassShards.dicePane)
pub fn glass_shards_dice_pane(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, f0: &crate::shards::GlassFragmentLike) { todo!() }

/// (src/game/shards.ts:313 GlassShards.plates)
pub fn glass_shards_plates(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike, n: V3) -> f64 { todo!() }

/// (src/game/shards.ts:337 GlassShards.cutPlates)
pub fn glass_shards_cut_plates(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike, n: V3, fr: crate::shards::GlassShardsCutPlatesFr) -> f64 { todo!() }

/// (src/game/shards.ts:357 GlassShards.plate)
pub fn glass_shards_plate(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64, meta: &crate::shards::PlateMeta, cut: Option<&crate::shards::GlassShardsPlateCut>) -> Option<DebrisChunkId> { todo!() }

/// (src/game/shards.ts:377 GlassShards.crumble)
pub fn glass_shards_crumble(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:381 GlassShards.crumbs)
pub fn glass_shards_crumbs(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:390 GlassShards.evicted)
pub fn glass_shards_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:391 GlassShards.gone)
pub fn glass_shards_gone(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:397 GlassShards.landed)
pub fn glass_shards_landed(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) -> bool { todo!() }

/// (src/game/shards.ts:409 GlassShards.rested)
pub fn glass_shards_rested(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:419 GlassShards.shed)
pub fn glass_shards_shed(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/shards.ts:435 GlassShards.shedCut)
pub fn glass_shards_shed_cut(g: &mut crate::game::Game, c_2: DebrisChunkId, n: f64) { todo!() }

/// (src/game/shards.ts:450 GlassShards.shatter)
pub fn glass_shards_shatter(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) { todo!() }

/// (src/game/shards.ts:510 GlassShards.crossed)
pub fn glass_shards_crossed(g: &mut crate::game::Game, x: crate::shards::PaneCrossLike, who: f64) -> bool { todo!() }

/// (src/game/shards.ts:535 GlassShards.budget)
pub fn glass_shards_budget(g: &mut crate::game::Game, want: f64) -> f64 { todo!() }

/// (src/game/shards.ts:548 GlassShards.emit)
pub fn glass_shards_emit(g: &mut crate::game::Game, pos: V3, dir: V3, count: f64, o: crate::shards::SprayOpts, seed: f64) { todo!() }

/// (src/game/shards.ts:555 GlassShards.send)
pub fn glass_shards_send(g: &mut crate::game::Game, pos: V3, dir: V3, count: f64, o: crate::shards::SprayOpts, seed: f64) { todo!() }

/// (src/game/shards.ts:570 GlassShards.fallOf)
pub fn glass_shards_fall_of(g: &mut crate::game::Game, c_2: V3, v: V3) -> crate::shards::GlassShardsFallOf { todo!() }

/// (src/game/shards.ts:571 GlassShards.schedule)
pub fn glass_shards_schedule(g: &mut crate::game::Game, fall: crate::shards::GlassShardsScheduleFall, name: crate::sim_fallshome::FallSound, gain: f64, rate: f64, amount: f64, who: f64, event: Option<bool>) { todo!() }

/// (src/game/shards.ts:577 GlassShards.landing)
pub fn glass_shards_landing(g: &mut crate::game::Game, fall: crate::shards::GlassShardsLandingFall, gain: f64, rate: f64, amount: f64, who: f64) { todo!() }

/// (src/game/shards.ts:580 GlassShards.sound)
pub fn glass_shards_sound(g: &mut crate::game::Game, name: &str, pos: V3, gain: f64, rate: f64) { todo!() }

/// (src/game/shards.ts:592 GlassShards.heard)
pub fn glass_shards_heard(g: &mut crate::game::Game, p: V3, level: f64, who: f64) { todo!() }

/// (src/game/shards.ts:597 GlassShards.deposit)
pub fn glass_shards_deposit(g: &mut crate::game::Game, p: V3, amount: f64) { todo!() }

/// (src/game/shards.ts:601 GlassShards.underfoot)
pub fn glass_shards_underfoot(g: &mut crate::game::Game, pos: V3) -> f64 { todo!() }

/// (src/game/shards.ts:604 GlassShards.step)
pub fn glass_shards_step(g: &mut crate::game::Game, pos: V3, level: f64) -> f64 { todo!() }

/// (src/game/shards.ts:615 GlassShards.update)
pub fn glass_shards_update(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/shards.ts:645 GlassShards.onRelease)
pub fn glass_shards_on_release(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike) { todo!() }

/// (src/game/shards.ts:647 GlassShards.reset)
pub fn glass_shards_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/shards.ts:650 GlassShards.clear)
pub fn glass_shards_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/shards.ts:651 GlassShards.live)
pub fn glass_shards_live(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/shards.ts:652 GlassShards.plateCount)
pub fn glass_shards_plate_count(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/shards.ts:653 GlassShards.note)
pub fn glass_shards_note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/shards.ts:654 GlassShards.summary)
pub fn glass_shards_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/shards.ts:661 GlassShards.underAlbedo)
pub fn glass_shards_under_albedo(g: &mut crate::game::Game, p: V3) -> V3 { todo!() }

/// (src/game/shards.ts:667 GlassShards.test)
pub fn glass_shards_test(g: &mut crate::game::Game, at: V3, dir: Option<V3>, area: Option<f64>, kind: Option<crate::shards::GlassShardsTestKind>, type_: Option<crate::glasspen::PaneType>) -> bool { todo!() }

/// (src/game/shards.ts:675 clampI)
pub fn clamp_i(x: f64, lo: f64, hi: f64) -> f64 { todo!() }

/// (src/game/shards.ts:678 edgeColour)
pub fn edge_colour(tint: V3, k: f64) -> V3 { todo!() }

/// (src/game/shards.ts:681 cutFaceAlbedo)
pub fn cut_face_albedo(g: &mut crate::game::Game, tint: V3, under: V3, type_: crate::glasspen::PaneType, resting: bool) -> V3 { todo!() }

/// (src/game/shards.ts:688 plateAlbedo)
pub fn plate_albedo(tint: V3, under: V3, type_: crate::glasspen::PaneType) -> V3 { todo!() }

/// (src/game/shards.ts:693 frameOf)
pub fn frame_of(outline: &[V3], centre: V3, n: V3) -> crate::shards::GlassShardsCutPlatesFr { todo!() }

/// (src/game/shards.ts:706 polyArea)
pub fn poly_area(p: &[[f64; 2]]) -> f64 { todo!() }

/// (src/game/shards.ts:708 tidyPoly)
pub fn tidy_poly(p: &[[f64; 2]]) -> Vec<[f64; 2]> { todo!() }

/// (src/game/shards.ts:719 polyCentroid)
pub fn poly_centroid(p: &[[f64; 2]]) -> [f64; 2] { todo!() }

/// (src/game/shards.ts:725 clipHalf)
pub fn clip_half(p: &[[f64; 2]], nx: f64, ny: f64, d: f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/shards.ts:737 cellParts)
pub fn cell_parts(poly: &[[f64; 2]], half_t: f64, seed: f64) -> Vec<Vec<[f64; 2]>> { todo!() }

/// (src/game/shards.ts:747 cutPlateBox)
pub fn cut_plate_box(g: &mut crate::game::Game, origin: V3, rot: Quat, poly: &[[f64; 2]], half_t: f64, albedo: V3) -> Option<crate::shards::CutPlateBox> { todo!() }

/// (src/game/shards.ts:754 signedArea)
pub fn signed_area(p: &[[f64; 2]]) -> f64 { todo!() }

/// (src/game/shards.ts:756 fragmentOfBox)
pub fn fragment_of_box(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64, seed: f64) -> crate::shards::GlassFragmentLike { todo!() }
