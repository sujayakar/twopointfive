//! src/game/doorblast.ts lines 494 to 511
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One line for the panel: which doors are off their hinges, and how many torn so far.   (doorblast.ts doorBlastSummary)
pub fn door_blast_summary(g: &mut crate::game::Game) -> String {
    // A copy of the list: door_off and door_slab take `g`, and neither changes the list.
    let list: Vec<DoorId> = g.doors.list.clone();
    let mut off: Vec<String> = Vec::new();
    for d in list {
        if !crate::doors::door_off(g, d) { continue; }
        // `d.slab?.rest ? '' : ' (in the air)'`: no slab at all reads as in the air, as in the TypeScript.
        let rest = match crate::doors::door_slab(g, d) { Some(s) => s.rest, None => false };
        off.push(alloc::format!("{}{}", g.dr[d].def.name, if rest { "" } else { " (in the air)" }));
    }
    let names = if off.is_empty() { String::from("none") } else { off.join(" · ") };
    alloc::format!("off their hinges ▸ {} · {} torn so far", names, g.stats.doorblast.doorblast_torn)
}
