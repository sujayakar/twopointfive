//! WHAT A HOST IS TOLD OF THE GAME beside what it draws. Reads: no word of the state is written.
use super::Held;
use game::handed::CursorNow;
use game::view::{ActionRow, CursorView, DebriefView, HostNow, KnobRow, RunView, ShownRow, Tally, TallyMan, TickView};
use game_start::host::{actions, field::Laid, knobs, shown};

/// the seat this machine shows
fn shown_seat(x: &Held) -> game::ids::SeatId { x.g.local_seat.unwrap_or(x.g.seat) }
pub fn tick_view(x: &Held) -> TickView { game::view::tick_view(&x.g, &game::seats::seat(&x.g, shown_seat(x))) }
pub fn debrief_view(x: &Held) -> DebriefView { game::view::debrief_view(&x.g, &game::seats::seat(&x.g, shown_seat(x))) }
pub fn run_view(x: &Held) -> RunView { game::verdict::run_view(&x.g) }
pub fn cursor_view(x: &Held, seat: Option<u32>, Laid(c): Laid<CursorNow>) -> CursorView {
    let now = HostNow { mouse: c.mouse, client: c.client, view_proj: c.view_proj.map(|v| v as f32), inv_view_proj: c.inv_view_proj.map(|v| v as f32) };
    let s = seat.and_then(|s| x.g.seats.get(s as usize).copied()).unwrap_or(shown_seat(x));
    game::view::cursor_view(&x.g, &game::seats::seat(&x.g, s), &now)
}
pub fn tally(x: &Held) -> Tally {
    let g = &x.g;
    let s = &g.saves.stats;
    Tally {
        men: g.guards.iter().map(|&gd| TallyMan { state: g.gd[gd].state, lit: g.tab.light[g.gd[gd].flashlight].enabled }).collect(),
        qrf: g.qrf.state, escalation: g.escalation, autosaves: s.autosaves, manual: s.manual, loads: s.loads, stage: g.mission.stage, alarm: g.mission.alarm_on, alerts: g.mission.stats.alerts,
        clear: game::roomclear::room_clear_scenario_active(g), kind: g.verdict.res.as_ref().and_then(|r| r.kind), stats: g.mission.stats.clone(),
    }
}
pub fn knobs(x: &Held) -> Vec<KnobRow> { knobs::all(&x.g).into_iter().map(|(name, yes, value)| KnobRow { name, yes, value }).collect() }
pub fn shown() -> Vec<ShownRow> { shown::ROWS.iter().map(shown::Row::told).collect() }
pub fn shown_now(x: &Held) -> Vec<f64> { shown::ROWS.iter().map(|r| knobs::get(&x.g, r.name).unwrap_or(0.0) * r.scale).collect() }
pub fn actions(x: &Held) -> Vec<ActionRow> { actions::ACTIONS.iter().map(|a| a.told(&x.g)).collect() }
