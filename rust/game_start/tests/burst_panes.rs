//! A BURST AND THE GLASS BETWEEN IT AND A MAN: it never hurts a man through a pane it left whole; behind one it broke he is judged by what is open.
mod world_ask;
use game::frag::{BlastResult, GlassView};
use game::glasspen::PaneType;
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use world_ask::GlassAsk;

const GLAZING_Z: f64 = 10.0;

/// The office at night with nobody's mind running and Sam out of it; the conference room's three sheets (x 6 to 9, a sill under them, the room north, the corridor south),
/// each of a kind; the men of the floor with one left at his post.
fn glazing(s: u32, kinds: [PaneType; 3]) -> (Play, [f64; 3], Vec<GuardId>) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.tune.chaos = 0.5;
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
/// and the pane still stands two seconds later. CONTROL: the same man with the burst a metre off the same toughened glass: it comes down and he is hit, so it was the
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
        p.run(2.0);
        let hit = p.hit_of(&rep, men[0]);
        assert!(hit.as_ref().map_or(false, |h| matches!(h.result, BlastResult::Shielded)), "{kind:?} {off} m off: his line of the report {hit:?}");
        assert!(p.up(men[0]) && p.reeled_at(men[0]) == was, "{kind:?} {off} m off: he is {:?}, reeled {}", p.state(men[0]), p.reeled_at(men[0]) != was);
        for b in panes { assert_eq!(p.pane_open(b), 0.0, "{kind:?} {off} m off: pane {b} is open"); }
        drop(p);
    }
    // CONTROL
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
        game::glassbreak::glass_break_blast(&mut p.g, [burst[0], 0.05 + lift, burst[1]], Some(game::glassbreak::GlassBlastOpts { radius: Some(power.0), power: Some(power.1), who: Some(who), ..game::glassbreak::GlassBlastOpts::zeroed() }));
        pts.iter().map(|&q| p.burst_line_open(burst, q, GlassView::Blown)).collect()
    };
    p.run(2.5);
    let after: Vec<bool> = pts.iter().map(|&q| p.burst_line_open(burst, q, GlassView::Whole)).collect();
    let wrong: Vec<_> = pts.iter().zip(judged.iter().zip(&after)).filter(|(_, (a, b))| a != b).map(|(q, _)| *q).collect();
    assert!(wrong.is_empty() && judged.iter().any(|x| *x) && judged.iter().any(|x| !*x), "seed {s}: of {} lines {} are open and {} disagree: {:?}", pts.len(), judged.iter().filter(|x| **x).count(), wrong.len(), &wrong[..wrong.len().min(4)]);
}
