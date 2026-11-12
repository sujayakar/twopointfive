pub use ::game_types::kitload::types;
pub use types::*;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3;
use alloc::{vec, vec::Vec};

/// The guns a card puts on member i, the one in his hands first.
fn guns_of(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> Vec<crate::weapons::WeaponId> {
    let hand = crate::kit::kit_gun(g, m);
    if m.weapon == crate::kit::KitWeapon::Pistol {
        return vec![hand];
    }
    let side = if i == 0.0 {
        let mut pistol_card = m.clone();
        pistol_card.weapon = crate::kit::KitWeapon::Pistol;
        Some(crate::kit::kit_gun(g, &pistol_card))
    } else {
        crate::kit::kit_side(g, m)
    };
    match side {
        Some(side) => vec![hand, side],
        None => vec![hand],
    }
}

/// What one gun takes off the two scores, by its row.
fn gun_load(g: &crate::game::Game, w: crate::weapons::WeaponId) -> crate::kitload::KitLoad {
    match g.m.weapons.weapons.get(w) {
        Some(d) => crate::kitload::KitLoad {
            mob: d.weight_kg * g.k.kitload.gun_kg,
            con: g.k.kitload.gun_bulk[0] + d.bulk * g.k.kitload.gun_bulk[1],
        },
        None => crate::kitload::KitLoad { mob: 0.0, con: 0.0 },
    }
}

/// A cost plus `n` (default 1) of another; no other, no change.
fn add(a: [f64; 2], b: Option<[f64; 2]>, n: Option<f64>) -> [f64; 2] {
    let n = n.unwrap_or(1.0);
    match b {
        Some(b) => [a[0] + b[0] * n, a[1] + b[1] * n],
        None => a,
    }
}

/// The card's count in a field, whole; a field that holds no count is 0.
fn card_count(m: &crate::kit::MemberKit, field: &str) -> f64 {
    js::to_i32(crate::kit::Thing::of(field).map_or(0.0, |k| m.n(k))) as f64
}

/// What ONE row of member i's card takes off his two scores as the card stands (None: a row that weighs nothing).
pub(super) fn load_share(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit, field: &str) -> Option<crate::kitload::KitLoad> {
    let mut c: [f64; 2] = [0.0, 0.0];
    if field == "primary" || field == "secondary" {
        let guns = guns_of(g, i, m);
        let long = m.weapon != crate::kit::KitWeapon::Pistol;
        let picked: &[crate::weapons::WeaponId] = if field == "primary" {
            if long { &guns[..1] } else { &[] }
        } else if long {
            &guns[1..]
        } else {
            &guns
        };
        for &w in picked {
            let x = gun_load(g, w);
            c = add(c, Some([x.mob, x.con]), None);
        }
    } else if field == "big" {
        let big = g.k.kitload.big.of(crate::kit::big_of(m).as_str()).copied();
        c = add(c, big, None);
    } else if field == "wand" {
        let wand = if m.wand { g.k.kitload.small.of("wand").copied() } else { None };
        c = add(c, wand, None);
    } else if let Some(small) = g.k.kitload.small.of(field).copied() {
        c = add(c, Some(small), Some(m::max(0.0, card_count(m, field))));
    }
    if js::truthy(c[0]) || js::truthy(c[1]) {
        Some(crate::kitload::KitLoad { mob: c[0], con: c[1] })
    } else {
        None
    }
}

/// Member i's two scores for the card `m`: 1 less what every row his card HAS takes.
pub(super) fn load_of(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> crate::kitload::KitLoad {
    let (mut mob, mut con) = (0.0, 0.0);
    for r in crate::kit::kit_rows(g, i) {
        if let Some(s) = load_share(g, i, m, r.field.as_str()) {
            mob += s.mob;
            con += s.con;
        }
    }
    crate::kitload::KitLoad { mob: v3::clamp(1.0 - mob, 0.0, 1.0), con: v3::clamp(1.0 - con, 0.0, 1.0) }
}

/// The reference for member i: his own card as issued, to a team of `n`.
pub(super) fn load_ref(g: &crate::game::Game, i: f64, n: Option<f64>) -> crate::kitload::KitLoad {
    let issued = crate::kit::kit_issued_shown(g, i, n);
    load_of(g, i, &issued)
}

/// A score within a hair of his reference IS it: ×1 exactly.
fn snap(x: f64, r: f64) -> f64 {
    if m::abs(x - r) < 1e-9 { r } else { x }
}

/// What member i wears with the card `m` on him.
fn load_worn(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit, n: Option<f64>) -> crate::kitload::KitWorn {
    let l = load_of(g, i, m);
    let r = load_ref(g, i, n);
    crate::kitload::KitWorn { mob: snap(l.mob, r.mob), con: snap(l.con, r.con), ref_mob: r.mob, ref_con: r.con }
}

/// His legs' pace × his reference.
fn load_pace_k(g: &crate::game::Game, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.5, 1.0 + (w.mob - w.ref_mob) * g.k.kitload.pace)
}

/// The notice he builds in an eye × his reference.
fn load_seen_k(g: &crate::game::Game, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.25, 1.0 - (w.con - w.ref_con) * g.k.kitload.seen)
}

/// His footfall's level × his reference.
fn load_heard_k(g: &crate::game::Game, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.25, 1.0 - (w.con - w.ref_con) * g.k.kitload.heard)
}

/// Member i's card has just gone on this man, one of `n` who came (None: it came off, or the staging chose his kit itself).
pub(super) fn load_man(g: &mut crate::game::Game, c: CharId, i: f64, m: Option<&crate::kit::MemberKit>, n: Option<f64>) {
    g.load[c] = m.map(|m| load_worn(g, i, m, n));
}

/// × the speed this man's legs may be asked for.
pub fn load_pace_on(g: &crate::game::Game, c: CharId) -> f64 { g.load[c].map_or(1.0, |w| load_pace_k(g, w)) }

/// × a flat-out run: as much of the pace factor as `sprint` keeps.
pub fn load_sprint_on(g: &crate::game::Game, c: CharId) -> f64 { g.load[c].map_or(1.0, |w| 1.0 + (load_pace_k(g, w) - 1.0) * g.k.kitload.sprint) }

/// The speed a player's legs may be asked for.
pub fn sam_load_pace(g: &crate::game::Game, c: CharId, speed: f64, sprint: Option<bool>) -> f64 {
    match g.load[c] {
        None => speed,
        Some(w) => speed * (if sprint.unwrap_or(false) { load_sprint_on(g, c) } else { load_pace_k(g, w) }),
    }
}

/// The speed an AI man's legs may be asked for; a light card never raises it past `paceTop`. NOT a cap: one call a leg.
pub fn load_pace(g: &crate::game::Game, c: CharId, speed: f64) -> f64 {
    let Some(w) = g.load[c] else { return speed };
    m::min(speed * load_pace_k(g, w), m::max(speed, g.k.kitload.pace_top))
}

/// The speed he would be doing with his issue on him, for "is this step a walk's or a run's".
pub fn load_gait(g: &crate::game::Game, c: CharId, speed: f64) -> f64 {
    let k = load_pace_on(g, c);
    if k == 1.0 { speed } else { speed / k - 1e-6 }
}

/// × the notice this man builds in an eye that has him.
pub fn load_seen(g: &crate::game::Game, c: CharId) -> f64 { g.load[c].map_or(1.0, |w| load_seen_k(g, w)) }

/// × a footfall's level, his.
pub fn load_heard(g: &crate::game::Game, c: CharId) -> f64 { g.load[c].map_or(1.0, |w| load_heard_k(g, w)) }

/// × what a low crossing under an eye costs this man in that eye's notice.
pub fn load_cross(g: &crate::game::Game, c: CharId) -> f64 { g.load[c].map_or(1.0, |w| load_seen_k(g, w) / load_pace_k(g, w)) }

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use super::*;

    /// A FAILURE MEANS: a card's cost is not what it carries: a thing with no cost changes nothing, and n of a thing cost n of it.
    #[test]
    fn a_cost_adds_n_of_another_and_a_thing_with_no_cost_adds_nothing() {
        assert_eq!(add([1.0, 2.0], None, Some(3.0)), [1.0, 2.0]);
        assert_eq!(add([1.0, 2.0], Some([0.5, 0.25]), None), [1.5, 2.25]);
        assert_eq!(add([1.0, 2.0], Some([0.5, 0.25]), Some(2.0)), [2.0, 2.5]);
    }

    /// A FAILURE MEANS: a man in his issue is a hair off his reference, so he walks and sounds a little otherwise for nothing he took.
    #[test]
    fn a_load_within_a_hair_of_the_reference_is_the_reference() {
        assert_eq!(snap(0.5 + 1e-10, 0.5), 0.5);
        assert_ne!(snap(0.5 + 1e-8, 0.5), 0.5);
    }

    /// A FAILURE MEANS: a card's cost counts a part of a thing, or a field that holds no count.
    #[test]
    fn a_cards_count_is_whole_and_a_field_that_holds_none_is_nothing() {
        let mut card = crate::kit::MemberKit::default();
        card.smoke = 2.9;
        assert_eq!(card_count(&card, "smoke"), 2.0);
        assert_eq!(card_count(&card, "suit"), 0.0);
    }

    fn worn(mob: f64, con: f64, ref_mob: f64, ref_con: f64) -> crate::kitload::KitWorn {
        crate::kitload::KitWorn { mob, con, ref_mob, ref_con }
    }

    /// A FAILURE MEANS: what a man carries does not tell on him: at his reference he is as he always was, under it he is slower, a bulkier load is seen and heard more, a lighter less, and none of it past its floor.
    #[test]
    fn a_load_moves_his_pace_and_how_he_is_noticed_down_to_a_floor() {
        let g = Game::blank();
        let at = worn(0.5, 0.5, 0.5, 0.5);
        assert_eq!((load_pace_k(&g, at), load_seen_k(&g, at), load_heard_k(&g, at)), (1.0, 1.0, 1.0));
        let (light, heavy) = (worn(0.6, 0.6, 0.5, 0.5), worn(0.4, 0.4, 0.5, 0.5));
        assert!(load_pace_k(&g, heavy) < 1.0 && 1.0 < load_pace_k(&g, light));
        assert!(load_seen_k(&g, light) < 1.0 && 1.0 < load_seen_k(&g, heavy));
        assert!(load_heard_k(&g, light) < 1.0 && 1.0 < load_heard_k(&g, heavy));
        let far = worn(-100.0, 100.0, 100.0, -100.0);
        assert_eq!((load_pace_k(&g, far), load_seen_k(&g, far), load_heard_k(&g, far)), (0.5, 0.25, 0.25));
    }

    /// A FAILURE MEANS: the guns stop costing by what they are: a heavier one costs more of his pace, a bulkier one more of his cover.
    #[test]
    fn a_heavier_gun_costs_more_pace_and_a_bigger_gun_more_cover() {
        use crate::weapons::WeaponId as W;
        let g = Game::blank();
        let mob: alloc::vec::Vec<f64> = [W::G19bare, W::Mp7, W::Fn2000, W::Mk46].iter().map(|&w| gun_load(&g, w).mob).collect();
        assert!(mob.windows(2).all(|p| 0.0 < p[0] && p[0] < p[1]), "{:?}", mob);
        let con: alloc::vec::Vec<f64> = [W::G19bare, W::Mp7, W::Fn2000, W::Mk12, W::Mk46].iter().map(|&w| gun_load(&g, w).con).collect();
        assert!(con.windows(2).all(|p| 0.0 < p[0] && p[0] < p[1]), "{:?}", con);
    }
}
