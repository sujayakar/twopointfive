//! src/game/issue.ts lines 1 to 20
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The night breach's staging, right after kitStage: the structure's own wall charges into the bag; the count now carried, `None` = not this level's rule.   (issue.ts issueStage)
pub fn issue_stage(g: &mut crate::game::Game) -> Option<f64> {
    let n = if crate::roomclear::room_clear_scenario_active(g) {
        crate::roomclear::room_clear_scenario_structure(g).and_then(|s| s.wall_charges)
    } else {
        None
    };
    let n = match n {
        Some(n) if n > 0.0 => n,
        _ => return None,
    };
    crate::wallcharge::wall_charge_raise_carried(g, n);
    g.wall_charge.carried
}
