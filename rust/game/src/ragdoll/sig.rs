#![allow(unused_variables)]
//! Every function and method of `ragdoll`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/ragdoll.ts:84 Ragdoll.constructor)
pub fn ragdoll_new(g: &mut crate::game::Game, pts: &[V3], vel: Option<&[V3]>, world: Option<&crate::ragdoll::RagdollWorld>) -> RagdollId { todo!() }

/// (src/game/ragdoll.ts:113 Ragdoll.dist)
pub fn ragdoll_dist(g: &mut crate::game::Game, id: RagdollId, i: f64, j: f64) -> f64 { todo!() }

/// (src/game/ragdoll.ts:115 Ragdoll.point)
pub fn ragdoll_point(g: &mut crate::game::Game, id: RagdollId, i: f64, out: Option<&mut V3>) -> V3 { todo!() }

/// (src/game/ragdoll.ts:116 Ragdoll.radius)
pub fn ragdoll_radius(g: &mut crate::game::Game, id: RagdollId, i: f64) -> f64 { todo!() }

/// (src/game/ragdoll.ts:118 Ragdoll.addVelocity)
pub fn ragdoll_add_velocity(g: &mut crate::game::Game, id: RagdollId, i: f64, v: V3, scale: Option<f64>) { todo!() }

/// (src/game/ragdoll.ts:121 Ragdoll.addRigidVelocity)
pub fn ragdoll_add_rigid_velocity(g: &mut crate::game::Game, id: RagdollId, v: V3, omega: V3) { todo!() }

/// (src/game/ragdoll.ts:130 Ragdoll.nudge)
pub fn ragdoll_nudge(g: &mut crate::game::Game, id: RagdollId, i: f64, dx: f64, dz: f64) { todo!() }

/// (src/game/ragdoll.ts:133 Ragdoll.wake)
pub fn ragdoll_wake(g: &mut crate::game::Game, id: RagdollId) { todo!() }

/// (src/game/ragdoll.ts:135 Ragdoll.setPin)
pub fn ragdoll_set_pin(g: &mut crate::game::Game, id: RagdollId, i: f64, target: V3) { todo!() }

/// (src/game/ragdoll.ts:141 Ragdoll.unpin)
pub fn ragdoll_unpin(g: &mut crate::game::Game, id: RagdollId, i: f64) { todo!() }

/// (src/game/ragdoll.ts:149 Ragdoll.isPinned)
pub fn ragdoll_is_pinned(g: &mut crate::game::Game, id: RagdollId, i: f64) -> bool { todo!() }

/// (src/game/ragdoll.ts:150 Ragdoll.clearPins)
pub fn ragdoll_clear_pins(g: &mut crate::game::Game, id: RagdollId) { todo!() }

/// (src/game/ragdoll.ts:156 Ragdoll.ride)
pub fn ragdoll_ride(g: &mut crate::game::Game, id: RagdollId, about: V3, dx: f64, dz: f64, dyaw: f64) { todo!() }

/// (src/game/ragdoll.ts:161 Ragdoll.rideBy)
pub fn ragdoll_ride_by(g: &mut crate::game::Game, id: RagdollId, k: f64) { todo!() }

/// (src/game/ragdoll.ts:178 Ragdoll.settleAt)
pub fn ragdoll_settle_at(g: &mut crate::game::Game, id: RagdollId, xs: &[f64], rot: Option<&[f64]>, own: Option<bool>) { todo!() }

/// (src/game/ragdoll.ts:199 Ragdoll.takeImpact)
pub fn ragdoll_take_impact(g: &mut crate::game::Game, id: RagdollId, min: Option<f64>, gap: Option<f64>) -> f64 { todo!() }

/// (src/game/ragdoll.ts:202 Ragdoll.step)
pub fn ragdoll_step(g: &mut crate::game::Game, id: RagdollId, dt: f64) { todo!() }

/// (src/game/ragdoll.ts:214 Ragdoll.substep)
pub fn ragdoll_substep(g: &mut crate::game::Game, id: RagdollId) { todo!() }

/// (src/game/ragdoll.ts:273 Ragdoll.distance)
pub fn ragdoll_distance(g: &mut crate::game::Game, id: RagdollId, i: f64, j: f64, lo: f64, hi: f64) { todo!() }

/// (src/game/ragdoll.ts:283 Ragdoll.apart)
pub fn ragdoll_apart(g: &mut crate::game::Game, id: RagdollId, i: f64, j: f64, lo: f64) { todo!() }

/// (src/game/ragdoll.ts:292 Ragdoll.cone)
pub fn ragdoll_cone(g: &mut crate::game::Game, id: RagdollId, i: f64, j: f64, axis0: V3, cos_max: f64) { todo!() }

/// (src/game/ragdoll.ts:306 Ragdoll.knee)
pub fn ragdoll_knee(g: &mut crate::game::Game, id: RagdollId, hi: f64, ki: f64, ai: f64) { todo!() }

/// (src/game/ragdoll.ts:320 Ragdoll.shapeMatch)
pub fn ragdoll_shape_match(g: &mut crate::game::Game, id: RagdollId) { todo!() }

/// (src/game/ragdoll.ts:338 Ragdoll.gatherNear)
pub fn ragdoll_gather_near(g: &mut crate::game::Game, id: RagdollId) { todo!() }

/// (src/game/ragdoll.ts:361 Ragdoll.buried)
pub fn ragdoll_buried(g: &mut crate::game::Game, id: RagdollId, i: f64) -> bool { todo!() }

/// (src/game/ragdoll.ts:371 Ragdoll.collide)
pub fn ragdoll_collide(g: &mut crate::game::Game, id: RagdollId, i: f64) { todo!() }

/// (src/game/ragdoll.ts:378 Ragdoll.contact)
pub fn ragdoll_contact(g: &mut crate::game::Game, id: RagdollId, i: f64, nx: f64, ny: f64, nz: f64, pen: f64) { todo!() }

/// (src/game/ragdoll.ts:384 Ragdoll.sphereBox)
pub fn ragdoll_sphere_box(g: &mut crate::game::Game, id: RagdollId, i: f64, r: f64, b: &[f64], q: f64, c_2: f64, s: f64) { todo!() }

/// (src/game/ragdoll.ts:413 extractRotation)
pub fn extract_rotation(a: &[f64], q: &mut Quat, iters: f64) { todo!() }

/// (src/game/ragdoll.ts:431 quatToMat)
pub fn quat_to_mat(q: Quat, out: &mut Vec<f64>) { todo!() }
