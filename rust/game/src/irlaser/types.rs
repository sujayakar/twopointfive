//! The types of `irlaser`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaserBand {
    Visible,
    Ir,
}
impl LaserBand {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaserBand::Visible => "visible",
            LaserBand::Ir => "ir",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct IrlaserMod {
    /// (src/game/irlaser.ts:73 viewTube)
    pub view_tube: Option<bool>,
    /// (src/game/irlaser.ts:77 IR_MADE)
    pub ir_made: JsSet<BoxId>,
}
impl IrlaserMod {
    pub fn blank() -> Self {
        IrlaserMod {
            view_tube: None,
            ir_made: JsSet::new(),
        }
    }
}

/// (src/game/irlaser.ts:62 BREAK)
pub const BREAK: bool = false;

/// (src/game/irlaser.ts:65 IR_TINT: a table nothing writes)
pub fn ir_tint() -> V3 { [0.2577, 0.2577, 0.2577] }
