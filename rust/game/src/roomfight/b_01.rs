//! THE BRING-UP'S OWN PART of roomfight: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// THE THREE INVENTED HELPERS OF A DEFENDER'S WATCHED POINT (sig.rs's doc lines are their rule). No job listed them; roomfight's own parts keep private
// twins (p_02 / p_03 fix_is_pt, move_pt, cover_pt), and enemysquads p_04, buddy and powerout call these public ones, which were stubs.
/// `gd.lastKnown === D.pt` (roomfight.ts:303)
pub fn def_fix_is_pt(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let def = &g.tab.roomfight_def[d];
    def.pt_stamp == Some(g.gd[def.gd].fix_stamp) && g.gd[def.gd].last_known == Some(def.pt)
}
/// `gd.lastKnown = D.pt`: a new fix only if it was not that object already
pub fn def_fix_to_pt(g: &mut crate::game::Game, d: RoomfightDefId) {
    let own = def_fix_is_pt(g, d);
    let (gd, pt) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].pt);
    g.gd[gd].last_known = Some(pt);
    if !own { g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1); g.tab.roomfight_def[d].pt_stamp = Some(g.gd[gd].fix_stamp); }
}
/// D.pt moved in place: whatever of his IS that object moves with it, with no new fix. His fix, when it is the point (the stamp); and `covers`, which
/// belief.ts:21 makes that very object on the page's path (`gd.covers = pt`, with D.pt handed in): here a copy, so equal bits stand for the same
/// object, as roomfight's own twin of this has it. Every in-place write of a Def's point comes through here or through that twin.
/// STAND-IN by value (`covers`): it belongs to the identity device.
pub fn def_move_pt(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    let same = def_fix_is_pt(g, d);
    let (gd, old) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].pt);
    g.tab.roomfight_def[d].pt = to;
    if same { g.gd[gd].last_known = Some(to); }
    if g.gd[gd].covers == Some(old) { g.gd[gd].covers = Some(to); }
}
