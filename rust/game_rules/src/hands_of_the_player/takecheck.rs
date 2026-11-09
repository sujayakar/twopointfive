use crate::game::Game;

/// Is somebody told that the thing is gone (so that a man is sent to look at its holder)?
pub fn take_told(g: &mut crate::game::Game) -> bool {
    if !g.quiet.active { return true; }
    let spot = crate::quiet::quiet_mission_spot(g);
    spot.map_or(false, |s| s.wired.unwrap_or(false))
}
