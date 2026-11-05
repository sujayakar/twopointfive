//! src/game/kitload.ts lines 1 to 158
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The guns a card puts on member i, the one in his hands first.   (kitload.ts gunsOf)
pub fn guns_of(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> Vec<crate::weapons::WeaponId> {
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

/// What one gun takes off the two scores, by its row.   (kitload.ts gunLoad)
pub fn gun_load(g: &mut crate::game::Game, w: crate::weapons::WeaponId) -> crate::kitload::KitLoad {
    match g.m.weapons.weapons.get(w) {
        Some(d) => crate::kitload::KitLoad {
            mob: d.weight_kg * g.k.kitload.gun_kg,
            con: g.k.kitload.gun_bulk[0] + d.bulk * g.k.kitload.gun_bulk[1],
        },
        None => crate::kitload::KitLoad { mob: 0.0, con: 0.0 },
    }
}

/// A cost plus `n` (default 1) of another; no other, no change.   (kitload.ts add)
pub fn add(a: [f64; 2], b: Option<[f64; 2]>, n: Option<f64>) -> [f64; 2] {
    let n = n.unwrap_or(1.0);
    match b {
        Some(b) => [a[0] + b[0] * n, a[1] + b[1] * n],
        None => a,
    }
}

/// `Number(m[field]) | 0` for the card's counted fields; a field that holds no count is 0. kitload.ts:98.
fn card_count(m: &crate::kit::MemberKit, field: &str) -> f64 {
    let n = match field {
        "smoke" => m.smoke,
        "bangs" => m.bangs,
        "frags" => m.frags,
        "shells" => m.shells,
        "charges" => m.charges,
        "cams" => m.cams,
        "rings" => m.rings,
        _ => 0.0,
    };
    js::to_i32(n) as f64
}

/// What ONE row of member i's card takes off his two scores as the card stands (None: a row that weighs nothing).   (kitload.ts loadShare)
pub fn load_share(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, field: &str) -> Option<crate::kitload::KitLoad> {
    let mut c: [f64; 2] = [0.0, 0.0];
    if field == "weapon" || field == "primary" || field == "secondary" {
        let guns = guns_of(g, i, m);
        let long = m.weapon != crate::kit::KitWeapon::Pistol;
        let picked: &[crate::weapons::WeaponId] = if field == "weapon" {
            &guns
        } else if field == "primary" {
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
        let big = g.k.kitload.big.get(String::from(crate::kit::big_of(m).as_str())).copied();
        c = add(c, big, None);
    } else if field == "walls" {
        let wall = g.k.kitload.big.get(String::from("wall")).copied();
        c = add(c, wall, Some(m::max(0.0, js::to_i32(m.walls) as f64)));
    } else if field == "wand" {
        let wand = if m.wand { g.k.kitload.small.get(String::from("wand")).copied() } else { None };
        c = add(c, wand, None);
    } else if let Some(small) = g.k.kitload.small.get(String::from(field)).copied() {
        c = add(c, Some(small), Some(m::max(0.0, card_count(m, field))));
    }
    if js::truthy(c[0]) || js::truthy(c[1]) {
        Some(crate::kitload::KitLoad { mob: c[0], con: c[1] })
    } else {
        None
    }
}

/// Member i's two scores for the card `m`: 1 less what every row his card HAS takes.   (kitload.ts loadOf)
pub fn load_of(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> crate::kitload::KitLoad {
    let (mut mob, mut con) = (0.0, 0.0);
    for r in crate::kit::kit_rows(g, i) {
        if let Some(s) = load_share(g, i, m, r.field.as_str()) {
            mob += s.mob;
            con += s.con;
        }
    }
    crate::kitload::KitLoad { mob: v3::clamp(1.0 - mob, 0.0, 1.0), con: v3::clamp(1.0 - con, 0.0, 1.0) }
}

/// The reference for member i: his own card as issued, to a team of `n`.   (kitload.ts loadRef)
pub fn load_ref(g: &mut crate::game::Game, i: f64, n: Option<f64>) -> crate::kitload::KitLoad {
    let issued = crate::kit::kit_issued_shown(g, i, n);
    load_of(g, i, &issued)
}

/// A score within a hair of his reference IS it: ×1 exactly.   (kitload.ts snap)
pub fn snap(x: f64, r: f64) -> f64 {
    if m::abs(x - r) < 1e-9 { r } else { x }
}

/// What member i wears with the card `m` on him; None with the pouches rule off.   (kitload.ts loadWorn)
pub fn load_worn(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, n: Option<f64>) -> Option<crate::kitload::KitWorn> {
    if !g.k.slots.on {
        return None;
    }
    let l = load_of(g, i, m);
    let r = load_ref(g, i, n);
    Some(crate::kitload::KitWorn { mob: snap(l.mob, r.mob), con: snap(l.con, r.con), ref_mob: r.mob, ref_con: r.con })
}

/// loadPaceK of a worn card, whoever wears it. The skeleton's key reaches only a teammate's row; yours is on the Player.
fn pace_k(k: &crate::knobs::KitloadK, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.5, 1.0 + (w.mob - w.ref_mob) * k.pace)
}

/// loadSeenK of a worn card, whoever wears it.
fn seen_k(k: &crate::knobs::KitloadK, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.25, 1.0 - (w.con - w.ref_con) * k.seen)
}

/// loadHeardK of a worn card, whoever wears it.
fn heard_k(k: &crate::knobs::KitloadK, w: crate::kitload::KitWorn) -> f64 {
    m::max(0.25, 1.0 - (w.con - w.ref_con) * k.heard)
}

/// A staged teammate's row: the TypeScript's `w` is never null here, so no row is a caller's bug.
fn man_row(g: &Game, gd: GuardId) -> crate::kitload::KitWorn {
    *g.m.kitload.man_load.get(gd).expect("loadPaceK / loadSeenK / loadHeardK of a man with no row")
}

/// His legs' pace × his reference.   (kitload.ts loadPaceK)
pub fn load_pace_k(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let w = man_row(g, gd);
    pace_k(&g.k.kitload, w)
}

/// The notice he builds in an eye × his reference.   (kitload.ts loadSeenK)
pub fn load_seen_k(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let w = man_row(g, gd);
    seen_k(&g.k.kitload, w)
}

/// His footfall's level × his reference.   (kitload.ts loadHeardK)
pub fn load_heard_k(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let w = man_row(g, gd);
    heard_k(&g.k.kitload, w)
}

/// Your card has just gone on this run's Player (None: it came off, or the staging chose his kit itself).   (kitload.ts loadWear)
pub fn load_wear(g: &mut crate::game::Game, m: Option<&crate::kit::MemberKit>) {
    let worn = match m {
        Some(m) => load_worn(g, 0.0, m, None),
        None => None,
    };
    let p = g.player;
    g.pl[p].kit_load = worn;
}

/// Member i's card has just gone on this teammate, one of `n` who came (None: the staging chose his kit itself).   (kitload.ts loadMan)
pub fn load_man(g: &mut crate::game::Game, gd: GuardId, i: f64, m: Option<&crate::kit::MemberKit>, n: Option<f64>) {
    let worn = match m {
        Some(m) => load_worn(g, i, m, n),
        None => None,
    };
    match worn {
        Some(w) => {
            g.m.kitload.man_load.set(gd, w);
        }
        None => {
            g.m.kitload.man_load.delete(gd);
        }
    }
}

/// What the floor reads for this man; None = nothing of his is touched (a show, nothing weighed on him).   (kitload.ts loadOn)
pub fn load_on(g: &mut crate::game::Game, who: crate::kitload::LoadOnWho) -> Option<crate::kitload::KitWorn> {
    if g.touring {
        return None;
    }
    match who {
        crate::kitload::LoadOnWho::Player => {
            let p = g.player;
            g.pl[p].kit_load
        }
        crate::kitload::LoadOnWho::Guard(gd) => g.m.kitload.man_load.get(gd).copied(),
    }
}

/// × the speed this man's legs may be asked for.   (kitload.ts loadPaceOn)
pub fn load_pace_on(g: &mut crate::game::Game, who: crate::kitload::LoadPaceOnWho) -> f64 {
    let who = match who {
        crate::kitload::LoadPaceOnWho::Guard(gd) => crate::kitload::LoadOnWho::Guard(gd),
        crate::kitload::LoadPaceOnWho::Player => crate::kitload::LoadOnWho::Player,
    };
    match load_on(g, who) {
        Some(w) => pace_k(&g.k.kitload, w),
        None => 1.0,
    }
}

/// × YOUR flat-out run: as much of the pace factor as `sprint` keeps.   (kitload.ts loadSprintOn)
pub fn load_sprint_on(g: &mut crate::game::Game) -> f64 {
    match load_on(g, crate::kitload::LoadOnWho::Player) {
        Some(w) => 1.0 + (pace_k(&g.k.kitload, w) - 1.0) * g.k.kitload.sprint,
        None => 1.0,
    }
}

/// The speed YOUR legs may be asked for.   (kitload.ts samLoadPace)
pub fn sam_load_pace(g: &mut crate::game::Game, speed: f64, sprint: Option<bool>) -> f64 {
    let sprint = sprint.unwrap_or(false);
    match load_on(g, crate::kitload::LoadOnWho::Player) {
        None => speed,
        Some(w) => speed * (if sprint { load_sprint_on(g) } else { pace_k(&g.k.kitload, w) }),
    }
}

/// The speed an AI man's legs may be asked for; a light card never raises it past `paceTop`. NOT a cap: one call a leg.   (kitload.ts loadPace)
pub fn load_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 {
    let w = match load_on(g, crate::kitload::LoadOnWho::Guard(gd)) {
        Some(w) => w,
        None => return speed,
    };
    m::min(speed * pace_k(&g.k.kitload, w), m::max(speed, g.k.kitload.pace_top))
}

/// The speed he would be doing with his issue on him, for "is this step a walk's or a run's".   (kitload.ts loadGait)
pub fn load_gait(g: &mut crate::game::Game, who: crate::kitload::LoadGaitWho, speed: f64) -> f64 {
    let who = match who {
        crate::kitload::LoadGaitWho::Guard(gd) => crate::kitload::LoadOnWho::Guard(gd),
        crate::kitload::LoadGaitWho::Player => crate::kitload::LoadOnWho::Player,
    };
    let k = match load_on(g, who) {
        Some(w) => pace_k(&g.k.kitload, w),
        None => 1.0,
    };
    if k == 1.0 { speed } else { speed / k - 1e-6 }
}

/// × the notice this man builds in an eye that has him.   (kitload.ts loadSeen)
pub fn load_seen(g: &mut crate::game::Game, who: crate::kitload::LoadSeenWho) -> f64 {
    let who = match who {
        crate::kitload::LoadSeenWho::Guard(gd) => crate::kitload::LoadOnWho::Guard(gd),
        crate::kitload::LoadSeenWho::Player => crate::kitload::LoadOnWho::Player,
    };
    match load_on(g, who) {
        Some(w) => seen_k(&g.k.kitload, w),
        None => 1.0,
    }
}

/// × a footfall's level, his.   (kitload.ts loadHeard)
pub fn load_heard(g: &mut crate::game::Game, who: crate::kitload::LoadHeardWho) -> f64 {
    let who = match who {
        crate::kitload::LoadHeardWho::Guard(gd) => crate::kitload::LoadOnWho::Guard(gd),
        crate::kitload::LoadHeardWho::Player => crate::kitload::LoadOnWho::Player,
    };
    match load_on(g, who) {
        Some(w) => heard_k(&g.k.kitload, w),
        None => 1.0,
    }
}

/// × what a low crossing under an eye costs this man in that eye's notice.   (kitload.ts loadCross)
pub fn load_cross(g: &mut crate::game::Game, who: crate::kitload::LoadCrossWho) -> f64 {
    let who = match who {
        crate::kitload::LoadCrossWho::Guard(gd) => crate::kitload::LoadOnWho::Guard(gd),
        crate::kitload::LoadCrossWho::Player => crate::kitload::LoadOnWho::Player,
    };
    match load_on(g, who) {
        Some(w) => seen_k(&g.k.kitload, w) / pace_k(&g.k.kitload, w),
        None => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_scales_or_passes_through() {
        assert_eq!(add([1.0, 2.0], None, Some(3.0)), [1.0, 2.0]);
        assert_eq!(add([1.0, 2.0], Some([0.5, 0.25]), None), [1.5, 2.25]);
        assert_eq!(add([1.0, 2.0], Some([0.5, 0.25]), Some(2.0)), [2.0, 2.5]);
    }

    #[test]
    fn snap_within_a_hair_is_the_reference() {
        assert_eq!(snap(0.5 + 1e-10, 0.5).to_bits(), 0.5f64.to_bits());
        assert_eq!(snap(0.5 + 1e-8, 0.5).to_bits(), (0.5f64 + 1e-8).to_bits());
    }

    #[test]
    fn a_cards_count_is_truncated_as_bitor_zero_does() {
        let mut card = crate::kit::MemberKit::zeroed();
        card.smoke = 2.9;
        card.cams = -1.5;
        assert_eq!(card_count(&card, "smoke"), 2.0);
        assert_eq!(card_count(&card, "cams"), -1.0);
        assert_eq!(card_count(&card, "suit"), 0.0);
    }

    fn worn(mob: f64, con: f64, ref_mob: f64, ref_con: f64) -> crate::kitload::KitWorn {
        crate::kitload::KitWorn { mob, con, ref_mob, ref_con }
    }

    /// Bits of loadPaceK / loadSeenK / loadHeardK from kitload.ts under bun with EXACT on; the last two rows hit the floors and the far side.
    #[test]
    fn the_three_factors_match_the_typescripts_bits() {
        let k = crate::knobs::page_kitload();
        let rows: [(crate::kitload::KitWorn, [u64; 3]); 3] = [
            (worn(0.37, 0.61, 0.52, 0.48), [0x3fee147ae147ae14, 0x3fed810624dd2f1b, 0x3fed16872b020c4a]),
            (worn(0.1, 0.1, 1.0, 1.0), [0x3fe47ae147ae147a, 0x3ff8a3d70a3d70a4, 0x3ffa147ae147ae14]),
            (worn(1.0, 1.0, 0.0, 0.0), [0x3ff6666666666666, 0x3fd999999999999a, 0x3fd3333333333334]),
        ];
        for (w, [p, s, h]) in rows {
            assert_eq!(pace_k(&k, w).to_bits(), p);
            assert_eq!(seen_k(&k, w).to_bits(), s);
            assert_eq!(heard_k(&k, w).to_bits(), h);
        }
    }

    /// Every gun's two costs off the skeleton's real rows, against gunLoad's bits under bun with EXACT on.
    #[test]
    fn every_gun_weighs_what_the_typescript_says() {
        use crate::weapons::WeaponId as W;
        let rows: [(W, u64, u64); 16] = [
            (W::Fiveseven, 0x3fa020c49ba5e355, 0x3fa47ae147ae147b),
            (W::Guard9mm, 0x3fa10624dd2f1aa0, 0x3fa47ae147ae147b),
            (W::Aks74u, 0x3fbae147ae147ae2, 0x3fd851eb851eb852),
            (W::Fn2000, 0x3fc178d4fdf3b646, 0x3fd851eb851eb852),
            (W::M870, 0x3fbe76c8b4395811, 0x3fe199999999999a),
            (W::Mk12, 0x3fc6666666666667, 0x3fe199999999999a),
            (W::Mp7, 0x3fb49ba5e353f7cf, 0x3fcae147ae147ae2),
            (W::Mk46, 0x3fd1eb851eb851ec, 0x3fe70a3d70a3d70b),
            (W::M1014, 0x3fc178d4fdf3b646, 0x3fe199999999999a),
            (W::Hk45, 0x3fa5810624dd2f1b, 0x3fa47ae147ae147b),
            (W::G18, 0x3fa1eb851eb851ec, 0x3fa47ae147ae147b),
            (W::Ak74m, 0x3fc020c49ba5e355, 0x3fe199999999999a),
            (W::Akms, 0x3fbf5c28f5c28f5d, 0x3fe199999999999a),
            (W::Pkm, 0x3fd0cccccccccccd, 0x3fe70a3d70a3d70b),
            (W::Mk18bare, 0x3fbe76c8b4395811, 0x3fd851eb851eb852),
            (W::G19bare, 0x3f9cac083126e97a, 0x3fa47ae147ae147b),
        ];
        let mut g = Game::blank();
        for (w, mob, con) in rows {
            let l = gun_load(&mut g, w);
            assert_eq!((l.mob.to_bits(), l.con.to_bits()), (mob, con), "{}", w.as_str());
        }
    }
}
