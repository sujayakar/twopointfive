#![allow(unused_imports, unused_variables)]
//! THE ROOM FIGHT: defenders who hold rooms (defender.rs) against a squad (operative.rs), and what both ask: the rooms (rooms.rs), who is in the line of a
//! round (line.rs), the fight as a whole (fight.rs).

pub use ::game_types::roomfight::types;
pub use types::*;
pub mod defender;
pub use defender::*;
pub mod fight;
pub use fight::*;
pub mod line;
pub use line::*;
pub mod operative;
pub use operative::*;
pub mod post;
pub use post::{boots, loose_at, at_post, back_on, def_of, fights_as_he_goes, is_loose, loose, on_leg, on_leg_of, run, send, Held, Leg};
pub mod rooms;
pub use rooms::*;
use crate::game::Game;
use crate::ids::*;
use crate::v3::{self, Quat, V3};

/// A DEFENDER'S WATCHED POINT MOVES (the one place it is written in place: roomfight's parts, the enemy squads and the lights-out rule all come
/// here). What his duty has his gun on (`covers`) is that point, so it moves with it; kept as a copy, equal numbers stand for "the same point".
/// His fix is not touched: where he thinks his man is, is his own, and no rule makes the watched point his fix any more.
pub fn def_move_pt(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    let (gd, old) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].pt);
    g.tab.roomfight_def[d].pt = to;
    if g.gd[gd].covers == Some(old) { g.gd[gd].covers = Some(to); }
}
