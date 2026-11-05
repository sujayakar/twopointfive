#![allow(unused_variables)]
//! Every function and method of `glass_pattern`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glassPattern.ts:55 gpHash)
pub fn gp_hash(seed: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:61 gpJit)
pub fn gp_jit(seed: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:62 wrap)
pub fn wrap(k: f64, n: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:64 gpBound)
pub fn gp_bound(s: crate::glass_pattern::GpSite, k: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:65 gpAngle)
pub fn gp_angle(s: crate::glass_pattern::GpSite, k: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:67 gpRho)
pub fn gp_rho(s: crate::glass_pattern::GpSite, k: f64, b: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:68 gpChordPt)
pub fn gp_chord_pt(s: crate::glass_pattern::GpSite, k: f64, b: f64) -> [f64; 2] { todo!() }

/// (src/game/glassPattern.ts:70 gpReach)
pub fn gp_reach(s: crate::glass_pattern::GpSite, k: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:72 gpMerged)
pub fn gp_merged(s: crate::glass_pattern::GpSite, s_2: f64) -> bool { todo!() }

/// (src/game/glassPattern.ts:74 gpSplit)
pub fn gp_split(s: crate::glass_pattern::GpSite, s_2: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:76 gpCut)
pub fn gp_cut(s: crate::glass_pattern::GpSite, s_2: f64, h: f64) -> Option<crate::glass_pattern::GpCut> { todo!() }

/// (src/game/glassPattern.ts:85 gpNearest)
pub fn gp_nearest(sites: &[crate::glass_pattern::GpSite], u: f64, v: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:90 beyond)
pub fn beyond(a: [f64; 2], b: [f64; 2], u: f64, v: f64) -> bool { todo!() }

/// (src/game/glassPattern.ts:92 gpSector)
pub fn gp_sector(s: crate::glass_pattern::GpSite, u: f64, v: f64) -> crate::glass_pattern::GpSector { todo!() }

/// (src/game/glassPattern.ts:100 gpSectorBand)
pub fn gp_sector_band(s: crate::glass_pattern::GpSite, u: f64, v: f64) -> crate::glass_pattern::GpSectorBand { todo!() }

/// (src/game/glassPattern.ts:109 gpFid)
pub fn gp_fid(i: f64, s: f64, b: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:110 gpUnfid)
pub fn gp_unfid(fid: f64) -> crate::glass_pattern::GpUnfid { todo!() }

/// (src/game/glassPattern.ts:112 gpClassify)
pub fn gp_classify(sites: &[crate::glass_pattern::GpSite], run: bool, u: f64, v: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:121 clip)
pub fn clip(pts: &[[f64; 2]], tags: &[crate::glass_pattern::Tag], nx: f64, ny: f64, d: f64) -> crate::glass_pattern::Clip { todo!() }

/// (src/game/glassPattern.ts:133 isect)
pub fn isect(a: [f64; 2], b: [f64; 2], nx: f64, ny: f64, d: f64) -> [f64; 2] { todo!() }

/// (src/game/glassPattern.ts:138 rectPoly)
pub fn rect_poly(hu: f64, hv: f64) -> crate::glass_pattern::Poly { todo!() }

/// (src/game/glassPattern.ts:140 gpRegion)
pub fn gp_region(sites: &[crate::glass_pattern::GpSite], i: f64, hu: f64, hv: f64) -> crate::glass_pattern::Poly { todo!() }

/// (src/game/glassPattern.ts:161 gpFragments)
pub fn gp_fragments(sites: &[crate::glass_pattern::GpSite], run: bool, hu: f64, hv: f64) -> Vec<crate::glass_pattern::GpFrag> { todo!() }

/// (src/game/glassPattern.ts:223 lineDist)
pub fn line_dist(a: [f64; 2], b: [f64; 2], u: f64, v: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:228 gpCrackAt)
pub fn gp_crack_at(sites: &[crate::glass_pattern::GpSite], run: bool, web: bool, u: f64, v: f64) -> crate::glass_pattern::GpCrack { todo!() }

/// (src/game/glassPattern.ts:275 gpDiceCentre)
pub fn gp_dice_centre(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> [f64; 2] { todo!() }

/// (src/game/glassPattern.ts:280 gpDiceClump)
pub fn gp_dice_clump(d: crate::glass_pattern::GpDice, u: f64, v: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:289 gpDiceKey)
pub fn gp_dice_key(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:294 gpDicePoly)
pub fn gp_dice_poly(d: crate::glass_pattern::GpDice, c_2: f64, r: f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/glassPattern.ts:305 gpDiceEdge)
pub fn gp_dice_edge(u: f64, v: f64, grain: f64, seed: f64) -> f64 { todo!() }

/// (src/game/glassPattern.ts:316 gpHull)
pub fn gp_hull(pts: &[[f64; 2]]) -> Vec<[f64; 2]> { todo!() }

/// (src/game/glassPattern.ts:323 gpClipRect)
pub fn gp_clip_rect(pts: &[[f64; 2]], hu: f64, hv: f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/glassPattern.ts:328 gpPolyArea)
pub fn gp_poly_area(pts: &[[f64; 2]]) -> crate::glass_pattern::GpPolyArea { todo!() }
