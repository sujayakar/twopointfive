//! THE VIEW: what a machine is shown of the game beside what it draws, asked of the game and never kept by it.
//!
//! `tick_view` takes the game SHARED, so it cannot settle, poke or file anything: asking it or not asking it is the same run. It depends on the game as
//! its last tick left it and on nothing of the host's (no cursor, no camera, no clock of a machine), so a host asks once a tick and keeps the answer.
//! It is the view of the seat that STANDS in the game between two ticks, which is the seat the machine shows (`seats::show_seat`).
//!
//! A SECTION IS FILLED BY ITS RULE'S OWN MODULE (`mission::objective`), by the functions the tick runs: nothing is worded or judged twice. No rule calls
//! this module. A new section: its struct in game_types, its line here, its type on the wire (tools/wire), the HUD's read of it, and the page's own
//! TypeScript for it deleted in the same commit.
use super::*;
use crate::game::Game;

/// the view of the shown seat, as the last tick left the game
pub fn tick_view(g: &Game) -> TickView {
    TickView { mission: crate::mission::objective(g) }
}
