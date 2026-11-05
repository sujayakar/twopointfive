//! The types of `camera`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FollowCamera, src/game/camera.ts:7)
#[derive(Clone, Debug)]
pub struct FollowCamera {
    pub target: V3,
    pub desired: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
    pub desired_distance: f64,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
    pub aim_lead: f64,
    pub view: crate::v3::m4::Mat4,
    pub proj: crate::v3::m4::Mat4,
    pub view_proj: crate::v3::m4::Mat4,
    pub inv_view_proj: crate::v3::m4::Mat4,
    pub prev_view_proj: crate::v3::m4::Mat4,
    pub pos: V3,
    pub forward: V3,
    pub aspect: f64,
    pub peek_offset: Option<V3>,
    pub pin: Option<crate::camera::FollowCameraPin>,
    pub before_pin: Option<crate::camera::FollowCameraBeforePin>,
    pub host_shift: Option<V3>,
    pub kick: Option<V3>,
    pub kick_held: Option<V3>,
    pub kick_drawn: Option<Vec<f32>>,
    pub kick_vp: crate::v3::m4::Mat4,
    pub draw_held: Option<crate::camera::FollowCameraDrawHeld>,
    pub draw_vp: crate::v3::m4::Mat4,
    pub auto_yaw_t: f64,
    pub yaw_goal: Option<f64>,
    /// INVENTED: bumped by every whole write of desired (game.ts:1614, :1626), for Seat.takeCut (cmd.ts:191)
    pub desired_serial: u32,
}
impl FollowCamera {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        FollowCamera {
            target: [10.0, 0.0, 15.0],
            desired: [10.0, 0.0, 15.0],
            yaw: -0.3839724354387525,
            pitch: 0.9599310885968813,
            distance: 21.0,
            desired_distance: 21.0,
            fov: 0.4886921905584123,
            near: 0.5,
            far: 120.0,
            aim_lead: 0.18,
            view: crate::M4_IDENTITY,
            proj: crate::M4_IDENTITY,
            view_proj: crate::M4_IDENTITY,
            inv_view_proj: crate::M4_IDENTITY,
            prev_view_proj: crate::M4_IDENTITY,
            pos: [0.0, 0.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            aspect: 1.7777777777777777,
            peek_offset: None,
            pin: None,
            before_pin: None,
            host_shift: None,
            kick: None,
            kick_held: None,
            kick_drawn: None,
            kick_vp: crate::M4_IDENTITY,
            draw_held: None,
            draw_vp: crate::M4_IDENTITY,
            auto_yaw_t: 0.0,
            yaw_goal: None,
            desired_serial: 0,
        }
    }
}
impl Default for FollowCamera { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/camera.ts:26 FollowCamera.pin, src/game/camera.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraPin {
    pub look: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}
impl FollowCameraPin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraPin {
            look: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for FollowCameraPin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:28 FollowCamera.beforePin, src/game/camera.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraBeforePin {
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
    pub desired_distance: f64,
}
impl FollowCameraBeforePin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraBeforePin {
            yaw: 0.0,
            pitch: 0.0,
            distance: 0.0,
            desired_distance: 0.0,
        }
    }
}
impl Default for FollowCameraBeforePin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:75 FollowCamera.drawHeld, src/game/camera.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraDrawHeld {
    pub target: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
}
impl FollowCameraDrawHeld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraDrawHeld {
            target: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            distance: 0.0,
        }
    }
}
impl Default for FollowCameraDrawHeld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:33 setPin(p), src/game/camera.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraSetPinP {
    pub look: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}
impl FollowCameraSetPinP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraSetPinP {
            look: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for FollowCameraSetPinP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:76 drawIn(p), src/game/camera.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraDrawInP {
    pub target: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
}
impl FollowCameraDrawInP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraDrawInP {
            target: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            distance: 0.0,
        }
    }
}
impl Default for FollowCameraDrawInP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:135 ray ->, src/game/camera.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraRay {
    pub ro: V3,
    pub rd: V3,
}
impl FollowCameraRay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraRay {
            ro: [0.0; 3],
            rd: [0.0; 3],
        }
    }
}
impl Default for FollowCameraRay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/camera.ts:149 planarBasis ->, src/game/camera.ts:149)
#[derive(Clone, Copy, Debug)]
pub struct FollowCameraPlanarBasis {
    pub fwd: V3,
    pub right: V3,
}
impl FollowCameraPlanarBasis {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FollowCameraPlanarBasis {
            fwd: [0.0; 3],
            right: [0.0; 3],
        }
    }
}
impl Default for FollowCameraPlanarBasis { fn default() -> Self { Self::zeroed() } }

/// (src/game/camera.ts:5 PIN_PITCH_MAX)
pub const PIN_PITCH_MAX: f64 = 1.5533430342749532;
