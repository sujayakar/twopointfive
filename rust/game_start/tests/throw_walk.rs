//! A THROW YOU HAVE TO WALK TO, AND ONE SHELL BY RANGE.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

/// the office with its four, nobody hostile but one far off, holding; a spot nobody has a throw to and one man can walk to a throw for, the longest such walk
fn a_walked_throw(s: u32) -> (Play, Vec<GuardId>, [f64; 2], GuardId, [f64; 2]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear=11&squad4", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.3);
    let men = p.men();
    let far = p.g.guards.iter().copied().find(|&x| !men.contains(&x)).unwrap();
    p.drop_all_but(&[far]);
    p.pin_man(far, [39.0, 27.0], 0.0, GuardState::Patrol, 0.0);
    let sam = p.sam_at();
    p.order("hold", sam, Who::All);
    p.run(1.0);
    let mut best: Option<([f64; 2], GuardId, [f64; 2], f64)> = None;
    let mut z = 2.25;
    while z <= 26.0 {
        let mut x = 2.25;
        while x <= 38.0 {
            let at = [x, z];
            if p.floor_free(at) && !p.throw_now(&men, at) {
                if let Some((m, post)) = p.throw_after_walk(&men, at) {
                    let d = dist(p.at(m), post);
                    if best.map_or(true, |b| d > b.3) { best = Some((at, m, post, d)); }
                }
            }
            x += 3.0;
        }
        z += 3.0;
    }
    let (at, m, post, _) = best.unwrap_or_else(|| panic!("seed {s}: no spot"));
    (p, men, at, m, post)
}

/// THE RULE: a spot nobody has a throw to gets a man who walks: the order is taken, one man goes to the floor he has a throw from, nothing has left a hand and nothing is spent until he is on it; the thing comes to rest by the spot and he goes
/// back to the order he had. Another order mid-walk ends it with nothing spent: that is the control.
/// A FAILURE MEANS: a frag is thrown from a place with no throw, or spent on a walk that never ended in one.
#[test]
fn a_teammate_walks_to_where_he_has_a_throw() {
    for s in SEEDS {
        let (mut p, men, at, _, _) = a_walked_throw(s);
        let frags = p.frags_left();
        assert_eq!(p.try_order("fragThere", at, Who::All), None, "seed {s}");
        p.run(0.1);
        let man = *men.iter().find(|&&m| p.doing(m).as_deref() == Some("fragThere")).unwrap_or_else(|| panic!("seed {s}: nobody on it"));
        let post = p.post_of(man).unwrap_or_else(|| panic!("seed {s}: he has no place to go"));
        assert!(p.frags_out().is_empty() && p.frags_left() == frags, "seed {s}: thrown before he is there");
        let mut far = 9.0f64;
        p.until(25.0, |p| { if p.frags_out().is_empty() { far = dist(p.at(man), post); } !p.frags_out().is_empty() }).unwrap_or_else(|| panic!("seed {s}: he never threw"));
        assert!(far < 0.7 && p.frags_left() == frags - 1.0, "seed {s}: thrown {far:.2} m from his place");
        let rest = p.frags_out()[0];
        p.run(1.5);
        let rest2 = p.frags_out().first().copied().unwrap_or(rest);
        assert!(dist(rest2, at) < 1.3, "seed {s}: it came to rest {:.1} m off", dist(rest2, at));
        p.run(5.0);
        assert_eq!(p.doing(man).as_deref(), Some("hold"), "seed {s}: back to the order he had");
        drop(p);

        let (mut p, men, at, _, _) = a_walked_throw(s);
        let frags = p.frags_left();
        assert_eq!(p.try_order("fragThere", at, Who::All), None, "seed {s}");
        p.run(0.3);
        let sam = p.sam_at();
        p.order("fallIn", sam, Who::All);
        p.run(5.0);
        assert!(p.frags_out().is_empty() && p.frags_left() == frags && men.iter().all(|&m| p.doing(m).as_deref() == Some("fallIn")), "seed {s}: the control: another order mid-walk");
    }
}

/// THE RULE: one shell: any one pellet inside the drop range drops a man, past it only the `need`-th of ONE shell does: a pattern, never a count kept on him. Across a room (2 to 6 m) a shell drops a standing man, near certain; down a corridor (20, 25
/// m) it is a poor bet, and it only falls with range. A man the pattern only caught the edge of is peppered, standing.
/// A FAILURE MEANS: the shotgun is a rifle, or a broom.
#[test]
fn a_shell_is_certain_across_a_room_and_poor_down_a_corridor() {
    let (mut p, ..) = { let mut p = Play::mission("office.json", "?levelFile=office.json", &Opts { seed: 11, ..Default::default() }); p.sam_unhurt(); p.run(0.2); (p, 0) };
    let (_, need, near, far) = p.shotgun_rule();
    assert!(near, "the first pellet inside the range");
    assert!(far[..need as usize - 1].iter().all(|x| !x) && far[need as usize - 1], "past it, pellet by pellet: {far:?}");
    let shooter = p.g.guards.iter().copied().find(|&m| !p.g.team(m).ours()).unwrap();
    let mut lane = None;   // (the office has no 27 m of clear floor indoors: this finds the lot along its south face)
    let mut z = 1.0;
    while z < 28.0 && lane.is_none() {
        let mut x = 0.0;
        while x <= 10.0 {
            if p.floor_free([x + 0.5, z]) && p.floor_free([x + 26.5, z]) && [-0.45, 0.0, 0.45].iter().all(|dz| p.clear_line([x, z + dz], [x + 27.0, z + dz])) { lane = Some([x, z]); break; }
            x += 0.5;
        }
        z += 0.25;
    }
    let [x0, z] = lane.expect("a 27 m clear lane in the level");
    p.sam_put([x0 + 0.5, z]);
    p.run(0.3);
    let sam = p.sam_at();
    let shells = 40;
    let mut rows = Vec::new();
    for r in [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 15.0, 20.0, 25.0] {
        p.pin_man(shooter, [sam[0] + r + 0.7, z], std::f64::consts::PI, GuardState::Patrol, 0.0);
        p.frame();
        let was = p.shell_stats();
        for i in 0..shells {
            p.shell_from(shooter, [sam[0] + r, z], [sam[0], 1.25, z], 0.0);
            if i % 20 == 19 { p.frame(); }
        }
        let now = p.shell_stats();
        rows.push((r, (now.1 - was.1) as f64 / shells as f64, (now.2 - was.2) as f64 / shells as f64));
    }
    let at = |r: f64| rows.iter().find(|x| x.0 == r).unwrap();
    for r in [2.0, 4.0, 6.0] { assert!(at(r).1 >= 0.97, "across a room: {rows:?}"); }
    assert!(at(20.0).1 <= 0.4 && at(25.0).1 <= 0.25, "down a corridor: {rows:?}");
    assert!(rows.windows(2).all(|w| w[1].1 <= w[0].1 + 0.1), "it only falls with range: {rows:?}");
    assert!(at(20.0).2 > 0.2, "peppered at 20 m: {rows:?}");
}
