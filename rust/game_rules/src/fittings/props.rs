pub use ::game_types::props::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the furniture's part of this game's block: the columns are the state (doors-kit.md)
#[inline]
fn cols(g: &Game) -> &'static mut sim::props::Props {
    &mut sim::state::state(g.sim).props
}
/// the same part, to read
#[inline]
fn cols_read(g: &Game) -> &'static sim::props::Props {
    &sim::state::state_read(g.sim).props
}
/// where thing `id` is now
#[inline]
fn row(s: &sim::props::Props, id: PropId) -> usize {
    s.row[id.0 as usize] as usize
}
/// a truth of a row written
#[inline]
fn set_flag(s: &mut sim::props::Props, r: usize, mask: u32, v: bool) {
    if v { s.flags[r] |= mask; } else { s.flags[r] &= !mask; }
}
/// the step's hand-over: `def` is the first field of a repr(C) struct, so the address of table 0 is the struct's. Scratch of the module, no part of `g`.
#[inline]
fn io() -> &'static mut sim::props::Io {
    unsafe { &mut *(sim::props::addr(0) as *mut sim::props::Io) }
}
/// the same hand-over, to read
fn io_read() -> &'static sim::props::Io {
    unsafe { &*(sim::props::addr(0) as *const sim::props::Io) }
}
/// the one target of the TypeScript's stored `emit`, called by name
#[inline]
fn emit(g: &mut Game, e: PropEvent) {
    crate::game::game_on_prop_event(g, e);
}
/// the speed caps, on a row of the columns
fn clamp_row(c: &mut sim::props::Props, r: usize) {
    let s = m::hyp2(c.vx[r], c.vz[r]);
    if s > MAX_V {
        c.vx[r] *= MAX_V / s;
        c.vz[r] *= MAX_V / s;
    }
    if c.w[r] > MAX_W { c.w[r] = MAX_W; } else if c.w[r] < -MAX_W { c.w[r] = -MAX_W; }
}

/// a row's yaw written, with its cos and sin when it is another value
fn turn_row(c: &mut sim::props::Props, r: usize, y: f64) {
    if y != c.yaw[r] {
        c.cs[r] = m::cos(y);
        c.sn[r] = m::sin(y);
    }
    c.yaw[r] = y;
}

/// A box's row (`Box.c`, `Box.yaw`, scene/boxes.ts:66). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows: once the table is back the body is that row
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

pub fn prop_x(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.x[row(s, id)]
}

/// where a thing stands, written by anybody but the step, is said
pub fn prop_set_x(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.x[r] = v;
    crate::scene_box_mirror::thing_wrote(g);
}

pub fn prop_z(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.z[row(s, id)]
}

pub fn prop_set_z(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.z[r] = v;
    crate::scene_box_mirror::thing_wrote(g);
}

pub fn prop_yaw(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.yaw[row(s, id)]
}

/// its yaw, with the cos and sin of it when it is another value
pub fn prop_set_yaw(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    turn_row(s, r, v);
    crate::scene_box_mirror::thing_wrote(g);
}

pub(in crate::fittings) fn prop_vx(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.vx[row(s, id)]
}

pub fn prop_set_vx(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.vx[r] = v;
}

pub(in crate::fittings) fn prop_vz(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.vz[row(s, id)]
}

pub fn prop_set_vz(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.vz[r] = v;
}

pub fn prop_w(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.w[row(s, id)]
}

pub fn prop_set_w(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.w[r] = v;
}

pub fn prop_asleep(g: &crate::game::Game, id: PropId) -> bool {
    let s = cols_read(g);
    s.flags[row(s, id)] & sim::props::F_ASLEEP != 0
}

pub(in crate::fittings) fn prop_set_asleep(g: &mut crate::game::Game, id: PropId, v: bool) {
    let s = cols(g);
    let r = row(s, id);
    set_flag(s, r, sim::props::F_ASLEEP, v);
}

pub(in crate::fittings) fn prop_set_idle(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.idle[r] = v;
}

pub fn prop_set_dirty(g: &mut crate::game::Game, id: PropId, v: bool) {
    let s = cols(g);
    let r = row(s, id);
    set_flag(s, r, sim::props::F_DIRTY, v);
}

/// coasting on its castors
pub fn prop_rolling(g: &crate::game::Game, id: PropId) -> bool {
    let s = cols_read(g);
    s.flags[row(s, id)] & sim::props::F_ROLLING != 0
}

pub(in crate::fittings) fn prop_set_rolling(g: &mut crate::game::Game, id: PropId, v: bool) {
    let s = cols(g);
    let r = row(s, id);
    set_flag(s, r, sim::props::F_ROLLING, v);
}

/// who moved it last, blamed for its noise
pub fn prop_last_who(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.who[row(s, id)] as f64
}

pub fn prop_set_last_who(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.who[r] = js::to_i32(v);
}

/// last event time, the noise's rate limit
pub(in crate::fittings) fn prop_noise_t(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.noise_t[row(s, id)]
}

pub(in crate::fittings) fn prop_set_noise_t(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.noise_t[r] = v;
}

/// last time a man or a leaf was pushing it
pub fn prop_push_t(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.push_t[row(s, id)]
}

pub(in crate::fittings) fn prop_set_push_t(g: &mut crate::game::Game, id: PropId, v: f64) {
    let s = cols(g);
    let r = row(s, id);
    s.push_t[r] = v;
}

/// a thing of furniture made: its row written, its facts, its part boxes, placed
pub(in crate::fittings) fn prop_new(g: &mut crate::game::Game, def: &crate::scene_level::PropDef, id_2: f64) -> PropId {
    let id = PropId(id_2 as u32);
    prop_set_x(g, id, def.x);
    prop_set_z(g, id, def.z);
    prop_set_yaw(g, id, def.yaw);
    let home = [def.x, def.z, def.yaw];
    {
        // (of the yaw it is made at, whatever that is: a yaw of −0 equals 0 and would keep the cos and sin of +0)
        let c = cols(g);
        let r = row(c, id);
        c.cs[r] = m::cos(def.yaw);
        c.sn[r] = m::sin(def.yaw);
    }
    let (hx, hz) = (def.half[0], def.half[1]);
    let radius = m::hyp2(hx, hz);
    let inv_i = 1.0 / (def.mass * (hx * hx + hz * hz) / 3.0 * 1.5);
    let mut boxes: Vec<BoxId> = Vec::with_capacity(def.parts.len());
    for (j, p) in def.parts.iter().enumerate() {
        let marks = match &p.marks {
            Some(k) if !k.is_empty() => Some(k.clone()),
            _ => None,
        };
        boxes.push(crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: [0.0, 0.0, 0.0],
            h: [p.h[0], p.h[1], p.h[2]],
            yaw: Some(0.0),
            albedo: Some(p.albedo),
            flags: Some(crate::scene_boxes::box_flag().dynamic),
            name: Some(def.name.clone()),
            mkey: Some(crate::scene_boxes::mkey().prop + id_2 * 16.0 + j as f64),
            marks,
            stuff: p.stuff.clone(),
            does: p.does.clone(),
            ..Default::default()
        }));
    }
    let got = g.tab.props_prop.push(Prop {
        hx,
        hz,
        radius,
        height: def.height,
        mass: def.mass,
        inv_mass: 1.0 / def.mass,
        inv_i,
        tune: *kind().get(String::from(def.kind.as_str())).expect("a kind of furniture KIND has no row for"),
        home,
        boxes,
        def: def.clone(),
        id: id_2,
    });
    debug_assert!(got == id, "a thing's id is its row of the table: one PropSystem a game");
    prop_place(g, id);
    id
}

pub fn prop_cs(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.cs[row(s, id)]
}

pub fn prop_sn(g: &crate::game::Game, id: PropId) -> f64 {
    let s = cols_read(g);
    s.sn[row(s, id)]
}

/// the part boxes put where the pose says, every part from the same five numbers
pub fn prop_place(g: &mut crate::game::Game, id: PropId) {
    let (x, z, yaw, cs, sn) = {
        let s = cols_read(g);
        let r = row(s, id);
        (s.x[r], s.z[r], s.yaw[r], s.cs[r], s.sn[r])
    };
    let n = g.tab.props_prop[id].def.parts.len();
    for i in 0..n {
        let (off, part_yaw, b) = {
            let t = &g.tab.props_prop[id];
            (t.def.parts[i].off, t.def.parts[i].yaw, t.boxes[i])
        };
        let c = [x + cs * off[0] + sn * off[2], off[1], z - sn * off[0] + cs * off[2]];
        let bx = seam_box(g, b);
        bx.c = c;
        bx.yaw = yaw + part_yaw;
    }
    prop_set_dirty(g, id, false);
    crate::scene_box_mirror::thing_wrote(g);
}

pub fn prop_wake(g: &mut crate::game::Game, id: PropId) {
    prop_set_asleep(g, id, false);
    prop_set_idle(g, id, 0.0);
}

/// the system's clock, a number of the block
pub(in crate::fittings) fn prop_system_time(g: &crate::game::Game) -> f64 {
    cols_read(g).time
}

/// a noise: held while a step is being taken, heard at once otherwise
pub(in crate::fittings) fn prop_system_say(g: &mut crate::game::Game, e: crate::props::PropEvent) {
    if g.props.stepping {
        g.props.said.push(e);
    } else {
        emit(g, e);
    }
}

/// the furniture of a level made and settled. RETURNS the system; for the settle, which reaches it by `g.props` as every method does, it stands in
/// `g.props` and the old value is put back, so the caller's `g.props = …` is what keeps it
pub fn prop_system_new(g: &mut crate::game::Game, defs: &[crate::scene_level::PropDef], st: Option<crate::Opaque>) -> crate::props::PropSystem {
    let mut sys = PropSystem::blank();
    sys.store = crate::sim_propshome::prop_store_new(g, st.unwrap_or(crate::Opaque), defs.len() as f64);
    for (i, d) in defs.iter().enumerate() {
        let id = prop_new(g, d, i as f64);
        sys.all.push(id);
    }
    sys.props = sys.all.clone();
    for &p in &sys.props {
        for &b in &g.tab.props_prop[p].boxes {
            sys.by_box.set(b, p);
        }
    }
    let old = core::mem::replace(&mut g.props, sys);
    prop_system_settle(g);
    core::mem::replace(&mut g.props, old)
}

/// every part box of what is in the level, in its order
pub fn prop_system_boxes(g: &crate::game::Game, out: &mut Vec<AnyBoxId>) {
    for &p in &g.props.props {
        out.extend(g.tab.props_prop[p].boxes.iter().map(|&b| AnyBoxId::World(b)));
    }
}

/// a thing leaves the level: no longer drawn, solid, pushed or stepped until reset
pub fn prop_system_remove(g: &mut crate::game::Game, p: PropId) {
    if let Some(i) = g.props.props.iter().position(|&q| q == p) {
        g.props.props.remove(i);
        crate::sim_propshome::prop_store_leave(g, p.0 as f64);
        crate::scene_box_mirror::thing_wrote(g);
    }
}

/// everything back to its rest pose, asleep
pub fn prop_system_reset(g: &mut crate::game::Game) {
    if g.props.props.len() != g.props.all.len() {
        g.props.props.clear();
        g.props.props.extend_from_slice(&g.props.all);
        crate::sim_propshome::prop_store_home_rows(g);
        crate::scene_box_mirror::thing_wrote(g);
    }
    for i in 0..g.props.props.len() {
        let p = g.props.props[i];
        let home = g.tab.props_prop[p].home;
        prop_set_x(g, p, home[0]);
        prop_set_z(g, p, home[1]);
        prop_set_yaw(g, p, home[2]);
        prop_set_w(g, p, 0.0);
        prop_set_vz(g, p, 0.0);
        prop_set_vx(g, p, 0.0);
        prop_set_asleep(g, p, true);
        prop_set_idle(g, p, 0.0);
        prop_set_push_t(g, p, -10.0);
        prop_set_noise_t(g, p, -10.0);
        prop_set_last_who(g, p, 0.0);
        prop_set_rolling(g, p, false);
        prop_set_dirty(g, p, true);
        prop_place(g, p);
    }
}

/// authoring overlaps resolved once, silently; the settled pose becomes home
pub(in crate::fittings) fn prop_system_settle(g: &mut crate::game::Game) {
    g.props.muted = true;
    for i in 0..g.props.props.len() {
        let p = g.props.props[i];
        prop_set_dirty(g, p, true);
    }
    if !g.props.props.is_empty() {
        crate::scene_collision::window(g, &mut |g: &mut Game| {
            prop_system_settle_world(g);
            prop_system_settle_there(g);
        });
    }
    for i in 0..g.props.props.len() {
        let p = g.props.props[i];
        prop_set_w(g, p, 0.0);
        prop_set_vz(g, p, 0.0);
        prop_set_vx(g, p, 0.0);
        prop_set_asleep(g, p, true);
        let home = [prop_x(g, p), prop_z(g, p), prop_yaw(g, p)];
        g.tab.props_prop[p].home = home;
        prop_place(g, p);
    }
    g.props.muted = false;
}

/// the box world's copy standing for the passes; in Rust the module always takes them
pub(in crate::fittings) fn prop_system_settle_world(g: &mut crate::game::Game) {
    crate::scene_collision::rules_statics(g);
    debug_assert!(crate::scene_collision::standing(g), "the furniture settled as a level stood up, with no copy of the box world standing");
}

/// the passes taken in the module: the level's facts by row, the call, and how often a thing was pushed out of a static
pub(in crate::fittings) fn prop_system_settle_there(g: &mut crate::game::Game) {
    prop_system_facts(g);
    io().par[sim::props::P_PROPS] = g.props.props.len() as f64;
    let walls = sim::props::settle(g.sim, g.col.mirror.slot as usize);
    assert!(!js::truthy(io_read().par[sim::props::P_WRONG]), "the furniture's settle in the Rust module could not be what it was handed (another count of things than its rows)");
    let c = &mut g.m.sim_propshome.props_stepped;
    c.settled += 1.0;
    c.settle_walls += walls as f64;
}

/// closest part of any thing along the ray (t < tmax): the module's ray, over the copy of the furniture
pub fn prop_system_raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::props::PropSystemRaycast> {
    if g.props.props.is_empty() {
        return None;
    }
    match crate::scene_collision::things_ray(g, ro, rd, tmax) {
        // (no copy of the furniture standing: the TypeScript's walk, which in Rust is the twin)
        None => prop_system_raycast_here(g, ro, rd, tmax).map(|h| PropSystemRaycast { t: h.t, n: h.n, prop: h.prop, box_: h.box_ }),
        Some(None) => None,
        Some(Some(h)) => Some(PropSystemRaycast { t: h.t, n: h.n, prop: g.props.props[h.row as usize], box_: h.box_ }),
    }
}

/// TWIN, with what its row and part name
pub fn prop_system_raycast_here(g: &crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::props::PropSystemRaycastHere> {
    let mut out = [0.0; sim::lines::RAY_PART + 1];
    let k = sim::lines::ray_things(g.col.mirror.slot as usize, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    if k < 0 {
        return None;
    }
    let box_ = thing_part(g, out[sim::lines::RAY_PART] as usize);
    Some(PropSystemRaycastHere { t: out[0], n: [out[1], out[2], out[3]], prop: g.props.props[k as usize], box_ })
}

/// which box a place in the furniture's list of parts is, the things in their order
fn thing_part(g: &Game, at: usize) -> BoxId {
    crate::scene_box_mirror::box_mirror_thing_box(g, at as f64).expect("the module's ray into the furniture met a part of its list that is no part of the furniture as it stands")
}

/// planar impulse J at a world point; `who` is blamed; thud reports it as an impact
pub fn prop_system_apply_impulse(g: &mut crate::game::Game, p: PropId, point: V3, j: V3, who: f64, thud: Option<bool>) {
    let thud = thud.unwrap_or(false);
    let rx = point[0] - prop_x(g, p);
    let rz = point[2] - prop_z(g, p);
    let (inv_mass, inv_i) = (g.tab.props_prop[p].inv_mass, g.tab.props_prop[p].inv_i);
    let vx = prop_vx(g, p) + j[0] * inv_mass;
    prop_set_vx(g, p, vx);
    let vz = prop_vz(g, p) + j[2] * inv_mass;
    prop_set_vz(g, p, vz);
    let w = prop_w(g, p) + (rz * j[0] - rx * j[2]) * inv_i;
    prop_set_w(g, p, w);
    prop_system_clamp_vel(g, p);
    prop_wake(g, p);
    if js::truthy(who) {
        prop_set_last_who(g, p, who);
    }
    if thud {
        let by = js::or(who, prop_last_who(g, p));
        prop_system_noise(g, p, PropSystemNoiseSound::Thud, m::hyp2(j[0], j[2]) * inv_mass + 1.0, by, 0.55);
    }
}

/// the speed caps on this thing's row
pub(in crate::fittings) fn prop_system_clamp_vel(g: &mut crate::game::Game, p: PropId) {
    let c = cols(g);
    let r = row(c, p);
    clamp_row(c, r);
}

/// a noise, rate limited by the thing: a chair along a wall is one scrape and the odd knock
pub(in crate::fittings) fn prop_system_noise(g: &mut crate::game::Game, p: PropId, sound: crate::props::PropSystemNoiseSound, speed: f64, who: f64, level: f64) {
    let time = prop_system_time(g);
    let gap = if sound == PropSystemNoiseSound::Thud { 0.35 } else { 0.8 };
    if g.props.muted || time - prop_noise_t(g, p) < gap {
        return;
    }
    prop_set_noise_t(g, p, time);
    let sound = match sound {
        PropSystemNoiseSound::Thud => PropEventSound::Thud,
        PropSystemNoiseSound::Scrape => PropEventSound::Scrape,
    };
    let e = PropEvent {
        prop: p,
        sound,
        pos: [prop_x(g, p), m::min(0.6, g.tab.props_prop[p].height * 0.5), prop_z(g, p)],
        who,
        level: m::min(1.0, level),
        speed,
        rolling: prop_rolling(g, p),
        roll: None,
    };
    prop_system_say(g, e);
}

/// one tick of the furniture: the module's step, then the boxes of what it moved placed
pub fn prop_system_update(g: &mut crate::game::Game, dt: f64, contacts: &mut Vec<crate::props::PropContact>, leaves: &[BoxId]) {
    let n = g.props.props.len();
    {
        let c = cols_read(g);
        let mut ok = n == c.n as usize;
        let mut r = 0;
        while ok && r < n {
            ok = c.id[r] == g.props.props[r].0;
            r += 1;
        }
        assert!(ok, "the furniture's list is not its rows in their order: something wrote PropSystem.props that is not remove() or reset()");
    }
    if let Some(t) = prop_system_there(g, contacts, leaves) {
        let t = PropSystemHandOverT { leaves: t.leaves, n: t.n };
        let ev = prop_system_hand_over(g, dt, contacts, &t);
        prop_system_take_back(g, contacts, ev);
    }
    let mut awake = 0;
    for r in 0..n {
        let f = cols_read(g).flags[r];
        if f & sim::props::F_DIRTY != 0 {
            let p = g.props.props[r];
            prop_place(g, p);
        }
        if f & sim::props::F_ASLEEP == 0 {
            awake += 1;
        }
    }
    g.props.stats.awake = awake as f64;
}

/// the world to step over and which of its leaves sweep; none in a level with no furniture, which has no step
pub(in crate::fittings) fn prop_system_there(g: &mut crate::game::Game, contacts: &[crate::props::PropContact], leaves: &[BoxId]) -> Option<crate::props::PropSystemThere> {
    if g.props.all.is_empty() {
        return None;
    }
    debug_assert!(!g.props.muted, "the furniture was stepped while its level was still being settled");
    debug_assert!(
        contacts.len() <= sim::props::MAXC && leaves.len() <= sim::world::MAX_LEAVES && g.props.doorways.len() <= sim::props::MAXW,
        "the furniture's step was handed more men, leaves or door frames than its hand-over holds"
    );
    let dyn_ = g.col.dynamic_boxes.clone();
    let mut idx: Vec<f64> = Vec::with_capacity(leaves.len());
    for &b in leaves {
        let i = dyn_.iter().position(|&q| q == b);
        debug_assert!(i.is_some(), "a leaf handed in that is not in the box world");
        if let Some(i) = i {
            idx.push(i as f64);
        }
    }
    crate::scene_collision::rules_world(g, &dyn_);
    let n = idx.len() as f64;
    Some(PropSystemThere { leaves: Some(idx), n })
}

/// the level's facts of each thing, by row, into the hand-over
pub(in crate::fittings) fn prop_system_facts(g: &mut crate::game::Game) {
    use sim::props::*;
    let io = io();
    for r in 0..g.props.props.len() {
        let p = &g.tab.props_prop[g.props.props[r]];
        let t = p.tune;
        io.def[D_HX][r] = p.hx;
        io.def[D_HZ][r] = p.hz;
        io.def[D_RADIUS][r] = p.radius;
        io.def[D_HEIGHT][r] = p.height;
        io.def[D_MASS][r] = p.mass;
        io.def[D_INV_MASS][r] = p.inv_mass;
        io.def[D_INV_I][r] = p.inv_i;
        io.def[D_DECEL][r] = t.decel;
        io.def[D_VISC][r] = t.visc;
        io.def[D_REST][r] = t.rest;
        io.def[D_CHAIR][r] = if p.def.kind == crate::scene_level_data::PropKindName::Chair { 1.0 } else { 0.0 };
    }
}

/// the hand-over written and the step taken: how many noises it listed
pub(in crate::fittings) fn prop_system_hand_over(g: &mut crate::game::Game, dt: f64, contacts: &[crate::props::PropContact], t: &crate::props::PropSystemHandOverT) -> f64 {
    use sim::props::*;
    let n = g.props.props.len();
    prop_system_facts(g);
    let io = io();
    for (c, k) in contacts.iter().enumerate() {
        io.con[K_X][c] = k.pos[0];
        io.con[K_Z][c] = k.pos[2];
        io.con[K_VX][c] = k.vel[0];
        io.con[K_VZ][c] = k.vel[2];
        io.con[K_RADIUS][c] = k.radius;
        io.con[K_WHO][c] = k.who;
        io.con[K_MASS][c] = k.mass;
        io.con[K_QUIET][c] = if k.quiet { 1.0 } else { 0.0 };
    }
    let nl = t.n as usize;
    for l in 0..nl {
        io.leaf[l] = match &t.leaves {
            Some(v) => v[l],
            None => l as f64,
        };
    }
    for (f, y) in g.props.doorways.iter().enumerate() {
        io.way[Y_CX][f] = y.cx;
        io.way[Y_CZ][f] = y.cz;
        io.way[Y_UX][f] = y.ux;
        io.way[Y_UZ][f] = y.uz;
        io.way[Y_HALF][f] = y.half;
    }
    io.par[P_DT] = dt;
    io.par[P_PROPS] = n as f64;
    io.par[P_CONTACTS] = contacts.len() as f64;
    io.par[P_LEAVES] = t.n;
    io.par[P_WAYS] = g.props.doorways.len() as f64;
    io.par[P_FLAGS] = ((if g.props.enabled { F_ENABLED } else { 0 }) | F_CASTOR) as f64;
    io.par[P_FRICTION] = g.props.friction_scale;
    io.par[P_CAS_DECEL] = g.k.castor.decel;
    io.par[P_CAS_VISC] = g.k.castor.visc;
    io.par[P_CAS_SPIN] = g.k.castor.spin;
    io.par[P_CAS_START] = g.k.castor.start_v;
    io.par[P_CAS_STOP] = g.k.castor.stop_v;
    let ev = sim::props::update(g.sim, g.col.mirror.slot as usize);
    let io = self::io();   // (taken again: the step wrote the hand-over through its own reference)
    {
        let s = &mut g.m.sim_propshome.props_stepped;
        s.swept += io.par[P_SWEPT];
        s.guided += io.par[P_GUIDED];
        s.pairs += io.par[P_PAIRS];
        s.walls += io.par[P_WALLS];
        s.held += io.par[P_HELD];
        s.substeps += io.par[P_SUBSTEPS];
    }
    assert!(!js::truthy(io.par[P_WRONG]), "the furniture's step in the Rust module could not be what it was handed");
    ev as f64
}

/// what the step did taken back: where it leaves each man, and the noises in the order they were made, read out before anybody hears one
pub(in crate::fittings) fn prop_system_take_back(g: &mut crate::game::Game, contacts: &mut Vec<crate::props::PropContact>, ev: f64) {
    use sim::props::*;
    let io = io_read();
    for c in 0..contacts.len() {
        contacts[c].pos[0] = io.con[K_X][c];
        contacts[c].pos[2] = io.con[K_Z][c];
    }
    {
        let s = &mut g.m.sim_propshome.props_stepped;
        s.there += 1.0;
        s.noises += ev;
    }
    if ev == 0.0 {
        return;
    }
    let ev = ev as usize;
    let mut list: Vec<f64> = Vec::with_capacity(ev * NEV);
    for row in &io.ev[..ev] {
        list.extend_from_slice(row);
    }
    for k in 0..ev {
        let e = prop_system_heard(g, &list, (k * NEV) as f64);
        emit(g, e);
    }
}

/// a noise of the module's list as the event
pub(in crate::fittings) fn prop_system_heard(g: &crate::game::Game, e: &[f64], o: f64) -> crate::props::PropEvent {
    use sim::props::*;
    let o = o as usize;
    let sound = match e[o + E_SOUND] as usize {
        0 => PropEventSound::Scrape,
        1 => PropEventSound::Thud,
        _ => PropEventSound::Roll,
    };
    let roll = if sound == PropEventSound::Roll {
        Some(if e[o + E_ROLL] != 0.0 { PropEventRoll::Start } else { PropEventRoll::Stop })
    } else {
        None
    };
    PropEvent {
        prop: g.props.all[e[o + E_PROP] as usize],
        sound,
        pos: [e[o + E_X], e[o + E_Y], e[o + E_Z]],
        who: e[o + E_WHO],
        level: e[o + E_LEVEL],
        speed: e[o + E_SPEED],
        rolling: e[o + E_ROLLING] != 0.0,
        roll,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (every blank game has the same block and the tests run at once: each case has a row of its own)
    #[test]
    fn clamp_row_caps_speed_and_spin() {
        let g = crate::game::Game::blank();
        let c = cols(&g);
        (c.vx[1], c.vz[1], c.w[1]) = (6.0, 8.0, 12.0);
        clamp_row(c, 1);
        assert_eq!((c.vx[1], c.vz[1], c.w[1]), (3.0, 4.0, 9.0));
        (c.vx[1], c.vz[1], c.w[1]) = (3.0, 4.0, -12.0);
        clamp_row(c, 1);
        assert_eq!((c.vx[1], c.vz[1], c.w[1]), (3.0, 4.0, -9.0));
    }

    #[test]
    fn turn_row_keeps_the_trig_of_an_equal_yaw() {
        // −0 equals 0 in the TypeScript's `!==` too: the yaw is written, its cos and sin stay
        let g = crate::game::Game::blank();
        let c = cols(&g);
        (c.yaw[2], c.cs[2], c.sn[2]) = (0.0, 0.25, 0.75);
        turn_row(c, 2, -0.0);
        assert_eq!(c.yaw[2].to_bits(), (-0.0f64).to_bits());
        assert_eq!((c.cs[2], c.sn[2]), (0.25, 0.75));
        turn_row(c, 2, 0.5);
        assert_eq!(c.yaw[2], 0.5);
        assert_eq!(c.cs[2].to_bits(), m::cos(0.5).to_bits());
        assert_eq!(c.sn[2].to_bits(), m::sin(0.5).to_bits());
    }

    #[test]
    fn the_step_moves_the_man_in_the_contact_and_nothing_else_of_him() {
        // props.ts:379: x and z come back from the hand-over; y, the velocity and the rest stay
        let mut g = crate::game::Game::blank();
        let mut contacts = vec![PropContact { pos: [1.0, 2.0, 3.0], vel: [4.0, 5.0, 6.0], radius: 0.3, who: 7.0, mass: 75.0, quiet: true }];
        io().con[sim::props::K_X][0] = 1.5;
        io().con[sim::props::K_Z][0] = 3.5;
        prop_system_take_back(&mut g, &mut contacts, 0.0);
        assert_eq!(contacts[0].pos, [1.5, 2.0, 3.5]);
        assert_eq!(contacts[0].vel, [4.0, 5.0, 6.0]);
        assert_eq!(g.m.sim_propshome.props_stepped.there, 1.0);
    }

    #[test]
    fn against_the_typescripts_own_bits() {
        // bun, EXACT.on: 1.7 * (5 / Math.hypot(1.7, 9.3)), 9.3 * (…), Math.cos(0.7), Math.sin(0.7)
        let g = crate::game::Game::blank();
        let c = cols(&g);
        (c.vx[3], c.vz[3], c.w[3], c.yaw[3]) = (1.7, 9.3, 0.0, 0.0);
        clamp_row(c, 3);
        assert_eq!(c.vx[3].to_bits(), 0x3fecc545374c7b17);
        assert_eq!(c.vz[3].to_bits(), 0x4013ac8b911bd42d);
        turn_row(c, 3, 0.7);
        assert_eq!(c.cs[3].to_bits(), 0x3fe87996529f9d92);
        assert_eq!(c.sn[3].to_bits(), 0x3fe49d6e694619b8);
    }
}
