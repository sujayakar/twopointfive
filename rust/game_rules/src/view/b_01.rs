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
use alloc::string::String;

/// the view about `seat`, as the last tick left the game
pub fn tick_view(g: &Game, seat: &crate::seats::SeatRef) -> TickView {
    let me = &g.ch[g.pl[seat.player].ch];
    TickView {
        mission: crate::mission::objective(g, seat),
        radio: crate::cues::radio_rows(g),
        bubbles: crate::cues::bubbles_view(g, seat),
        me: me.id,
        place: crate::chronicle::room_of(g, me.pos),
        kit: crate::kit::kit_view(g, seat),
    }
}

/// what the game itself has of a host's now: a seat's cursor and camera as its last step left them (the tick's own questions of the cursor)
pub fn host_now(g: &Game, input: crate::ids::InputId, cam: crate::ids::CameraId, canvas: crate::host_hud::View) -> HostNow {
    let (i, c) = (&g.tab.input[input], &g.tab.camera[cam]);
    HostNow { mouse: [i.mouse_x, i.mouse_y], client: [canvas.client_width, canvas.client_height], view_proj: c.view_proj, inv_view_proj: c.inv_view_proj }
}

/// the view of what `seat`'s cursor is on, through the camera as it stands (the copy of the boxes standing: scene_collision::stands)
pub fn cursor_view(g: &Game, seat: &crate::seats::SeatRef, now: &HostNow) -> CursorView {
    CursorView { under: crate::orders::hover(g, seat, now).map(|t| under(g, &t)) }
}
/// a target in the view's words: a man by his character's id, a door by its name, a wall by its place
pub fn under(g: &Game, t: &crate::orders::OrderTarget) -> Under {
    use crate::orders::OrderTarget as T;
    let id = |gd: crate::ids::GuardId| g.ch[g.gd[gd].ch].id;
    match t {
        T::Floor(f) => Under {
            kind: UnderKind::Floor, at: f.at, thing: f.thing.clone(),
            wall: f.wall.as_ref().map(|w| UnderWall { index: w.index, along_x: w.along_x, u: w.u, side: w.side, ok: w.ok, why: w.why.clone(), outer: w.outer == Some(true) }),
            ..Under::default()
        },
        T::Door(d) => Under { kind: UnderKind::Door, at: d.at, door: Some(g.dr[d.door].def.name.clone()), side: d.side, ..Under::default() },
        T::None(n) => Under { kind: UnderKind::None, at: n.at, man: n.man.map_or(-1.0, id), ..Under::default() },
        T::Foe(f) => Under { kind: UnderKind::Foe, at: f.at, man: id(f.foe), ..Under::default() },
    }
}

/// what the end card shows of a run, about `seat`: the chronicle's words and counts, the floor's rooms and the recap. Asked as the card opens
pub fn debrief_view(g: &Game, seat: &crate::seats::SeatRef) -> DebriefView {
    let c = &g.chronicle;
    let rows = c.events.iter().map(|e| DebriefRow { t: e.t, kind: e.kind, room: e.room.clone(), pos: e.pos, text: crate::chronicle::event_text(g, e), down: crate::chronicle::alarm_down(e) }).collect();
    let n = c.path_n as usize * 3;
    DebriefView { debrief: crate::chronicle::debrief_of(g, seat), rows, dropped: c.dropped, path: c.path[..n.min(c.path.len())].to_vec(), rooms: crate::chronicle::map_rooms(g), recap: crate::deathrecap::recap_card(g) }
}
