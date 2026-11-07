//! What the parts of `lesson` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub mod flags {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/lesson.ts:135 FLAGS.wall)
    pub fn wall(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:138 FLAGS.nv)
    pub fn nv(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:147 FLAGS.setup)
    pub fn setup(g: &mut crate::game::Game) -> bool { todo!() }

}
