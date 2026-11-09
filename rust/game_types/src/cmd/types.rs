//! The types of `cmd`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, ToBytes)]
pub struct Seat {
    pub index: f64,
    /// (walked by its own parts: in none of the row's bytes)
    #[bytes(skip)]
    pub own: Option<crate::seats::SeatSet>,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub input: InputId,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub cam: CameraId,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub canvas: crate::host_hud::View,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub cmd: Option<crate::cmd::PlayerCmd>,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub remote: bool,
    pub fixed: f64,
    pub fixed_cmds: f64,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub mask: f64,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub waiting: Option<crate::cmd::PlayerCmd>,
    /// (a host's hand, which differs by machine: in no bytes)
    #[bytes(skip)]
    pub mark: u32,
}
impl Seat {
    /// what a new one holds
    pub fn blank() -> Self {
        Seat {
            index: 0.0,
            own: None,
            input: InputId(0),
            cam: CameraId(0),
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

#[derive(Clone, Debug, Default)]
pub struct CheckCmd {
    pub cmd: crate::cmd::PlayerCmd,
    pub fixed: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckCmdIn {
    pub cmd: crate::cmd::PlayerCmd,
    pub fixed: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct CmdMod {
    /// (src/game/cmd.ts:74 checkLens)
    pub check_lens: Option<CameraId>,
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
