//! THE SEARCH: it deals hide spots round the fix, looks in the dark and covered places first, splits them between the searchers and deals nothing a man could not have got to.
mod world_ask;
use game_start::play::{dist, Opts, Play};
use world_ask::YardAsk;

/// the middle aisle of the office's cubicles, between the four clusters
const FIX: [f64; 2] = [19.9, 18.4];
/// The office at night, Sam out of it, the shift but the first `keep` men parked out of it.
fn office(s: u32, keep: usize) -> (Play, Vec<game::ids::GuardId>) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.player_invisible = true;
    p.g.god_mode = true;
    p.sam_put([37.5, 26.5]);
    p.run(0.15);
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate().skip(keep) { p.parked_far(m, k); }
    (p, men)
}
/// up to `n` spots he looks into, for at most `secs`
fn looks(p: &mut Play, gd: game::ids::GuardId, n: usize, secs: f64) -> Vec<([f64; 2], f64, f64, bool)> {
    let (mut out, mut last): (Vec<([f64; 2], f64, f64, bool)>, Option<[f64; 2]>) = (Vec::new(), None);
    p.until(secs, |p| {
        match p.looking_at(gd) { Some(l) if last != Some(l.0) => { last = Some(l.0); out.push(l); } _ => {} }
        out.len() >= n
    });
    out
}

/// THE RULE: one searcher on a fix in the middle of the floor is dealt ten spots or more; the first three he looks into are cover, not open floor, and darker than the lot he was dealt.
/// A FAILURE MEANS: a searcher walks the open floor and misses the man in the corner: the search no longer reads the light on what it deals.
#[test]
fn a_search_looks_in_the_dark_and_covered_places_first() {
    for s in [301u32, 302, 303] {
        let (mut p, men) = office(s, 1);
        p.searches(men[0], FIX, FIX);
        p.frame();
        let dealt = p.spots(men[0]);
        assert!(dealt.len() >= 10, "seed {s}: {} spots dealt", dealt.len());
        let lot = dealt.iter().map(|x| x.1).sum::<f64>() / dealt.len() as f64;
        let first = looks(&mut p, men[0], 3, 25.0);
        assert_eq!(first.len(), 3, "seed {s}: {first:?}: {}", p.why(men[0]));
        let mean = first.iter().map(|x| x.1).sum::<f64>() / 3.0;
        assert!(first[0].1 > 0.5 && mean > lot && first.iter().all(|x| !x.3 && x.2 >= 0.3), "seed {s}: looked into {first:?}, the lot's mean darkness is {lot:.2}");
    }
}

/// THE RULE: two searchers on one fix split the spots: no two they look into are within a metre of each other. A FAILURE MEANS: two men searching look at one corner together.
#[test]
fn two_searchers_split_the_spots() {
    for s in [311u32, 312] {
        let (mut p, men) = office(s, 2);
        p.searches(men[0], FIX, FIX);
        p.searches(men[1], [20.7, 17.3], FIX);
        let (mut a, mut b): (Vec<[f64; 2]>, Vec<[f64; 2]>) = (Vec::new(), Vec::new());
        p.until(18.0, |p| {
            if let Some(l) = p.looking_at(men[0]) { if a.last() != Some(&l.0) { a.push(l.0); } }
            if let Some(l) = p.looking_at(men[1]) { if b.last() != Some(&l.0) { b.push(l.0); } }
            a.len() >= 2 && b.len() >= 2
        }).unwrap_or_else(|| panic!("seed {s}: looked into {a:?} and {b:?}"));
        for q in &a { for r in &b { assert!(dist(*q, *r) >= 1.0, "seed {s}: both looked at about the same place: {q:?} and {r:?}"); } }
    }
}

/// THE RULE: nothing implausible: a fix by the break room's door with that leaf latched and LOCKED against him: not one spot inside the room is dealt (Sam has no key). CONTROL: latched and not locked: they are dealt, at no plausibility at his first pick.
/// A FAILURE MEANS: a man searches a room nobody could have got into, or never looks behind a door that was only shut.
#[test]
fn a_search_deals_nothing_behind_a_door_he_could_not_have_gone_through() {
    let in_break_room = |q: &[f64; 2]| q[0] > 28.3 && q[1] > 12.4;
    for (locked, s) in [(true, 303u32), (false, 304)] {
        let (mut p, men) = office(s, 1);
        p.door_shut_locked("breakroom_w", locked);
        p.door_shut_locked("breakroom_n", false);
        let fix = [26.6, 17.6];
        p.searches(men[0], fix, fix);
        p.frame();
        let inside: Vec<_> = p.spots(men[0]).into_iter().filter(|x| in_break_room(&x.0)).collect();
        assert!(p.spots(men[0]).len() >= 6, "seed {s}: dealt {:?}", p.spots(men[0]));
        if locked { assert!(inside.is_empty(), "seed {s}: locked against him and dealt {inside:?}"); continue; }
        assert!(!inside.is_empty(), "seed {s}: latched only and nothing dealt behind it");
        p.run(1.2);   // (to his first pick: the sweep)
        game::guards::score_spots(&mut p.g, men[0], None);
        let inside: Vec<_> = p.spots(men[0]).into_iter().filter(|x| in_break_room(&x.0)).collect();
        assert!(inside.iter().all(|x| x.4 == 0.0), "seed {s}: too soon to have slipped through and they are plausible: {inside:?}");
    }
}
