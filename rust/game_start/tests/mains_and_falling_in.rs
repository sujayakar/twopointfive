//! THE MAINS GO AND NO POSTED MAN WALKS, AND THE ERRAND IS TO THE PANEL'S OWN FACE. EVERY TEAMMATE IS ON FALL IN FROM THE STAGING'S OWN FRAME.
//! (A teammate throws the breaker on order and says so: held already by world.rs a_teammate_throwing_the_breaker_gets_the_games_line_and_his_own.)
mod world_ask;
use game::ids::GuardId;
use game::lighting::{breaker_reset_near, set_blackout, SetBlackoutCause};
use game::scene_collision::{nav_grid_is_blocked, solid_blocked};
use game::workthings::{breaker_errand, work_things_things, WorkThingKind};
use game_start::play::{dist, Opts, Play};

fn on_errand(p: &Play) -> Vec<GuardId> { p.g.guards.iter().copied().filter(|&m| matches!(&p.g.gd[m].task, Some(game::game::GuardTask::Arm0(t)) if t.kind == game::game::GuardTaskArm0Kind::Breaker)).collect() }
fn said_it(p: &Play) -> usize { p.said.iter().filter(|l| l.text.contains("checking the breaker")).count() }
fn keep_dark(p: &mut Play, secs: f64) { for _ in 0..(secs * 60.0) as usize { p.g.shown[p.g.pl[p.g.player].ch].visibility = 0.05; p.frame(); } }

/// THE RULE: the mains are thrown on a room clear, where every defender is held on his mark: nobody is handed the errand, and nobody says he's checking the breaker, then or six seconds on. CONTROL: the office's night shift: a man who can walk is handed it, says it and walks for the panel. (The floor's own word on a blackout, the powerout, comes later than this asks and is dark_rooms.rs's.)
/// A FAILURE MEANS: a man says he's off to the breaker and stands there.
#[test]
fn the_mains_go_and_no_posted_man_walks() {
    for s in [21u32] {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear", &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.2);
        assert!(p.hostiles().iter().all(|&m| p.g.gd[m].hold || p.g.gd[m].pinned), "seed {s}: the staging: a defender is off his mark");
        set_blackout(&mut p.g, 86400.0, SetBlackoutCause::Hand);
        keep_dark(&mut p, 2.5);
        assert!(on_errand(&p).is_empty() && said_it(&p) == 0, "seed {s}: errand {:?}, said {}", on_errand(&p), said_it(&p));
        drop(p);

        let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.1);
        let panel = p.g.level.breaker.pos;
        set_blackout(&mut p.g, 86400.0, SetBlackoutCause::Hand);
        keep_dark(&mut p, 1.0);
        let man = *on_errand(&p).first().unwrap_or_else(|| panic!("seed {s}: nobody was handed it"));
        let d0 = dist(p.at(man), [panel[0], panel[2]]);
        keep_dark(&mut p, 2.5);
        assert!(!p.g.gd[man].hold && !p.g.gd[man].pinned && said_it(&p) >= 1, "seed {s}: he is held or said nothing ({} lines)", said_it(&p));
        assert!(d0 - dist(p.at(man), [panel[0], panel[2]]) > 1.0, "seed {s}: he has not walked for it: {d0} m then {}", dist(p.at(man), [panel[0], panel[2]]));
    }
}

/// The panel as it hangs: a point just off its face, chest high, and the way out of it.
fn panel_of(p: &Play) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let th = work_things_things(&game::seats::here(&p.g)).into_iter().find(|x| x.kind == WorkThingKind::Breaker).expect("a panel");
    ([th.face[0] + th.n[0] * 0.05, 1.0, th.face[2] + th.n[2] * 0.05], th.face, th.n)
}

/// THE RULE: the other side's own errand goes to the panel's own face: on the relay station the panel hangs on the inside of the compound wall and a fixed spot off it is in the wall, and the man re-arms the mains from in front of it, nothing solid between. (The fault: the gate man re-armed the mains from the road, or never.)
/// A FAILURE MEANS: the lights come back by a hand through a wall.
#[test]
fn the_errand_is_to_the_panels_own_face() {
    let mut p = Play::mission("quiet-relay.json", "?levelFile=quiet-relay.json&quiet=3", &Opts { seed: 21, ..Default::default() });
    p.g.god_mode = true;
    let bp = p.g.level.breaker.pos;
    assert!(nav_grid_is_blocked(&p.g, bp[0] - 0.4, bp[2] - 0.9), "the staging: the fixed spot is floor here");
    p.g.shown[p.g.pl[p.g.player].ch].visibility = 0.0;
    p.run(0.5);
    let (at, face, n) = panel_of(&p);
    set_blackout(&mut p.g, 86400.0, SetBlackoutCause::Hand);
    let (mut through, mut from) = (0, None);
    for _ in 0..45 * 60 {
        p.g.shown[p.g.pl[p.g.player].ch].visibility = 0.0;
        p.frame();
        for m in p.g.guards.clone() {
            if p.g.light_home.mains.active && breaker_reset_near(&mut p.g, m) {
                let q = p.at(m);
                from = Some(q);
                if solid_blocked(&mut p.g, [q[0], 1.0, q[1]], at, None) { through += 1; }
            }
        }
        if !p.g.light_home.mains.active { break; }
    }
    assert!(!p.g.light_home.mains.active, "45 s and the mains are not back: {}", p.roll());
    let q = from.expect("a man worked it");
    assert_eq!(through, 0, "frames a man worked the lever with something solid between");
    assert!((q[0] - face[0]) * n[0] + (q[1] - face[2]) * n[2] > 0.0, "he worked it from behind its face: {q:?}");
}

/// THE CONTROL: the office, where the fixed spot is good: the errand keeps it.
#[test]
fn where_the_fixed_spot_is_good_the_errand_keeps_it() {
    let mut p = Play::level("office.json", false, &Opts { seed: 21, ..Default::default() });
    p.run(0.1);
    let bp = p.g.level.breaker.pos;
    let e = breaker_errand(&mut p.g).expect("an errand");
    assert_eq!((e.goal[0], e.goal[2]), (bp[0] - 0.4, bp[2] - 0.9), "the office's errand left its good spot");
}

fn order_of(p: &Play, m: GuardId) -> Option<(String, Option<[f64; 3]>)> {
    game::orders::squad_orders_of(&game::seats::here(&p.g), m).map(|a| { let a = &p.g.tab.orders_assignment[a]; (a.order.clone(), a.post) })
}

/// THE RULE (a settled rule): on the staging's own frame every living teammate is on the menu's fall in, no post, and nobody said it: no row in the orders' log and no line. Three seconds on with Sam standing still they're still on it and with him.
/// A FAILURE MEANS: the team stands waiting for a word at the start of a mission, or the rescue's men walk into the glass and start its clock.
#[test]
fn every_teammate_is_on_fall_in_from_the_staging_frame() {
    for (q, n) in [("nightbreach&squad=4", 4), ("roomclear&squad4", 4)] {
        let mut p = Play::mission("office.json", &format!("?levelFile=office.json&{q}"), &Opts { seed: 7, ..Default::default() });
        p.g.god_mode = true;
        let men = p.mates();
        assert_eq!(men.len(), n, "{q}");
        for &m in &men { assert_eq!(order_of(&p, m), Some((String::from("fallIn"), None)), "{q}: staged off fall in"); }
        assert!(game::seats::here(&p.g).orders.log.is_empty() && p.said.is_empty(), "{q}: somebody said it: {:?} {:?}", game::seats::here(&p.g).orders.log.len(), p.said.iter().map(|l| l.text.clone()).collect::<Vec<_>>());
        let sam = p.sam_at();
        p.run(3.0);
        for &m in &men { assert_eq!(order_of(&p, m).map(|o| o.0).as_deref(), Some("fallIn"), "{q}: off it after three seconds"); assert!(dist(p.at(m), sam) < 7.0, "{q}: {} m from Sam", dist(p.at(m), sam)); }
        assert!(game::seats::here(&p.g).orders.log.is_empty(), "{q}: a row in the log");
    }
}

/// THE CONTROL: the rescue isn't staged that way: its men are held with no order, and three idle seconds start no clock and fire no round.
#[test]
fn the_rescue_stages_its_men_held_with_no_order() {
    for s in [1u32, 2, 7] {
        let mut p = Play::mission("office.json", "?levelFile=office.json&rescue&squad=2", &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        for m in p.mates() { assert!(order_of(&p, m).is_none() && p.g.gd[m].hold && p.g.gd[m].pinned, "rescue, seed {s}: {:?}", order_of(&p, m)); }
        p.run(3.0);
        assert!(p.g.rescue.t0.is_nan(), "rescue, seed {s}: the clock started by itself");
        for m in p.mates() { assert_eq!(p.rounds_fired(m), 0.0, "rescue, seed {s}: a round fired"); }
    }
}
