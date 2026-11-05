#![allow(unused_variables)]
//! Every function and method of `char_look`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/charLook.ts:15 sideOfTeam)
pub fn side_of_team(team: crate::teams::Team, trained: Option<bool>) -> crate::anim_gltf::CastSide { todo!() }

/// (src/game/charLook.ts:34 lookOf)
pub fn look_of<'g>(g: &'g mut crate::game::Game, c_2: CharId) -> &'g mut crate::char_look::Look { todo!() }

/// (src/game/charLook.ts:56 charSides)
pub fn char_sides(g: &mut crate::game::Game) -> crate::FnField { todo!() }

/// (src/game/charLook.ts:63 charLookFrame)
pub fn char_look_frame(g: &mut crate::game::Game) { todo!() }

pub mod frame {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/charLook.ts:45 FRAME.palette)
    pub fn palette(g: &mut crate::game::Game, c_2: CharId) -> Vec<f32> { todo!() }

    /// (src/game/charLook.ts:46 FRAME.side)
    pub fn side(g: &mut crate::game::Game, c_2: CharId) -> crate::anim_gltf::CastSide { todo!() }

    /// (src/game/charLook.ts:47 FRAME.proxies)
    pub fn proxies(g: &mut crate::game::Game, c_2: CharId, wet_k: f64, whole: bool) -> Vec<V3> { todo!() }

}
