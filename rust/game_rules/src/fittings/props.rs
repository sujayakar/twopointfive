//! THE FURNITURE MEN SHOVE: every chair, carton, bin and plant of a level. It coasts and comes to rest, the men walking into it set it going and are held back by what
//! cannot yield, it knocks into its neighbours, a swinging leaf sweeps it aside, a door frame guides it through, the statics stop it; and the scrapes and knocks all that makes.
//!
//! A thing is one record (`Prop`, in `g.tab.props_prop`), its pose and its speed beside the facts of its kind. The things in the level now are `g.props.props`, in the level's
//! own order; a thing a blast took out is out of that list and keeps its record for the level put back. No box of the box world is written by the step: furniture is not in it
//! (nothing sees, hears or walks by a chair), so the boxes the picture draws are placed from the record after it (`prop_place`). The statics a thing rests against and the
//! leaves that sweep it are read from the box world's rows (`sim::collide`).
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::V3;
use ::sim::maths::{cos, exp, hypot2, sin};
use ::sim::world::{asking, js_abs, js_ceil, js_max, js_min, World, REC, R_C, R_COS, R_H, R_SIN};
use alloc::{string::String, sync::Arc, vec::Vec};
use prim::sqrt;
pub use ::game_types::props::types;
pub use types::*;

/// the one target of the TypeScript's stored `emit`, called by name
#[inline]
fn emit(g: &mut Game, e: PropEvent) {
    crate::game::game_on_prop_event(g, e);
}

/// the speed caps
fn clamp_vel(p: &mut Prop) {
    let s = hypot2(p.vx, p.vz);
    if s > MAX_V {
        p.vx *= MAX_V / s;
        p.vz *= MAX_V / s;
    }
    if p.w > MAX_W { p.w = MAX_W; } else if p.w < -MAX_W { p.w = -MAX_W; }
}

/// its yaw written, with its cos and sin when it is another value (a yaw that goes from −0 to +0 is the same yaw, and keeps the sin it had)
#[inline(always)]
fn turn(p: &mut Prop, yaw: f64) {
    if yaw != p.yaw {
        p.cs = cos(yaw);
        p.sn = sin(yaw);
    }
    p.yaw = yaw;
}

/// where a thing stands, written by anybody but the step, is said
pub fn prop_move_to(g: &mut Game, id: PropId, x: f64, z: f64) {
    let p = &mut g.tab.props_prop[id];
    (p.x, p.z) = (x, z);
    stands(&mut g.tab.boxes, p);
}

/// how it is turned, likewise
pub fn prop_turn_to(g: &mut Game, id: PropId, yaw: f64) {
    turn(&mut g.tab.props_prop[id], yaw);
    stands(&mut g.tab.boxes, &g.tab.props_prop[id]);
}
/// the box world is told where a thing stands
fn stands(boxes: &mut crate::boxworld::Boxes, p: &Prop) { boxes.thing(p.id as u32, p.x, p.z, p.yaw, p.radius); }
/// …and what of the furniture is in the level, in its order
fn listed(g: &mut Game) {
    let t = &g.tab.props_prop;
    g.tab.boxes.things(g.props.props.iter().map(|&p| (t[p].id as u32, &t[p].boxes[..])));
}

/// a thing of furniture made: its record, its part boxes, placed
fn prop_new(g: &mut Game, def: &Arc<crate::scene_level::PropDef>, id_2: f64) -> PropId {
    let id = PropId(id_2 as u32);
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
        tune: *kind().of(def.kind.as_str()).expect("a kind of furniture KIND has no row for"),
        home: [def.x, def.z, def.yaw],
        boxes,
        def: def.clone(),
        id: id_2,
        x: def.x,
        z: def.z,
        yaw: def.yaw,
        // (of the yaw it is made at, whatever that is: a yaw of −0 equals 0 and would keep the cos and sin of +0)
        cs: m::cos(def.yaw),
        sn: m::sin(def.yaw),
        vx: 0.0,
        vz: 0.0,
        w: 0.0,
        idle: 0.0,
        push_t: -10.0,
        noise_t: -10.0,
        who: 0.0,
        asleep: true,
        dirty: true,
        rolling: false,
    });
    debug_assert!(got == id, "a thing's id is its row of the table: one PropSystem a game");
    prop_place(g, id);
    id
}

/// the part boxes put where the pose says, every part from the same five numbers
pub fn prop_place(g: &mut Game, id: PropId) {
    let (t, boxes) = (&mut g.tab.props_prop[id], &mut g.tab.boxes);
    let (x, z, yaw, cs, sn) = (t.x, t.z, t.yaw, t.cs, t.sn);
    for (part, &b) in t.def.parts.iter().zip(t.boxes.iter()) {
        let off = part.off;
        let mut bx = boxes.edit(b);
        bx.c = [x + cs * off[0] + sn * off[2], off[1], z - sn * off[0] + cs * off[2]];
        bx.yaw = yaw + part.yaw;
    }
    t.dirty = false;
    stands(boxes, t);
}

pub fn prop_wake(g: &mut Game, id: PropId) {
    wake(&mut g.tab.props_prop[id]);
}
#[inline(always)]
fn wake(p: &mut Prop) {
    p.asleep = false;
    p.idle = 0.0;
}

/// the furniture of a level made and settled. RETURNS the system; for the settle, which reaches it by `g.props` as every method does, it stands in
/// `g.props` and the old value is put back, so the caller's `g.props = …` is what keeps it
pub fn prop_system_new(g: &mut Game, defs: &[Arc<crate::scene_level::PropDef>]) -> PropSystem {
    if defs.len() as f64 > PROP_CAP {
        panic!("a level of {} loose things of furniture: the game holds {}", defs.len(), PROP_CAP);
    }
    let mut sys = PropSystem::blank();
    for (i, d) in defs.iter().enumerate() {
        let id = prop_new(g, d, i as f64);
        sys.all.push(id);
    }
    sys.props = sys.all.clone();
    let old = core::mem::replace(&mut g.props, sys);
    listed(g);
    prop_system_settle(g);
    core::mem::replace(&mut g.props, old)
}

/// every part box of what is in the level, in its order
pub fn prop_system_boxes(g: &Game, out: &mut Vec<AnyBoxId>) {
    for &p in &g.props.props {
        out.extend(g.tab.props_prop[p].boxes.iter().map(|&b| AnyBoxId::World(b)));
    }
}

/// a thing leaves the level: no longer drawn, solid, pushed or stepped until reset
pub fn prop_system_remove(g: &mut Game, p: PropId) {
    if let Some(i) = g.props.props.iter().position(|&q| q == p) {
        g.props.props.remove(i);
        listed(g);
    }
}

/// everything back to its rest pose, asleep
pub fn prop_system_reset(g: &mut Game) {
    if g.props.props.len() != g.props.all.len() {
        g.props.props.clear();
        g.props.props.extend_from_slice(&g.props.all);
        listed(g);
    }
    for i in 0..g.props.props.len() {
        let id = g.props.props[i];
        let p = &mut g.tab.props_prop[id];
        let home = p.home;
        (p.x, p.z) = (home[0], home[1]);
        turn(p, home[2]);
        (p.w, p.vz, p.vx, p.idle) = (0.0, 0.0, 0.0, 0.0);
        p.asleep = true;
        (p.push_t, p.noise_t) = (-10.0, -10.0);
        p.who = 0.0;
        p.rolling = false;
        prop_place(g, id);
    }
}

/// authoring overlaps resolved once, silently; the settled pose becomes home
fn prop_system_settle(g: &mut Game) {
    g.props.muted = true;
    for i in 0..g.props.props.len() {
        let id = g.props.props[i];
        g.tab.props_prop[id].dirty = true;
    }
    if !g.props.props.is_empty() {
        let mut st = Step { t: &mut g.tab.props_prop, live: &g.props.props, ways: &[], men: &mut [], leaves: &[], w: g.tab.boxes.world(), enabled: true, time: g.props.time, friction: 0.0, cas: &g.k.castor, said: Vec::new() };
        for _ in 0..SETTLE_PASSES {
            for &i in st.live {
                st.solve_statics(i, false);
            }
            st.solve_pairs(false);
        }
    }
    for i in 0..g.props.props.len() {
        let id = g.props.props[i];
        let p = &mut g.tab.props_prop[id];
        (p.w, p.vz, p.vx) = (0.0, 0.0, 0.0);
        p.asleep = true;
        p.home = [p.x, p.z, p.yaw];
        prop_place(g, id);
    }
    g.props.muted = false;
}

/// closest part of any thing along the ray (t < tmax)
pub fn prop_system_raycast(g: &Game, ro: V3, rd: V3, tmax: f64) -> Option<PropSystemRaycast> {
    crate::scene_collision::things_ray(g, ro, rd, tmax).map(|h| PropSystemRaycast { t: h.t, n: h.n, prop: g.props.props[h.row as usize], box_: h.box_ })
}

/// planar impulse J at a world point; `who` is blamed; thud reports it as an impact
pub fn prop_system_apply_impulse(g: &mut Game, id: PropId, point: V3, j: V3, who: f64, thud: Option<bool>) {
    let p = &mut g.tab.props_prop[id];
    let (rx, rz) = (point[0] - p.x, point[2] - p.z);
    p.vx += j[0] * p.inv_mass;
    p.vz += j[2] * p.inv_mass;
    p.w += (rz * j[0] - rx * j[2]) * p.inv_i;
    clamp_vel(p);
    wake(p);
    if js::truthy(who) {
        p.who = who;
    }
    if thud.unwrap_or(false) {
        let (by, speed) = (js::or(who, p.who), m::hyp2(j[0], j[2]) * p.inv_mass + 1.0);
        prop_system_noise(g, id, crate::props::PropEventSound::Thud, speed, by, 0.55);
    }
}

/// a noise, rate limited by the thing: a chair along a wall is one scrape and the odd knock
fn prop_system_noise(g: &mut Game, id: PropId, sound: crate::props::PropEventSound, speed: f64, who: f64, level: f64) {
    let (time, p) = (g.props.time, &mut g.tab.props_prop[id]);
    let gap = if sound == crate::props::PropEventSound::Thud { 0.35 } else { 0.8 };
    if g.props.muted || time - p.noise_t < gap {
        return;
    }
    p.noise_t = time;
    let sound = sound;
    let e = PropEvent { prop: id, sound, pos: [p.x, m::min(0.6, p.height * 0.5), p.z], who, level: m::min(1.0, level), speed, rolling: p.rolling, roll: None };
    emit(g, e);
}

/// one tick of the furniture: its step, then the boxes of what it moved placed
pub fn prop_system_update(g: &mut Game, dt: f64, contacts: &mut [PropContact], leaves: &[BoxId]) {
    let n = g.props.props.len();
    if !g.props.all.is_empty() {
        debug_assert!(!g.props.muted, "the furniture was stepped while its level was still being settled");
        let sweeps: Vec<usize> = leaves.iter().filter_map(|b| g.col.dynamic_boxes.iter().position(|q| q == b)).collect();
        let said = prop_system_step(g, dt, contacts, &sweeps);
        // (the noises are heard once the step is whole, in the order they were made)
        for e in said {
            emit(g, e);
        }
    }
    for r in 0..n {
        let id = g.props.props[r];
        if g.tab.props_prop[id].dirty {
            prop_place(g, id);
        }
    }
}

/// the substeps of one tick: the men's places and the things' records written, the noises listed
fn prop_system_step(g: &mut Game, dt: f64, men: &mut [PropContact], leaves: &[usize]) -> Vec<PropEvent> {
    let g = &mut *g;
    let w = g.tab.boxes.world();
    let mut st = Step {
        t: &mut g.tab.props_prop,
        live: &g.props.props,
        ways: &g.props.doorways,
        men,
        leaves,
        w,
        enabled: g.props.enabled,
        time: g.props.time,
        friction: g.props.friction_scale,
        cas: &g.k.castor,
        said: Vec::new(),
    };
    let steps = js_min(4.0, js_max(1.0, js_ceil(dt * 60.0 - 1e-6)));
    let h = dt / steps;
    let mut s = 0.0;
    while s < steps {
        st.time += h;
        st.step(h);
        s += 1.0;
    }
    let (time, said) = (st.time, st.said);
    g.props.time = time;
    said
}

/// what a ray through the level meets first
#[derive(Clone, Copy)]
pub struct LevelRay {
    pub t: f64,
    pub n: V3,
    /// the thing of furniture it is, where it is not the level itself
    pub thing: Option<PropId>,
}

/// the nearest of the statics, the leaves and the furniture along a ray
pub fn level_ray(g: &Game, ro: V3, rd: V3, tmax: f64) -> Option<LevelRay> {
    let st = crate::scene_collision::raycast(g, ro, rd, tmax);
    if let Some(h) = prop_system_raycast(g, ro, rd, st.as_ref().map_or(tmax, |s| s.t)) {
        return Some(LevelRay { t: h.t, n: h.n, thing: Some(h.prop) });
    }
    st.map(|s| LevelRay { t: s.t, n: s.n, thing: None })
}

/// the settle's passes: authoring overlaps (a chair tucked under a table's edge, a bin grazing a desk's corner) are resolved once as the level is stood up, and where a thing comes to stand is its home
const SETTLE_PASSES: usize = 12;
const SLEEP_V: f64 = 0.04;
const SLEEP_W: f64 = 0.06;
const SLEEP_T: f64 = 0.3;
const DOOR_GUIDE: f64 = 1.2;
const Y_MIN: f64 = 0.04;
/// statics one gather can list
const NEAR: usize = 256;
const PI: f64 = core::f64::consts::PI;

/// what two footprints do to each other: the way out for B, how deep, and where they touch
#[derive(Clone, Copy)]
struct Sat {
    nx: f64,
    nz: f64,
    depth: f64,
    px: f64,
    pz: f64,
}
/// The farthest point of a footprint along (dx, dz), an edge that lies flat giving its middle
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn support(cx: f64, cz: f64, cs: f64, sn: f64, hx: f64, hz: f64, dx: f64, dz: f64) -> (f64, f64) {
    let (du, dv) = (dx * cs - dz * sn, dx * sn + dz * cs);
    let su = if du > 0.06 {
        hx
    } else if du < -0.06 {
        -hx
    } else {
        0.0
    };
    let sv = if dv > 0.06 {
        hz
    } else if dv < -0.06 {
        -hz
    } else {
        0.0
    };
    (cx + cs * su + sn * sv, cz - sn * su + cs * sv)
}
/// Two turned rectangles on the floor, A and B
#[allow(clippy::too_many_arguments)]
fn sat_rects(ax: f64, az: f64, acs: f64, asn: f64, ahx: f64, ahz: f64, bx: f64, bz: f64, bcs: f64, bsn: f64, bhx: f64, bhz: f64) -> Option<Sat> {
    let (dx, dz) = (bx - ax, bz - az);
    let (cuu, cuv) = (js_abs(acs * bcs + asn * bsn), js_abs(acs * bsn - asn * bcs));
    let (mut best, mut bnx, mut bnz, mut from_a) = (1e9, 0.0, 0.0, true);
    // A.u
    let mut d = dx * acs - dz * asn;
    let mut ov = ahx + bhx * cuu + bhz * cuv - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = acs * s;
        bnz = -asn * s;
        from_a = true;
    }
    // A.v
    d = dx * asn + dz * acs;
    ov = ahz + bhx * cuv + bhz * cuu - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = asn * s;
        bnz = acs * s;
        from_a = true;
    }
    // B.u
    d = dx * bcs - dz * bsn;
    ov = bhx + ahx * cuu + ahz * cuv - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = bcs * s;
        bnz = -bsn * s;
        from_a = false;
    }
    // B.v
    d = dx * bsn + dz * bcs;
    ov = bhz + ahx * cuv + ahz * cuu - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = bsn * s;
        bnz = bcs * s;
        from_a = false;
    }
    let (px, pz) = if from_a { support(bx, bz, bcs, bsn, bhx, bhz, -bnx, -bnz) } else { support(ax, az, acs, asn, ahx, ahz, bnx, bnz) };
    Some(Sat { nx: bnx, nz: bnz, depth: best, px, pz })
}

/// a man's circle against a thing's footprint: the way out for him, how deep, and the point on the thing
struct Touch {
    nx: f64,
    nz: f64,
    pen: f64,
    px: f64,
    pz: f64,
}

/// this thing's footprint against a rectangle A (a neighbour's, a leaf's, a static's): sat_rects(A, this)
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn sat_with(p: &Prop, ax: f64, az: f64, acs: f64, asn: f64, ahx: f64, ahz: f64) -> Option<Sat> {
    sat_rects(ax, az, acs, asn, ahx, ahz, p.x, p.z, p.cs, p.sn, p.hx, p.hz)
}

/// a man's circle against a thing's footprint
fn circle_contact(p: &Prop, cx: f64, cz: f64, r: f64) -> Option<Touch> {
    let (cs, sn, x, z, hx, hz) = (p.cs, p.sn, p.x, p.z, p.hx, p.hz);
    let (dx, dz) = (cx - x, cz - z);
    let (lx, lz) = (cs * dx - sn * dz, sn * dx + cs * dz);
    let qx = if lx < -hx {
        -hx
    } else if lx > hx {
        hx
    } else {
        lx
    };
    let qz = if lz < -hz {
        -hz
    } else if lz > hz {
        hz
    } else {
        lz
    };
    let (ddx, ddz) = (lx - qx, lz - qz);
    let d2 = ddx * ddx + ddz * ddz;
    if d2 >= r * r {
        return None;
    }
    let (nlx, nlz, pen);
    if d2 > 1e-10 {
        let d = sqrt(d2);
        nlx = ddx / d;
        nlz = ddz / d;
        pen = r - d;
    } else {
        let (px, pz) = (hx - js_abs(lx), hz - js_abs(lz));
        if px < pz {
            nlx = if lx < 0.0 { -1.0 } else { 1.0 };
            nlz = 0.0;
            pen = px + r;
        } else {
            nlx = 0.0;
            nlz = if lz < 0.0 { -1.0 } else { 1.0 };
            pen = pz + r;
        }
    }
    Some(Touch { nx: cs * nlx + sn * nlz, nz: -sn * nlx + cs * nlz, pen, px: x + cs * qx + sn * qz, pz: z - sn * qx + cs * qz })
}

/// could a thing meet a circle of `r` about (x, z)
#[inline(always)]
fn nears(p: &Prop, x: f64, z: f64, r: f64) -> bool {
    let (dx, dz) = (p.x - x, p.z - z);
    let reach = p.radius + r;
    !(dx * dx + dz * dz > reach * reach)
}

/// a man against a thing
#[inline(always)]
fn man_contact(p: &Prop, man: &PropContact) -> Option<Touch> {
    let (x, z, r) = (man.pos[0], man.pos[2], man.radius);
    if nears(p, x, z, r) { circle_contact(p, x, z, r) } else { None }
}

/// how fast a point of it goes, (rx, rz) from its middle
#[inline(always)]
fn speed_at(p: &Prop, rx: f64, rz: f64) -> (f64, f64) {
    (p.vx + p.w * rz, p.vz - p.w * rx)
}

/// a knock of `j` along (nx, nz), `arm` from its middle
#[inline(always)]
fn knock(p: &mut Prop, j: f64, nx: f64, nz: f64, arm: f64) {
    p.vx += j * nx * p.inv_mass;
    p.vz += j * nz * p.inv_mass;
    p.w += j * arm * p.inv_i;
}

/// a leaf's footprint, out of the box world's row of it, and the circle round it
struct Leaf {
    x: f64,
    z: f64,
    hx: f64,
    hz: f64,
    cs: f64,
    sn: f64,
    r: f64,
}

impl Leaf {
    fn of(o: &World, l: usize) -> Leaf {
        let b = l * REC;
        let (hx, hz) = (o.leaf[b + R_H], o.leaf[b + R_H + 2]);
        Leaf { x: o.leaf[b + R_C], z: o.leaf[b + R_C + 2], hx, hz, cs: o.leaf[b + R_COS], sn: o.leaf[b + R_SIN], r: hypot2(hx, hz) } // (cos and sin are kept beside the leaf's yaw by whoever wrote the row)
    }
}

/// One step's view of the game: the things' records, the men, the leaves and the frames, and the switches.
struct Step<'a> {
    t: &'a mut Tab<PropId, Prop>,
    live: &'a [PropId],
    ways: &'a [PropSystemDoorwaysItem],
    men: &'a mut [PropContact],
    /// which of the box world's leaves may sweep them
    leaves: &'a [usize],
    w: &'a World,
    enabled: bool,
    time: f64,
    friction: f64,
    cas: &'a crate::knobs::CastorK,
    said: Vec<PropEvent>,
}

impl Step<'_> {
    /// a noise onto the list
    #[allow(clippy::too_many_arguments)]
    fn emit(&mut self, i: PropId, sound: PropEventSound, roll: Option<PropEventRoll>, y: f64, who: f64, level: f64, speed: f64) {
        let p = &self.t[i];
        self.said.push(PropEvent { prop: i, sound, pos: [p.x, y, p.z], who, level, speed, rolling: p.rolling, roll });
    }
    /// a castor chair's coast, said once as it starts and once as it stops
    fn roll_note(&mut self, i: PropId, speed: f64) {
        let (cas, rolling, who) = (self.cas, self.t[i].rolling, self.t[i].who);
        if !rolling && speed > cas.start_v {
            self.t[i].rolling = true;
            self.emit(i, PropEventSound::Roll, Some(PropEventRoll::Start), 0.05, who, 0.0, speed);
        } else if rolling && speed < cas.stop_v {
            self.t[i].rolling = false;
            self.emit(i, PropEventSound::Roll, Some(PropEventRoll::Stop), 0.05, who, 0.0, speed);
        }
    }
    /// a scrape or a knock, no more often than a thing can make one
    fn noise(&mut self, i: PropId, sound: PropEventSound, speed: f64, who: f64, level: f64) {
        let t = self.time;
        let p = &mut self.t[i];
        if t - p.noise_t < (if sound == PropEventSound::Thud { 0.35 } else { 0.8 }) {
            return;
        }
        p.noise_t = t;
        let y = js_min(0.6, p.height * 0.5);
        self.emit(i, sound, None, y, who, js_min(1.0, level), speed);
    }
    /// thing against thing: `dynamic` false = the two are pushed apart and nothing else (the level's settle)
    fn solve_pairs(&mut self, dynamic: bool) {
        let live = self.live;
        for (at, &a) in live.iter().enumerate() {
            for &b in &live[at + 1..] {
                let (pa, pb) = (&self.t[a], &self.t[b]);
                if pa.asleep && pb.asleep && !pa.dirty && !pb.dirty {
                    continue;
                }
                if !nears(pb, pa.x, pa.z, pa.radius) {
                    continue;
                }
                let Some(sat) = sat_with(pb, pa.x, pa.z, pa.cs, pa.sn, pa.hx, pa.hz) else { continue };
                let (nx, nz) = (sat.nx, sat.nz);
                let (ima, imb, iia, iib) = (pa.inv_mass, pb.inv_mass, pa.inv_i, pb.inv_i);
                let corr = js_max(0.0, sat.depth - 0.003);
                let wa = ima / (ima + imb);
                {
                    let p = &mut self.t[a];
                    p.x -= nx * corr * wa;
                    p.z -= nz * corr * wa;
                    p.dirty = true;
                    let q = &mut self.t[b];
                    q.x += nx * corr * (1.0 - wa);
                    q.z += nz * corr * (1.0 - wa);
                    q.dirty = true;
                }
                if !dynamic {
                    continue;
                }
                let (pa, pb) = (&self.t[a], &self.t[b]);
                let (rax, raz, rbx, rbz) = (sat.px - pa.x, sat.pz - pa.z, sat.px - pb.x, sat.pz - pb.z);
                let ((vax, vaz), (vbx, vbz)) = (speed_at(pa, rax, raz), speed_at(pb, rbx, rbz));
                let vn = (vbx - vax) * nx + (vbz - vaz) * nz;
                if vn > -0.05 {
                    continue;
                }
                let vlin = (pb.vx - pa.vx) * nx + (pb.vz - pa.vz) * nz;
                let who = if pa.asleep {
                    pb.who
                } else if pb.asleep {
                    pa.who
                } else if hypot2(pa.vx, pa.vz) > hypot2(pb.vx, pb.vz) {
                    pa.who
                } else {
                    pb.who
                };
                let (ca, cb) = (raz * nx - rax * nz, rbz * nx - rbx * nz);
                let e = js_min(pa.tune.rest, pb.tune.rest);
                let jn = -(1.0 + e) * vn / (ima + imb + iia * ca * ca + iib * cb * cb);
                for (id, j, arm) in [(a, -jn, ca), (b, jn, cb)] {
                    let p = &mut self.t[id];
                    knock(p, j, nx, nz, arm);
                    clamp_vel(p);
                    p.who = who;
                    wake(p);
                }
                let (pa, pb) = (&self.t[a], &self.t[b]);
                if -vlin > 0.9 && self.time - js_max(pa.push_t, pb.push_t) > 0.05 {
                    let lighter = if pa.mass < pb.mass { a } else { b };
                    self.noise(lighter, PropEventSound::Thud, -vlin, who, 0.12 + 0.14 * -vlin);
                }
            }
        }
    }

    /// a thing pushed out of the statics round it and, `dynamic`, its turn and its speed answering the wall, and the knock; `dynamic` false = pushed out and nothing else (the level's settle)
    fn solve_statics(&mut self, i: PropId, dynamic: bool) {
        let (radius, height, im, ii, rest) = { let p = &self.t[i]; (p.radius, p.height, p.inv_mass, p.inv_i, p.tune.rest) };
        for _ in 0..3 {
            let rr = radius + 0.02;
            let (x, z) = (self.t[i].x, self.t[i].z);
            let n = ::sim::collide::gather(self.w, x - rr, z - rr, x + rr, z + rr, Y_MIN, height, NEAR);
            let o = self.w;
            let mut any = false;
            for k in 0..n {
                let b = asking().list[k] as usize * REC;
                let (bx, bz, bhx, bhz) = (o.rec[b + R_C], o.rec[b + R_C + 2], o.rec[b + R_H], o.rec[b + R_H + 2]);
                let (bcs, bsn) = (o.rec[b + R_COS], o.rec[b + R_SIN]);
                let c = &mut self.t[i];
                let (dx, dz) = (c.x - bx, c.z - bz);
                let (lx, lz) = (bcs * dx - bsn * dz, bsn * dx + bcs * dz);
                let (ex, ez) = (js_abs(lx) - bhx, js_abs(lz) - bhz);
                if ex > radius || ez > radius || (ex > 0.0 && ez > 0.0 && ex * ex + ez * ez > radius * radius) {
                    continue;
                }
                let Some(sat) = sat_with(c, bx, bz, bcs, bsn, bhx, bhz) else { continue };
                any = true;
                let (nx, nz) = (sat.nx, sat.nz);
                c.x += nx * sat.depth;
                c.z += nz * sat.depth;
                c.dirty = true;
                if !dynamic {
                    continue;
                }
                let (rx, rz) = (sat.px - c.x, sat.pz - c.z);
                let (vpx, vpz) = speed_at(c, rx, rz);
                let vn = vpx * nx + vpz * nz;
                if vn >= 0.0 {
                    continue;
                }
                let vlin = c.vx * nx + c.vz * nz;
                let cr = rz * nx - rx * nz;
                let kn = im + ii * cr * cr;
                let jn = -(1.0 + rest) * vn / kn;
                knock(c, jn, nx, nz, cr);
                let (tx, tz) = (-nz, nx);
                let vt = vpx * tx + vpz * tz;
                if js_abs(vt) > 1e-4 {
                    let ct = rz * tx - rx * tz;
                    let kt = im + ii * ct * ct;
                    let mut jt = -vt / kt;
                    let lim = 0.4 * jn;
                    if jt > lim {
                        jt = lim;
                    } else if jt < -lim {
                        jt = -lim;
                    }
                    knock(c, jt, tx, tz, ct);
                }
                clamp_vel(c);
                if -vlin > 0.8 && self.time - c.push_t > 0.05 {
                    let who = c.who;
                    self.noise(i, PropEventSound::Thud, -vlin, who, 0.1 + 0.15 * -vlin);
                }
            }
            if !any {
                break;
            }
        }
    }

    /// one substep of `h` seconds: they coast, the men push them, they knock together, the leaves and the frames and the statics have them, and the men give back what could not yield
    fn step(&mut self, h: f64) {
        let live = self.live;
        if self.enabled {
            for &i in live {
                self.coasts(i, h);
            }
        }
        for q in 0..self.men.len() {
            for &i in live {
                self.pushed(q, i);
            }
        }
        if self.enabled {
            self.solve_pairs(true);
            for &l in self.leaves {
                let leaf = Leaf::of(self.w, l);
                for &i in live {
                    self.swept(&leaf, i, h);
                }
            }
            for f in self.ways {
                for &i in live {
                    self.guided(f, i, h);
                }
            }
            // the statics, last of what moves a thing
            for &i in live {
                if !self.t[i].asleep || self.t[i].dirty {
                    self.solve_statics(i, true);
                }
            }
        }
        for q in 0..self.men.len() {
            for &i in live {
                self.gives_back(q, i);
            }
        }
    }

    /// it coasts, the floor slows it, and it sleeps
    fn coasts(&mut self, i: PropId, h: f64) {
        let fs = self.friction;
        let c = &mut self.t[i];
        if c.asleep {
            return;
        }
        let cas = c.def.kind == crate::scene_level_data::PropKindName::Chair;
        let (dec, vis, spin) = if cas { (self.cas.decel, self.cas.visc, self.cas.spin) } else { (c.tune.decel, c.tune.visc, 1.5 * c.tune.decel) };
        c.x += c.vx * h;
        c.z += c.vz * h;
        let yaw = c.yaw + c.w * h;
        turn(c, yaw);
        c.dirty = true;
        let sp = hypot2(c.vx, c.vz);
        if sp > 0.0 {
            let ns = js_max(0.0, sp - dec * fs * h) * exp(-vis * fs * h);
            c.vx *= ns / sp;
            c.vz *= ns / sp;
        }
        let aw = js_abs(c.w);
        if aw > 0.0 {
            let na = js_max(0.0, aw - spin * fs / c.radius * h) * exp(-vis * fs * h);
            c.w *= na / aw;
        }
        if cas || c.rolling {
            let v = if cas { hypot2(c.vx, c.vz) } else { 0.0 };
            self.roll_note(i, v);
        }
        let c = &mut self.t[i];
        rests(c, sp < SLEEP_V && aw < SLEEP_W, h);
        if c.yaw > PI * 4.0 || c.yaw < -PI * 4.0 {
            let yaw = c.yaw % (PI * 2.0);
            turn(c, yaw);
        }
    }

    /// a man walking into it sets it going, each giving way by his weight and its own
    fn pushed(&mut self, q: usize, i: PropId) {
        let man = self.men[q];
        let c = &mut self.t[i];
        let Some(cc) = man_contact(c, &man) else { return };
        let (nx, nz, pen) = (cc.nx, cc.nz, cc.pen);
        if !self.enabled {
            self.men[q].pos[0] += nx * pen;
            self.men[q].pos[2] += nz * pen;
            return;
        }
        let fresh = self.time - c.push_t > 0.25;
        c.push_t = self.time;
        wake(c);
        c.who = man.who;
        c.dirty = true;
        let (im, inv_mc) = (c.inv_mass, 1.0 / man.mass);
        let (rx, rz) = (cc.px - c.x, cc.pz - c.z);
        let (vpx, vpz) = speed_at(c, rx, rz);
        let vn = (vpx - man.vel[0]) * nx + (vpz - man.vel[2]) * nz;
        if vn > 0.0 {
            let cr = rz * nx - rx * nz;
            let jn = vn / (im + c.inv_i * cr * cr + inv_mc);
            knock(c, -jn, nx, nz, cr);
            clamp_vel(c);
            let dv = jn * im;
            if fresh && dv > 0.7 {
                self.noise(i, PropEventSound::Scrape, dv, man.who, (0.28 * dv) * (if man.quiet { 0.5 } else { 1.0 }));
            }
        }
        let share = im / (im + inv_mc);
        let c = &mut self.t[i];
        c.x -= nx * pen * share;
        c.z -= nz * pen * share;
        self.men[q].pos[0] += nx * pen * (1.0 - share);
        self.men[q].pos[2] += nz * pen * (1.0 - share);
    }

    /// a leaf swung into it sweeps it aside
    fn swept(&mut self, l: &Leaf, i: PropId, h: f64) {
        let c = &mut self.t[i];
        if !nears(c, l.x, l.z, l.r) {
            return;
        }
        let Some(sat) = sat_with(c, l.x, l.z, l.cs, l.sn, l.hx, l.hz) else { return };
        c.x += sat.nx * sat.depth;
        c.z += sat.nz * sat.depth;
        c.dirty = true;
        wake(c);
        c.push_t = self.time;
        c.who = 0.0;
        let vn = c.vx * sat.nx + c.vz * sat.nz;
        let want = js_min(2.0, sat.depth / h * 0.5);
        if vn < want {
            c.vx += (want - vn) * sat.nx;
            c.vz += (want - vn) * sat.nz;
        }
    }

    /// a door frame guides through it a thing somebody is shoving at it
    fn guided(&mut self, f: &PropSystemDoorwaysItem, i: PropId, h: f64) {
        let (fux, fuz, half) = (f.ux, f.uz, f.half);
        let c = &mut self.t[i];
        if c.asleep || self.time - c.push_t > 0.12 || c.who == 0.0 {
            return;
        }
        let (dx, dz) = (c.x - f.cx, c.z - f.cz);
        if dx * dx + dz * dz > (half + c.radius + 0.4) * (half + c.radius + 0.4) {
            return;
        }
        let (cs, sn) = (c.cs, c.sn);
        let (along, off) = (dx * fux + dz * fuz, -dx * fuz + dz * fux);
        let eu = c.hx * js_abs(cs * fux - sn * fuz) + c.hz * js_abs(sn * fux + cs * fuz);
        let en = c.hx * js_abs(-cs * fuz - sn * fux) + c.hz * js_abs(-sn * fuz + cs * fux);
        let lim = half - eu - 0.01;
        if lim < 0.0 || js_abs(along) > half + 0.05 || js_abs(off) > en + 0.15 {
            return;
        }
        let over = js_abs(along) - lim;
        let passing = js_abs(-c.vx * fuz + c.vz * fux) >= 0.5 * js_abs(c.vx * fux + c.vz * fuz);
        if over <= 0.0 || !passing {
            return;
        }
        let by = js_min(over, DOOR_GUIDE * h) * (if along > 0.0 { -1.0 } else { 1.0 });
        c.x += fux * by;
        c.z += fuz * by;
        c.dirty = true;
    }

    /// what it could not yield the man gives back, and a thing found squeezed sheds what he keeps feeding it
    fn gives_back(&mut self, q: usize, i: PropId) {
        let c = &mut self.t[i];
        let Some(cc) = man_contact(c, &self.men[q]) else { return };
        self.men[q].pos[0] += cc.nx * cc.pen;
        self.men[q].pos[2] += cc.nz * cc.pen;
        if cc.pen > 0.004 {
            c.vx *= 0.3;
            c.vz *= 0.3;
            c.w *= 0.3;
        }
    }
}

/// it goes to sleep once it has been `still` for long enough
fn rests(c: &mut Prop, still: bool, h: f64) {
    if !still {
        c.idle = 0.0;
        return;
    }
    c.idle += h;
    if c.idle > SLEEP_T {
        c.asleep = true;
        c.vx = 0.0;
        c.vz = 0.0;
        c.w = 0.0;
    }
}

/// a floor point's distance to a prop's footprint where it RESTS (0 = inside it)
pub fn prop_gap(home: V3, hx: f64, hz: f64, at: V3) -> f64 {
    let dx = at[0] - home[0];
    let dz = at[2] - home[1];
    let c = m::cos(home[2]);
    let s = m::sin(home[2]);
    m::hyp2(m::max(0.0, m::abs(c * dx - s * dz) - hx), m::max(0.0, m::abs(s * dx + c * dz) - hz))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_rest() -> Prop {
        Prop::blank()
    }

    #[test]
    fn the_speed_caps_hold_speed_and_spin() {
        let mut p = at_rest();
        (p.vx, p.vz, p.w) = (6.0, 8.0, 12.0);
        clamp_vel(&mut p);
        assert_eq!((p.vx, p.vz, p.w), (3.0, 4.0, 9.0));
        (p.vx, p.vz, p.w) = (3.0, 4.0, -12.0);
        clamp_vel(&mut p);
        assert_eq!((p.vx, p.vz, p.w), (3.0, 4.0, -9.0));
    }

    #[test]
    fn a_turn_keeps_the_trig_of_an_equal_yaw() {
        // −0 equals 0: the yaw is written, its cos and sin stay
        let mut p = at_rest();
        (p.yaw, p.cs, p.sn) = (0.0, 0.25, 0.75);
        turn(&mut p, -0.0);
        assert_eq!(p.yaw.to_bits(), (-0.0f64).to_bits());
        assert_eq!((p.cs, p.sn), (0.25, 0.75));
        turn(&mut p, 0.5);
        assert_eq!(p.yaw, 0.5);
        assert_eq!(p.cs.to_bits(), m::cos(0.5).to_bits());
        assert_eq!(p.sn.to_bits(), m::sin(0.5).to_bits());
    }

    #[test]
    fn against_the_typescripts_own_bits() {
        // bun, EXACT.on: 1.7 * (5 / Math.hypot(1.7, 9.3)), 9.3 * (…), Math.cos(0.7), Math.sin(0.7)
        let mut p = at_rest();
        (p.vx, p.vz) = (1.7, 9.3);
        clamp_vel(&mut p);
        assert_eq!(p.vx.to_bits(), 0x3fecc545374c7b17);
        assert_eq!(p.vz.to_bits(), 0x4013ac8b911bd42d);
        turn(&mut p, 0.7);
        assert_eq!(p.cs.to_bits(), 0x3fe87996529f9d92);
        assert_eq!(p.sn.to_bits(), 0x3fe49d6e694619b8);
    }
}
