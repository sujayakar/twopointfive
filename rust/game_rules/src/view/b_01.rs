//! THE VIEW: what a machine is shown of the game beside what it draws, asked of the game and never kept by it.
//!
//! `tick_view` takes the game SHARED, so it cannot settle, poke or file anything: asking it or not asking it is the same run. It depends on the game as
//! its last tick left it and on nothing of the host's (no cursor, no camera, no clock of a machine), so a host asks once a tick and keeps the answer.
//! It is about the seat it is HANDED (`SeatRef`): any seat of the game, standing on it or not, so a machine asks about the one it shows.
//!
//! A SECTION IS FILLED BY ITS RULE'S OWN MODULE (`mission::objective`), by the functions the tick runs: nothing is worded or judged twice. No rule calls
//! this module. A new section: its struct in game_types, its line here, its type on the wire (tools/wire), the HUD's read of it, and the page's own
//! TypeScript for it deleted in the same commit.
use super::*;
use crate::game::Game;

/// the view about `seat`, as the last tick left the game
pub fn tick_view(g: &Game, seat: &crate::seats::SeatRef) -> TickView {
    let me = &g.ch[g.pl[seat.player].ch];
    TickView {
        mission: crate::mission::objective(g, seat),
        radio: crate::cues::radio_rows(g),
        bubbles: crate::cues::bubbles_view(g, seat),
        me: me.id,
        place: crate::chronicle::room_of(g, me.pos),
    }
}
