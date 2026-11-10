//! A PLACE TO STAND (rust/game_rules/src/world/places.rs): what every rule asks of floor before it sends a man there.
use game::teams::Team;
use game_start::play::{dist, Opts, Play};

/// THE RULE: no rule sends a man to a place another man is on his way to, whatever rule sent that one.
/// A FAILURE MEANS: a man under fire shifts to the cover a mate, sent by his squad, is already running for, and they stand in each other (before: he asked only who stood
/// there and who else was shifting).
#[test]
fn nobody_is_sent_where_another_is_going() {
    let mut asked = 0;
    for seed in 1..=3 {
        for level in ["office", "depot", "warehouse"] {
            let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed, ..Default::default() });
            p.sam_unhurt();
            p.run(0.5);
            let men: Vec<_> = p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == Team::Hostile).collect();
            for (i, &a) in men.iter().enumerate() {
                let at = p.at(a);
                for k in 0..4 {
                    let w = std::f64::consts::FRAC_PI_2 * k as f64;
                    let threat = [at[0] + 5.0 * w.sin(), 0.0, at[1] + 5.0 * w.cos()];
                    let Some(q) = game::suppression::cover_from(&mut p.g, a, threat) else { continue };
                    let b = men[(i + 1) % men.len()];
                    if dist(p.at(b), [q[0], q[2]]) < 2.0 { continue; }
                    game::squad::order(&mut p.g, b, game::squad::OrderPatch { goal: Some(Some(q)), ..Default::default() });
                    let other = game::suppression::cover_from(&mut p.g, a, threat);
                    asked += 1;
                    assert!(other.is_none_or(|o| dist([o[0], o[2]], [q[0], q[2]]) >= 0.5), "seed {seed} {level}: a man is sent to ({:.1}, {:.1}), where another is going", q[0], q[2]);
                    game::squad::end_guard_script(&mut p.g, b, None);
                }
            }
        }
    }
    assert!(asked >= 20, "only {asked} places asked for");
}
