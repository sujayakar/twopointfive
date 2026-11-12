//! A BURST AND THE GLASS BETWEEN IT AND A MAN: it never hurts a man through a pane it left whole; behind one it broke he is judged by what is open.
mod world_ask;
use game::frag::{BlastResult, GlassView};
use game::glasspen::PaneType;
use game::ids::GuardId;
use game_start::json::Json;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, a_box_of, scene_with, GlassAsk};

const GLAZING_Z: f64 = 10.0;

/// The office at night with nobody's mind running and Sam out of it; the conference room's three sheets (x 6 to 9, a sill under them, the room north, the corridor south),
/// each of a kind; the men of the floor with one left at his post.
fn glazing(s: u32, kinds: [PaneType; 3]) -> (Play, [f64; 3], Vec<GuardId>) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.k.chaos.def = 0.5;
    p.sam_put([24.0, 17.6]);
    p.frame();
    let panes = [p.pane_at(6.5, GLAZING_Z), p.pane_at(7.5, GLAZING_Z), p.pane_at(8.5, GLAZING_Z)];
    for (b, k) in panes.iter().zip(kinds) { p.pane_is(*b, k); }
    let men: Vec<GuardId> = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate() { p.man_put(m, [36.0 + k as f64, 26.0], [0.0, 0.0]); }
    p.frame();
    (p, panes, men)
}

/// THE RULE: a man behind a pane the burst left whole is not hurt and not even folded: toughened glass 3 m off the burst, float glass 3.9 m off, laminated 2.5 m off,
/// and the pane still stands a second and more later. CONTROL: the same man with the burst a metre off the same toughened glass: it comes down and he is hit, so it was the
/// pane and not the range that saved him.
/// A FAILURE MEANS: a burst reaches through standing glass (the fragment reckoned as a pistol round), or glass stands up to a burst beside it.
#[test]
fn a_burst_never_hurts_a_man_through_a_pane_it_left_whole() {
    for (k, (kind, off)) in [(PaneType::Tempered, 3.0), (PaneType::Annealed, 3.9), (PaneType::Laminated, 2.5)].into_iter().enumerate() {
        let s = k as u32 + 1;
        let (mut p, panes, men) = glazing(s, [kind; 3]);
        let (man, burst) = ([7.5 + 0.2 * nudge(s, 0), 9.3], [7.5 + 0.2 * nudge(s, 1), GLAZING_Z + off]);
        p.man_put(men[0], man, burst);
        p.run(0.05);
        let was = p.reeled_at(men[0]);
        let rep = p.burst(burst);
        p.run(1.2);
        let hit = p.hit_of(&rep, men[0]);
        assert!(hit.as_ref().map_or(false, |h| matches!(h.result, BlastResult::Shielded)), "{kind:?} {off} m off: his line of the report {hit:?}");
        assert!(p.up(men[0]) && p.reeled_at(men[0]) == was, "{kind:?} {off} m off: he is {:?}, reeled {}", p.state(men[0]), p.reeled_at(men[0]) != was);
        for b in panes { assert_eq!(p.pane_open(b), 0.0, "{kind:?} {off} m off: pane {b} is open"); }
        drop(p);
    }
}

/// THE CONTROL: the same man with the burst a metre off the same toughened glass: it comes down and he is hit, so it was the pane and not the range that saved him.
#[test]
fn a_burst_a_metre_off_the_same_glass_brings_it_down() {
    let (mut p, panes, men) = glazing(4, [PaneType::Tempered; 3]);
    let (man, burst) = ([7.5, 9.3], [7.5, GLAZING_Z + 1.0]);
    p.man_put(men[0], man, burst);
    p.run(0.05);
    let rep = p.burst(burst);
    p.run(2.0);
    let hit = p.hit_of(&rep, men[0]);
    assert!(hit.map_or(false, |h| !matches!(h.result, BlastResult::Shielded)) && p.pane_open(panes[1]) == 1.0, "a metre off the glass: {:?}, the pane {}", p.hit_of(&rep, men[0]), p.pane_open(panes[1]));
}

/// THE RULE: one burst, three panes, three men: float glass nearest it breaks and the man behind it is hit through the part that went; the laminated and the toughened sheets
/// stand and the men behind them are untouched, inside the wound radius. And the share of each man the blast judged in its line is the share really open once the glass has
/// finished falling: nothing judged through glass that then stood, nothing missed that then fell.
/// A FAILURE MEANS: what the blast reckons and what the room then is are two models.
#[test]
fn behind_a_pane_it_broke_a_man_is_judged_by_what_is_open() {
    for s in [51u32, 52] {
        let (mut p, panes, men) = glazing(s, [PaneType::Annealed, PaneType::Laminated, PaneType::Tempered]);
        let burst = [5.2 + 0.1 * nudge(s, 0), 11.2];
        let at = [[7.0, 9.0], [9.2, 8.6], [10.15, 9.4]];
        for (m, q) in men.iter().zip(at) { p.man_put(*m, q, burst); }
        p.run(0.05);
        let was: Vec<f64> = men.iter().take(3).map(|&m| p.reeled_at(m)).collect();
        let rep = p.burst(burst);
        p.run(2.5);
        assert!(p.pane_open(panes[0]) > 0.0 && p.pane_open(panes[1]) == 0.0 && p.pane_open(panes[2]) == 0.0, "seed {s}: open: {} {} {}", p.pane_open(panes[0]), p.pane_open(panes[1]), p.pane_open(panes[2]));
        let a = p.hit_of(&rep, men[0]).unwrap_or_else(|| panic!("seed {s}: the man behind the float pane is not in the report"));
        assert!(!matches!(a.result, BlastResult::Shielded) && a.exposed > 0.0 && (!p.up(men[0]) || p.reeled_at(men[0]) != was[0]), "seed {s}: behind the broken pane: {a:?}");
        for k in 1..3 {
            let h = p.hit_of(&rep, men[k]).unwrap_or_else(|| panic!("seed {s}: man {k} is not in the report"));
            assert!(matches!(h.result, BlastResult::Shielded) && p.up(men[k]) && p.reeled_at(men[k]) == was[k], "seed {s}: man {k} behind a standing pane: {h:?}");
        }
        for k in 0..3 {
            let judged = p.hit_of(&rep, men[k]).unwrap().exposed;
            let now = p.share_open(burst, at[k]);
            assert!((judged - now).abs() < 1e-9, "seed {s}: man {k}: the blast judged {judged} of him open and {now} is");
        }
    }
}

/// THE RULE: over a grid of points beyond the glazing, a line the blast takes as open is open once the glass has fallen, and no other. Some are open and some are not, so the
/// grid asked something.
/// A FAILURE MEANS: a burst reckons a line by a pane the fracture model then does otherwise with.
#[test]
fn a_line_the_blast_takes_as_open_is_open_once_the_glass_has_fallen() {
    let (s, burst) = (61, [5.2, 11.2]);
    let (mut p, _, _) = glazing(s, [PaneType::Annealed, PaneType::Laminated, PaneType::Tempered]);
    let mut pts = Vec::new();
    for i in 0..9 { for z in [9.5, 9.0, 8.4] { for y in [0.7, 1.1, 1.6, 2.1] { pts.push([5.6 + 0.8 * i as f64, y, z]); } } }
    let lift = p.g.k.frag.lift;
    let power = (p.g.k.frag.glass_m, p.g.k.frag.glass_power);
    let who = 1.0;
    let judged: Vec<bool> = {
        p.run(0.2);
        game::glassbreak::glass_break_blast(&mut p.g, &game::ask::Over::OWN, [burst[0], 0.05 + lift, burst[1]], Some(game::glassbreak::GlassBlastOpts { radius: Some(power.0), power: Some(power.1), who: Some(who), ..game::glassbreak::GlassBlastOpts::default() }));
        pts.iter().map(|&q| p.burst_line_open(burst, q, GlassView::Blown)).collect()
    };
    p.run(2.5);
    let after: Vec<bool> = pts.iter().map(|&q| p.burst_line_open(burst, q, GlassView::Whole)).collect();
    let wrong: Vec<_> = pts.iter().zip(judged.iter().zip(&after)).filter(|(_, (a, b))| a != b).map(|(q, _)| *q).collect();
    assert!(wrong.is_empty() && judged.iter().any(|x| *x) && judged.iter().any(|x| !*x), "seed {s}: of {} lines {} are open and {} disagree: {:?}", pts.len(), judged.iter().filter(|x| **x).count(), wrong.len(), &wrong[..wrong.len().min(4)]);
}

/// A room in two by a wall with a pane in it (x 18, z 11.5 to 12.5; from the floor, or over a sill), and more boxes.
fn cut_by_a_pane(kind: PaneType, sill: bool, more: Vec<Json>) -> (Play, f64) {
    let mut s = Scene::new("slab");
    s.room("long", 10.0, 8.0, 26.0, 16.0).door("long-door", Axis::X, 16.0, 13.0).sam(4.0, 24.0);
    let (c, h) = if sill { (1.9, 1.0) } else { (1.5, 1.5) };
    let mut all = vec![
        a_box("wall-a", [18.0, 1.5, 9.75], [0.12, 1.5, 1.75], "wall", &[]),
        a_box("pane", [18.0, c, 12.0], [0.02, h, 0.5], "white", &["Glass"]),
        a_box("wall-b", [18.0, 1.5, 14.25], [0.12, 1.5, 1.75], "wall", &[]),
    ];
    if sill { all.push(a_box_of("sill", [18.0, 0.45, 12.0], [0.12, 0.45, 0.5], "wall", "concrete")); }
    all.extend(more);
    let mut p = scene_with(&s, all, &Opts::default());
    p.sam_unhurt();
    p.sam_unseen();
    p.g.k.chaos.def = 0.5;
    p.run(0.1);
    let bi = p.pane_at(18.0, 12.0);
    p.pane_is(bi, kind);
    (p, bi)
}
/// between the pane and a burst to be at x 17.2, a slab from floor to ceiling and wall to wall: 30 cm of concrete, or a partition
fn slab(does: &str) -> Vec<Json> { vec![a_box_of("slab", [17.6, 1.5, 12.0], [if does == "concrete" { 0.15 } else { 0.06 }, 1.5, 4.0], "wall", does)] }
/// is a pane not as it was: a piece out, or a row of it made, as any blow makes one
fn touched(p: &mut Play, bi: f64) -> bool { p.pane_open(bi) > 0.0 || p.g.tab.boxes.world().panes.row_of(bi as u32).is_some() }
/// does a man reckon that a burst on the floor at (x, 12) would take the glass at a point of the pane out of the way
fn reckoned_out(p: &mut Play, x: f64, bi: f64, y: f64, view: game::glassbreak::GlassBreakViewOfKind) -> bool {
    let (from, size) = ([x, 0.05 + p.g.k.frag.lift, 12.0], game::glassbreak::GlassBlastOpts { radius: Some(p.g.k.frag.glass_m), power: Some(p.g.k.frag.glass_power), who: None });
    game::glassbreak::glass_break_see_through(&mut p.g, &game::ask::Over::OWN, view, from, size, &mut |g| game::glassbreak::glass_break_rule_open(g, bi, [18.0, y, 12.0]))
}

/// THE RULE: a burst breaks and stars the glass it has a line to and no other, and a man reckoning it counts on no more: 30 cm of concrete between, and a pane 0.8 m off is as it was, of every kind. CONTROL: the same burst with nothing between takes each of them, and he reckoned so.
/// A FAILURE MEANS: windows go in the next room, out of a blast that reached nobody and nothing else there.
#[test]
fn a_burst_breaks_no_glass_behind_a_wall_that_held() {
    use game::glassbreak::GlassBreakViewOfKind::{Counted, Wary};
    for kind in [PaneType::Tempered, PaneType::Annealed, PaneType::Laminated] {
        for walled in [true, false] {
            let (mut p, bi) = cut_by_a_pane(kind, false, if walled { slab("concrete") } else { Vec::new() });
            let how = if walled { "behind 30 cm of concrete" } else { "the control, nothing between" };
            for view in [Wary, Counted] { assert_eq!(reckoned_out(&mut p, 17.2, bi, 0.5, view), !walled, "{kind:?}, {how}: reckoned, {view:?}"); }
            p.burst([17.2, 12.0]);
            p.run(2.0);
            assert_eq!(touched(&mut p, bi), !walled, "{kind:?}, {how}: open {}", p.pane_open(bi));
        }
    }
}

/// THE RULE: a wall a blast opens is no wall to it: the glass behind a partition that went is broken by the same blast. CONTROL: the rule above, where the wall held.
/// A FAILURE MEANS: a hole is blown in a wall and the window a hand's breadth behind the hole has not a crack in it.
#[test]
fn a_burst_breaks_the_glass_behind_a_wall_it_opened() {
    let (mut p, bi) = cut_by_a_pane(PaneType::Tempered, false, slab("plasterboard"));
    p.burst([17.2, 12.0]);
    p.run(2.0);
    assert!(game::wallbreak::wall_break_all(&p.g).iter().any(|w| w.open > 0.0 && (p.g.tab.boxes[w.box_].c[0] - 17.6).abs() < 0.01), "the staging: the partition held");
    assert!(p.pane_open(bi) >= 1.0, "the pane behind the partition that went: open {}", p.pane_open(bi));
}

/// THE RULE: a window over a sill deeper than the glass is reached by a burst on the floor before it, over the sill, straight before the burst. CONTROL: 30 cm of concrete between, above.
/// A FAILURE MEANS: the glass over a sill is out of every grenade's reach, or goes from its far corner.
#[test]
fn a_burst_on_the_floor_reaches_the_glass_over_a_sill() {
    let (mut p, bi) = cut_by_a_pane(PaneType::Annealed, true, Vec::new());
    let from = [17.0, 0.05 + p.g.k.frag.lift, 12.1];
    let met = game::scene_collision::panes_met(&p.g, &game::ask::Over::OWN, from, 4.0);
    let (_, q) = *met.iter().find(|m| m.0 as f64 == bi).expect("the burst meets the pane over the sill");
    assert!((q[2] - 12.1).abs() < 0.01 && q[1] < 1.5, "it meets the pane at {q:?}");
    p.burst([17.0, 12.1]);
    p.run(2.0);
    assert!(p.pane_open(bi) > 0.0, "the glass over the sill is whole");
}

/// THE RULE: a leaf a blast tears off is not in its way: the glass behind a door that went is broken by the same blast. CONTROL: the same glass with the door's frame between and the burst two metres further off, where the leaf stays.
/// A FAILURE MEANS: a grenade takes a door off and the window behind it stands without a crack.
#[test]
fn a_burst_breaks_the_glass_behind_a_door_it_took_off() {
    for (x, goes) in [(17.3, true), (14.6, false)] {
        let mut s = Scene::new("two-rooms");
        s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
        s.door("between", Axis::Z, 18.0, 12.4).door("yard", Axis::X, 16.0, 14.0).sam(4.0, 24.0);
        let mut p = scene_with(&s, vec![a_box("pane", [19.0, 1.5, 13.0], [0.02, 1.5, 1.5], "white", &["Glass"])], &Opts::default());
        p.sam_unhurt();
        p.sam_unseen();
        p.g.k.chaos.def = 0.5;
        p.run(0.1);
        let bi = p.pane_at(19.0, 13.0);
        p.pane_is(bi, PaneType::Tempered);
        let leaf = game::doors::doors_by_name(&p.g, "between").expect("the door");
        let c = p.g.dr[leaf].frame_centre;
        p.burst([x, c[2]]);
        p.run(2.0);
        assert_eq!(p.g.dr[leaf].leaf.off, goes, "the staging: a burst at x {x} and the leaf");
        assert_eq!(p.pane_open(bi) > 0.0, goes, "a burst at x {x}: the glass behind the door is open {}", p.pane_open(bi));
    }
}
