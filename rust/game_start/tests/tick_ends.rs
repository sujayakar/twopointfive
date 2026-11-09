//! A RULE'S STATE IS STEPPED BY THE TICK AND BY NOTHING ELSE. What a host asks between two ticks, or whether it asks at all, moves no word of the game; a game taken in goes on as the one it
//! was taken from; a flash is stepped with no host's call round the tick; a row of the frame's table is free only when no list names it.
use game::ids::AnyBoxId;
use game_start::play::{Opts, Play};

fn office() -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 11, ..Default::default() });
    p.sam_unhurt();
    p
}
/// the squad on the move, and Sam firing now and then
fn act(p: &mut Play, k: usize) {
    if k == 20 { p.order("fallIn", [20.0, 14.0], game::orders::Who::All); }
    if k % 90 == 60 { p.sam_fires_at([16.0 + (k % 7) as f64, 12.0], 0.0); }
}
/// every word of the game's state, a word a part: the game's own parts, then rust/sim's block
fn words(p: &mut Play) -> Vec<u32> {
    let mut w = p.words();
    w.extend(game_start::block::part_hashes(p.g.sim));
    w
}
/// which parts two lines of words differ in
fn differ(a: &[u32], b: &[u32]) -> Vec<String> {
    let names = game::state_words::state_names().into_iter().chain(game_start::block::PART_NAMES.iter().map(|s| format!("block.{s}")));
    assert_eq!(a.len(), b.len());
    names.zip(a.iter().zip(b.iter())).filter(|(_, (x, y))| x != y).map(|(s, _)| s).collect()
}
/// what a host asks between ticks: rays round Sam, the view of its HUD. Answers how many boxes the rays met, and their indices, each a box of the world
fn ask(p: &mut Play) -> Vec<(f64, f64)> {
    let at = { let c = p.g.pl[p.g.player].ch; p.g.ch[c].pos };
    let mut met = Vec::new();
    for a in 0..24 {
        let t = a as f64 * core::f64::consts::PI / 12.0;
        if let Some(h) = game::scene_boxes::box_world_raycast(&p.g, [at[0], 1.2, at[2]], [t.cos(), 0.0, t.sin()], None, None, None, None) {
            assert!(game::scene_boxes::box_world_at(&p.g, h.index as usize).is_some());
            met.push((h.index, h.t));
        }
    }
    let _ = p.view();
    met
}

/// A FAILURE MEANS: a host's asking is part of the game, so a host that shows more plays another game.
#[test]
fn a_host_that_asks_and_one_that_does_not_play_one_game() {
    let run = |asks: bool| -> (Vec<Vec<u32>>, usize) {
        let mut p = office();
        let (mut out, mut met) = (Vec::new(), 0);
        for k in 0..300 {
            act(&mut p, k);
            if asks { met += ask(&mut p).len(); }
            p.frame();
            out.push(words(&mut p));
        }
        (out, met)
    };
    let ((asked, met), (quiet, _)) = (run(true), run(false));
    assert!(met > 1000, "the rays met {met} boxes: the host asked nothing to speak of");
    for k in 0..300 {
        let off = differ(&asked[k], &quiet[k]);
        assert!(off.is_empty(), "tick {k}: {off:?}");
    }
}

/// A FAILURE MEANS: a rule reads something that is only made from the record before it is made again.
#[test]
fn what_is_made_from_the_record_may_be_forgotten() {
    let run = |forget: bool| -> Vec<Vec<u32>> {
        let mut p = office();
        let mut out = Vec::new();
        for k in 0..360 {
            if k == 240 && forget {
                let had = ask(&mut p);
                p.g.ray_grid = Default::default();
                p.g.m.hours.reach = Default::default();
                p.g.col.lamp_q = Default::default();
                game::scene_collision::unstand(&mut p.g);
                assert_eq!(ask(&mut p), had, "a ray with no grid and a ray with one");
            }
            act(&mut p, k);
            p.frame();
            if k >= 240 { out.push(words(&mut p)); }
        }
        out
    };
    let (forgot, kept) = (run(true), run(false));
    for k in 0..120 {
        let off = differ(&forgot[k], &kept[k]);
        assert!(off.is_empty(), "tick {}: {off:?}", k + 240);
    }
}

/// A FAILURE MEANS: a muzzle flash stands for good unless a host steps it, and the men's light is off for as long.
#[test]
fn a_flash_is_retired_with_no_hosts_call() {
    let mut p = office();
    p.sam_takes_his_rifle();
    p.point_at([16.0, 12.0]);
    let flashes = |p: &Play| p.g.engine.lights.lights.iter().filter(|&&l| p.g.tab.light[l].ttl >= 0.0).count();
    p.press(game_start::play::Button::Left);
    let mut seen = 0;
    for _ in 0..4 { p.frame(); seen = seen.max(flashes(&p)); }
    p.release(game_start::play::Button::Left);
    assert!(seen > 0, "Sam fired and no flash stood");
    p.run(2.0);
    assert_eq!(flashes(&p), 0, "a flash outlived its time with no host round the tick");
}

/// A FAILURE MEANS: a row was used again while a list still names it.
#[test]
fn no_row_is_free_while_a_list_names_it() {
    let mut p = office();
    for k in 0..300 {
        act(&mut p, k);
        p.frame();
        for b in p.g.dyn_boxes.iter().chain(p.g.engine.world.dynamics.iter()) {
            if let AnyBoxId::Frame(f) = *b { assert!(!p.g.box_frame_free.contains(&f), "tick {k}: row {f:?} is free and on a list"); }
        }
    }
}
