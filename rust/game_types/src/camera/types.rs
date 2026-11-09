//! The types of `camera`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Blank)]
pub struct FollowCamera {
    #[is([10.0, 0.0, 15.0])] pub target: V3,
    #[is([10.0, 0.0, 15.0])] pub desired: V3,
    #[is(-0.3839724354387525)] pub yaw: f64,
    #[is(0.9599310885968813)] pub pitch: f64,
    #[is(21.0)] pub distance: f64,
    #[is(21.0)] pub desired_distance: f64,
    #[is(0.4886921905584123)] pub fov: f64,
    #[is(0.5)] pub near: f64,
    #[is(120.0)] pub far: f64,
    #[is(0.18)] pub aim_lead: f64,
    #[is(crate::M4_IDENTITY)] pub view: crate::v3::m4::Mat4,
    #[is(crate::M4_IDENTITY)] pub proj: crate::v3::m4::Mat4,
    #[is(crate::M4_IDENTITY)] pub view_proj: crate::v3::m4::Mat4,
    #[is(crate::M4_IDENTITY)] pub inv_view_proj: crate::v3::m4::Mat4,
    #[is(crate::M4_IDENTITY)] pub prev_view_proj: crate::v3::m4::Mat4,
    pub pos: V3,
    #[is([0.0, 0.0, -1.0])] pub forward: V3,
    #[is(1.7777777777777777)] pub aspect: f64,
    pub peek_offset: Option<V3>,
    pub pin: Option<crate::camera::FollowCameraPin>,
    pub before_pin: Option<crate::camera::FollowCameraBeforePin>,
    pub kick: Option<V3>,
    pub kick_held: Option<V3>,
    pub kick_drawn: Option<crate::v3::m4::Mat4>,
    #[is(crate::M4_IDENTITY)] pub kick_vp: crate::v3::m4::Mat4,
    pub draw_held: Option<crate::camera::FollowCameraDrawHeld>,
    #[is(crate::M4_IDENTITY)] pub draw_vp: crate::v3::m4::Mat4,
    pub auto_yaw_t: f64,
    pub yaw_goal: Option<f64>,
    /// INVENTED: bumped by every whole write of desired (game.ts:1614, :1626), for Seat.takeCut (cmd.ts:191)
    pub desired_serial: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraPin {
    pub look: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraBeforePin {
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
    pub desired_distance: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraDrawHeld {
    pub target: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraSetPinP {
    pub look: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraDrawInP {
    pub target: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraRay {
    pub ro: V3,
    pub rd: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FollowCameraPlanarBasis {
    pub fwd: V3,
    pub right: V3,
}

/// (src/game/camera.ts:5 PIN_PITCH_MAX)
pub const PIN_PITCH_MAX: f64 = 1.5533430342749532;
