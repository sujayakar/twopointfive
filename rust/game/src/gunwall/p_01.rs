//! src/game/gunwall.ts lines 1 to 299
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `GUNWALL_STATS.loweredS += dt`: the counter's field came out `i32`, it sums seconds (gunwall.ts:218).
fn seam_stats_lowered_s(g: &mut Game, dt: f64) {
    todo!("needs: GunwallStats.lowered_s as f64")
}

/// The row of a man the rule has already asked about: the one truth behind every `S` of the TypeScript.
fn row(g: &mut Game, c: CharId) -> &mut St {
    g.m.gunwall.st.get_mut(c).unwrap()
}

/// His row, made on first asking. (gunwall.ts st)
pub fn st<'g>(g: &'g mut crate::game::Game, c_2: CharId) -> &'g mut crate::gunwall::St {
    let p = g.ch[c_2].pos;
    g.m.gunwall.st.get_or_insert_with(c_2, || St { low: false, t: -1.0, mine: false, hold: None, wood: 0.0, rose: false, px: p[0], pz: p[2], vt: -1.0, vx: 0.0, vz: 0.0 })
}

/// The floor's fight: a man gone alert or a scripted man with one in his sights, remembered `fightS`. (gunwall.ts fightOn)
pub fn fight_on(g: &mut crate::game::Game) -> bool {
    let fight_s = g.k.gunwall.fight_s;
    if !(fight_s > 0.0) { return false; }
    let mut f = match g.m.gunwall.fight { Some(f) => f, None => GunwallModFIGHTValue { t: -1.0, until: -1.0 } };
    if f.t != g.time {
        f.t = g.time;
        f.until = m::min(f.until, g.time + fight_s);
        for &gd in &g.guards {
            let row = &g.gd[gd];
            if g.ch[row.ch].alive && (row.state == crate::game::GuardState::Alert || row.script.as_ref().map_or(false, |s| s.over.is_some())) {
                f.until = g.time + fight_s;
                break;
            }
        }
    }
    g.m.gunwall.fight = Some(f);
    g.time < f.until
}

/// A showcase has the floor (the tour, a Director showcase). (gunwall.ts showing)
pub fn showing(g: &mut crate::game::Game) -> bool {
    g.touring || crate::director::director_showing(g)
}

/// His frame on `yaw`: forward, right, the gun shoulder over his feet, how crouched (0..1). (gunwall.ts frame)
pub fn frame(g: &mut crate::game::Game, c_2: CharId, yaw: f64) -> crate::gunwall::Frame {
    let (s, co) = (m::sin(yaw), m::cos(yaw));
    let f: V3 = [s, 0.0, co];
    let r: V3 = [-co, 0.0, s];
    let py = g.ch[c_2].pos[1];
    let sh = (match g.ch[c_2].bones.get(crate::character::Bone::UpperArmR) { Some(b) => b[1], None => py + STAND_SH }) - py;
    Frame { f, r, sh, k: v3::clamp((STAND_SH - sh) / (STAND_SH - CROUCH_SH), 0.0, 1.0) }
}

/// A point off `o` in his frame. (gunwall.ts at)
pub fn at(o: V3, f: crate::gunwall::AtF, fwd: f64, right: f64, y: f64) -> V3 {
    [o[0] + f.f[0] * fwd + f.r[0] * right, o[1] + y, o[2] + f.f[2] * fwd + f.r[2] * right]
}

/// His feet's speed over the floor, sampled once a frame by whoever asks first. (gunwall.ts track)
pub fn track(g: &mut crate::game::Game, c_2: CharId) {
    let (now, p) = (g.time, g.ch[c_2].pos);
    let s = row(g, c_2);
    if s.vt == now { return; }
    let h = now - s.vt;
    let dx = p[0] - s.px;
    let dz = p[2] - s.pz;
    if h > 1e-5 && h < 0.1 && m::hyp2(dx, dz) < TELEPORT_M { s.vx = dx / h; s.vz = dz / h; } else { s.vx = 0.0; s.vz = 0.0; }
    s.px = p[0];
    s.pz = p[2];
    s.vt = now;
}

/// How fast he closes along `f`, m/s, never negative. (gunwall.ts closing)
pub fn closing(s: crate::gunwall::St, f: V3) -> f64 {
    m::max(0.0, s.vx * f[0] + s.vz * f[2])
}

/// The line a raised gun lies along on `yaw`: `a` at the hand end, `b` at the muzzle. (gunwall.ts raisedLine)
pub fn raised_line(g: &mut crate::game::Game, c_2: CharId, up: crate::gunwall::GunUp, yaw: f64, o: Option<V3>) -> crate::gunwall::RaisedLine {
    let o = o.unwrap_or(g.ch[c_2].pos);
    let fr = frame(g, c_2, yaw);
    let af = AtF { f: fr.f, r: fr.r };
    let long = g.ch[c_2].anim.long_gun;
    if up == GunUp::Aim {
        let reach = (if long { 0.11 } else { 0.385 }) + g.ch[c_2].muzzle_ahead;
        let side = if long { v3::lerp(0.2, 0.16, fr.k) } else { v3::lerp(0.16, 0.02, fr.k) };
        let y = fr.sh - (if long { 0.11 } else { v3::lerp(0.02, 0.22, fr.k) });
        let p = v3::clamp(g.ch[c_2].aim_pitch, -0.7, 0.7);
        return RaisedLine { a: at(o, af, 0.0, side, y), b: at(o, af, reach * m::cos(p), side, y + reach * m::sin(p)) };
    }
    if long {
        RaisedLine { a: at(o, af, 0.15, 0.12, fr.sh - 0.28), b: at(o, af, 0.44, -0.065, fr.sh + 0.13) }
    } else {
        RaisedLine { a: at(o, af, 0.0, 0.07, fr.sh + 0.04), b: at(o, af, 0.465, 0.07, fr.sh + 0.07) }
    }
}

/// A collision ray a → b, at most `most` metres. (gunwall.ts gunClearFrom's `hits`)
fn hits(g: &mut Game, a: V3, b: V3, most: f64) -> bool {
    let d = v3::sub(b, a);
    let l = v3::len(d);
    l > 1e-3 && crate::scene_collision::raycast(g, a, v3::scale(d, 1.0 / l), m::min(l, most)).is_some()
}

/// Stood at (x, z) with the long gun in the shoulder on `to`: would the gun, and the first `ahead` metres of its round's way, lie clear? (gunwall.ts gunClearFrom)
pub fn gun_clear_from(g: &mut crate::game::Game, c_2: CharId, x: f64, z: f64, to: V3, ahead: f64) -> bool {
    let yaw = m::atan2(to[0] - x, to[2] - z);
    let fr = frame(g, c_2, yaw);
    let py = g.ch[c_2].pos[1];
    let ml = raised_line(g, c_2, GunUp::Aim, yaw, Some([x, py, z]));
    let sh: V3 = [x + fr.r[0] * 0.14 - fr.f[0] * 0.15, ml.a[1] + 0.11, z + fr.r[2] * 0.14 - fr.f[2] * 0.15];
    for w in [0.0, CLEAR_WIDE] {
        let off: V3 = [fr.r[0] * w, 0.0, fr.r[2] * w];
        let a = v3::add(ml.a, off);
        let b = v3::add(ml.b, off);
        if hits(g, sh, b, m::INF) || hits(g, a, b, m::INF) || hits(g, b, [to[0], b[1], to[2]], ahead) { return false; }
    }
    true
}

/// How far ahead of his feet the low ready leaves the muzzle, untucked. (gunwall.ts lowReach)
pub fn low_reach(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    let ahead = g.ch[c_2].muzzle_ahead;
    if g.ch[c_2].anim.long_gun { 0.145 + 0.7 * ahead } else { 0.241 + 0.74 * ahead }
}

/// A panel on a → b as a round sees one, counted. (gunwall.ts panel)
pub fn panel(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    g.stats.gunwall.asked += 1;
    wood_between(g, a, b)
}

/// A panel on a → b as a round sees one (a static or a leaf standing shut), and not a pane rounds go through. (gunwall.ts woodBetween)
pub fn wood_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    if !crate::combat::solid_between(g, a, b) { return false; }
    if !crate::glasspen::glass_pen_live(g) { return true; }
    let d = v3::sub(b, a);
    let l = v3::len(d);
    let h = if l > 1e-3 { crate::scene_collision::raycast(g, a, v3::scale(d, 1.0 / l), l) } else { None };
    match h {
        Some(h) => !(h.index < 1e6 && js::to_i32(g.col.boxes[BoxId(h.index as u32)].flags) & js::to_i32(crate::scene_boxes::box_flag().glass) != 0),
        None => true,
    }
}

/// A ray along a → b, `need` past it; a hit where he aims (`to`) does not count. (gunwall.ts walledNow's `cast`)
fn cast(g: &mut Game, to: Option<V3>, deep: bool, a: V3, b: V3, need: f64) -> bool {
    let d = v3::sub(b, a);
    let l = v3::len(d);
    if l < 1e-3 { return false; }
    let rd = v3::scale(d, 1.0 / l);
    g.stats.gunwall.asked += 1;
    let h = match crate::scene_collision::raycast(g, a, rd, l + need) { Some(h) => h, None => return false };
    let at_m = g.k.gunwall.at_m;
    !(match to { Some(to) => (deep || h.t > l - AT_IN) && v3::dist(to, v3::mad(a, rd, h.t)) < at_m, None => false })
}

/// Remember the pose and the segment that found the gun in the wood. (gunwall.ts walledNow's `held`)
fn held(g: &mut Game, c: CharId, sh: f64, hot: bool, a: V3, b: V3) -> bool {
    let (p, yaw, pitch) = (g.ch[c].pos, g.ch[c].aim_yaw, g.ch[c].aim_pitch);
    let serial = g.serial();
    row(g, c).hold = Some(Hold { x: p[0], z: p[2], yaw, pitch, sh, a, b, hot, serial });
    true
}

/// Is the gun he wants up kept down by a wall this frame? (gunwall.ts walledNow)
pub fn walled_now(g: &mut crate::game::Game, c_2: CharId, up: crate::gunwall::GunUp, to: Option<V3>, dt: f64, hot: bool, deep: bool) -> bool {
    let c = c_2;
    let aim_yaw = g.ch[c].aim_yaw;
    let fr = frame(g, c, aim_yaw);
    let ml = raised_line(g, c, up, aim_yaw, None);
    let s = *row(g, c);
    if let (true, Some(hd)) = (s.low, s.hold) {
        let (p, yaw, pitch) = (g.ch[c].pos, g.ch[c].aim_yaw, g.ch[c].aim_pitch);
        let same = m::hyp2(p[0] - hd.x, p[2] - hd.z) < HOLD_M && m::abs(v3::wrap_angle(yaw - hd.yaw)) < HOLD_RAD && m::abs(fr.sh - hd.sh) < HOLD_M && m::abs(pitch - hd.pitch) < HOLD_RAD;
        if same && (if hd.hot { panel(g, hd.a, hd.b) } else { cast(g, to, deep, hd.a, hd.b, 0.0) }) { return true; }
        row(g, c).hold = None;
    }
    let hand = g.ch[c].bones.get(crate::character::Bone::HandR).copied();
    let sh = g.ch[c].bones.get(crate::character::Bone::UpperArmR).copied();
    let bb = g.ch[c].bones_at;
    let (pos, muzzle) = (g.ch[c].pos, g.ch[c].muzzle);
    let settled = up == GunUp::Aim && hand.is_some() && sh.is_some()
        && (muzzle[0] - bb[0]) * fr.f[0] + (muzzle[2] - bb[1]) * fr.f[2] > (ml.b[0] - pos[0]) * fr.f[0] + (ml.b[2] - pos[2]) * fr.f[2] - SETTLED_M
        && m::abs(muzzle[1] - ml.b[1]) < 2.0 * SETTLED_M;
    if hot {
        if s.low { return panel(g, ml.a, ml.b); }
        let in_wood = settled && crate::combat::arm_through(g, c) && { let (from, to_m) = (sh.unwrap(), g.ch[c].muzzle); panel(g, from, to_m) };
        let wood = if in_wood { row(g, c).wood + dt } else { 0.0 };
        row(g, c).wood = wood;
        if !(wood >= HOT_SECS) { return false; }
        let to_m = g.ch[c].muzzle;
        return held(g, c, fr.sh, hot, sh.unwrap(), to_m);
    }
    row(g, c).wood = 0.0;
    let lead = closing(s, fr.f) * g.k.gunwall.lead_s;
    if !s.low && settled {
        let dx = pos[0] - bb[0];
        let dz = pos[2] - bb[1];
        let mv = |p: V3| -> V3 { [p[0] + dx, p[1], p[2] + dz] };
        let h = mv(hand.unwrap());
        let mz = mv(muzzle);
        let sm = mv(sh.unwrap());
        let down_m = g.k.gunwall.down_m;
        return if cast(g, to, deep, h, mz, down_m + lead) { held(g, c, fr.sh, hot, h, mz) } else { cast(g, to, deep, sm, h, 0.0) && held(g, c, fr.sh, hot, sm, h) };
    }
    let need = (if s.low { g.k.gunwall.up_m } else { g.k.gunwall.down_m }) + lead;
    let w = if s.low { GUN_HALF_LOW } else { GUN_HALF };
    let gun_yaw = crate::character::character_gun_yaw(g, c);
    let aim_yaw_now = g.ch[c].aim_yaw;
    let lines = if m::abs(v3::wrap_angle(gun_yaw - aim_yaw_now)) > 0.1 { vec![ml, raised_line(g, c, up, gun_yaw, None)] } else { vec![ml] };
    for ln in &lines {
        for o in [0.0, -w, w] {
            let off: V3 = [fr.r[0] * o, 0.0, fr.r[2] * o];
            if cast(g, to, deep, v3::add(ln.a, off), v3::add(ln.b, off), need) { return true; }
        }
    }
    false
}

/// YOUR BREACH SHOT: where the raised gun's own line meets a lock a shell takes, inside the gun's reach and a stride: that point; else none. (gunwall.ts breachAim)
pub fn breach_aim(g: &mut crate::game::Game, c_2: CharId, b: crate::weapons::PelletSpecBreach) -> Option<V3> {
    let aim_yaw = g.ch[c_2].aim_yaw;
    let ml = raised_line(g, c_2, GunUp::Aim, aim_yaw, None);
    let rd = v3::sub(ml.b, ml.a);
    let l = v3::len(rd);
    if l < 1e-3 { return None; }
    let dir = v3::scale(rd, 1.0 / l);
    let h = crate::scene_collision::raycast(g, ml.a, dir, BREACH_AIM_M)?;
    if h.index < 1e6 { return None; }
    let bx = g.col.dynamic_boxes.get((h.index - 1e6) as usize).copied()?;
    let d = g.doors.list.iter().copied().find(|&x| g.dr[x].box_ == bx)?;
    if !crate::doors::door_latched(g, d) || !crate::doors::door_locked(g, d) || g.dr[d].lock_id.starts_with("wedge:") { return None; }
    let p = v3::mad(ml.a, dir, h.t);
    let u = crate::doors::door_dir(g, d);
    let (hinge, width) = (g.dr[d].def.hinge, g.dr[d].def.width);
    let along = (p[0] - hinge[0]) * u[0] + (p[2] - hinge[1]) * u[1];
    if along < width - b.latch_m { None } else { Some(p) }
}

/// This frame's answer for a character, asked once a frame and then remembered. (gunwall.ts gunWalled)
pub fn gun_walled(g: &mut crate::game::Game, c_2: CharId, up: Option<crate::gunwall::GunUp>, to: Option<V3>, dt: f64, hot: Option<bool>, deep: Option<bool>) -> bool {
    let (hot, deep) = (hot.unwrap_or(false), deep.unwrap_or(false));
    let now = g.time;
    let s = st(g, c_2);
    if s.t != now {
        s.t = now;
        track(g, c_2);
        let was = row(g, c_2).low;
        let low = match up {
            Some(up) if !showing(g) => walled_now(g, c_2, up, to, dt, hot, deep),
            _ => false,
        };
        let r = row(g, c_2);
        r.low = low;
        if !low { r.hold = None; }
        r.rose = was && !low;
        if low {
            seam_stats_lowered_s(g, dt);
            if !was { g.stats.gunwall.lowered += 1; }
        }
    }
    row(g, c_2).low
}

/// The last answer, read only, if it is this frame's or the last's. (gunwall.ts gunWallIs)
pub fn gun_wall_is(g: &mut crate::game::Game, c_2: CharId) -> bool {
    match g.m.gunwall.st.get(c_2) {
        Some(s) => s.low && g.time - s.t < FRESH_S,
        None => false,
    }
}

/// For one of the men: a gun kept down by a wall is `muzzleDown`, held there every frame it is asked. (gunwall.ts gunWallCheck)
pub fn gun_wall_check(g: &mut crate::game::Game, gd: GuardId, up: Option<crate::gunwall::GunUp>, to: Option<V3>, dt: f64, hot: Option<bool>) -> bool {
    let hot = hot.unwrap_or(false) || fight_on(g);
    let c = g.gd[gd].ch;
    let def = g.fa[g.gd[gd].weapon].def;
    let deep = g.m.weapons.weapons.get(def).unwrap().pellets.is_some();
    let low = gun_walled(g, c, up, to, dt, Some(hot), Some(deep));
    if low {
        g.gd[gd].muzzle_down = true;
        g.gd[gd].muzzle_clear_t = 0.0;
    } else if hot && g.gd[gd].muzzle_down && g.m.gunwall.st.get(c).map_or(false, |s| s.rose) && g.gd[gd].muzzle_eval_t == g.time && g.gd[gd].muzzle_clear_t > 0.0 {
        g.gd[gd].muzzle_down = false;
        g.gd[gd].muzzle_clear_t = 0.0;
    }
    low
}

/// One heading's rays along the low carriage: the most `w` they ask for. (gunwall.ts gunWallPose's `one`)
fn pose_one(g: &mut Game, c: CharId, yaw: f64, reach0: f64, span: f64, w: &mut f64) {
    let fr = frame(g, c, yaw);
    let af = AtF { f: fr.f, r: fr.r };
    let long = g.ch[c].anim.long_gun;
    let bb = g.ch[c].bones_at;
    let sh_f = match g.ch[c].bones.get(crate::character::Bone::UpperArmR) {
        Some(sh) => (sh[0] - bb[0]) * fr.f[0] + (sh[2] - bb[1]) * fr.f[2],
        None => SH_REST,
    };
    let reach = reach0 + v3::clamp((sh_f - SH_REST) * SH_CARRY, 0.0, REACH_OVER);
    let pos = g.ch[c].pos;
    let a = if long { at(pos, af, 0.0, 0.18, fr.sh - 0.18) } else { at(pos, af, 0.0, 0.08, fr.sh - 0.3) };
    let b = if long { at(pos, af, reach0, 0.03, fr.sh - 0.51) } else { at(pos, af, reach0, 0.08, fr.sh - 0.49) };
    let d = v3::sub(b, a);
    let l = v3::len(d);
    let rd = v3::scale(d, 1.0 / l);
    let s = *row(g, c);
    let (tuck_m, tuck_lead_s) = (g.k.gunwall.tuck_m, g.k.gunwall.tuck_lead_s);
    let lead = closing(s, fr.f) * tuck_lead_s;
    for o in [0.0, -TUCK_HALF, TUCK_HALF] {
        g.stats.gunwall.asked += 1;
        let h = match crate::scene_collision::raycast(g, [a[0] + fr.r[0] * o, a[1], a[2] + fr.r[2] * o], rd, l * (reach + tuck_m + lead) / reach0) { Some(h) => h, None => continue };
        let room = h.t / l * reach0 - tuck_m - lead;
        let k = (reach - room) / span;
        *w = m::max(*w, if k <= 1.0 { m::max(0.0, k) } else { 1.0 + v3::clamp((k - 1.0) * span / crate::character::wall_tuck().over, 0.0, 1.0) });
    }
}

/// The low carriages pulled in by the room they have: `gunWall` for this frame. (gunwall.ts gunWallPose)
pub fn gun_wall_pose(g: &mut crate::game::Game, c_2: CharId) {
    let c = c_2;
    if showing(g) {
        if g.ch[c].anim.gun_wall != 0.0 { g.ch[c].anim.gun_wall = 0.0; }
        return;
    }
    let stance = g.ch[c].anim.stance;
    let carry = match g.ch[c].anim.long_stance {
        Some(ls) => ls,
        None => if stance == crate::character::Stance::LowReady || stance == crate::character::Stance::None { crate::character::LongStance::LowLong } else { crate::character::LongStance::PortLong },
    };
    let low_now = g.ch[c].anim.upper != crate::character::UpperMode::Aim
        && (if g.ch[c].anim.long_gun { carry == crate::character::LongStance::LowLong || carry == crate::character::LongStance::PatrolLong } else { stance == crate::character::Stance::LowReady });
    if !low_now || !g.ch[c].alive {
        g.ch[c].anim.gun_wall = 0.0;
        return;
    }
    st(g, c);
    track(g, c);
    let reach0 = low_reach(g, c);
    let span = reach0 - TUCK_REACH;
    let mut w = 0.0;
    let (aim_yaw, body_yaw) = (g.ch[c].aim_yaw, g.ch[c].body_yaw);
    pose_one(g, c, aim_yaw, reach0, span, &mut w);
    if m::abs(v3::wrap_angle(aim_yaw - body_yaw)) > 0.15 { pose_one(g, c, body_yaw, reach0, span, &mut w); }
    g.ch[c].anim.gun_wall = w;
    if w > 0.0 { g.stats.gunwall.tucked += 1; }
}

/// Sam's pistol: the low ready this rule put him in is taken off again by it and by nobody else. (gunwall.ts gunWallStance)
pub fn gun_wall_stance(g: &mut crate::game::Game, c_2: CharId, set: bool) {
    if set {
        st(g, c_2);
        if g.ch[c_2].anim.stance == crate::character::Stance::None {
            g.ch[c_2].anim.stance = crate::character::Stance::LowReady;
            row(g, c_2).mine = true;
        }
    } else if g.m.gunwall.st.get(c_2).map_or(false, |s| s.mine) {
        if g.ch[c_2].anim.stance == crate::character::Stance::LowReady { g.ch[c_2].anim.stance = crate::character::Stance::None; }
        row(g, c_2).mine = false;
    }
}

/// A restart: the fight the floor remembered was the last try's. (gunwall.ts gunWallFresh)
pub fn gun_wall_fresh(g: &mut crate::game::Game) {
    g.m.gunwall.fight = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_with(long: bool, ahead: f64, pitch: f64, arm: Option<f64>) -> (Game, CharId) {
        let mut g = Game::blank();
        let mut ch = crate::character::Character::blank();
        ch.pos = [1.25, 0.1, -3.5];
        ch.aim_pitch = pitch;
        ch.muzzle_ahead = ahead;
        ch.anim.long_gun = long;
        if let Some(y) = arm { ch.bones.set(crate::character::Bone::UpperArmR, [0.0, y, 0.0]); }
        let c = g.ch.push(ch);
        (g, c)
    }
    fn bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }

    #[test]
    fn closing_is_never_negative() {
        let mut s = St::zeroed();
        s.vx = 2.0;
        s.vz = -1.0;
        assert_eq!(closing(s, [1.0, 0.0, 0.0]), 2.0);
        assert_eq!(closing(s, [-1.0, 0.0, 0.0]).to_bits(), 0f64.to_bits());
    }

    /// The doubles are the TypeScript's own: gunWallModel.raisedLine and lowReach under bun with EXACT on, the same four men.
    #[test]
    fn raised_line_and_low_reach_are_the_typescripts_bits() {
        let cases: [(bool, f64, f64, Option<f64>, GunUp, f64, [u64; 3], [u64; 3], u64); 4] = [
            (true, 0.52, 0.3, Some(1.31), GunUp::Aim, 0.7,
             [0x3ff1c7833830f4b3, 0x3ff3333333333333, 0xc00b1095b70a6e79], [0x3ff7fba7bc99f599, 0x3ff62dc8b31a35a5, 0xc00761d4838b84f7], 0x3fe049ba5e353f7d),
            (false, 0.255, -0.9, Some(1.0), GunUp::Aim, -2.1,
             [0x3ff4295b63e2ce63, 0x3fe8f5c28f5c28f6, 0xc00c235b67ffe5b2], [0x3feacd43eb4ea0a7, 0x3fd788686f026e72, 0xc00e1d76542f772e], 0x3fdb80346dc5d638),
            (true, 0.52, 0.0, None, GunUp::Port, 1.3,
             [0x3ff5cc8774ba509f, 0x3ff3851eb851eb85, 0xc00ac1053264d038], [0x3ffb0fc830a45f96, 0x3ffa147ae147ae14, 0xc00b8f3866f83c9c], 0x3fe049ba5e353f7d),
            (false, 0.255, 0.1, Some(1.2), GunUp::Port, 3.0,
             [0x3ff51bd9c41bb9f7, 0x3ff3d70a3d70a3d7, 0xc00bebc4df851ebf], [0x3ff628a22a7c460c, 0x3ff451eb851eb852, 0xc00f9a8f064eecfc], 0x3fdb80346dc5d638),
        ];
        for (long, ahead, pitch, arm, up, yaw, a, b, low) in cases {
            let (mut g, c) = game_with(long, ahead, pitch, arm);
            let l = raised_line(&mut g, c, up, yaw, None);
            assert_eq!(bits(l.a), a);
            assert_eq!(bits(l.b), b);
            assert_eq!(low_reach(&mut g, c).to_bits(), low);
        }
    }

    #[test]
    fn track_reads_a_step_and_not_a_jump_or_a_gap() {
        let (mut g, c) = game_with(true, 0.5, 0.0, None);
        g.time = 1.0;
        st(&mut g, c);
        track(&mut g, c);                                      // the first sample: a gap of 2 s, standing
        assert_eq!(row(&mut g, c).vx, 0.0);
        g.time = 1.05;
        g.ch[c].pos[0] += 0.1;
        track(&mut g, c);
        let h = 1.05 - 1.0;
        assert_eq!(row(&mut g, c).vx.to_bits(), ((1.25 + 0.1 - 1.25) / h as f64).to_bits());
        let vx = row(&mut g, c).vx;
        g.ch[c].pos[0] += 5.0;
        track(&mut g, c);                                      // the same frame: asked once
        assert_eq!(row(&mut g, c).vx, vx);
        g.time = 1.1;
        track(&mut g, c);                                      // 5 m in a frame is not a step
        assert_eq!(row(&mut g, c).vx, 0.0);
    }

    #[test]
    fn the_stance_is_taken_off_only_by_the_rule_that_set_it() {
        let (mut g, c) = game_with(false, 0.2, 0.0, None);
        gun_wall_stance(&mut g, c, false);                     // nobody asked yet: no row is made
        assert!(g.m.gunwall.st.get(c).is_none());
        g.ch[c].anim.stance = crate::character::Stance::None;
        gun_wall_stance(&mut g, c, true);
        assert!(g.ch[c].anim.stance == crate::character::Stance::LowReady && row(&mut g, c).mine);
        gun_wall_stance(&mut g, c, false);
        assert!(g.ch[c].anim.stance == crate::character::Stance::None && !row(&mut g, c).mine);
        g.ch[c].anim.stance = crate::character::Stance::LowReady;   // somebody else's low ready
        gun_wall_stance(&mut g, c, true);
        assert!(!row(&mut g, c).mine);
        gun_wall_stance(&mut g, c, false);
        assert!(g.ch[c].anim.stance == crate::character::Stance::LowReady);
    }

    #[test]
    fn an_answer_goes_stale() {
        let (mut g, c) = game_with(false, 0.2, 0.0, None);
        g.time = 2.0;
        assert!(!gun_wall_is(&mut g, c));
        st(&mut g, c);
        row(&mut g, c).low = true;
        row(&mut g, c).t = 2.0;
        assert!(gun_wall_is(&mut g, c));
        g.time = 2.06;
        assert!(!gun_wall_is(&mut g, c));
    }

    /// `st` hands back the row itself, and this frame's answer lands in it: asked once, the gun let go, the hold forgotten.
    #[test]
    fn the_answer_lands_in_his_row_once_a_frame() {
        let (mut g, c) = game_with(false, 0.2, 0.0, None);
        g.time = 3.0;
        st(&mut g, c).low = true;
        st(&mut g, c).hold = Some(Hold::zeroed());
        assert!(!gun_walled(&mut g, c, None, None, 0.016, None, None));
        let s = *row(&mut g, c);
        assert!(s.t == 3.0 && s.vt == 3.0 && !s.low && s.rose && s.hold.is_none());
        row(&mut g, c).low = true;
        assert!(gun_walled(&mut g, c, None, None, 0.016, None, None));   // the same frame: remembered, not asked again
        assert!(row(&mut g, c).rose);
    }

    #[test]
    fn a_new_row_holds_the_typescripts_numbers() {
        let (mut g, c) = game_with(false, 0.2, 0.0, None);
        let s = *st(&mut g, c);
        assert!(s.t == -1.0 && s.vt == -1.0 && s.px == 1.25 && s.pz == -3.5);
    }

    #[test]
    fn the_floor_remembers_a_fight_and_no_longer_than_a_new_one_would() {
        let (mut g, _c) = game_with(false, 0.2, 0.0, None);
        g.k.gunwall.fight_s = 5.0;
        g.time = 10.0;
        assert!(!fight_on(&mut g));                            // no man, no fight
        g.m.gunwall.fight = Some(GunwallModFIGHTValue { t: 9.0, until: 100.0 });
        assert!(fight_on(&mut g));                             // the clock gone back: cut to a fight seen now
        assert_eq!(g.m.gunwall.fight.unwrap().until, 15.0);
        g.k.gunwall.fight_s = 0.0;
        assert!(!fight_on(&mut g));
    }
}
