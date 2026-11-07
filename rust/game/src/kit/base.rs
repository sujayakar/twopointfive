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
    /// (src/game/kit.ts:253 SLOT_AMOUNT.smoke)
    pub fn smoke(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.bangs)
    pub fn bangs(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.frags)
    pub fn frags(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.shells)
    pub fn shells(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.charges)
    pub fn charges(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.walls)
    pub fn walls(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:256 SLOT_AMOUNT.cams)
    pub fn cams(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:257 SLOT_AMOUNT.rings)
    pub fn rings(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

}

pub mod slot_put {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/kit.ts:262 SLOT_PUT.smoke)
    pub fn smoke(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.bangs)
    pub fn bangs(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.frags)
    pub fn frags(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.shells)
    pub fn shells(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.charges)
    pub fn charges(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.walls)
    pub fn walls(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.cams)
    pub fn cams(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.rings)
    pub fn rings(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

}
