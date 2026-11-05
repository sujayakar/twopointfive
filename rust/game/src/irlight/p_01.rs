//! src/game/irlight.ts lines 1 to 315
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// ?irpower and ?irnear read the address, which is the host's: with no page the TypeScript does nothing, and so does this. (irlight.ts irFlags)
pub fn ir_flags(g: &mut crate::game::Game) {}

/// The kit's row, made if there is none. The TypeScript's literal holds nothing but empty lists, which is what `zeroed` holds. (irlight.ts kitOf)
pub fn kit_of(g: &mut crate::game::Game) -> &mut crate::irlight::Kit {
    g.m.irlight.kits.get_or_insert_with(Kit::zeroed)
}

/// One carrier's reading, which `burn` has made: re-got after every call that takes `g`.
fn need_row(g: &mut Game, key: f64) -> &mut Need {
    kit_of(g).need.get_mut(crate::Num(key)).unwrap()
}

/// A new illuminator: out, nowhere, the flood's shape as the knobs have it. No beam in the haze. (irlight.ts lamp)
pub fn lamp(g: &mut crate::game::Game, name: &str, owner: f64) -> LightId {
    let (range, inner_deg, outer_deg) = (g.k.irlight.reach, g.k.irlight.core, g.k.irlight.cone);
    g.tab.light.push(crate::render_lights::RtLight {
        name: String::from(name),
        group: String::from("infrared"),
        kind: 1.0,
        pos: [0.0, -10.0, 0.0],
        dir: [0.0, 0.0, 1.0],
        color: [1.0, 1.0, 1.0],
        intensity: 0.0,
        range,
        inner_deg,
        outer_deg,
        radius: LENS_R,
        volumetric: 0.0,
        owner,
        enabled: false,
        fixture_box: -1.0,
        ttl: -1.0,
        age: 0.0,
        peak_intensity: 0.0,
        ..crate::render_lights::RtLight::zeroed()
    })
}

/// One man's illuminator posed for this frame: at his brow, along his aim, tipped down. (irlight.ts pose)
pub fn pose(g: &mut crate::game::Game, l: LightId, c_2: CharId) {
    let pos = g.ch[c_2].pos;
    let yaw = g.ch[c_2].aim_yaw;
    let head = match g.ch[c_2].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(pos, [0.0, HEAD_Y, 0.0]),
    };
    let fx = m::sin(yaw);
    let fz = m::cos(yaw);
    let dn = m::sin(g.k.irlight.dip * v3::DEG);
    let fl = m::cos(g.k.irlight.dip * v3::DEG);
    // (a face against a wall or a leaf: the lens stays at the brow, this side of it)
    let blocked = crate::scene_collision::segment_blocked(g, head, [head[0] + fx * LENS_CLEAR, head[1], head[2] + fz * LENS_CLEAR], None, None, None);
    let at = if blocked { v3::copy(head) } else { [head[0] + fx * LENS_AHEAD, head[1], head[2] + fz * LENS_AHEAD] };
    let e = g.engine.settings.exposure;
    let strength = g.k.irlight.power / (if e > 0.0 { e } else { 1.0 });
    let reach = g.k.irlight.reach;
    let outer = v3::clamp(g.k.irlight.cone, 5.0, 89.0);
    let inner = m::min(g.k.irlight.core, outer - 0.5);
    let row = &mut g.tab.light[l];
    row.pos = at;
    row.dir = [fx * fl, -dn, fz * fl];
    row.intensity = strength;
    row.peak_intensity = strength;
    row.range = reach;
    row.outer_deg = outer;
    row.inner_deg = inner;
    row.enabled = true;
}

/// Is this floor too dark for the tube by its own light? (irlight.ts darkAt)
pub fn dark_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let e = crate::nvg::nvg_place_light(g, [x, 1.2, z]);
    crate::nvg::tube_vis(g, e) < g.k.nvg.black_at
}

/// Is the floor his flood would fall on too dark? A fan across the cone, each ray to the first wall; FAN_NEED of its readings must be dark. (irlight.ts fanDark)
pub fn fan_dark(g: &mut crate::game::Game, at: V3, yaw: f64) -> bool {
    let eye: V3 = [at[0], 1.5, at[2]];
    let mut n = 0.0;
    for off in fan_deg() {
        let fx = m::sin(yaw + off * v3::DEG);
        let fz = m::cos(yaw + off * v3::DEG);
        for r in fan_m() {
            let p: V3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r];
            if crate::scene_collision::segment_blocked(g, eye, p, None, None, None) {
                break;
            }
            if dark_at(g, p[0], p[2]) {
                n += 1.0;
                if n >= FAN_NEED {
                    return true;
                }
            }
        }
    }
    false
}

/// The sensors' key of one carrier's slot. (irlight.ts skey)
pub fn skey(key: f64, i: f64) -> String {
    alloc::format!("irl{}:{}", key, i)
}

/// What he asked of the renderer is forgotten, every slot. (irlight.ts forget)
pub fn forget(g: &mut crate::game::Game, key: f64) {
    let slots = irl_meter().slots;
    let mut i = 0.0;
    while i <= slots {
        crate::game::game_light_forget(g, &skey(key, i));
        i += 1.0;
    }
}

/// Every point of his fan the CPU's direct light leaves under the picture's line, in the fan's order: x, z and the CPU's light there. (irlight.ts fanDarks)
pub fn fan_darks(g: &mut crate::game::Game, at: V3, yaw: f64) -> Vec<f64> {
    let eye: V3 = [at[0], 1.5, at[2]];
    let mut out_pts: Vec<f64> = Vec::new();
    for off in fan_deg() {
        let fx = m::sin(yaw + off * v3::DEG);
        let fz = m::cos(yaw + off * v3::DEG);
        for r in fan_m() {
            let p: V3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r];
            if crate::scene_collision::segment_blocked(g, eye, p, None, None, None) {
                break;
            }
            let e = crate::nvg::nvg_place_light(g, p);
            if crate::nvg::tube_sig(g, e) < g.k.irlight.show {
                out_pts.push(p[0]);
                out_pts.push(p[2]);
                out_pts.push(e);
            }
        }
    }
    out_pts
}

/// Is a place the CPU calls dark (its direct light `cpu`) dark in the picture, by the renderer's answer? None: no answer yet. (irlight.ts shownDark)
pub fn shown_dark(g: &mut crate::game::Game, cpu: f64, e: Option<f64>) -> Option<bool> {
    let e = match e {
        Some(e) if e.is_finite() => e,
        _ => return None,
    };
    let s = g.engine.settings.indirect_scale;
    let sig = crate::nvg::tube_sig(g, cpu + (if s >= 0.0 { s } else { 1.0 }) * m::max(e - cpu, 0.0));
    Some(sig < g.k.irlight.show)
}

/// His reading is over: what he asked of the renderer is forgotten with it. (irlight.ts drop)
pub fn drop(g: &mut crate::game::Game, key: f64) {
    let asked = match kit_of(g).need.get(crate::Num(key)) {
        Some(n) => n.since.iter().any(|t| *t != f64::INFINITY),
        None => return,
    };
    if asked {
        forget(g, key);
    }
    kit_of(g).need.delete(crate::Num(key));
}

/// One slot's word: whether the renderer's answer, or the CPU's word after the wait, calls it dark; waiting sets `pending`. (irlight.ts burn: word)
fn word(g: &mut Game, cpu: f64, e: Option<f64>, since: f64, used: bool, pending: &mut bool) -> bool {
    if !used {
        return false;
    }
    if let Some(d) = shown_dark(g, cpu, e) {
        return d;
    }
    if g.time - since < irl_meter().wait {
        *pending = true;
        return false;
    }
    true
}

/// How much of his light burns this frame, 0‥1: `at` / `yaw` where his flood starts and looks, `each` the floor under him is read every frame, `live` the renderer answers and is asked, `own_key` the meter key that already reads his floor. (irlight.ts burn)
pub fn burn(g: &mut crate::game::Game, key: f64, at: V3, yaw: f64, each: bool, live: bool, own_key: Option<&str>) -> f64 {
    let k_key = crate::Num(key);
    let time = g.time;
    let had_t = kit_of(g).need.get(k_key).map(|n| n.t);
    let born = match had_t {
        None => true,
        Some(t) => t > time,
    };
    let s = irl_meter().slots as usize;
    if born {
        if had_t.is_some() {
            forget(g, key);
        }
        kit_of(g).need.set(k_key, crate::irlight::Need {
            read_t: f64::NEG_INFINITY,
            dark_t: f64::NEG_INFINITY,
            k: 0.0,
            t: time,
            fresh: true,
            t0: time,
            own: -1.0,
            pts: Vec::new(),
            since: vec![f64::INFINITY; s + 1],
        });
    }
    let beat = time - need_row(g, key).read_t >= READ_EVERY;
    let mut dark;
    let mut pending = false;
    if !live {
        // the CPU's rule, whole
        let n = need_row(g, key);
        if !n.pts.is_empty() || n.own >= 0.0 || n.since.iter().any(|t| *t != f64::INFINITY) {
            forget(g, key);
            let n = need_row(g, key);
            n.pts.clear();
            n.own = -1.0;
            n.since.fill(f64::INFINITY);
        }
        dark = each && dark_at(g, at[0], at[2]);
        if beat {
            dark = dark || (!each && dark_at(g, at[0], at[2])) || fan_dark(g, at, yaw);
        }
    } else {
        // the CPU's word: the floor under him, and on his beat the dark points of his fan, a spread of them put to the renderer
        if each || beat {
            let e = crate::nvg::nvg_place_light(g, [at[0], 1.2, at[2]]);
            let own = if crate::nvg::tube_sig(g, e) < g.k.irlight.show { e } else { -1.0 };
            need_row(g, key).own = own;
        }
        if beat {
            let d = fan_darks(g, at, yaw);
            let count = d.len() / 3;
            let take = if (count as f64) < FAN_NEED { 0 } else { core::cmp::min(count, s) };
            need_row(g, key).pts.clear();
            for i in 0..s {
                if i >= take {
                    // (a slot out of use: its answer is of another place when it is used again)
                    if need_row(g, key).since[i] != f64::INFINITY {
                        crate::game::game_light_forget(g, &skey(key, i as f64));
                        need_row(g, key).since[i] = f64::INFINITY;
                    }
                    continue;
                }
                let j = 3 * (if take == count { i } else { m::round(i as f64 * (count as f64 - 1.0) / (take as f64 - 1.0)) as usize });
                need_row(g, key).pts.extend_from_slice(&[d[j], d[j + 1], d[j + 2]]);
                crate::game::game_light_query(g, &skey(key, i as f64), [d[j], 1.2, d[j + 1]], None);
                let n = need_row(g, key);
                if n.since[i] == f64::INFINITY {
                    n.since[i] = time;
                }
            }
        }
        if own_key.is_none() && need_row(g, key).own >= 0.0 {
            crate::game::game_light_query(g, &skey(key, s as f64), [at[0], 1.2, at[2]], None);
            let n = need_row(g, key);
            if n.since[s] == f64::INFINITY {
                n.since[s] = time;
            }
        }
        // the renderer's: every key read every frame, in use or not (one read sequence a frame)
        let own = need_row(g, key).own;
        let answer = match own_key {
            Some(own) => crate::game::game_light_at(g, own),
            None => crate::game::game_light_at(g, &skey(key, s as f64)),
        };
        let n = need_row(g, key);
        let since = if own_key.is_some() { n.t0 } else { n.since[s] };
        dark = word(g, m::max(own, 0.0), answer, since, own >= 0.0, &mut pending);
        let mut hits = 0.0;
        for i in 0..s {
            let n = need_row(g, key);
            let used = 3 * i < n.pts.len();
            let cpu = if used { n.pts[3 * i + 2] } else { 0.0 };
            let since = n.since[i];
            let answer = crate::game::game_light_at(g, &skey(key, i as f64));
            if word(g, cpu, answer, since, used, &mut pending) {
                hits += 1.0;
            }
        }
        dark = dark || hits >= FAN_NEED;
    }
    let hold = g.k.irlight.hold;
    let n = need_row(g, key);
    if beat {
        // (the first reading is at once; the beat after it is his own, so five men's fans do not all fall on one frame)
        n.read_t = if born { time - js::rem(m::abs(key) * 0.6180339887, 1.0) * READ_EVERY } else { time };
    }
    if dark {
        n.dark_t = time;
    }
    let on = time - n.dark_t < hold;
    let dt = m::max(time - n.t, 0.0);
    n.t = time;
    if n.fresh {
        n.k = if on { 1.0 } else { 0.0 };
        n.fresh = pending;
    } else {
        n.k = v3::clamp(n.k + (if on { dt / FADE_IN } else { -dt / FADE_OUT }), 0.0, 1.0);
    }
    n.k
}

/// His burns exactly while the picture is the tube's. (irlight.ts samOn)
pub fn sam_on(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    g.pl[pl].nv_amount > 0.5
}

/// His lamp is out of the list; the renderer's slot memory of it goes with it. (irlight.ts out)
pub fn out(g: &mut crate::game::Game, l: Option<LightId>) {
    if let Some(l) = l {
        let row = &mut g.tab.light[l];
        row.enabled = false;
        row.slot = Some(-1.0);
    }
}

/// How much of its strength a posed light keeps for what stands in front of it: three rays across the middle of the flood, eased. (irlight.ts nearK)
pub fn near_k(g: &mut crate::game::Game, key: f64, l: LightId) -> f64 {
    let near_m = m::max(0.5, g.k.irlight.near_m);
    let mut d = near_m;
    for a in near_yaw() {
        let c = m::cos(a);
        let s = m::sin(a);
        let (pos, dir) = (g.tab.light[l].pos, g.tab.light[l].dir);
        let h = crate::scene_collision::raycast(g, pos, [dir[0] * c + dir[2] * s, dir[1], dir[2] * c - dir[0] * s], d);
        if let Some(h) = h {
            if h.t < d {
                d = h.t;
            }
        }
    }
    let want = v3::clamp((d / near_m) * (d / near_m), g.k.irlight.near_min, 1.0);
    let was = kit_of(g).near.get(crate::Num(key)).copied();
    let time = g.time;
    let mut k = match was {
        None => want,
        Some(w) => {
            let dt = time - w.t;
            if dt < 0.0 || dt > 0.25 {
                want
            } else {
                w.k + (want - w.k) * m::min(1.0, dt * (if want < w.k { g.k.irlight.near_down } else { g.k.irlight.near_up }))
            }
        }
    };
    // (there: the ease has no tail)
    if m::abs(want - k) < 0.005 * m::max(want, k) {
        k = want;
    }
    kit_of(g).near.set(crate::Num(key), crate::irlight::KitNearValue { t: time, k });
    k
}

/// (QA) how far a carrier's light stands turned down this frame: 1 not at all, or no reading of his this frame. (irlight.ts irNearOf)
pub fn ir_near_of(g: &mut crate::game::Game, key: f64) -> f64 {
    let s = g.m.irlight.kits.as_ref().and_then(|kit| kit.near.get(crate::Num(key)).copied());
    match s {
        Some(s) if s.t == g.time => s.k,
        _ => 1.0,
    }
}

/// His light posed, and in the list if any of it burns. `live` is read once a frame and only when somebody's goggles are down. (irlight.ts irLightTick: lit)
fn lit(g: &mut Game, live: &mut Option<bool>, l: LightId, key: f64, at: V3, yaw: f64, each: bool, own_key: Option<&str>) {
    let is_live = match *live {
        Some(v) => v,
        None => {
            let v = !g.k.sense.cpu && crate::game::game_light_at(g, "playerChest").is_some();
            *live = Some(v);
            v
        }
    };
    let k = burn(g, key, at, yaw, each, is_live, own_key);
    if k > 0.0 {
        let was = g.tab.light[l].intensity;
        let near = near_k(g, key, l);
        let now = was * (k * near);
        g.tab.light[l].intensity = now;
        g.tab.light[l].peak_intensity = now;
        g.engine.lights.ir.push(l);
    } else {
        out(g, Some(l));
    }
}

/// One seat's man, over that seat's view (`g` has the seat's set in): his own lamp, or the lens's while the wand is under a door. (irlight.ts irLightTick: eachSeat's closure)
fn seat_light(g: &mut Game, host: PlayerId, live: &mut Option<bool>) {
    let pl = g.player;
    let sc = g.pl[pl].ch;
    let index = g.tab.cmd_seat[g.seat].index;
    let id = g.ch[sc].id;
    let first = index == 0.0;
    let wand_key = WAND_KEY0 - index;
    if sam_on(g, pl) {
        let had = if first { kit_of(g).sam } else { kit_of(g).more.get(crate::Num(id)).copied() };
        let l = match had {
            Some(l) => l,
            None => {
                let name = if first { String::from("ir_sam") } else { alloc::format!("ir_sam{}", index) };
                let l = lamp(g, &name, id);
                if first {
                    kit_of(g).sam = Some(l);
                } else {
                    kit_of(g).more.set(crate::Num(id), l);
                }
                l
            }
        };
        pose(g, l, sc);
        // (the wand under a door, his goggles down: the flood is the lens's too, level along its look, and the place read is the one it looks into)
        let eye = crate::wand::wand_eye(g);
        if let Some(e) = &eye {
            g.tab.light[l].pos = v3::copy(e.pos);
            g.tab.light[l].dir = [e.look[0], 0.0, e.look[2]];
        }
        if let Some(e) = &eye {
            drop(g, id);
            lit(g, live, l, wand_key, [e.pos[0] + e.look[0] * 0.7, 0.0, e.pos[2] + e.look[2] * 0.7], m::atan2(e.look[0], e.look[2]), true, None);
        } else {
            drop(g, wand_key);
            let at = g.ch[sc].pos;
            let yaw = g.ch[sc].aim_yaw;
            lit(g, live, l, id, at, yaw, true, if pl == host { Some("playerChest") } else { None });
        }
    } else {
        let had = if first { kit_of(g).sam } else { kit_of(g).more.get(crate::Num(id)).copied() };
        out(g, had);
        drop(g, id);
        drop(g, wand_key);
    }
}

/// Once a frame from Game.update, after everybody has moved and the doors have swung: the list of infrared lights made again from whose goggles are down. (irlight.ts irLightTick)
pub fn ir_light_tick(g: &mut crate::game::Game) {
    g.engine.lights.ir.clear();
    kit_of(g).seen.clear();
    let mut live: Option<bool> = None;
    let host = crate::seats::host_man(g);
    on_each_seat(g, &mut |g: &mut Game| seat_light(g, host, &mut live));
    for gd in kit_of(g).men.keys() {
        if !g.guards.contains(&gd) {
            kit_of(g).men.delete(gd);
            let id = g.ch[g.gd[gd].ch].id;
            drop(g, id);
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let c = g.gd[gd].ch;
        let id = g.ch[c].id;
        if g.gd[gd].state == crate::game::GuardState::Dead || !crate::nvg::nvg_down(g, gd) {
            let had = kit_of(g).men.get(gd).copied();
            out(g, had);
            drop(g, id);
            continue;
        }
        let l = match kit_of(g).men.get(gd).copied() {
            Some(l) => l,
            None => {
                let l = lamp(g, &alloc::format!("ir_{}", id), id);
                kit_of(g).men.set(gd, l);
                l
            }
        };
        pose(g, l, c);
        let at = g.ch[c].pos;
        let yaw = g.ch[c].aim_yaw;
        // (his own meter's chest key: guards.ts meterOf)
        lit(g, &mut live, l, id, at, yaw, false, Some(&alloc::format!("unitC{}", id)));
    }
}

/// The infrared light at a point, W/m² as the light meter would report it if it read infrared. `skip`: a carrier's own light is not asked. (irlight.ts irLightAt)
pub fn ir_light_at(g: &mut crate::game::Game, p: V3, skip: Option<f64>) -> f64 {
    let mut total = 0.0;
    let mut i = 0;
    while i < g.engine.lights.ir.len() {
        let id = g.engine.lights.ir[i];
        i += 1;
        let l = &g.tab.light[id];
        if !l.enabled || l.intensity <= 0.0 {
            continue;
        }
        if let Some(s) = skip {
            if l.owner == s {
                continue;
            }
        }
        let dx = l.pos[0] - p[0];
        let dy = l.pos[1] - p[1];
        let dz = l.pos[2] - p[2];
        let d2 = dx * dx + dy * dy + dz * dz;
        let d = m::sqrt(d2);
        if d > l.range || d < 1e-3 {
            continue;
        }
        let lx = dx / d;
        let ly = dy / d;
        let lz = dz / d;
        let cd = -(lx * l.dir[0] + ly * l.dir[1] + lz * l.dir[2]);
        let c_o = m::cos(l.outer_deg * v3::DEG);
        let c_i = m::cos(l.inner_deg * v3::DEG);
        if cd < c_o {
            continue;
        }
        let x = d / l.range;
        let w = v3::clamp(1.0 - x * x * x * x, 0.0, 1.0);
        let s = v3::clamp((cd - c_o) / m::max(c_i - c_o, 1e-6), 0.0, 1.0);
        let e = l.intensity * w * w / m::max(d2, l.radius * l.radius) * s * s * (3.0 - 2.0 * s) * m::max_n(&[m::abs(lx), m::abs(ly), m::abs(lz)]);
        if e < 0.002 {
            continue;
        }
        let tmax = d - l.radius - 0.02;
        if tmax > 0.05 && crate::scene_collision::segment_blocked(g, p, [p[0] + lx * tmax, p[1] + ly * tmax, p[2] + lz * tmax], None, None, None) {
            continue;
        }
        total += e;
    }
    total
}

/// The infrared light ON a man this frame, as a goggled teammate's tube takes it; asked once a man a frame. (irlight.ts irOnMan)
pub fn ir_on_man(g: &mut crate::game::Game, o: GuardId) -> f64 {
    if g.engine.lights.ir.is_empty() {
        return 0.0;
    }
    let now = g.time;
    if let Some(had) = kit_of(g).seen.get(o).copied() {
        if had.t == now {
            return had.e;
        }
    }
    let c = g.gd[o].ch;
    let pos = g.ch[c].pos;
    let at = match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(b) => *b,
        None => [pos[0], pos[1] + 1.2, pos[2]],
    };
    let id = g.ch[c].id;
    let e = ir_light_at(g, at, Some(id));
    kit_of(g).seen.set(o, crate::irlight::KitSeenValue { t: now, e });
    e
}

/// Would his own illuminator show him a standing man at `p`, if he looked that way with a clear line? (irlight.ts irShowsAt)
pub fn ir_shows_at(g: &mut crate::game::Game, m: GuardId, p: V3) -> bool {
    let at = g.ch[g.gd[m].ch].pos;
    let dx = p[0] - at[0];
    let dz = p[2] - at[2];
    let d2 = m::max(dx * dx + dz * dz, 0.25);
    let d = m::sqrt(d2);
    let reach = g.k.irlight.reach;
    if d >= reach {
        return false;
    }
    // (his light comes on for dark floor his fan finds, and the fan ends there: past it he is credited with nothing)
    if d > LOOK_MAX {
        return false;
    }
    let x = d / reach;
    let w = v3::clamp(1.0 - x * x * x * x, 0.0, 1.0);
    let s = g.engine.settings.exposure;
    let v = crate::nvg::tube_vis(g, g.k.irlight.power / (if s > 0.0 { s } else { 1.0 }) * w * w / d2);
    v >= g.k.nvg.black_at
}

/// He carries an illuminator of his own: both of its rows are settled on on the page. (irlight.ts irCarried)
pub fn ir_carried(g: &mut crate::game::Game) -> bool {
    true
}

/// QA / the panel: whose illuminator burns. (irlight.ts irLightState)
pub fn ir_light_state(g: &mut crate::game::Game) -> crate::irlight::IrLightState {
    let host = crate::seats::host_man(g);
    let shown = g.pl[host].nv_amount > 0.5;
    let list = &g.engine.lights.ir;
    let (sam, men) = match &g.m.irlight.kits {
        Some(kit) => (
            kit.sam.map_or(false, |l| list.contains(&l)),
            kit.men.iter().filter(|(_, l)| list.contains(*l)).map(|(gd, _)| gd).collect(),
        ),
        None => (false, Vec::new()),
    };
    crate::irlight::IrLightState { sam, men, shown }
}

/// 'IR light: yours on · Blue One · Blue Two — only goggles see it'. (irlight.ts irLightSummary)
pub fn ir_light_summary(g: &mut crate::game::Game) -> String {
    let st = ir_light_state(g);
    let yours = if st.sam { "on" } else if st.shown { "off (light enough here)" } else { "off (goggles up)" };
    let mut team = String::new();
    if st.men.is_empty() {
        team.push_str(" · no teammate in the dark");
    } else {
        for gd in st.men {
            team.push_str(" · ");
            team.push_str(&g.gd[gd].callsign);
        }
    }
    alloc::format!("IR light: yours {}{} — only goggles see it", yours, team)
}

/// eachSeat(g, f) over one view: seats::each_seat's closure takes two `&mut Game` and cannot be lent, so its one body is here, through with_seat. (seats.ts eachSeat)
fn on_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    if g.seats.len() == 1 {
        f(g);
        return;
    }
    let mut i = 0;
    while i < g.seats.len() {
        let s = g.seats[i];
        i += 1;
        let man = crate::cmd::seat_man(g, s);
        crate::seats::with_seat(g, man, &mut *f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a key is a number written as JavaScript writes it: no ".0", the minus kept (the sensors' journal lists these strings)
    #[test]
    fn skey_writes_numbers_as_the_typescript_does() {
        assert_eq!(skey(-1.0, 5.0), "irl-1:5");
        assert_eq!(skey(42.0, 0.0), "irl42:0");
    }

    fn test_light(pos: V3, dir: V3, intensity: f64, radius: f64, owner: f64, range: f64, outer_deg: f64, inner_deg: f64, enabled: bool) -> crate::render_lights::RtLight {
        crate::render_lights::RtLight { pos, dir, intensity, radius, owner, range, outer_deg, inner_deg, enabled, ..crate::render_lights::RtLight::zeroed() }
    }

    /// irLightAt's sums against the TypeScript's own bits (irlight.ts under bun, EXACT on). Every lens stands too near the point to ask the world (tmax <= 0.05).
    #[test]
    fn ir_light_at_has_the_typescripts_bits() {
        let mut g = Game::blank();
        let lights = [
            test_light([1.25, 1.62, -0.75], [0.17364817766693033, -0.17364817766693033, 0.9698463103929542], 3.7, 0.04, 7.0, 40.0, 62.0, 37.0, true),
            test_light([1.2, 1.58, -0.72], [0.3, -0.6, 0.7416198487095663], 0.91, 0.0625, 9.0, 0.5, 62.0, 37.0, true),
            test_light([1.28, 1.6, -0.74], [0.1, -0.2, 0.9746794344808963], 12.5, 0.04, 11.0, 40.0, 88.0, 5.0, true),
            test_light([1.21, 1.6, -0.7], [0.0, 0.0, 1.0], 3.7, 0.04, 13.0, 40.0, 62.0, 37.0, false),
        ];
        for l in lights {
            let id = g.tab.light.push(l);
            g.engine.lights.ir.push(id);
        }
        let p: V3 = [1.23, 1.56, -0.69];
        assert_eq!(ir_light_at(&mut g, p, None).to_bits(), 0x4093b2e004f20ff5);
        assert_eq!(ir_light_at(&mut g, p, Some(7.0)).to_bits(), 0x408d9d4ecc9f6b9f);
        assert_eq!(ir_light_at(&mut g, p, Some(9.0)).to_bits(), 0x40915ef5e44d0b06);
        assert_eq!(ir_light_at(&mut g, p, Some(11.0)).to_bits(), 0x407ce08afd1d7c50);
    }

    /// a new lamp holds the TypeScript literal's values and the knobs' shape; `out` writes the row it is given, and with no lamp does nothing
    #[test]
    fn lamp_and_out_write_the_row() {
        let mut g = Game::blank();
        let l = lamp(&mut g, "ir_sam", 42.0);
        let (reach, core, cone) = (g.k.irlight.reach, g.k.irlight.core, g.k.irlight.cone);
        {
            let r = &g.tab.light[l];
            assert_eq!((r.name.as_str(), r.group.as_str(), r.kind, r.owner), ("ir_sam", "infrared", 1.0, 42.0));
            assert_eq!((r.pos, r.dir, r.color), ([0.0, -10.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0]));
            assert_eq!((r.range, r.inner_deg, r.outer_deg, r.radius), (reach, core, cone, LENS_R));
            assert_eq!((r.fixture_box, r.ttl, r.enabled, r.slot), (-1.0, -1.0, false, None));
        }
        g.tab.light[l].enabled = true;
        out(&mut g, Some(l));
        assert_eq!((g.tab.light[l].enabled, g.tab.light[l].slot), (false, Some(-1.0)));
        out(&mut g, None);
    }

    /// a write through kit_of lands in the game; a reading is good for the frame it was made in only
    #[test]
    fn kit_of_is_the_row() {
        let mut g = Game::blank();
        assert!(g.m.irlight.kits.is_none());
        let now = g.time;
        kit_of(&mut g).near.set(crate::Num(5.0), KitNearValue { t: now, k: 0.25 });
        assert_eq!(ir_near_of(&mut g, 5.0), 0.25);
        assert_eq!(ir_near_of(&mut g, 6.0), 1.0);
        g.time = now + 1.0;
        assert_eq!(ir_near_of(&mut g, 5.0), 1.0);
    }
}
