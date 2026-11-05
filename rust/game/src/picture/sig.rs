#![allow(unused_variables)]
//! Every function and method of `picture`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/picture.ts:119 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::picture::State { todo!() }

/// (src/game/picture.ts:120 picOf)
pub fn pic_of<'g>(g: &'g mut crate::game::Game, gd: GuardId, now: f64) -> &'g mut crate::picture::Pic { todo!() }

/// (src/game/picture.ts:122 pictureReset)
pub fn picture_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/picture.ts:124 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/picture.ts:125 floor)
pub fn floor(p: V3) -> V3 { todo!() }

/// (src/game/picture.ts:126 nameOf)
pub fn name_of(g: &mut crate::game::Game, w: crate::picture::Who) -> String { todo!() }

/// (src/game/picture.ts:127 up)
pub fn up(g: &mut crate::game::Game, w: crate::picture::Who) -> bool { todo!() }

/// (src/game/picture.ts:128 posOf)
pub fn pos_of(g: &mut crate::game::Game, w: crate::picture::Who) -> V3 { todo!() }

/// (src/game/picture.ts:130 gone)
pub fn gone(g: &mut crate::game::Game, w: crate::picture::Who) -> bool { todo!() }

/// (src/game/picture.ts:132 madeOut)
pub fn made_out(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/picture.ts:134 fighter)
pub fn fighter(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/picture.ts:136 foesOf)
pub fn foes_of(g: &mut crate::game::Game, team: crate::teams::Team) -> Vec<crate::picture::Who> { todo!() }

/// (src/game/picture.ts:143 learn)
pub fn learn(g: &mut crate::game::Game, p: &mut crate::picture::Pic, who: crate::picture::Who, at: V3, t: f64, how: crate::picture::IdeaHow, by: Option<GuardId>) { todo!() }

/// (src/game/picture.ts:152 look)
pub fn look(g: &mut crate::game::Game, p: &mut crate::picture::Pic, gd: GuardId) { todo!() }

/// (src/game/picture.ts:184 lookSam)
pub fn look_sam(g: &mut crate::game::Game, p: &mut crate::picture::Pic) { todo!() }

/// (src/game/picture.ts:201 hear)
pub fn hear(g: &mut crate::game::Game) { todo!() }

/// (src/game/picture.ts:224 onNet)
pub fn on_net(g: &mut crate::game::Game, a: Option<GuardId>, b: Option<GuardId>) -> bool { todo!() }

/// (src/game/picture.ts:235 tell)
pub fn tell(g: &mut crate::game::Game) { todo!() }

/// (src/game/picture.ts:255 pictureTell)
pub fn picture_tell(g: &mut crate::game::Game, gd: GuardId, who: crate::picture::Who, at: V3, age: Option<f64>) { todo!() }

/// (src/game/picture.ts:257 pictureTick)
pub fn picture_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/picture.ts:274 ideasOf)
pub fn ideas_of(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::picture::Idea> { todo!() }

/// (src/game/picture.ts:276 ideaWeight)
pub fn idea_weight(g: &mut crate::game::Game, i: crate::picture::Idea) -> f64 { todo!() }

/// (src/game/picture.ts:282 ideaReach)
pub fn idea_reach(g: &mut crate::game::Game, i: crate::picture::Idea) -> f64 { todo!() }

/// (src/game/picture.ts:285 maybeOf)
pub fn maybe_of(g: &mut crate::game::Game, i: crate::picture::Idea) -> Vec<V3> { todo!() }

/// (src/game/picture.ts:292 lineTo)
pub fn line_to(g: &mut crate::game::Game, from: V3, p: V3, h: f64) -> bool { todo!() }

/// (src/game/picture.ts:293 exposedTo)
pub fn exposed_to(g: &mut crate::game::Game, i: crate::picture::Idea, p: V3, h: f64) -> f64 { todo!() }

/// (src/game/picture.ts:301 exposedAt)
pub fn exposed_at(g: &mut crate::game::Game, gd: GuardId, p: V3, low: Option<bool>) -> f64 { todo!() }

/// (src/game/picture.ts:309 exposureAsker)
pub fn exposure_asker(g: &mut crate::game::Game, gd: GuardId, low: Option<bool>) -> Option<crate::picture::ExposureAsker> { todo!() }

/// (src/game/picture.ts:322 believedOf)
pub fn believed_of(g: &mut crate::game::Game, gd: GuardId) -> Vec<V3> { todo!() }

/// (src/game/picture.ts:324 pictureQuiet)
pub fn picture_quiet<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/picture.ts:326 exposedFor)
pub fn exposed_for(g: &mut crate::game::Game, men: &[GuardId], p: V3, low: Option<bool>) -> f64 { todo!() }

/// (src/game/picture.ts:331 knownPlaces)
pub fn known_places(g: &mut crate::game::Game, men: &[GuardId], max_age: f64) -> Vec<crate::picture::KnownPlacesItem> { todo!() }

/// (src/game/picture.ts:343 anyIdea)
pub fn any_idea(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/picture.ts:346 safestSets)
pub fn safest_sets(g: &mut crate::game::Game, men: &[GuardId], sets: &[Vec<crate::picture::SafestSetsSetsItemItem>]) -> Vec<Vec<crate::picture::SafestSetsSetsItemItem>> { todo!() }

/// (src/game/picture.ts:354 coverNear)
pub fn cover_near(g: &mut crate::game::Game, gd: GuardId, around: V3, r: f64, n: Option<f64>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>) -> Vec<crate::picture::CoverSpot> { todo!() }

/// (src/game/picture.ts:373 pictureStep)
pub fn picture_step(g: &mut crate::game::Game, gd: GuardId, key: u32, mark: V3, ok: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, busy: Option<bool>) -> Option<crate::picture::PictureStep> { todo!() }

/// (src/game/picture.ts:413 pictureStepOf)
pub fn picture_step_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::picture::PictureStepOf> { todo!() }

/// (src/game/picture.ts:423 pictureShown)
pub fn picture_shown(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/picture.ts:429 viewOf)
pub fn view_of(g: &mut crate::game::Game, gd: GuardId, ground: bool) -> crate::picture::PictureView { todo!() }

/// (src/game/picture.ts:430 viewOf1)
pub fn view_of1(g: &mut crate::game::Game, gd: GuardId, ground: bool) -> crate::picture::PictureView { todo!() }

/// (src/game/picture.ts:439 pictureViews)
pub fn picture_views(g: &mut crate::game::Game) -> Vec<crate::picture::PictureView> { todo!() }

/// (src/game/picture.ts:480 pictureSummary)
pub fn picture_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/picture.ts:493 ideaOf)
pub fn idea_of(g: &mut crate::game::Game, gd: GuardId, who: crate::picture::Who) -> Option<crate::picture::Idea> { todo!() }
