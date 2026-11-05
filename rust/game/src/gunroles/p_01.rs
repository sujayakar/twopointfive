//! src/game/gunroles.ts lines 1 to 184
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------ the two tables
/// What a row is for, where it is for something: `ROLE` of a weapon's id (gunroles.ts:69).
fn role_row(id: crate::weapons::WeaponId) -> Option<GunRole> {
    use crate::weapons::WeaponId as W;
    match id {
        W::Mp7 | W::M1014 => Some(GunRole::Room),
        W::Mk12 => Some(GunRole::Reach),
        W::Mk46 | W::Pkm => Some(GunRole::Belt),
        _ => None,
    }
}
/// Who goes through a door when, 0 first and 2 last: `RANK` (gunroles.ts:71).
fn rank_row(r: GunRole) -> f64 {
    match r {
        GunRole::Room => 0.0,
        GunRole::Rifle | GunRole::Pistol => 1.0,
        GunRole::Reach | GunRole::Belt => 2.0,
    }
}
/// A whole number as a template string has it: `-0` reads `0` (gunroles.ts:104, 118).
fn round_text(x: f64) -> String {
    alloc::format!("{}", m::round(x) + 0.0)
}
/// What a man's two draws are hashed from: his place in the cast and his mark; the salt is 0, `SAMESTART.on` being settled (gunroles.ts:103-104).
fn deal_key(i: usize, x: f64, z: f64) -> String {
    alloc::format!("0|{}|{}|{}", i, round_text(x * 10.0), round_text(z * 10.0))
}

/// `WEAPONS[id]`, for a read (gunroles.ts:91, 92, 118). A Firearm's `def` is this table's key, one row an id.
fn row(g: &Game, id: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(id).unwrap()
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
    row(g, g.fa[g.gd[gd].weapon].def)
}

/// Whether a rule of guns is on: all settled on, on the page. (gunroles.ts useOn)
pub fn use_on(g: &mut crate::game::Game, key: crate::gunroles::UseOnKey) -> bool {
    true
}

/// The row of his primary: the long gun on its sling while the sidearm is out, his own while the big slot's shotgun is in his hands. (gunroles.ts gunPrimary)
pub fn gun_primary(g: &mut crate::game::Game, gd: GuardId) -> crate::weapons::WeaponDef {
    let d = primary_id(g, gd);
    row(g, d).clone()
}

/// What his primary is for. (gunroles.ts gunRole)
pub fn gun_role(g: &mut crate::game::Game, gd: GuardId) -> crate::gunroles::GunRole {
    let d = primary_id(g, gd);
    if row(g, d).cls == crate::weapons::WeaponClass::Pistol { return GunRole::Pistol; }
    role_row(d).unwrap_or(GunRole::Rifle)
}

/// His place through a door. (gunroles.ts rankOf)
pub fn rank_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    rank_row(gun_role(g, gd))
}

/// The clear floor from his mark toward what he watches, chest high (m). (gunroles.ts lineOf)
pub fn line_of(g: &mut crate::game::Game, p: V3, to: V3) -> f64 {
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

/// A row of the table into a man's hands: nothing for the row he carries or one switched off. (gunroles.ts gunHand)
pub fn gun_hand(g: &mut crate::game::Game, gd: GuardId, id_2: crate::weapons::WeaponId) {
    let held = g.gd[gd].weapon;
    if g.fa[held].def == id_2 || !row(g, id_2).enabled { return; }
    let route = g.gd[gd].route;
    g.tab.route[route].weapon = Some(id_2);
    let def = row(g, id_2).clone();
    let gun = crate::weapons::firearm_new(g, &def);
    g.gd[gd].weapon = gun;
    g.gd[gd].reload_t = -1.0;
    let c = g.gd[gd].ch;
    g.ch[c].muzzle_ahead = def.muzzle_ahead;
}

/// A cast is dealt its guns, once, with the men on their marks and the doors as the mission opens with them. (gunroles.ts gunDeal)
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
        let mut d = crate::math_dice::die_of(crate::math_dice::hash_str(&deal_key(i, p[0], p[2])));
        let first = crate::math_dice::roll(&mut d);
        let second = crate::math_dice::roll(&mut d);
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
        g.stats.gunroles.dealt += 1;
        if got == W::Pkm { g.stats.gunroles.belt += 1; } else if got == W::Guard9mm { g.stats.gunroles.pistol += 1; } else if got == W::Akms { g.stats.gunroles.akms += 1; }
        out.push(GunDealItem { gd: dm.gd, gun: got, line: line[i] });
    }
    let mut said: Vec<String> = Vec::with_capacity(out.len());
    for o in &out {
        let short = row(g, o.gun).short.replace(' ', "-");
        said.push(alloc::format!("{} {} ({} m)", g.gd[o.gd].callsign, short, round_text(o.line)));
    }
    let text = alloc::format!("guns ▸ {}", said.join(" · "));
    crate::game::game_note(g, &text);
    out
}

/// A defender with the belt-fed gun keeps the post he was dealt. (gunroles.ts gunKeepsPost)
pub fn gun_keeps_post(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].team != crate::teams::Team::Hostile || g.gd[gd].state == crate::game::GuardState::Dead || gun_role(g, gd) != GunRole::Belt { return false; }
    g.stats.gunroles.posts += 1;
    true
}

/// What this dealing of men to stack marks costs for the guns in it. (gunroles.ts gunFileCost)
pub fn gun_file_cost(g: &mut crate::game::Game, r: &[f64], spots: &[V3], front: V3) -> f64 {
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

/// The men's ranks for that price: None for men whose guns are all of one kind. (gunroles.ts gunFileRanks)
pub fn gun_file_ranks(g: &mut crate::game::Game, men: &[GuardId]) -> Option<Vec<f64>> {
    if men.len() < 2 { return None; }
    let r: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    if r.iter().all(|&x| x == r[0]) { None } else { Some(r) }
}

/// Of the men who may go first, the ones whose gun is the room's. (gunroles.ts gunFirst)
pub fn gun_first(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> {
    if men.len() < 2 { return men.to_vec(); }
    let ranks: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    let lo = m::min_n(&ranks);
    let keep: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r == lo).map(|(&x, _)| x).collect();
    if keep.len() == men.len() { return men.to_vec(); }
    g.stats.gunroles.first += 1;
    keep
}

/// Why he went ahead of the lead: his gun goes through a door before the lead's. (gunroles.ts gunBefore)
pub fn gun_before(g: &mut crate::game::Game, first: GuardId, lead: GuardId) -> bool {
    rank_of(g, first) < rank_of(g, lead)
}

/// The turn the men go in by, the long guns last in the order they had; the man picked first never moves. (gunroles.ts gunTurn)
pub fn gun_turn(g: &mut crate::game::Game, turn: &[GuardId], first: GuardId) -> Vec<GuardId> {
    if turn.len() < 3 { return turn.to_vec(); }
    let mut long: Vec<GuardId> = Vec::new();
    for &x in turn {
        if x != first && rank_of(g, x) == 2.0 { long.push(x); }
    }
    if long.is_empty() || long.len() == turn.iter().filter(|&&x| x != first).count() { return turn.to_vec(); }
    let mut out: Vec<GuardId> = turn.iter().copied().filter(|x| !long.contains(x)).collect();
    out.extend_from_slice(&long);
    if out.iter().zip(turn).any(|(a, b)| a != b) { g.stats.gunroles.last += 1; }
    out
}

/// The men through a blown wall, nearest the hole first: the long guns' men after the others. (gunroles.ts gunLast)
pub fn gun_last(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> {
    if men.len() < 2 { return men.to_vec(); }
    let ranks: Vec<f64> = men.iter().map(|&x| rank_of(g, x)).collect();
    let back: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r == 2.0).map(|(&x, _)| x).collect();
    if back.is_empty() || back.len() == men.len() { return men.to_vec(); }
    g.stats.gunroles.last += 1;
    let mut out: Vec<GuardId> = men.iter().zip(&ranks).filter(|(_, &r)| r != 2.0).map(|(&x, _)| x).collect();
    out.extend_from_slice(&back);
    out
}

/// The gap between a blinded defender's rounds: the staging's own for an automatic, the gun's own rate for one that fires a round a pull. (gunroles.ts gunBlindGap)
pub fn gun_blind_gap(g: &mut crate::game::Game, gd: GuardId, every: f64) -> f64 {
    let d = held_row(g, gd);
    if d.modes.contains(&crate::weapons::FireMode::Auto) { every } else { m::max(every, d.cd) }
}

/// What an opening `d` metres off is worth to him over another man, for his gun. (gunroles.ts gunLane)
pub fn gun_lane(g: &mut crate::game::Game, gd: GuardId, d: f64, reach: f64) -> f64 {
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

/// How many rounds his trigger sends before the beat between strings; 0 is the controlled pair. (gunroles.ts gunString)
pub fn gun_string(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let w = g.gd[gd].weapon;
    let d = g.fa[w].def;
    if d != primary_id(g, gd) { return 0.0; }
    let r = gun_role(g, gd);
    if r == GunRole::Belt {
        if let Some(b) = &row(g, d).burst { return m::round((b.min + b.max) / 2.0); }
    }
    if r == GunRole::Reach { 1.0 } else { 0.0 }
}

/// The seconds to the next round inside a string. (gunroles.ts gunStringGap)
pub fn gun_string_gap(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let d = held_row(g, gd);
    match &d.burst {
        Some(b) => b.interval,
        None => d.cd,
    }
}

/// How much wider than his hands the k-th round of a string is drawn. (gunroles.ts gunStringGrow)
pub fn gun_string_grow(g: &mut crate::game::Game, gd: GuardId, k: f64) -> f64 {
    match &held_row(g, gd).burst {
        Some(b) => m::min(b.spread_cap, b.spread_per_round * k),
        None => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_deal_key_reads_as_the_template_string_does() {
        // the TypeScript's own: Math.round(-0.02 * 10) prints 0, Math.round(-12.45 * 10) prints -124
        assert_eq!(deal_key(0, -0.02, 3.14), "0|0|0|31");
        assert_eq!(deal_key(3, -12.45, 4.2), "0|3|-124|42");
    }

    #[test]
    fn the_two_tables_are_the_generated_ones() {
        use crate::weapons::WeaponId as W;
        let t = role();
        for (id, word) in [
            (W::Fiveseven, t.fiveseven), (W::Guard9mm, t.guard9mm), (W::Aks74u, t.aks74u), (W::Ak74m, t.ak74m),
            (W::Akms, t.akms), (W::Fn2000, t.fn2000), (W::M870, t.m870), (W::Mk12, t.mk12), (W::Mp7, t.mp7),
            (W::Mk46, t.mk46), (W::M1014, t.m1014), (W::Hk45, t.hk45), (W::G18, t.g18), (W::Pkm, t.pkm),
            (W::Mk18bare, t.mk18bare), (W::G19bare, t.g19bare),
        ] {
            assert_eq!(role_row(id).map(|r| r.as_str()), word.as_deref());
        }
        let ranks = rank();
        for r in [GunRole::Pistol, GunRole::Rifle, GunRole::Room, GunRole::Reach, GunRole::Belt] {
            assert_eq!(Some(&rank_row(r)), ranks.get(String::from(r.as_str())));
        }
    }
}
