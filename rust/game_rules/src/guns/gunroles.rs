pub use ::game_types::gunroles::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

// ------------------------------------------------------------ the two tables
/// What a row is for, where it is for something: `ROLE` of a weapon's id.
fn role_row(id: crate::weapons::WeaponId) -> Option<GunRole> {
    use crate::weapons::WeaponId as W;
    match id {
        W::Mp7 | W::M1014 => Some(GunRole::Room),
        W::Mk12 => Some(GunRole::Reach),
        W::Mk46 | W::Pkm => Some(GunRole::Belt),
        _ => None,
    }
}
/// Who goes through a door when, 0 first and 2 last: `RANK`.
fn rank_row(r: GunRole) -> f64 {
    match r {
        GunRole::Room => 0.0,
        GunRole::Rifle | GunRole::Pistol => 1.0,
        GunRole::Reach | GunRole::Belt => 2.0,
    }
}
/// A whole number as a template string has it: `-0` reads `0`.
fn round_text(x: f64) -> String {
    alloc::format!("{}", m::round(x) + 0.0)
}
/// What a man's two draws are hashed from: his place in the cast and his mark; the salt is 0, `SAMESTART.on` being settled.
fn deal_key(i: usize, x: f64, z: f64) -> String {
    alloc::format!("0|{}|{}|{}", i, round_text(x * 10.0), round_text(z * 10.0))
}

/// The id of the row of his primary: `gunPrimary` without the copy of the row, for the rules here that ask only which row it is.
fn primary_id(g: &Game, gd: GuardId) -> crate::weapons::WeaponId {
    if let Some(s) = &g.gd[gd].second {
        if s.out { return g.fa[s.gun].def; }
    }
    if let Some(s) = &g.gd[gd].slung {
        if s.out { return g.fa[s.own].def; }
    }
    g.fa[g.gd[gd].weapon].def
}
/// The row in his hands, `gd.weapon.def`.
fn held_row(g: &Game, gd: GuardId) -> &crate::weapons::WeaponDef {
    g.weapon(g.fa[g.gd[gd].weapon].def)
}

/// What his primary is for.
fn gun_role(g: &crate::game::Game, gd: GuardId) -> crate::gunroles::GunRole {
    let d = primary_id(g, gd);
    if g.weapon(d).cls == crate::weapons::WeaponClass::Pistol { return GunRole::Pistol; }
    role_row(d).unwrap_or(GunRole::Rifle)
}

/// His place through a door.
fn rank_of(g: &crate::game::Game, gd: GuardId) -> f64 {
    rank_row(gun_role(g, gd))
}

/// The clear floor from his mark toward what he watches, chest high (m).
fn line_of(g: &crate::game::Game, p: V3, to: V3) -> f64 {
    let dx = to[0] - p[0];
    let dz = to[2] - p[2];
    let l = m::hyp2(dx, dz);
    if l < 0.05 { return 0.0; }
    let line_max = g.k.gunroles.line_max;
    match crate::scene_collision::raycast(g, [p[0], 1.2, p[2]], [dx / l, 0.0, dz / l], line_max) {
        Some(hit) => hit.t,
        None => line_max,
    }
}

/// A row of the table into a man's hands: nothing for the row he carries or one switched off.
fn gun_hand(g: &mut crate::game::Game, gd: GuardId, id_2: crate::weapons::WeaponId) {
    let held = g.gd[gd].weapon;
    if g.fa[held].def == id_2 || !g.weapon(id_2).enabled { return; }
    let route = g.gd[gd].route;
    g.tab.route[route].weapon = Some(id_2);
    let def = g.weapon(id_2).clone();
    let gun = crate::weapons::firearm_new(g, &def);
    g.gd[gd].weapon = gun;
    g.gd[gd].reload_t = -1.0;
    let c = g.gd[gd].ch;
    g.ch[c].muzzle_ahead = def.muzzle_ahead;
}

/// A cast is dealt its guns, once, with the men on their marks and the doors as the mission opens with them.
pub fn gun_deal(g: &mut crate::game::Game, men: &[crate::gunroles::DealMan]) -> Vec<crate::gunroles::GunDealItem> {
    use crate::weapons::WeaponId as W;
    if men.is_empty() { return Vec::new(); }
    let mut at = 0;
    while at < g.doors.list.len() {
        let d = g.doors.list[at];
        crate::doors::door_place(g, d);
        at += 1;
    }
    let mut line: Vec<f64> = Vec::with_capacity(men.len());
    for dm in men {
        let c = g.gd[dm.gd].ch;
        let p = g.ch[c].pos;
        let l = line_of(g, p, dm.watch);
        line.push(l);
    }
    let mut draws: Vec<[f64; 2]> = Vec::with_capacity(men.len());
    for (i, dm) in men.iter().enumerate() {
        let c = g.gd[dm.gd].ch;
        let p = g.ch[c].pos;
        let mut d = crate::dice::Die(js::to_u32(crate::dice::hash_str(&deal_key(i, p[0], p[2]))));
        let first = d.roll();
        let second = d.roll();
        draws.push([first, second]);
    }
    let n = men.len() as f64;
    let (belt_lane, belt_min, belt_squad) = (g.k.gunroles.belt_lane, g.k.gunroles.belt_min, g.k.gunroles.belt_squad);
    let (pistol_near, pistol_share, pistol_max) = (g.k.gunroles.pistol_near, g.k.gunroles.pistol_share, g.k.gunroles.pistol_max);
    let akms = g.k.gunroles.akms;
    let holds_hostage = |dm: &DealMan| dm.hostage == Some(true);
    // the belt-fed gun: the longest line, if it is a lane or the cast is a big one
    let mut belt: Option<usize> = None;
    for (i, dm) in men.iter().enumerate() {
        if !holds_hostage(dm) && belt.map_or(true, |b| line[i] > line[b] + 1e-6) { belt = Some(i); }
    }
    if let Some(b) = belt {
        if !((line[b] >= belt_lane && n >= belt_min) || n >= belt_squad) { belt = None; }
    }
    // the pistols: of the men on a near post whose die says so, the lowest draws first, a share of the cast at most
    let mut near: Vec<usize> = (0..men.len())
        .filter(|&i| Some(i) != belt && !holds_hostage(&men[i]) && line[i] < pistol_near && draws[i][0] < pistol_share)
        .collect();
    js::sort_by(&mut near, |&a, &b| js::or(draws[a][0] - draws[b][0], a as f64 - b as f64));
    near.truncate(m::floor(n * pistol_max) as usize);
    let mut out: Vec<GunDealItem> = Vec::with_capacity(men.len());
    for (i, dm) in men.iter().enumerate() {
        if holds_hostage(dm) {
            // not dealt: the gun his scenario gave him
            let w = g.gd[dm.gd].weapon;
            out.push(GunDealItem { gd: dm.gd, gun: g.fa[w].def, line: line[i] });
            continue;
        }
        let gun = if Some(i) == belt { W::Pkm } else if near.contains(&i) { W::Guard9mm } else if draws[i][1] < akms { W::Akms } else { W::Aks74u };
        gun_hand(g, dm.gd, gun);
        let w = g.gd[dm.gd].weapon;
        let got = g.fa[w].def;
        if got == W::Pkm {  } else if got == W::Guard9mm {  } else if got == W::Akms {  }
        out.push(GunDealItem { gd: dm.gd, gun: got, line: line[i] });
    }
    let mut said: Vec<String> = Vec::with_capacity(out.len());
    for o in &out {
        let short = g.weapon(o.gun).short.replace(' ', "-");
        said.push(alloc::format!("{} {} ({} m)", g.gd[o.gd].callsign, short, round_text(o.line)));
    }
    out
}

/// A defender with the belt-fed gun keeps the post he was dealt.
pub fn gun_keeps_post(g: &crate::game::Game, gd: GuardId) -> bool {
    if g.team(gd) != crate::teams::Team::Hostile || crate::condition::down(g, g.gd[gd].ch) || gun_role(g, gd) != GunRole::Belt { return false; }
    true
}

/// What this dealing of men to stack marks costs for the guns in it.
pub fn gun_file_cost(g: &crate::game::Game, r: &[f64], spots: &[V3], front: V3) -> f64 {
    let d: Vec<f64> = spots.iter().map(|s| m::hyp2(s[0] - front[0], s[2] - front[2])).collect();
    let file_cost = g.k.gunroles.file_cost;
    let mut c = 0.0;
    for a in 0..r.len() {
        for b in 0..r.len() {
            if r[a] < r[b] && d[a] > d[b] + 0.25 { c += file_cost; }
        }
    }
    c
}

/// The men's ranks for that price: None for men whose guns are all of one kind.
pub fn gun_file_ranks(g: &crate::game::Game, men: &[GuardId]) -> Option<Vec<f64>> {
    if men.len() < 2 { return None; }
    let r: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    if r.iter().all(|&x| x == r[0]) { None } else { Some(r) }
}

/// Of the men who may go first, the ones whose gun is the room's.
pub fn gun_first(g: &crate::game::Game, men: &[GuardId]) -> Vec<GuardId> {
    if men.len() < 2 { return men.to_vec(); }
    let ranks: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    let lo = m::min_n(&ranks);
    let keep: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r == lo).map(|(&x, _)| x).collect();
    if keep.len() == men.len() { return men.to_vec(); }
    keep
}

/// Why he went ahead of the lead: his gun goes through a door before the lead's.
pub fn gun_before(g: &crate::game::Game, first: GuardId, lead: GuardId) -> bool {
    rank_of(g, first) < rank_of(g, lead)
}

/// The turn the men go in by, the long guns last in the order they had; the man picked first never moves.
pub fn gun_turn(g: &crate::game::Game, turn: &[GuardId], first: GuardId) -> Vec<GuardId> {
    if turn.len() < 3 { return turn.to_vec(); }
    let mut long: Vec<GuardId> = Vec::new();
    for &x in turn {
        if x != first && rank_of(g, x) == 2.0 { long.push(x); }
    }
    if long.is_empty() || long.len() == turn.iter().filter(|&&x| x != first).count() { return turn.to_vec(); }
    let mut out: Vec<GuardId> = turn.iter().copied().filter(|x| !long.contains(x)).collect();
    out.extend_from_slice(&long);
    out
}

/// The men through a blown wall, nearest the hole first: the long guns' men after the others.
pub fn gun_last(g: &crate::game::Game, men: &[GuardId]) -> Vec<GuardId> {
    if men.len() < 2 { return men.to_vec(); }
    let ranks: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    let back: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r == 2.0).map(|(&x, _)| x).collect();
    if back.is_empty() || back.len() == men.len() { return men.to_vec(); }
    let mut out: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r != 2.0).map(|(&x, _)| x).collect();
    out.extend_from_slice(&back);
    out
}

/// The gap between a blinded defender's rounds: the staging's own for an automatic, the gun's own rate for one that fires a round a pull.
pub fn gun_blind_gap(g: &crate::game::Game, gd: GuardId, every: f64) -> f64 {
    let d = held_row(g, gd);
    if d.modes.contains(&crate::weapons::FireMode::Auto) { every } else { m::max(every, d.cd) }
}

/// What an opening `d` metres off is worth to him over another man, for his gun.
pub fn gun_lane(g: &crate::game::Game, gd: GuardId, d: f64, reach: f64) -> f64 {
    let r = gun_role(g, gd);
    let k = m::min(1.0, m::max(0.0, d / m::max(reach, 0.1)));
    let lane_k = g.k.gunroles.lane_k;
    match r {
        GunRole::Reach | GunRole::Belt => lane_k * k,
        GunRole::Pistol => -lane_k * k,
        GunRole::Room => -0.5 * lane_k * k,
        GunRole::Rifle => 0.0,
    }
}

/// How many rounds his trigger sends before the beat between strings; 0 is the controlled pair.
pub fn gun_string(g: &crate::game::Game, gd: GuardId) -> f64 {
    let w = g.gd[gd].weapon;
    let d = g.fa[w].def;
    if d != primary_id(g, gd) { return 0.0; }
    let r = gun_role(g, gd);
    if r == GunRole::Belt {
        if let Some(b) = &g.weapon(d).burst { return m::round((b.min + b.max) / 2.0); }
    }
    if r == GunRole::Reach { 1.0 } else { 0.0 }
}

/// The seconds to the next round inside a string.
pub fn gun_string_gap(g: &crate::game::Game, gd: GuardId) -> f64 {
    let d = held_row(g, gd);
    match &d.burst {
        Some(b) => b.interval,
        None => d.cd,
    }
}

/// How much wider than his hands the k-th round of a string is drawn.
pub fn gun_string_grow(g: &crate::game::Game, gd: GuardId, k: f64) -> f64 {
    match &held_row(g, gd).burst {
        Some(b) => m::min(b.spread_cap, b.spread_per_round * k),
        None => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::WeaponId as W;
    use alloc::{boxed::Box, vec};

    fn man(g: &mut Game, id: W) -> GuardId {
        let weapon = g.fa.push(crate::weapons::Firearm { def: id, ..crate::weapons::Firearm::blank() });
        let mut row = crate::game::Guard::blank();
        row.weapon = weapon;
        g.gd.push(row)
    }

    /// Every man's draws go by his place in the cast and his mark to the tenth of a metre, so a level deals its guns the same every time, and a mark a hair either side of zero is one mark.
    #[test]
    fn a_mans_draws_go_by_his_place_and_his_mark_to_the_tenth() {
        assert_eq!(deal_key(2, 4.01, -7.02), deal_key(2, 3.99, -6.98));
        assert_ne!(deal_key(2, 4.0, 7.0), deal_key(3, 4.0, 7.0));
        assert_ne!(deal_key(2, 4.0, 7.0), deal_key(2, 4.2, 7.0));
        assert_ne!(deal_key(2, 4.0, 7.0), deal_key(2, 7.0, 4.0));
        assert_eq!(deal_key(0, -0.02, 0.0), deal_key(0, 0.02, 0.0));
    }

    /// The room guns go through a door first; a stack of one kind has nobody to put ahead.
    #[test]
    fn the_room_gun_goes_through_a_door_first() {
        let mut g = Box::new(Game::blank());
        let (room, rifle, pistol, reach, belt) = (man(&mut g, W::Mp7), man(&mut g, W::Aks74u), man(&mut g, W::Guard9mm), man(&mut g, W::Mk12), man(&mut g, W::Pkm));
        assert_eq!(gun_first(&mut g, &[rifle, reach, room]), vec![room]);
        assert_eq!(gun_first(&mut g, &[reach, rifle, pistol]), vec![rifle, pistol], "a pistol goes when a rifle does");
        assert_eq!(gun_first(&mut g, &[rifle, pistol]), vec![rifle, pistol]);
        assert!(gun_before(&g, room, rifle) && gun_before(&g, rifle, belt) && !gun_before(&g, rifle, pistol) && !gun_before(&g, reach, belt));
    }

    /// The long guns go in last, in the order they had, but the man picked first stays first.
    #[test]
    fn the_long_guns_go_in_last() {
        let mut g = Box::new(Game::blank());
        let (room, rifle, reach, belt) = (man(&mut g, W::M1014), man(&mut g, W::Ak74m), man(&mut g, W::Mk12), man(&mut g, W::Mk46));
        assert_eq!(gun_turn(&mut g, &[reach, rifle, belt, room], rifle), vec![rifle, room, reach, belt]);
        assert_eq!(gun_turn(&mut g, &[reach, rifle, belt, room], reach), vec![reach, rifle, room, belt]);
        assert_eq!(gun_last(&mut g, &[belt, rifle, reach, room]), vec![rifle, room, belt, reach]);
        assert_eq!(gun_last(&mut g, &[belt, reach]), vec![belt, reach], "all long: the order they had");
    }

    /// A long gun wants the far opening and a short one the near, a rifle minds neither, and it counts for more the further it is.
    #[test]
    fn a_far_opening_is_for_the_long_gun() {
        let mut g = Box::new(Game::blank());
        let (room, rifle, pistol, reach, belt) = (man(&mut g, W::Mp7), man(&mut g, W::Aks74u), man(&mut g, W::Guard9mm), man(&mut g, W::Mk12), man(&mut g, W::Pkm));
        let at = |g: &Game, m: GuardId, d: f64| gun_lane(g, m, d, 20.0);
        assert!(at(&g, reach, 20.0) > 0.0 && at(&g, reach, 20.0) == at(&g, belt, 20.0));
        assert!(at(&g, reach, 20.0) > at(&g, reach, 5.0));
        assert_eq!(at(&g, reach, 40.0), at(&g, reach, 20.0), "past his reach it is worth no more");
        assert_eq!(at(&g, rifle, 20.0), 0.0);
        assert!(at(&g, pistol, 20.0) < at(&g, room, 20.0) && at(&g, room, 20.0) < 0.0);
    }
}
