use crate::game::Game;

/// The night breach's staging, right after kitStage: the structure's own wall charges into the bag; the count now carried, `None` = not this level's rule.
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
