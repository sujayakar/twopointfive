//! What the parts of `char_look` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


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
