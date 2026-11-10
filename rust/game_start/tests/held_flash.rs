//! THE STUN CAN: it blinds the room and spares its own side, and a bang and clear's can goes into the room from a man's hand.
mod kit_ask;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

/// Two lit rooms with a door each to the moonlit yard south of them and one between them; the team in the yard; a defender in each room.
fn yard(s: u32) -> (Play, Scene) {
    let mut sc = Scene::new("yard");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 13.4 + 0.5 * nudge(s, 0)).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    sc.team([14.0, 23.5], std::f64::consts::PI, &[[13.0, 21.5], [15.0, 21.5], [13.0, 22.5], [15.0, 22.5]]);
    sc.defender("west", [13.0 + 0.5 * nudge(s, 1), 12.5], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    (p, sc)
}

/// THE RULE: a can let off inside an open door blinds and folds the man it has a line to, and he fires nothing while folded; the team stood outside in plain view of it, past its reach of their own, is untouched. The same can behind the shut door (the control) touches nobody inside.
/// A FAILURE MEANS: a stun can is no room's answer any more, blinds its own side, or goes through a door.
#[test]
fn a_stun_blinds_the_room_and_spares_its_own_side() {
    for s in [1, 2, 3] {
        let (mut p, sc) = yard(s);
        let (d, c) = (p.door("west-yard"), sc.door_centre("west-yard"));
        let man = p.defender_at([13.0 + 0.5 * nudge(s, 1), 12.5]);
        p.drop_all_but(&[man]);
        let mates = p.mates();
        p.leaf_left_at(d, 1.7);
        p.flash_lands([c[0], c[1] - 1.4]);
        p.until(3.0, |p| p.blind(man)).unwrap_or_else(|| panic!("seed {s}: the can never blinded the man with a line to it ({})", p.why(man)));
        assert!(p.folded(man) > 0.0, "seed {s}: blind and not folded");
        let rounds = p.rounds_fired(man);
        p.until(10.0, |p| { assert_eq!(p.rounds_fired(man), rounds, "seed {s}: a round while folded"); p.folded(man) == 0.0 }).expect("he stayed folded");
        assert!(!p.any_blind(&mates), "seed {s}: the team outside was blinded");
        drop(p);

        let (mut p, sc) = yard(s);
        let c = sc.door_centre("west-yard");
        let man = p.defender_at([13.0 + 0.5 * nudge(s, 1), 12.5]);
        p.drop_all_but(&[man]);
        p.flash_lands([c[0], c[1] + 1.0]);
        p.always(4.0, |p| !p.blind(man)).map(|t| panic!("seed {s}: the control: a can outside the shut door blinded him after {t:.1} s"));
    }
}

/// THE RULE: "they often don't really throw the flash bang into the room": on a bang and clear the can first shows at a man's own hand and goes off inside the room, well in from the door; nobody of ours is blinded. The control: open and clear throws no can.
/// A FAILURE MEANS: the can appears in the frame out of nobody's hand, or goes off in the doorway on the team's own eyes.
#[test]
fn a_bang_and_clears_can_goes_into_the_room_from_the_hand() {
    for s in [1, 2, 3] {
        let (mut p, sc) = yard(s);
        let c = sc.door_centre("west-yard");
        let man = p.defender_at([13.0 + 0.5 * nudge(s, 1), 12.5]);
        p.drop_all_but(&[man]);
        let mates = p.mates();
        p.order("bangClear", c, Who::All);
        let (mut first, mut mates_blind) = (None, false);
        let mut near_hand = f64::MAX;
        p.until(40.0, |p| {
            if let (None, Some(&at)) = (first, p.cans().first()) { first = Some(at); near_hand = mates.iter().map(|&m| dist(p.at(m), [at[0], at[2]])).fold(f64::MAX, f64::min); }
            mates_blind |= p.any_blind(&mates);
            p.last_bang().is_some()
        }).unwrap_or_else(|| panic!("seed {s}: 40 s and no can went off: first {first:?} ({})", p.roll()));
        p.run(0.5);
        mates_blind |= p.any_blind(&mates);
        let at = p.last_bang().unwrap();
        assert!(near_hand < 0.8, "seed {s}: the can first shows {near_hand:.2} m from any man");
        assert!(c[1] - at[2] >= 1.5, "seed {s}: it went off {:.2} m inside the room", c[1] - at[2]);
        assert!(!mates_blind, "seed {s}: one of ours was blinded");
        drop(p);

        let (mut p, sc) = yard(s);
        let man = p.defender_at([13.0 + 0.5 * nudge(s, 1), 12.5]);
        p.drop_all_but(&[man]);
        p.order("openClear", sc.door_centre("west-yard"), Who::All);
        p.always(8.0, |p| p.cans().is_empty()).map(|t| panic!("seed {s}: the control: open and clear showed a can after {t:.1} s"));
    }
}
