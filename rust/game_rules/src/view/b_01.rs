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
    TickView { mission: crate::mission::objective(g, seat) }
}

/// what the end card shows of a run, about `seat`: the chronicle's words and counts, the floor's rooms and the recap. Asked as the card opens
pub fn debrief_view(g: &Game, seat: &crate::seats::SeatRef) -> DebriefView {
    let c = &g.chronicle;
    let rows = c.events.iter().map(|e| DebriefRow { t: e.t, kind: e.kind, room: e.room.clone(), pos: e.pos, text: crate::chronicle::event_text(g, e), down: crate::chronicle::alarm_down(e) }).collect();
    let n = c.path_n as usize * 3;
    DebriefView { debrief: crate::chronicle::debrief_of(g, seat), rows, dropped: c.dropped, path: c.path[..n.min(c.path.len())].to_vec(), rooms: crate::chronicle::map_rooms(g), recap: crate::deathrecap::recap_card(g) }
}
