#![allow(unused_variables)]
//! Every function and method of `camera`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/camera.ts:33 FollowCamera.setPin)
pub fn follow_camera_set_pin(g: &mut crate::game::Game, p: Option<crate::camera::FollowCameraSetPinP>) -> Option<crate::camera::FollowCameraPin> { todo!() }

/// (src/game/camera.ts:46 FollowCamera.update)
pub fn follow_camera_update(g: &mut crate::game::Game, dt: f64, follow: V3, aim_point: Option<V3>) { todo!() }

/// (src/game/camera.ts:49 FollowCamera.frame)
pub fn follow_camera_frame(g: &mut crate::game::Game) { todo!() }

/// (src/game/camera.ts:50 FollowCamera.step)
pub fn follow_camera_step(g: &mut crate::game::Game, dt: f64, follow: V3, aim_point: Option<V3>) { todo!() }

/// (src/game/camera.ts:68 FollowCamera.focus)
pub fn follow_camera_focus(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/camera.ts:69 FollowCamera.kickIn)
pub fn follow_camera_kick_in(g: &mut crate::game::Game) { todo!() }

/// (src/game/camera.ts:70 FollowCamera.kickOut)
pub fn follow_camera_kick_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/camera.ts:76 FollowCamera.drawIn)
pub fn follow_camera_draw_in(g: &mut crate::game::Game, p: crate::camera::FollowCameraDrawInP) { todo!() }

/// (src/game/camera.ts:81 FollowCamera.drawOut)
pub fn follow_camera_draw_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/camera.ts:87 FollowCamera.rebuild)
pub fn follow_camera_rebuild(g: &mut crate::game::Game) { todo!() }

/// (src/game/camera.ts:103 FollowCamera.autoYaw)
pub fn follow_camera_auto_yaw(g: &mut crate::game::Game, dt: f64, target: V3, blocked: &mut dyn FnMut(&mut crate::game::Game, V3, V3) -> bool) { todo!() }

/// (src/game/camera.ts:117 FollowCamera.easeYaw)
pub fn follow_camera_ease_yaw(g: &mut crate::game::Game, dt: f64, on: bool) { todo!() }

/// (src/game/camera.ts:118 FollowCamera.zoom)
pub fn follow_camera_zoom(g: &mut crate::game::Game, delta: f64) { todo!() }

/// (src/game/camera.ts:119 FollowCamera.rotate)
pub fn follow_camera_rotate(g: &mut crate::game::Game, d_yaw: f64) { todo!() }

/// (src/game/camera.ts:126 FollowCamera.project)
pub fn follow_camera_project(g: &mut crate::game::Game, p: V3, w: f64, h: f64, margin: Option<f64>, min_w: Option<f64>) -> (f64, f64, bool) { todo!() }

/// (src/game/camera.ts:135 FollowCamera.ray)
pub fn follow_camera_ray(g: &mut crate::game::Game, u: f64, v: f64) -> crate::camera::FollowCameraRay { todo!() }

/// (src/game/camera.ts:142 FollowCamera.groundPoint)
pub fn follow_camera_ground_point(g: &mut crate::game::Game, u: f64, v: f64, h: Option<f64>) -> Option<V3> { todo!() }

/// (src/game/camera.ts:149 FollowCamera.planarBasis)
pub fn follow_camera_planar_basis(g: &mut crate::game::Game) -> crate::camera::FollowCameraPlanarBasis { todo!() }
