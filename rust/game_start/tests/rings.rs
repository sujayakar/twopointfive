//! WHAT A SEAT IS TOLD OF ITS RINGS AND ITS PLAN (`view.ring`, `hand`, `empty`, `plan`; rust/game_rules/src/tick/view/rings.rs, plan.rs, orders/words.rs): the page draws these.
use game::orders::Who;
use game::view::{KeyCap, PlanView, RingView, WaitWhy};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

fn yard(seed: u32) -> (Scene, [f64; 2]) {
    let x = 13.4 + nudge(seed, 0);
    let mut s = Scene::new("ring-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, x).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 22.0], NORTH, &[[x - 1.0, 23.0], [x + 1.0, 23.0], [x - 1.0, 24.0], [x + 1.0, 24.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    (s, [x + 0.6, 16.0])
}
fn ring(p: &Play) -> Option<RingView> { p.view().ring }
/// the squad's key held on a place until the ring is up
fn ring_on(p: &mut Play, at: [f64; 2]) {
    p.point_at(at);
    p.frame();
    let key = p.g.k.cmdring.key.clone();
    p.key_down(&key);
    p.until(2.0, |p| ring(p).is_some()).unwrap_or_else(|| panic!("no ring two seconds after its key went down on {at:?}: {:?}", p.g.sets[p.g.seat].orders.menu.is_some()));
}

/// THE RULE: opening the ring on a door lists the menu's rows on their digits with their why-nots, in the words the tick last read; its first wedge is the quick key's word. A FAILURE
/// MEANS: the ring says something the menu does not. The control is the view before the key, with no ring.
#[test]
fn opening_it_on_a_door_lists_the_menus_rows_with_their_why_nots() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        assert!(ring(&p).is_none(), "seed {seed}: no ring before the key");
        p.g.sets[p.g.seat].door_tools.bangs = 0.0;
        ring_on(&mut p, door);
        let v = ring(&p).unwrap();
        let m = p.g.sets[p.g.seat].orders.menu.clone().unwrap();
        assert!(matches!(m.target, game::orders::OrderTarget::Door(_)), "seed {seed}: on the door: {:?}", m.target);
        assert_eq!(m.words.len(), m.rows.len(), "seed {seed}: each row is worded");
        assert!(v.wedges.len() >= 2, "seed {seed}: a door has rows: {v:?}");
        assert!(v.caption.contains('\u{2192}'), "seed {seed}: who it goes to and what it is on: {}", v.caption);
        for w in &v.wedges {
            if w.opens || w.key == 1.0 { continue; }
            let i = m.words.iter().position(|x| *x == w.text).unwrap_or_else(|| panic!("seed {seed}: {w:?} is no row of {:?}", m.words));
            assert_eq!(w.off, m.why[i].as_ref().is_some_and(|y| !y.is_empty()), "seed {seed}: greyed where a man says why: {w:?} {:?}", m.why[i]);
        }
        assert_eq!(v.wedges.iter().find(|w| w.key == 1.0).map(|w| w.text.clone()), m.quick.clone().filter(|q| !q.is_empty()), "seed {seed}: wedge 1 is the quick word");
        // a row that says what wedge 1 says has no wedge of its own
        assert!(v.wedges.iter().filter(|w| Some(&w.text) == m.quick.as_ref()).count() <= 1, "seed {seed}: {v:?}");
        // no bangs: the row that needs one says so, and is greyed on the ring, a page of 'more' down if it is not on the first
        let i = m.rows.iter().position(|&d| p.g.tab.orders_order_def[d].id == "bangClear").expect("a door has its bang's row");
        let on_first = v.wedges.iter().any(|w| w.text == m.words[i]);
        if !on_first { assert_eq!(more_wedge(&mut p), true, "seed {seed}: its row is behind 'more': {v:?}"); }
        let v2 = ring(&p).unwrap();
        let m2 = p.g.sets[p.g.seat].orders.menu.clone().unwrap();
        let w = v2.wedges.iter().find(|w| w.text == m2.words[i]).unwrap_or_else(|| panic!("seed {seed}: no wedge says {:?}: {v2:?}", m2.words[i]));
        assert!(w.off, "seed {seed}: no bangs and the row is live: {w:?}");
        assert!(m2.why[i].as_deref().unwrap_or("").contains("no bangs"), "seed {seed}: {:?}", m2.why[i]);
    }
}

/// a wedge of 'more' pressed by its digit: true where there was one, and the ring stands on the rest of the rows
fn more_wedge(p: &mut Play) -> bool {
    let Some(w) = ring(p).and_then(|v| v.wedges.into_iter().find(|w| w.text == "more")) else { return false };
    p.key_down(&format!("Digit{}", w.key as i32));
    p.frame();
    p.key_up(&format!("Digit{}", w.key as i32));
    p.g.sets[p.g.seat].orders.cmd_ring.is_some_and(|r| r.more == Some(true))
}

/// THE RULE: the `more` page lists the rest of the rows from wedge 1, and the first page's wedge 1 is no quick word there. A FAILURE MEANS: the ring's other page shows nothing. The control is the first page, whose rows it does not repeat.
#[test]
fn the_more_page_lists_the_rest_of_the_rows() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        ring_on(&mut p, door);
        let first = ring(&p).unwrap();
        assert!(first.wedges.iter().any(|w| w.text == "more"), "seed {seed}: a door has more rows than wedges: {first:?}");
        assert!(more_wedge(&mut p), "seed {seed}: the digit turned the ring");
        let rest = ring(&p).unwrap();
        assert!(!rest.wedges.is_empty() && rest.wedges.iter().all(|w| w.text != "more"), "seed {seed}: {rest:?}");
        assert!(rest.wedges.iter().all(|w| !first.wedges.iter().any(|f| f.text == w.text)), "seed {seed}: the first page's rows again: {first:?} {rest:?}");
    }
}

/// the view's wedges on rows against the tick's own seats, by digit
fn same_as_the_ticks(p: &Play, seed: u32) {
    let (m, r) = (p.g.sets[p.g.seat].orders.menu.clone().unwrap(), p.g.sets[p.g.seat].orders.cmd_ring.unwrap());
    let seat = game::orders::squad_orders_seats(&p.g, &m, Some(r));
    let v = ring(p).unwrap();
    let more = r.more == Some(true);
    for d in 1..=p.g.k.cmdring.wedges as usize {
        let w = v.wedges.iter().find(|w| w.key == d as f64);
        let row = seat.row.get(d).copied().unwrap_or(-1.0);
        if d as f64 == seat.more { assert_eq!(w.map(|w| w.text.as_str()), Some("more"), "seed {seed}: wedge {d}"); continue; }
        if m.page.is_none() && !more && d == 1 { assert_eq!(w.map(|w| w.text.clone()), m.quick.clone().filter(|q| !q.is_empty()), "seed {seed}: wedge 1"); continue; }
        assert_eq!(w.map(|w| w.text.clone()), if row >= 0.0 { Some(m.words[row as usize].clone()) } else { None }, "seed {seed}: wedge {d}: {v:?} {:?}", seat.row);
    }
}

/// THE RULE: a word that changes while the ring is up is in the view the next frame, and the ring the view lays out is the tick's own, whatever its words are by then. A FAILURE MEANS: the HUD shows
/// words the tick does not lay out, or a row that the flat ring's first wedge took. The control is the same ring a frame before, with no man near.
#[test]
fn a_word_that_changes_under_the_ring_is_told_the_next_frame_and_the_wedges_stay_the_ticks() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        p.order("charge", door, Who::All);
        p.until(60.0, |p| p.g.sets[p.g.seat].door_tools.set.len() > 0).unwrap_or_else(|| panic!("seed {seed}: no charge in a minute"));
        p.sam_put([door[0], door[1] + 9.0]);
        let men = p.mates();
        for &gd in &men { p.man_stands(gd, [door[0], door[1] + 9.0]); }
        p.run(0.1);
        ring_on(&mut p, door);
        same_as_the_ticks(&p, seed);
        let says = |p: &Play| { let m = p.g.sets[p.g.seat].orders.menu.as_ref().unwrap(); m.words.iter().chain(m.quick.iter()).filter(|w| w.contains("too close")).count() };
        let before = says(&p);
        assert_eq!(before, 0, "seed {seed}: nobody is near: {:?}", p.g.sets[p.g.seat].orders.menu.as_ref().unwrap().words);
        // a man at the door: the next frame's ring is worded so
        p.man_stands(men[0], [door[0], door[1] + 0.9]);
        p.frame();
        assert!(says(&p) > 0, "seed {seed}: a man stands at the charge and no word says so: {:?} {:?}", p.g.sets[p.g.seat].orders.menu.as_ref().unwrap().words, p.g.sets[p.g.seat].orders.menu.as_ref().unwrap().quick);
        same_as_the_ticks(&p, seed);
        // (its row may be behind 'more')
        if !ring(&p).unwrap().wedges.iter().any(|w| w.text.contains("too close")) { assert!(more_wedge(&mut p), "seed {seed}: no wedge says it and there is no other page: {:?}", ring(&p)); same_as_the_ticks(&p, seed); }
        assert!(ring(&p).unwrap().wedges.iter().any(|w| w.text.contains("too close")), "seed {seed}: the view has it: {:?}", ring(&p));
        // and he goes
        p.man_stands(men[0], [door[0], door[1] + 9.0]);
        p.frame();
        assert_eq!(says(&p), 0, "seed {seed}: he has gone");
        same_as_the_ticks(&p, seed);
    }
}

/// THE RULE: the tap's word is told while it is still what a tap says, and live says the run. A FAILURE MEANS: the word stays up after a step away, or the page takes a show for a run.
#[test]
fn a_taps_word_and_live_are_told() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        assert!(p.view().live, "seed {seed}: the run is live");
        assert!(p.view().empty.is_none(), "seed {seed}: nothing was tapped");
        // on open floor
        p.key_down(game::consts::INTERACT_KEY); p.frame(); p.key_up(game::consts::INTERACT_KEY); p.run(0.4);
        let e = p.view().empty.unwrap_or_else(|| panic!("seed {seed}: a tap on open floor says nothing: {:?} nearby {}", p.g.pl[p.g.player].sel.word, p.g.sets[p.g.seat].nearby.len()));
        assert_eq!((e.word.as_str(), e.kind, e.dist), (game::handring::EMPTY_WORD, game::view::ThingKind::Wall, None), "seed {seed}");
        // a step up to a door takes it down
        p.sam_put([door[0], door[1] + 0.9]);
        p.run(0.2);
        assert!(p.view().empty.is_none(), "seed {seed}: he has rows now: {:?}", p.view().empty);
        // its time is up
        p.sam_put([door[0], door[1] + 7.0]);
        p.run(game::handring::EMPTY_SHOW_SECS + 0.2);
        assert!(p.view().empty.is_none(), "seed {seed}: the word has gone with its time");
    }
}

/// THE RULE: a ring and a throw are in their pair's colour by name. The control: a whole squad in two pairs, whose ring is gold.
#[test]
fn a_ring_wears_the_colour_of_who_it_goes_to() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        ring_on(&mut p, door);
        assert_eq!(ring(&p).unwrap().tone, Some(game::view::Tone::Gold), "seed {seed}: all four");
        // the next order goes to one pair: the ring is theirs, and wears their colour
        game::orders::squad_orders_step_who(&mut p.g, 1.0, None);
        let want = game::orders::squad_orders_pair_of(&p.g, p.g.sets[p.g.seat].orders.menu.as_ref().unwrap().men[0]).map(|x| if x == game::game::PairId::Red { game::view::Tone::Red } else { game::view::Tone::Blue });
        assert!(want.is_some(), "seed {seed}");
        assert_eq!(ring(&p).unwrap().tone, want, "seed {seed}: {:?}", ring(&p).map(|r| r.caption));
    }
}

/// THE RULE: the lit wedge follows the cursor, off the tick's own state. A FAILURE MEANS: the view's lit wedge lags the one a click would give. The control: nothing is lit with the cursor at the middle.
#[test]
fn the_lit_wedge_follows_the_cursor() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        ring_on(&mut p, door);
        let mut seen = std::collections::BTreeSet::new();
        for k in 0..24 {
            let a = k as f64 / 24.0 * std::f64::consts::TAU;
            p.point_at([door[0] + a.sin() * 3.0, door[1] + a.cos() * 3.0]);
            p.frame();
            let Some(v) = ring(&p) else { break };
            assert_eq!(v.lit, p.g.sets[p.g.seat].orders.cmd_ring.unwrap().lit, "seed {seed}: the tick's lit wedge");
            seen.insert(v.lit as i32);
        }
        assert!(seen.len() >= 2, "seed {seed}: the cursor went round and the lit wedge stayed: {seen:?}");
    }
}

/// ASKING WRITES NOTHING, with a ring up and a plan on the floor. A FAILURE MEANS: a read the HUD makes changes the run.
#[test]
fn asking_writes_nothing_with_a_ring_up() {
    for seed in SEEDS {
        let run = |ask: bool| -> Vec<Vec<u32>> {
            let (s, door) = yard(seed);
            let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
            p.sam_unhurt();
            p.run(0.2);
            p.order("stack", door, Who::All);
            let mut out = Vec::new();
            for k in 0..120 {
                if k == 30 { let key = p.g.k.cmdring.key.clone(); p.point_at(door); p.key_down(&key); }
                p.frame();
                if ask { let v = p.view(); assert!(!v.plan.own.men.is_empty(), "seed {seed}: the plan on the floor has nobody"); }
                if k % 30 == 29 { out.push(p.words()); }
            }
            out
        };
        assert_eq!(run(false), run(true), "seed {seed}");
    }
}

/// A GAME OF TWO: a ring is its seat's. The control is the other seat, whose ring is none.
#[test]
fn in_a_game_of_two_the_ring_is_the_seats_who_opened_it() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        assert!(p.view_of(0).ring.is_none() && p.view_of(1).ring.is_none(), "seed {seed}");
        let st = p.g.seats[1];
        let input = p.host.input;
        game::seats::with_seat(&mut p.g, st, &mut |g| {
            let t = game::orders::squad_orders_target_at(g, [door[0], 0.0, door[1]]);
            assert!(game::orders::squad_orders_open(g, Some(&t), Some(true), None));
            g.sets[g.seat].orders.cmd_ring = Some(game::cmdring::ring_open(g, input, 800.0, 600.0, 0.0, None));
        });
        let two = p.view_of(1).ring.expect("the second player's ring");
        assert!(p.view_of(0).ring.is_none(), "seed {seed}: the first player's is shut");
        assert!(two.wedges.len() >= 2, "seed {seed}: {two:?}");
        // the ring goes to the second player's men and is worded by his own menu
        let m = game::seats::seat(&p.g, st).orders.menu.clone().unwrap();
        let men = p.view_of(1).squad.unwrap().mates;
        assert!(m.men.iter().all(|&gd| men.iter().any(|x| x.man == p.id_of(gd))) && !m.men.is_empty(), "seed {seed}: his own men: {:?}", men);
        assert!(two.wedges.iter().all(|w| w.opens || w.key == 1.0 || m.words.contains(&w.text)), "seed {seed}: {two:?} {:?}", m.words);
    }
}

/// the rows of a hand's words: his ring's wedge says a row up to its dash
#[test]
fn his_own_ring_is_told_of_the_rows_in_reach() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        assert!(p.view().hand.is_none(), "seed {seed}: no ring before the key");
        p.sam_put([door[0], door[1] + 0.9]);
        p.run(0.3);
        p.key_down(game::consts::INTERACT_KEY);
        p.until(2.0, |p| p.view().hand.is_some()).unwrap_or_else(|| panic!("seed {seed}: his ring did not come up: {:?}", p.g.sets[p.g.seat].nearby.iter().map(|r| r.line2.clone()).collect::<Vec<_>>()));
        let h = p.view().hand.unwrap();
        assert!(h.wedges.iter().any(|w| w.text != "whistle"), "seed {seed}: a door's rows: {h:?}");
        assert!(h.wedges.iter().all(|w| !w.text.starts_with("Space") && !w.text.contains(" · ")), "seed {seed}: its words up to the dot, less the key: {h:?}");
        assert_eq!(p.view_of(0).ring.is_none(), true, "seed {seed}: the squad's ring is another");
    }
}

fn plan(p: &Play, seat: usize) -> PlanView { p.view_of(seat).plan }

/// THE RULE: an order just given shows who goes and where, and goes out; with the menu up it stands; what waits on the key is listed on its door, and the key is lit only with the men set.
/// A FAILURE MEANS: the floor shows a plan the squad does not have. The control is the plan before any order, with nothing in it.
#[test]
fn the_plan_shows_an_order_and_what_waits_on_the_key() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let v = plan(&p, 0);
        assert!(v.own.men.is_empty() && v.own.doors.is_empty(), "seed {seed}: nothing is ordered: {v:?}");
        p.order("stack", door, Who::All);
        p.frame();
        let v = plan(&p, 0);
        assert!(!v.own.men.is_empty(), "seed {seed}: the men just sent are drawn");
        assert!(v.own.men.iter().all(|m| m.tag.len() <= 3 && !m.tag.is_empty()), "seed {seed}: {v:?}");
        let walking = v.own.men.iter().find_map(|m| m.way.clone().map(|w| (m.mark, w))).expect("somebody walks");
        assert_eq!(walking.1.stops.last(), Some(&walking.0), "seed {seed}: his way ends on his mark");
        p.run(10.0);
        assert!(plan(&p, 0).own.men.is_empty(), "seed {seed}: with no menu up an order goes out with its age: {:?}", plan(&p, 0).own.men);
        p.until(20.0, |p| plan(p, 0).own.doors.iter().any(|d| d.why == WaitWhy::Stack && d.cap == KeyCap::Lit)).unwrap_or_else(|| panic!("seed {seed}: the stack never lit its key: {:?}", plan(&p, 0).own.doors));
        let d = plan(&p, 0).own.doors.into_iter().find(|d| d.why == WaitWhy::Stack).unwrap();
        assert_eq!(d.door.as_deref(), Some("west-yard"), "seed {seed}");
        assert!(d.now && d.key.is_some(), "seed {seed}: the key sends them: {d:?}");
        ring_on(&mut p, door);
        let v = plan(&p, 0);
        assert!(v.own.mode && !v.own.men.is_empty() && !v.own.who.is_empty() && v.own.you.is_some(), "seed {seed}: the menu up: the plan stands, and who it goes to: {v:?}");
    }
}

/// A GAME OF TWO: the plan is about the seat handed, with the other's as another's: no key of his on it. The control is the second's own plan, with its key.
#[test]
fn in_a_game_of_two_the_plan_is_the_seats_and_the_others_is_listed_without_its_key() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        p.seat_orders(1, "stack", door, Who::All);
        p.frame();
        let (one, two) = (plan(&p, 0), plan(&p, 1));
        assert!(one.own.men.is_empty(), "seed {seed}: the first has ordered nothing: {:?}", one.own.men);
        assert!(!two.own.men.is_empty(), "seed {seed}: the second's men are his own");
        assert_eq!(one.others.len(), 1, "seed {seed}");
        assert_eq!(one.others[0].men.iter().map(|m| m.man).collect::<Vec<_>>(), two.own.men.iter().map(|m| m.man).collect::<Vec<_>>(), "seed {seed}");
        p.until(20.0, |p| plan(p, 1).own.doors.iter().any(|d| d.key.is_some())).unwrap_or_else(|| panic!("seed {seed}: nothing waits on the second's key: {:?}", plan(&p, 1).own.doors));
        let one = plan(&p, 0);
        assert!(one.others[0].doors.iter().all(|d| d.key.is_none() && d.cap == KeyCap::None), "seed {seed}: the press is its own player's: {:?}", one.others[0].doors);
        let faint = p.view_of(0).look.plan.others;
        assert!(faint > 0.0 && faint < 1.0, "seed {seed}: another's plan is drawn weaker than his own: {faint}");
        p.g.k.plan.others = 0.0;
        assert!(plan(&p, 0).others.is_empty() && plan(&p, 1).own.doors.iter().any(|d| d.key.is_some()), "seed {seed}: at none it is not listed, and the second's own still is");
    }
}

/// a man's tag on the floor: 'Red Two' is R2, a callsign of one word its letter
#[test]
fn a_tag_is_two_characters_of_his_callsign() {
    assert_eq!((game::view::plan_tag("Red Two").as_str(), game::view::plan_tag("Blue One").as_str(), game::view::plan_tag("Kestrel").as_str(), game::view::plan_tag("").as_str()), ("R2", "B1", "K", ""));
}
