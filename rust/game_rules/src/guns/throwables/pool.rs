use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Closest hit among statics + door leaves and props.
fn throwables_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::throwables::ThrowablesRay> {
    let st = crate::scene_collision::raycast(g, ro, rd, tmax);
    let ph = crate::props::prop_system_raycast(g, ro, rd, match &st { Some(s) => s.t, None => tmax });
    if let Some(ph) = ph {
        return Some(ThrowablesRay { t: ph.t });
    }
    st.map(|s| ThrowablesRay { t: s.t })
}

/// A new thing in the pool: the dice in the order they were always drawn and before anything is given up for it; `opts` over the rest.
pub fn throwables_spawn(g: &mut crate::game::Game, kind: crate::throwables::ItemKind, pos: V3, vel: V3, opts: Option<crate::throwables::ThrowablesSpawnOpts>) -> ItemId {
    let opts = opts.unwrap_or_default();
    let can = kind == ItemKind::Smoke || kind == ItemKind::Flash;
    let (rifle, ring, frag, shell) = (kind == ItemKind::Rifle, kind == ItemKind::Ring, kind == ItemKind::Frag, kind == ItemKind::Shell);
    let (c, r, rg, f, s) = (can_item(), rifle_item(), ring_item(), frag_item(), shell_item());
    let radius = if can { c.radius } else if frag { f.radius } else if kind == ItemKind::Pistol { 0.05 } else if kind == ItemKind::Torch { 0.045 } else if rifle { r.radius } else if ring { rg.radius } else if shell { s.radius } else { 0.03 };
    let restitution = if can { c.restitution } else if frag { f.restitution } else if kind == ItemKind::Mag { 0.25 } else if rifle { r.restitution } else if ring { rg.restitution } else if shell { s.restitution } else { 0.22 };
    let friction = if can { c.friction } else if kind == ItemKind::Pistol || kind == ItemKind::Torch { 0.5 } else if rifle { r.friction } else if ring { rg.friction } else if frag { f.friction } else if shell { s.friction } else { 0.35 };
    // the dice, one at a time and in this order
    let yaw = crate::dice::draw_world(g, "throwables::throwables_spawn#1") * m::PI;
    let spin = (crate::dice::draw_world(g, "throwables::throwables_spawn#2") - 0.5) * 14.0;
    let tumble = crate::dice::draw_world(g, "throwables::throwables_spawn#3") * 6.28;
    let way = if crate::dice::draw_world(g, "throwables::throwables_spawn#4") < 0.5 { -1.0 } else { 1.0 };
    let tumble_rate = way * (7.0 + crate::dice::draw_world(g, "throwables::throwables_spawn#5") * 9.0);
    let life = if kind == ItemKind::Mag { 40.0 } else if ring { rg.life } else if shell { s.life } else { 60.0 };
    let heading = v3::yaw_of(vel);
    let serial = g.items.serial;
    g.items.serial += 1.0;
    // FIFO cap, before the push so the new item can never evict itself; a can on its fuse or venting is never evicted: spent things first, then the oldest
    // dropped gun or torch, and past ITEM_ROWS the oldest whatever it is.
    if g.items.items.len() as f64 >= ITEM_CAP {
        let mut i = first_spent(g);
        if i.is_none() {
            i = first_not_live(g);
        }
        if let Some(i) = i {
            throwables_remove_at(g, i as f64);
        }
        if g.items.items.len() as f64 >= ITEM_ROWS {
            throwables_remove_at(g, 0.0);
        }
    }
    let mut it = Item::blank();
    it.kind = opts.kind.unwrap_or(kind);
    it.pos = opts.pos.unwrap_or(pos);
    it.vel = opts.vel.unwrap_or(vel);
    it.radius = opts.radius.unwrap_or(radius);
    it.restitution = opts.restitution.unwrap_or(restitution);
    it.friction = opts.friction.unwrap_or(friction);
    it.yaw = opts.yaw.unwrap_or(yaw);
    it.spin = opts.spin.unwrap_or(spin);
    it.tumble = opts.tumble.unwrap_or(tumble);
    it.tumble_rate = opts.tumble_rate.unwrap_or(tumble_rate);
    it.settled = opts.settled.unwrap_or(false);
    it.age = opts.age.unwrap_or(0.0);
    it.fuse = opts.fuse.unwrap_or(1.3);
    it.fired = opts.fired.unwrap_or(false);
    it.life = opts.life.unwrap_or(life);
    it.emit_t = opts.emit_t.unwrap_or(0.0);
    it.fill = opts.fill.unwrap_or(0.0);
    it.bounces = opts.bounces.unwrap_or(0.0);
    it.heading = opts.heading.unwrap_or(heading);
    it.who = opts.who.unwrap_or(0.0);
    it.on_pose = opts.on_pose.unwrap_or(0.0);
    it.serial = opts.serial.unwrap_or(serial);
    it.carried = opts.carried;
    it.side = opts.side;
    it.clatters = opts.clatters;
    let it = g.tab.item.push(it);
    item_set_on(g, it, opts.on);
    g.items.items.push(it);
    it
}

/// The first spent thing in the list: a magazine, a ring, a dud, a can that has gone off and finished.
fn first_spent(g: &mut Game) -> Option<usize> {
    for i in 0..g.items.items.len() {
        let x = g.items.items[i];
        let k = g.tab.item[x].kind;
        if k == ItemKind::Mag || k == ItemKind::Ring || k == ItemKind::Shell || ((k == ItemKind::Smoke || k == ItemKind::Flash || k == ItemKind::Frag) && !live_can(g, x)) {
            return Some(i);
        }
    }
    None
}

/// The first thing in the list that is no live can.
fn first_not_live(g: &mut Game) -> Option<usize> {
    for i in 0..g.items.items.len() {
        let x = g.items.items[i];
        if !live_can(g, x) {
            return Some(i);
        }
    }
    None
}

/// An item leaves by the pool's own rule.
pub fn throwables_remove_at(g: &mut crate::game::Game, i: f64) {
    g.items.items.remove(i as usize);
}

/// Launch velocity from `from` to land at `to` with a pleasant arc; also the sampled preview points until first impact.
pub fn throwables_solve(g: &mut crate::game::Game, from: V3, to: V3, max_speed: Option<f64>) -> crate::throwables::ThrowSolution {
    let max_speed = max_speed.unwrap_or(11.0);
    let d = v3::sub(to, from);
    let dist = m::hyp2(d[0], d[2]);
    let mut t = m::min(1.25, m::max(0.42, dist / 7.5));
    let mut v0: V3 = [d[0] / t, (d[1] + 0.5 * GRAVITY * t * t) / t, d[2] / t];
    let sp = v3::len(v0);
    if sp > max_speed {
        v0 = v3::scale(v0, max_speed / sp);
    }
    // preview: integrate until first hit
    let mut points: Vec<V3> = Vec::new();
    let mut p = v3::copy(from);
    let mut v = v3::copy(v0);
    let mut land = v3::copy(to);
    let h = 1.0 / 30.0;
    for i in 0..90 {
        points.push(v3::copy(p));
        let np: V3 = [p[0] + v[0] * h, p[1] + v[1] * h - 0.5 * GRAVITY * h * h, p[2] + v[2] * h];
        let seg = v3::sub(np, p);
        let len = v3::len(seg);
        if len > 1e-5 {
            if let Some(hit) = throwables_ray(g, p, v3::scale(seg, 1.0 / len), len + 0.05) {
                land = v3::mad(p, seg, hit.t / len);
                points.push(land);
                break;
            }
        }
        if np[1] < 0.02 {
            land = [np[0], 0.02, np[2]];
            points.push(land);
            break;
        }
        v = [v[0], v[1] - GRAVITY * h, v[2]];
        p = np;
        t = i as f64 * h;
    }
    ThrowSolution { v0, t, points, land }
}

const GRAVITY_NOW: f64 = 9.81;
/// substeps of a tick
const SUB: f64 = 2.0;

use ::sim::world::World;
/// what the walk asks of the world, and what it knows of it
struct Walk<'a> {
    w: &'a World,
    snow: Option<::sim::snow::Snow>,
    rifle_share: f64,
    tune: &'a crate::knobs::Knobs,
}
/// What a step did that is not its own numbers, in the order it happened, to be done when the walk is back. None is read by a later step: furniture is met by where it stands, which an impulse does not move.
enum Told {
    Bounce { it: ItemId, speed: f64, at: V3, n: V3 },
    Impulse { it: ItemId, thing: usize, at: V3, j: V3 },
    Gone { it: ItemId },
}
/// a thing of furniture's pose as one number: any change of pose changes it
fn pose_key(o: &World, id: usize) -> f64 {
    use ::sim::world::{TH, TH_X, TH_YAW, TH_Z};
    o.thing[id * TH + TH_X] * 7919.0 + o.thing[id * TH + TH_Z] * 104729.0 + o.thing[id * TH + TH_YAW]
}
/// the furniture along a ray: how far, the normal, and the id of the thing met
fn things(w: &World, p: V3, d: V3, tmax: f64) -> Option<(f64, V3, usize)> {
    let mut out = [0.0f64; 5];
    let r = ::sim::lines::ray_things(w, p[0], p[1], p[2], d[0], d[1], d[2], tmax, &mut out);
    if r < 0 { return None; }
    Some((out[0], [out[1], out[2], out[3]], w.things[r as usize].id as usize))
}
/// the closest hit among the statics, the leaves and the furniture: how far, the face's normal, and the id of the thing of furniture plus one (0: a wall, a floor, a leaf)
fn hit_of(d: &Walk, p: V3, dir: V3, tmax: f64) -> Option<(f64, V3, usize)> {
    let mut out = [0.0f64; 4];
    let k = ::sim::lines::ray_rule(d.w, p[0], p[1], p[2], dir[0], dir[1], dir[2], tmax, -1, true, &mut out, d.tune);
    if k == ::sim::lines::RAY_HOST { panic!("a ray that is no number, from a thrown thing at {:?}", p); }
    let st = if k >= 0 { Some((out[0], [out[1], out[2], out[3]])) } else { None };
    if let Some((t, n, id)) = things(d.w, p, dir, st.map_or(tmax, |s| s.0)) { return Some((t, n, id + 1)); }
    st.map(|(t, n)| (t, n, 0))
}
fn bog_at(d: &Walk, x: f64, y: f64, z: f64) -> f64 {
    match &d.snow { Some(s) => ::sim::snow::bog_of(d.w, s, x, y, z), None => 0.0 }
}
/// One thing's step: it ages, falls, meets the walls, the leaves and the furniture by a ray from its middle, bounces, hands the furniture a share of what it lost, and comes to rest on a floor or on a chair's seat; lying on a thing that has moved, it looks under itself and drops.
fn step(d: &Walk, id: ItemId, it: &mut Item, dt: f64, told: &mut Vec<Told>) {
    it.age += dt;
    it.life -= dt;
    if it.life <= 0.0 {
        told.push(Told::Gone { it: id });
        return;
    }
    if it.carried == Some(true) {   // (in a hand: it goes where the hand goes)
        return;
    }
    if it.settled && lies(d, it, dt) {
        return;
    }
    let h = dt / SUB;
    let mut s = 0.0;
    while s < SUB && !it.settled {
        s += 1.0;
        flies(d, id, it, h, told);
    }
}

/// Settled, it lies along its axis; on a chair seat or a carton it stays put while that thing is still underneath. False: it drops.
fn lies(d: &Walk, it: &mut Item, dt: f64) -> bool {
    use ::sim::maths::round_half_up;
    use ::sim::world::{js_min, MAX_THINGS};
    it.tumble_rate = 0.0;
    let flat_a = round_half_up(it.tumble / m::PI) * m::PI;
    it.tumble += (flat_a - it.tumble) * js_min(1.0, dt * 12.0);   // (0 or half a turn)
    let on = it.on as usize;
    if on == 0 || on > MAX_THINGS || pose_key(d.w, on - 1) == it.on_pose {
        return true;
    }
    let under = things(d.w, [it.pos[0], it.pos[1] + 0.05, it.pos[2]], [0.0, -1.0, 0.0], 0.15);
    if under.is_some_and(|(_, _, thing)| thing + 1 == on) {
        it.on_pose = pose_key(d.w, on - 1);
        return true;
    }
    it.settled = false;
    it.on = 0;
    it.vel = [0.0, 0.0, 0.0];
    false
}

/// A substep in the air: on, or into what is in the way of it.
fn flies(d: &Walk, id: ItemId, it: &mut Item, h: f64, told: &mut Vec<Told>) {
    use ::sim::maths::hypot3;
    it.vel[1] -= GRAVITY_NOW * h;
    let mv = [it.vel[0] * h, it.vel[1] * h, it.vel[2] * h];
    let len = hypot3(mv[0], mv[1], mv[2]);
    if len < 1e-6 { return; }
    let il = 1.0 / len;
    let dir = [mv[0] * il, mv[1] * il, mv[2] * il];
    match meets(d, it, dir, len) {
        Some((t, n, thing)) => {
            it.pos = [it.pos[0] + dir[0] * t, it.pos[1] + dir[1] * t, it.pos[2] + dir[2] * t];
            bounces(d, id, it, n, thing, told);
            it.pos = [it.pos[0] + n[0] * 0.002, it.pos[1] + n[1] * 0.002, it.pos[2] + n[2] * 0.002];
        }
        None => it.pos = [it.pos[0] + mv[0], it.pos[1] + mv[1], it.pos[2] + mv[2]],
    }
    if it.pos[1] < it.radius { on_the_ground(d, id, it, told); }
    it.yaw += it.spin * h;
    it.tumble += it.tumble_rate * h;
}

/// What it meets in the next `len`, and how far on it is at contact: a sphere against a box from the centre's ray, so contact is radius/cos(incidence) before the centre would hit.
fn meets(d: &Walk, it: &Item, dir: V3, len: f64) -> Option<(f64, V3, usize)> {
    use ::sim::world::js_max;
    let (t, n, thing) = hit_of(d, it.pos, dir, len + it.radius * 3.5)?;
    let back = it.radius / js_max(0.3, -(dir[0] * n[0] + dir[1] * n[1] + dir[2] * n[2]));
    if t - back > len { return None; }
    Some((js_max(0.0, t - back), n, thing))
}

/// Into a face whose normal is `n`: what it struck takes its share, it comes off with what is left, and slow enough on a floor it settles.
fn bounces(d: &Walk, id: ItemId, it: &mut Item, n: V3, thing: usize, told: &mut Vec<Told>) {
    use ::sim::maths::{atan2, hypot2};
    use ::sim::world::{js_abs, js_max};
    let vn = it.vel[0] * n[0] + it.vel[1] * n[1] + it.vel[2] * n[2];
    if vn >= 0.0 { return; }
    if thing != 0 {
        // the thing's lost momentum goes into the chair or carton it struck
        let j = vn * (1.0 + it.restitution) * share_of(d, it.kind);
        told.push(Told::Impulse { it: id, thing: thing - 1, at: it.pos, j: [n[0] * j, n[1] * j, n[2] * j] });
    }
    let (vn0, vn1, vn2) = (n[0] * vn, n[1] * vn, n[2] * vn);
    let (vt0, vt1, vt2) = (it.vel[0] - vn0, it.vel[1] - vn1, it.vel[2] - vn2);
    // lying snow under it (0 = none)
    let floor = n[1] > 0.6;
    let bog = if floor && thing == 0 { bog_at(d, it.pos[0], it.pos[1] - it.radius, it.pos[2]) } else { 0.0 };
    let f = js_max(0.0, 1.0 - it.friction * (0.5 + 0.5 * js_abs(n[1])));
    it.vel = [vt0 * f + vn0 * -it.restitution, vt1 * f + vn1 * -it.restitution, vt2 * f + vn2 * -it.restitution];
    if bog > 0.0 {
        // … it keeps little of its slide and of its bounce
        let sn = d.snow.as_ref().unwrap();
        let up = it.vel[0] * n[0] + it.vel[1] * n[1] + it.vel[2] * n[2];
        let (b, sl) = (1.0 - bog * sn.dead, 1.0 - bog * sn.grip);
        it.vel = [(it.vel[0] - n[0] * up) * sl + n[0] * (up * b), (it.vel[1] - n[1] * up) * sl + n[1] * (up * b), (it.vel[2] - n[2] * up) * sl + n[2] * (up * b)];
    }
    it.spin *= 0.6;
    it.tumble_rate *= 0.45;
    if hypot2(it.vel[0], it.vel[2]) > 0.3 { it.heading = atan2(it.vel[0], it.vel[2]); }
    struck(id, it, -vn, n, told);
    if floor && slow(d, it, bog) {
        settles(it);
        it.on = thing as u32;
        it.on_pose = if thing != 0 { pose_key(d.w, thing - 1) } else { 0.0 };
        it.spin = 0.0;
        it.pos[1] += 0.001;
    }
}

/// The safety floor: nothing goes below it.
fn on_the_ground(d: &Walk, id: ItemId, it: &mut Item, told: &mut Vec<Told>) {
    let bog = bog_at(d, it.pos[0], 0.0, it.pos[2]);
    it.pos[1] = it.radius;
    if it.vel[1] < 0.0 {
        let sp = -it.vel[1];
        it.vel[1] *= -it.restitution;
        it.vel[0] *= 0.7;
        it.vel[2] *= 0.7;
        if bog > 0.0 {
            let sn = d.snow.as_ref().unwrap();
            it.vel[1] *= 1.0 - bog * sn.dead;
            it.vel[0] *= 1.0 - bog * sn.grip;
            it.vel[2] *= 1.0 - bog * sn.grip;
        }
        struck(id, it, sp, [0.0, 1.0, 0.0], told);
    }
    if slow(d, it, bog) { settles(it); }
}

/// The share of what it lost that a thing of furniture takes.
fn share_of(d: &Walk, kind: ItemKind) -> f64 {
    match kind {
        ItemKind::Smoke | ItemKind::Frag => 0.35,
        ItemKind::Rifle => d.rifle_share,
        _ => 0.1,
    }
}

/// A bounce counted, and said if it was hard enough to hear.
fn struck(id: ItemId, it: &mut Item, speed: f64, n: V3, told: &mut Vec<Told>) {
    it.bounces += 1.0;
    if speed > 0.6 { told.push(Told::Bounce { it: id, speed, at: it.pos, n }); }
}

/// Slow enough to lie where it is; on lying snow a faster one sticks.
fn slow(d: &Walk, it: &Item, bog: f64) -> bool {
    let stick = match &d.snow { Some(s) if bog > 0.0 => bog * s.stick, _ => 0.0 };
    ::sim::world::js_abs(it.vel[1]) < 0.5 && ::sim::maths::hypot2(it.vel[0], it.vel[2]) < 0.35 + stick
}

fn settles(it: &mut Item) {
    it.settled = true;
    it.vel = [0.0, 0.0, 0.0];
}

/// One tick of the pool: it walks from the newest down, then what the steps told is done in order.
pub fn throwables_update(g: &mut crate::game::Game, dt: f64) {
    if g.items.items.is_empty() { return; }
    let snow = if crate::snow_bog::snow_bog_may(g) {
        // lying snow: the level's table in the box world, the rule's numbers and the roofs in force
        let roofs = &g.weather.roof_rects;
        debug_assert!(roofs.len() <= ::sim::snow::MAX_ROOFS, "more roofs over the snow than a walk is handed");
        let k = &g.k.snowbog;
        let mut roof = [0.0; ::sim::snow::MAX_ROOFS * 4];
        for (i, r) in roofs.iter().enumerate() { roof[i * 4..i * 4 + 4].copy_from_slice(&r[..4]); }
        Some(::sim::snow::Snow { top: k.top, bank: k.bank_m, packed: k.packed, old: k.old, dead: k.dead, grip: k.grip, stick: k.stick, roofs: roofs.len(), roof })
    } else { None };
    let d = Walk { w: g.tab.boxes.world(), snow, rifle_share: rifle_item().prop_share, tune: &g.k };
    let mut told = Vec::new();
    for &id in g.items.items.iter().rev() { step(&d, id, &mut g.tab.item[id], dt, &mut told); }
    throwables_say(g, told);
}

/// What the steps told, done in the order they told it: a bounce said by the item's own callback, a share of momentum to a thing of furniture, an item whose time is up taken out.
fn throwables_say(g: &mut crate::game::Game, told: Vec<Told>) {
    for t in told {
        match t {
            Told::Gone { it } => { if let Some(i) = g.items.items.iter().position(|&x| x == it) { throwables_remove_at(g, i as f64); } }
            Told::Impulse { it, thing, at, j } => {
                let prop = g.props.all[thing];
                let who = g.tab.item[it].who;
                crate::props::prop_system_apply_impulse(g, prop, at, j, who, None);
            }
            Told::Bounce { it, speed, at, n } => { if let Some(b) = g.tab.item[it].on_bounce.clone() { item_bounced(g, b, speed, at, it, n); } }
        }
    }
}

/// `it.onBounce(sp, p, g, it, n)`: what each arm stands for, as its setter wrote it.
fn item_bounced(g: &mut Game, b: ItemBounce, sp: f64, p: V3, it: ItemId, n: V3) {
    match b {
        // doortools, tactics, squadthrow and the rest: the can's knock
        ItemBounce::Can => crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(1.0, sp / 4.0)), None),
        // the player's own throw, and the room hears it
        ItemBounce::CanHeard => {
            crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(1.0, sp / 4.0)), None);
            let e = crate::game::GameEvent { kind: crate::director::EventKind::Shot, pos: v3::copy(p), time: g.time, loud: false, serial: g.serial(), ..Default::default() };
            g.events.push(e);
        }
        ItemBounce::RingSkitter => {
            if sp > 1.2 {
                crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(0.5, sp / 12.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.7 }));
            }
        }
        // he40.ts:123
        ItemBounce::ShellSkitter => {
            if sp > 1.0 {
                crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(p), Some(m::min(0.7, sp / 8.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.8 }));
            }
        }
        ItemBounce::Mag => crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0)), None),
        // player.ts:382, guards.ts:2782
        ItemBounce::MagRate { belt } => crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0)), Some(&crate::audio_iface::PlayOpts::Rate { rate: if belt { 0.6 } else { 0.8 } })),
        // combat.ts:423 dropFromHand; who 0: nobody living made it, a clatter the room cannot explain
        ItemBounce::Dropped { kind, quiet } => {
            if sp < 0.7 {
                return;
            }
            let rate = if kind == ItemKind::Pistol { 0.72 } else if kind == ItemKind::Rifle { 0.6 } else { 0.85 };
            crate::audio_iface::game_audio_like_play(g, "magDrop", Some(p), Some(m::min(1.0, sp / 3.0) * (if quiet { 0.5 } else { 1.0 })), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
            if !quiet {
                let c = g.tab.item[it].clatters.unwrap_or(0.0) + 1.0;
                g.tab.item[it].clatters = Some(c);
                if c <= 2.0 {
                    let e = crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: v3::copy(p), time: g.time, loud: false, level: Some(m::min(0.5, 0.2 + sp * 0.08)), who: Some(0.0), serial: g.serial(), ..Default::default() };
                    g.events.push(e);
                }
            }
        }
        ItemBounce::GunDown => crate::disarm::gun_down(g, sp, p, it, n),
        // disturbed after a load, it clatters as any
        ItemBounce::Loaded { kind } => {
            if sp >= 0.7 {
                let name = if kind == ItemKind::Smoke || kind == ItemKind::Flash { "canisterBounce" } else { "magDrop" };
                crate::audio_iface::game_audio_like_play(g, name, Some(p), Some(m::min(1.0, sp / 3.0)), None);
            }
        }
    }
}

/// Fill 4 to 7's magazines: half length, half width, half thickness, colour.
const OWN_MAGS: [(f64, f64, f64, V3); 4] = [
    (0.11, 0.016, 0.012, [0.06, 0.06, 0.065]),
    (0.06, 0.045, 0.035, [0.16, 0.17, 0.1]),
    (0.065, 0.014, 0.012, [0.03, 0.03, 0.033]),
    (0.1, 0.011, 0.012, [0.03, 0.03, 0.033]),
];

/// A thrown thing's parts as boxes into this frame's list, each keyed off its serial.
pub fn throwables_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let item_key = crate::scene_boxes::mkey().item;
    for i in 0..g.items.items.len() {
        let it = Shown::of(g, g.items.items[i]);
        let n0 = out.len();
        match it.kind {
            ItemKind::Smoke | ItemKind::Flash => can_boxes(g, out, &it),
            ItemKind::Frag => frag_boxes(g, out, &it),
            ItemKind::Pistol | ItemKind::Torch | ItemKind::Rifle => gun_boxes(g, out, &it),
            // GEAR.on, settled true on the page: the 40 mm sponge round
            ItemKind::Ring => sponge_boxes(g, out, it.middle(sponge_item().wide, 0.002), if it.settled { v3::quat::yaw(it.yaw) } else { it.flying }, it.yaw),
            ItemKind::Shell => shell_boxes(g, out, &it),
            ItemKind::Mag => mag_boxes(g, out, &it),
        }
        // motion identity per part: the g-buffer reprojects the flying can through its own last transform, whatever spawned or died around it in the list
        for (k, b) in out[n0..].iter().enumerate() {
            g.tab.edit(*b).mkey = Some(item_key + (js::to_i32(it.serial) & 8191) as f64 * 8.0 + m::min(7.0, k as f64));
        }
    }
}

/// A thing of the pool as its boxes are laid out: its row, read once.
struct Shown {
    kind: ItemKind,
    p: V3,
    settled: bool,
    fired: bool,
    yaw: f64,
    tumble: f64,
    radius: f64,
    fill: f64,
    serial: f64,
    /// How it is turned in the air: end over end.
    flying: Quat,
}

impl Shown {
    fn of(g: &Game, id: ItemId) -> Shown {
        let it = &g.tab.item[id];
        let flying = v3::quat::mul(v3::quat::yaw(it.yaw), v3::quat::axis_angle([1.0, 0.0, 0.0], it.tumble));
        Shown { kind: it.kind, p: it.pos, settled: it.settled, fired: it.fired, yaw: it.yaw, tumble: it.tumble, radius: it.radius, fill: it.fill, serial: it.serial, flying }
    }
    /// Its middle: where it is, or lying, half its thickness off the floor and a hair `over`.
    fn middle(&self, half: f64, over: f64) -> V3 {
        if self.settled { [self.p[0], self.p[1] - self.radius + half + over, self.p[2]] } else { self.p }
    }
    /// A part of it, `off` its middle `c` along the way it is turned.
    fn part(&self, g: &mut Game, out: &mut Vec<AnyBoxId>, c: V3, off: V3, h: V3, albedo: V3, flags: f64) {
        out.push(item_box(g, v3::add(c, v3::quat::rotate(self.flying, off)), h, self.yaw, self.flying, albedo, flags).into());
    }
}

/// The flags of a part, and of one that casts no shadow.
fn part_flags() -> [f64; 2] {
    let flag = crate::scene_boxes::box_flag();
    [flag.dynamic, (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64]
}

fn can_boxes(g: &mut Game, out: &mut Vec<AnyBoxId>, it: &Shown) {
    let [dy, dn] = part_flags();
    let (body, band): (V3, V3) = match (it.kind, it.fired) {
        (ItemKind::Flash, true) => ([0.03, 0.03, 0.03], [0.12, 0.11, 0.1]),
        (ItemKind::Flash, false) => ([0.07, 0.075, 0.08], [0.7, 0.7, 0.66]),
        _ => ([0.25, 0.29, 0.2], [0.6, 0.08, 0.06]),
    };
    let c = it.middle(0.034, 0.0);
    out.push(item_box(g, c, [0.032, 0.032, 0.07], it.yaw, it.flying, body, dy).into());
    it.part(g, out, c, [0.0, 0.0, 0.045], [0.034, 0.034, 0.012], band, dn);
}

fn frag_boxes(g: &mut Game, out: &mut Vec<AnyBoxId>, it: &Shown) {
    let [dy, dn] = part_flags();
    let f = frag_item();
    let c = it.middle(f.half[0], 0.002);
    out.push(item_box(g, c, f.half, it.yaw, it.flying, f.body, dy).into());
    it.part(g, out, c, [0.0, 0.0, 0.012], [0.034, 0.034, 0.006], f.band, dn);
    it.part(g, out, c, [0.0, 0.0, 0.05], [0.012, 0.012, 0.014], f.fuze, dn);
}

fn shell_boxes(g: &mut Game, out: &mut Vec<AnyBoxId>, it: &Shown) {
    let [dy, dn] = part_flags();
    let s = shell_item();
    let c = it.middle(s.half[0], 0.002);
    out.push(item_box(g, c, s.half, it.yaw, it.flying, s.body, dy).into());
    it.part(g, out, c, [0.0, 0.0, s.half[2] * 0.62], [s.half[0] + 0.001, s.half[1] + 0.001, s.half[2] * 0.4], s.nose, dn);
}

/// A gun or a torch: in the air it flips end over end; settled, it is on its side with that face on the floor.
fn gun_boxes(g: &mut Game, out: &mut Vec<AnyBoxId>, it: &Shown) {
    let x_axis: V3 = [1.0, 0.0, 0.0];
    let [dy, dn] = part_flags();
    let fn2000 = it.fill == 1.0;
    let thick = match it.kind {
        ItemKind::Pistol => 0.016,
        ItemKind::Rifle if fn2000 => sc20_k_item().thick,
        ItemKind::Rifle => rifle_item().thick,
        _ => 0.022,
    };
    let yaw = it.yaw;
    let rot = if it.settled {
        v3::quat::mul(v3::quat::mul(v3::quat::yaw(yaw), v3::quat::axis_angle(x_axis, m::round(it.tumble / m::PI) * m::PI)), v3::quat::axis_angle([0.0, 0.0, 1.0], m::PI / 2.0))
    } else {
        it.flying
    };
    let c = it.middle(thick, 0.004);
    let at = |off: V3| -> V3 { v3::add(c, v3::quat::rotate(rot, off)) };
    let mut part = |c: V3, h: V3, rot: Quat, albedo: V3, flags: f64| out.push(item_box(g, c, h, yaw, rot, albedo, flags).into());
    match it.kind {
        ItemKind::Pistol => {
            part(at([0.0, 0.0, 0.0]), [0.016, 0.026, 0.095], rot, [0.05, 0.05, 0.055], dy);
            let grip = v3::quat::mul(rot, v3::quat::axis_angle(x_axis, 108.0 * m::PI / 180.0));
            part(v3::add(at([0.0, -0.024, -0.045]), v3::quat::rotate(grip, [0.0, 0.0, 0.047])), [0.014, 0.017, 0.047], grip, [0.05, 0.05, 0.055], dy);
            if it.fill > 0.0 { part(at([0.0, 0.0, 0.17]), [0.019, 0.019, 0.075], rot, [0.09, 0.09, 0.09], dy); }
        }
        ItemKind::Rifle if fn2000 => {
            let s = m4_item();
            part(at([0.0, 0.0, s.hull.z]), s.hull.h, rot, s.polymer, dy);
            part(at([0.0, 0.0, s.fore.z]), s.fore.h, rot, s.fore_col, dy);
            part(at([0.0, s.mag.y, s.mag.z]), s.mag.h, rot, s.metal, dy);
            part(at([0.0, s.optic.y, s.optic.z]), s.optic.h, rot, s.metal, dn);
        }
        ItemKind::Rifle => {
            let r = rifle_item();
            part(at([0.0, 0.0, r.receiver.z]), r.receiver.h, rot, r.steel, dy);
            part(at([0.0, 0.0, r.fore.z]), r.fore.h, rot, r.wood, dy);
            let mag = v3::quat::mul(rot, v3::quat::axis_angle(x_axis, r.mag.cant));
            part(at([0.0, r.mag.y, r.mag.z]), r.mag.h, mag, r.wood, dy);
            part(at([0.0, -0.012, r.stock.z]), r.stock.h, rot, r.steel, dn);
        }
        _ => {
            part(at([0.0, 0.0, 0.0]), [0.022, 0.022, 0.09], rot, [0.09, 0.09, 0.1], dy);
            part(at([0.0, 0.0, 0.095]), [0.026, 0.026, 0.008], rot, [0.16, 0.16, 0.15], dn);
        }
    }
}

/// A magazine. fill 2: a rifleman's empty thirty; 3: the FN2000's black thirty; 4 to 7: OWN_MAGS; else a pistol's (0 empty, 0 to 1 part full).
fn mag_boxes(g: &mut Game, out: &mut Vec<AnyBoxId>, it: &Shown) {
    let [_, dn] = part_flags();
    let fill = it.fill;
    let (len, wide, thick, col) = if fill >= 4.0 {
        OWN_MAGS[m::min(3.0, fill - 4.0) as usize]
    } else if fill >= 3.0 {
        (0.095, 0.014, 0.012, sc20_k_item().metal)
    } else if fill > 1.0 {
        (0.095, 0.014, 0.012, rifle_item().wood)
    } else {
        (0.055, 0.018, 0.012, if fill > 0.0 { [0.09, 0.09, 0.1] } else { [0.06, 0.06, 0.065] })
    };
    if it.settled {
        let p = it.p;
        let b = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP { c: [p[0], p[1] + thick, p[2]], h: [wide, thick, len], yaw: Some(it.yaw), albedo: Some(col), flags: Some(dn), ..Default::default() });
        out.push(b.into());
    } else {
        out.push(item_box(g, it.p, [wide, len, m::max(0.03, thick)], it.yaw, it.flying, col, dn).into());
    }
}

/// A canister that is still something: its fuse not yet gone, or (smoke) gone and still venting.
fn live_can(g: &mut crate::game::Game, x: ItemId) -> bool {
    let k = g.tab.item[x].kind;
    (k == ItemKind::Smoke || k == ItemKind::Flash || k == ItemKind::Frag) && (!g.tab.item[x].fired || g.tab.item[x].emit_t > 0.0)
}
