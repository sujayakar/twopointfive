//! The blast itself: the wall, the report, who it catches, and the squad that set it goes in behind it.
use super::*;
use crate::game::{DownBy, DownKind, With};

/// Sam's trigger on one charge: the whole of the blow is the team's doing.
pub fn wall_charge_blow(g: &mut Game, c: Option<WallChargeId>, pressed: Option<bool>) -> Option<WallChargeBlow> {
    let c = match c { Some(c) => Some(c), None => wall_charge_nearest(g, None) };
    let pressed = pressed.unwrap_or(true);
    let who = g.player_id(g.player);
    crate::fogofwar::fog_of_war_doing(g, Some(who), &mut |g: &mut Game| blown(g, c, pressed))
}

/// The blast as it went: the wall and how thick it is, the middle of it, who pressed it, whether a hand did (it kills who stands at it), how far it
/// reaches, the hole it made, and the room it opened on. `near` is who it caught on its own side, `seats_near` each player's share of it there.
struct Blow {
    w: WallSpot, tk: f64, xz: V3, who: f64, pressed: bool, reach: f64, hole: Option<WallChargeHoleAt>, inn: V3, room: f64, near: Vec<GuardId>, seats_near: Vec<f64>,
}
impl Blow {
    /// Past the wall, which is no cover now where it opened.
    fn beyond(&self, g: &Game, p: V3) -> bool { wall_off(g, &self.w, p) < -self.tk / 2.0 }
    /// In line with the hole.
    fn through(&self, g: &Game, p: V3) -> bool { self.hole.is_some() && !crate::scene_collision::solid_blocked(g, self.inn, [p[0], 1.1, p[2]], None) }
    fn far(&self, p: V3) -> f64 { v3::dist_xz(p, self.xz) }
    /// The way a man at it is thrown.
    fn away(&self, p: V3) -> V3 { v3::normalize([p[0] - self.xz[0], 0.15, p[2] - self.xz[2]]) }
    /// How long a man on the charge's own side is blind.
    fn near_secs(&self, g: &Game, p: V3) -> f64 { let s = g.k.wallcharge.safe_secs; v3::lerp(s[0], s[1], v3::clamp(self.far(p) / self.reach, 0.0, 1.0)) }
}

fn blown(g: &mut Game, c: Option<WallChargeId>, pressed: bool) -> Option<WallChargeBlow> {
    let c = c?;
    let i = g.sets[g.seat].wall_charge.set.iter().position(|&x| x == c)?;
    g.sets[g.seat].wall_charge.set.remove(i);
    let w = g.tab.wallcharge_set_wall_charge[c].wall.clone();
    crate::holdfire::hold_fire_order(g, WALL_ORDER, Some(crate::holdfire::HoldFireOrderO { blown: Some(true), ..Default::default() }));   // (a charge starts the fight when it GOES)
    let b = wall_opened(g, w, pressed);
    for gd in g.guards.clone() { guard_caught(g, &b, gd); }
    if b.pressed { for cv in crate::civilians::civilians_shown(g) { civilian_caught(g, &b, cv); } }
    let (mut sam, mut seat) = (0.0, 0);
    crate::seats::each_seat(g, &mut |g: &mut Game| { sam = m::max(sam, player_caught(g, &b, b.seats_near[seat])); seat += 1; });
    g.sets[g.seat].wall_charge.last_blow = Some(WallChargeLastBlow { sam });
    if b.hole.is_some() { crate::morale::morale_blast(g, b.inn, Some(b.who)); }   // (the men past the wall it opened are shaken by it)
    let st = &mut g.sets[g.seat].wall_charge.stats;
    st.blown += 1.0;
    if b.hole.is_some() { st.holes += 1.0; }
    squad_behind(g, &b);
    Some(WallChargeBlow { hole: b.hole.is_some() })
}

/// The wall goes. Who the blast catches on its own side is asked once, before it, by the warning's own call (`wall_charge_too_close`): what stood
/// between a man and the charge shielded him, and the note he was given was true, whatever the blast then smashed. What it does beyond the wall is after
/// it: the hole, and the lines through it.
fn wall_opened(g: &mut Game, w: WallSpot, pressed: bool) -> Blow {
    let (tk, at) = (thick(g, &w), charge_at(g, &w));
    let outer = w.outer.unwrap_or(false);
    let breach = if outer && crate::wallbreak::wall_break_adopt(g, w.index) { Some(w.index) } else { None };   // (an outer wall: this blast alone opens it)
    let hung = Some(crate::wallbreak::BlastOptsHung { wall: w.index, r: g.k.wallcharge.hung_m });
    crate::cleared::cleared_rooms_room_at(g, at);   // (the room beyond is named on the floor as it is BEFORE this hole joins two rooms)
    let who = g.player_id(g.player);
    let r = safe_radius(g, &w);
    let near = near_men(g, &w, r);
    let seats_near = near_players(g, &w, r);
    crate::destruction::destruction_blast(g, at, Some(crate::wallbreak::BlastOpts { who: Some(who), chaos: Some(false), breach, hung, ..Default::default() }));
    let hole = wall_charge_hole_at(g, &w);
    let du = hole.map_or(0.0, |h| h.du);
    let base = wall_pt(g, &w, du, 0.0, Some(0.0));
    let (along, surf, pall) = (if w.along_x { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] }, if outer { crate::scene_materials::Surf::Concrete } else { crate::scene_materials::Surf::Drywall }, g.k.wallshot.pall);
    crate::destruction::destruction_pall_wall(g, base, wall_n(&w), along, tk, hole.is_some(), Some(surf), Some(pall));
    crate::floormess::mess_wall(g, base, wall_n(&w), tk, hole.is_some());
    report(g, &w, who);
    let room = hole.map_or(0.0, |h| room_beyond(g, &w, h.du, tk));
    Blow { xz: wall_pt(g, &w, 0.0, 0.0, None), inn: wall_pt(g, &w, du, -(tk + 0.45), Some(1.1)), tk, who, pressed, reach: r.reach, hole, room, near, seats_near, w }
}
/// The men and the civilians it catches on its own side.
fn near_men(g: &Game, w: &WallSpot, r: WallChargeAwayMarksValueR) -> Vec<GuardId> {
    let mut men = g.guards.clone();
    for cv in crate::civilians::civilians_shown(g) { if !men.contains(&cv) { men.push(cv); } }
    men.retain(|&gd| !crate::condition::down(g, g.gd[gd].ch) && wall_charge_blast_share(g, w, g.pos_of(gd), Some(r), Some(false)) > 0.0);
    men
}
/// Each player's share of it there (0: not caught).
fn near_players(g: &mut Game, w: &WallSpot, r: WallChargeAwayMarksValueR) -> Vec<f64> {
    let mut shares: Vec<f64> = Vec::new();
    crate::seats::each_seat(g, &mut |g: &mut Game| {
        let c = g.pl[g.player].ch;
        shares.push(if crate::condition::down(g, c) { 0.0 } else { wall_charge_blast_share(g, w, g.ch[c].pos, Some(r), Some(false)) });
    });
    shares
}
/// Its flash and its bang, with no man to judge: who it hurts is this file's to say.
fn report(g: &mut Game, w: &WallSpot, who: f64) {
    let at = wall_pt(g, w, 0.0, 0.15, Some(0.5));
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, at, [0.0, 0.0, 0.0], Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who), ..Default::default() }));
    g.tab.item[it].fired = true;
    let all = core::mem::take(&mut g.guards);
    crate::combat::detonate_stun(g, it, Some(true), None);
    g.guards = all;
    if let Some(i) = g.items.items.iter().position(|&x| x == it) { g.items.items.remove(i); }
}

/// On the charge's own side a man is caught or not as he stood when it went, and at it, by a hand's press, he is killed. Beyond a wall that held nobody is;
/// beyond one that opened, those in line are felled up close and dazzled farther off, and the rest of the room is shocked.
fn guard_caught(g: &mut Game, b: &Blow, gd: GuardId) {
    if crate::condition::down(g, g.gd[gd].ch) || (b.pressed && g.gd[gd].god) { return; }
    let p = g.pos_of(gd);
    let dist = b.far(p);
    if !b.beyond(g, p) {
        if !b.near.contains(&gd) { return; }
        if b.pressed && dist < g.k.wallkill.near_m { killed(g, b, gd, DownKind::Shot, With::Blast("wall charge")); return; }
        let secs = b.near_secs(g, p);
        wall_charge_dazzle(g, gd, secs, b.inn);
        return;
    }
    if b.hole.is_none() { return; }
    let (ours, knock_m, stun_m) = (g.team(gd).ours(), g.k.doortools.knock_m, g.k.doortools.stun_m);
    let lined = dist <= stun_m && b.through(g, p);
    let in_room = js::truthy(b.room) && !ours && dist <= g.k.wallshot.room_m && crate::cleared::cleared_rooms_room_at(g, p) == b.room;
    let stagger = g.k.wallshot.stagger_secs;
    if !lined {   // (in the room with no line to the hole: the shock, not the flash)
        if in_room { wall_charge_shock(g, gd, stagger, b.inn); }
        return;
    }
    if ours && (if b.pressed { dist >= knock_m } else { crate::orders::squad_orders_big(g, &crate::seats::here(g)) }) {   // (one of ours past the wall: dazzled, not felled)
        let secs = g.k.wallshot.ours_secs;
        wall_charge_dazzle(g, gd, secs, b.inn);
        return;
    }
    if dist < knock_m { killed(g, b, gd, DownKind::Struck, With::Charge); return; }
    let ss = g.k.wallshot.stun_secs;
    wall_charge_dazzle(g, gd, v3::lerp(ss[0], ss[1], v3::clamp(dist / stun_m, 0.0, 1.0)), b.inn);
    crate::condition::fold(g, g.gd[gd].ch, stagger);
    let f = g.gd[gd].weapon;
    g.fa[f].burst = None;
}
fn killed(g: &mut Game, b: &Blow, gd: GuardId, kind: DownKind, with: With) {
    crate::combat::kill_guard(g, gd, b.away(g.pos_of(gd)), Some(false), Some(kind), Some(DownBy::Player), with);
}
/// A civilian in the building is a man on the floor too: at the charge, or behind the wall close up, with the same line.
fn civilian_caught(g: &mut Game, b: &Blow, cv: GuardId) {
    if crate::condition::down(g, g.gd[cv].ch) || g.gd[cv].god { return; }
    let p = g.pos_of(cv);
    if b.beyond(g, p) {
        if b.far(p) < g.k.doortools.knock_m && b.through(g, p) { killed(g, b, cv, DownKind::Struck, With::Charge); }
    } else if b.far(p) < g.k.wallkill.near_m && b.near.contains(&cv) { killed(g, b, cv, DownKind::Shot, With::Hand); }
}
/// What it does to one seat's player, its share of him on the charge's own side (0: not caught there) taken before it went: it answers what it did him of it.
fn player_caught(g: &mut Game, b: &Blow, near: f64) -> f64 {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if b.pressed && !crate::condition::down(g, c) && !g.god_mode {
        let p = g.ch[c].pos;
        let struck = if b.beyond(g, p) { b.far(p) < g.k.doortools.knock_m && b.through(g, p) } else { near > 0.0 && b.far(p) < g.k.wallkill.near_m };
        if struck {
            crate::combat::hit_player(g, b.away(p));
            g.pl[pl].hit_flash = 1.0;
            crate::chronicle::chron_hit(g, Some("the wall charge"));
        }
    }
    if crate::condition::down(g, c) { return 0.0; }
    if near > 0.0 { player_rocked(g, pl, near, b.xz); }
    // (the blast does to the player what it does to a teammate standing there)
    let p = g.player_pos(pl);
    if b.beyond(g, p) {
        if b.far(p) < g.k.doortools.stun_m && b.through(g, p) { let secs = g.k.wallshot.ours_secs; crate::samstun::dazzle_player(g, secs, b.inn); }
    } else if near > 0.0 { let secs = b.near_secs(g, p); crate::samstun::dazzle_player(g, secs, b.inn); }
    near
}
/// In reach of it: knocked along the blast, no round for a beat, his aim open. Never a hit.
fn player_rocked(g: &mut Game, pl: PlayerId, k: f64, from: V3) {
    let c = g.pl[pl].ch;
    let along = v3::sub(g.ch[c].pos, from);
    crate::character::anim_hit_from(g, c, Some(along));
    g.pl[pl].hit_flash = m::max(g.pl[pl].hit_flash, k);
    let (ss, sup) = (g.k.wallcharge.sam_secs, g.k.wallcharge.sam_sup);
    g.arms[c].fire_cd = m::max(g.arms[c].fire_cd, v3::lerp(ss[1], ss[0], k));
    let s = &mut g.cond[c].sup;
    s.v = m::max(s.v, v3::lerp(sup, 1.0, k));
    s.kick = m::max(s.kick, k);
}

/// The squad that set it goes in behind it, after the start it gives them; with no way through, the order is over.
fn squad_behind(g: &mut Game, b: &Blow) {
    for op in g.sets[g.seat].wall_charge.ops.clone() {
        if !same_wall(&o!(g, op).wall, &b.w) || !matches!(o!(g, op).phase, Phase::Place | Phase::Standoff | Phase::Armed) { continue; }
        let Some(h) = b.hole else {
            let r = &o!(g, op);
            let up = core::iter::once(r.lead).chain(r.men.iter().copied()).find(|&m| !crate::condition::down(g, g.gd[m].ch));
            if let Some(up) = up { crate::game::game_say(g, up, "no way through.", None, None); }
            wall_charge_end(g, op);
            continue;
        };
        let t = g.time;
        let r = &mut o!(g, op);
        (r.hole, r.phase, r.t0) = (Some(WallChargeDoneItemHole { du: h.du, half: h.half }), Phase::Wait, t);
        for gd in r.men.clone() {
            if crate::condition::down(g, g.gd[gd].ch) || !still_mine(g, &crate::seats::here(g), op, gd) { continue; }
            if !crate::control::man::blind(g, gd) { let c = g.gd[gd].ch; crate::character::anim_flinch(g, c); }   // (the start at the blast; then round to the wall)
            let spare = o!(g, op).mine.get(gd).is_some_and(|&a| asg!(g, a).spare.unwrap_or(false));
            if !spare { wall_charge_stand_off(g, op, gd); }
        }
    }
}
