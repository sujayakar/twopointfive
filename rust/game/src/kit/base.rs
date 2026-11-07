//! What the parts of `kit` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub mod slot_amount {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/kit.ts:256 SLOT_AMOUNT.cams)
    pub fn cams(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

}

pub mod slot_put {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/kit.ts:263 SLOT_PUT.cams)
    pub fn cams(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

}
