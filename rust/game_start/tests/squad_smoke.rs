//! SMOKE ON ORDER: counted off the cards; with none issued the row is not on the menu.
mod doors_ask;
use doors_ask::*;
use game::game::GuardState;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn stood(s: u32, squad: u32) -> Play {
    let (sc, _) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad, ..Default::default() });
    p.sam_unhurt();
    p.men_spared();
    p.run(0.2);
    p
}
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [14.0, 4.0], 0.0, GuardState::Patrol, 0.0); p.run(0.2); }

/// THE RULE: every card as issued: no row on any menu, nothing to order, no can in the world. The control is the same squad with a can on one card: after a restart the row is there and the count is that card's. A card of a man who did
/// not come brings nothing; the cards put back and a restart take the row away.
/// A FAILURE MEANS: the menu offers smoke nobody carries, or carrying it does not show.
#[test]
fn smoke_comes_off_the_cards_and_none_issued_is_no_row() {
    for s in SEEDS {
        let mut p = stood(s, 4);
        assert!(!p.on_menu("smokeThere"), "seed {s}: as issued, a row");
        assert_eq!(p.smoke_left(), 0.0, "seed {s}");
        assert!(p.try_order("smokeThere", [20.0, 24.0], Who::All).is_some(), "seed {s}: an order for smoke nobody carries was taken");
        assert_eq!(p.smoke_cans().len(), 0, "seed {s}");
        p.cards_carry(&[(1, 2.0)]);
        p.run(0.2);
        assert!(p.on_menu("smokeThere") && p.smoke_left() == 2.0, "seed {s}: two on a card: left {}", p.smoke_left());
        p.cards_carry(&[(1, 0.0)]);
        p.run(0.2);
        assert!(!p.on_menu("smokeThere") && p.smoke_left() == 0.0, "seed {s}: the cards back as issued");
        drop(p);

        let mut p = stood(s, 2);
        p.cards_carry(&[(1, 1.0), (4, 2.0)]);
        p.run(0.2);
        assert_eq!(p.smoke_left(), 1.0, "seed {s}: the pair: the count is its two men's");
    }
}

/// THE RULE: two cans: the nearest man with a throw tosses one that comes to rest near the spot and vents; he keeps the order he was on and nobody else moves; the count drops. A silent squad stays silent. At none a third is refused in
/// the row's words, a spot nobody can reach too. A restart puts the count back and no can is left. The control is the spot out of reach.
/// A FAILURE MEANS: a smoke order walks the squad off its marks, spends a can for nothing, or goes loud.
#[test]
fn a_teammate_throws_smoke_on_order() {
    for s in SEEDS {
        let mut p = stood(s, 4);
        hush(&mut p);
        p.cards_carry(&[(1, 2.0)]);
        p.run(0.2);
        hush(&mut p);
        let sam = p.sam_at();
        p.order("hold", sam, Who::All);
        p.order("silent", sam, Who::All);
        p.run(1.5);
        let men = p.men();
        let was: Vec<[f64; 2]> = men.iter().map(|&m| p.at(m)).collect();
        assert_eq!(p.try_order("smokeThere", [36.0, 26.0], Who::All).as_deref(), Some("no throw from here."), "seed {s}: the control: out of reach");
        assert_eq!(p.smoke_left(), 2.0, "seed {s}: no can for it");
        let spot = [18.5, 21.0];
        assert_eq!(p.try_order("smokeThere", spot, Who::All), None, "seed {s}");
        assert_eq!(p.smoke_left(), 1.0, "seed {s}: the count");
        p.until(8.0, |p| p.smoke_cans().iter().any(|c| c.1)).unwrap_or_else(|| panic!("seed {s}: no can vented"));
        let (at, _) = *p.smoke_cans().iter().find(|c| c.1).unwrap();
        assert!(dist(at, spot) < 1.5, "seed {s}: it came to rest {:.1} m off", dist(at, spot));
        for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), was[i]) < 0.3 && p.doing(m).as_deref() == Some("hold"), "seed {s}: man {i}: {:?}", p.doing(m)); }
        assert!(p.said_it("smoke out.") && p.silent(), "seed {s}");
        assert_eq!(p.try_order("smokeThere", [11.0, 21.0], Who::All), None, "seed {s}");
        assert_eq!(p.smoke_left(), 0.0, "seed {s}");
        assert_eq!(p.try_order("smokeThere", [11.0, 21.0], Who::All).as_deref(), Some("no smoke left."), "seed {s}");
        p.cards_carry(&[(1, 2.0)]);
        p.run(0.2);
        let (left, cans) = (p.smoke_left(), p.smoke_cans().len());
        assert!(left == 2.0 && cans == 0, "seed {s}: a restart: {left} left, {cans} cans");
    }
}
