//! CAUGHT IN THE OPEN UNDER FIRE (rust/game_rules/src/people/suppression.rs, control/open.rs): a man gets his head down only where that hides him.
use game::game::GuardState;
use game::ids::GuardId;
use game::teams::Team;
use game_start::play::{Opts, Play};

/// what a man in plain view of Sam does while rounds have him pinned: the seconds he is down in that view, the rounds he fires, whether he went for cover, and the rounds of those he fired as he went
#[derive(Debug, Default)]
struct Under { down: f64, fired: f64, went: bool, as_he_went: f64 }

fn under(p: &mut Play, m: GuardId) -> Option<Under> {
    let at = p.at(m);
    let put = (0..16).map(|k| k as f64 * std::f64::consts::TAU / 16.0).map(|a| [at[0] + 6.0 * a.sin(), at[1] + 6.0 * a.cos()]).find(|q| {
        !game::scene_collision::nav_grid_is_blocked(&p.g, q[0], q[1]) && !game::scene_collision::segment_blocked(&mut p.g, [q[0], 1.45, q[1]], [at[0], 0.7, at[1]], None, None, None)
    })?;
    p.sam_put(put);
    p.sam_unhurt();
    p.pin_man(m, at, (put[0] - at[0]).atan2(put[1] - at[1]), GuardState::Alert, 1.0);
    p.g.gd[m].last_known = Some([put[0], 0.0, put[1]]);
    if p.until(4.0, |p| p.g.gd[m].foe.is_some() && p.rounds_fired(m) > 0.0).is_none() { println!("no fight: {} foe {:?} state {:?} {}", p.g.gd[m].callsign, p.g.gd[m].foe, p.state(m), p.why(m)); return None; }
    let ch = p.g.gd[m].ch;
    let mut u = Under::default();
    let mut n = p.rounds_fired(m);
    for _ in 0..150 {
        let (sam, now) = (p.sam_at(), p.time());
        let sup = &mut p.g.cond[ch].sup;
        (sup.v, sup.src, sup.src_t) = (0.9, Some([sam[0], 1.4, sam[1]]), now);
        p.frame();
        if !p.up(m) { return None; }
        let here = p.g.ch[ch].pos;
        let seen = !game::scene_collision::segment_blocked(&mut p.g, [sam[0], 1.45, sam[1]], [here[0], 0.7, here[2]], None, None, None);
        if seen && p.rule(m) == Some("head down under fire") { u.down += 1.0 / 60.0; }
        let going = game::roomfight::fights_as_he_goes(&p.g, m) || p.sent_to_cover(m).is_some();
        u.went |= going;
        let k = p.rounds_fired(m);
        if k > n { u.fired += k - n; if going { u.as_he_went += k - n; } }
        n = k;
    }
    Some(u)
}

/// THE RULE: rounds have a man pinned in plain view of who fires them: he does not squat there with cover to be had. A beat with his head down, and he goes for it, and fires as he goes.
/// A FAILURE MEANS: a man under fire crouches in the open with his gun down in front of the men shooting him (before: 0.9 seconds and more of it, no round fired, nowhere gone).
#[test]
fn a_man_pinned_in_plain_view_does_not_squat_in_it() {
    let (mut n, mut went, mut as_he_went, mut fired) = (0, 0, 0.0, 0.0);
    for seed in 1..=3 {
        for level in ["depot", "pass", "warehouse"] {
            for i in 0..4 {
                let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed, ..Default::default() });
                p.run(0.3);
                let men: Vec<_> = p.g.guards.iter().copied().filter(|&m| p.g.team(m) == Team::Hostile).collect();
                let Some(u) = under(&mut p, men[i]) else { continue };
                n += 1;
                went += u.went as u32;
                as_he_went += u.as_he_went;
                fired += u.fired;
                assert!(!u.went || u.down < 1.0, "seed {seed} {level}: {} is down {:.2} s in plain view of the man who fires on him: {u:?} {}", p.g.gd[men[i]].callsign, u.down, p.why(men[i]));
            }
        }
    }
    println!("{n} men, {went} went for cover, {as_he_went} rounds as they went, {fired} in all");
    assert!(n >= 12 && went >= 4, "{n} men in plain view, {went} of them went for cover");
    assert!(as_he_went >= 3.0, "{as_he_went} rounds fired on the way to cover by {went} men");
}
