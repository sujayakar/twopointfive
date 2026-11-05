//! The types of `cmd`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PlayerCmd, src/game/cmd.ts:25)
#[derive(Clone, Debug)]
pub struct PlayerCmd {
    pub keys: Vec<String>,
    pub pressed: Vec<String>,
    pub buttons: f64,
    pub clicked: f64,
    pub wheel: f64,
    pub mx: f64,
    pub my: f64,
    pub w: f64,
    pub h: f64,
    pub pw: f64,
    pub ph: f64,
    pub view: crate::cmd::PlayerCmdView,
}
impl PlayerCmd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerCmd {
            keys: Vec::new(),
            pressed: Vec::new(),
            buttons: 0.0,
            clicked: 0.0,
            wheel: 0.0,
            mx: 0.0,
            my: 0.0,
            w: 0.0,
            h: 0.0,
            pw: 0.0,
            ph: 0.0,
            view: crate::cmd::PlayerCmdView::zeroed(),
        }
    }
}
impl Default for PlayerCmd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmd.ts:33 PlayerCmd.view, src/game/cmd.ts:33)
#[derive(Clone, Debug)]
pub struct PlayerCmdView {
    pub vp: Vec<f64>,
    pub ivp: Vec<f64>,
    pub pos: V3,
    pub forward: V3,
    pub target: V3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
    pub fov: f64,
    pub aspect: f64,
    pub aim_lead: f64,
}
impl PlayerCmdView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerCmdView {
            vp: Vec::new(),
            ivp: Vec::new(),
            pos: [0.0; 3],
            forward: [0.0; 3],
            target: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            distance: 0.0,
            fov: 0.0,
            aspect: 0.0,
            aim_lead: 0.0,
        }
    }
}
impl Default for PlayerCmdView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Seat, src/game/cmd.ts:146)
#[derive(Clone, Debug)]
pub struct Seat {
    pub index: f64,
    pub own: Option<crate::seats::SeatSet>,
    pub input: InputId,
    pub cam: crate::camera::FollowCamera,
    pub canvas: crate::host_hud::View,
    pub cmd: Option<crate::cmd::PlayerCmd>,
    pub remote: bool,
    pub fixed: f64,
    pub fixed_cmds: f64,
    pub mask: f64,
    pub waiting: Option<crate::cmd::PlayerCmd>,
    pub mark: u32,
}
impl Seat {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Seat {
            index: 0.0,
            own: None,
            input: InputId(0),
            cam: crate::camera::FollowCamera::blank(),
            canvas: crate::host_hud::View { client_width: 0.0, client_height: 0.0, width: 0.0, height: 0.0 },
            cmd: None,
            remote: false,
            fixed: 0.0,
            fixed_cmds: 0.0,
            mask: 0.0,
            waiting: None,
            mark: 0,
        }
    }
}
impl Default for Seat { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/cmd.ts:80 checkCmd ->, src/game/cmd.ts:80)
#[derive(Clone, Debug)]
pub struct CheckCmd {
    pub cmd: crate::cmd::PlayerCmd,
    pub fixed: f64,
}
impl CheckCmd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckCmd {
            cmd: crate::cmd::PlayerCmd::zeroed(),
            fixed: 0.0,
        }
    }
}
impl Default for CheckCmd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmd.ts:83 checkCmdIn ->, src/game/cmd.ts:83)
#[derive(Clone, Debug)]
pub struct CheckCmdIn {
    pub cmd: crate::cmd::PlayerCmd,
    pub fixed: f64,
}
impl CheckCmdIn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckCmdIn {
            cmd: crate::cmd::PlayerCmd::zeroed(),
            fixed: 0.0,
        }
    }
}
impl Default for CheckCmdIn { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CmdMod {
    /// (src/game/cmd.ts:74 checkLens)
    pub check_lens: Option<crate::camera::FollowCamera>,
}
impl CmdMod {
    pub fn blank() -> Self {
        CmdMod {
            check_lens: None,
        }
    }
}

/// (src/game/cmd.ts:73 KEY_CODE)
pub const KEY_CODE: &str = "/^[A-Za-z0-9]+$/";
