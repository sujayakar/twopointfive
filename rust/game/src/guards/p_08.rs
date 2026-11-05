//! src/game/guards.ts lines 2553 to 2903
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ seams: the one step the skeleton does not give yet

/// RtLight.enabled. (guards.ts:2652)
fn seam_lamp_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled of a LightId") }
/// RtLight.intensity. (guards.ts:2652)
fn seam_lamp_intensity(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.intensity of a LightId") }
/// RtLight.kind, the number. (guards.ts:2652)
fn seam_lamp_kind(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.kind of a LightId") }
/// RtLight.owner; 0 where it has none. (guards.ts:2652)
fn seam_lamp_owner(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.owner of a LightId") }
/// RtLight.ttl. (guards.ts:2653)
fn seam_lamp_ttl(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.ttl of a LightId") }
/// RtLight.transient. (guards.ts:2653)
fn seam_lamp_transient(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.transient of a LightId") }
/// RtLight.steady: its pos, radius, intensity. (guards.ts:2662)
fn seam_lamp_steady(g: &mut Game, l: LightId) -> Option<(V3, f64, f64)> { todo!("needs: RtLight.steady of a LightId") }
/// RtLight.pos. (guards.ts:2662)
fn seam_lamp_pos(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.pos of a LightId") }
/// RtLight.pos written. (guards.ts:2902)
fn seam_lamp_set_pos(g: &mut Game, l: LightId, pos: V3) { todo!("needs: RtLight.pos written of a LightId") }
/// RtLight.range. (guards.ts:2662)
fn seam_lamp_range(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.range of a LightId") }
/// RtLight.radius. (guards.ts:2664)
fn seam_lamp_radius(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.radius of a LightId") }
/// RtLight.dir. (guards.ts:2664)
fn seam_lamp_dir(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.dir of a LightId") }
/// RtLight.outerDeg. (guards.ts:2664)
fn seam_lamp_outer_deg(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.outerDeg of a LightId") }
/// RtLight.innerDeg. (guards.ts:2664)
fn seam_lamp_inner_deg(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.innerDeg of a LightId") }
/// RtLight.color. (guards.ts:2664)
fn seam_lamp_color(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.color of a LightId") }

// ------------------------------------------------------------ small private helpers

/// Lends a skip_owner on for one call, keeping it for the next.
fn lend<'a, 'b>(s: &'a mut Option<&'b mut (dyn FnMut(&mut Game, f64) -> bool + 'b)>) -> Option<&'a mut (dyn FnMut(&mut Game, f64) -> bool + 'a)> {
    match s {
        Some(f) => Some(&mut **f),
        None => None,
    }
}

/// field.grid.fed. A LightField is g.cpu_smoke with a grid, by its type. (guards.ts:2678, 2682)
fn field_fed(g: &Game) -> bool {
    g.cpu_smoke.grid.as_ref().unwrap().fed
}

/// feedDrop(def) as this reader needs it: None = undefined (no row of its own, the box that always fell), Some(None) = null (nothing falls). feed_drop's Option cannot say which. (guards.ts:2823; gunreload.ts feedDrop)
fn feed_drop_told(g: &mut Game, def: &crate::weapons::WeaponDef) -> Option<Option<f64>> {
    crate::gunreload::feed_of(g, def).map(|f| f.drop)
}

/// A gun's row, as a copy: a call on it lends g beside it. Nothing here writes a row. (guards.ts w.def)
fn def_row(g: &Game, w: FirearmId) -> crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(g.fa[w].def).unwrap().clone()
}

/// f === gd.foe. (guards.ts:2747, 2754)
fn same_foe(a: crate::game::Foe, b: Option<crate::game::Foe>) -> bool {
    match (a, b) {
        (crate::game::Foe::Player, Some(crate::game::Foe::Player)) => true,
        (crate::game::Foe::Guard(x), Some(crate::game::Foe::Guard(y))) => x == y,
        _ => false,
    }
}

// ------------------------------------------------------------ the guards' collision

/// g.col.collideCircle(c.pos, c.radius, 0.2, 1.5, iterations): moves c.pos in place. (guards.ts:2581, 2596, 2605)
fn push_out(g: &mut Game, c: CharId, iterations: Option<f64>) {
    let (mut p, r) = (g.ch[c].pos, g.ch[c].radius);
    crate::scene_collision::collide_circle(g, &mut p, r, 0.2, 1.5, iterations);
    g.ch[c].pos = p;
}

/// The bottom of updateGuard: the statics swept and pushed out, the colleagues' capped shove, the statics again for the shove. (guards.ts collideGuard)
pub fn collide_guard(g: &mut crate::game::Game, gd: GuardId, p0x: f64, p0z: f64) {
    let c = g.gd[gd].ch;
    // (1) statics, swept from the frame's start (or the snap's spot)
    let snapped = g.m.guards.unstick.last_snap_who == Some(gd) && g.m.guards.unstick.last_snap_t == g.time;
    let (fx, fz) = if snapped { (g.gd[gd].last_pos[0], g.gd[gd].last_pos[2]) } else { (p0x, p0z) };
    sweep_statics(g, c, fx, fz);
    push_out(g, c, None);
    // (2) colleagues, each in turn off where the last left him, then the whole of it capped
    let (ax, az) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead { continue; }
        let oc = g.gd[o].ch;
        let mut d = v3::sub(g.ch[c].pos, g.ch[oc].pos);
        d[1] = 0.0;
        let l = v3::len(d);
        if l < 0.6 && l > 1e-4 { g.ch[c].pos = v3::mad(g.ch[c].pos, d, (0.6 - l) / l * 0.5); }
    }
    let sl = m::hyp2(g.ch[c].pos[0] - ax, g.ch[c].pos[2] - az);
    if sl <= SEP_SETTLE_MIN { return; }
    if sl > SEP_MAX_M {
        g.ch[c].pos[0] = ax + (g.ch[c].pos[0] - ax) * (SEP_MAX_M / sl);
        g.ch[c].pos[2] = az + (g.ch[c].pos[2] - az) * (SEP_MAX_M / sl);
    }
    // (3) the statics again for the shove
    sweep_statics(g, c, ax, az);
    push_out(g, c, None);
}

/// Walks a body from (fx, fz) to where its pos says in strides with a two-pass push-out after each, when that is more than one stride. (guards.ts sweepStatics)
pub fn sweep_statics(g: &mut crate::game::Game, c_2: CharId, fx: f64, fz: f64) {
    let c = c_2;
    let (tx, tz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let dd = m::hyp2(tx - fx, tz - fz);
    let n = m::min(16.0, m::ceil(dd / GUARD_SWEEP_STEP));
    if n <= 1.0 { return; }
    g.ch[c].pos[0] = fx;
    g.ch[c].pos[2] = fz;
    let mut i = 1.0;
    while i < n {
        g.ch[c].pos[0] += (tx - fx) / n;
        g.ch[c].pos[2] += (tz - fz) / n;
        push_out(g, c, Some(2.0));
        i += 1.0;
    }
    g.ch[c].pos[0] += (tx - fx) / n;
    g.ch[c].pos[2] += (tz - fz) / n;
}

// ------------------------------------------------------------ the light meter

/// The shader's range window: (1 - (d/range)^4)^2, clamped. (guards.ts lightWindow)
pub fn light_window(d: f64, range: f64) -> f64 {
    let x = d / range;
    let w = v3::clamp(1.0 - x * x * x * x, 0.0, 1.0);
    w * w
}

/// The CPU's estimate of the renderer's light meter at a point. `field` None takes lightField(g). (guards.ts lightEstimateAt)
pub fn light_estimate_at(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: Option<bool>, field: Option<&mut crate::sense::LightField>, moon: Option<bool>) -> f64 {
    let mut skip_owner = skip_owner;
    let steady_only = steady_only.unwrap_or(false);
    let mut own;
    let mut field = match field {
        Some(f) => Some(f),
        None => {
            own = light_field(g);
            own.as_mut()
        }
    };
    let moon = moon.unwrap_or(g.k.sense.moon);
    if g.col.mirror.live {
        light_estimate_in(g, p, lend(&mut skip_owner), steady_only, field, moon)
    } else {
        crate::scene_collision::window(g, &mut |g: &mut Game| light_estimate_in(g, p, lend(&mut skip_owner), steady_only, field.as_deref_mut(), moon))
    }
}

/// The lamps, the hour, the moon, the stroke, onto six faces; the best of them, plus the light that comes round. The order is the bits. (guards.ts lightEstimateIn)
pub fn light_estimate_in(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, field: Option<&mut crate::sense::LightField>, moon: bool) -> f64 {
    let mut skip_owner = skip_owner;
    let mut field = field;
    let mut acc: Vec<f64> = vec![0.0; 6];
    if !lamps_there(g, p, lend(&mut skip_owner), steady_only, &mut acc, field.as_deref().copied()) {
        lamps_here(g, p, lend(&mut skip_owner), steady_only, &mut acc, field.as_deref_mut());
    }
    crate::hours::hour_light(g, p, &mut acc);
    if moon { crate::hours::moon_light(g, p, &mut acc); }
    if !steady_only { crate::weather::stroke_light(g, p, &mut acc); }
    let direct = m::max_n(&[acc[0], acc[1], acc[2], acc[3], acc[4], acc[5]]);
    if field.is_some() { direct + crate::sense::smoke_grid_glow_at(g, p) } else { direct }
}

/// The light model's field for this game, where the senses are the game's, the switch is up and a grid stands. Some = g.cpu_smoke. (guards.ts lightField)
pub fn light_field(g: &mut crate::game::Game) -> Option<crate::sense::LightField> {
    let stands = match g.cpu_smoke.grid.as_ref() {
        Some(grid) => grid.stands,
        None => false,
    };
    if g.k.sense.cpu && g.k.sense.grid && g.k.sense.bounce && stands {
        Some(crate::sense::LightField {})
    } else {
        None
    }
}

/// Is this lamp one of a light question's: the one copy of the gates. (guards.ts lampAsked)
pub fn lamp_asked(g: &mut crate::game::Game, l: LightId, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool {
    if !seam_lamp_enabled(g, l) || seam_lamp_intensity(g, l) <= 0.0 || seam_lamp_kind(g, l) == 2.0 { return false; }
    if let Some(f) = skip_owner {
        let owner = seam_lamp_owner(g, l);
        if js::truthy(owner) && f(g, owner) { return false; }
    }
    !(steady_only && (seam_lamp_ttl(g, l) >= 0.0 || seam_lamp_transient(g, l) || g.beacons.iter().any(|b| b.light == l)))
}

/// A light question's lamps written to the module, in the list's order. False = the module cannot take them. (guards.ts writeLamps)
pub fn write_lamps(g: &mut crate::game::Game, p: Option<V3>, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool {
    let mut skip_owner = skip_owner;
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        if !lamp_asked(g, l, lend(&mut skip_owner), steady_only) { continue; }
        let sp = if steady_only { seam_lamp_steady(g, l) } else { None };
        let lp = match sp { Some(s) => s.0, None => seam_lamp_pos(g, l) };
        let range = seam_lamp_range(g, l);
        if let Some(q) = p {
            let (dx, dy, dz) = (lp[0] - q[0], lp[1] - q[1], lp[2] - q[2]);
            if dx > range || -dx > range || dy > range || -dy > range || dz > range || -dz > range { continue; }
        }
        let radius = match sp { Some(s) => s.1, None => seam_lamp_radius(g, l) };
        let kind = seam_lamp_kind(g, l);
        let dir = seam_lamp_dir(g, l);
        let outer = seam_lamp_outer_deg(g, l);
        let inner = seam_lamp_inner_deg(g, l);
        let color = seam_lamp_color(g, l);
        let intensity = match sp { Some(s) => s.2, None => seam_lamp_intensity(g, l) };
        if !crate::scene_collision::lamp(g, lp, range, radius, kind, dir, outer, inner, color, intensity) { return false; }
    }
    true
}

/// A skip_owner that leaves out every lamp with an owner. (guards.ts CARRIED)
pub fn carried() -> bool {
    true
}

/// The level's own steady lamps, written for the light that comes round to be swept from. (guards.ts steadyLamps)
pub fn steady_lamps(g: &mut crate::game::Game) -> bool {
    write_lamps(g, None, Some(&mut |_g: &mut Game, _owner: f64| carried()), true)
}

/// The lamps' part of the estimate from the module. False = lampsHere's loop, from the start. (guards.ts lampsThere)
pub fn lamps_there(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, acc: &mut Vec<f64>, field: Option<crate::sense::LightField>) -> bool {
    use crate::scene_collision::{BlobRow, LampsBeginShade, LampsBeginShadeGrid, SmokeNumbers};
    let shade = if field.is_some() && field_fed(g) {
        let grid = g.cpu_smoke.grid.as_ref().unwrap();
        let grid = LampsBeginShadeGrid { sim: grid.sim, stands: grid.stands };
        let blobs = g.cpu_smoke.blobs.iter().map(|b| BlobRow { x: b.x, y0: b.y0, z: b.z, r2: b.r2, age: b.age, mass: b.mass }).collect();
        Some(LampsBeginShade { grid, blobs })
    } else {
        None
    };
    let s = &g.k.sense.smoke;
    let k = SmokeNumbers { sigma: s.sigma, spread: s.spread, widest: s.widest, rise: s.rise, rise_rate: s.rise_rate };
    crate::scene_collision::lamps_begin(g, p, shade.as_ref(), Some(k)) && write_lamps(g, Some(p), skip_owner, steady_only) && crate::scene_collision::lamps_on(g, acc)
}

/// The lamps' part of the estimate as this file answers it: the shadowed direct light of every lamp, onto six faces. (guards.ts lampsHere)
pub fn lamps_here(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, acc: &mut Vec<f64>, field: Option<&mut crate::sense::LightField>) {
    let mut skip_owner = skip_owner;
    let smoky = field.is_some() && field_fed(g);
    let normals = meter_normals();
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        if !lamp_asked(g, l, lend(&mut skip_owner), steady_only) { continue; }
        let sp = if steady_only { seam_lamp_steady(g, l) } else { None };
        let lp = match sp { Some(s) => s.0, None => seam_lamp_pos(g, l) };
        let (dx, dy, dz) = (lp[0] - p[0], lp[1] - p[1], lp[2] - p[2]);
        let dist2 = dx * dx + dy * dy + dz * dz;
        let dist = m::sqrt(dist2);
        let range = seam_lamp_range(g, l);
        if dist > range || dist < 1e-3 { continue; }
        let (lx, ly, lz) = (dx / dist, dy / dist, dz / dist);
        let lr = match sp { Some(s) => s.1, None => seam_lamp_radius(g, l) };
        let mut att = light_window(dist, range) / m::max(dist2, lr * lr);
        let kind = seam_lamp_kind(g, l);
        if kind == 1.0 {
            let dir = seam_lamp_dir(g, l);
            let cd = -(lx * dir[0] + ly * dir[1] + lz * dir[2]);
            let (outer, inner) = (seam_lamp_outer_deg(g, l), seam_lamp_inner_deg(g, l));
            let cos_outer = m::cos(outer * v3::DEG);
            let cos_inner = m::cos(m::min(inner, outer - 0.5) * v3::DEG);
            if cd < cos_outer { continue; }
            att *= v3::smoothstep(cos_outer, cos_inner, cd);
        } else if kind == 3.0 {
            let dir = seam_lamp_dir(g, l);
            let ce = -(lx * dir[0] + ly * dir[1] + lz * dir[2]);
            if ce <= 0.0 { continue; }
            att *= ce;
        }
        if att <= 0.0 { continue; }
        let c = seam_lamp_color(g, l);
        let intensity = match sp { Some(s) => s.2, None => seam_lamp_intensity(g, l) };
        if m::max_n(&[c[0], c[1], c[2]]) * intensity * att < 0.002 { continue; }
        let mut lum = (0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]) * intensity * att;
        let tmax = dist - lr - 0.02;
        if tmax > 0.05 {
            let end: V3 = [p[0] + lx * tmax, p[1] + ly * tmax, p[2] + lz * tmax];
            if crate::scene_collision::segment_blocked(g, p, end, None, None, None) { continue; }
            if smoky { lum *= crate::sense::cpu_smoke_along(g, p, end); }
        }
        for k in 0..6 {
            let n = normals[k];
            let cs = n[0] * lx + n[1] * ly + n[2] * lz;
            if cs > 0.0 { acc[k] += lum * cs; }
        }
    }
}

/// The chest + head meter of one body: the probe's two answers when both are finite, else the CPU estimate at the same points. (guards.ts meterOf)
pub fn meter_of(g: &mut crate::game::Game, c_2: CharId, prefix: &str) -> f64 {
    let c = c_2;
    let id = g.ch[c].id;
    let pos = g.ch[c].pos;
    let pc = match g.ch[c].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => v3::add(pos, [0.0, 1.2, 0.0]) };
    let ph = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(pos, [0.0, 1.6, 0.0]) };
    let kc = format!("{}C{}", prefix, id);
    let kh = format!("{}H{}", prefix, id);
    crate::game::game_light_query(g, &kc, pc, Some(id));
    crate::game::game_light_query(g, &kh, ph, Some(id));
    let r1 = crate::game::game_light_at(g, &kc);
    let r2 = crate::game::game_light_at(g, &kh);
    if let (Some(a), Some(b)) = (r1, r2) {
        if a.is_finite() && b.is_finite() { return m::max(a, b); }
    }
    let a = light_estimate_at(g, pc, None, None, None, None);
    let b = light_estimate_at(g, ph, None, None, None, None);
    m::max(a, b)
}

/// The light meter of a man some AI side can be hostile to, eased at Sam's rate into his visibility. (guards.ts unitLightMeter)
pub fn unit_light_meter(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let c = g.gd[gd].ch;
    let e = meter_of(g, c, "unit");
    g.gd[gd].irradiance = e;
    let was = g.gd[gd].visibility;
    let to = crate::game::game_visibility_of(g, e);
    g.gd[gd].visibility = v3::damp(was, to, 6.0, dt);
}

// ------------------------------------------------------------ a unit's foe

/// The closure of acquireFoe: one enemy looked at; the most salient is kept. (guards.ts:2744 consider)
fn consider(g: &mut Game, gd: GuardId, f: crate::game::Foe, ch: CharId, best: &mut Option<crate::game::Foe>, best_rate: &mut f64, cur_visible: &mut bool) {
    let rate = match f {
        crate::game::Foe::Player => {
            let vis = can_see(g, gd);
            if !vis.visible { return; }
            sight_gain(g, gd, vis.dist)
        }
        crate::game::Foe::Guard(o) => {
            let vis = can_see_char(g, gd, ch, false);
            if !vis.visible { return; }
            sight_gain_on(g, gd, o, vis.dist)
        }
    };
    if same_foe(f, g.gd[gd].foe) && rate > 0.05 { *cur_visible = true; }
    if rate > *best_rate {
        *best_rate = rate;
        *best = Some(f);
    }
}

/// A unit's foe this frame: off a man who is gone, then on the most salient enemy he can see; a fight his eyes have won ends here. (guards.ts acquireFoe)
pub fn acquire_foe(g: &mut crate::game::Game, gd: GuardId) {
    use crate::game::{Foe, GuardState};
    use crate::teams::Team;
    let cur_man = match g.gd[gd].foe {
        Some(Foe::Guard(o)) => Some(o),
        _ => None,
    };
    let (mut lost_man, mut mine) = (false, false);
    let knee = match cur_man {
        Some(o) => crate::rails::rails_down(g, o),
        None => false,
    };
    if let Some(cur) = cur_man {
        if g.gd[cur].state == GuardState::Dead || !g.guards.contains(&cur) || crate::runner::runner_fled(g, cur) || knee {
            let cur_pos = g.ch[g.gd[cur].ch].pos;
            if let Some(lk) = g.gd[gd].last_known {
                if v3::dist_xz(lk, cur_pos) < 1.5 { g.gd[gd].last_known = None; }
            }
            mine = g.gd[cur].state == GuardState::Dead && g.gd[cur].killer_id == g.ch[g.gd[gd].ch].id && g.gd[gd].state == GuardState::Alert;
            g.gd[gd].foe = None;
            crate::aim::aim_state_clear(&mut g.gd[gd].aim);
            lost_man = true;
            if knee {
                let w = g.gd[gd].weapon;
                g.fa[w].burst = None;
            }
        }
    }
    let (mut best, mut best_rate, mut cur_visible): (Option<Foe>, f64, bool) = (None, 0.05, false);
    let team = g.gd[gd].team;
    if crate::teams::is_hostile(g, team, Team::Player) && !g.pl[g.player].down && !g.player_invisible {
        let ch = g.pl[g.player].ch;
        consider(g, gd, Foe::Player, ch, &mut best, &mut best_rate, &mut cur_visible);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].state == GuardState::Dead { continue; }
        let och = g.gd[o].ch;
        if !g.ch[och].alive { continue; }
        let oteam = g.gd[o].team;
        if crate::teams::is_hostile(g, team, oteam) && !crate::runner::runner_fled(g, o) && !crate::rails::rails_down(g, o) {
            consider(g, gd, Foe::Guard(o), och, &mut best, &mut best_rate, &mut cur_visible);
        }
    }
    let op = team == Team::Operative;
    if cur_visible { return; }
    if let Some(b) = best {
        if !same_foe(b, g.gd[gd].foe) {
            g.gd[gd].foe = Some(b);
            crate::aim::aim_state_clear(&mut g.gd[gd].aim);
            if mine { crate::game::game_say(g, gd, if op { "he's down — next." } else { "got him!" }, None, None, None); }
            return;
        }
    }
    if lost_man && best.is_none() && g.gd[gd].unit.is_none() && team == Team::Guard && !g.pl[g.player].down {
        // a staff man: the man he dropped is one of two intruders, and the other is still on his floor
        g.gd[gd].foe = Some(Foe::Player);
        g.gd[gd].last_seen_t = m::min(g.gd[gd].last_seen_t, g.time - 0.5);
        let w = g.gd[gd].weapon;
        g.fa[w].burst = None;
        if mine { crate::game::game_say(g, gd, "got him!", None, None, None); }
    } else if lost_man && best.is_none() && (g.gd[gd].state == GuardState::Alert || g.gd[gd].state == GuardState::Search) {
        // the man he was fighting is down and no other enemy is in his sight: he comes off the alert to a wary look round
        g.gd[gd].state = GuardState::Suspicious;
        g.gd[gd].awareness = 0.45;
        g.gd[gd].react_t = 1.2;
        g.gd[gd].search_t = 0.0;
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_goal = None;
        g.gd[gd].search_plan = None;
        g.gd[gd].pieing = None;
        let left = enemy_up_on_floor(g, team);
        let said = left || floor_called_clear(g, gd, team);
        if mine {
            let text = if op { if said { "he's down." } else { "he's down — clear." } } else if said { "got him!" } else { "got him — that's all of them." };
            crate::game::game_say(g, gd, text, None, None, None);
        } else if !said && !knee {
            crate::game::game_say(g, gd, if op { "clear." } else { "that's all of them." }, None, None, None);
        }
    } else if mine {
        crate::game::game_say(g, gd, if op { "he's down." } else { "got him!" }, None, None, None);
    }
}

/// Nobody calls a floor clear with an enemy still up on it. (guards.ts:2768, WORDS.on settled on)
fn enemy_up_on_floor(g: &mut Game, team: crate::teams::Team) -> bool {
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if g.gd[o].state == crate::game::GuardState::Dead { continue; }
        let oteam = g.gd[o].team;
        if crate::teams::is_hostile(g, team, oteam) { return true; }
    }
    crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[g.player].down
}

/// One of his side said it in the last three seconds. (guards.ts:2769)
fn floor_called_clear(g: &Game, gd: GuardId, team: crate::teams::Team) -> bool {
    g.guards.iter().any(|&o| {
        if o == gd || g.gd[o].team != team { return false; }
        match g.gd[o].bubble.as_ref() {
            Some(b) => g.time - b.t < 3.0 && (b.text.contains("clear.") || b.text.contains("all of them")),
            None => false,
        }
    })
}

// ------------------------------------------------------------ the right hand

/// Which rig is in his hands. (guards.ts rigOf)
pub fn rig_of(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::RigOf {
    use crate::weapons::WeaponDefRig;
    let def = g.fa[g.gd[gd].weapon].def;
    let rig = g.m.weapons.weapons.get(def).unwrap().rig;
    if g.gd[gd].carbine || rig == WeaponDefRig::Carbine {
        crate::guards::RigOf::Carbine
    } else if rig == WeaponDefRig::Sc20k {
        crate::guards::RigOf::Sc20k
    } else {
        crate::guards::RigOf::Pistol
    }
}

/// Which long gun that is; None = the sidearm. (guards.ts longRigOf)
pub fn long_rig_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::LongRigOf> {
    match rig_of(g, gd) {
        crate::guards::RigOf::Pistol => None,
        crate::guards::RigOf::Carbine => Some(crate::guards::LongRigOf::Carbine),
        crate::guards::RigOf::Sc20k => Some(crate::guards::LongRigOf::Sc20k),
    }
}

/// What his state has him carry in the gun hand. (guards.ts handCarry)
pub fn hand_carry(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::HandCarry {
    use crate::game::GuardState;
    let torch = crate::startle::startle_torch(g, gd)
        || ((g.gd[gd].state == GuardState::Patrol || (g.gd[gd].state == GuardState::Suspicious && crate::whistle::whistle_soft(g, gd)))
            && !g.gd[gd].drawn
            && rig_of(g, gd) == crate::guards::RigOf::Pistol
            && !crate::kit::kit_pistol(g, gd));
    if torch { crate::guards::HandCarry::Torch } else { crate::guards::HandCarry::Weapon }
}

/// A living man has the gun knocked out of his hand at the grab again: off the floor and in his fist. (guards.ts gunBack)
pub fn gun_back(g: &mut crate::game::Game, gd: GuardId) {
    let dropped = g.gd[gd].dropped;
    if let Some(d) = dropped {
        if crate::throwables::item_kind(g, d) != crate::throwables::ItemKind::Torch {
            if let Some(i) = g.items.items.iter().position(|&x| x == d) {
                g.items.items.remove(i);
                g.gd[gd].dropped = None;
            }
        }
    }
    g.gd[gd].gun_lost_at = -1.0;
}

/// What is physically in the hand. (guards.ts inHand)
pub fn in_hand(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::InHand> {
    if crate::runner::runner_fled(g, gd) { return None; }
    if g.gd[gd].gun_lost_at >= 0.0 { return None; }
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].civ.is_some() { return None; }
    match hand_carry(g, gd) {
        crate::guards::HandCarry::Torch => {
            if g.gd[gd].dropped.is_some() || g.gd[gd].torch_lost_at >= 0.0 { None } else { Some(crate::guards::InHand::Torch) }
        }
        crate::guards::HandCarry::Weapon => Some(crate::guards::InHand::Weapon),
    }
}

/// The magazine is empty: he stands there changing it, says so, and the clack plays at him. (guards.ts startReloadBeat)
pub fn start_reload_beat(g: &mut crate::game::Game, gd: GuardId) {
    if g.gd[gd].second.is_some() && crate::gunslots::side_out_alert(g, gd) { return; }
    let c = g.gd[gd].ch;
    let w = g.gd[gd].weapon;
    g.gd[gd].reload_t = crate::gunreload::feed_begin(g, w).dur;
    g.fa[w].burst = None;
    crate::character::anim_reload(g, c, None, None);
    let gun = if g.ch[c].anim.long_gun { Some(w) } else { None };
    crate::gunreload::feed_deal(g, c, gun, Some(true));
    let def = def_row(g, w);
    if !crate::gunreload::feed_starts(g, &def) {
        let at = g.ch[c].pos;
        crate::audio_iface::game_audio_like_play(g, "magOut", Some(at), Some(0.5), None);
    }
    crate::game::game_say(g, gd, "reloading!", None, None, None);
}

/// Counted down each frame he could otherwise fire; a staged row lets the empty go as the beat crosses `out`. (guards.ts tickReloadBeat)
pub fn tick_reload_beat(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let w = g.gd[gd].weapon;
    let rr = crate::weapons::firearm_reload_row(g, w);
    let before = rr.dur - g.gd[gd].reload_t;
    g.gd[gd].reload_t -= dt;
    let def = def_row(g, w);
    let drop = feed_drop_told(g, &def);
    let c = g.gd[gd].ch;
    let gun = if g.ch[c].anim.long_gun { Some(w) } else { None };
    crate::gunreload::feed_deal(g, c, gun, Some(true));
    let now = rr.dur - g.gd[gd].reload_t;
    crate::gunreload::feed_tick(g, c, &def, before, now, None, Some(w));
    if let Some(out) = rr.out {
        if drop != Some(None) && before < out && rr.dur - g.gd[gd].reload_t >= out {
            let pos = g.ch[c].pos;
            let h = match g.ch[c].bones.get(crate::character::Bone::HandR) { Some(b) => *b, None => v3::add(pos, [0.0, 1.1, 0.0]) };
            let d = g.ch[c].gun_dir;
            let vel = g.ch[c].vel;
            let (id, yaw) = (g.ch[c].id, g.ch[c].aim_yaw);
            let opts = crate::throwables::ThrowablesSpawnOpts { fill: Some(drop.flatten().unwrap_or(2.0)), who: Some(id), heading: Some(yaw), yaw: Some(yaw), ..Default::default() };
            let it = crate::throwables::throwables_spawn(
                g,
                crate::throwables::ItemKind::Mag,
                [h[0] + d[0] * 0.12, m::max(0.3, h[1] - 0.1), h[2] + d[2] * 0.12],
                [vel[0] * 0.5 + d[0] * 0.3, -0.4, vel[2] * 0.5 + d[2] * 0.3],
                Some(opts),
            );
            // it.onBounce: rate 0.6 where drop === 5, else 0.8
            g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::MagRate { belt: drop == Some(Some(5.0)) });
        }
    }
    if g.gd[gd].reload_t < 0.0 { crate::weapons::firearm_rearm(g, w); }
}

// ------------------------------------------------------------ rifles: the burst trigger

/// The closure of burstTrigger: the burst ends, and the beat to the next runs from its last serviced frame. (guards.ts:2854 end)
fn burst_end(g: &mut Game, gd: GuardId, w: FirearmId, cd: f64) {
    let b = g.fa[w].burst;
    g.fa[w].burst = None;
    if let Some(b) = b {
        if b.idx > 0.0 {
            g.fa[w].bursts += 1.0;
            g.fa[w].last_burst_t = b.t;
        }
    }
    let since = match b {
        Some(b) => g.time - b.t,
        None => 0.0,
    };
    g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, cd - since);
}

/// The trigger of a row with a BurstSpec, asked once a frame at the alert branch's fire site. (guards.ts burstTrigger)
pub fn burst_trigger(g: &mut crate::game::Game, gd: GuardId, dt: f64, aim_at: V3, start_ok: bool, round_ok: bool, except: Option<GuardId>) {
    let w = g.gd[gd].weapon;
    let def = g.fa[w].def;
    let spec = match g.m.weapons.weapons.get(def).unwrap().burst {
        Some(s) => s,
        None => return,
    };
    let c = g.gd[gd].ch;
    if let Some(b) = g.fa[w].burst {
        if g.time - b.t > dt * 1.5 + 1e-6 { burst_end(g, gd, w, BURST_BROKEN_CD); }
    }
    if g.fa[w].burst.is_none() {
        if !start_ok || g.gd[gd].fire_cd > 0.0 || round_past_colleague(g, gd, aim_at, except, None) { return; }
        let n = v3::clamp(spec.min + m::floor(crate::math_dice::sim_random(g) * (spec.max - spec.min + 1.0)), spec.min, spec.max);
        let mut off: Option<V3> = None;
        let opening = g.fa[w].bursts == 0.0 || g.time - g.fa[w].last_burst_t > SUPPRESS_FIRST_LAPSE;
        let wide = g.tune.suppress_first && opening && {
            let foe = match crate::teams::foe_char(g, gd) {
                Some(f) => f,
                None => g.pl[g.player].ch,
            };
            v3::dist_xz(g.ch[c].pos, g.ch[foe].pos) >= SUPPRESS_FIRST_MIN_M
        };
        if wide {
            let pos = g.ch[c].pos;
            let (dx, dz) = (aim_at[0] - pos[0], aim_at[2] - pos[2]);
            let ll = js::or(m::hyp2(dx, dz), 1.0);
            let (px, pz) = (-dz / ll, dx / ll);
            let chest_p: V3 = [aim_at[0], aim_at[1], aim_at[2]];
            let l = crate::scene_collision::raycast(g, chest_p, [px, 0.0, pz], SUPPRESS_FIRST_COVER_M);
            let r = crate::scene_collision::raycast(g, chest_p, [-px, 0.0, -pz], SUPPRESS_FIRST_COVER_M);
            let side: f64 = match (&l, &r) {
                (Some(a), Some(b)) => if a.t <= b.t { 1.0 } else { -1.0 },
                (Some(_), None) => 1.0,
                (None, Some(_)) => -1.0,
                (None, None) => if crate::math_dice::sim_random(g) < 0.5 { 1.0 } else { -1.0 },
            };
            let out = SUPPRESS_FIRST_OFF_M + crate::math_dice::sim_random(g) * SUPPRESS_FIRST_JITTER_M;
            off = Some([px * side * out, SUPPRESS_FIRST_UP_M, pz * side * out]);
        }
        g.fa[w].burst = Some(crate::weapons::BurstState { left: n, n, next_t: 0.0, idx: 0.0, t: g.time, off });
    }
    // `b` is the TypeScript's own local: a burst something else ends under the calls below is written no more, and b is still read
    let mut b = g.fa[w].burst.unwrap();
    b.t = g.time;
    g.fa[w].burst = Some(b);
    if !round_ok {
        burst_end(g, gd, w, BURST_BROKEN_CD);
        return;
    }
    b.next_t -= dt;
    g.fa[w].burst = Some(b);
    if b.next_t > 1e-4 { return; }
    let aim: V3 = match b.off {
        Some(o) => [aim_at[0] + o[0], aim_at[1] + o[1] + SUPPRESS_FIRST_WALK_M * b.idx, aim_at[2] + o[2]],
        None => aim_at,
    };
    let grow = m::min(spec.spread_cap, spec.spread_per_round * b.idx);
    let mut cone = 0.0;
    if b.off.is_none() && b.idx > 0.0 { cone = crate::aim::hands_cone(g, gd, grow) / 2.0; }
    if round_past_colleague(g, gd, aim, except, Some(cone)) {
        burst_end(g, gd, w, BURST_BROKEN_CD);
        return;
    }
    if !crate::weapons::firearm_fire(g, w) {
        g.fa[w].burst = None;
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 && g.gd[gd].reload_t < 0.0 { start_reload_beat(g, gd); }
        return;
    }
    let mods = crate::suppression::guard_fire_mods(g, gd);
    let opts = if b.off.is_some() {
        crate::combat::FireOpts { spread: Some(0.0), steady: Some(true), ..Default::default() }
    } else {
        crate::aim::guard_round(g, gd, grow + mods.spread, Some(b.idx == 0.0))
    };
    crate::combat::fire_weapon(g, c, aim, false, Some(&opts));
    if b.off.is_none() { crate::aim::note_bore(g, gd, aim); }
    b.idx += 1.0;
    b.left -= 1.0;
    b.next_t = spec.interval;
    if let Some(live) = g.fa[w].burst.as_mut() { *live = b; }
    if b.left <= 0.0 || crate::weapons::firearm_rounds_ready(g, w) == 0.0 {
        let gap = (spec.gap_min + crate::math_dice::sim_random(g) * (spec.gap_max - spec.gap_min)) * mods.gap;
        burst_end(g, gd, w, gap);
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 { start_reload_beat(g, gd); }
    }
}

/// Panel / QA: one line per living rifleman. (guards.ts burstSummary)
pub fn burst_summary(g: &mut crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let w = g.gd[gd].weapon;
        let def = g.fa[w].def;
        let row = g.m.weapons.weapons.get(def).unwrap();
        if g.gd[gd].state == crate::game::GuardState::Dead || row.burst.is_none() { continue; }
        let cap = row.mag_capacity;
        let ready = crate::weapons::firearm_rounds_ready(g, w);
        let reloading = if g.gd[gd].reload_t >= 0.0 { format!(" reloading {:.1} s", g.gd[gd].reload_t) } else { String::new() };
        let beat = match g.fa[w].burst {
            Some(b) => format!("burst {}/{}{}", b.idx, b.n, if b.off.is_some() { " (opening, wide)" } else { "" }),
            None => format!("cd {:.2}", m::max(0.0, g.gd[gd].fire_cd)),
        };
        parts.push(format!("{}: {}/{}{} · {} · {} bursts", g.gd[gd].callsign, ready, cap, reloading, beat, g.fa[w].bursts));
    }
    if !parts.is_empty() { format!("rifles ▸ {}", parts.join(" | ")) } else { format!("rifles ▸ nobody carries one ({})", g.tune.carbine_guards.as_str()) }
}

/// His light posed for the frame; a burning lens carried through a wall lights from this side of it. (guards.ts attachFlashlight)
pub fn attach_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) {
    let dt = dt.unwrap_or(1.0 / 60.0);
    let held = pose_flashlight(g, gd, dt);
    let fl = g.gd[gd].flashlight;
    if let Some(from) = held {
        if seam_lamp_enabled(g, fl) {
            let lens = seam_lamp_pos(g, fl);
            let radius = g.engine.settings.flashlight_radius;
            let to = crate::lighting::lens_clear(g, from, lens, radius, Some(fl));
            seam_lamp_set_pos(g, fl, to);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// lightWindow against the TypeScript's own bits (bun, EXACT on).
    #[test]
    fn light_window_bits() {
        let rows: [(f64, f64, u64); 6] = [
            (1.3, 7.7, 0x3feff2b19d0037d7),
            (0.0, 5.0, 0x3ff0000000000000),
            (6.1, 6.1, 0x0000000000000000),
            (9.0, 4.0, 0x0000000000000000),
            (0.123456789, 0.987654321, 0x3feffc002002720d),
            (3.3, 11.17, 0x3fef83a9584869e6),
        ];
        for (d, r, bits) in rows {
            assert_eq!(light_window(d, r).to_bits(), bits);
        }
    }
}
