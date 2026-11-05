//! src/game/wind.ts lines 1 to 334
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// STILL A GAP, wind.ts:259,303: `gpuFx` hands the pool back by value and HostApi holds no `particles`, so `P.setWind` has nothing to write to.
fn seam_particles_set_wind(g: &mut Game, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) { todo!("needs: gpu_fx to give the pool itself, or particles_set_wind to take g (wind.ts:261,303)") }
/// STILL A GAP, wind.ts:328: `(g.smoke as { stats? }).stats`; SmokeSystemLike has no stats. None = the solver is off.
fn seam_smoke_wind_srcs(g: &mut Game) -> Option<Option<f64>> { todo!("needs: SmokeSystemLike.stats.windSrcs (wind.ts:328)") }

/// Hand back `v`, and write it to the caller's `out` where one was given: the TypeScript's every path fills all three.
fn put(out: Option<&mut V3>, v: V3) -> V3 {
    if let Some(o) = out {
        *o = v;
    }
    v
}

/// The master and the rows as this machine last had them: with no browser storage in the game this is the path the TypeScript takes under bun. (wind.ts loadWind)
pub fn load_wind(g: &mut crate::game::Game, tune: &mut crate::wind::WindTune) {
    tune.wind = Some(g.k.wind.def);
}

/// Nothing of the game's: the whole body is the host's browser storage. (wind.ts saveWind)
pub fn save_wind(g: &mut crate::game::Game) {}

/// The rows back to the shipped values; the storage's half is the host's. (wind.ts resetWindKnobs)
pub fn reset_wind_knobs(g: &mut crate::game::Game) {
    g.k.wind = wind_defaults();
    g.tune.wind = g.k.wind.def;
    g.tune.moths = true;
}

/// The master as the systems read it, clamped 0‥1. (wind.ts windOf)
pub fn wind_of(g: &mut crate::game::Game) -> f64 {
    m::max(0.0, m::min(1.0, g.tune.wind))
}

/// (wind.ts smoothstep)
pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = m::max(0.0, m::min(1.0, (x - a) / (b - a)));
    t * t * (3.0 - 2.0 * t)
}

/// (wind.ts inRect)
pub fn in_rect(x: f64, z: f64, r: &[f64]) -> bool {
    x > r[0] && z > r[1] && x < r[2] && z < r[3]
}

/// A diffuser's air at p, m/s: a jet from `pos` along unit `dir`, turning to run along a floor it is blown at. (wind.ts jetVel)
pub fn jet_vel(pos: V3, dir: V3, speed: f64, reach: f64, tan_spread: f64, r0: f64, d_max: f64, floor_y: f64, p: V3, out: Option<&mut V3>) -> V3 {
    let wf = wind_field();
    let zero: V3 = [0.0, 0.0, 0.0];
    if !(speed > 0.0) {
        return put(out, zero);
    }
    let qx = p[0] - pos[0];
    let qy = p[1] - pos[1];
    let qz = p[2] - pos[2];
    let d = qx * dir[0] + qy * dir[1] + qz * dir[2];
    if d <= 0.0 || d >= d_max {
        return put(out, zero);
    }
    let rx = qx - dir[0] * d;
    let ry = qy - dir[1] * d;
    let rz = qz - dir[2] * d;
    let r = m::hyp3(rx, ry, rz);
    let sigma = r0 + d * tan_spread;
    if r > wf.support_sigma * sigma {
        return put(out, zero);
    }
    let x = d / m::max(reach, 1e-3);
    let rs = r / sigma;
    let mut s = speed * m::exp(-rs * rs) / (1.0 + x * x) * (1.0 - smoothstep(0.7 * d_max, d_max, d));
    let ap = d + r0 / m::max(tan_spread, 0.02);
    let mut ux = dir[0] * ap + rx;
    let mut uy = dir[1] * ap + ry;
    let mut uz = dir[2] * ap + rz;
    {
        let l = js::or(m::hyp3(ux, uy, uz), 1.0);
        ux /= l;
        uy /= l;
        uz /= l;
    }
    if floor_y > -100.0 && dir[1] < -0.5 {
        let h = p[1] - floor_y;
        if h < 0.0 {
            return put(out, zero);
        }
        if h < wf.deflect_h {
            let t = 1.0 - h / wf.deflect_h;
            let mut hx = rx;
            let mut hz = rz;
            let hl = m::hyp2(hx, hz);
            if hl > 1e-3 {
                hx /= hl;
                hz /= hl;
            } else {
                hx = dir[2] + 0.37;
                hz = 0.29 - dir[0];
                let l = m::hyp2(hx, hz);
                hx /= l;
                hz /= l;
            }
            let k = t * t;
            let mx = ux + (hx - ux) * k;
            let my = uy + (0.0 - uy) * k;
            let mz = uz + (hz - uz) * k;
            let ml = js::or(m::hyp3(mx, my, mz), 1.0);
            ux = mx / ml;
            uy = my / ml;
            uz = mz / ml;
            s *= 1.0 - wf.deflect_loss * t;
        }
    }
    put(out, [ux * s, uy * s, uz * s])
}

/// The storm's air at p: `wind` outdoors, nothing inside the two footprints or over stormTop. (wind.ts stormVel)
pub fn storm_vel(wind: V3, interior: &[f64], annex: &[f64], p: V3, out: Option<&mut V3>) -> V3 {
    let wf = wind_field();
    if p[1] > wf.storm_top || p[1] < 0.0 || in_rect(p[0], p[2], interior) || in_rect(p[0], p[2], annex) {
        return put(out, [0.0, 0.0, 0.0]);
    }
    let d_out = m::max(m::max(interior[0] - p[0], p[0] - interior[2]), m::max(interior[1] - p[2], p[2] - interior[3]));
    let k = smoothstep(0.0, wf.storm_lee, d_out) * (wf.storm_low + wf.storm_gain * m::max(0.0, m::min(1.0, p[1] / wf.storm_full_at)));
    put(out, [wind[0] * k, wind[1] * k, wind[2] * k])
}

/// Metres (x, z) stands outside roof R along its nearer axis; a footprint with no width is far away. (wind.ts roofOut)
pub fn roof_out(x: f64, z: f64, r: &[f64]) -> f64 {
    if !(r[2] > r[0]) || !(r[3] > r[1]) {
        1e6
    } else {
        m::max(m::max(r[0] - x, x - r[2]), m::max(r[1] - z, z - r[3]))
    }
}

/// The storm's air roof by roof: `wind` wherever none of `roofs` (≤ 4) is overhead, easing in off the nearest. (wind.ts stormVelRoofs)
pub fn storm_vel_roofs(wind: V3, roofs: &[Vec<f64>], p: V3, out: Option<&mut V3>) -> V3 {
    let wf = wind_field();
    if p[1] > wf.storm_top || p[1] < 0.0 {
        return put(out, [0.0, 0.0, 0.0]);
    }
    let mut d_out = 1e6;
    let mut i = 0;
    while i < roofs.len() && i < 4 {
        d_out = m::min(d_out, roof_out(p[0], p[2], &roofs[i]));
        i += 1;
    }
    if d_out <= 0.0 {
        return put(out, [0.0, 0.0, 0.0]);
    }
    let k = smoothstep(0.0, wf.storm_lee, d_out) * (wf.storm_low + wf.storm_gain * m::max(0.0, m::min(1.0, p[1] / wf.storm_full_at)));
    put(out, [wind[0] * k, wind[1] * k, wind[2] * k])
}

/// Noise off the clock: a hash of a 32-bit word. (wind.ts hashU)
pub fn hash_u(x: f64) -> f64 {
    let mut u = (js::to_i32(x) ^ (0x9E3779B9u32 as i32)) as u32;
    u = m::imul(u as i32, 0x85EBCA6Bu32 as i32) as u32;
    u ^= u >> 13;
    u = m::imul(u as i32, 0xC2B2AE35u32 as i32) as u32;
    u ^= u >> 16;
    u as f64
}

/// (wind.ts hash01)
pub fn hash01(x: f64) -> f64 {
    (js::to_u32(hash_u(x)) >> 8) as f64 / 16777216.0
}

/// Smooth value noise in [−1, 1] at x, one lattice value per integer, per seed. (wind.ts vnoise)
pub fn vnoise(seed: f64, x: f64) -> f64 {
    let i = m::floor(x);
    let f = x - i;
    let mult = js::to_i32(2654435761.0);
    let a = hash01((js::to_i32(seed) ^ m::imul(js::to_i32(i), mult)) as f64) * 2.0 - 1.0;
    let b = hash01((js::to_i32(seed) ^ m::imul(js::to_i32(i + 1.0), mult)) as f64) * 2.0 - 1.0;
    let t = f * f * (3.0 - 2.0 * f);
    a + (b - a) * t
}

/// Whether a vent's own cycle has it blowing at game time `t`. (wind.ts ventDue)
pub fn vent_due(v: &crate::scene_level::VentDef, t: f64) -> bool {
    let period = match v.period {
        Some(p) if p > 0.0 => p,
        _ => return true,
    };
    let duty = match v.duty {
        Some(d) => d,
        None => crate::scene_level_data::vent_defaults().duty,
    };
    let ph = js::rem(js::rem((t - v.phase.unwrap_or(0.0)) / period, 1.0) + 1.0, 1.0);
    !(ph > duty)
}

/// What went to the particles this frame; a copy of the list of IDs, the one reader (the panel's view) only reads it. (wind.ts Wind.forParticles)
pub fn wind_for_particles(g: &mut crate::game::Game) -> Vec<WindWindSourceId> {
    g.wind.to_pool.list.clone()
}

/// Every source the level can have, built once. `reroof` is reached with this Wind stood at `g.wind` for the call. (wind.ts Wind.constructor)
pub fn wind_new(g: &mut crate::game::Game) -> crate::wind::Wind {
    let wf = wind_field();
    let mut w = crate::wind::Wind::blank();
    let vents = g.level.vents.clone().unwrap_or_default();
    for v in vents {
        let dir = v3::normalize(v.dir);
        let size = match v.size {
            Some(s) => s,
            None => crate::scene_level_data::vent_defaults().size,
        };
        let d_max = wf.cut_k * v.reach;
        let tan_spread = m::tan(m::max(0.0, m::min(89.0, v.spread)) * DEG);
        let r0 = wf.core_k * size;
        let bc = v3::mad(v.pos, dir, d_max / 2.0);
        let br = m::hyp2(d_max / 2.0, r0 + d_max * tan_spread) * 1.05 + 0.3;
        let seed = crate::math_dice::hash_str(&v.name);
        let id = g.tab.wind_wind_source.push(WindSource {
            why: WindWhy::Vent,
            base: v.speed,
            on: 1.0,
            gust: 1.0,
            seed,
            door: None,
            kind: WindSourceKind::Jet,
            name: v.name.clone(),
            pos: v3::copy(v.pos),
            dir,
            speed: 0.0,
            reach: v.reach,
            tan_spread,
            r0,
            d_max,
            floor_y: Some(0.0),
            interior: None,
            annex: None,
            roofs: None,
            field: None,
            bound: crate::smoke_types::WindGpuSourceBound { c: bc, r: br },
            vent: Some(v),
        });
        w.all.push(id);
    }
    let interior = crate::weather::interior();
    let annex = crate::weather::annex();
    let cx = (interior[0] + interior[2]) / 2.0;
    let cz = (interior[1] + interior[3]) / 2.0;
    w.storm = g.tab.wind_wind_source.push(WindSource {
        why: WindWhy::Storm,
        base: 0.0,
        on: 1.0,
        gust: 1.0,
        seed: crate::math_dice::hash_str("storm"),
        vent: None,
        door: None,
        kind: WindSourceKind::Storm,
        name: String::from("storm"),
        pos: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
        speed: 0.0,
        reach: 0.0,
        tan_spread: 0.0,
        r0: 0.0,
        d_max: 0.0,
        floor_y: None,
        interior: Some(interior),
        annex: Some(annex),
        roofs: None,
        field: None,
        bound: crate::smoke_types::WindGpuSourceBound { c: [cx, 1.0, cz], r: 1e6 },
    });
    w.all.push(w.storm);
    let doors = g.doors.list.clone();
    for d in doors {
        if !g.dr[d].def.exterior.unwrap_or(false) {
            continue;
        }
        let cd = g.dr[d].def.closed_dir;
        let n1: V3 = [-m::sin(cd), 0.0, m::cos(cd)];
        let fc = g.dr[d].frame_centre;
        let inward: V3 = if in_rect(fc[0] + n1[0], fc[2] + n1[2], &interior) { n1 } else { [-n1[0], 0.0, -n1[2]] };
        if in_rect(fc[0] - inward[0], fc[2] - inward[2], &annex) {
            w.annexed.push(d);
            continue;
        }
        let s = wind_door_source(g, d, inward);
        w.all.push(s);
    }
    if crate::weather::weather_level_roofs(g) {
        let roofs: Vec<Vec<f64>> = g.weather.roofs.iter().map(|r| r.to_vec()).collect();
        let own = crate::weather::weather_own_storm(g);
        let before = core::mem::replace(&mut g.wind, w);
        wind_reroof(g, Some(&roofs), Some(own));
        w = core::mem::replace(&mut g.wind, before);
    }
    w
}

/// The gust through an exterior door: a jet from just outside the frame, chest height, along `inward`. (wind.ts Wind.doorSource)
pub fn wind_door_source(g: &mut crate::game::Game, d: DoorId, inward: V3) -> WindWindSourceId {
    let fc = g.dr[d].frame_centre;
    let reach = g.k.wind.door_reach;
    let d_max = wind_field().cut_k * reach;
    let tan_spread = m::tan(g.k.wind.door_spread_deg * DEG);
    let r0 = g.dr[d].def.width * 0.4;
    let pos: V3 = [fc[0] - inward[0] * 0.15, 1.05, fc[2] - inward[2] * 0.15];
    let name = alloc::format!("door:{}", g.dr[d].def.name);
    let seed = crate::math_dice::hash_str(&name);
    g.tab.wind_wind_source.push(WindSource {
        why: WindWhy::Door,
        base: 0.0,
        on: 1.0,
        gust: 1.0,
        seed,
        vent: None,
        door: Some(d),
        kind: WindSourceKind::Jet,
        name,
        pos,
        dir: inward,
        speed: 0.0,
        reach,
        tan_spread,
        r0,
        d_max,
        floor_y: None,
        interior: None,
        annex: None,
        roofs: None,
        field: None,
        bound: crate::smoke_types::WindGpuSourceBound {
            c: v3::mad(pos, inward, d_max / 2.0),
            r: m::hyp2(d_max / 2.0, r0 + d_max * tan_spread) + 0.3,
        },
    })
}

/// The record a source goes to both consumers as. (WindSource extends WindGpuSource in the TypeScript)
fn gpu_of(s: &WindSource) -> crate::smoke_types::WindGpuSource {
    crate::smoke_types::WindGpuSource {
        kind: match s.kind {
            WindSourceKind::Jet => crate::smoke_types::WindGpuSourceKind::Jet,
            WindSourceKind::Storm => crate::smoke_types::WindGpuSourceKind::Storm,
        },
        name: s.name.clone(),
        pos: s.pos,
        dir: s.dir,
        speed: s.speed,
        reach: s.reach,
        tan_spread: s.tan_spread,
        r0: s.r0,
        d_max: s.d_max,
        floor_y: s.floor_y,
        interior: s.interior,
        annex: s.annex,
        roofs: s.roofs.clone(),
        field: s.field,
        bound: s.bound,
    }
}

/// wind.ts:189 in1, :197 under: is (x, z) under any of the roofs (edges in)?
fn under_any(roofs: &[Vec<f64>], x: f64, z: f64) -> bool {
    roofs.iter().any(|r| x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3])
}

/// Weather kinds: the footprints the storm's air keeps out of, and which way every exterior door's gust blows. `None` = the office's two. (wind.ts Wind.reroof)
pub fn wind_reroof(g: &mut crate::game::Game, roofs: Option<&[Vec<f64>]>, own: Option<bool>) {
    // weather.ts:229 `wind?.reroof`: before the constructor there is no Wind; here a blank one, whose `storm` names no row. A real one holds at least the storm.
    if g.wind.all.is_empty() {
        return;
    }
    let own = own.unwrap_or(false);
    let none: [f64; 4] = [1e6, 1e6, -1e6, -1e6];
    if !g.wind.extra.is_empty() {
        let extra = core::mem::take(&mut g.wind.extra);
        for s in extra {
            if let Some(i) = g.wind.all.iter().position(|a| *a == s) {
                g.wind.all.remove(i);
            }
        }
    }
    if let (true, Some(rs)) = (own, roofs) {
        let annexed = g.wind.annexed.clone();
        for d in annexed {
            let cd = g.dr[d].def.closed_dir;
            let nx = -m::sin(cd);
            let nz = m::cos(cd);
            let fc = g.dr[d].frame_centre;
            if under_any(rs, fc[0] + nx, fc[2] + nz) == under_any(rs, fc[0] - nx, fc[2] - nz) {
                continue;
            }
            let s = wind_door_source(g, d, [nx, 0.0, nz]);
            g.wind.extra.push(s);
            g.wind.all.push(s);
        }
    }
    let i: [f64; 4] = match roofs {
        None => crate::weather::interior(),
        Some(rs) if rs.is_empty() => none,
        Some(rs) if rs.len() <= 2 => [rs[0][0], rs[0][1], rs[0][2], rs[0][3]],
        Some(rs) => {
            let c = |k: usize| -> Vec<f64> { rs.iter().map(|r| r[k]).collect() };
            [m::min_n(&c(0)), m::min_n(&c(1)), m::max_n(&c(2)), m::max_n(&c(3))]
        }
    };
    let a: [f64; 4] = match roofs {
        None => crate::weather::annex(),
        Some(rs) if rs.len() == 2 => [rs[1][0], rs[1][1], rs[1][2], rs[1][3]],
        Some(_) => none,
    };
    let st = g.wind.storm;
    g.tab.wind_wind_source[st].interior = Some(i);
    g.tab.wind_wind_source[st].annex = Some(a);
    g.wind.lv_roofs = roofs.map(|rs| rs.iter().map(|r| vec![r[0], r[1], r[2], r[3]]).collect());
    wind_sync_field(g);
    for k in 0..g.wind.all.len() {
        let id = g.wind.all[k];
        let d = match (g.tab.wind_wind_source[id].why, g.tab.wind_wind_source[id].door) {
            (WindWhy::Door, Some(d)) => d,
            _ => continue,
        };
        let cd = g.dr[d].def.closed_dir;
        let n1: V3 = [-m::sin(cd), 0.0, m::cos(cd)];
        let fc = g.dr[d].frame_centre;
        let mut inward: V3 = if in_rect(fc[0] + n1[0], fc[2] + n1[2], &crate::weather::interior()) { n1 } else { [-n1[0], 0.0, -n1[2]] };
        if let Some(rs) = roofs {
            let a = under_any(rs, fc[0] + n1[0], fc[2] + n1[2]);
            let b = under_any(rs, fc[0] - n1[0], fc[2] - n1[2]);
            if a != b {
                inward = if a { n1 } else { [-n1[0], 0.0, -n1[2]] };
            }
        }
        let s = &mut g.tab.wind_wind_source[id];
        s.dir = inward;
        s.pos = [fc[0] - inward[0] * 0.15, 1.05, fc[2] - inward[2] * 0.15];
        s.bound = crate::smoke_types::WindGpuSourceBound {
            c: v3::mad(s.pos, inward, s.d_max / 2.0),
            r: m::hyp2(s.d_max / 2.0, s.r0 + s.d_max * s.tan_spread) + 0.3,
        };
    }
}

/// The storm record says whether the one field outdoors is on, and carries the roofs it blows between. (wind.ts Wind.syncField)
pub fn wind_sync_field(g: &mut crate::game::Game) {
    let st = g.wind.storm;
    g.tab.wind_wind_source[st].field = if g.k.wx_own.wind { Some(true) } else { None };
    g.tab.wind_wind_source[st].roofs = wind_field_roofs(g);
}

/// The roofs the storm blows between one by one, where the switch is up and the level has three or four of its own. (wind.ts Wind.fieldRoofs)
pub fn wind_field_roofs(g: &mut crate::game::Game) -> Option<Vec<Vec<f64>>> {
    if !g.k.wx_own.wind {
        return None;
    }
    match &g.wind.lv_roofs {
        Some(r) if r.len() > 2 => Some(r.clone()),
        _ => None,
    }
}

/// The share 0‥1 of its own vector the storm's field has at ground point (x, z) under the roofs in force. (wind.ts Wind.stormShare)
pub fn wind_storm_share(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    let roofs = g.tab.wind_wind_source[g.wind.storm].roofs.clone();
    wind_share_under(g, x, z, roofs.as_deref())
}

/// …under the roofs given; none = the storm record's two rectangles. (wind.ts Wind.shareUnder)
pub fn wind_share_under(g: &mut crate::game::Game, x: f64, z: f64, roofs: Option<&[Vec<f64>]>) -> f64 {
    let p: V3 = [x, 0.0, z];
    let q = match roofs {
        Some(rs) => storm_vel_roofs(unit_x(), rs, p, None),
        None => {
            let s = &g.tab.wind_wind_source[g.wind.storm];
            storm_vel(unit_x(), &s.interior.unwrap(), &s.annex.unwrap(), p, None)
        }
    };
    m::max(0.0, m::min(1.0, q[0] / wind_field().storm_low))
}

/// The wind the game's own smoke drifts by, or none in still air: no machine's settings and no gust in it. (wind.ts Wind.stormDrift)
pub fn wind_storm_drift(g: &mut crate::game::Game) -> Option<crate::scene_collision::GridWind> {
    if !wind_blows(g, false) {
        return None;
    }
    let x = g.weather.wind[0];
    let z = g.weather.wind[1];
    if x == 0.0 && z == 0.0 {
        return None;
    }
    let roofs = wind_field_roofs(g);
    let wf = wind_field();
    let share = crate::scene_collision::GridWindShare::Under { roofs };
    Some(crate::scene_collision::GridWind { x, z, low: wf.storm_low, gain: wf.storm_gain, full_at: wf.storm_full_at, share })
}

/// Does the weather blow at all; `session` adds the weather's own switch, which the game's drift does not ask. (wind.ts Wind.blows)
pub fn wind_blows(g: &mut crate::game::Game, session: bool) -> bool {
    if session && !crate::weather::weather_enabled(g) {
        return false;
    }
    if crate::lighting::is_daylight(g) && g.weather.kind.is_none() {
        return false;
    }
    g.weather.kind.is_some() || g.weather.rain_rate > 0.001 || g.weather.storm > 0.01
}

/// The storm's wind vector when it should be felt, none otherwise. (wind.ts Wind.stormWind)
pub fn wind_storm_wind(g: &mut crate::game::Game) -> Option<V3> {
    if !wind_blows(g, true) {
        return None;
    }
    let wx = g.weather.wind[0] * g.k.wind.storm_k;
    let wz = g.weather.wind[1] * g.k.wind.storm_k;
    if !(m::hyp2(wx, wz) >= 0.05) {
        return None;
    }
    Some([wx, 0.0, wz])
}

/// Once a frame: every source's speed for this frame, the active list, and the hand-off to both GPU consumers. (wind.ts Wind.update)
pub fn wind_update(g: &mut crate::game::Game, dt: f64) {
    let master = wind_of(g);
    let pool_on = crate::particle_fx::gpu_fx(g).is_some();
    let relax = g.k.wind.smoke_relax;
    g.wind.active.clear();
    g.wind.to_pool.list.clear();
    if master <= 0.0 {
        if pool_on {
            seam_particles_set_wind(g, &[], knobs0());
        }
        crate::smoke_types::smoke_system_like_set_wind(g, &[], relax);
        return;
    }
    g.stats.wind.frames += 1;
    wind_sync_field(g);
    let t = g.time;
    let sw = wind_storm_wind(g);
    let storm_speed = match sw {
        Some(w) => m::hyp2(w[0], w[2]),
        None => 0.0,
    };
    if sw.is_some() {
        g.stats.wind.storm_frames += 1;
    }
    let vent_k = g.k.wind.vent_k;
    let ramp_up = g.k.wind.ramp_up;
    let ramp_down = g.k.wind.ramp_down;
    let gust_amp = g.k.wind.gust_amp;
    let gust_period = g.k.wind.gust_period;
    let storm_gust_amp = g.k.wind.storm_gust_amp;
    let storm_gust_period = g.k.wind.storm_gust_period;
    for k in 0..g.wind.all.len() {
        let id = g.wind.all[k];
        let base;
        let seed = g.tab.wind_wind_source[id].seed;
        match g.tab.wind_wind_source[id].why {
            WindWhy::Vent => {
                let (speed, due, mains) = {
                    let v = g.tab.wind_wind_source[id].vent.as_ref().unwrap();
                    (v.speed, vent_due(v, t), v.mains.unwrap_or(crate::scene_level_data::vent_defaults().mains))
                };
                base = speed * vent_k;
                let mut target = 1.0;
                if !due {
                    target = 0.0;
                }
                if mains && crate::sim_lightshome::mains_active(g) {
                    target = 0.0;
                }
                let s = &mut g.tab.wind_wind_source[id];
                let rate = if target > s.on { 1.0 / m::max(0.1, ramp_up) } else { 1.0 / m::max(0.1, ramp_down) };
                s.on += m::max(-rate * dt, m::min(rate * dt, target - s.on));
                s.gust = 1.0 + gust_amp * vnoise(seed, t / gust_period);
            }
            WindWhy::Storm => {
                let s = &mut g.tab.wind_wind_source[id];
                let w = match sw {
                    Some(w) => w,
                    None => {
                        s.speed = 0.0;
                        s.base = 0.0;
                        continue;
                    }
                };
                base = storm_speed;
                s.dir = [w[0] / storm_speed, 0.0, w[2] / storm_speed];
                s.on = 1.0;
                s.gust = 1.0 + storm_gust_amp * vnoise(seed, t / storm_gust_period);
            }
            WindWhy::Glass => {
                let s = &mut g.tab.wind_wind_source[id];
                base = s.base;
                s.on = 1.0;
                s.gust = 1.0 + gust_amp * vnoise(seed, t / (0.6 * gust_period));
            }
            WindWhy::Door => {
                let d = g.tab.wind_wind_source[id].door.unwrap();
                let w = match sw {
                    Some(w) => w,
                    None => {
                        let s = &mut g.tab.wind_wind_source[id];
                        s.speed = 0.0;
                        s.base = 0.0;
                        continue;
                    }
                };
                let open = m::min(1.0, m::abs(crate::doors::door_angle(g, d)) / g.k.wind.door_open_rad);
                if open < 0.04 {
                    let s = &mut g.tab.wind_wind_source[id];
                    s.speed = 0.0;
                    s.base = 0.0;
                    continue;
                }
                let door_k = g.k.wind.door_k;
                let s = &mut g.tab.wind_wind_source[id];
                let facing = m::max(0.0, (w[0] * s.dir[0] + w[2] * s.dir[2]) / storm_speed);
                base = storm_speed * door_k * open * (0.35 + 0.65 * facing);
                s.on = 1.0;
                s.gust = 1.0 + storm_gust_amp * vnoise(seed, t / storm_gust_period);
                g.stats.wind.door_gusts += 1;
            }
        }
        let s = &mut g.tab.wind_wind_source[id];
        s.base = base;
        s.speed = base * s.on * s.gust * master;
        if s.speed > 0.005 {
            g.wind.active.push(id);
        } else {
            s.speed = 0.0;
        }
    }
    g.stats.wind.active += g.wind.active.len() as i32;
    if pool_on {
        let f = match g.engine.view_focus {
            Some(f) => f,
            None => {
                let c = g.pl[g.player].ch;
                g.ch[c].pos
            }
        };
        let near_m = g.k.wind.near_m;
        let tab = &g.tab.wind_wind_source;
        let mut pool: Vec<WindWindSourceId> = g.wind.active.iter().copied().filter(|&s| tab[s].kind == WindSourceKind::Jet && v3::dist(tab[s].bound.c, f) < near_m + tab[s].bound.r).collect();
        let most = (crate::smoke_types::WIND_GPU_MAX - if sw.is_some() { 1.0 } else { 0.0 }) as usize;
        if pool.len() > most {
            js::sort_by(&mut pool, |&a, &b| v3::dist(tab[a].bound.c, f) - v3::dist(tab[b].bound.c, f));
            pool.truncate(most);
        }
        if sw.is_some() && tab[g.wind.storm].speed > 0.0 {
            pool.push(g.wind.storm);
        }
        let recs: Vec<crate::smoke_types::WindGpuSource> = pool.iter().map(|&s| gpu_of(&tab[s])).collect();
        let knobs = [g.k.wind.sheet_wake, g.k.wind.sheet_kick, g.k.wind.debris_k];
        g.wind.to_pool.list = pool;
        seam_particles_set_wind(g, &recs, knobs);
        g.stats.wind.to_particles += recs.len() as i32;
    }
    let recs: Vec<crate::smoke_types::WindGpuSource> = g.wind.active.iter().map(|&s| gpu_of(&g.tab.wind_wind_source[s])).collect();
    crate::smoke_types::smoke_system_like_set_wind(g, &recs, relax);
    g.stats.wind.to_smoke += recs.len() as i32;
}

/// The air's velocity at p this frame, m/s: the sum over every active source. (wind.ts Wind.at)
pub fn wind_at(g: &mut crate::game::Game, p: V3, out: Option<&mut V3>) -> V3 {
    g.stats.wind.at_calls += 1;
    let mut sum: V3 = [0.0, 0.0, 0.0];
    for &id in g.wind.active.iter() {
        let s = &g.tab.wind_wind_source[id];
        let q = if s.kind == WindSourceKind::Storm {
            let w: V3 = [s.dir[0] * s.speed, 0.0, s.dir[2] * s.speed];
            match &s.roofs {
                Some(r) => storm_vel_roofs(w, r, p, None),
                None => storm_vel(w, &s.interior.unwrap(), &s.annex.unwrap(), p, None),
            }
        } else {
            jet_vel(s.pos, s.dir, s.speed, s.reach, s.tan_spread, s.r0, s.d_max, s.floor_y.unwrap_or(-1e9), p, None)
        };
        sum[0] += q[0];
        sum[1] += q[1];
        sum[2] += q[2];
    }
    put(out, sum)
}

/// The source called `name`. (wind.ts Wind.byName)
pub fn wind_by_name(g: &mut crate::game::Game, name: &str) -> Option<WindWindSourceId> {
    g.wind.all.iter().copied().find(|&s| g.tab.wind_wind_source[s].name == name)
}

/// A restart: every vent at once where its own cycle has it, with the mains in. (wind.ts Wind.fresh)
pub fn wind_fresh(g: &mut crate::game::Game) {
    let t = g.time;
    for k in 0..g.wind.all.len() {
        let id = g.wind.all[k];
        let s = &mut g.tab.wind_wind_source[id];
        if s.why == WindWhy::Vent {
            s.on = if vent_due(s.vent.as_ref().unwrap(), t) { 1.0 } else { 0.0 };
        }
    }
}

/// One line per source for the panel. (wind.ts Wind.describe)
pub fn wind_describe(g: &mut crate::game::Game) -> Vec<String> {
    g.wind
        .all
        .iter()
        .map(|&id| {
            let s = &g.tab.wind_wind_source[id];
            alloc::format!("{:<18} {} speed {} (base {} × on {} × gust {})", s.name, if s.speed > 0.0 { "ON " } else { "off" }, js::to_fixed(s.speed, 2), js::to_fixed(s.base, 2), js::to_fixed(s.on, 2), js::to_fixed(s.gust, 2))
        })
        .collect()
}

/// The panel's one line. (wind.ts Wind.summary)
pub fn wind_summary(g: &mut crate::game::Game) -> String {
    let master = wind_of(g);
    let p = crate::particle_fx::gpu_fx(g);
    let ss = seam_smoke_wind_srcs(g);
    let tab = &g.tab.wind_wind_source;
    let w = &g.wind;
    let vents = w.active.iter().filter(|&&s| tab[s].why == WindWhy::Vent).count();
    let doors = w.active.iter().filter(|&&s| tab[s].why == WindWhy::Door).count();
    let still = if master <= 0.0 { " (still air)" } else { "" };
    let storm_speed = tab[w.storm].speed;
    let storm = if storm_speed > 0.0 { alloc::format!(", storm {} m/s", js::to_fixed(storm_speed, 1)) } else { String::new() };
    let gusts = if doors > 0 { alloc::format!(", {} door gust(s)", doors) } else { String::new() };
    let parts = match &p {
        Some(p) => alloc::format!("{} records, {} leaves picked up", p.stats.winds, p.stats.wind_woken),
        None => String::from("off"),
    };
    let solver = match ss {
        Some(n) => alloc::format!("{} sources", n.unwrap_or(0.0)),
        None => String::from("off"),
    };
    alloc::format!("wind {}{} · {}/{} sources blowing ({} vents{}{}) · particles {} · solver {}", js::to_fixed(master, 2), still, w.active.len(), w.all.len(), vents, storm, gusts, parts, solver)
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: [f64; 4] = [-3.1, -2.2, 10.7, 8.9];
    const A: [f64; 4] = [12.5, 1.0, 15.25, 4.75];

    fn bits(v: V3) -> [u64; 3] {
        [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()]
    }

    // the doubles of the TypeScript's own stormVel / stormVelRoofs under bun with EXACT.on
    #[test]
    fn storm_vel_is_the_typescripts_bits() {
        let cases: [(V3, V3, [u64; 3]); 5] = [
            ([1.3, 0.0, -0.7], [-4.2, 1.1, 3.3], [0x3fe9a89f6f5ccf01, 0, 0xbfdba1e6c6b2b78a]),
            ([1.0, 0.0, 0.0], [11.2, 0.0, 3.3], [0x3fc240795ceb2408, 0, 0]),
            ([2.9, 0.0, 0.37], [-3.9, 3.7, -2.9], [0x3ff9843d78684032, 0, 0x3fca0b63d168a464]),
            ([2.9, 0.0, 0.37], [5.0, 1.0, 5.0], [0, 0, 0]),
            ([2.9, 0.0, 0.37], [20.0, 9.0, 5.0], [0, 0, 0]),
        ];
        for (w, p, want) in cases {
            assert_eq!(bits(storm_vel(w, &I, &A, p, None)), want);
        }
    }

    #[test]
    fn storm_vel_roofs_is_the_typescripts_bits() {
        // the fourth has no width and the fifth lies past the first four: neither counts
        let roofs: Vec<Vec<f64>> = vec![vec![0.0, 0.0, 4.0, 4.0], vec![6.0, 0.0, 9.0, 4.0], vec![0.0, 6.0, 4.0, 9.0], vec![5.0, 5.0, 4.0, 9.0], vec![100.0, 100.0, 101.0, 101.0]];
        let cases: [(V3, V3, [u64; 3]); 4] = [
            ([1.3, 0.0, -0.7], [5.3, 0.9, 1.1], [0x3fdaa9631374116d, 0, 0xbfccb66ab27cff12]),
            ([1.0, 0.0, 0.0], [4.7, 0.0, 6.2], [0x3fcfaf6a3cf45d48, 0, 0]),
            ([1.3, 0.0, -0.7], [2.0, 1.0, 2.0], [0, 0, 0]),
            ([1.0, 0.0, 0.0], [20.0, 2.6, 2.0], [0x3ff0000000000000, 0, 0]),
        ];
        for (w, p, want) in cases {
            assert_eq!(bits(storm_vel_roofs(w, &roofs, p, None)), want);
        }
    }

    // the doubles of the TypeScript's own jetVel under bun with EXACT.on: a jet at a floor, on the axis at a floor, sideways, under the floor, no speed
    #[test]
    fn jet_vel_is_the_typescripts_bits() {
        let down: V3 = [0.0, -1.0, 0.0];
        let pos: V3 = [1.0, 3.0, 2.0];
        let a = jet_vel(pos, down, 2.4, 2.0, 0.4227932187381618, 0.24, 5.0, 0.0, [1.3, 0.3, 2.2], None);
        assert_eq!(bits(a), [0x3fc8d2a278d54f87, 0xbfe407f353bc4101, 0x3fc08c6c508e3507]);
        let b = jet_vel(pos, down, 2.4, 2.0, 0.4227932187381618, 0.24, 5.0, 0.0, [1.0, 0.2, 2.0], None);
        assert_eq!(bits(b), [0x3fd1dc5257a9f197, 0xbfe0ac2133a5c6f2, 0x3fcbff6c52115130]);
        let c = jet_vel(pos, [0.6, 0.0, 0.8], 2.4, 2.0, 0.3, 0.24, 5.0, -1e9, [2.5, 3.3, 4.0], None);
        assert_eq!(bits(c), [0x3fe0565baba38eba, 0x3fb3cd866370eb0d, 0x3fe5c87a3a2f68f9]);
        assert_eq!(bits(jet_vel(pos, down, 2.4, 2.0, 0.3, 0.24, 5.0, 0.0, [1.0, -0.5, 2.0], None)), [0, 0, 0]);
        assert_eq!(bits(jet_vel(pos, down, 0.0, 2.0, 0.3, 0.24, 5.0, 0.0, [1.0, 1.0, 2.0], None)), [0, 0, 0]);
    }

    #[test]
    fn vnoise_is_the_typescripts_bits() {
        let cases: [(f64, f64, u64); 4] = [
            (12345.0, 0.3, 0xbfe4becb49ba5e35),
            (3735928559.0, 17.75, 0xbfe483b878000000),
            (0.0, -2.5, 0xbfdefc8b80000000),
            (2147483648.0, 1000.125, 0x3fe38396c1800000),
        ];
        for (seed, x, want) in cases {
            assert_eq!(vnoise(seed, x).to_bits(), want);
        }
    }

    #[test]
    fn out_is_filled_on_every_path() {
        let mut o: V3 = [9.0, 9.0, 9.0];
        let r = storm_vel([1.0, 0.0, 0.0], &I, &A, [5.0, 1.0, 5.0], Some(&mut o));
        assert_eq!(o, r);
        assert_eq!(o, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn a_vents_cycle() {
        let mut v = crate::scene_level::VentDef::zeroed();
        assert!(vent_due(&v, 123.0)); // no period: always
        v.period = Some(10.0);
        v.duty = Some(0.5);
        assert!(vent_due(&v, 3.0));
        assert!(!vent_due(&v, 7.0));
        assert!(!vent_due(&v, -3.0)); // a time before the phase stays on the same wheel
        assert!(vent_due(&v, -7.0));
        v.phase = Some(6.0);
        assert!(vent_due(&v, 7.0));
    }

    fn with_storm() -> (Game, WindWindSourceId) {
        let mut g = Game::blank();
        let mut s = WindSource::zeroed();
        s.why = WindWhy::Storm;
        s.kind = WindSourceKind::Storm;
        s.name = String::from("storm");
        s.interior = Some(I);
        s.annex = Some(A);
        let id = g.tab.wind_wind_source.push(s);
        g.wind.storm = id;
        g.wind.all.push(id);
        (g, id)
    }

    // weather.ts:229 asks before the Wind is built: a blank one has no storm row to write
    #[test]
    fn a_reroof_before_the_wind_exists_is_nothing() {
        let mut g = Game::blank();
        wind_reroof(&mut g, None, None);
        assert!(g.wind.lv_roofs.is_none());
    }

    // wind.ts:161,187: `storm` and its row in `all` are one thing, and a row of `extra` leaves `all` by identity, however many share its name
    #[test]
    fn a_reroof_writes_the_one_storm_and_takes_out_only_extra() {
        let (mut g, st) = with_storm();
        let mut door = WindSource::zeroed();
        door.why = WindWhy::Glass;
        door.name = String::from("twin");
        let keep = g.tab.wind_wind_source.push(door.clone());
        let gone = g.tab.wind_wind_source.push(door);
        g.wind.all.push(keep);
        g.wind.all.push(gone);
        g.wind.extra.push(gone);
        let roofs: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0, 8.0]];
        wind_reroof(&mut g, Some(&roofs), Some(false));
        assert_eq!(g.wind.all, vec![st, keep]);
        assert!(g.wind.extra.is_empty());
        assert_eq!(wind_by_name(&mut g, "storm"), Some(st));
        assert_eq!(g.tab.wind_wind_source[st].interior, Some([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(g.tab.wind_wind_source[st].annex, Some([5.0, 6.0, 7.0, 8.0]));
        assert_eq!(g.wind.lv_roofs, Some(roofs));
    }

    // wind.ts:144: `active` names the row itself, so what a holder writes through its ID is what `at` reads
    #[test]
    fn at_reads_the_row_through_active() {
        let (mut g, st) = with_storm();
        g.wind.active.push(st);
        g.tab.wind_wind_source[st].dir = [1.0, 0.0, 0.0];
        g.tab.wind_wind_source[st].speed = 2.0;
        let p: V3 = [-4.2, 1.1, 3.3];
        let want = storm_vel([2.0, 0.0, 0.0], &I, &A, p, None);
        let mut o: V3 = [9.0, 9.0, 9.0];
        assert_eq!(bits(wind_at(&mut g, p, Some(&mut o))), bits(want));
        assert_eq!(o, want);
        assert_eq!(g.stats.wind.at_calls, 1);
    }

    #[test]
    fn share_under_the_storms_rectangles() {
        let (mut g, _) = with_storm();
        assert_eq!(wind_share_under(&mut g, 5.0, 5.0, None), 0.0);
        assert_eq!(wind_storm_share(&mut g, 5.0, 5.0), 0.0);
        assert_eq!(wind_share_under(&mut g, -50.0, 5.0, None), 1.0);
    }

    #[test]
    fn describe_rounds_as_tofixed_does() {
        let (mut g, st) = with_storm();
        g.tab.wind_wind_source[st].base = 0.125; // an exact tie: the language gives 0.13
        assert_eq!(wind_describe(&mut g), vec![String::from("storm              off speed 0.00 (base 0.13 × on 0.00 × gust 0.00)")]);
    }
}
