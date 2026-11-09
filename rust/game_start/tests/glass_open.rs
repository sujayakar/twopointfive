//! A PANE: a wall to feet until what is below 1.9 m has gone; rays, sight and solid lines agree on what is open; through it a man is seen, heard the long way round, and fired at where the round goes through.
//! (The third reading of "open" the old check held, the rule the shader runs over the rows it is handed, is not here: it is a WGSL program and this host draws nothing.)
mod world_ask;
use game::game::GuardState;
use game::glasspen::PaneType;
use game_start::play::{dist, Opts, Play};
use world_ask::{GlassAsk, WorldAsk, YardAsk};

/// The office at night, the conference partition plain float, the floor's men parked; Sam in the corridor.
fn partition(s: u32, sam: [f64; 2]) -> (Play, Vec<game::ids::GuardId>) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, dress: true, ..Default::default() });
    p.g.god_mode = true;
    p.g.tune.chaos = 0.5;
    p.run(0.08);
    for x in [6.5, 7.5, 8.5] { let b = p.pane_at(x, 10.0); p.pane_is(b, PaneType::Annealed); }
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
    p.sam_put(sam);
    p.run(0.05);
    (p, men)
}
fn blow(p: &mut Play, bi: f64, at: [f64; 3], dir: [f64; 3]) { game::glass::glass_impact(&mut p.g, bi, at, Some(dir), 1.0, 1.0, None, None, None); }

/// every point of a grid on the sheet at (7.5, z 10) where the frame does not stand nearer: (open by the glass's rule, open to a ray, open to a solid line)
fn readings(p: &mut Play, bi: f64) -> Vec<(bool, bool, bool)> {
    let mut out = Vec::new();
    let mut u = -0.47;
    while u <= 0.47 {
        let mut v = -0.97;
        while v <= 0.97 {
            let (a, b, x) = ([7.5 + u, 1.5 + v, 10.3], [7.5 + u, 1.5 + v, 9.94], [7.5 + u, 1.5 + v, 10.0]);
            let hit = game::scene_collision::raycast(&mut p.g, a, [0.0, 0.0, -1.0], 0.36);
            let on_pane = hit.as_ref().map_or(false, |h| h.index == bi);
            if !hit.as_ref().map_or(false, |h| h.index != bi && h.t < 0.29) {
                let open = game::glassbreak::glass_break_open_at(&p.g, bi, x);
                out.push((open, !on_pane, !game::scene_collision::solid_blocked(&mut p.g, a, b, None)));
            }
            v += 0.0923;
        }
        u += 0.0673;
    }
    out
}

/// THE RULE: one model of what is open on a pane: the glass's own rule, what a ray does through it and what a solid line does through it agree at every point of the glass, with two craters in it and again once it has run. Sight passes whole glass and holes alike.
/// CONTROL: before any blow nothing is open, so the agreement is not one of everything open.
/// A FAILURE MEANS: a round goes through a piece of glass the picture shows standing, or a man walks where the rays say a wall.
#[test]
fn rays_solid_lines_and_the_glass_agree_on_what_is_open() {
    let (mut p, _) = partition(9104, [8.3, 11.6]);
    let bi = p.pane_at(7.5, 10.0);
    let w = |u: f64, v: f64| [7.5 + u, 1.5 + v, 10.0];
    let whole = readings(&mut p, bi);
    assert!(whole.len() > 150 && whole.iter().all(|r| !r.0 && !r.1 && !r.2), "whole: {} points, {} open", whole.len(), whole.iter().filter(|r| r.0).count());
    blow(&mut p, bi, w(-0.25, -0.2), [0.0, 0.0, -1.0]);
    blow(&mut p, bi, w(0.2, 0.3), [0.0, 0.0, -1.0]);
    p.run(0.5);
    let holed = readings(&mut p, bi);
    assert!(holed.iter().any(|r| r.0) && holed.iter().any(|r| !r.0), "two craters: {} of {} open", holed.iter().filter(|r| r.0).count(), holed.len());
    for (k, r) in holed.iter().enumerate() { assert!(r.0 == r.1 && r.0 == r.2, "two craters, point {k}: {r:?}"); }
    for (u, v) in [(0.05, -0.6), (-0.1, 0.7), (0.3, -0.7)] { blow(&mut p, bi, w(u, v), [0.0, 0.0, -1.0]); }
    p.run(1.5);
    let run = readings(&mut p, bi);
    assert!(run.iter().filter(|r| r.0).count() > holed.iter().filter(|r| r.0).count(), "five blows opened no more than two");
    for (k, r) in run.iter().enumerate() { assert!(r.0 == r.1 && r.0 == r.2, "run, point {k}: {r:?}"); }
    let eye = [7.5, 1.6, 11.5];
    for v in [-0.6, 0.0, 0.6] { assert!(!game::scene_collision::segment_blocked(&mut p.g, eye, [7.5, 1.5 + v, 8.5], None, None, None), "sight is stopped through the sheet at {v}"); }
}

/// how far a walker's body stood at a point is pushed by what is there
fn pushed(p: &mut Play, at: [f64; 2]) -> f64 {
    let mut q = [at[0], 0.0, at[1]];
    game::scene_collision::collide_circle(&mut p.g, &mut q, 0.34, 0.7, 1.5, Some(3.0));
    dist([q[0], q[2]], at)
}

/// THE RULE: a full-height toughened sheet is a wall to a walker until it has gone: pushed out, the chest-high line blocked, not walkable; one round dices it, not walkable while it is coming down, walkable once it has gone, and then feet and the solid line agree. The grid is not baked again.
/// A FAILURE MEANS: a man walks through standing glass or is stopped by glass that has gone; or a pane starts a new route.
#[test]
fn a_toughened_sheet_is_a_wall_to_feet_until_it_has_gone() {
    let (mut p, _) = partition(9105, [8.3, 11.6]);
    let bi = p.pane_at(4.0, 17.75);
    p.pane_is(bi, PaneType::Tempered);
    let (c, inward) = ([4.0, 17.75], [1.0, 0.0]);
    let (a, b) = ([c[0] - 0.6, 1.0, c[1]], [c[0] + 0.6, 1.0, c[1]]);
    let nav = p.level_nav();
    assert!(pushed(&mut p, [c[0] - 0.2, c[1]]) > 0.1 && game::scene_collision::solid_blocked(&mut p.g, a, b, None) && !game::glassbreak::glass_break_walkable(&mut p.g, bi), "whole: not a wall to feet");
    blow(&mut p, bi, [c[0] - 0.01, 1.2, c[1]], [inward[0], 0.0, inward[1]]);
    assert!(!game::glassbreak::glass_break_walkable(&mut p.g, bi), "walkable in the round's own frame");
    p.run(1.5);
    assert!(game::glassbreak::glass_break_walkable(&mut p.g, bi), "the sheet has had a second and a half: open {}", p.pane_open(bi));
    assert!(pushed(&mut p, [c[0] - 0.2, c[1]]) < 1e-6 && !game::scene_collision::solid_blocked(&mut p.g, a, b, None), "gone: feet or the line are still stopped");
    assert_eq!(p.level_nav(), nav, "a pane going rebaked the walk grid");
}

/// THE RULE: a float sheet holed at head height only is still a wall to feet: not walkable, the body pushed, shins blocked, the hole itself open at its own height.
/// A FAILURE MEANS: a man steps through a pane that stands to his chest.
#[test]
fn a_pane_holed_at_head_height_is_still_a_wall_to_feet() {
    let (mut p, _) = partition(9106, [8.3, 11.6]);
    let bi = p.pane_at(7.5, 10.0);
    blow(&mut p, bi, [7.6, 2.25, 10.0], [0.0, 0.0, -1.0]);
    p.run(0.7);
    assert!(p.pane_open(bi) > 0.0, "the staging: the blow opened nothing");
    let shin = game::scene_collision::solid_blocked(&mut p.g, [7.6, 0.8, 10.3], [7.6, 0.8, 9.7], None);
    let head = game::scene_collision::solid_blocked(&mut p.g, [7.6, 2.25, 10.3], [7.6, 2.25, 9.7], None);
    assert!(!game::glassbreak::glass_break_walkable(&mut p.g, bi) && pushed(&mut p, [7.6, 10.2]) > 0.1 && shin && !head, "walkable {}, shins blocked {shin}, the hole blocked {head}", game::glassbreak::glass_break_walkable(&mut p.g, bi));
}

/// THE RULE: a man in the corridor sees Sam lit behind the glass (the sight line passes, the solid line does not), and fires through it at where his round goes through; his rounds reach Sam. CONTROL: a brick wall between, same range: he never takes him.
/// A FAILURE MEANS: glass hides a man from the AI that the picture shows, or a man stands at a window and cannot fire through what his round would cross.
#[test]
fn through_glass_a_man_is_seen_and_fired_at() {
    for s in [911u32, 912] {
        let (sam, at) = ([12.0, 7.0], [12.0, 11.2]);
        let (mut p, men) = partition(s, sam);
        p.sam_takes_his_rifle();
        p.sam_put(sam);
        let m = men[0];
        p.posted_calm(m, at, sam);
        let c = p.g.gd[m].ch;
        let (eye, chest) = ([at[0], 1.65, at[1]], [sam[0], 1.2, sam[1]]);
        assert!(!game::scene_collision::segment_blocked(&mut p.g, eye, chest, None, None, None) && game::scene_collision::solid_blocked(&mut p.g, eye, chest, None), "seed {s}: the staging: only glass stands between them");
        p.until(5.0, |p| p.state(m) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: lit behind glass 4 m in front of him and he is {:?}, awareness {:.2}; Sam's light {:.2}", p.state(m), p.awareness(m), p.sam_light()));
        assert!(game::guards::clear_shot(&mut p.g, c, chest), "seed {s}: alert and no line of fire through the glass");
        let (rounds, flash) = (p.rounds_fired(m), std::cell::Cell::new(false));
        p.until(6.0, |p| { if p.g.pl[p.g.player].hit_flash > 0.9 { flash.set(true); } flash.get() }).unwrap_or_else(|| panic!("seed {s}: 6 s alert and no round reached Sam: {} fired", p.rounds_fired(m) - rounds));
        drop(p);

        // CONTROL: from the lobby side of the conference room's solid end, the same range
        let (at, sam) = ([16.5, 7.0], [12.0, 7.0]);
        let (mut p, men) = partition(s, sam);
        p.sam_put(sam);
        let m = men[0];
        p.posted_calm(m, at, sam);
        assert!(game::scene_collision::segment_blocked(&mut p.g, [at[0], 1.65, at[1]], [sam[0], 1.2, sam[1]], None, None, None), "seed {s}: the control: a wall stands between them");
        if let Some(t) = p.always(3.5, |p| p.state(m) != GuardState::Alert) { panic!("seed {s}: a wall between them and he went alert at {t:.2} s"); }
    }
}

/// THE RULE: through a whole pane, with the door shut, a sound goes the long way round: not direct, its carry small, yards longer than the 2 m straight; a scrape behind the glass reaches no ear. CONTROL: the same scrape as far off on his own side of the glass does.
/// A FAILURE MEANS: a player can be heard through glass as through air, or glass has made a man deaf on his own side.
#[test]
fn through_glass_a_sound_goes_the_long_way_round() {
    for s in [913u32, 914] {
        let (mut p, men) = partition(s, [12.4, 8.9]);
        let m = men[0];
        p.door_shut_locked("conference", false);
        p.posted_calm(m, [12.6, 11.2], [30.0, 11.2]);
        p.run(0.1);
        let (src, dst) = ([12.4, 1.0, 8.9], [12.6, 1.5, 11.2]);
        let straight = ((src[0] - dst[0]) as f64).hypot(src[2] - dst[2]);
        let r = game::scene_collision::sound_paths_propagate(&mut p.g, src, dst, None);
        assert!(!r.direct && r.carry <= 0.35 && r.path_len > straight + 1.0, "seed {s}: {straight:.1} m straight: direct {}, carry {:.2}, way {:.1} m", r.direct, r.carry, r.path_len);
        let n = p.small_heard(m);
        p.knock_at([12.4, 8.9]);
        p.run(0.05);
        assert_eq!(p.small_heard(m), n, "seed {s}: he heard a scrape through the glass");
        p.knock_at([14.7, 11.2]);
        p.run(0.05);
        assert!(p.small_heard(m) > n, "seed {s}: the same scrape as near on his own side did not reach him");
    }
}
