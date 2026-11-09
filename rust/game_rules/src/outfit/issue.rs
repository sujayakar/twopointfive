use crate::game::Game;

/// The night breach's staging, right after kitStage: the mission's own wall charges into the bag; the count now carried, `None` = not this level's rule.   (issue.ts issueStage)
pub fn issue_stage(g: &mut crate::game::Game) -> Option<f64> {
    let n = match g.staged.walls {
        Some(n) if n > 0.0 => n,
        _ => return None,
    };
    crate::wallcharge::wall_charge_raise_carried(g, n);
    g.wall_charge.carried
}
