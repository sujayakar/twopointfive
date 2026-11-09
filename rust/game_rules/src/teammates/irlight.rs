pub use ::game_types::irlight::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::string::String;

/// ?irpower and ?irnear read the address, which is the host's: with no page the TypeScript does nothing, and so does this. (irlight.ts irFlags)
pub fn ir_flags(g: &crate::game::Game) {}

/// The kit's row, made if there is none. The TypeScript's literal holds nothing but empty lists, which is what `zeroed` holds. (irlight.ts kitOf)
pub fn kit_of(g: &mut crate::game::Game) -> &mut crate::irlight::Kit {
    g.m.irlight.kits.get_or_insert_with(Kit::default)
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
        flicker: 0.0,
        ttl: -1.0,
        age: 0.0,
        peak_intensity: 0.0,
        ..crate::render_lights::RtLight::default()
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
    // (the game's own exposure for the hour: never a player's Brightness)
    let strength = g.k.irlight.power / crate::hours::exposure_of(g);
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

/// Does the tube make too little of this floor by its own light? By its picture (`show`), not by what shows nothing at all: between the two a man is neither in a flood nor plain without one.
pub fn dark_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let e = crate::nvg::nvg_place_light(g, [x, 1.2, z]);
    crate::nvg::tube_sig(g, e) < g.k.irlight.show
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

/// How much of his light burns this frame, 0‥1: `at` / `yaw` where his flood starts and looks, `each` the floor under him is read every frame and not on his beat alone. (irlight.ts burn)
pub fn burn(g: &mut crate::game::Game, key: f64, at: V3, yaw: f64, each: bool) -> f64 {
    let k_key = crate::Num(key);
    let time = g.time;
    let born = match kit_of(g).need.get(k_key).map(|n| n.t) {
        None => true,
        Some(t) => t > time,
    };
    if born {
        kit_of(g).need.set(k_key, crate::irlight::Need { read_t: f64::NEG_INFINITY, dark_t: f64::NEG_INFINITY, k: 0.0, t: time, fresh: true });
    }
    let beat = time - need_row(g, key).read_t >= READ_EVERY;
    let mut dark = each && dark_at(g, at[0], at[2]);
    if beat {
        dark = dark || (!each && dark_at(g, at[0], at[2])) || fan_dark(g, at, yaw);
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
        n.fresh = false;
    } else {
        n.k = v3::clamp(n.k + (if on { dt / FADE_IN } else { -dt / FADE_OUT }), 0.0, 1.0);
    }
    n.k
}

/// His burns exactly while the picture is the tube's. (irlight.ts samOn)
pub fn sam_on(g: &crate::game::Game, pl: PlayerId) -> bool {
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
pub fn ir_near_of(g: &crate::game::Game, key: f64) -> f64 {
    let s = g.m.irlight.kits.as_ref().and_then(|kit| kit.near.get(crate::Num(key)).copied());
    match s {
        Some(s) if s.t == g.time => s.k,
        _ => 1.0,
    }
}

/// His reading is over. (irlight.ts drop)
pub fn drop(g: &mut crate::game::Game, key: f64) {
    kit_of(g).need.delete(crate::Num(key));
}

/// His light posed, and in the list if any of it burns. (irlight.ts irLightTick: lit)
fn lit(g: &mut Game, l: LightId, key: f64, at: V3, yaw: f64, each: bool) {
    let k = burn(g, key, at, yaw, each);
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
fn seat_light(g: &mut Game) {
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
        let eye = crate::wand::wand_eye(g, &crate::seats::here(g));
        if let Some(e) = &eye {
            g.tab.light[l].pos = v3::copy(e.pos);
            g.tab.light[l].dir = [e.look[0], 0.0, e.look[2]];
        }
        if let Some(e) = &eye {
            drop(g, id);
            lit(g, l, wand_key, [e.pos[0] + e.look[0] * 0.7, 0.0, e.pos[2] + e.look[2] * 0.7], m::atan2(e.look[0], e.look[2]), true);
        } else {
            drop(g, wand_key);
            let at = g.ch[sc].pos;
            let yaw = g.ch[sc].aim_yaw;
            lit(g, l, id, at, yaw, true);
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
    crate::seats::each_seat(g, &mut |g: &mut Game| seat_light(g));
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
        lit(g, l, id, at, yaw, false);
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
pub fn ir_shows_at(g: &crate::game::Game, m: GuardId, p: V3) -> bool {
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
    let power = g.k.irlight.power;
    let e = power / crate::hours::exposure_of(g) * w * w / d2;
    let v = crate::nvg::tube_vis(g, e);
    v >= g.k.nvg.black_at
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_light(pos: V3, dir: V3, intensity: f64, radius: f64, owner: f64, range: f64, outer_deg: f64, inner_deg: f64, enabled: bool) -> crate::render_lights::RtLight {
        crate::render_lights::RtLight { pos, dir, intensity, radius, owner, range, outer_deg, inner_deg, enabled, ..crate::render_lights::RtLight::default() }
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
