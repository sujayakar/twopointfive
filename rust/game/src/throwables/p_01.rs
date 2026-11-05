//! src/game/throwables.ts lines 1 to 201
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A column's place in a row's numbers (sim/itemshome.ts IC), named once here: `crate::sim_itemshome::ic()` builds a map of 26 Strings a call and every getter of
/// every thing reads a place. The tests at the foot hold these to `ic()` and to rust/sim's `items::COLS`.
mod ic {
    pub const PX: f64 = 0.0;
    pub const PY: f64 = 1.0;
    pub const PZ: f64 = 2.0;
    pub const VX: f64 = 3.0;
    pub const VY: f64 = 4.0;
    pub const VZ: f64 = 5.0;
    pub const YAW: f64 = 6.0;
    pub const SPIN: f64 = 7.0;
    pub const TUMBLE: f64 = 8.0;
    pub const TUMBLE_RATE: f64 = 9.0;
    pub const AGE: f64 = 10.0;
    pub const LIFE: f64 = 11.0;
    pub const RADIUS: f64 = 12.0;
    pub const RESTITUTION: f64 = 13.0;
    pub const FRICTION: f64 = 14.0;
    pub const HEADING: f64 = 15.0;
    pub const ON_POSE: f64 = 16.0;
    pub const FUSE: f64 = 17.0;
    pub const EMIT_T: f64 = 18.0;
    pub const FILL: f64 = 19.0;
    pub const FLAGS: f64 = 20.0;
    pub const KIND: f64 = 21.0;
    pub const BOUNCES: f64 = 22.0;
    pub const WHO: f64 = 23.0;
    pub const ON: f64 = 24.0;
    pub const SERIAL: f64 = 25.0;
}

/// A kind by its place in the `kind` column (sim/itemshome.ts ITEM_KINDS; `crate::sim_itemshome::item_kinds()` is the same list as nine Strings a call, and a test
/// holds this to it). NOT the order of the `ItemKind` enum: never `as u32`.
const ITEM_KINDS: [ItemKind; 9] = [
    ItemKind::Smoke,
    ItemKind::Mag,
    ItemKind::Flash,
    ItemKind::Frag,
    ItemKind::Pistol,
    ItemKind::Torch,
    ItemKind::Rifle,
    ItemKind::Ring,
    ItemKind::Shell,
];

/// a double column of row `r` by its place (ItemStore: `d[b8 + at[c] + r]`)
fn column_f(c: &mut sim::items::Items, at: usize) -> &mut [f64; sim::items::CAP] {
    match at {
        0 => &mut c.px,
        1 => &mut c.py,
        2 => &mut c.pz,
        3 => &mut c.vx,
        4 => &mut c.vy,
        5 => &mut c.vz,
        6 => &mut c.yaw,
        7 => &mut c.spin,
        8 => &mut c.tumble,
        9 => &mut c.tumble_rate,
        10 => &mut c.age,
        11 => &mut c.life,
        12 => &mut c.radius,
        13 => &mut c.restitution,
        14 => &mut c.friction,
        15 => &mut c.heading,
        16 => &mut c.on_pose,
        17 => &mut c.fuse,
        18 => &mut c.emit_t,
        19 => &mut c.fill,
        _ => unreachable!("no double column at this place"),
    }
}

/// a word of row `r` by its place, as the TypeScript's Uint32Array reads it (`who` is an i32 column there: the same bits)
fn word_of(c: &sim::items::Items, at: usize, r: usize) -> u32 {
    match at {
        20 => c.flags[r],
        21 => c.kind[r],
        22 => c.bounces[r],
        23 => c.who[r] as u32,
        24 => c.on[r],
        25 => c.serial[r],
        _ => unreachable!("no word column at this place"),
    }
}

/// …and into it
fn set_word(c: &mut sim::items::Items, at: usize, r: usize, v: u32) {
    match at {
        20 => c.flags[r] = v,
        21 => c.kind[r] = v,
        22 => c.bounces[r] = v,
        23 => c.who[r] = v as i32,
        24 => c.on[r] = v,
        25 => c.serial[r] = v,
        _ => unreachable!("no word column at this place"),
    }
}

/// The sponge round's three boxes about `c_2`, its length along `rot`'s z, the nose toward +z. (throwables.ts spongeBoxes)
pub fn sponge_boxes(out: &mut Vec<BoxId>, c_2: V3, rot: Quat, yaw: f64) {
    let s = sponge_item();
    let flag = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    for (z, h, albedo) in [(s.body.z, s.body.h, s.body.albedo), (s.nose.z, s.nose.h, s.nose.albedo), (s.band.z, s.band.h, s.band.albedo)] {
        out.push(item_box(v3::add(c_2, v3::quat::rotate(rot, [0.0, 0.0, z])), h, yaw, rot, albedo, flags));
    }
}

/// A handle on row `r` of the pool's columns (the store is `g.items.store`). (throwables.ts Item.constructor)
pub fn item_new(g: &mut crate::game::Game, r: f64, things: &[PropId]) -> ItemId {
    g.tab.item.push(Item { r, kept: None, side: None, on_bounce: None, clatters: None, hand: None, things: things.to_vec() })
}

/// Which row it stands in, -1 out of the pool. (throwables.ts Item.row)
pub fn item_row(g: &mut crate::game::Game, id: ItemId) -> f64 {
    if g.tab.item[id].kept.is_none() { g.tab.item[id].r } else { -1.0 }
}

/// Out of the pool, with its numbers. (throwables.ts Item.leave)
pub fn item_leave(g: &mut crate::game::Game, id: ItemId) {
    if g.tab.item[id].kept.is_none() {
        let r = g.tab.item[id].r;
        let kept = crate::sim_itemshome::item_store_row_out(g, r);
        g.tab.item[id].kept = Some(kept);
        g.tab.item[id].r = -1.0;
    }
}

/// Its numbers as they stand, by their places. (throwables.ts Item.numbers)
pub fn item_numbers(g: &mut crate::game::Game, id: ItemId) -> Vec<f64> {
    if let Some(k) = &g.tab.item[id].kept {
        return k.clone();
    }
    let r = g.tab.item[id].r;
    crate::sim_itemshome::item_store_row_out(g, r)
}

/// Into row `r`. (throwables.ts Item.stand)
pub fn item_stand(g: &mut crate::game::Game, id: ItemId, r: f64) {
    g.tab.item[id].kept = None;
    g.tab.item[id].r = r;
}

/// A double by its place. (throwables.ts Item.f)
pub fn item_f(g: &mut crate::game::Game, id: ItemId, c_2: f64) -> f64 {
    if let Some(k) = &g.tab.item[id].kept {
        return k[c_2 as usize];
    }
    let r = g.tab.item[id].r as usize;
    column_f(&mut sim::state::state(g.sim).items, c_2 as usize)[r]
}

/// …and written. (throwables.ts Item.pf)
pub fn item_pf(g: &mut crate::game::Game, id: ItemId, c_2: f64, v: f64) {
    if let Some(k) = &mut g.tab.item[id].kept {
        k[c_2 as usize] = v;
        return;
    }
    let r = g.tab.item[id].r as usize;
    column_f(&mut sim::state::state(g.sim).items, c_2 as usize)[r] = v;
}

/// A word by its place. (throwables.ts Item.w)
pub fn item_w(g: &mut crate::game::Game, id: ItemId, c_2: f64) -> f64 {
    if let Some(k) = &g.tab.item[id].kept {
        return k[c_2 as usize];
    }
    let r = g.tab.item[id].r as usize;
    word_of(&sim::state::state(g.sim).items, c_2 as usize, r) as f64
}

/// …and written: ToUint32 both ways, by `>>> 0` kept and by the Uint32Array's store in a row. (throwables.ts Item.pw)
pub fn item_pw(g: &mut crate::game::Game, id: ItemId, c_2: f64, v: f64) {
    if let Some(k) = &mut g.tab.item[id].kept {
        k[c_2 as usize] = js::to_u32(v) as f64;
        return;
    }
    let r = g.tab.item[id].r as usize;
    set_word(&mut sim::state::state(g.sim).items, c_2 as usize, r, js::to_u32(v));
}

/// A truth: a bit of `flags`. (throwables.ts Item.bit)
pub fn item_bit(g: &mut crate::game::Game, id: ItemId, b: f64) -> bool {
    (js::to_i32(item_w(g, id, ic::FLAGS)) & js::to_i32(b)) != 0
}

/// …and written. (throwables.ts Item.pbit)
pub fn item_pbit(g: &mut crate::game::Game, id: ItemId, b: f64, on: bool) {
    let f = js::to_i32(item_w(g, id, ic::FLAGS));
    let b = js::to_i32(b);
    item_pw(g, id, ic::FLAGS, (if on { f | b } else { f & !b }) as f64);
}

/// (throwables.ts Item.kind)
pub fn item_kind(g: &mut crate::game::Game, id: ItemId) -> crate::throwables::ItemKind {
    ITEM_KINDS[item_w(g, id, ic::KIND) as usize]
}

/// (throwables.ts Item.kind, set)
pub fn item_set_kind(g: &mut crate::game::Game, id: ItemId, v: crate::throwables::ItemKind) {
    let at = ITEM_KINDS.iter().position(|k| *k == v).unwrap();
    item_pw(g, id, ic::KIND, at as f64);
}

/// Given as a new vector. (throwables.ts Item.pos)
pub fn item_pos(g: &mut crate::game::Game, id: ItemId) -> V3 {
    [item_f(g, id, ic::PX), item_f(g, id, ic::PY), item_f(g, id, ic::PZ)]
}

/// Taken whole. (throwables.ts Item.pos, set)
pub fn item_set_pos(g: &mut crate::game::Game, id: ItemId, v: V3) {
    item_pf(g, id, ic::PX, v[0]);
    item_pf(g, id, ic::PY, v[1]);
    item_pf(g, id, ic::PZ, v[2]);
}

/// Given as a new vector. (throwables.ts Item.vel)
pub fn item_vel(g: &mut crate::game::Game, id: ItemId) -> V3 {
    [item_f(g, id, ic::VX), item_f(g, id, ic::VY), item_f(g, id, ic::VZ)]
}

/// Taken whole. (throwables.ts Item.vel, set)
pub fn item_set_vel(g: &mut crate::game::Game, id: ItemId, v: V3) {
    item_pf(g, id, ic::VX, v[0]);
    item_pf(g, id, ic::VY, v[1]);
    item_pf(g, id, ic::VZ, v[2]);
}

/// (throwables.ts Item.radius)
pub fn item_radius(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::RADIUS)
}

/// (throwables.ts Item.radius, set)
pub fn item_set_radius(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::RADIUS, v);
}

/// (throwables.ts Item.restitution)
pub fn item_restitution(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::RESTITUTION)
}

/// (throwables.ts Item.restitution, set)
pub fn item_set_restitution(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::RESTITUTION, v);
}

/// (throwables.ts Item.friction)
pub fn item_friction(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::FRICTION)
}

/// (throwables.ts Item.friction, set)
pub fn item_set_friction(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::FRICTION, v);
}

/// (throwables.ts Item.yaw)
pub fn item_yaw(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::YAW)
}

/// (throwables.ts Item.yaw, set)
pub fn item_set_yaw(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::YAW, v);
}

/// (throwables.ts Item.spin)
pub fn item_spin(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::SPIN)
}

/// (throwables.ts Item.spin, set)
pub fn item_set_spin(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::SPIN, v);
}

/// (throwables.ts Item.tumble)
pub fn item_tumble(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::TUMBLE)
}

/// (throwables.ts Item.tumble, set)
pub fn item_set_tumble(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::TUMBLE, v);
}

/// (throwables.ts Item.tumbleRate)
pub fn item_tumble_rate(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::TUMBLE_RATE)
}

/// (throwables.ts Item.tumbleRate, set)
pub fn item_set_tumble_rate(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::TUMBLE_RATE, v);
}

/// (throwables.ts Item.age)
pub fn item_age(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::AGE)
}

/// (throwables.ts Item.age, set)
pub fn item_set_age(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::AGE, v);
}

/// (throwables.ts Item.life)
pub fn item_life(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::LIFE)
}

/// (throwables.ts Item.life, set)
pub fn item_set_life(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::LIFE, v);
}

/// (throwables.ts Item.fuse)
pub fn item_fuse(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::FUSE)
}

/// (throwables.ts Item.fuse, set)
pub fn item_set_fuse(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::FUSE, v);
}

/// (throwables.ts Item.emitT)
pub fn item_emit_t(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::EMIT_T)
}

/// (throwables.ts Item.emitT, set)
pub fn item_set_emit_t(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::EMIT_T, v);
}

/// (throwables.ts Item.fill)
pub fn item_fill(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::FILL)
}

/// (throwables.ts Item.fill, set)
pub fn item_set_fill(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::FILL, v);
}

/// (throwables.ts Item.heading)
pub fn item_heading(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::HEADING)
}

/// (throwables.ts Item.heading, set)
pub fn item_set_heading(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::HEADING, v);
}

/// (throwables.ts Item.onPose)
pub fn item_on_pose(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_f(g, id, ic::ON_POSE)
}

/// (throwables.ts Item.onPose, set)
pub fn item_set_on_pose(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pf(g, id, ic::ON_POSE, v);
}

/// (throwables.ts Item.settled)
pub fn item_settled(g: &mut crate::game::Game, id: ItemId) -> bool {
    item_bit(g, id, crate::sim_itemshome::itf().settled)
}

/// (throwables.ts Item.settled, set)
pub fn item_set_settled(g: &mut crate::game::Game, id: ItemId, v: bool) {
    item_pbit(g, id, crate::sim_itemshome::itf().settled, v);
}

/// (throwables.ts Item.fired)
pub fn item_fired(g: &mut crate::game::Game, id: ItemId) -> bool {
    item_bit(g, id, crate::sim_itemshome::itf().fired)
}

/// (throwables.ts Item.fired, set)
pub fn item_set_fired(g: &mut crate::game::Game, id: ItemId, v: bool) {
    item_pbit(g, id, crate::sim_itemshome::itf().fired, v);
}

/// As it was last set (unset is not false to a save file); the step reads its bit. (throwables.ts Item.carried)
pub fn item_carried(g: &mut crate::game::Game, id: ItemId) -> Option<bool> {
    g.tab.item[id].hand
}

/// (throwables.ts Item.carried, set)
pub fn item_set_carried(g: &mut crate::game::Game, id: ItemId, v: Option<bool>) {
    g.tab.item[id].hand = v;
    item_pbit(g, id, crate::sim_itemshome::itf().carried, v == Some(true));
}

/// (throwables.ts Item.bounces)
pub fn item_bounces(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_w(g, id, ic::BOUNCES)
}

/// (throwables.ts Item.bounces, set)
pub fn item_set_bounces(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pw(g, id, ic::BOUNCES, v);
}

/// Who threw or dropped it, an i32. (throwables.ts Item.who)
pub fn item_who(g: &mut crate::game::Game, id: ItemId) -> f64 {
    js::to_i32(item_w(g, id, ic::WHO)) as f64
}

/// (throwables.ts Item.who, set)
pub fn item_set_who(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pw(g, id, ic::WHO, v);
}

/// (throwables.ts Item.serial)
pub fn item_serial(g: &mut crate::game::Game, id: ItemId) -> f64 {
    item_w(g, id, ic::SERIAL)
}

/// (throwables.ts Item.serial, set)
pub fn item_set_serial(g: &mut crate::game::Game, id: ItemId, v: f64) {
    item_pw(g, id, ic::SERIAL, v);
}

/// The thing of furniture it lies on: the column holds its id plus one. (throwables.ts Item.on)
pub fn item_on(g: &mut crate::game::Game, id: ItemId) -> Option<PropId> {
    let k = item_w(g, id, ic::ON);
    if js::truthy(k) { g.tab.item[id].things.get(k as usize - 1).copied() } else { None }
}

/// (throwables.ts Item.on, set)
pub fn item_set_on(g: &mut crate::game::Game, id: ItemId, p: Option<PropId>) {
    let v = match p {
        Some(p) => g.tab.props_prop[p].id + 1.0,
        None => 0.0,
    };
    item_pw(g, id, ic::ON, v);
}

/// A box for an item's part. (throwables.ts itemBox)
pub fn item_box(c_2: V3, h: V3, yaw: f64, rot: Quat, albedo: V3, flags: f64) -> BoxId {
    crate::scene_boxes::box_of(c_2, h, yaw, albedo, [0.0, 0.0, 0.0], flags, 0.0, Some(rot), None)
}

/// The share of its slide a thing keeps on lying snow. (throwables.ts bogSlide)
pub fn bog_slide(g: &mut crate::game::Game, bog: f64) -> f64 {
    1.0 - bog * g.k.snowbog.grip
}

/// The share of its bounce. (throwables.ts bogBounce)
pub fn bog_bounce(g: &mut crate::game::Game, bog: f64) -> f64 {
    1.0 - bog * g.k.snowbog.dead
}

/// How much faster than on a floor it may still be going and stop dead. (throwables.ts bogStick)
pub fn bog_stick(g: &mut crate::game::Game, bog: f64) -> f64 {
    bog * g.k.snowbog.stick
}

#[cfg(test)]
mod tests {
    use super::*;

    /// every place of `ic` is the column at that place of rust/sim's own list (`items::COLS`, byte offsets into `Items`)
    #[test]
    fn places_are_rust_sims_columns() {
        let mut c: Box<sim::items::Items> = unsafe { Box::new(core::mem::zeroed()) };
        for at in 0..20 {
            column_f(&mut c, at)[3] = 1.5 + at as f64;
            let base = &*c as *const sim::items::Items as *const u8;
            let read = unsafe { *(base.add(sim::items::COLS[at]) as *const f64).add(3) };
            assert_eq!(read.to_bits(), (1.5 + at as f64).to_bits(), "double {at}");
        }
        for at in 20..26 {
            set_word(&mut c, at, 3, 7 + at as u32);
            let base = &*c as *const sim::items::Items as *const u8;
            let read = unsafe { *(base.add(sim::items::COLS[at]) as *const u32).add(3) };
            assert_eq!(read, 7 + at as u32, "word {at}");
            assert_eq!(word_of(&c, at, 3), 7 + at as u32);
        }
    }

    /// every place named here is the place `sim_itemshome::ic()` gives that column
    #[test]
    fn places_are_the_skeletons_ic() {
        let table = crate::sim_itemshome::ic();
        let named: [(&str, f64); 26] = [
            ("px", ic::PX), ("py", ic::PY), ("pz", ic::PZ), ("vx", ic::VX), ("vy", ic::VY), ("vz", ic::VZ), ("yaw", ic::YAW), ("spin", ic::SPIN), ("tumble", ic::TUMBLE),
            ("tumbleRate", ic::TUMBLE_RATE), ("age", ic::AGE), ("life", ic::LIFE), ("radius", ic::RADIUS), ("restitution", ic::RESTITUTION), ("friction", ic::FRICTION),
            ("heading", ic::HEADING), ("onPose", ic::ON_POSE), ("fuse", ic::FUSE), ("emitT", ic::EMIT_T), ("fill", ic::FILL), ("flags", ic::FLAGS), ("kind", ic::KIND),
            ("bounces", ic::BOUNCES), ("who", ic::WHO), ("on", ic::ON), ("serial", ic::SERIAL),
        ];
        for (name, place) in named {
            assert_eq!(table.get(String::from(name)).copied(), Some(place), "{name}");
        }
        assert_eq!((crate::sim_itemshome::NF, crate::sim_itemshome::NC), (20.0, 26.0));
    }

    /// the three kinds rust/sim names sit where its constants say: the column's order, not the enum's; and the whole list is the skeleton's
    #[test]
    fn kinds_by_their_place() {
        let words = crate::sim_itemshome::item_kinds();
        for at in 0..9 {
            assert_eq!(ITEM_KINDS[at].as_str(), words[at].as_str());
        }
        assert_eq!(ITEM_KINDS[sim::items::K_SMOKE as usize], ItemKind::Smoke);
        assert_eq!(ITEM_KINDS[sim::items::K_FRAG as usize], ItemKind::Frag);
        assert_eq!(ITEM_KINDS[sim::items::K_RIFLE as usize], ItemKind::Rifle);
    }

    /// `who` is an i32 in rust/sim and a Uint32Array word in the TypeScript: -1 goes through as the same bits
    #[test]
    fn who_keeps_its_sign() {
        let mut c: Box<sim::items::Items> = unsafe { Box::new(core::mem::zeroed()) };
        set_word(&mut c, 23, 0, js::to_u32(-1.0));
        assert_eq!(c.who[0], -1);
        assert_eq!(word_of(&c, 23, 0) as f64, 4294967295.0);
        assert_eq!(js::to_i32(word_of(&c, 23, 0) as f64) as f64, -1.0);
    }

    /// a game with one handle; `sim` picks the block's slot
    fn one_item(things: &[PropId]) -> (Box<Game>, ItemId) {
        let mut g = Box::new(Game::blank());
        let id = item_new(&mut g, 2.0, things);
        (g, id)
    }

    /// out of the pool: reads and writes its own numbers, by the same places, `>>> 0` on a word
    #[test]
    fn a_kept_item_answers_from_its_numbers() {
        let (mut g, id) = one_item(&[]);
        assert_eq!(item_row(&mut g, id), 2.0);
        g.tab.item[id].kept = Some(vec![0.0; 26]);
        g.tab.item[id].r = -1.0;
        assert_eq!(item_row(&mut g, id), -1.0);
        item_set_pos(&mut g, id, [1.5, 2.5, 3.5]);
        assert_eq!(item_pos(&mut g, id), [1.5, 2.5, 3.5]);
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[2], 3.5);
        item_set_who(&mut g, id, -1.0);
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::WHO as usize], 4294967295.0);
        assert_eq!(item_who(&mut g, id), -1.0);
        item_set_kind(&mut g, id, ItemKind::Frag);
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::KIND as usize], 3.0);
        assert_eq!(item_kind(&mut g, id), ItemKind::Frag);
        assert_eq!(item_numbers(&mut g, id), *g.tab.item[id].kept.as_ref().unwrap());
        item_stand(&mut g, id, 5.0);
        assert_eq!((item_row(&mut g, id), g.tab.item[id].kept.is_none()), (5.0, true));
    }

    /// the truths are bits of `flags` and leave each other alone, the top bit of the word too (`~b` as an i32, stored `>>> 0`)
    #[test]
    fn truths_are_bits_of_flags() {
        let (mut g, id) = one_item(&[]);
        g.tab.item[id].kept = Some(vec![0.0; 26]);
        g.tab.item[id].kept.as_mut().unwrap()[ic::FLAGS as usize] = 2147483648.0;
        item_set_settled(&mut g, id, true);
        item_set_fired(&mut g, id, true);
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::FLAGS as usize], 2147483651.0);
        item_set_settled(&mut g, id, false);
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::FLAGS as usize], 2147483650.0);
        assert_eq!((item_settled(&mut g, id), item_fired(&mut g, id)), (false, true));
    }

    /// carried tells unset from false to its reader; only true sets the bit
    #[test]
    fn carried_keeps_unset_apart_from_false() {
        let (mut g, id) = one_item(&[]);
        g.tab.item[id].kept = Some(vec![0.0; 26]);
        assert_eq!(item_carried(&mut g, id), None);
        item_set_carried(&mut g, id, Some(false));
        assert_eq!(item_carried(&mut g, id), Some(false));
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::FLAGS as usize], 0.0);
        item_set_carried(&mut g, id, Some(true));
        assert_eq!(g.tab.item[id].kept.as_ref().unwrap()[ic::FLAGS as usize], 4.0);
        item_set_carried(&mut g, id, None);
        assert_eq!((item_carried(&mut g, id), g.tab.item[id].kept.as_ref().unwrap()[ic::FLAGS as usize]), (None, 0.0));
    }

    /// `on` holds a thing's id plus one: 0 is none, and past the list is none
    #[test]
    fn on_is_an_id_plus_one() {
        let (mut g, id) = one_item(&[PropId(7), PropId(9)]);
        g.tab.item[id].kept = Some(vec![0.0; 26]);
        assert_eq!(item_on(&mut g, id), None);
        g.tab.item[id].kept.as_mut().unwrap()[ic::ON as usize] = 2.0;
        assert_eq!(item_on(&mut g, id), Some(PropId(9)));
        g.tab.item[id].kept.as_mut().unwrap()[ic::ON as usize] = 3.0;
        assert_eq!(item_on(&mut g, id), None);
    }

    /// in the pool: the same getters and setters reach the game's block, row by row
    #[test]
    fn a_row_in_the_pool_reads_the_block() {
        let (mut g, id) = one_item(&[]);
        g.sim = 0;
        item_set_vel(&mut g, id, [0.25, -0.5, 0.75]);
        item_set_who(&mut g, id, -1.0);
        item_set_kind(&mut g, id, ItemKind::Rifle);
        item_set_fired(&mut g, id, true);
        let c = &sim::state::state(0).items;
        assert_eq!((c.vx[2], c.vy[2], c.vz[2], c.who[2], c.kind[2], c.flags[2]), (0.25, -0.5, 0.75, -1, sim::items::K_RIFLE, 2));
        assert_eq!(item_vel(&mut g, id), [0.25, -0.5, 0.75]);
        assert_eq!(item_w(&mut g, id, ic::WHO), 4294967295.0);
        assert_eq!(item_who(&mut g, id), -1.0);
        assert_eq!(item_kind(&mut g, id), ItemKind::Rifle);
        assert!(item_fired(&mut g, id));
    }
}
