//! WHAT A SEAT IS TOLD OF ITS MEN'S THROWS (`view.throws`; rust/game_rules/src/tick/view/throws.rs): the page draws this and works out nothing of a throw.
use game::orders::Who;
use game::view::{ThrowKind, ThrowLine, Tone as ThrowTone};
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

/// a room with a door on the yard, the team in the yard; a spot of floor in the yard well clear of them, and one round the far corner of the room that no throw from where they stand reaches
fn yard(seed: u32) -> (Scene, [f64; 2], [f64; 2]) {
    let x = 13.4 + nudge(seed, 0);
    let mut s = Scene::new("throw-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, x).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 22.0], NORTH, &[[x - 1.0, 23.0], [x + 1.0, 23.0], [x - 1.0, 24.0], [x + 1.0, 24.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    (s, [x + 12.0 + nudge(seed, 1), 22.0], [8.0 + nudge(seed, 1), 8.0])
}
fn throws(p: &Play) -> Vec<ThrowLine> { p.view().throws }
fn left(p: &Play) -> f64 { p.g.door_tools.frags }

/// THE RULE: an ordered frag shows the man who will throw it, the arc and where it lands, until it leaves his hand, and then nothing. A FAILURE MEANS: the HUD draws a throw nobody was
/// told of or one that has gone. The control is the view before the order, with nothing in it.
#[test]
fn an_ordered_frag_shows_who_and_where_and_goes_when_it_leaves_his_hand() {
    for seed in SEEDS {
        let (s, _, spot) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.3);
        p.g.door_tools.frags = 2.0;
        assert!(throws(&p).is_empty(), "seed {seed}: nothing is ordered yet");
        p.order("fragThere", spot, Who::All);
        let was = left(&p);
        {
            let t = throws(&p);
            assert_eq!(t.len(), 1, "seed {seed}: one man throws: {t:?}");
            let t = &t[0];
            assert_eq!(t.kind, ThrowKind::Frag, "seed {seed}");
            assert!(!t.preview, "seed {seed}: it is an order");
            assert!(p.mates().iter().any(|&m| p.id_of(m) == t.man), "seed {seed}: one of ours: {}", t.man);
            assert!(t.points.len() > 1 && dist([t.land[0], t.land[2]], spot) < 6.0, "seed {seed}: its arc and landing: {t:?}");
            assert!(t.walk.as_ref().is_some_and(|w| w.len() > 1), "seed {seed}: he walks to where he can throw it: {t:?}");
            assert!(t.kill > 0.0 && t.wound > t.kill, "seed {seed}: what a frag reaches: {t:?}");
            let thrower = *p.mates().iter().find(|&&m| p.id_of(m) == t.man).unwrap();
            let pair = game::orders::squad_orders_pair_of(&p.g, thrower).map(|x| if x == game::game::PairId::Red { ThrowTone::Red } else { ThrowTone::Blue });
            assert_eq!(t.tone, pair, "seed {seed}: his pair's name");
            p.until(30.0, |p| !p.g.m.squadthrow.squadthrow_log.is_empty()).unwrap_or_else(|| panic!("seed {seed}: never thrown: {}", p.roll()));
        }
        p.run(0.1);
        assert!(throws(&p).is_empty(), "seed {seed}: it has left his hand: {:?}", throws(&p));
        assert!(left(&p) <= was, "seed {seed}");
    }
}

/// the throws' page opened on `at`, its frag's row lit (the menu's own record, as the ring lights it)
fn lit_frag(p: &mut Play, at: [f64; 2]) -> bool {
    let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
    if !game::orders::squad_orders_open(&mut p.g, Some(&t), None, None) { return false; }
    game::orders::squad_orders_turn(&mut p.g, Some(game::squadthrow::THROW_PAGE), None);
    let row = p.g.orders.menu.as_ref().and_then(|m| m.rows.iter().position(|&d| p.g.tab.orders_order_def[d].id == "fragThere"));
    match (row, p.g.orders.menu.as_mut()) { (Some(i), Some(m)) => { m.sel = i as f64; true } _ => false }
}

/// THE RULE: a lit frag row previews the plan the row stands on (the man, the spot and the walk the press would give), and nothing where the row is refused. A FAILURE MEANS: the HUD shows a
/// throw the press would not give. The control is the same row with no frags left.
#[test]
fn a_lit_frag_row_previews_its_plan_and_nothing_when_refused() {
    for seed in SEEDS {
        let (s, spot, _) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.3);
        p.g.door_tools.frags = 2.0;
        assert!(lit_frag(&mut p, spot), "seed {seed}: the row is on the page");
        p.frame();
        if p.g.orders.menu.is_none() { assert!(lit_frag(&mut p, spot), "seed {seed}"); }
        let v = throws(&p);
        let k = p.g.m.squadthrow.states.get(p.g.player).and_then(|s| s.plans.get(game::squadthrow::SquadThrowKind::Frag)).cloned().expect("the row's plan is kept");
        assert!(k.refuse.is_none(), "seed {seed}: the yard is open ground: {:?}", k.refuse);
        let th = k.thrower.as_ref().expect("a plan not refused has a thrower");
        assert_eq!(v.len(), 1, "seed {seed}: {v:?}");
        assert!(v[0].preview, "seed {seed}");
        assert_eq!(v[0].man, p.id_of(th.gd), "seed {seed}: the man the press would send");
        assert_eq!(v[0].land, th.rest, "seed {seed}: where the row's plan rests");
        assert_eq!(v[0].walk, k.walk, "seed {seed}: the walk already worked out");
        // the control: nothing left, and the row is refused
        game::orders::squad_orders_close(&mut p.g);
        p.g.door_tools.frags = 0.0;
        assert!(lit_frag(&mut p, spot), "seed {seed}: the row stays, greyed");
        p.frame();
        if p.g.orders.menu.is_none() { continue; }
        assert!(throws(&p).iter().all(|t| !t.preview), "seed {seed}: refused and previewed: {:?}", throws(&p));
    }
}

/// THE RULE: a lit shell row previews the shot the tick keeps for it (the man and where it goes off), and nothing where the row is refused. A FAILURE MEANS: the HUD shows a shell the press
/// would not fire. The control is the same row on a spot too near to arm.
#[test]
fn a_lit_shell_row_previews_its_kept_plan_and_nothing_when_refused() {
    let lit_shell = |p: &mut Play, at: [f64; 2]| {
        let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
        assert!(game::orders::squad_orders_open(&mut p.g, Some(&t), None, None));
        game::orders::squad_orders_turn(&mut p.g, Some(game::squadthrow::THROW_PAGE), None);
        let rows = p.g.orders.menu.as_ref().unwrap().rows.clone();
        p.g.orders.menu.as_mut().unwrap().sel = rows.iter().position(|&d| p.g.tab.orders_order_def[d].id == "shellThere").expect("the shell's row is on the page") as f64;
    };
    let kept = |p: &Play| p.g.m.squad40.states.get(p.g.player).and_then(|s| s.plan.clone());
    for seed in SEEDS {
        let (s, spot, _) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        let carrier = *p.mates().iter().min_by(|a, b| dist(p.at(**a), spot).partial_cmp(&dist(p.at(**b), spot)).unwrap()).unwrap();
        game::squad40::squad40_stage(&mut p.g, &[(carrier, 2.0)]);
        p.run(0.5);
        lit_shell(&mut p, spot);
        p.run(0.2);
        let plan = kept(&p).expect("the lit row's plan is kept");
        assert!(plan.no.is_none(), "seed {seed}: open ground: {:?}", plan.no);
        let shot = plan.shot.as_ref().expect("a plan not refused has a shot");
        let v = throws(&p);
        assert_eq!(v.len(), 1, "seed {seed}: {v:?}");
        assert!(v[0].preview, "seed {seed}");
        assert_eq!(v[0].man, p.id_of(shot.gd), "seed {seed}: the man the press would send");
        assert_eq!(v[0].land, shot.burst, "seed {seed}: where the kept plan goes off");
        // the control: too near to arm, and the row is refused
        game::orders::squad_orders_close(&mut p.g);
        let me = p.at(carrier);
        lit_shell(&mut p, [me[0] + 3.0, me[1]]);
        p.run(0.2);
        assert!(kept(&p).and_then(|x| x.no).is_some(), "seed {seed}: a shell 3 m off is shown as one that may be fired");
        assert!(throws(&p).iter().all(|t| !t.preview), "seed {seed}: refused and previewed: {:?}", throws(&p));
    }
}

/// A GAME OF TWO: each is told of his own men's throw. The control is the first player's, empty while only the second's man has one.
#[test]
fn in_a_game_of_two_a_throw_is_told_to_the_seat_whose_man_it_is() {
    for seed in SEEDS {
        let (s, _, spot) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let st = p.g.seats[1];
        game::seats::with_seat(&mut p.g, st, &mut |g| g.door_tools.frags = 2.0);
        let walks_of = |p: &Play| game::seats::seat(&p.g, p.g.seats[1]).door_tools.frags;
        let before = walks_of(&p);
        p.seat_orders(1, "fragThere", spot, Who::All);
        assert!(p.view_of(0).throws.is_empty(), "seed {seed}: not his man's: {:?}", p.view_of(0).throws);
        let two = p.view_of(1).throws;
        assert_eq!(walks_of(&p), before, "seed {seed}: he walks first");
        {
            assert_eq!(two.len(), 1, "seed {seed}: the second player's man is walking to it: {two:?}");
            let mine = p.view_of(1).squad.unwrap();
            assert!(mine.mates.iter().any(|m| m.man == two[0].man), "seed {seed}: one of his own");
        }
    }
}

/// THE RULE: a frag refused for who is in it says WHO: hostages, or a civilian; and floor with nobody unarmed on it is refused to nobody. The man in the room is each in turn.
/// A FAILURE MEANS: the squad names the wrong people, or throws on them. The control is the same spot with him nobody's.
#[test]
fn a_frag_refused_for_who_is_in_it_says_who() {
    for seed in SEEDS {
        let (s, _, _) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.run(0.1);
        let man = p.defender_at([11.25, 11.25]);
        let at = p.at(man);
        let spot = [at[0], 0.0, at[1]];
        let mut says = |hostages: Vec<_>, civilians: Vec<_>| {
            p.g.staged.captives = hostages;
            p.g.civilians = civilians;
            game::squadthrow::squad_frag_catches(&mut p.g, spot, spot, None, Some(false), None, None)
        };
        assert_eq!(says(vec![], vec![]), None, "seed {seed}: nobody unarmed");
        assert_eq!(says(vec![], vec![man]).as_deref(), Some("a civilian is in it. no frag."), "seed {seed}");
        assert_eq!(says(vec![man], vec![]).as_deref(), Some("hostages. no frag."), "seed {seed}");
        assert_eq!(says(vec![man], vec![man]).as_deref(), Some("hostages. no frag."), "seed {seed}: a hostage is said first");
    }
}
