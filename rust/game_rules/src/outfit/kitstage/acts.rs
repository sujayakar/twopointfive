//! THE KIT PICTURE'S ACTS: the man whose card is open uses the thing under the card's highlight, at the door, again and again. Every action is the game's: the squad's orders to a teammate, Sam's own keys and his hand (`sam_hand`) for
//! Sam, doors crack and pull_to, the door tools' trigger and reset, the wall charge's spot, blow and reset, `fire_weapon` for a round. A pick cuts whatever is running and that thing begins at once, from where he stands.
//! What a row leaves that has no undo is held as a resting picture, then the room is staged again under the page's dissolve.
use super::aim::{self, aim_legs, arrived, chan, chan_tick, ch_of, clamp, lim, off, own, quiet_tick, rest_yaw, wrap, Step, R};
use super::light::{KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK, KIT_VIEW_STAGE as ST};
use super::plan::KIT_PLAN;
use super::quiet;
use super::types::*;
use super::walk::{self, along, at, away, flat, free, go, home_of, low, low_at, out_of, rotate, sam_goes, sam_goes_low, sam_stand, step_out, v2, v3, walk_of};
use crate::doors::{door_is_closed, doors_crack, doors_pull_to};
use crate::game::{Game, GuardState, InteractableKind, SlotKind, Verb};
use crate::ids::*;
use crate::kit::{BigKit, MemberKit};
use crate::m;
use crate::orders::{OrderTarget, Who};
use crate::throwables as th;
use crate::weapons::RifleAttachment;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// what a row does to the leaf: pulled to while it plays · his hand holds it · the door tools' own order has it
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaf { Shut, Open, None }
/// what a row leaves that only a restage mends
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Damage { None, Leaf, World }
/// A row: what it does to the leaf, what it leaves, it is loud, the whole team is the squad's own order's, it works a leaf (refused with none), it needs an opening (a blank wall: the wall charge first), seconds of rest before it plays again, seconds of game time at most
#[derive(Clone, Copy, Debug)]
pub struct Row { pub door: Leaf, pub damage: Damage, pub loud: bool, pub team: bool, pub leaf: bool, pub opening: bool, pub rest: f64, pub cap: f64 }
const fn base(door: Leaf, damage: Damage, rest: f64, cap: f64) -> Row { Row { door, damage, loud: false, team: false, leaf: false, opening: false, rest, cap } }
pub(super) fn row(a: Act) -> Row {
    match a {
        Act::Gun => Row { loud: true, ..base(Leaf::Shut, Damage::None, K.rest_quiet, K.gun_secs + 9.0) },
        Act::GunDoor => Row { loud: true, ..base(Leaf::Open, Damage::None, K.rest_quiet * 2.0, K.gun_secs + 9.0) },
        Act::Light => Row { opening: true, ..base(Leaf::Open, Damage::None, K.rest_quiet, 12.0) },
        Act::BangOrder => Row { loud: true, leaf: true, ..base(Leaf::None, Damage::None, K.rest_can, 10.0) },
        Act::FragOrder => Row { loud: true, leaf: true, ..base(Leaf::None, Damage::World, K.hold, 10.0) },
        Act::ThrowSmoke => Row { opening: true, ..base(Leaf::Open, Damage::None, K.rest_can, 14.0) },
        Act::ThrowFlash => Row { opening: true, loud: true, ..base(Leaf::Open, Damage::None, K.rest_can, 14.0) },
        Act::ThrowFrag => Row { opening: true, loud: true, ..base(Leaf::Open, Damage::World, K.hold, 14.0) },
        Act::ChargeMate | Act::ChargeSam => Row { loud: true, leaf: true, ..base(Leaf::None, Damage::Leaf, K.hold, 12.0) },
        Act::WandSam => Row { leaf: true, ..base(Leaf::Shut, Damage::None, K.rest_quiet, 13.0) },
        Act::WandMate => Row { leaf: true, ..base(Leaf::None, Damage::None, K.rest_quiet, 12.0) },
        Act::Cam => Row { leaf: true, ..base(Leaf::Open, Damage::None, K.rest_can, 10.0) },
        Act::Shell | Act::Ring => base(Leaf::Shut, Damage::None, K.rest_quiet, 12.0),
        Act::WallCharge => Row { loud: true, team: true, ..base(Leaf::Shut, Damage::World, K.hold, 26.0) },
        Act::BigMate | Act::BigSam => Row { loud: true, leaf: true, ..base(Leaf::None, Damage::Leaf, K.hold, 14.0) },
        Act::Turn => base(Leaf::Shut, Damage::None, K.rest_quiet, 10.0),
        Act::Swap => base(Leaf::Shut, Damage::None, K.rest_quiet, 5.0),
        Act::QuietGun => base(Leaf::Shut, Damage::World, QK.gun_rest, 14.0),
        Act::QuietLean | Act::QuietLight => base(Leaf::Shut, Damage::None, K.rest_quiet, 8.0),
    }
}
/// THE TABLE: a thing on a card to how a teammate and Sam use it at the door. A thing with no row plays the gun's.
fn beats(f: &str, mate: bool) -> Option<Act> {
    Some(match (f, mate) {
        ("weapon", true) | ("weapon", false) => Act::Gun,
        ("light", _) => Act::Light,
        ("swap", _) => Act::Swap,
        ("bangs", true) => Act::BangOrder, ("bangs", false) => Act::ThrowFlash,
        ("frags", true) => Act::FragOrder, ("frags", false) => Act::ThrowFrag,
        ("smoke", _) => Act::ThrowSmoke,
        ("charges", true) => Act::ChargeMate, ("charges", false) => Act::ChargeSam,
        ("wand", true) => Act::WandMate, ("wand", false) => Act::WandSam,
        ("cams", false) => Act::Cam, ("shells", false) => Act::Shell, ("rings", false) => Act::Ring,
        ("walls", _) => Act::WallCharge,
        ("bigtool", true) => Act::BigMate, ("bigtool", false) => Act::BigSam,
        ("turn", _) => Act::Turn,
        _ => return None,
    })
}
/// THE QUIET KIND'S ROWS, over the table. A thing not here, and a quiet row that refuses, plays its row of the table.
fn quiet_beats(f: &str, mate: bool) -> Option<Act> {
    if mate { return None; }
    match f { "weapon" => Some(Act::QuietGun), "turn" => Some(Act::QuietLean), "light" => Some(Act::QuietLight), _ => None }
}
fn row_for(st: &KitStage, f: &str, mate: bool) -> Option<Act> { (if st.quiet { quiet_beats(f, mate) } else { None }).or(beats(f, mate)) }
/// THE TILES: a tile's field to the thing it plays, by the card it is on (a field not here plays the row of its own name; none: the gun, then the light)
pub fn thing(g: &Game, f: &str, i: u32) -> Option<&'static str> {
    let c = g.m.kit.kit.members.get(i as usize);
    match f {
        "weapon" | "primary" | "secondary" | "suppressed" | "sideCan" => Some("weapon"),
        "suit" | "trim" | "nvg" => Some("turn"),
        "big" => { let c = c?; if c.walls > 0.0 { Some("walls") } else if c.big != BigKit::None { Some("bigtool") } else { None } }
        "light" => Some("light"), "bangs" => Some("bangs"), "frags" => Some("frags"), "smoke" => Some("smoke"), "charges" => Some("charges"), "wand" => Some("wand"),
        "cams" => Some("cams"), "shells" => Some("shells"), "rings" => Some("rings"), "walls" => Some("walls"), "bigtool" => Some("bigtool"),
        _ => None,
    }
}
/// the man of the open card (member i is the team's [i - 1])
pub(super) fn actor(g: &Game, st: &KitStage) -> Option<GuardId> { if st.who > 0 { g.staged.team.get(st.who as usize - 1).copied() } else { None } }
/// the card asked for, if its man is on the team as it now stands. True: another card.
pub(super) fn open_card(g: &Game, st: &mut KitStage) -> bool {
    let n = g.staged.team.len() as u32;
    let i = if st.want >= 1 && st.want <= n { st.want } else { 0 };
    if i == st.who { return false; }
    st.who = i;
    st.a.flip = 0;
    true
}
/// A RESTAGE THE PAGE SHOULD COVER: nothing is directed until it has laid its copy, or half a second of game time from now.
pub(super) fn ask_veil(g: &Game, st: &mut KitStage) {
    if st.veil_asked.is_some() { return; }
    st.veil = st.veil.wrapping_add(1);
    st.veil_asked = Some(g.time);
    st.veil_laid = false;
}
pub(super) fn press(st: &mut KitStage, key: &'static str) { if !st.a.press.contains(&key) { st.a.press.push(key); } }
fn unpress(st: &mut KitStage, key: &'static str) { st.a.press.retain(|k| *k != key); }
pub(super) fn to(g: &Game, b: &mut Beat, phase: &'static str) { b.phase = phase; b.pt = g.time; }
fn mark(g: &Game, b: &mut Beat, word: &'static str) { b.marks.push(word); if b.mark_t.is_nan() { b.mark_t = g.time; } }
fn pos(g: &Game, b: &Beat) -> V3 { g.ch[b.ch_id].pos }
fn ch_id(g: &Game, b: &Beat) -> f64 { g.ch[b.ch_id].id }
fn walled(st: &KitStage) -> bool { st.walled && !st.hole }
fn the_door(st: &KitStage) -> DoorId { st.door.expect("a row that works a leaf is refused where there is none") }
fn door_steel(g: &Game, d: DoorId) -> bool { g.dr[d].def.steel == Some(true) }

// ------------------------------------------------------------ the leaf, where there is one
fn crack(g: &mut Game, st: &KitStage, b: &Beat, wide: f64) { if let Some(d) = st.door { let (p, id) = (pos(g, b), ch_id(g, b)); doors_crack(g, d, p, id, st.a.step_dt, None, Some(K.swing), Some(wide)); } }
fn wide_open(g: &Game, st: &KitStage, wide: f64) -> bool { st.door.map_or(true, |d| m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= wide - 0.1) }
fn is_ajar(g: &Game, st: &KitStage) -> bool { st.door.map_or(false, |d| !g.dr[d].leaf.off && m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= 0.03) }
/// not yet shut for a row that works a shut leaf: ajar, or on the quiet picture still on its way
fn leaf_wait(g: &Game, st: &KitStage) -> bool { is_ajar(g, st) || (st.leaf_rest != st.leaf_shut && st.door.map_or(false, |d| !g.dr[d].leaf.off && (!g.dr[d].leaf.latched || g.dr[d].leaf.closing))) }
pub(super) fn off_rest(g: &Game, st: &KitStage) -> bool { st.door.map_or(false, |d| !g.dr[d].leaf.off && m::abs(g.dr[d].leaf.angle - st.leaf_rest) >= 0.03) }
fn unlock(g: &mut Game, st: &KitStage) { if let Some(d) = st.door { g.dr[d].leaf.locked = false; } }
fn on_door(st: &KitStage) -> OrderTarget { OrderTarget::Door(crate::orders::OrderTargetDoor { at: [st.foot[0], 0.0, st.foot[2]], door: the_door(st), side: st.side }) }
fn floor(p: V3) -> OrderTarget { OrderTarget::Floor(crate::orders::OrderTargetFloor { at: [p[0], 0.0, p[2]], ..crate::orders::OrderTargetFloor::default() }) }
fn roster(g: &Game, x: GuardId) -> Who { Who::Num(crate::orders::squad_orders_squad(g, &crate::seats::here(g)).iter().position(|&o| o == x).map_or(-1.0, |i| i as f64)) }
fn order_def(g: &Game, seat: &crate::seats::SeatRef, id: &str) -> Option<OrdersOrderDefId> { seat.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == id) }
fn assignment_order(g: &Game, x: GuardId) -> Option<String> { crate::orders::squad_orders_of(&crate::seats::here(g), x).map(|a| g.tab.orders_assignment[a].order.clone()) }
fn door_op(g: &Game, seat: &crate::seats::SeatRef, d: DoorId) -> Option<DoorOpId> { seat.door_tools.ops.iter().copied().find(|&o| g.tab.doortools_door_op[o].door == d) }

// ------------------------------------------------------------ the game's rows held for a row
/// the men's own legs off a live grenade are held for the length of SAM'S frag row by the game's own row
pub(super) fn stay(g: &mut Game, st: &mut KitStage, on: bool) {
    if !on { st.a.pin = None; st.a.knees.clear(); }
    if on { if st.a.flee_m.is_none() { st.a.flee_m = Some(g.k.frag.flee_m); } g.k.frag.flee_m = 0.0; }
    else if let Some(f) = st.a.flee_m.take() { g.k.frag.flee_m = f; }
}
/// the game's rows a row holds, put back as they were: the tables outlast a staging
pub(super) fn tables_back(g: &mut Game, st: &mut KitStage) {
    stay(g, st, false);
}
// ------------------------------------------------------------ cans
/// the thing thrown has gone off: it says so, or it has left the list, as a thing that has gone off does
fn item_went(g: &Game, b: &Beat) -> bool { b.item.map_or(false, |i| !g.items.items.contains(&i) || g.tab.item[i].fired) }
fn cans(g: &Game) -> f64 { g.items.items.iter().filter(|&&i| g.tab.item[i].kind == th::ItemKind::Smoke && g.tab.item[i].fired).count() as f64 }
/// the smoke cans venting are shut: the game's own emitters, and the word to the picture's solver where one was venting
pub(super) fn shut_cans(g: &mut Game) {
    let mut any = false;
    for k in 0..g.items.items.len() { let i = g.items.items[k]; if g.tab.item[i].kind == th::ItemKind::Smoke && g.tab.item[i].fired && g.tab.item[i].emit_t > 0.0 { g.tab.item[i].emit_t = 0.0; any = true; } }
    crate::sense::smoke_shut(g);
    if any { crate::out_events::smoke_cans_shut(g); }
}
/// the air empty, for the lamp's dark: the game's smoke, and the picture's: the muzzle's jet is told to the picture's solver alone, so the game's own smoke says nothing of it. `first`: the row says so, whatever was told; else only what has been told since the last clear
pub(super) fn gas_out(g: &mut Game, st: &mut KitStage, first: bool) {
    if crate::sense::smoky(g) { crate::smoke_types::smoke_system_like_clear_all(g); }
    if first || st.a.smoke_told { crate::out_events::push_smoke_system_like_clear_all(g); st.a.smoke_told = false; }
}

// ------------------------------------------------------------ the pools
fn count(c: &MemberKit, k: &str) -> f64 {
    let v = match k { "bangs" => c.bangs, "charges" => c.charges, "smoke" => c.smoke, "frags" => c.frags, "walls" => c.walls, "shells" => c.shells, "cams" => c.cams, "rings" => c.rings, _ => 0.0 };
    m::max(0.0, m::floor(if v.is_nan() { 0.0 } else { v }))
}
/// what the row spends, from the cards: the squad's one bag and Sam's own belt where his card has the tile. `f`: the thing about to play has one at least (a tile's thing plays at a count of 0 too)
pub(super) fn pools(g: &mut Game, st: &KitStage, f: Option<&str>) {
    let n = st.order.len();
    let cards = &st.a.cards;
    let sum = |k: &str, from: usize| -> f64 { let mut s = 0.0; for i in from..=n { if let Some(c) = cards.get(i) { s += count(c, k); } } m::max(s, if Some(k) == f { 1.0 } else { 0.0 }) };
    crate::doortools::door_tools_sync(g);
    g.sets[g.seat].door_tools.bangs = sum("bangs", 1);
    g.sets[g.seat].door_tools.charges = sum("charges", 1);
    g.sets[g.seat].door_tools.frags = sum("frags", 1);
    let smoke = sum("smoke", 1);
    crate::squadsmoke::squad_smoke_stage(g, smoke);
    let walls = sum("walls", 0);
    crate::wallcharge::wall_charge_set_carried(g, Some(walls));
    let mine: Vec<&'static str> = crate::kit::kit_rows(g, 0.0).iter().map(|r| r.field.as_str()).collect();
    let empty = MemberKit::default();
    let c = cards.first().unwrap_or(&empty);
    let one = |k: &str| m::max(count(c, k), if Some(k) == f { 1.0 } else { 0.0 });
    let has = |k: &str| mine.iter().any(|x| *x == k) || Some(k) == f;
    let pl = g.player;
    let rifle = g.pl[pl].rifle;
    if has("smoke") { g.pl[pl].canisters = one("smoke"); }
    if has("bangs") { g.pl[pl].flashbangs = one("bangs"); }
    if has("frags") { g.pl[pl].frags = one("frags"); }
    if has("shells") { g.fa[rifle].shells = one("shells"); }
    let ring = g.fa[rifle].attachments.contains(&RifleAttachment::Ring);
    if has("rings") && ring { g.fa[rifle].rings = one("rings"); }
    if has("cams") && one("cams") > 0.0 && ring {
        g.fa[rifle].cams = one("cams");
        if !g.fa[rifle].attachments.contains(&RifleAttachment::Cam) { g.fa[rifle].attachments.push(RifleAttachment::Cam); }
    }
}

// ------------------------------------------------------------ the gun fires
pub(super) fn fire_at(g: &mut Game, man: Option<GuardId>, to: V3) {
    let c = match man { Some(x) => g.gd[x].ch, None => g.pl[g.player].ch };
    let w = crate::combat::firearm_of(g, c);
    if !crate::weapons::firearm_fire(g, w) { crate::weapons::firearm_rearm(g, w); crate::weapons::firearm_fire(g, w); }
    let o = crate::combat::FireOpts { steady: Some(true), ..crate::combat::FireOpts::default() };
    crate::combat::fire_weapon(g, c, to, Some(&o));
}
/// WHAT A BURST LEAVES, all of it put back plain: the marks, the glass's books and holes, the panes, the destruction, the shards, every prop, every shot-out fixture; never the doors, the switches or the men. A wall left open keeps its marks, its cells and its props: they go with the restage that stands it.
pub(super) fn mend(g: &mut Game, st: &mut KitStage) {
    if !st.a.shot { return; }
    st.a.shot = false;
    if let Some(d) = st.door { if !g.dr[d].leaf.off { g.dr[d].leaf.shot = 0.0; } }
    if st.hole {
        st.a.hole_shots += 1;
        crate::glass::reset_glass(g); crate::glasspen::reset_glass_pen(g);
        crate::glassbreak::glass_break_reset(g);
        crate::destruction::destruction_clear_moving(g);
        crate::shards::glass_shards_reset(g);
        crate::lighting::repair_lights(g);
        super::stage::pool_sync(g, st);
        return;
    }
    crate::render_decals::decals_clear(g);
    crate::glass::reset_glass(g); crate::glasspen::reset_glass_pen(g);
    crate::glassbreak::glass_break_reset(g);
    crate::destruction::destruction_reset(g);
    crate::shards::glass_shards_reset(g);
    crate::props::prop_system_reset(g);
    if let Some(p) = st.plan.as_ref() { let hidden = p.hidden_props.clone(); super::stage::drop_props(g, &hidden); }
    crate::lighting::repair_lights(g);
    if let Some(f) = st.fixture { if !g.engine.world.statics.contains(&f) { crate::scene_boxes::box_world_add_static(g, f); } }
    super::stage::pool_sync(g, st);
}
fn burst_of(g: &Game, seat: &crate::seats::SeatRef, b: &Beat) -> (u32, f64) {
    let pl = seat.player;
    let slot_rifle = g.pl[pl].slot == SlotKind::Rifle;
    let fa = match b.man { Some(x) => g.gd[x].weapon, None => if slot_rifle { g.pl[pl].rifle } else { g.pl[pl].pistol } };
    let def = g.m.weapons.weapons.get(g.fa[fa].def).unwrap();
    let k = if def.pellets.is_some() { 2 } else if (b.man.is_none() && !slot_rifle) || def.cls == crate::weapons::WeaponClass::Pistol { 1 } else { 0 };
    (K.fire_n[k], K.fire_gap[k])
}
/// the opening as a man at P has it: the yaws of the lines through its far edge and its near one, a hand in from the jamb, never within `wall_min` of the wall's line. None: no line into the room from there
fn slice(st: &KitStage, p: V2) -> Option<(f64, f64)> {
    let hw = st.width / 2.0;
    let (al, out) = (along(st, p), out_of(st, p));
    if out < 0.2 { return None; }
    let hinge_low = st.latch > 0.0;
    let leaf_in = if st.door.is_some() { K.leaf_in } else { 0.0 };
    let u0 = -hw + K.jamb_in + if hinge_low { leaf_in } else { 0.0 };
    let u1 = hw - K.jamb_in - if hinge_low { 0.0 } else { leaf_in };
    if u1 <= u0 { return None; }
    let u_far = if m::abs(u0 - al) > m::abs(u1 - al) { u0 } else { u1 };
    let u_near = if u_far == u0 { u1 } else { u0 };
    let sg = if u_far - al != 0.0 { m::sign(u_far - al) } else { 1.0 };
    let a_far = m::atan2(out, m::abs(u_far - al));
    let a_near = if al > u0 && al < u1 { m::PI / 2.0 } else { m::atan2(out, m::abs(u_near - al)) };
    let lo = m::max(a_far, K.wall_min * R + 1.0 * R);
    let hi = m::min(a_near, K.slice_max * R);
    if a_near < lo { return None; }
    let yaw = |a: f64| m::atan2(st.s[0] * sg * m::cos(a) - st.n[0] * m::sin(a), st.s[1] * sg * m::cos(a) - st.n[1] * m::sin(a));
    Some((yaw(lo), yaw(m::max(lo, hi))))
}
fn wall_angle(st: &KitStage, y: f64) -> f64 { m::acos(m::min(1.0, m::abs(m::sin(y) * st.s[0] + m::cos(y) * st.s[1]))) }
fn fire_line(g: &mut Game, st: &KitStage, b: &Beat) -> Option<Fire> {
    let mut men: Vec<V2> = st.order.iter().filter(|&&x| Some(x) != b.man).map(|&x| home_of(g, st, x).0).collect();
    if b.man.is_some() { men.push(st.sam); }
    let eye = 1.35;
    let (ah, oh) = (along(st, b.home), out_of(st, b.home));
    let near = |p: V2, d: f64| men.iter().any(|q| m::hyp2(q[0] - p[0], q[1] - p[1]) < d);
    let walk_ok = |g: &Game, a: V2, c: V2| { let n = m::ceil(m::hyp2(c[0] - a[0], c[1] - a[1]) / 0.25); let mut i = 1.0; while i <= n { let p: V2 = [a[0] + (c[0] - a[0]) * i / n, a[1] + (c[1] - a[1]) * i / n]; if !free(g, p) || near(p, 0.6) { return false; } i += 1.0; } true };
    let line = |g: &mut Game, p: V2, t: V3, nrm: V2| -> bool {
        let (dx, dz) = (t[0] - p[0], t[2] - p[1]);
        let l = m::hyp2(dx, dz);
        let l3 = m::hyp2(l, t[1] - eye);
        if l < 0.8 { return false; }
        if m::abs(dx * nrm[0] + dz * nrm[1]) / l < K.fire_slant { return false; }
        for q in &men {
            let (qx, qz) = (q[0] - p[0], q[1] - p[1]);
            let ql = m::hyp2(qx, qz);
            if (qx * dx + qz * dz) / ((if ql != 0.0 { ql } else { 1.0 }) * l) > m::cos(K.fire_clear * R) { return false; }
        }
        match crate::scene_collision::raycast(g, [p[0], eye, p[1]], [dx / l3, (t[1] - eye) / l3, dz / l3], l3 + 0.3) { None => true, Some(h) => h.t >= l3 - 0.35 }
    };
    let plan = st.plan.as_ref()?;
    let pp = plan.pane.clone();
    let asked = plan.src.iter().any(|(k, s)| *k == "pane" && *s == Src::Authored);
    let only: Option<BoxId> = if asked { pp.as_ref().and_then(|x| g.col.boxes.get(x.index).copied()) } else { None };
    let list: Vec<BoxId> = if asked && pp.is_none() { Vec::new() } else { crate::glass::glass_panes(g).iter().filter_map(|&i| g.col.boxes.get(i as usize).copied()).collect() };
    let mut panes: Vec<(BoxId, f64)> = Vec::new();
    for b0 in list {
        if only.map_or(false, |o| o != b0) { continue; }
        let r = &g.tab.boxes[b0];
        let (c, h) = (r.c, r.h);
        let al = along(st, [c[0], c[2]]);
        if m::abs(out_of(st, [c[0], c[2]])) <= KIT_PLAN.pane_wall && m::abs(al) <= K.pane_reach && c[1] - h[1] <= KIT_PLAN.pane_y[0] && c[1] + h[1] >= KIT_PLAN.pane_y[1] { panes.push((b0, al)); }
    }
    panes.sort_by(|a, c| m::abs(a.1).partial_cmp(&m::abs(c.1)).unwrap_or(core::cmp::Ordering::Equal));
    for (b0, al) in panes {
        let r = &g.tab.boxes[b0];
        let (c, h) = (r.c, r.h);
        let t: V3 = [c[0], m::max(c[1] - h[1] + 0.15, m::min(c[1] + h[1] - 0.15, eye)), c[2]];
        let nrm: V2 = if h[0] <= h[2] { [1.0, 0.0] } else { [0.0, 1.0] };
        if line(g, b.home, t, nrm) { return Some(Fire { post: b.home, via: None, at: t, door: false }); }
        if b.man.is_none() { continue; }
        let mut posts: Vec<V2> = Vec::new();
        if only.is_some() { if let Some(x) = pp.as_ref() { posts.push([x.stand[0], x.stand[1]]); } }
        for o in K.pane_off { let q = at(st, [al + o, K.pane_out, 0.0]); posts.push([q[0], q[2]]); }
        for p in posts {
            if !free(g, p) || near(p, K.fire_off) || !line(g, p, t, nrm) { continue; }
            let v = at(st, [ah, K.pane_out, 0.0]);
            let via: Option<V2> = if m::abs(oh - K.pane_out) > 0.3 { Some([v[0], v[2]]) } else { None };
            let ok = match via { Some(q) => walk_ok(g, b.home, q) && walk_ok(g, q, p), None => walk_ok(g, b.home, p) };
            if !ok { continue; }
            return Some(Fire { post: p, via, at: t, door: false });
        }
    }
    if !walled(st) && st.reach_in >= 1.0 {
        let p = step_out(st, b, K.light_step);
        if let Some((far, nr)) = slice(st, p) {
            if free(g, p) {
                let y = far + wrap(nr - far) * 0.5;
                return Some(Fire { post: p, via: None, at: [p[0] + m::sin(y) * 6.0, 1.2, p[1] + m::cos(y) * 6.0], door: true });
            }
        }
    }
    None
}
fn gun_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if let Some(f) = fire_line(g, st, b) {
        let (n, gap) = burst_of(g, &crate::seats::here(g), b);
        let p = v3(f.post);
        let here = f.post == b.home && flat(pos(g, b), f.post) <= K.way_m;
        b.fire = Some(f); b.post = Some(f.post); b.via = f.via; b.h_to = f.at[1]; b.quiet = here;
        if f.door { b.row = Act::GunDoor; unlock(g, st); }
        b.legs = vec![Leg { yaw: crate::v3::yaw_to(p, f.at), cap: K.gun_rate * R, min_t: -1.0, hold: 0.3 + (n as f64 - 1.0) * gap + K.fire_hold }];
        if here { if b.man.is_none() { sam_stand(g, st); } }
        else {
            to(g, b, "go");
            match b.man {
                Some(x) => { let pts = match f.via { Some(q) => vec![q, f.post], None => vec![f.post] }; go(g, st, x, pts, K.pace, 0.0, false); }
                None => sam_goes(st, f.post),
            }
        }
        return Ok(());
    }
    let rest = rest_yaw(g, st, b);
    let mid = lim(rest, crate::v3::yaw_to(pos(g, b), at(st, [0.0, 0.0, 1.1])));
    let (dy, slow) = (K.gun_scan * R, K.sweep_rate * R);
    b.quiet = true; b.h_to = 1.1;
    b.legs = vec![
        Leg { yaw: mid, cap: K.gun_rate * R, min_t: -1.0, hold: 0.0 },
        Leg { yaw: lim(rest, mid + dy), cap: slow, min_t: 0.0, hold: 0.0 },
        Leg { yaw: lim(rest, mid - dy), cap: slow, min_t: 0.0, hold: 0.0 },
        Leg { yaw: mid, cap: slow, min_t: 0.0, hold: K.gun_hold },
    ];
    if b.man.is_none() { sam_stand(g, st); }
    Ok(())
}
fn gun_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let Some(f) = b.fire else { return quiet_tick(g, st, b) };
    let now = g.time;
    if b.phase == "home" { return if flat(pos(g, b), b.home) <= K.on_slot || now - b.pt >= 4.0 { Step::Done } else { Step::On }; }
    if f.door && b.phase != "down" { crack(g, st, b, K.wide_light); }
    if b.phase == "go" {
        if flat(pos(g, b), f.post) > K.way_m && now - b.pt < 6.0 { return Step::On; }
        to(g, b, "act");
        b.quiet = true;
    }
    if b.phase == "act" && b.leg == 0 && arrived(now, ch_of(st, b)) && off(g.ch[b.ch_id].aim_yaw, b.legs[0].yaw) <= 4.0 * R && (!f.door || wide_open(g, st, K.wide_light)) {
        let (n, gap) = burst_of(g, &crate::seats::here(g), b);
        if b.shots < n && (b.shot_t.is_nan() || now - b.shot_t >= gap) {
            fire_at(g, b.man, f.at);
            st.a.shots += 1;
            b.shots += 1;
            b.shot_t = now; st.a.shot_at = now; st.a.shot = true;
            if b.shots == 1 { b.marks.push("shot"); }
            if b.shots == n { b.marks.push("burst"); }
        }
    }
    let r = quiet_tick(g, st, b);
    if r != Step::Done || flat(pos(g, b), b.home) <= K.on_slot { return r; }
    to(g, b, "home");
    match b.man {
        Some(x) => { let pts = match b.via { Some(q) => vec![q, b.home], None => vec![b.home] }; go(g, st, x, pts, K.pace, 0.0, false); }
        None => sam_goes(st, b.home),
    }
    Step::On
}
// ------------------------------------------------------------ the weapon light
fn light_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if walled(st) { return Err(String::from("a blank wall: no opening to light")); }
    let p = step_out(st, b, K.light_step);
    let Some((far, nr)) = slice(st, p).filter(|_| free(g, p)) else { return Err(String::from("no line through the door")) };
    b.quiet = true; b.lit = true; b.h_to = 1.1; b.post = Some(p);
    unlock(g, st);
    b.pitch = Some(chan(-K.feet_deg * R));
    b.rose = false;
    b.legs = vec![Leg { yaw: far, cap: K.gun_rate * R, min_t: -1.0, hold: 0.0 }, Leg { yaw: nr, cap: K.sweep_rate * R, min_t: 0.0, hold: K.light_hold }];
    match b.man { Some(x) => go(g, st, x, vec![p], K.pace, 0.0, false), None => sam_goes(st, p) }
    Ok(())
}
fn light_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    if b.phase != "down" { crack(g, st, b, K.wide_light); }
    let now = g.time;
    let ch_v = ch_of(st, b).v;
    if !b.rose && wall_angle(st, g.ch[b.ch_id].aim_yaw) >= K.wall_min * R + 0.5 * R && wall_angle(st, ch_v) >= K.wall_min * R + 0.5 * R {
        b.rose = true;
        if let Some(p) = b.pitch.as_mut() { own(now, p, 0.0, K.gun_rate * R, K.rise_secs); }
    }
    if let Some(p) = b.pitch.as_mut() { chan_tick(now, p, K.gun_rate * R, st.a.step_dt); }
    quiet_tick(g, st, b)
}
// ------------------------------------------------------------ a teammate's door order with a can in it, cut before anybody goes in
fn stat(seat: &crate::seats::SeatRef, a: Act) -> f64 { if a == Act::BangOrder { seat.door_tools.stats.bangs } else { seat.door_tools.stats.frags } }
fn order_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    b.base = stat(&crate::seats::here(g), b.row);
    unlock(g, st);
    let x = b.man.unwrap();
    let d = the_door(st);
    if b.row == Act::BangOrder {
        match crate::doortools::door_tools_order(g, crate::doortools::DoorToolKind::BangClear, &[x], d, st.side, None, None) {
            crate::doortools::DoorToolsOrder::Text(t) => return Err(t),
            _ => {}
        }
    } else {
        let t = on_door(st);
        let who = roster(g, x);
        if let Some(why) = crate::orders::squad_orders_give(g, "fragClear", Some(&t), Some(who), None) { return Err(why); }
    }
    Ok(())
}
fn order_tick(g: &Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let d = the_door(st);
    let gone = door_op(g, &crate::seats::here(g), d).map_or(true, |o| g.tab.doortools_door_op[o].phase == crate::doortools::Phase::Enter);
    let (word, after) = if b.row == Act::BangOrder { ("bang", K.after_stun) } else { ("frag", K.aftermath) };
    if b.mark_t.is_nan() && stat(&crate::seats::here(g), b.row) > b.base { mark(g, b, word); }
    if b.mark_t.is_nan() { if gone { b.marks.push("lost"); return Step::Done; } return Step::On; }
    if gone || g.time - b.mark_t >= after { Step::Done } else { Step::On }
}
// ------------------------------------------------------------ a can through the door, by hand
/// a burst at `p` has a line to Sam as he is (as the blast judges him: from `lift` over the grenade, glass that is going counted open)
fn reaches_sam(g: &mut Game, p: V3) -> bool {
    let c = g.pl[g.player].ch;
    let feet = g.ch[c].pos;
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or([feet[0], feet[1] + 1.6, feet[2]]);
    let lift = g.k.frag.lift;
    crate::frag::exposed_to(g, [p[0], p[1] + lift, p[2]], feet, head, Some(crate::frag::GlassView::Blown)) > 0.0
}
fn takes_leaf(g: &Game, st: &KitStage, p: V3) -> bool {
    let Some(d) = st.door else { return false };
    let radius = g.k.frag.destr_m;
    crate::destruction::destruction_doors_for(g, p, Some(crate::destruction::DestructionDoorsForOpts { radius: Some(radius), power: Some(1.0), ..Default::default() })).contains(&d)
}
/// SAM'S FRAG, AIMED THROUGH THE GATE: the leaf held open stands in the room on the hinge's side, so the way in is between the latch jamb and the leaf's free edge; the grenade is aimed from his hand through the middle of that gap. True once the arc the game has solved first lands `frag_land` inside
fn frag_aim(g: &Game, st: &KitStage, b: &mut Beat) -> bool {
    let Some(pv) = g.sets[g.seat].hand.throw_preview.as_ref() else { return false };
    let Some(&o) = pv.points.first() else { return false };
    let land = pv.land;
    if st.a.led.is_none() { return false; }
    let gate: Option<V2> = st.door.and_then(|d| {
        if g.dr[d].leaf.off { return None; }
        let def = &g.dr[d].def;
        let (w, h) = (def.width, def.hinge);
        let a = def.closed_dir + g.dr[d].leaf.angle;
        let j: V2 = [h[0] + m::cos(def.closed_dir) * w, h[1] + m::sin(def.closed_dir) * w];
        let t: V2 = [h[0] + m::cos(a) * w, h[1] + m::sin(a) * w];
        if out_of(st, t) < 0.0 { Some([(j[0] + t[0]) / 2.0, (j[1] + t[1]) / 2.0]) } else { None }
    });
    let mm: V2 = gate.unwrap_or([st.foot[0], st.foot[2]]);
    let (oo, mo) = (out_of(st, [o[0], o[2]]), out_of(st, mm));
    if oo - mo < 0.2 { return false; }
    let k = (oo + st.frag_m) / (oo - mo);
    let spot: V3 = [o[0] + (mm[0] - o[0]) * k, 0.0, o[2] + (mm[1] - o[2]) * k];
    b.spot = Some(spot);
    out_of(st, [land[0], land[2]]) <= -st.frag_land
}
fn throw_kind(a: Act) -> (th::ItemKind, &'static str) { match a { Act::ThrowSmoke => (th::ItemKind::Smoke, "Digit3"), Act::ThrowFlash => (th::ItemKind::Flash, "Digit4"), _ => (th::ItemKind::Frag, "Digit5") } }
fn thrown_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if walled(st) { return Err(String::from("a blank wall: no opening to throw through")); }
    let frag = b.row == Act::ThrowFrag;
    let p = step_out(st, b, if st.quiet && b.man.is_none() { QK.throw_step } else if frag { K.frag_half } else { K.half });
    let spot = match b.man {
        Some(_) => st.spots_smoke.first().or(st.spots_in.first()).copied(),
        None => Some(match b.row { Act::ThrowSmoke => st.sam_smoke, Act::ThrowFlash => st.sam_stun, _ => st.sam_frag }),
    };
    let Some(spot) = spot.filter(|_| free(g, p)) else { return Err(String::from("no room at the door")) };
    b.post = Some(p); b.spot = Some(spot);
    b.base = if frag { g.sets[g.seat].door_tools.stats.frags } else { cans(g) };
    to(g, b, "open");
    unlock(g, st);
    match b.man {
        Some(x) => { go(g, st, x, vec![p], K.pace, 0.0, false); walk_of(st, x).unwrap().face = Some(crate::v3::yaw_to(v3(p), spot)); }
        None => st.a.led = Some(Led { to: Some(p), at: Some(spot), low: false }),
    }
    if frag { stay(g, st, true); st.a.pin = Some(b.n); }
    Ok(())
}
fn thrown_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let frag = b.row == Act::ThrowFrag;
    let smoke = b.row == Act::ThrowSmoke;
    let now = g.time;
    let el = now - b.pt;
    let wide = if b.man.is_some() { K.wide } else if frag { K.frag_wide } else { K.wide_sam };
    if let Some(d) = st.door {
        let (p, id) = (pos(g, b), ch_id(g, b));
        if !frag || b.phase == "open" || b.phase == "throw" || (b.phase == "out" && el < K.ajar_at) { doors_crack(g, d, p, id, st.a.step_dt, None, Some(K.swing), Some(wide)); }
        else if m::abs(g.dr[d].leaf.angle - st.leaf_shut) > K.ajar + 0.04 { doors_pull_to(g, d, id); }
        else { doors_crack(g, d, p, id, st.a.step_dt, None, Some(K.swing), Some(K.ajar)); }
    }
    if b.phase == "open" {
        let post = b.post.unwrap();
        if el < K.open_secs[0] || ((flat(pos(g, b), post) > K.way_m || !wide_open(g, st, wide)) && el < K.open_secs[1]) { return Step::On; }
        if st.door.map_or(false, |d| door_is_closed(g, d)) { b.marks.push("shut"); return Step::Done; }
        match b.man {
            Some(x) => {
                let def = order_def(g, &crate::seats::here(g), "smokeThere");
                let spots: Vec<V3> = st.spots_smoke.iter().chain(st.spots_in.iter()).copied().collect();
                let mut chose = None;
                if let Some(d) = def { for q in spots { if crate::orders::squad_orders_why_not(g, d, &floor(q), Some(&[x])).is_none() { chose = Some(q); break; } } }
                let why = match chose {
                    Some(q) => { let who = roster(g, x); crate::orders::squad_orders_give(g, "smokeThere", Some(&floor(q)), Some(who), None) }
                    None => Some(String::from("no throw from here")),
                };
                if let Some(why) = why { st.a.refused = Some(format!("smoke: {}", why)); b.marks.push("kept"); return Step::Done; }
                b.spot = chose; b.thrown = true; b.given = true;
            }
            None => { b.hand = Some(g.pl[pl].slot); press(st, throw_kind(b.row).1); }
        }
        to(g, b, "throw");
        return Step::On;
    }
    if b.phase == "throw" {
        if b.man.is_none() {
            if g.pl[pl].pending_throw.is_some() { b.thrown = true; return Step::On; }
            if !b.thrown {
                let clear = !frag || frag_aim(g, st, b);
                if el >= K.fist_secs && clear { st.a.clicked = (crate::js::to_i32(st.a.clicked) | 1) as f64; }
                if el > 3.0 {
                    b.marks.push("kept");
                    if !clear { st.a.refused = Some(String::from("frags: no clear throw through the door")); }
                    return Step::Done;
                }
                return Step::On;
            }
        }
        to(g, b, "out");
        return Step::On;
    }
    if frag {
        if b.mark_t.is_nan() && item_went(g, b) { mark(g, b, "frag"); }
        if !b.mark_t.is_nan() { return if now - b.mark_t >= K.aftermath { Step::Done } else { Step::On }; }
        if let Some(it) = b.item {
            if let Some(i) = g.items.items.iter().position(|&x| x == it) {
                if !g.tab.item[it].fired && g.tab.item[it].bounces > 0.0 {
                    let p = g.tab.item[it].pos;
                    let (settled, fuse) = (g.tab.item[it].settled, g.tab.item[it].fuse);
                    if out_of(st, [p[0], p[2]]) > -K.frag_min || ((settled || fuse < 0.3) && takes_leaf(g, st, p)) || (fuse < 0.3 && reaches_sam(g, p)) {
                        th::throwables_remove_at(g, i as f64);
                        b.marks.push("kept");
                        st.a.refused = Some(String::from("frags: it did not go in, or its burst had a line to the player"));
                        return Step::Done;
                    }
                }
            }
        }
        if b.phase == "out" {
            if b.spot.is_some() { b.spot = None; sam_goes_low(st, b.home, true); }
            if flat(g.seat_pos(&crate::seats::here(g)), b.home) > K.on_slot && el < 1.5 { return Step::On; }
            st.a.led = None;
            st.a.kneel = Some(b.n);
            st.a.kneel_t = now;
            for k in 0..st.order.len() { let x = st.order[k]; low(g, st, x, true); }
            to(g, b, "fuse");
            return Step::On;
        }
        if el > 6.0 { b.marks.push("lost"); return Step::Done; }
        return Step::On;
    }
    if b.mark_t.is_nan() && (item_went(g, b) || (smoke && cans(g) > b.base)) { mark(g, b, if smoke { "smoke" } else { "bang" }); }
    if b.mark_t.is_nan() { return if el > 6.0 { Step::Done } else { Step::On }; }
    if now - b.mark_t >= (if smoke { K.after_smoke } else { K.after_stun }) { Step::Done } else { Step::On }
}
// ------------------------------------------------------------ door charges
/// the stack a slot back from the door, rearmost first, facing it. All of them or none
fn stand_off(g: &Game, st: &mut KitStage, b: &mut Beat) {
    let ns = m::min(3.0, st.order.len() as f64) as usize;
    if ns == 0 { return; }
    let back = |st: &KitStage, k: usize| -> V2 { [st.spots[k][0] + st.s[0] * ST.gap, st.spots[k][1] + st.s[1] * ST.gap] };
    if !free(g, back(st, ns - 1)) { return; }
    for k in (0..ns).rev() {
        let x = st.order[k];
        if low_at(g, st, x) { continue; }   // (the quiet picture's partner rests behind Sam, clear of the door: the stack's place would walk him across its front)
        let q = back(st, k);
        go(g, st, x, vec![q], K.pace, K.shuffle[1] * b.moved.len() as f64, false);
        b.moved.push(x);
    }
}
fn charge_mate_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let d = the_door(st);
    if b.mark_t.is_nan() && g.sets[g.seat].door_tools.stats.blown > b.base { mark(g, b, "blow"); }
    if !b.mark_t.is_nan() { return if g.time - b.mark_t >= K.aftermath { Step::Done } else { Step::On }; }
    if let Some(c) = crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), d) {
        if b.phase != "set" { to(g, b, "set"); b.marks.push("set"); }
        let armed = g.sets[g.seat].door_tools.ops.iter().any(|&o| { let r = &g.tab.doortools_door_op[o]; r.kind == crate::doortools::DoorToolKind::Charge && r.door == d && r.phase == crate::doortools::Phase::Armed });
        if g.time - b.pt >= K.clear_secs || armed { crate::doortools::door_tools_trigger(g, Some(c)); }
    }
    Step::On
}
fn charge_sam_start(g: &Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let q = at(st, [K.own_post[0] * st.latch, K.own_post[1], 0.0]);
    if !free(g, v2(q)) { return Err(String::from("no room at the door")); }
    let d = the_door(st);
    if door_steel(g, d) { return Err(String::from("steel: a charge will not cut it")); }
    if g.dr[d].leaf.off { return Err(String::from("no leaf to set it on")); }
    b.base = g.sets[g.seat].door_tools.stats.blown;
    b.post = Some(v2(q));
    to(g, b, "go");
    sam_goes(st, v2(q));
    Ok(())
}
fn charge_sam_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let now = g.time;
    let el = now - b.pt;
    let d = the_door(st);
    let c = crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), d);
    if b.mark_t.is_nan() && g.sets[g.seat].door_tools.stats.blown > b.base { mark(g, b, "blow"); }
    if !b.mark_t.is_nan() { return if now - b.mark_t >= K.aftermath { Step::Done } else { Step::On }; }
    let ajar = !g.dr[d].leaf.off && m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= 0.03;
    let id = ch_id(g, b);
    if ajar && (b.phase == "go" || b.phase == "ask") { doors_pull_to(g, d, id); }
    if b.phase == "go" {
        if (flat(g.seat_pos(&crate::seats::here(g)), b.post.unwrap()) > 0.15 || ajar) && el < 2.0 { return Step::On; }
        st.a.led = None;
        stand_off(g, st, b);
        to(g, b, "ask");
        return Step::On;
    }
    if b.phase == "ask" {
        if g.pl[pl].charging().is_some() { b.given = true; to(g, b, "hands"); return Step::On; }
        if el >= K.own_ask { b.marks.push("kept"); st.a.refused = Some(String::from("charges: the door never offered it")); return Step::Done; }
        if (b.leg_t.is_nan() || now - b.leg_t > 0.5) && crate::game::game_select_verb(g, InteractableKind::Door, Some(Verb::Charge)) { press(st, "Space"); b.leg_t = now; }
        return Step::On;
    }
    if b.phase == "hands" {
        if c.is_none() { if g.pl[pl].charging().is_none() { b.marks.push("kept"); return Step::Done; } return Step::On; }
        b.marks.push("set");
        to(g, b, "off");
        sam_goes(st, b.home);
        return Step::On;
    }
    let Some(c) = c else { b.marks.push("lost"); return Step::Done };
    if flat(g.seat_pos(&crate::seats::here(g)), b.home) <= K.on_slot || el >= K.clear_secs { crate::doortools::door_tools_trigger(g, Some(c)); }
    Step::On
}
// ------------------------------------------------------------ the wand
fn tiles(g: &Game, i: u32) -> Vec<&'static str> { crate::kit::kit_rows(g, i as f64).iter().map(|r| r.field.as_str()).collect() }
fn fog_on(g: &mut Game, st: &mut KitStage) { st.a.fog_up = true; g.fog_picture.viewer = true; g.fog_of_war.on = true; }
fn wand_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let q = at(st, [K.wand_post[0], K.wand_post[1], 0.0]);
    if !tiles(g, 0).contains(&"wand") { return Err(String::from("no wand on this card")); }
    let d = the_door(st);
    if door_steel(g, d) { return Err(String::from("steel: no gap under it")); }
    if g.dr[d].leaf.off { return Err(String::from("no leaf to look under")); }
    if !free(g, v2(q)) { return Err(String::from("no room at the door")); }
    let pl = g.player;
    st.a.wand_was = g.pl[pl].wand;
    g.pl[pl].wand = true;
    fog_on(g, st);
    b.post = Some(v2(q));
    // (a leaf left wide by the row before is pulled home first, with him out of its sweep; only then is he walked to its foot)
    if m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= 0.03 {
        let def = &g.dr[d].def;
        let p = g.seat_pos(&crate::seats::here(g));
        let clear = m::hyp2(p[0] - def.hinge[0], p[2] - def.hinge[1]) > def.width + K.wand_clear;
        let w = if clear { v2(p) } else { st.sam };
        to(g, b, "shut");
        sam_goes(st, w);
        return Ok(());
    }
    to(g, b, "go");
    sam_goes(st, v2(q));
    Ok(())
}
fn wand_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let now = g.time;
    let el = now - b.pt;
    let d = the_door(st);
    let ajar = m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= 0.03;
    let id = ch_id(g, b);
    if ajar && (b.phase == "shut" || b.phase == "go" || b.phase == "ask") { doors_pull_to(g, d, id); }
    if b.phase == "shut" {
        if ajar && el < K.wand_shut { return Step::On; }
        to(g, b, "go");
        sam_goes(st, b.post.unwrap());
        return Step::On;
    }
    if b.phase == "go" {
        if (flat(g.seat_pos(&crate::seats::here(g)), b.post.unwrap()) > 0.15 || ajar) && el < 2.0 { return Step::On; }
        st.a.led = None;
        to(g, b, "ask");
        return Step::On;
    }
    let wanding = g.pl[pl].wanding().map(|w| w.live);
    if b.phase == "ask" {
        if wanding.is_some() { b.marks.push("knee"); to(g, b, "feed"); return Step::On; }
        if el >= K.own_ask { b.marks.push("kept"); st.a.refused = Some(String::from("wand: the door never offered it")); return Step::Done; }
        if (b.leg_t.is_nan() || now - b.leg_t > 0.5) && crate::game::game_select_verb(g, InteractableKind::Door, Some(Verb::Wand)) { press(st, "Space"); b.leg_t = now; }
        return Step::On;
    }
    if b.phase == "off" { return if flat(g.seat_pos(&crate::seats::here(g)), b.home) <= K.on_slot || el >= 1.5 { Step::Done } else { Step::On }; }
    let Some(live) = wanding else { b.marks.push("lost"); return Step::Done };
    if b.phase == "feed" { if live { mark(g, b, "through"); to(g, b, "pan"); } return Step::On; }
    if el < 2.0 * K.wand_pan { st.a.cursor = Some(at(st, [if el < K.wand_pan { 6.0 } else { -6.0 }, 1.5, 0.0])); return Step::On; }
    b.marks.push("out");
    to(g, b, "off");
    sam_goes_low(st, b.home, st.quiet);
    Step::On
}
fn wand_mate_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let x = b.man.unwrap();
    if !tiles(g, b.i).contains(&"wand") { return Err(String::from("no wand on this card")); }
    let Some(card) = g.m.kit.man_of.get(x).map(|e| e.kit.wand) else { return Err(String::from("no card on him")) };
    let d = the_door(st);
    if door_steel(g, d) { return Err(String::from("steel: no gap under it")); }
    if g.dr[d].leaf.off { return Err(String::from("no leaf to look under")); }
    st.a.wand_kit = Some((x, card));
    g.m.kit.man_of.get_mut(x).unwrap().kit.wand = true;
    st.a.wand_was = g.pl[g.player].wand;
    fog_on(g, st);
    b.base = g.stats.wandorder.live as f64;
    to(g, b, "ask");
    Ok(())
}
fn wand_mate_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let now = g.time;
    let el = now - b.pt;
    let x = b.man.unwrap();
    let d = the_door(st);
    if b.phase == "ask" {
        let mut why = String::from("the squad never had the order");
        if is_ajar(g, st) { let id = ch_id(g, b); doors_pull_to(g, d, id); why = String::from("the leaf never shut"); }
        else {
            let t = on_door(st);
            let no = match order_def(g, &crate::seats::here(g), crate::wandorder::WAND_ORDER) {
                Some(def) => match crate::orders::squad_orders_why_not(g, def, &t, Some(&[x])) {
                    Some(w) => Some(w),
                    None => { let who = roster(g, x); crate::orders::squad_orders_give(g, crate::wandorder::WAND_ORDER, Some(&t), Some(who), None) }
                },
                None => Some(why.clone()),
            };
            match no { None => { b.given = true; to(g, b, "go"); return Step::On; } Some(w) => why = w }
        }
        if el >= K.wait { b.marks.push("kept"); st.a.refused = Some(format!("wand: {}", why)); return Step::Done; }
        return Step::On;
    }
    if b.mark_t.is_nan() {
        if g.stats.wandorder.live as f64 > b.base { b.marks.push("knee"); mark(g, b, "through"); return Step::On; }
        if assignment_order(g, x).as_deref() != Some(crate::wandorder::WAND_ORDER) { b.marks.push("lost"); return Step::Done; }
        return Step::On;
    }
    if now - b.mark_t < 2.0 * K.wand_pan { return Step::On; }
    b.marks.push("out");
    Step::Done
}
// ------------------------------------------------------------ the launcher
/// what Sam had in his hands as a launcher's row begins: the hand a row cut this very tick still owes him is not let land
fn hand_was(g: &Game, seat: &crate::seats::SeatRef, st: &mut KitStage) -> Option<SlotKind> {
    match st.a.hand_due.take() {
        Some(h) => { unpress(st, if h == SlotKind::Rifle { "Digit2" } else { "Digit1" }); Some(h) }
        None => Some(g.pl[seat.player].slot),
    }
}
/// the launcher's load stepped to `load`: true once that load is up
fn tube(g: &mut Game, st: &mut KitStage, load: RifleAttachment) -> bool {
    let pl = g.player;
    if !crate::game::player_rifle_up(g, pl) { press(st, "Digit2"); return false; }
    if g.pl[pl].equip_t > 0.0 { return false; }
    let rifle = g.pl[pl].rifle;
    if crate::weapons::rifle_attachment(g, rifle) == Some(load) { return true; }
    let Some(i) = g.fa[rifle].attachments.iter().position(|&a| a == load) else { return false };
    g.fa[rifle].att_idx = i as i32;
    true
}
/// the camera's wall, from where he stands: the nearest wall inside by a ray through the opening that clears both jambs and the leaf as it will stand
fn cam_spot(g: &Game, st: &KitStage, p: V2) -> Option<V3> {
    let d = st.door?;
    let out = out_of(st, p);
    let hw = st.width / 2.0 - K.cam_clear;
    if hw <= 0.0 || out < 0.3 { return None; }
    let (hinge, closed, width) = (g.dr[d].def.hinge, g.dr[d].def.closed_dir, g.dr[d].def.width);
    let tip = [1.0, -1.0].iter().map(|k| { let a = closed + k * K.cam_wide; [m::cos(a), m::sin(a)] }).find(|t: &[f64; 2]| t[0] * st.n[0] + t[1] * st.n[1] < 0.0)?;
    let (mut best, mut bt) = (None, f64::INFINITY);
    for i in 0..K.cam_rays {
        let q = at(st, [-hw + 2.0 * hw * i as f64 / (K.cam_rays as f64 - 1.0), 0.0, 0.0]);
        let l = m::hyp2(q[0] - p[0], q[2] - p[1]);
        let l = if l != 0.0 { l } else { 1.0 };
        let dir: V3 = [(q[0] - p[0]) / l, 0.0, (q[2] - p[1]) / l];
        let dn = -(dir[0] * st.n[0] + dir[2] * st.n[1]);
        if dn < 0.2 { continue; }
        let mut near = f64::INFINITY;
        for j in 0..=8 {
            let f = j as f64 / 8.0;
            let (lx, lz) = (hinge[0] + tip[0] * width * f - p[0], hinge[1] + tip[1] * width * f - p[1]);
            if lx * dir[0] + lz * dir[2] > 0.0 { near = m::min(near, m::abs(lx * dir[2] - lz * dir[0])); }
        }
        if near < K.cam_clear { continue; }
        let t0 = (out + K.cam_in) / dn;
        let o: V3 = [p[0] + dir[0] * t0, 1.2, p[1] + dir[2] * t0];
        let Some(h) = crate::scene_collision::raycast(g, o, dir, K.cam_reach) else { continue };
        if t0 + h.t >= bt { continue; }
        bt = t0 + h.t;
        best = Some([o[0] + dir[0] * h.t, 0.0, o[2] + dir[2] * h.t]);
    }
    best
}
fn cam_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let pl = g.player;
    let q = at(st, [(if along(st, b.home) < 0.0 { -1.0 } else { 1.0 }) * K.cam_post[0], K.cam_post[1], 0.0]);
    let p = v2(q);
    let rifle = g.pl[pl].rifle;
    if !g.pl[pl].has_rifle || !g.fa[rifle].attachments.contains(&RifleAttachment::Cam) { return Err(String::from("no launcher for it on this card")); }
    let d = the_door(st);
    if g.dr[d].def.width / 2.0 <= K.cam_clear || !free(g, p) { return Err(String::from("no line through the door")); }
    let Some(best) = cam_spot(g, st, p) else { return Err(String::from("no wall inside to stick it to")) };
    b.post = Some(p); b.spot = Some(best);
    b.hand = hand_was(g, &crate::seats::here(g), st);
    b.base = g.pl[pl].cam_seq;
    to(g, b, "open");
    g.dr[d].leaf.locked = false;
    st.a.tube_was = Some(g.fa[rifle].att_idx);
    sam_goes(st, p);
    Ok(())
}
fn cam_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let now = g.time;
    let el = now - b.pt;
    let d = the_door(st);
    let p = g.seat_pos(&crate::seats::here(g));
    let id = g.seat_id(&crate::seats::here(g));
    doors_crack(g, d, p, id, st.a.step_dt, None, Some(K.swing), Some(K.cam_wide));
    let post = b.post.unwrap();
    if b.phase == "open" {
        if flat(p, post) <= K.way_m { if let Some(q) = cam_spot(g, st, v2(p)) { b.spot = Some(q); } }
        let up = tube(g, st, RifleAttachment::Cam);
        let there = flat(p, post) <= K.way_m && m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= K.cam_wide - 0.1 && off(g.ch[b.ch_id].aim_yaw, crate::v3::yaw_to(p, b.spot.unwrap())) <= 3.0 * R;
        if !(up && there) && el < 4.0 { return Step::On; }
        if !up { b.marks.push("kept"); st.a.refused = Some(String::from("cams: the launcher never came up")); return Step::Done; }
        to(g, b, "fire");
        b.leg_t = f64::NAN;
        return Step::On;
    }
    if b.phase == "fire" {
        if g.pl[pl].cam_seq > b.base { b.thrown = true; mark(g, b, "cam"); to(g, b, "fly"); return Step::On; }
        if el >= K.cam_ask { b.marks.push("kept"); st.a.refused = Some(String::from("cams: the launcher never sent it")); return Step::Done; }
        if g.pl[pl].launch_held.is_none() && (b.leg_t.is_nan() || now - b.leg_t > 0.3) { st.a.clicked = (crate::js::to_i32(st.a.clicked) | 4) as f64; b.leg_t = now; }
        return Step::On;
    }
    if b.phase == "fly" {
        let cam1 = g.pl[pl].cams.last().copied();
        match cam1 {
            Some(c) if g.tab.stickycam_sticky_cam[c].vel.is_none() => { b.marks.push("stuck"); b.rest = Some(g.tab.stickycam_sticky_cam[c].pos); to(g, b, "stuck"); }
            _ if cam1.is_none() || el > 2.0 => { b.marks.push("lost"); return Step::Done; }
            _ => {}
        }
        return Step::On;
    }
    if el >= K.cam_hold { Step::Done } else { Step::On }
}
fn tube_load(a: Act) -> RifleAttachment { if a == Act::Ring { RifleAttachment::Ring } else { RifleAttachment::He40 } }
fn tube_start(g: &Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let pl = g.player;
    let rifle = g.pl[pl].rifle;
    let a = K.shell_yard * R;
    if !g.pl[pl].has_rifle || !g.fa[rifle].attachments.contains(&tube_load(b.row)) { return Err(String::from("no launcher on this card")); }
    b.quiet = true; b.h_to = 0.0;
    b.hand = hand_was(g, &crate::seats::here(g), st);
    let y = m::atan2(-st.s[0] * m::cos(a) + st.n[0] * m::sin(a), -st.s[1] * m::cos(a) + st.n[1] * m::sin(a));
    b.legs = vec![Leg { yaw: y, cap: K.gun_rate * R, min_t: -1.0, hold: K.shell_hold }];
    st.a.tube_was = Some(g.fa[rifle].att_idx);
    sam_stand(g, st);
    Ok(())
}
fn tube_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    if b.phase != "down" {
        let load = tube_load(b.row);
        if !b.rose && tube(g, st, load) { b.rose = true; b.marks.push("up"); }
        b.h_to = if b.rose && b.leg == 0 && arrived(g.time, ch_of(st, b)) { 1.3 } else { 0.0 };
    }
    quiet_tick(g, st, b)
}
// ------------------------------------------------------------ the wall charge
fn wall_kept(st: &mut KitStage, b: &Beat, no: &str) {
    if st.a.shut_own.is_none() { st.a.shut_own = Some(String::from(no)); }
    if (b.prelude || (st.walled && b.on_span)) && st.a.shut_wall.is_none() { st.a.shut_wall = Some(String::from(no)); }
}
fn spot_along(g: &Game, st: &KitStage, w: &crate::wallcharge::WallSpot, du: f64) -> f64 {
    let c = g.tab.boxes[w.box_].c;
    along(st, if w.along_x { [w.u + du, c[2]] } else { [c[0], w.u + du] })
}
fn wall_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let Some(def) = order_def(g, &crate::seats::here(g), "wallCharge") else { return Err(String::from("no wall charge in this build")) };
    if let Some(no) = (if b.prelude { &st.a.shut_wall } else { &st.a.shut_own }).clone() { return Err(no); }
    // (a refusal judged with every man on his place is the way's, and kept; one met mid-walk is asked again)
    let placed = flat(g.seat_pos(&crate::seats::here(g)), st.sam) <= K.on_slot + 0.1 && !away(g, st);
    let keep = |st: &mut KitStage, b: &Beat, no: String| -> Result<(), String> { if placed { wall_kept(st, b, &no); } Err(no) };
    let side = match st.order.first() { Some(&x) => { let v = along(st, home_of(g, st, x).0); if v != 0.0 { m::sign(v) } else { 1.0 } } None => 1.0 };
    let stack: Vec<GuardId> = st.order.iter().copied().filter(|&x| along(st, home_of(g, st, x).0) * side > 0.0).collect();
    let mut men = stack.clone();
    let mut asks: Vec<f64> = Vec::new();
    if st.walled { asks.push(0.0); if !b.prelude { asks.extend_from_slice(&K.wall_at); } } else { asks.extend_from_slice(&K.wall_at); }
    let mut t = None;
    let mut said: Vec<String> = Vec::new();
    for a in asks {
        let p = at(st, [a * side, 0.0, 0.0]);
        let mm: Vec<GuardId> = if st.walled { st.order.clone() } else { stack.clone() };
        let q = crate::orders::squad_orders_target_at(g, [p[0], 0.0, p[2]]);
        match crate::orders::squad_orders_why_not(g, def, &q, Some(&mm)) {
            None => { t = Some(q); b.spot = Some(p); men = mm; b.on_span = st.walled && a == 0.0; break; }
            Some(w) => { if !said.contains(&w) { said.push(w); } }
        }
    }
    let Some(t) = t else {
        b.on_span = st.walled;
        let no = if said.is_empty() { String::from("no wall by the stack to set it on") } else { said.join(" / ") };
        return keep(st, b, no);
    };
    b.base = g.sets[g.seat].wall_charge.stats.blown;
    st.a.wall_ask = Some((t.clone(), men.clone()));
    // (THE BLAST TOUCHES NOBODY: where the charge will sit is asked first; off the span, one that is to open it is refused and remembered; where Sam stands inside its reach he is walked out of it BEFORE the order is given)
    let w = crate::wallcharge::wall_charge_spot(g, &t);
    let pl = g.seat_pos(&crate::seats::here(g));
    if let (Some(w), true, true) = (w.as_ref(), st.walled, b.prelude) {
        let al = spot_along(g, st, w, 0.0);
        if m::abs(al) > st.width / 2.0 {
            let no = format!("the wall charge would sit {:.2} m from the span's middle: no hole on the span", m::abs(al));
            st.a.shut_wall = Some(no.clone());
            b.spot = None;
            return Err(no);
        }
    }
    let share = |g: &Game, x: f64, z: f64| -> f64 { match w.as_ref() { Some(w) => crate::wallcharge::wall_charge_blast_share(g, w, [x, 0.0, z], None, None), None => 0.0 } };
    if st.walled && share(g, pl[0], pl[2]) > 0.0 {
        let (a_s, o_s) = (along(st, st.sam), out_of(st, st.sam));
        let mut chose: Option<V2> = None;
        let wall = w.clone().unwrap();
        let wc = g.tab.boxes[wall.box_].c;
        let (cx, cz) = (if wall.along_x { wall.u } else { wc[0] }, if wall.along_x { wc[2] } else { wall.u });
        let mut dd = 0.5;
        while dd <= 6.0 && chose.is_none() {
            for gg in [at(st, [a_s - side * dd, o_s, 0.0]), at(st, [a_s, o_s + dd, 0.0]), at(st, [a_s - side * dd, o_s + dd, 0.0])] {
                let n = m::ceil(flat(pl, [gg[0], gg[2]]) / 0.25);
                let mut clear = true;
                let mut i = 1.0;
                while i <= n { if crate::scene_collision::nav_grid_is_blocked(g, pl[0] + (gg[0] - pl[0]) * i / n, pl[2] + (gg[2] - pl[2]) * i / n) { clear = false; break; } i += 1.0; }
                if !clear { continue; }
                let l = m::hyp2(cx - gg[0], cz - gg[2]);
                let l = if l != 0.0 { l } else { 1.0 };
                let mm = 2.0 * K.way_m + 0.1;
                if share(g, gg[0], gg[2]) == 0.0 && share(g, gg[0] + (cx - gg[0]) / l * mm, gg[2] + (cz - gg[2]) / l * mm) == 0.0 { chose = Some([gg[0], gg[2]]); break; }
            }
            dd += 0.5;
        }
        let Some(p) = chose else { b.spot = None; return keep(st, b, String::from("no floor for Sam to stand clear of the wall charge")) };
        b.post = Some(p);
        sam_goes(st, p);
        to(g, b, "clear");
        return Ok(());
    }
    let opts = crate::orders::GiveOpts { men: Some(men.clone()), ..crate::orders::GiveOpts::default() };
    if let Some(why) = crate::orders::squad_orders_give(g, "wallCharge", Some(&t), Some(Who::All), Some(&opts)) { b.spot = None; return keep(st, b, why); }
    b.moved = men;
    b.given = true;
    to(g, b, "place");
    Ok(())
}
fn wall_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let now = g.time;
    let c = g.sets[g.seat].wall_charge.set.first().copied();
    let op = g.sets[g.seat].wall_charge.ops.first().copied();
    if b.phase == "clear" {
        if flat(g.seat_pos(&crate::seats::here(g)), b.post.unwrap()) > K.way_m && now - b.pt < 8.0 { return Step::On; }
        st.a.led = None;
        let a = st.a.wall_ask.clone();
        let why = match a.as_ref() {
            Some((t, men)) => { let opts = crate::orders::GiveOpts { men: Some(men.clone()), ..crate::orders::GiveOpts::default() }; crate::orders::squad_orders_give(g, "wallCharge", Some(t), Some(Who::All), Some(&opts)) }
            None => Some(String::from("no wall to set it on")),
        };
        if let Some(why) = why { st.a.refused = Some(format!("walls: {}", why)); wall_kept(st, b, &why); b.marks.push("kept"); return Step::Done; }
        b.moved = a.unwrap().1;
        b.given = true;
        to(g, b, "place");
        return Step::On;
    }
    if !b.mark_t.is_nan() {
        if !b.loose {
            b.loose = true;
            for k in 0..b.moved.len() {
                let x = b.moved[k];
                let c0 = g.gd[x].ch;
                let (p, y) = (g.ch[c0].pos, g.ch[c0].body_yaw);
                go(g, st, x, vec![v2(p)], K.pace, 0.0, false);
                walk_of(st, x).unwrap().face = Some(y);
            }
        }
        return if now - b.mark_t >= K.aftermath { Step::Done } else { Step::On };
    }
    if let Some(c) = c {
        let wall = g.tab.wallcharge_set_wall_charge[c].wall.clone();
        let span = st.width / 2.0;
        let walled0 = st.walled;
        let on_span = move |al: f64| !walled0 || m::abs(al) <= span;
        if b.phase != "set" {
            to(g, b, "set");
            b.marks.push("set");
            let al = spot_along(g, st, &wall, 0.0);
            if b.prelude && !on_span(al) {
                let no = format!("the wall charge sits {:.2} m from the span's middle: no hole on the span", m::abs(al));
                st.a.shut_wall = Some(no.clone());
                st.a.refused = Some(format!("walls: {}", no));
                b.marks.push("kept"); b.marks.push("down");
                return Step::Done;
            }
        }
        let armed = op.map_or(false, |o| g.tab.wallcharge_wall_op[o].phase == crate::wallcharge::Phase::Armed);
        if armed || now - b.pt >= K.wall_clear {
            // (nobody inside its reach as it goes: the men still walking off are waited for, then the charge is taken down unblown)
            let mut in_it = 0;
            if st.walled {
                for k in 0..st.order.len() { let p = g.ch[g.gd[st.order[k]].ch].pos; if crate::wallcharge::wall_charge_blast_share(g, &wall, p, None, None) > 0.0 { in_it += 1; } }
                let p = g.seat_pos(&crate::seats::here(g));
                if crate::wallcharge::wall_charge_blast_share(g, &wall, p, None, None) > 0.0 { in_it += 1; }
            }
            if in_it > 0 {
                if now - b.pt < K.wall_clear + K.wall_late { return Step::On; }
                let no = format!("{} of the team could not stand clear of the wall charge", in_it);
                wall_kept(st, b, &no);
                st.a.refused = Some(format!("walls: {}", no));
                b.marks.push("kept"); b.marks.push("down");
                return Step::Done;
            }
            match crate::wallcharge::wall_charge_blow(g, Some(c), Some(false)) {
                Some(r) => {
                    mark(g, b, "blow");
                    let h = if r.hole { crate::wallcharge::wall_charge_hole_at(g, &wall) } else { None };
                    let al = spot_along(g, st, &wall, h.map_or(0.0, |h| h.du));
                    if r.hole && on_span(al) { b.marks.push("hole"); }
                    else if b.prelude { st.a.shut_wall = Some(if r.hole { format!("the wall charge's hole opened {:.2} m from the span's middle: not on the span", m::abs(al)) } else { String::from("the wall charge opened no hole in this wall") }); }
                }
                None => { b.marks.push("lost"); return Step::Done; }
            }
        }
        return Step::On;
    }
    if op.is_none() { b.marks.push("lost"); return Step::Done; }
    Step::On
}
// ------------------------------------------------------------ the big slot's tools
fn gave(g: &Game, st: &KitStage) -> bool { st.door.map_or(false, |d| g.dr[d].leaf.lock_broken || g.dr[d].leaf.off || m::abs(g.dr[d].leaf.angle - st.leaf_shut) > 0.2) }
fn big_of(st: &KitStage, i: u32) -> Option<BigKit> { st.a.cards.get(i as usize).map(|c| c.big).filter(|b| *b != BigKit::None) }
fn big_give(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Option<String> {
    let d = the_door(st);
    g.dr[d].leaf.lock_broken = false;
    pools(g, st, Some("bigtool"));
    let t = on_door(st);
    let who = roster(g, b.man.unwrap());
    if let Some(why) = crate::orders::squad_orders_give(g, crate::doortools::BIG_ORDER, Some(&t), Some(who), None) { return Some(why); }
    b.given = true;
    to(g, b, "go");
    None
}
fn big_mate_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if big_of(st, b.i).is_none() { return Err(String::from("nothing for a door in his big slot")); }
    let d = the_door(st);
    if door_steel(g, d) { return Err(String::from("steel: it holds")); }
    if g.dr[d].leaf.off { return Err(String::from("no leaf to break")); }
    if is_ajar(g, st) { to(g, b, "shut"); return Ok(()); }
    match big_give(g, st, b) { Some(w) => Err(w), None => Ok(()) }
}
fn big_mate_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let now = g.time;
    let d = the_door(st);
    let id = ch_id(g, b);
    let gone = door_op(g, &crate::seats::here(g), d).map_or(true, |o| g.tab.doortools_door_op[o].phase == crate::doortools::Phase::Enter);
    if b.phase == "shut" {
        if is_ajar(g, st) && now - b.pt < K.wait { doors_pull_to(g, d, id); return Step::On; }
        if let Some(why) = big_give(g, st, b) { st.a.refused = Some(format!("bigtool: {}", why)); b.marks.push("kept"); return Step::Done; }
        return Step::On;
    }
    if b.mark_t.is_nan() {
        if gave(g, st) { mark(g, b, "gave"); to(g, b, "open"); crate::doortools::door_tools_reset(g); pools(g, st, None); b.given = false; return Step::On; }
        if gone { b.marks.push("lost"); return Step::Done; }
        return Step::On;
    }
    crack(g, st, b, K.wide_light);
    if now - b.mark_t >= K.aftermath { Step::Done } else { Step::On }
}
fn big_sam_start(g: &Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let f = at(st, [K.own_post[0] * st.latch, K.own_post[1], 0.0]);
    let Some(tool) = big_of(st, 0) else { return Err(String::from("nothing for a door in his big slot")) };
    let d = the_door(st);
    if door_steel(g, d) { return Err(String::from("steel: it holds")); }
    if g.dr[d].leaf.off { return Err(String::from("no leaf to break")); }
    let mut q = f;
    let t = if tool == BigKit::Hammer { crate::kit::BigTool::Hammer } else { crate::kit::BigTool::Shotgun };
    if let Some(sp) = crate::bigkit::sam_big_spot(g, d, f, t) {
        let c = g.dr[d].frame_centre;
        let (dx, dz) = (sp.pos[0] - c[0], sp.pos[2] - c[2]);
        let l = m::hyp2(dx, dz);
        let k = if l > K.big_reach { K.big_reach / l } else { 1.0 };
        let p: V3 = [c[0] + dx * k, 0.0, c[2] + dz * k];
        if free(g, v2(p)) { q = p; }
    }
    if !free(g, v2(q)) { return Err(String::from("no room at the door")); }
    b.post = Some(v2(q));
    to(g, b, "go");
    sam_goes(st, v2(q));
    Ok(())
}
fn big_sam_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let now = g.time;
    let el = now - b.pt;
    let d = the_door(st);
    if b.phase == "go" {
        let id = ch_id(g, b);
        if is_ajar(g, st) { doors_pull_to(g, d, id); }
        if (flat(g.seat_pos(&crate::seats::here(g)), b.post.unwrap()) > K.way_m || is_ajar(g, st)) && el < 4.0 { return Step::On; }
        g.dr[d].leaf.lock_broken = false;
        st.a.led = None;
        to(g, b, "ask");
        b.leg_t = f64::NAN;
        return Step::On;
    }
    if b.phase == "ask" {
        if g.pl[pl].kick().is_some() { b.given = true; to(g, b, "work"); return Step::On; }
        if el >= K.own_ask { b.marks.push("kept"); st.a.refused = Some(String::from("big: the door never offered it")); return Step::Done; }
        let verb = if big_of(st, 0) == Some(BigKit::Hammer) { Verb::Hammer } else { Verb::Breach };
        if (b.leg_t.is_nan() || now - b.leg_t > 0.5) && crate::game::game_select_verb(g, InteractableKind::Door, Some(verb)) { press(st, "Space"); b.leg_t = now; }
        return Step::On;
    }
    if b.phase == "work" {
        if b.mark_t.is_nan() && gave(g, st) { mark(g, b, "gave"); }
        if g.pl[pl].kick().is_some() { return Step::On; }
        if b.mark_t.is_nan() { b.marks.push("held"); return Step::Done; }
        to(g, b, "off");
        sam_goes(st, b.home);
        return Step::On;
    }
    if flat(g.seat_pos(&crate::seats::here(g)), b.home) <= K.on_slot || el >= 3.0 { Step::Done } else { Step::On }
}
// ------------------------------------------------------------ a gun changed, suit and trim
fn sam_draw(st: &mut KitStage) {
    let pistol = st.a.cards.first().map_or(false, |c| c.weapon == crate::kit::KitWeapon::Pistol);
    press(st, if pistol { "Digit1" } else { "Digit2" });
}
fn swap_up(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let rows: &[Act] = if st.quiet && b.man.is_none() { &[Act::QuietGun, Act::Gun] } else { &[Act::Gun] };
    b.field = "weapon";
    for &r in rows {
        b.row = r;
        to(g, b, "act");
        if start(g, st, b).is_ok() { return Step::On; }
    }
    Step::Done
}
fn swap_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let el = g.time - b.pt;
    if b.man.is_some() { return if el >= K.swap_up { swap_up(g, st, b) } else { Step::On }; }
    let c = g.pl[g.player].ch;
    let busy = crate::character::anim_kit_busy(g, c);
    if (busy || el < 0.2) && el < 1.5 { return Step::On; }
    if b.phase == "stow" { super::cards::sam_card(g, st); to(g, b, "draw"); sam_draw(st); return Step::On; }
    swap_up(g, st, b)
}
fn turn_start(g: &Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    let (a, c) = (b.yaw + K.turn_deg * R, b.yaw - K.turn_deg * R);
    let y = if m::sin(a) * st.n[0] + m::cos(a) * st.n[1] >= m::sin(c) * st.n[0] + m::cos(c) * st.n[1] { a } else { c };
    b.quiet = true; b.turn = true; b.h_to = 1.1;
    b.legs = vec![Leg { yaw: y, cap: K.body_rate * R, min_t: K.turn_ease, hold: K.turn_secs }];
    if b.man.is_none() { sam_stand(g, st); }
    Ok(())
}
// ------------------------------------------------------------ one start and one tick per row
pub(super) fn start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    match b.row {
        Act::Gun | Act::GunDoor => gun_start(g, st, b),
        Act::Light => light_start(g, st, b),
        Act::BangOrder | Act::FragOrder => order_start(g, st, b),
        Act::ThrowSmoke | Act::ThrowFlash | Act::ThrowFrag => thrown_start(g, st, b),
        Act::ChargeMate => {
            b.base = g.sets[g.seat].door_tools.stats.blown;
            let t = on_door(st);
            let who = roster(g, b.man.unwrap());
            match crate::orders::squad_orders_give(g, "charge", Some(&t), Some(who), None) { Some(w) => Err(w), None => Ok(()) }
        }
        Act::ChargeSam => charge_sam_start(g, st, b),
        Act::WandSam => wand_start(g, st, b),
        Act::WandMate => wand_mate_start(g, st, b),
        Act::Cam => cam_start(g, st, b),
        Act::Shell | Act::Ring => tube_start(g, st, b),
        Act::WallCharge => wall_start(g, st, b),
        Act::BigMate => big_mate_start(g, st, b),
        Act::BigSam => big_sam_start(g, st, b),
        Act::Turn => turn_start(g, st, b),
        Act::Swap => {
            to(g, b, "stow");
            if b.man.is_none() { unpress(st, "Digit1"); unpress(st, "Digit2"); if !g.pl[g.player].holstered { press(st, "KeyE"); } }
            Ok(())
        }
        Act::QuietGun => quiet::gun_start(g, st, b),
        Act::QuietLean => quiet::lean_start(g, st, b),
        Act::QuietLight => quiet::light_start(g, st, b),
    }
}
fn tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    match b.row {
        Act::Gun | Act::GunDoor => gun_tick(g, st, b),
        Act::Light => light_tick(g, st, b),
        Act::BangOrder | Act::FragOrder => order_tick(g, st, b),
        Act::ThrowSmoke | Act::ThrowFlash | Act::ThrowFrag => thrown_tick(g, st, b),
        Act::ChargeMate => charge_mate_tick(g, st, b),
        Act::ChargeSam => charge_sam_tick(g, st, b),
        Act::WandSam => wand_tick(g, st, b),
        Act::WandMate => wand_mate_tick(g, st, b),
        Act::Cam => cam_tick(g, st, b),
        Act::Shell | Act::Ring => tube_tick(g, st, b),
        Act::WallCharge => wall_tick(g, st, b),
        Act::BigMate => big_mate_tick(g, st, b),
        Act::BigSam => big_sam_tick(g, st, b),
        Act::Turn => quiet_tick(g, st, b),
        Act::Swap => swap_tick(g, st, b),
        Act::QuietGun => quiet::gun_tick(g, st, b),
        Act::QuietLean => quiet::lean_tick(g, st, b),
        Act::QuietLight => quiet::light_tick(g, st, b),
    }
}
// ------------------------------------------------------------ the director
/// everything a staging needs before the world is let go: Sam's yaws governed, each man sent to his place from where he stands, the pools as the cards have them
pub(in crate::outfit) fn live(g: &mut Game, st: &mut KitStage) {
    if st.a.cards.is_empty() { st.a.cards = g.m.kit.kit.members.clone(); }
    if st.who > 0 && actor(g, st).is_none() { st.who = 0; }
    let c = g.pl[g.player].ch;
    st.a.body = chan(g.ch[c].body_yaw);
    st.a.aim = chan(g.ch[c].aim_yaw);
    st.a.sam_ch = chan(g.ch[c].aim_yaw);
    st.a.sam_h = 0.0;
    st.a.sam_at = f64::NAN;
    st.a.walks.clear();
    st.a.swaps.clear();
    st.a.scans.clear();
    st.a.beam = None;
    st.a.kneel = None;
    st.a.home_t = f64::NAN;
    st.a.hole_men.clear();
    st.a.hole_all = false;
    st.a.beat = None;
    st.a.apart = false;
    st.a.dirty = false;
    st.a.shot = false;
    st.a.hole_shots = 0;
    st.a.fog_up = false;
    st.a.wand_kit = None;
    st.a.tube_was = None;
    st.a.hand_due = None;
    st.a.lift_at = f64::NAN;
    st.a.rise = 0.0;
    stay(g, st, false);
    for i in 0..st.order.len() { let x = st.order[i]; let p = home_of(g, st, x).0; go(g, st, x, vec![p], K.pace, 0.0, false); }
    let team = g.staged.team.clone();
    for (k, &x) in team.iter().enumerate() {
        let i = k as u32 + 1;
        let c = g.gd[x].ch;
        let m0 = g.m.kit.kit.members.get(i as usize);
        let (s_issued, t_issued) = (m0.map_or(true, |m| m.suit == crate::kit::SuitId::Issued), m0.map_or(true, |m| m.trim == crate::kit::TrimId::Issued));
        if s_issued { let v = g.ch[c].tint; st.a.issued_suit.retain(|e| e.0 != i); st.a.issued_suit.push((i, v)); }
        if t_issued { let v = g.ch[c].tint2; st.a.issued_trim.retain(|e| e.0 != i); st.a.issued_trim.push((i, v)); }
    }
    pools(g, st, None);
    st.a.vent_at = -1.0;
    st.a.vent_kill = -1.0;
    st.a.rest_until = g.time + if st.a.asked.is_some() || st.a.seated { 0.0 } else { K.rest_stage };
    if st.who > 0 { let m0 = actor(g, st); rotate(g, st, m0); }
}
/// WHAT LIVES ON THROUGH A RESTAGE: the game and every row id are new and its clock has gone back; what the director was asked and what it remembers of this way, and the counters a check reads, go on
pub(super) fn carry(was: &Acts, a: &mut Acts) {
    a.cards = was.cards.clone();
    (a.hi, a.asked, a.flip, a.rest_next, a.beat_n) = (was.hi, was.asked, was.flip, was.rest_next, was.beat_n);
    (a.seated, a.seat_tried) = (was.seated, was.seat_tried);
    (a.shut_wall, a.shut_own) = (was.shut_wall.clone(), was.shut_own.clone());
    (a.issued_suit, a.issued_trim) = (was.issued_suit.clone(), was.issued_trim.clone());
    a.kept = was.kept.clone();
    (a.cost_n, a.cost_cuts, a.cost_last) = (was.cost_n, was.cost_cuts, was.cost_last);
    (a.life_on, a.life_seen, a.life_cuts, a.life_n) = (was.life_on, was.life_seen, was.life_cuts, was.life_n);
    (a.last, a.refused) = (was.last.clone(), was.refused.clone());
    (a.cuts, a.shots, a.hits_sam, a.hits_team, a.pins) = (was.cuts, was.shots, was.hits_sam, was.hits_team, was.pins);
    a.restages = was.restages + 1;
}
pub(super) fn finish(g: &mut Game, st: &mut KitStage, mut b: Beat, how: &'static str) {
    st.a.last_end = g.time;
    let team_row = row(b.row).team;
    let far = team_row && st.order.iter().any(|&x| walk::off_home(g, st, x) > K.wall_off);
    if st.a.fog_up {
        st.a.fog_up = false;
        g.fog_of_war.on = false;
        g.fog_picture.viewer = false;
        crate::fogofwar::fog_of_war_reset(g);
        let pl = g.player;
        g.pl[pl].wand = st.a.wand_was;
        if let Some((x, was)) = st.a.wand_kit.take() { if let Some(e) = g.m.kit.man_of.get_mut(x) { e.kit.wand = was; } }
    }
    st.a.last = Some(BeatOver { n: b.n, field: b.field, phase: b.phase, marks: b.marks.clone(), how });
    if b.field == "smoke" && !b.mark_t.is_nan() { st.a.vent_at = b.mark_t + K.vent_secs; }
    st.a.clicked = 0.0;
    b.lit = false;
    st.a.beam = None;
    st.a.kneel = None;
    stay(g, st, false);
    let now = g.time;
    for k in 0..st.order.len() { let x = st.order[k]; let l = low_at(g, st, x); low(g, st, x, l); }
    let quick = how == "cut";
    let hole_made = b.prelude && b.marks.contains(&"hole");
    st.a.home_men = b.moved.iter().copied().filter(|&x| Some(x) != b.man).collect();
    // (at a wall left open: who the next row waits for. The charge that opened it: everybody, Sam too. Any other row: the men it moved, once)
    if hole_made { st.a.hole_men = st.order.clone(); st.a.hole_all = true; }
    else if st.hole { for x in b.moved.iter().copied().chain(b.man) { if !st.a.hole_men.contains(&x) { st.a.hole_men.push(x); } } }
    if !far || hole_made {
        let n = b.moved.len();
        for j in 0..n {
            let x = b.moved[j];
            if Some(x) != b.man && st.order.contains(&x) {
                let p = home_of(g, st, x).0;
                let (pace, delay) = if quick || hole_made { (K.swap_pace, 0.0) } else { (K.pace, K.shuffle[1] * (n - 1 - j) as f64) };
                go(g, st, x, vec![p], pace, delay, false);
                walk_of(st, x).unwrap().t0 = -1e9;
            }
        }
    }
    let pl = g.player;
    if b.man.is_none() { g.pl[pl].crouch = st.quiet; }
    if b.shots > 0 {
        let gun = match b.man { Some(x) => Some(g.gd[x].weapon), None => crate::game::player_gun_in_hand(g, pl) };
        if let Some(w) = gun { crate::weapons::firearm_rearm(g, w); }
    }
    match b.man {
        Some(x) => {
            let lane = walk_of(st, x).map_or(false, |w| { w.watch = None; w.face = None; w.lane });
            let in_order = st.order.contains(&x);
            let hy = home_of(g, st, x).1;
            if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), x) {
                let r = &mut g.tab.orders_assignment[a];
                if r.order == "cover" { r.watch = None; r.upper = crate::character::UpperMode::None; r.stance = crate::character::Stance::LowReady; if in_order { r.face = Some(hy); } }
            }
            if in_order && !lane && (!far || hole_made) {
                let h = home_of(g, st, x).0;
                let pts = match b.via { Some(q) => vec![q, h], None => vec![h] };
                go(g, st, x, pts, if how == "cut" || hole_made { K.swap_pace } else { K.pace }, 0.0, false);
            }
        }
        None => {
            if let Some(h) = b.hand.filter(|&h| h != g.pl[pl].slot) {
                if st.quiet { g.pl[pl].slot = h; g.pl[pl].holstered = true; }
                else { press(st, if h == SlotKind::Rifle { "Digit2" } else { "Digit1" }); st.a.hand_due = Some(h); }
            }
            if st.a.led.map_or(true, |l| l.at.is_none()) { let a = st.a.aim; st.a.sam_ch.v = a.v; st.a.sam_ch.want = a.v; st.a.sam_ch.w = a.w; st.a.sam_ch.own = false; }
            st.a.led = None;
            if let Some(i) = st.a.tube_was.take() { let r = g.pl[pl].rifle; g.fa[r].att_idx = i; }
            if b.row == Act::Swap && b.phase == "stow" { super::cards::sam_card(g, st); sam_draw(st); }
        }
    }
    let info = row(b.row);
    let kept = b.marks.contains(&"kept");
    st.a.rest_until = now + if how == "cut" { 0.0 } else if kept { m::min(info.rest, K.rest_quiet) } else if st.a.hi.is_none() && info.damage == Damage::None && info.rest == K.rest_quiet { K.rest_none } else { info.rest };
    let door_gone = st.door.map_or(false, |d| g.dr[d].leaf.off);
    // A WALL THAT OPENS ONCE, LOUDLY: the charge that opened a blank wall for another row leaves its hole, and the row asked plays through it from then on
    if hole_made {
        st.hole = true;
        st.a.apart = false;
        st.a.rest_until = now + if how == "cut" { 0.0 } else { K.rest_quiet };
    } else if info.damage != Damage::None && (door_gone || b.marks.contains(&"frag") || b.marks.contains(&"gave") || (info.team && (b.marks.contains(&"blow") || far))) {
        st.a.dirty = true;
        st.a.apart = st.a.apart || far;
        if how != "cut" && !b.mark_t.is_nan() { st.a.rest_until = b.mark_t + K.hold; }
        else if how != "cut" && info.team { st.a.rest_until = now + K.rest_quiet; }
    }
    // THE LAMP SHOT OUT has no undo under the eye: the dark held to the row's end, then the room staged again
    if b.row == Act::QuietGun && st.pool.map_or(false, |p| g.tab.fixture[p].broken) {
        st.a.dirty = true;
        st.a.rest_until = now;
        st.a.rest_next = if how == "cut" { f64::NAN } else { QK.gun_rest };
    }
    // THE WALL CHARGE'S ROW OVER WITH NOTHING BLOWN: remembered for this way, and a charge still on the wall goes under the dissolve
    if b.row == Act::WallCharge && how != "cut" && !b.marks.contains(&"blow") {
        let no = if b.marks.contains(&"cap") { String::from("the wall charge never went off inside its row") }
            else { let r = st.a.refused.clone().unwrap_or_default(); let r = r.split_once(": ").map_or(r.clone(), |x| String::from(x.1)); if r.is_empty() { String::from("the wall charge came down unblown") } else { r } };
        wall_kept(st, &b, &no);
        if st.a.refused.is_none() { st.a.refused = Some(format!("walls: {}", no)); }
        if !g.sets[g.seat].wall_charge.set.is_empty() || !g.sets[g.seat].wall_charge.ops.is_empty() || b.marks.contains(&"down") || far { st.a.dirty = true; st.a.apart = st.a.apart || far; st.a.rest_until = now; }
    }
    // A LEAF SHOT TO PIECES under a row that says it leaves nothing
    if door_gone && !st.a.dirty { st.a.dirty = true; }
    // LOUD KIT'S COST: a loud row that went off is answered from inside; what the row left is mended by the cost's own restage
    let gun_row = b.row == Act::Gun || b.row == Act::GunDoor;
    let supp = match b.man { Some(x) => Some(g.gd[x].weapon), None => crate::game::player_gun_in_hand(g, pl) }.map_or(false, |w| g.m.weapons.weapons.get(g.fa[w].def).map_or(false, |d| d.suppressed));
    if quiet::costly(st.quiet, info.loud, b.prelude, how, gun_row, b.shots, supp, !b.mark_t.is_nan()) { quiet::cost_begin(g, st); }
}
/// THE CUT: a pick ends whatever is running, now.
pub(super) fn cut(g: &mut Game, st: &mut KitStage) {
    let Some(mut b) = st.a.beat.take() else { return };
    let pl = g.player;
    st.a.cuts += 1;
    if b.man.is_none() && b.item.is_none() { g.pl[pl].pending_throw = None; g.sets[g.seat].hand.throw_held = false; }
    if b.man.is_none() { g.pl[pl].charging_over(); }
    if b.field == "smoke" && (b.thrown || b.item.is_some()) { shut_cans(g); st.a.vent_kill = g.time + K.vent_kill; }
    // (asked before what lies unfired is taken up: a frag taken off the floor has not gone off)
    if b.field == "frags" && b.mark_t.is_nan() && (g.sets[g.seat].door_tools.stats.frags > b.base || item_went(g, &b)) { mark(g, &mut b, "frag"); }
    let mut i = g.items.items.len();
    while i > 0 { i -= 1; let it = g.items.items[i]; if g.tab.item[it].kind == th::ItemKind::Frag && !g.tab.item[it].fired { th::throwables_remove_at(g, i as f64); } }
    if b.man.is_none() { g.pl[pl].launch_held = None; if b.row == Act::Cam { g.pl[pl].cams.clear(); } }
    let info = row(b.row);
    let on_wall = info.team && !g.sets[g.seat].wall_charge.set.is_empty();
    if info.team && !on_wall { crate::wallcharge::wall_charge_reset(g); }
    if b.row == Act::BigSam { g.pl[pl].kick_over(); }
    if info.door == Leaf::None || b.given { crate::doortools::door_tools_reset(g); pools(g, st, None); }
    finish(g, st, b, "cut");
    if on_wall { st.a.dirty = true; }
}
/// what the last rows left on the floor, off it once it is old: nothing is taken away under the eye
fn litter(g: &mut Game) {
    let mut i = g.items.items.len();
    while i > 0 { i -= 1; let it = g.items.items[i]; if g.tab.item[it].age > K.litter { th::throwables_remove_at(g, i as f64); } }
    while g.items.items.len() > 5 { th::throwables_remove_at(g, 0.0); }
}
fn begin(g: &mut Game, st: &mut KitStage, f0: &'static str) {
    let man = actor(g, st);
    if let Some(x) = man { if !st.order.contains(&x) { st.a.rest_until = g.time + 1.0; return; } }
    let mut f = f0;
    let lit_card = st.a.cards.get(st.who as usize).map_or(false, |c| c.light);
    if f == "light" && !lit_card { f = "weapon"; }
    if beats(f, man.is_some()).is_none() { f = "weapon"; }
    let q_row = if st.quiet { quiet_beats(f, man.is_some()) } else { None };
    let c = match man { Some(x) => g.gd[x].ch, None => g.pl[g.player].ch };
    let (home, yaw) = match man { Some(x) => home_of(g, st, x), None => (st.sam, super::walk::faces(st, st.sam)) };
    let now = g.time;
    let table = beats(f, man.is_some()).unwrap();
    let mut b = Beat {
        n: st.a.beat_n + 1, field: f, i: st.who, time: 0.0, phase: "act", marks: Vec::new(), row: q_row.unwrap_or(table), prelude: false, on_span: false, man, ch_id: c, home, yaw, t0: now, pt: now, mark_t: f64::NAN, base: 0.0,
        post: None, spot: None, lit: false, thrown: false, given: false, hand: None, item: None, old: g.items.items.clone(), rest: None,
        ch: chan(g.ch[c].aim_yaw), legs: Vec::new(), leg: 0, leg_t: f64::NAN, quiet: false, turn: false, pitch: None, rose: false, h_to: 1.1, moved: Vec::new(), loose: false, fire: None, shots: 0, shot_t: f64::NAN, via: None, drift: 0.0,
    };
    // (the wall charge's own tile at a wall already open: the wall stood again under the dissolve first, then its whole row. So too after `hole_bursts` bursts through it)
    if st.hole && (b.row == Act::WallCharge || (st.a.shot && st.a.hole_shots >= K.hole_bursts)) { st.a.dirty = true; st.a.asked = Some(f); return; }
    if row(b.row).opening && walled(st) {
        if let Some(no) = st.a.shut_wall.clone() {
            st.a.refused = Some(format!("{}: {}", f, no));
            if man.is_none() { st.a.led = None; }
            return begin(g, st, "weapon");
        }
        b.row = Act::WallCharge; b.prelude = true; b.marks.push("prelude");
    }
    mend(g, st);
    litter(g);
    pools(g, st, Some(if b.prelude { "walls" } else { f }));
    let pl = g.player;
    g.pl[pl].cams.clear();
    let leafless = |st: &KitStage, r: Act| row(r).leaf && st.door.is_none();
    let mut why = if leafless(st, b.row) { Err(String::from("no leaf at this way in")) } else { start(g, st, &mut b) };
    if q_row.is_some() {
        st.a.kept = why.as_ref().err().map(|w| format!("{}: {}", f, w));
        if why.is_err() { b.row = table; why = if leafless(st, b.row) { Err(String::from("no leaf at this way in")) } else { start(g, st, &mut b) }; }
    }
    if let Err(w) = why {
        st.a.refused = Some(format!("{}: {}", f, w));
        if man.is_none() { st.a.led = None; }
        if f != "weapon" { return begin(g, st, "weapon"); }
        st.a.rest_until = now + 1.0;
        return;
    }
    if st.quiet && man.is_none() && g.pl[pl].holstered && matches!(b.row, Act::Gun | Act::GunDoor | Act::Light) { let k = if g.pl[pl].slot == SlotKind::Rifle { "Digit2" } else { "Digit1" }; press(st, k); }
    st.a.beat_n += 1;
    if f != "weapon" || !matches!(b.row, Act::Gun | Act::GunDoor | Act::QuietGun) { st.a.refused = None; }
    st.a.beat = Some(b);
}
/// between and under the rows: the leaf pulled to when no row has it open, Sam walked the half step home, a door order nobody's row owns taken down
fn tidy(g: &mut Game, st: &mut KitStage) {
    let pl = g.player;
    let now = g.time;
    let (b_some, b_leaf_shut, b_leaf, b_man, b_team, b_id) = match st.a.beat.as_ref() { Some(b) => { let r = row(b.row); (true, r.door == Leaf::Shut, r.leaf, b.man, r.team, Some(g.ch[b.ch_id].id)) } None => (false, false, false, None, false, None) };
    let to_shut = st.leaf_rest == st.leaf_shut || (if b_some { b_leaf } else { now < st.a.leaf_ask });
    if let Some(d) = st.door {
        if (!b_some || b_leaf_shut) && !g.dr[d].leaf.off {
            if to_shut && m::abs(g.dr[d].leaf.angle - st.leaf_shut) >= 0.03 && now - st.a.pulled > 1.0 {
                st.a.pulled = now;
                let id = b_id.unwrap_or_else(|| match st.order.first() { Some(&x) => g.ch[g.gd[x].ch].id, None => g.seat_id(&crate::seats::here(g)) });
                doors_pull_to(g, d, id);
            }
            if !to_shut && st.a.cost_t.is_nan() && m::abs(g.dr[d].leaf.angle - st.leaf_rest) > 1e-4 { walk::ease_leaf(g, st, d); }
        }
    }
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    // (his gun, left out by a row, goes back once he is home)
    if st.quiet && !b_some && st.a.led.is_none() && !g.pl[pl].holstered && matches!(g.pl[pl].slot, SlotKind::Pistol | SlotKind::Rifle) && now - st.a.stow_t > 1.0 && flat(p, st.sam) <= K.on_slot && !crate::character::anim_kit_busy(g, c) && st.a.asked.is_none() && now - st.a.last_end >= QK.stow {
        st.a.stow_t = now;
        press(st, "KeyE");
    }
    // (at a wall left open what the charge threw down may lie on his place: a half step home that gets no nearer for a second is given up, and where he stands is his place)
    let plain_walk = st.a.led.map_or(false, |l| l.at.is_none());
    if st.hole && plain_walk && (!b_some || b_man.is_some()) {
        let dh = flat(p, st.sam);
        if dh <= K.on_slot || dh < st.a.stuck_d - 0.02 { st.a.stuck_d = dh; st.a.stuck_t = now; }
        else if now - st.a.stuck_t >= 1.0 && dh <= K.sam_give { st.sam = v2(p); st.a.stuck_d = f64::INFINITY; st.a.stuck_t = now; }
    } else { st.a.stuck_d = f64::INFINITY; st.a.stuck_t = now; }
    // (the quiet picture: he gives way first: with a partner's card open his place is a step back along the wall)
    if let Some(back) = st.sam_back { st.sam = if actor(g, st).is_some() { back } else { st.sam_rest }; }
    let slack = if st.quiet && b_man.is_some() { 0.25 } else { 0.0 };
    if !st.a.cost_t.is_nan() { st.a.led = Some(Led { at: Some(at(st, [0.0, 0.0, QK.cost.aim_y])), ..Led::default() }); }
    else if (!b_some || b_man.is_some()) && !b_team {
        // (the men first. At a wall left open: only the men it has yet to wait for, `swap_secs` at most: then he is walked home by the same wait)
        // (and never under the man whose card just opened: he is on his way past him)
        let men_first = !b_some && walk::arriving(g, st) || !b_some && actor(g, st).is_none() && away(g, st) && (!st.hole || (!st.a.hole_men.is_empty() && (st.a.home_t.is_nan() || now - st.a.home_t < K.swap_secs)));
        if flat(p, st.sam) > K.on_slot + slack && !men_first { { let home = st.sam; sam_goes(st, home) }; }
        else { st.a.led = None; }
    }
    if !b_some && (!g.sets[g.seat].door_tools.ops.is_empty() || !g.sets[g.seat].door_tools.set.is_empty()) { crate::doortools::door_tools_reset(g); pools(g, st, None); }
    if !b_some && !st.a.dirty && (!g.sets[g.seat].wall_charge.ops.is_empty() || !g.sets[g.seat].wall_charge.set.is_empty()) { crate::wallcharge::wall_charge_reset(g); pools(g, st, None); }
    if !b_some && st.a.shot && now - st.a.shot_at >= K.hold { mend(g, st); }
}
/// THE WAIT: no row is begun into a refusal that is about to clear. `wait` seconds at most, then begun as it stands
fn waits(g: &Game, seat: &crate::seats::SeatRef, st: &mut KitStage, f: &str) -> bool {
    let man = actor(g, st);
    let row0 = row_for(st, f, man.is_some());
    let row1 = if row0.map_or(false, |a| row(a).opening) && walled(st) && st.a.shut_wall.is_none() { Some(Act::WallCharge) } else { row0 };
    let r = if row1 == Some(Act::WallCharge) && row0 == Some(Act::WallCharge) && st.a.shut_own.is_some() { Some(Act::Gun) } else { row1 };
    let info = r.map(row);
    let sam_off = flat(g.seat_pos(seat), st.sam) > K.on_slot;
    let wait = (man.is_some() && st.a.led.is_some())
        || info.map_or(false, |i| i.team && (st.a.led.is_some() || sam_off))
        || (info.map_or(false, |i| i.door == Leaf::None) && !matches!(r, Some(Act::ChargeSam) | Some(Act::BigSam) | Some(Act::BigMate)) && leaf_wait(g, st));
    if !wait { return false; }
    let now = g.time;
    if st.a.wait_t.is_nan() { st.a.wait_t = now; }
    if st.leaf_rest != st.leaf_shut && info.map_or(false, |i| i.door == Leaf::None) { st.a.leaf_ask = now + 0.5; }
    now - st.a.wait_t < K.wait
}
/// the director's own hand, each tick: a pick's cut, the row's tick, the tidying, and what begins
fn direct(g: &mut Game, st: &mut KitStage) {
    if st.a.cut_now { st.a.cut_now = false; if st.a.beat.is_some() { cut(g, st); } }
    if let Some(mut b) = st.a.beat.take() {
        let mut r = tick(g, st, &mut b);
        if r == Step::On && b.time >= row(b.row).cap { b.marks.push("cap"); r = Step::Done; }
        if r == Step::Done { let how = if b.marks.contains(&"cap") { "cap" } else { "done" }; finish(g, st, b, how); } else { st.a.beat = Some(b); }
    }
    tidy(g, st);
    if !st.shown { return; }
    quiet::life(g, st);
    if st.a.beat.is_some() { return; }
    if quiet::cost_tick(g, st) { return; }
    let now = g.time;
    let asked = st.a.asked;
    let pick_now = asked.is_some();
    if !pick_now && now < st.a.rest_until { return; }
    if st.a.dirty { st.a.dirty = false; ask_veil(g, st); return; }
    if walk::arriving(g, st) { return; }
    if walk::homing(g, st) { return; }
    if st.quiet && !pick_now && st.a.hi.is_none() { st.a.rest_until = now + 0.5; return; }
    if !pick_now && !st.a.rest_next.is_nan() { st.a.rest_until = now + st.a.rest_next; st.a.rest_next = f64::NAN; return; }
    st.a.rest_next = f64::NAN;
    let f: &'static str = match asked { Some(f) => f, None => st.a.hi.unwrap_or(if st.a.flip % 2 == 1 { "light" } else { "weapon" }) };
    if waits(g, &crate::seats::here(g), st, f) { return; }
    st.a.wait_t = f64::NAN;
    if pick_now { st.a.asked = None; } else if st.a.hi.is_none() { st.a.flip += 1; }
    if st.a.sam_card_due && f != "swap" { super::cards::sam_card(g, st); }
    begin(g, st, f);
}
/// a thing asked for now: a pick cuts whatever is running and its thing starts from where he stands
pub(super) fn pick(st: &mut KitStage, f: Option<&'static str>) {
    st.a.asked = Some(f.unwrap_or("weapon"));
    st.a.wait_t = f64::NAN;
    if f.is_none() { st.a.flip = 1; }
    if st.a.beat.is_some() { st.a.cut_now = true; }
}
/// EACH TICK, BEFORE THE GAME'S STEP. `cmd`'s cursor is the foot of the door already.
pub(in crate::outfit) fn before_step(g: &mut Game, st: &mut KitStage, cmd: &crate::cmd::PlayerCmd) {
    let now = g.time;
    st.a.step_dt = clamp(now - st.a.step_t, 0.0, 0.25);
    st.a.step_t = now;
    let mut i = st.a.swaps.len();
    while i > 0 {
        i -= 1;
        let w = st.a.swaps[i];
        if now < w.at { continue; }
        st.a.swaps.remove(i);
        if !st.order.contains(&w.man) { continue; }
        let c = g.gd[w.man].ch;
        let gun = g.fa[g.gd[w.man].weapon].def;
        if gun != w.id && !super::cards::short_gun(gun) && !super::cards::short_gun(w.id) { g.ch[c].anim.long_gun = false; }
        let route = g.gd[w.man].route;
        g.tab.route[route].weapon = Some(w.id);
    }
    if st.a.sam_card_due && !st.a.beat.as_ref().map_or(false, |b| b.man.is_none()) && st.a.asked != Some("swap") { super::cards::sam_card(g, st); }
    if let Some(b) = st.a.beat.as_mut() { b.time = now - b.t0; }
    if st.veil_asked.is_none() {
        let pl = g.player;
        let ended = super::stage::staged_ended(g, st.asks_none);
        if crate::condition::down(g, g.pl[pl].ch) || st.order.iter().any(|&x| crate::condition::down(g, g.gd[x].ch)) || ended {
            if let Some(b) = st.a.beat.take() { finish(g, st, b, "down"); }
            ask_veil(g, st);
        } else { direct(g, st); }
    }
    walk::walks(g, st);
    aim::aims(g, st);
    aim::idle(g, st, cmd);
    st.a.hand_due = None;
}
/// …AND AFTER IT: Sam's yaws, the thing thrown followed, a can's vent shut off, the hold through Sam's frag
pub(in crate::outfit) fn after_step(g: &mut Game, st: &mut KitStage) {
    let now = g.time;
    let pl = g.player;
    let c = g.pl[pl].ch;
    aim::yaws(g, st, clamp(now - st.a.yaw_t, 0.0, 0.25));
    st.a.yaw_t = now;
    st.a.beam = None;
    if let Some(b) = st.a.beat.as_ref() {
        if b.lit {
            let (y, pit) = (g.ch[b.ch_id].aim_yaw, b.pitch.map_or(0.0, |p| p.v));
            st.a.beam = Some([m::sin(y) * m::cos(pit), m::sin(pit), m::cos(y) * m::cos(pit)]);
        }
    }
    if let Some(mut b) = st.a.beat.take() {
        if b.item.is_none() { let mut i = g.items.items.len(); while i > 0 { i -= 1; let it = g.items.items[i]; if !b.old.contains(&it) { b.item = Some(it); break; } } }
        if let Some(it) = b.item { if g.items.items.contains(&it) { b.rest = Some(g.tab.item[it].pos); } else { /* (it has gone: what a row reads of it is that it went off) */ } }
        for k in 0..st.order.len() {
            let x = st.order[k];
            if Some(x) != b.man && !st.a.walks.iter().find(|w| w.man == x).map_or(false, |w| w.lane) { b.drift = m::max(b.drift, walk::off_home(g, st, x)); }
        }
        if b.man.is_some() { b.drift = m::max(b.drift, flat(g.ch[c].pos, st.sam)); }
        // THE HOLD, Sam's frag: whatever the game's own men would do about a live grenade, every teammate is on his place and, from the moment the row has them down, on his knee. Counted.
        if st.a.pin == Some(b.n) {
            let el = clamp(now - st.a.pin_t, 0.0, 0.25);
            for k in 0..st.order.len() {
                let x = st.order[k];
                if Some(x) == b.man { continue; }
                let (hp, hy) = home_of(g, st, x);
                let c0 = g.gd[x].ch;
                let mut put = false;
                if flat(g.ch[c0].pos, hp) > K.home_m + 0.02 {
                    g.ch[c0].pos = [hp[0], 0.0, hp[1]];
                    g.ch[c0].vel = [0.0; 3];
                    g.gd[x].speed = 0.0;
                    g.ch[c0].anim.speed = 0.0;
                    g.gd[x].path.clear();
                    g.gd[x].path_goal = None;
                    g.ch[c0].body_yaw = hy;
                    put = true;
                }
                if st.a.kneel == Some(b.n) && now - st.a.kneel_t > 0.3 {
                    let was = st.a.knees.iter().find(|e| e.0 == x).map_or(g.ch[c0].anim.crouch, |e| e.1);
                    if g.ch[c0].anim.crouch_target < 1.0 || g.ch[c0].anim.crouch < was - 1e-3 {
                        g.ch[c0].anim.crouch_target = 1.0;
                        g.ch[c0].anim.crouch = m::min(1.0, was + el * 4.0);
                        put = true;
                    }
                    let v = g.ch[c0].anim.crouch;
                    st.a.knees.retain(|e| e.0 != x);
                    st.a.knees.push((x, v));
                }
                if put { st.a.pins += 1; }
            }
        }
        // (Sam's own charge with no floor for the stack to step back onto, a wall charge cut short by the clock: nobody of his is left blind by it. Counted)
        let info = row(b.row);
        if (b.man.is_none() && info.damage == Damage::Leaf) || info.team {
            for k in 0..st.order.len() {
                let x = st.order[k];
                let r = &mut g.cond[g.gd[x].ch];
                if r.blind_until > now || r.fold_until > now { st.a.hits_team += 1; }
                if r.blind_until > now { r.blind_until = -1.0; }
                if r.fold_until > now { r.fold_until = -1.0; }
            }
        }
        st.a.beat = Some(b);
    }
    st.a.pin_t = now;
    if g.cond[g.pl[pl].ch].fold_until > now { st.a.hits_sam += 1; }
    // (a can whose fuse runs out under a cut vents nothing: the word to shut comes before a tick's sources are played, so one told this very tick has to be told shut. And what has been told since the air was last cleared)
    let killed = now < st.a.vent_kill && !st.a.beat.as_ref().map_or(false, |b| b.field == "smoke");
    for e in g.out.iter_mut() {
        if let crate::out_events::OutEvent::SmokeSystemLikeEmit(o) = e {
            st.a.smoke_told = true;
            if killed && matches!(o.e.kind, crate::smoke_types::SmokeEmitterKind::Vent | crate::smoke_types::SmokeEmitterKind::Canister) { o.e.ttl = 0.0; }
        }
    }
    if st.a.vent_at >= 0.0 && now >= st.a.vent_at { shut_cans(g); st.a.vent_at = -1.0; }
    if killed { shut_cans(g); }
}
