//! The types of `look_decals`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct LookDecalsMod {
    /// (src/game/lookDecals.ts:3 LOOK_DECALS)
    pub look_decals: JsSet<crate::ObjKey>,
}
impl LookDecalsMod {
    pub fn blank() -> Self {
        LookDecalsMod {
            look_decals: JsSet::new(),
        }
    }
}
