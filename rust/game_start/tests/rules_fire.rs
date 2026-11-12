//! WHAT A ROUND AND A BLAST DO TO GLASS, BARS, WALLS AND THE THINGS ON THEM: each rule in a room with the thing in it and nobody's mind running.
mod world_ask;
use game::glasspen::PaneType;
use game::scene_collision::{raycast, segment_blocked, shot_blocked, solid_blocked};
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, a_box_of, scene_with, FurnAsk, GlassAsk, WallAsk};

/// a box of a level's file with one word more
fn with(b: Json, key: &str, value: Json) -> Json {
    let Json::Obj(mut members, _) = b else { panic!("a box is an object") };
    members.push((String::from(key), value));
    json::obj(members.iter().map(|(k, v)| (k.as_str(), v.clone())).collect())
}

/// One wide room with a door, nobody's mind running, Sam out of it, chaos on, and the boxes asked about.
fn hall(boxes: Vec<Json>) -> Play {
    let mut s = Scene::new("hall");
    s.room("hall", 8.0, 6.0, 30.0, 22.0).door("yard", Axis::X, 22.0, 9.4).sam(9.0, 7.0).sentry("gun", [27.0, 20.0], [27.0, 7.0]);
    let mut p = scene_with(&s, boxes, &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.tune.chaos = 0.5;
    p.run(0.05);
    p
}

/// half a sheet of the thickness the glass's tolls are said for, so that a square-on crossing costs just its kind's cost
fn half_thick(p_k: &game::knobs::GlasspenK) -> f64 { p_k.ref_thick / 2.0 }
const THIN: f64 = 0.006;
/// a sheet of glass across the hall, its faces looking along z
fn sheet(id: &str, c: [f64; 3]) -> Json { a_box(id, c, [3.0, 1.4, THIN], "white", &["Glass"]) }
/// a round's way through the sheet at (18, 1.5, 14), slanted by a given cosine off square-on: from a metre short of it to a metre past it
fn slanted(cos: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let sin = (1.0 - cos * cos).sqrt();
    let d = [sin, 0.0, cos];
    ([18.0 - d[0], 1.5, 14.0 - d[2]], [18.0 + d[0], 1.5, 14.0 + d[2]], d)
}
fn cost(p: &Play, kind: PaneType) -> f64 { *p.g.k.glasspen.cost.get(kind).unwrap() }

/// THE RULE: a round goes through a sheet only with enough left over: the kind's toll and the floor under which it is spent, the same for a line of fire as a round. CONTROL: a little more energy, and it passes.
/// A FAILURE MEANS: a pistol round goes through laminated glass as through a shop window; a man fires at what his round will never reach.
#[test]
fn a_sheet_takes_its_kinds_toll_and_a_round_spent_stops_in_it() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    assert_eq!(THIN, half_thick(&p.g.k.glasspen), "the sheet's thickness is not the one the tolls are said for");
    let bi = p.pane_at(18.0, 14.0);
    let (a, b, _) = slanted(1.0);
    let floor = p.g.k.glasspen.min_energy;
    for kind in [PaneType::Annealed, PaneType::Tempered, PaneType::Laminated] {
        p.pane_is(bi, kind);
        let toll = cost(&p, kind);
        assert!(!shot_blocked(&mut p.g, a, b, None, Some(toll + floor + 0.03)), "{kind:?}: a round with enough left over is stopped");
        assert!(shot_blocked(&mut p.g, a, b, None, Some(toll + floor - 0.03)), "{kind:?}: a round left under the floor goes through");
    }
    assert!(cost(&p, PaneType::Laminated) > 2.0 * cost(&p, PaneType::Annealed), "the staging: laminated is no dearer than float");
}

/// THE RULE: a slanted round pays for the longer way through, up to the cap, and one that comes in nearly along the sheet skids off whatever its energy. CONTROL: the same energy square-on goes through.
/// A FAILURE MEANS: glass stops nothing at a slant, or a round fired along a window goes through it.
#[test]
fn a_slanted_round_pays_more_and_a_grazing_one_skids_off() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Annealed);
    let (toll, floor) = (cost(&p, PaneType::Annealed), p.g.k.glasspen.min_energy);
    // 60 degrees off square: twice the toll
    let e = 2.0 * toll + floor - 0.03;
    let (a, b, _) = slanted(1.0);
    assert!(!shot_blocked(&mut p.g, a, b, None, Some(e)), "square-on, a round of {e} is stopped");
    let (a, b, _) = slanted(0.5);
    assert!(shot_blocked(&mut p.g, a, b, None, Some(e)), "at 60 degrees, a round of {e} pays no more than square-on");
    assert!(!shot_blocked(&mut p.g, a, b, None, Some(2.0 * toll + floor + 0.03)), "at 60 degrees, a round with twice the toll is stopped");
    // under the grazing cosine nothing goes through, over it the same hard round does
    let graze = p.g.k.glasspen.graze_cos;
    let (a, b, _) = slanted(graze * 0.6);
    assert!(shot_blocked(&mut p.g, a, b, None, Some(10.0)), "at a graze, a hard round goes through");
    let (a, b, _) = slanted(graze * 1.5);
    assert!(!shot_blocked(&mut p.g, a, b, None, Some(10.0)), "just over the graze, a hard round is stopped");
}

/// THE RULE: where a round has been through a sheet already, the next one through that hole pays a fraction. CONTROL: a hand along the same sheet, it pays the whole toll.
/// A FAILURE MEANS: a holed laminated pane is as good as new to the man shooting through it, or as bad as open to the whole pane.
#[test]
fn a_hole_in_a_sheet_costs_a_round_less_than_the_sheet() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Laminated);
    let (toll, floor, hole_k) = (cost(&p, PaneType::Laminated), p.g.k.glasspen.min_energy, p.g.k.glasspen.hole_k);
    let e = (toll * hole_k + floor + toll + floor) / 2.0;
    let (at, off) = ([18.0, 1.5, 14.0], [18.0, 1.5, 14.0 - 1.0]);
    assert!(shot_blocked(&mut p.g, off, [18.0, 1.5, 15.0], None, Some(e)), "the staging: whole, the sheet stops a round of {e}");
    sim::pen::add_hole(p.g.tab.boxes.panes(), bi as u32, at, &p.g.k);
    assert!(!shot_blocked(&mut p.g, off, [18.0, 1.5, 15.0], None, Some(e)), "through the hole, a round of {e} pays the whole toll");
    assert!(shot_blocked(&mut p.g, [18.5, 1.5, 13.0], [18.5, 1.5, 15.0], None, Some(e)), "half a metre along, the hole lets a round of {e} through");
}

/// THE RULE: one flight goes through so many panes and no more, however hard the round. CONTROL: one pane fewer, and it goes through.
/// A FAILURE MEANS: a rifle round flies through a whole glass building.
#[test]
fn a_round_goes_through_so_many_panes_and_no_more() {
    let most = hall(vec![]).g.k.glasspen.max_panes as usize;
    let boxes: Vec<Json> = (0..=most).map(|k| a_box(&format!("sheet{k}"), [18.0, 1.5, 10.0 + k as f64], [3.0, 1.4, THIN], "white", &["Glass"])).collect();
    let mut p = hall(boxes);
    let a = [18.0, 1.5, 9.5];
    assert!(!shot_blocked(&mut p.g, a, [18.0, 1.5, 9.5 + most as f64], None, Some(100.0)), "through {most} panes a hard round is stopped");
    assert!(shot_blocked(&mut p.g, a, [18.0, 1.5, 10.5 + most as f64], None, Some(100.0)), "through {} panes a hard round goes on", most + 1);
}

/// THE RULE: glass turns a round a little, more for a dear crossing, and never much. A FAILURE MEANS: a shot through a window lands where it was aimed to the millimetre, or goes off round a corner.
#[test]
fn a_round_comes_out_of_glass_turned_a_little() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Laminated);
    let (mut sum, mut most, mut n) = (0.0, 0.0f64, 0);
    for k in 0..40 {
        let at = [16.5 + 0.075 * k as f64, 1.0 + 0.025 * k as f64];
        let w = p.g.tab.boxes.world();
        let x = sim::pen::pane_crossing(w, bi as u32, [at[0], at[1], 13.0], [0.0, 0.0, 1.0], 1.0 - THIN, 1.0, 0, &p.g.k).expect("a hard round goes through");
        sum += x.turn_deg;
        most = most.max(x.turn_deg);
        n += 1;
    }
    let cone = p.g.k.glasspen.deflect_deg;
    assert!(sum / n as f64 > 0.05 * cone, "laminated turns a round {:.3} degrees on the average", sum / n as f64);
    assert!(most < 2.0 * cone, "laminated turned a round {most:.2} degrees");
}

/// THE RULE: a round that would leave a sheet straight into a wall the sheet is laid on is spent in the sheet: it makes no crossing. CONTROL: the same sheet where nothing stands behind it.
/// A FAILURE MEANS: glass against a wall is holed through and the round comes out inside the wall.
#[test]
fn a_round_does_not_come_out_of_glass_into_a_wall() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0]), a_box_of("behind", [17.0, 1.5, 14.0 + THIN + 0.1], [1.0, 1.5, 0.1], "white", "concrete")]);
    let bi = p.pane_at(18.0, 14.0);
    let w = p.g.tab.boxes.world();
    let at = |x: f64| sim::pen::pane_crossing(w, bi as u32, [x, 1.5, 13.0], [0.0, 0.0, 1.0], 1.0 - THIN, 1.0, 0, &p.g.k);
    assert!(at(17.0).is_none(), "against the wall, the round comes out of the glass");
    assert!(at(19.5).is_some_and(|x| x.into == -1), "with air behind it, the round does not come out");
}

fn blow(p: &mut Play, bi: f64, at: [f64; 3]) { game::glass::glass_impact(&mut p.g, bi, at, Some([0.0, 0.0, 1.0]), 1.0, 1.0, None, None, None); }
/// is the line across the sheet at (x, y) open to a solid thing
fn across_open(p: &mut Play, x: f64, y: f64) -> bool { !solid_blocked(&mut p.g, [x, y, 13.7], [x, y, 14.3], None) }

/// THE RULE: toughened glass, once struck, stands white where it was for a moment, nearly whole, and then comes down all together. CONTROL: a moment after that, every part of it is open.
/// A FAILURE MEANS: a sheet vanishes in the frame it is shot, with no moment of it standing crazed.
#[test]
fn a_toughened_sheet_hangs_a_moment_and_then_comes_down() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Tempered);
    blow(&mut p, bi, [16.0, 1.5, 14.0]);
    p.frame();
    let far = [(19.5, 2.4), (20.0, 0.6), (18.0, 1.5)];
    for (x, y) in far { assert!(!across_open(&mut p, x, y) && !game::glassbreak::glass_break_open_at(&p.g, bi, [x, y, 14.0]), "a frame after the blow, the sheet is open at ({x}, {y})"); }
    assert!(p.pane_open(bi) < 0.5, "a frame after the blow, {} of the sheet is open", p.pane_open(bi));
    p.run(p.g.k.glassreal.hang + 0.2);
    for (x, y) in far { assert!(across_open(&mut p, x, y), "after its moment, the sheet still stands at ({x}, {y})"); }
    assert_eq!(p.pane_open(bi), 1.0, "after its moment, the sheet has not gone");
}

/// THE RULE: a laminated sheet a charge goes off beside leaves its frame whole, and then it is air everywhere. CONTROL: two metres off, the same blast leaves it standing.
/// A FAILURE MEANS: a man is stopped by, and hidden behind, a window that has gone.
#[test]
fn a_laminated_sheet_blown_out_is_air() {
    let mut p = hall(vec![a_box("near", [14.0, 1.5, 14.0], [1.0, 1.4, THIN], "white", &["Glass"]), a_box("far", [14.0, 1.5, 16.2], [1.0, 1.4, THIN], "white", &["Glass"])]);
    let (near, far) = (p.pane_at(14.0, 14.0), p.pane_at(14.0, 16.2));
    p.pane_is(near, PaneType::Laminated);
    p.pane_is(far, PaneType::Laminated);
    let (a, b) = ([14.3, 1.5, 13.5], [14.3, 1.5, 14.5]);
    assert!(solid_blocked(&mut p.g, a, b, None), "the staging: whole, the sheet is solid");
    let o = game::glassbreak::GlassBlastOpts { radius: Some(p.g.k.glassbreak.blast_m), power: Some(1.0), who: Some(1.0) };
    game::glassbreak::glass_break_blast(&mut p.g, &game::ask::Over::OWN, [14.0, 1.5, 13.9], Some(o));
    p.run(0.2);
    assert_eq!(p.pane_open(near), 1.0, "a charge a hand off, the sheet has not gone");
    assert!(!solid_blocked(&mut p.g, a, b, None) && !shot_blocked(&mut p.g, a, b, None, Some(0.01)), "the sheet has gone and it stops a line");
    assert_eq!(p.pane_open(far), 0.0, "two metres off, the sheet has gone");
    assert!(solid_blocked(&mut p.g, [14.3, 1.5, 15.7], [14.3, 1.5, 16.7], None), "two metres off, the sheet stops no line");
}

/// the hit a sheet has taken, as its row keeps it
fn sheet_damage(p: &Play, bi: f64) -> f64 {
    let panes = &p.g.tab.boxes.world().panes;
    panes.rows.iter().find(|r| r.w[0] as f64 == bi).map_or(0.0, |r| r.damage + r.energy)
}

/// THE RULE: a stun charge beside a sheet stars it. CONTROL: a stun charge across the hall, none of it.
/// A FAILURE MEANS: a bang in a room of glass leaves every pane as new.
#[test]
fn a_bang_beside_a_sheet_stars_it() {
    let mut p = hall(vec![a_box("near", [14.0, 1.5, 14.0], [1.0, 1.4, THIN], "white", &["Glass"]), a_box("far", [26.0, 1.5, 14.0], [1.0, 1.4, THIN], "white", &["Glass"])]);
    let (near, far) = (p.pane_at(14.0, 14.0), p.pane_at(26.0, 14.0));
    p.pane_is(near, PaneType::Annealed);
    p.pane_is(far, PaneType::Annealed);
    assert_eq!(sheet_damage(&p, near), 0.0, "the staging: the sheet is marked already");
    p.flash_lands([14.0, 13.2]);
    p.run(4.0);
    assert!(sheet_damage(&p, near) > 0.0, "a bang 80 cm off left the sheet whole");
    assert_eq!(sheet_damage(&p, far), 0.0, "a bang 12 m off marked the sheet");
}

/// THE RULE: the kit range's mending puts glass back: a pane shot out is a whole pane again. CONTROL: it was open before.
/// A FAILURE MEANS: the range's windows stay broken for the next try.
#[test]
fn mending_the_glass_puts_a_pane_back() {
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Tempered);
    blow(&mut p, bi, [18.0, 1.5, 14.0]);
    p.run(0.5);
    assert!(p.pane_open(bi) == 1.0 && across_open(&mut p, 18.0, 1.5), "the staging: the sheet did not go");
    game::glassbreak::glass_break_reset(&mut p.g);
    p.run(0.1);
    assert_eq!(p.pane_open(bi), 0.0, "mended, the sheet is open");
    assert!(!across_open(&mut p, 18.0, 1.5), "mended, the sheet stops no line");
}

/// THE RULE: a man reckoning a burst counts out the glass it would break: a float sheet beside the burst is no cover to him, though it stands. CONTROL: the sheet as it stands stops the line, and from across the hall the burst breaks nothing he need count.
/// A FAILURE MEANS: a man waits out a grenade behind a window it will take away.
#[test]
fn a_man_reckoning_a_burst_counts_out_the_glass_it_would_break() {
    use game::frag::GlassView;
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Annealed);
    let (from, to) = ([18.0, 13.5], [18.0, 1.2, 15.5]);
    assert!(!p.burst_line_open(from, to, GlassView::Whole), "the staging: the sheet as it stands is open to a burst");
    assert!(p.burst_line_open(from, to, GlassView::Wary), "half a metre off a float sheet, he takes it for cover");
    let lift_far = [18.0, 6.5];
    assert!(!p.burst_line_open(lift_far, to, GlassView::Wary), "from seven metres off, he counts the sheet out");
}

/// THE RULE: at the moment of a burst, the wedges of a crater that has begun to go all count as gone: a man's reckoning of the blast takes the crater whole. Every wedge of such a crater, by the glass's own map of itself, reads open in that view.
/// A FAILURE MEANS: a fragment is stopped by a sliver of glass that goes in the same crack.
#[test]
fn a_burst_counts_a_whole_crater_out_with_its_first_wedge() {
    use game::glassbreak::{glass_break_blast, glass_break_rule_open, glass_break_see_through, GlassBlastOpts, GlassBreakViewOfKind};
    use sim::glass::{classify_row, GW_CRATER, GW_GOING};
    use sim::pattern::FRAGS_PER_SITE;
    let mut p = hall(vec![sheet("sheet", [18.0, 1.5, 14.0])]);
    let bi = p.pane_at(18.0, 14.0);
    p.pane_is(bi, PaneType::Annealed);
    let opts = GlassBlastOpts { radius: Some(p.g.k.frag.glass_m), power: Some(1.0), who: Some(1.0) };
    let from = [18.0, 0.05 + p.g.k.frag.lift, 13.2];
    glass_break_blast(&mut p.g, &game::ask::Over::OWN, from, Some(opts));
    // points of the sheet that lie in a wedge not itself queued, of a crater one of whose wedges is
    let row = *p.g.tab.boxes.world().panes.rows.iter().find(|r| r.w[0] as f64 == bi).expect("the blast touched the sheet");
    const CRATER_WEDGES: usize = 12;   // (a site's first pieces, by the glass's own numbering, are its crater's wedges)
    let (mut pts, mut sectors) = (Vec::new(), 0usize);
    for i in 0..1200 {
        for j in 0..500 {
            let (u, v) = (-3.0 + 6.0 * (i as f64 + 0.5) / 1200.0, -1.4 + 2.8 * (j as f64 + 0.5) / 500.0);
            let fid = classify_row(&row.d, &row.w, u, v);
            if fid < 0 { continue; }
            let f = fid as usize;
            sectors += 1;
            let wedge = (f % FRAGS_PER_SITE as usize) < CRATER_WEDGES;
            let crater = row.w[GW_CRATER] >> (f / FRAGS_PER_SITE as usize) & 1 != 0;
            let going = row.w[GW_GOING + (f >> 5)] >> (f & 31) & 1 != 0;
            let out = row.w[sim::glass::GW_OPEN + (f >> 5)] >> (f & 31) & 1 != 0;
            if wedge && crater && !going && !out { pts.push([18.0 + u, 1.5 + v, 14.0]); }
        }
    }
    assert!(sectors > 0 && !pts.is_empty(), "the staging: {sectors} points on pieces and none in a crater half gone");
    let shut = glass_break_see_through(&mut p.g, &game::ask::Over::OWN, GlassBreakViewOfKind::Blown, from, opts, &mut |g| pts.iter().filter(|&&q| !glass_break_rule_open(g, bi, q)).count());
    assert_eq!(shut, 0, "of {} points in a crater half gone, the blast takes some for glass", pts.len());
}

/// a window in the hall: a sill, a head, three bars between them; the places of the bars along x
const BARS: [f64; 3] = [17.6, 18.0, 18.4];
fn window() -> Vec<Json> {
    let mut v = vec![a_box("sill", [18.0, 0.45, 14.0], [1.0, 0.45, 0.06], "white", &[]), a_box("head", [18.0, 2.6, 14.0], [1.0, 0.4, 0.06], "white", &[])];
    for (k, x) in BARS.iter().enumerate() { v.push(a_box_of(&format!("bar{k}"), [*x, 1.55, 14.0], [0.02, 0.65, 0.05], "white", "steel")); }
    v
}
fn bar_stands(p: &mut Play, x: f64) -> bool {
    let i = p.g.col.boxes.iter().position(|&b| { let c = p.g.tab.boxes[b].c; (c[0] - x).abs() < 1e-6 && (c[1] - 1.55).abs() < 1e-6 && (c[2] - 14.0).abs() < 1e-6 }).expect("a bar there");
    game::winbars::bar_at(&mut p.g, i as f64)
}
/// the man in the hall, stood at a place looking at a point, and a round from him at it
fn gun_fires(p: &mut Play, at: [f64; 2], to: [f64; 3]) {
    let gd = p.guard("gun");
    p.man_put(gd, at, [to[0], to[2]]);
    p.frame();
    let c = p.g.gd[gd].ch;
    let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::default() };
    game::combat::fire_weapon(&mut p.g, c, to, Some(&o));
    p.run(0.1);
}

/// THE RULE: a window's bar a round meets goes, and the bars beside it stand. CONTROL: before the round it stopped the eye, and its neighbours do yet.
/// A FAILURE MEANS: bars are air to a round, or iron to one.
#[test]
fn a_round_takes_the_bar_it_meets() {
    let mut p = hall(window());
    for x in BARS { assert!(bar_stands(&mut p, x), "the staging: no bar at {x}"); }
    assert!(segment_blocked(&mut p.g, [18.0, 1.3, 13.0], [18.0, 1.3, 15.0], None, None, None), "the staging: the bar stops no eye");
    gun_fires(&mut p, [18.0, 12.0], [18.0, 1.3, 14.0]);
    assert!(!bar_stands(&mut p, 18.0), "a round struck the bar and it stands");
    assert!(!segment_blocked(&mut p.g, [18.0, 1.3, 13.0], [18.0, 1.3, 15.0], None, None, None), "the bar has gone and it stops an eye");
    assert!(bar_stands(&mut p, 17.6) && bar_stands(&mut p, 18.4), "the bars beside it went with it");
}

/// THE RULE: a charge takes the bars it has a line to. CONTROL: the same charge with a concrete slab between: they all stand.
/// A FAILURE MEANS: bars come out of a window through a wall, or a charge at a window leaves them.
#[test]
fn a_blast_takes_the_bars_it_can_reach_and_no_others() {
    let at = [18.0, 1.0, 13.0];
    let mut p = hall(window());
    assert!(p.wall_blast(at, None) >= 0.0);
    p.run(0.05);
    for x in BARS { assert!(!bar_stands(&mut p, x), "a charge a metre off and the bar at {x} stands"); }
    drop(p);
    let mut boxes = window();
    boxes.push(a_box_of("slab", [18.0, 1.5, 13.4], [2.0, 1.5, 0.1], "white", "concrete"));
    let mut p = hall(boxes);
    p.wall_blast(at, None);
    p.run(0.05);
    for x in BARS { assert!(bar_stands(&mut p, x), "concrete between, and the charge took the bar at {x}"); }
}

/// THE RULE: an outside pane shot out lets the storm in: a wind source through it. CONTROL: whole it lets nothing in, and a pane indoors shot out lets in nothing.
/// A FAILURE MEANS: smoke in a room with its windows out sits still in a gale.
#[test]
fn a_storm_blows_in_through_an_outside_pane_shot_out() {
    let mut s = Scene::new("lot");
    s.room("hut", 30.0, 20.0, 36.0, 26.0).door("hut-door", Axis::X, 20.0, 31.0).sam(4.0, 24.0);
    let out = a_box("out", [34.8, 1.5, 19.9], [1.0, 1.2, THIN], "white", &["Glass"]);
    let inn = a_box("in", [34.0, 1.5, 23.0], [1.0, 1.2, THIN], "white", &["Glass"]);
    let mut p = scene_with(&s, vec![out, inn], &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.run(0.05);
    game::weather::weather_set_kind_2(&mut p.g, Some(game::weather_kinds::WeatherKind::Rain));
    p.g.weather.next_flash_at = 1e9;
    p.run(0.1);
    assert!(game::wind::wind_storm_wind(&p.g).is_some(), "the staging: the storm blows nothing");
    let (out, inn) = (p.pane_at(34.8, 19.9), p.pane_at(34.0, 23.0));
    p.pane_is(out, PaneType::Tempered);
    p.pane_is(inn, PaneType::Tempered);
    let gust = |p: &Play, bi: f64| p.g.glass_break.gusts.get(game::Num(bi)).map_or(0.0, |&w| p.g.tab.wind_wind_source[w].base);
    p.run(0.3);
    assert_eq!(gust(&p, out), 0.0, "the pane is whole and the storm blows through it");
    blow(&mut p, out, [34.8, 1.5, 19.9]);
    blow(&mut p, inn, [34.0, 1.5, 23.0]);
    p.run(1.0);
    assert!(p.pane_open(out) == 1.0 && p.pane_open(inn) == 1.0, "the staging: the panes did not go");
    assert!(gust(&p, out) > 0.05, "shot out, the outside pane lets in {}", gust(&p, out));
    assert_eq!(gust(&p, inn), 0.0, "the indoor pane lets the storm in");
}

/// a slab of cardboard across the hall of the given thickness, thin side along z, saying what it is
fn board(thick: f64) -> Json { a_box_of("board", [18.0, 1.0, 14.0], [1.0, 1.0, thick / 2.0], "white", "card") }
fn round_gets_through(p: &mut Play, energy: f64) -> bool {
    matches!(game::wallpen::thin_line(&mut p.g, [18.0, 1.0, 12.5], [18.0, 1.0, 15.5], energy, None, None), Some(game::wallpen::ThinLine2::ThinLine(_)))
}

/// THE RULE: a thin thing a round has enough for lets it through, and takes a toll: under the floor the round is spent in it. CONTROL: more energy, the same board.
/// A FAILURE MEANS: a pistol round through a door is as good as a rifle's, or none goes through.
#[test]
fn a_round_with_too_little_left_stops_in_a_thin_thing() {
    use game::wallpen::ThinStuff;
    let thick = 0.1;
    let mut p = hall(vec![board(thick)]);
    let k = &p.g.k.wallpen;
    let (toll, floor) = (k.base.get(ThinStuff::Cardboard).unwrap() + k.per_m.get(ThinStuff::Cardboard).unwrap() * thick, k.min_energy);
    assert!(round_gets_through(&mut p, toll + floor + 0.03), "enough left over, and the board stops the round");
    assert!(!round_gets_through(&mut p, toll + floor - 0.03), "under the floor, and the round goes through the board");
}

/// THE RULE: a thing past the thickness its stuff may be stops every round. CONTROL: the same stuff under that thickness lets a hard round through.
/// A FAILURE MEANS: a stack of cardboard a metre through is no cover.
#[test]
fn a_thing_too_thick_stops_every_round() {
    use game::wallpen::ThinStuff;
    let (most, longest) = { let p = hall(vec![]); (*p.g.k.wallpen.thin_max.get(ThinStuff::Cardboard).unwrap(), *p.g.k.wallpen.max_thru.get(ThinStuff::Cardboard).unwrap()) };
    let (thin, thick) = (most * 0.9, most * 1.07);
    assert!(thick < longest, "the staging: a round's longest way through {longest} stops it first");
    let mut p = hall(vec![board(thin)]);
    assert!(round_gets_through(&mut p, 3.0), "{thin} m of it stopped a hard round");
    drop(p);
    let mut p = hall(vec![board(thick)]);
    assert!(!round_gets_through(&mut p, 3.0), "{thick} m of it let a hard round through");
}

/// THE RULE: metal stops a round whatever else it says it is. CONTROL: the same board not metal lets it through.
/// A FAILURE MEANS: a man is safe behind nothing he can see, or a steel door is paper.
#[test]
fn metal_stops_a_round_that_would_go_through_the_same_board() {
    let mut p = hall(vec![board(0.1)]);
    assert!(round_gets_through(&mut p, 3.0), "the control: the board stopped a hard round");
    drop(p);
    let mut p = hall(vec![with(board(0.1), "metal", json::b(true))]);
    assert!(!round_gets_through(&mut p, 3.0), "metal let a hard round through");
}

/// Two rooms with a plasterboard partition at x 18 and the door well away, chaos on.
fn rooms(boxes: Vec<Json>) -> Play {
    let mut s = Scene::new("partition");
    s.room("west", 10.0, 8.0, 18.0, 18.0).room("east", 18.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 10.0, 15.4).sam(2.25, 2.25);
    let mut p = scene_with(&s, boxes, &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.tune.chaos = 0.5;
    p.run(0.1);
    p
}
fn cells_open(p: &Play) -> f64 { game::wallbreak::wall_break_all(&p.g).iter().map(|w| w.open).sum() }
/// rounds of a gun at one place of the partition, by the wall's own call, until it opens
fn rounds_to_open(pistol: bool) -> usize {
    let mut p = rooms(vec![]);
    let hit = raycast(&mut p.g, [16.8, 1.6, 11.0], [1.0, 0.0, 0.0], 2.0).expect("the partition");
    let at = [16.8 + hit.t, 1.6, 11.0];
    let man = &p.g.pl[p.g.player];
    let gun = if pistol { man.pistol } else { man.rifle };
    let def = p.g.m.weapons.weapons.get(p.g.fa[gun].def).expect("its row").clone();
    let who = p.g.ch[p.g.pl[p.g.player].ch].id;
    let chaos = p.g.tune.chaos;
    for n in 1..400 {
        game::wallbreak::wall_break_round(&mut p.g, who, at, [1.0, 0.0, 0.0], hit.index, &def, chaos);
        if cells_open(&p) > 0.0 { return n; }
    }
    panic!("400 rounds and the partition stands");
}

/// THE RULE: a pistol's round wears a wall by its share of a rifle's. CONTROL: the rifle's own count opens it.
/// A FAILURE MEANS: a sidearm chews through plasterboard as a rifle does.
#[test]
fn a_pistol_wears_a_wall_slower_than_a_rifle() {
    let (rifle, pistol) = (rounds_to_open(false), rounds_to_open(true));
    let share = hall(vec![]).g.k.walls.pistol_k;
    assert!(share < 0.8, "the staging: a pistol's share is {share}");
    assert!(pistol as f64 >= (rifle as f64 / share) * 0.8, "the rifle opens it in {rifle} rounds and the pistol, at {share} of it, in {pistol}");
}

/// THE RULE: a blast opens only what it reaches: a second partition behind the first keeps every cell. CONTROL: from between them, the same blast opens the second.
/// A FAILURE MEANS: a charge takes out the wall behind a wall.
#[test]
fn a_blast_does_not_open_a_wall_behind_a_wall() {
    let two = || {
        let mut s = Scene::new("two-partitions");
        s.room("west", 10.0, 8.0, 18.0, 18.0).room("mid", 18.0, 8.0, 18.8, 18.0).room("east", 18.8, 8.0, 26.0, 18.0).door("yard", Axis::Z, 10.0, 15.4).sam(2.25, 2.25);
        let mut p = scene_with(&s, vec![], &Opts { seed: 1, ..Default::default() });
        p.g.god_mode = true;
        p.g.ai_enabled = false;
        p.g.tune.chaos = 0.5;
        p.run(0.1);
        p
    };
    let open_on = |p: &Play, x: f64| -> f64 { game::wallbreak::wall_break_all(&p.g).iter().filter(|w| (p.g.tab.boxes[w.box_].c[0] - x).abs() < 0.01).map(|w| w.open).sum() };
    let mut p = two();
    assert!(p.wall_blast([17.7, 0.3, 12.0], None) > 0.0, "the staging: the charge opened nothing");
    p.run(0.1);
    assert!(open_on(&p, 18.0) > 0.0, "the charge's own wall stands whole");
    assert_eq!(open_on(&p, 18.8), 0.0, "the wall behind the wall lost cells");
    drop(p);
    let mut p = two();
    p.wall_blast([18.5, 0.3, 12.0], None);
    p.run(0.1);
    assert!(open_on(&p, 18.8) > 0.0, "the control: from between them, the far wall stands whole");
}

/// THE RULE: what stood on a thing chewed away comes down with it. CONTROL: while the crate stands, it stands on it.
/// A FAILURE MEANS: a box hangs in the air over the place a crate was.
#[test]
fn what_stood_on_a_crate_comes_down_when_the_crate_goes() {
    let mut p = rooms(vec![a_box_of("crate", [14.0, 0.4, 12.0], [0.4, 0.4, 0.4], "white", "wood"), a_box("on-it", [14.0, 1.0, 12.0], [0.25, 0.2, 0.25], "white", &[])]);
    let (a, b) = ([13.0, 1.0, 12.0], [15.0, 1.0, 12.0]);
    assert!(solid_blocked(&mut p.g, a, b, None), "the staging: nothing stands on the crate");
    let man = &p.g.pl[p.g.player];
    let def = p.g.m.weapons.weapons.get(p.g.fa[man.rifle].def).expect("its row").clone();
    let who = p.g.ch[p.g.pl[p.g.player].ch].id;
    let chaos = p.g.tune.chaos;
    let gone = |p: &Play| game::wallbreak::wall_break_all(&p.g).iter().any(|w| w.kind == game::wallbreak::BKind::Cover && w.open >= w.cells);
    for _ in 0..2000 {
        if gone(&p) { break; }
        let Some(hit) = raycast(&mut p.g, [13.0, 0.4, 12.0], [1.0, 0.0, 0.0], 2.0) else { break };
        game::wallbreak::wall_break_round(&mut p.g, who, [13.0 + hit.t, 0.4, 12.0], [1.0, 0.0, 0.0], hit.index, &def, chaos);
        let Some(hit) = raycast(&mut p.g, [13.0, 0.2, 11.9], [1.0, 0.0, 0.0], 2.0) else { continue };
        game::wallbreak::wall_break_round(&mut p.g, who, [13.0 + hit.t, 0.2, 11.9], [1.0, 0.0, 0.0], hit.index, &def, chaos);
    }
    assert!(gone(&p), "the staging: the crate was not chewed away");
    p.run(0.2);
    assert!(!solid_blocked(&mut p.g, a, b, None), "the crate has gone and what stood on it hangs in the air");
}

/// the width of the hole in the partition at knee height, in metres along it, within a few metres of z
fn hole_width(p: &mut Play, z: f64) -> f64 {
    let n = 80;
    let open = (0..n).filter(|k| { let q = z - 2.0 + 4.0 * (*k as f64 + 0.5) / n as f64; !solid_blocked(&mut p.g, [17.5, 0.5, q], [18.5, 0.5, q], None) }).count();
    4.0 * open as f64 / n as f64
}

/// THE RULE: a charge against a wall makes a hole a man fits through, however small the charge: at least so many columns. CONTROL: the same small charge a stride off the wall makes no such hole.
/// A FAILURE MEANS: a breach a man cannot walk through.
#[test]
fn a_charge_on_a_wall_makes_a_hole_a_man_fits_through() {
    let radius = 1.0;
    let blast = |p: &mut Play, x: f64| { let o = game::wallbreak::BlastOpts { who: Some(game::consts::PLAYER_ID), chaos: Some(false), radius: Some(radius), ..Default::default() }; game::destruction::destruction_blast(&mut p.g, [x, 0.3, 12.0], Some(o)); p.run(0.1); };
    let mut p = rooms(vec![]);
    let (columns, col) = (p.g.k.walls.blast.min_hole, p.g.k.walls.col);
    blast(&mut p, 17.7);
    let width = hole_width(&mut p, 12.0);
    assert!(width >= columns * col - 0.11, "on the wall, the hole is {width} m at the knee, and a man needs {} columns of {col} m", columns);
    let end = p.body_walks([17.2, 12.0], [18.8, 12.0]);
    assert!((end[0] - 18.8).abs() < 0.05, "a man walking at the hole ends at {end:?}");
    drop(p);
    let mut p = rooms(vec![]);
    blast(&mut p, 17.0);
    let width = hole_width(&mut p, 12.0);
    assert!(width < columns * col - 0.3, "a stride off the wall the small charge opened {width} m at the knee");
}

/// THE RULE: rounds around small things on a desk knock them off, once the place has taken enough. CONTROL: the first round leaves them.
/// A FAILURE MEANS: a firefight leaves every mug in its place.
#[test]
fn rounds_knock_small_things_off_a_desk() {
    let mut p = hall(vec![a_box_of("desk", [14.0, 0.4, 12.0], [0.6, 0.4, 0.4], "white", "concrete"), a_box("mug", [14.0, 0.86, 12.0], [0.05, 0.06, 0.05], "white", &[])]);
    let (a, b) = ([13.5, 0.86, 12.0], [14.5, 0.86, 12.0]);
    assert!(solid_blocked(&mut p.g, a, b, None), "the staging: no mug on the desk");
    gun_fires(&mut p, [14.0, 9.0], [14.0, 0.86, 12.0]);
    assert!(solid_blocked(&mut p.g, a, b, None), "one round took the mug off");
    let enough = (p.g.k.chaos.t_knock / (2.0 * p.g.tune.chaos)).ceil() as usize + 2;
    for _ in 0..enough { gun_fires(&mut p, [14.0, 9.0], [14.0, 0.86, 12.0]); }
    assert!(!solid_blocked(&mut p.g, a, b, None), "{} rounds at it and the mug stands", enough + 1);
}

/// THE RULE: in a fight, a round a man fires at a lamp puts it out. CONTROL: the lamp in the next room burns on.
/// A FAILURE MEANS: lights in a firefight are out of reach of the men in it.
#[test]
fn a_mans_round_puts_out_a_lamp_in_a_fight() {
    let mut s = Scene::new("yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.g.tune.chaos = 0.5;
    p.run(0.2);
    let lamps: Vec<_> = p.g.targets.clone();
    let near = |p: &Play, x: f64| lamps.iter().copied().find(|&t| { let f = &p.g.tab.fixture[t]; (f.pos[0] - x).abs() < 1.0 && (f.pos[2] - 13.0).abs() < 1.0 }).unwrap_or_else(|| panic!("no lamp at x {x}"));
    let (west, east) = (near(&p, 14.0), near(&p, 22.0));
    assert!(!p.g.tab.fixture[west].broken && !p.g.tab.fixture[east].broken, "the staging: a lamp is out");
    let man = p.defender_at([12.25, 12.25]);
    let at = p.g.tab.fixture[west].pos;
    let c = p.g.gd[man].ch;
    let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::default() };
    game::combat::fire_weapon(&mut p.g, c, at, Some(&o));
    p.frame();
    assert!(p.g.tab.fixture[west].broken, "a round at the lamp and it burns on");
    assert!(!p.g.tab.fixture[east].broken, "a round at one lamp put out the next room's");
}

/// THE RULE: the kit range's mending puts a blown wall back: every question says wall again. CONTROL: it was a hole before.
/// A FAILURE MEANS: the range's next try plays on the last try's ruins.
#[test]
fn mending_the_level_puts_a_blown_wall_back() {
    let mut p = rooms(vec![]);
    let was = p.level_as_it_stands();
    p.wall_blast([17.7, 0.3, 11.0], None);
    p.run(0.1);
    let (a, b) = ([16.8, 1.6, 11.0], [19.2, 1.6, 11.0]);
    assert!(!solid_blocked(&mut p.g, a, b, None) && p.level_as_it_stands() != was, "the staging: the blast opened nothing");
    game::destruction::destruction_reset(&mut p.g);
    p.run(0.1);
    assert!(solid_blocked(&mut p.g, a, b, None), "mended, the wall is a hole");
    assert_eq!(cells_open(&p), 0.0, "mended, cells of the wall are open");
}
