//! src/game/guards.ts lines 806 to 1128
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A bone of a body, if the body has it (the TypeScript's `c.bones.x`).
fn bone(g: &mut Game, c: CharId, b: crate::character::Bone) -> Option<V3> {
    g.ch[c].bones.get(b).copied()
}

/// Is the shield up: a living man in Sam's arm, Sam on his feet, in any phase of the hold. (guards.ts shieldUp)
pub fn shield_up(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let held = match &g.pl[p].holding {
        Some(h) => h.g,
        None => return false,
    };
    if g.pl[p].down {
        return false;
    }
    let hc = g.gd[held].ch;
    g.gd[held].state != crate::game::GuardState::Dead
        && g.ch[hc].alive
        && matches!(&g.gd[held].held, Some(h) if h.by == crate::game::GuardHeldBy::Player)
        && crate::game::game_on_floor(g, held)
}

/// The pair as the observers reason about it this frame; None when nobody is held. `key` is the hold's serial. (guards.ts pairOf)
pub fn pair_of(g: &mut crate::game::Game) -> Option<crate::guards::Pair> {
    let p = g.player;
    if g.pl[p].holding.is_none() || !shield_up(g) {
        return None;
    }
    let (key, held) = match &g.pl[p].holding {
        Some(h) => (h.serial, h.g),
        None => return None,
    };
    let pc = g.pl[p].ch;
    let hc = g.gd[held].ch;
    let (pc_pos, hc_pos) = (g.ch[pc].pos, g.ch[hc].pos);
    let sh = bone(g, pc, crate::character::Bone::Hips).unwrap_or(pc_pos);
    let hh = bone(g, hc, crate::character::Bone::Hips).unwrap_or(hc_pos);
    let sam: V3 = [sh[0], 0.0, sh[2]];
    let held_axis: V3 = [hh[0], 0.0, hh[2]];
    Some(crate::guards::Pair {
        key,
        held,
        sam,
        held_axis,
        centre: v3::lerp3(sam, held_axis, 0.5),
        facing: g.ch[pc].body_yaw,
        aim: [sam[0], pc_pos[1] + STANDOFF_AIM_Y, sam[2]],
        vel: v3::copy(g.ch[pc].vel),
    })
}

/// A checkpoint put the time back: the floor's bark clock must not sit in the future. (guards.ts forgetGuardClocks)
pub fn forget_guard_clocks(g: &mut crate::game::Game) {
    g.m.guards.floor_bark_t = None;
}

/// The pair in this man's open view: either body's chest or head inside his cone and range with nothing solid between. (guards.ts pairInView)
pub fn pair_in_view(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::guards::Pair) -> bool {
    if g.player_invisible || g.time < g.gd[gd].dazzled_until || g.gd[gd].smoke_trans < 0.3 || crate::pairorders::pair_gripped(g, gd) || crate::doze::doze_asleep(g, gd) {
        return false;
    }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let eye = bone(g, c, crate::character::Bone::Head).unwrap_or_else(|| v3::add(pos, [0.0, 1.65, 0.0]));
    let chest_dir = g.ch[c].chest_dir;
    let head_dir = crate::character::character_head_dir(g, c);
    let look = v3::lerp3(chest_dir, head_dir, 0.5);
    let ll = js::or(m::hyp2(look[0], look[2]), 1.0);
    let (lx, lz) = (look[0] / ll, look[2] / ll);
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let hc = g.gd[p.held].ch;
    let seen = g.pl[pl].visibility;
    let lit = m::max(seen, held_visibility(g, p));
    let reach = eye_reach(g, lit);
    let view_m = m::max(STANDOFF_VIEW_M, reach);
    let (hp, pp) = (g.ch[hc].pos, g.ch[pc].pos);
    let points: [V3; 4] = [
        bone(g, hc, crate::character::Bone::Chest).unwrap_or_else(|| v3::add(hp, [0.0, 1.1, 0.0])),
        bone(g, hc, crate::character::Bone::Head).unwrap_or_else(|| v3::add(hp, [0.0, 1.6, 0.0])),
        bone(g, pc, crate::character::Bone::Chest).unwrap_or_else(|| v3::add(pp, [0.0, 1.1, 0.0])),
        bone(g, pc, crate::character::Bone::Head).unwrap_or_else(|| v3::add(pp, [0.0, 1.6, 0.0])),
    ];
    for q in points {
        let (tx, ty, tz) = (q[0] - eye[0], q[1] - eye[1], q[2] - eye[2]);
        if m::hyp3(tx, ty, tz) > view_m {
            continue;
        }
        let fl = m::hyp2(tx, tz);
        if fl > 0.3 && (tx * lx + tz * lz) / fl < m::cos((if fl < 3.5 { 100.0 } else { STANDOFF_VIEW_DEG }) * v3::DEG) {
            continue;
        }
        if !crate::scene_collision::segment_blocked(g, eye, q, None, None, None) {
            return true;
        }
    }
    false
}

/// What a watching man's eye gets off the pair. (guards.ts pairSightRate)
pub fn pair_sight_rate(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::guards::Pair, dist: f64) -> f64 {
    let pl = g.player;
    let seen = g.pl[pl].visibility;
    let light = m::max(seen, held_visibility(g, p));
    let dist_f = dist_term(g, dist);
    let detect = g.tune.detect_rate;
    let lit = m::pow(light, 1.5);
    let smoke = v3::clamp((g.gd[gd].smoke_trans - 0.3) / 0.5, 0.0, 1.0);
    let keen = if escalation_of(g) >= 1.0 { KEEN } else { 1.0 };
    let load = crate::kitload::load_seen(g, crate::kitload::LoadSeenWho::Player);
    let mut rate = detect * lit * dist_f * smoke * keen * PAIR_LEGIBLE * load;
    if dist < 1.8 {
        rate = m::max(rate, 3.5);
    }
    let state = g.gd[gd].state;
    if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search {
        rate *= 2.2;
    }
    rate
}

/// Forget the probe's answers under the held man's keys. (guards.ts forgetHeldMeter)
pub fn forget_held_meter(g: &mut crate::game::Game, gd: GuardId) {
    let id = g.ch[g.gd[gd].ch].id;
    crate::game::game_light_forget(g, &format!("heldC{}", id));
    crate::game::game_light_forget(g, &format!("heldH{}", id));
}

/// The held man's own light meter while he is held, kept per hold; started over for a new hold and for a pair unmetered for a gap that has moved. (guards.ts heldVisibility)
pub fn held_visibility(g: &mut crate::game::Game, p: &mut crate::guards::Pair) -> f64 {
    let hd = p.held;
    let hc = g.gd[hd].ch;
    let now = g.time;
    let meter = held_meter();
    let mut row = g.m.guards.held_meters.get(p.key).copied();
    let pos = g.ch[hc].pos;
    let stale = match &row {
        None => true,
        Some(r) => now - r.t > meter.gap_secs && m::hyp2(r.at[0] - pos[0], r.at[2] - pos[2]) > meter.moved_m,
    };
    if stale {
        row = None;
        forget_held_meter(g, hd);
    }
    if let Some(r) = &row {
        if r.t == now {
            return r.vis;
        }
    }
    let e = meter_of(g, hc, "held");
    let dt = match &row {
        Some(r) => m::min(0.1, m::max(0.0, now - r.t)),
        None => 0.1,
    };
    let from = match &row {
        Some(r) => r.vis,
        None => crate::game::game_visibility_of(g, e),
    };
    let to = crate::game::game_visibility_of(g, e);
    let vis = v3::damp(from, to, 6.0, dt);
    let at = v3::copy(g.ch[hc].pos);
    g.m.guards.held_meters.set(p.key, crate::guards::GuardsModHeldMetersValue { vis, t: now, at });
    g.gd[hd].visibility = vis;
    vis
}

/// `gd.lastKnown = at`, a whole write: the stamp says so (doze.ts asks by identity).
fn fix_on(g: &mut Game, gd: GuardId, at: V3) {
    g.gd[gd].last_known = Some(v3::copy(at));
    g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1);
}

/// Perception's share: is a held colleague in his view this frame? If so he is in the standoff and all his eyes-on-Sam is refreshed off the pair. (guards.ts standoffPerceive)
pub fn standoff_perceive(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) -> bool {
    let dt = dt.unwrap_or(1.0 / 60.0);
    if g.quiet_utility || g.gd[gd].hold || g.gd[gd].pinned {
        return false;
    }
    let mut p = match pair_of(g) {
        Some(p) if p.held != gd => p,
        _ => return false,
    };
    let held_team = g.gd[p.held].team;
    if (held_team != g.gd[gd].team && held_team != crate::teams::Team::Civilian) || !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player) {
        return false;
    }
    if !pair_in_view(g, gd, &mut p) {
        return false;
    }
    let now = g.time;
    let pl = g.player;
    let pp = g.ch[g.pl[pl].ch].pos;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let eye = bone(g, c, crate::character::Bone::Head).unwrap_or_else(|| v3::add(pos, [0.0, 1.65, 0.0]));
    let dist = v3::dist(eye, p.centre);
    let rate = pair_sight_rate(g, gd, &mut p, dist);
    let in_it = match g.gd[gd].standoff {
        Some(s) => g.tab.guards_standoff[s].pair == p.held,
        None => false,
    };
    if !in_it {
        // not yet in it: he acquires the pair the way he acquires Sam alone
        if rate <= 0.05 {
            return false;
        }
        let was = g.gd[gd].awareness;
        g.gd[gd].awareness = v3::clamp(was + rate * dt, 0.0, 1.0);
        if !far_faint(g, gd, dist, was) {
            fix_on(g, gd, pp);
        }
        if g.gd[gd].awareness < 0.8 {
            return rate > 0.25;
        }
    } else if rate <= 0.05 {
        return false;
    }
    if !in_it {
        let side = pick_side(g, gd, p);
        let s = g.tab.guards_standoff.push(crate::guards::Standoff {
            pair: p.held,
            since: now,
            side,
            way: 0.0,
            orbit_goal: None,
            replan_t: 0.0,
            rest_t: 0.0,
            seen_t: now,
            last_bark_t: -100.0,
            line_open: false,
            shots: 0.0,
            orbit_m: 0.0,
            stalls: 0.0,
            cornered: false,
            backing: false,
        });
        g.gd[gd].standoff = Some(s);
        let who = g.gd[p.held].callsign.clone();
        let me = &mut g.gd[gd];
        me.witness.saw_held = Some(crate::game::WitnessSawHeld { who, t: now });
        me.fire_cd = m::max(me.fire_cd, 0.8);
        me.path = Vec::new();
        me.path_serial = me.path_serial.wrapping_add(1);
        me.path_goal = None;
        me.path_goal_orbit = false;
        me.stall_ref = None;
        me.pieing = None;
        me.search_plan = None;
        if !g.m.guards.holds_called.has(p.key) {
            g.m.guards.holds_called.add(p.key);
            hostage_call(g, gd, p);
        } else {
            standoff_bark(g, gd, StandoffMoment::Enter, Some(StandoffBarkForce::Bool(true)), None);
        }
    }
    if let Some(s) = g.gd[gd].standoff {
        g.tab.guards_standoff[s].seen_t = now;
    }
    crate::gunhold::gun_hold_see(g, gd, p);
    g.gd[gd].awareness = 1.0;
    fix_on(g, gd, pp);
    let me = &mut g.gd[gd];
    me.last_seen_t = now;
    me.sight_pos = v3::copy(pp);
    me.sight_vel = v3::copy(p.vel);
    me.sight_t = now;
    me.led_t = now;
    me.alert_best = 1e9;
    me.alert_stall_t = 0.0;
    me.alert_ref = None;
    me.chase_t = 0.0;
    true
}

/// The first man to see it calls it in, and the floor answers: his side comes to the place, and the staff's alarm goes to lockdown. Once a hold. (guards.ts hostageCall)
pub fn hostage_call(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) {
    let c = g.gd[gd].ch;
    let where_ = crate::dialogue::room_name(g, p.sam);
    let text = match g.gd[p.held].civ {
        Some(cv) => format!("he's got {}, the {} — {}! hold your fire — civilian in his arm, nobody push him", g.cv[cv].name, g.cv[cv].label, where_),
        None => {
            let name = crate::teams::spoken_name(g, p.held).unwrap_or_else(|| String::from("one of ours"));
            format!("he's got {} — {}! hold your fire, nobody push him", name, where_)
        }
    };
    crate::game::game_say(g, gd, &text, Some(true), None, None);
    let pos = g.ch[c].pos;
    let at = bone(g, c, crate::character::Bone::Head).unwrap_or(pos);
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.9), None);
    if g.time - g.last_sting_t > 8.0 {
        crate::audio_iface::game_audio_like_play(g, "alertSting", None, Some(0.75), None);
        g.last_sting_t = g.time;
    }
    let team = g.gd[gd].team;
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || o == p.held || g.gd[o].state == crate::game::GuardState::Dead || g.gd[o].held.is_some() || g.gd[o].team != team {
            continue;
        }
        let state = g.gd[o].state;
        let was_calm = state == crate::game::GuardState::Patrol || state == crate::game::GuardState::Suspicious;
        g.gd[o].awareness = m::max(g.gd[o].awareness, 0.92);
        fix_on(g, o, p.sam);
        if was_calm {
            let d = v3::dist(g.ch[g.gd[o].ch].pos, p.sam);
            let roll = crate::math_dice::sim_random(g);
            g.gd[o].react_t = m::max(g.gd[o].react_t, 0.25 + m::min(1.2, d / 25.0) + roll * 0.4);
        }
    }
    if team != crate::teams::Team::Guard {
        return;
    }
    g.alarm.pos[0] = p.sam[0];
    g.alarm.pos[1] = 0.0;
    g.alarm.pos[2] = p.sam[2];
    g.alarm.placed = true;
    raise_escalation(g, 2.0, Some(gd), Some(RaiseEscalationCause::Hostage), None);
}

/// pickSide's `room`: how far round the ring is open going way `d`, every 20°, and whether all of it to their rear is.
fn ring_room(g: &mut Game, c_2: V3, brg: f64, to_rear: f64, short: f64, d: f64) -> (i32, bool) {
    let span = if d == short { m::abs(to_rear) } else { 2.0 * m::PI - m::abs(to_rear) };
    let (mut n, mut full) = (0, true);
    let mut a = 20.0 * v3::DEG;
    while a <= span + 1e-6 {
        let b = brg + d * a;
        let q: V3 = [c_2[0] + m::sin(b) * STANDOFF_ORBIT_M, 0.0, c_2[2] + m::cos(b) * STANDOFF_ORBIT_M];
        if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || no_sight_at(g, q[0], q[2], c_2) {
            full = false;
            break;
        }
        n += 1;
        a += 20.0 * v3::DEG;
    }
    (n, full)
}

/// Is something solid between chest height over (x, z) and chest height over the ring's centre?
fn no_sight_at(g: &mut Game, x: f64, z: f64, c_2: V3) -> bool {
    crate::scene_collision::segment_blocked(g, [x, 1.2, z], [c_2[0], 1.2, c_2[2]], None, None, None)
}

/// Which way round the ring this man works: the other way from a colleague on this pair; else the short way to their rear if open, else whichever has more open ring. (guards.ts pickSide)
pub fn pick_side(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> f64 {
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        if let Some(s) = g.gd[o].standoff {
            if g.tab.guards_standoff[s].pair == p.held {
                return if g.tab.guards_standoff[s].side > 0.0 { -1.0 } else { 1.0 };
            }
        }
    }
    let c_2 = p.centre;
    let pos = g.ch[g.gd[gd].ch].pos;
    let brg = m::atan2(pos[0] - c_2[0], pos[2] - c_2[2]);
    let to_rear = v3::wrap_angle(p.facing + m::PI - brg);
    let short = if to_rear < 0.0 { -1.0 } else { 1.0 };
    let (rs_n, rs_full) = ring_room(g, c_2, brg, to_rear, short, short);
    let (rl_n, _) = ring_room(g, c_2, brg, to_rear, short, -short);
    if rs_full || rs_n >= rl_n {
        short
    } else if short > 0.0 {
        -1.0
    } else {
        1.0
    }
}

/// The sum of axisToSegment, with the man's axis and his soles and crown read.
fn axis_measure(from: V3, to: V3, ax: f64, az: f64, y0: f64, y1: f64) -> f64 {
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let l2 = dx * dx + dz * dz;
    let t = if l2 > 1e-6 { v3::clamp(((ax - from[0]) * dx + (az - from[2]) * dz) / l2, 0.0, 1.0) } else { 0.0 };
    let (px, py, pz) = (from[0] + dx * t, from[1] + dy * t, from[2] + dz * t);
    let ddy = if py < y0 { y0 - py } else if py > y1 { py - y1 } else { 0.0 };
    m::hyp3(px - ax, pz - az, ddy)
}

/// Metres between colleague `o`'s body axis and the segment `from` → `to`. (guards.ts axisToSegment)
pub fn axis_to_segment(g: &mut crate::game::Game, from: V3, to: V3, o: GuardId) -> f64 {
    let c = g.gd[o].ch;
    let pos = g.ch[c].pos;
    let h = bone(g, c, crate::character::Bone::Hips).unwrap_or(pos);
    let y0 = pos[1] + 0.05;
    let y1 = match bone(g, c, crate::character::Bone::HeadTop) {
        Some(top) => top[1],
        None => pos[1] + 1.75,
    };
    axis_measure(from, to, h[0], h[2], y0, y1)
}

/// The fire rule: may this man's round go, at Sam, past the colleague in his arm? (guards.ts flankShot)
pub fn flank_shot(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> crate::guards::FlankShot {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let from = bone(g, c, crate::character::Bone::UpperArmR).unwrap_or_else(|| v3::add(pos, [0.0, 1.4, 0.0]));
    let dist = v3::dist_xz(pos, p.centre);
    let brg = m::atan2(pos[0] - p.sam[0], pos[2] - p.sam[2]);
    let rear = m::abs(v3::wrap_angle(brg - (p.facing + m::PI)));
    let to = p.aim;
    let d = v3::sub(to, from);
    let l = v3::len(d);
    let impact = if l > SAM_R + 0.05 { v3::mad(from, d, (l - SAM_R) / l) } else { to };
    let clear = axis_to_segment(g, from, impact, p.held);
    let ok = rear < STANDOFF_REAR_DEG * v3::DEG
        && clear >= STANDOFF_CLEAR_M
        && dist <= STANDOFF_FIRE_MAX_M
        && dist >= STANDOFF_MIN_M - 0.6
        && clear_shot(g, c, to);
    crate::guards::FlankShot { ok, rear, clear, dist }
}

/// STANDOFF_LINES, a moment's lines for one temperament.
fn lines_of(moment: StandoffMoment, t: crate::game::Temperament) -> &'static [&'static str] {
    use crate::game::Temperament::{Hard, Nervy, Steady};
    match (moment, t) {
        (StandoffMoment::Enter, Steady) => &["{held}, hold still — we've got you", "easy. easy. nobody has to get hurt here"],
        (StandoffMoment::Enter, Nervy) => &["{held}! —jesus— okay, okay, nobody shoot!", "oh god— he's got {held}—"],
        (StandoffMoment::Enter, Hard) => &["let {him} go. now.", "you. hands off {him}."],
        (StandoffMoment::NoShot, Steady) => &["let {him} go and we talk about it", "{held}, stay loose — I'm coming round"],
        (StandoffMoment::NoShot, Nervy) => &["{held}! —I can't— I haven't got a line—", "don't you hurt {him}— don't—"],
        (StandoffMoment::NoShot, Hard) => &["you've got nowhere to take {him}", "the second {he} slips, you're done"],
        (StandoffMoment::Line, Nervy) => &["I— I think I have him—"],
        (StandoffMoment::Line, Hard) => &["there you are."],
        (StandoffMoment::Cornered, Nervy) => &["just— just let {him} go, please—"],
        (StandoffMoment::Cornered, Hard) => &["one way out of this for you. through us."],
        (StandoffMoment::Dropped, Nervy) => &["{held}! —he's— shoot, SHOOT—"],
        (StandoffMoment::Lost, Nervy) => &["where— where did they go—"],
        (StandoffMoment::HitHim, Nervy) => &["no— no no no— {held}—"],
        (StandoffMoment::HitHim, Hard) => &["…{held}'s hit. that's on him. keep on him—"],
        _ => &[],
    }
}

/// STANDOFF_LINES, a moment's lines for anybody ('*').
fn lines_any(moment: StandoffMoment) -> &'static [&'static str] {
    match moment {
        StandoffMoment::Enter => &[],
        StandoffMoment::NoShot => &["let {him} go!", "drop {him} — NOW", "{held}, don't move", "I don't have a shot", "no shot — {held}'s square in front of him"],
        StandoffMoment::Line => &["I have him.", "got a line —"],
        StandoffMoment::Cornered => &["can't get round him — hold what you've got", "no way round. {held}, hang on", "nowhere for either of us to go — let {him} walk and this ends quiet"],
        StandoffMoment::Dropped => &["he's dropped {held} — TAKE HIM", "{held}'s down — light him up!"],
        StandoffMoment::Freed => &["{held}, DOWN — get clear!", "he's loose — {held}, out of the way!"],
        StandoffMoment::Lost => &["lost them — where'd he take {him}?", "I've lost sight — moving up"],
        StandoffMoment::HitHim => &["…christ— I hit {him}. I hit {held}—"],
    }
}

/// STANDOFF_NOW: a line as it is said now, by the line it stands for.
static STANDOFF_NOW: &[(&str, &str)] = &[
    ("{held}, stay loose — I'm coming round", "{held}, stay loose — I'm coming around"),
    ("{held}! —I can't— I haven't got a line—", "{held}! —I can't— I don't have a shot—"),
    ("I have him.", "I've got him."),
    ("I— I think I have him—", "I— I think I've got him—"),
    ("can't get round him — hold what you've got", "can't get around him — hold what you've got"),
    ("no way round. {held}, hang on", "no way around. {held}, hang on"),
];

/// STANDOFF_NAMELESS: the moments whose every line names him, for a man whose side has no name to say.
fn nameless_line(moment: StandoffMoment) -> Option<&'static str> {
    match moment {
        StandoffMoment::Dropped => Some("he's dropped {him} — TAKE HIM"),
        StandoffMoment::Freed => Some("he's loose — get clear!"),
        StandoffMoment::HitHim => Some("…christ— I hit {him}—"),
        _ => None,
    }
}

/// The pool a line is drawn from, in the order the die reads it: temperament's, then anybody's, then as said now.
fn pool_of(moment: StandoffMoment, t: crate::game::Temperament, nameless: bool) -> Vec<&'static str> {
    let mut pool: Vec<&'static str> = lines_of(moment, t).iter().chain(lines_any(moment).iter()).copied().collect();
    if nameless {
        pool.retain(|l| !l.contains("{held}"));
        if pool.is_empty() {
            if let Some(l) = nameless_line(moment) {
                pool.push(l);
            }
        }
    }
    pool.iter().map(|l| STANDOFF_NOW.iter().find(|(was, _)| was == l).map_or(*l, |(_, now)| *now)).collect()
}

/// One standoff line from this man if the clocks allow; never the line he said last. (guards.ts standoffBark)
pub fn standoff_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::guards::StandoffMoment, force: Option<crate::guards::StandoffBarkForce>, held: Option<GuardId>) {
    let force = force.unwrap_or(StandoffBarkForce::Bool(false));
    let forced = !matches!(force, StandoffBarkForce::Bool(false));
    let always = matches!(force, StandoffBarkForce::Always);
    let s = g.gd[gd].standoff;
    let held = held.or_else(|| s.map(|s| g.tab.guards_standoff[s].pair));
    let now = g.time;
    if !forced {
        let mine = s.map_or(false, |s| now - g.tab.guards_standoff[s].last_bark_t < STANDOFF_BARK_SECS);
        if mine || now - g.m.guards.floor_bark_t.unwrap_or(-100.0) < STANDOFF_BARK_FLOOR_SECS {
            return;
        }
    }
    if !always {
        if let Some(b) = &g.gd[gd].bubble {
            if b.radio && now - b.t < 3.0 {
                return;
            }
        }
    }
    let temper = crate::dialogue::temperament_of(g, gd);
    let nameless = match held {
        Some(h) => crate::teams::spoken_name(g, h).is_none(),
        None => false,
    };
    let pool = pool_of(moment, temper, nameless);
    if pool.is_empty() {
        return;
    }
    let n = pool.len() as f64;
    let mut i = js::rem(m::floor(crate::math_dice::sim_random(g) * n), n) as usize;
    if pool.len() > 1 && g.m.guards.last_bark.get(gd).map_or(false, |l| l.as_str() == pool[i]) {
        i = (i + 1) % pool.len();
    }
    let (mut name, mut him, mut he) = (String::from("him"), String::from("him"), String::from("he"));
    if let Some(h) = held {
        name = g.gd[h].callsign.clone();
        if let Some(cv) = g.gd[h].civ {
            him = g.cv[cv].her.clone();
            he = g.cv[cv].she.clone();
        }
    }
    let text = pool[i].replace("{held}", &name).replace("{him}", &him).replace("{he}", &he);
    g.m.guards.last_bark.set(gd, String::from(pool[i]));
    if let Some(s) = s {
        g.tab.guards_standoff[s].last_bark_t = now;
    }
    g.m.guards.floor_bark_t = Some(now);
    crate::game::game_say(g, gd, &text, None, None, None);
}

/// Side-step along the current nav path, the body not turned to it. True once the path is consumed. (guards.ts stepAlongPath)
pub fn step_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool {
    let c = g.gd[gd].ch;
    let paced = crate::condition::condition_pace(g, gd, speed);
    let speed = team_legs(g, gd, paced);
    if unstick_first(g, gd, dt) {
        return false;
    }
    while g.gd[gd].path_i < g.gd[gd].path.len() {
        let (i, n) = (g.gd[gd].path_i, g.gd[gd].path.len());
        let wp = g.gd[gd].path[i];
        let pos = g.ch[c].pos;
        let (dx, dz) = (wp[0] - pos[0], wp[2] - pos[2]);
        let d = m::hyp2(dx, dz);
        if d < (if i == n - 1 { 0.25 } else { 0.45 }) {
            g.gd[gd].path_i += 1;
            continue;
        }
        let legs = v3::damp(g.gd[gd].speed, speed * v3::clamp((d + (n - 1 - i) as f64) / 0.8, 0.35, 1.0), 4.0, dt);
        g.gd[gd].speed = legs;
        let vel: V3 = [dx / d * legs, 0.0, dz / d * legs];
        g.ch[c].vel = vel;
        g.ch[c].pos = v3::mad(pos, vel, dt);
        return false;
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// The next point on the ring for this man, an arc's step toward dead astern of them; None = planted or cornered. (guards.ts orbitGoalFor)
pub fn orbit_goal_for(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, p: crate::guards::Pair) -> Option<V3> {
    let pos = g.ch[g.gd[gd].ch].pos;
    let c_2 = p.centre;
    let dist = v3::dist_xz(pos, c_2);
    let brg = m::atan2(pos[0] - c_2[0], pos[2] - c_2[2]);
    let to_rear = v3::wrap_angle(p.facing + m::PI - brg);
    let short = if to_rear < 0.0 { -1.0 } else { 1.0 };
    let round = m::abs(to_rear) < STANDOFF_SETTLE_DEG * v3::DEG;
    if round && m::abs(dist - STANDOFF_ORBIT_M) < 0.6 {
        g.tab.guards_standoff[s].cornered = false;
        return None;
    }
    let going = g.tab.guards_standoff[s].way;
    if js::truthy(going) && m::abs(to_rear) < 120.0 * v3::DEG && short != going {
        g.tab.guards_standoff[s].way = short;
    }
    let going = g.tab.guards_standoff[s].way;
    let dir = if round {
        short
    } else {
        js::or(going, if m::abs(to_rear) < 150.0 * v3::DEG { short } else { g.tab.guards_standoff[s].side })
    };
    if dist > STANDOFF_MAX_M {
        if let Some(ap) = approach_point(g, gd, c_2, None) {
            g.tab.guards_standoff[s].cornered = false;
            return Some(ap);
        }
    }
    let mut others: Vec<GuardId> = Vec::new();
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        if let Some(os) = g.gd[o].standoff {
            if g.tab.guards_standoff[os].pair == p.held {
                others.push(o);
            }
        }
    }
    let mut plans = 0;
    let ways: Vec<f64> = if js::truthy(going) { vec![dir] } else { vec![dir, -dir] };
    let steps: &[f64] = if round { &[0.0] } else { &[50.0, 30.0, 70.0] };
    for &way in &ways {
        for &step_deg in steps {
            let span = if way == short { m::abs(to_rear) } else { 2.0 * m::PI - m::abs(to_rear) };
            let b = brg + way * m::min(step_deg * v3::DEG, span);
            for r in [STANDOFF_ORBIT_M, 4.0, 5.2, 3.9, 3.6, 5.8] {
                let q: V3 = [c_2[0] + m::sin(b) * r, 0.0, c_2[2] + m::cos(b) * r];
                if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || v3::dist_xz(q, pos) < 0.5 {
                    continue;
                }
                if no_sight_at(g, q[0], q[2], c_2) {
                    continue;
                }
                if no_sight_at(g, (q[0] + pos[0]) / 2.0, (q[2] + pos[2]) / 2.0, c_2) {
                    continue;
                }
                let taken = others.iter().any(|&o| {
                    let at = g.gd[o].standoff.and_then(|os| g.tab.guards_standoff[os].orbit_goal).unwrap_or(g.ch[g.gd[o].ch].pos);
                    v3::dist_xz(at, q) < 1.5
                });
                if taken {
                    continue;
                }
                plans += 1;
                if plans > 12 {
                    g.tab.guards_standoff[s].cornered = false;
                    return if dist > STANDOFF_MAX_M { approach_point(g, gd, c_2, Some(false)) } else { None };
                }
                let path = match crate::scene_collision::nav_grid_find_path(g, pos, q, None, None) {
                    Some(path) if !path.is_empty() => path,
                    _ => continue,
                };
                let (mut l, mut at, mut cuts) = (0.0, pos, false);
                for &w in &path {
                    l += v3::dist_xz(at, w);
                    at = w;
                    if v3::dist_xz(w, c_2) < STANDOFF_MIN_M - 0.7 {
                        cuts = true;
                    }
                }
                if cuts || l > 2.5 * v3::dist_xz(pos, q) + 2.5 {
                    continue;
                }
                g.tab.guards_standoff[s].cornered = false;
                if !round {
                    g.tab.guards_standoff[s].way = way;
                }
                return Some(q);
            }
        }
        if round {
            break;
        }
    }
    if dist > STANDOFF_MAX_M {
        g.tab.guards_standoff[s].cornered = false;
        return approach_point(g, gd, c_2, Some(false));
    }
    if dist > STANDOFF_ORBIT_M + 1.0 {
        if let Some(ap) = approach_point(g, gd, c_2, Some(false)) {
            if v3::dist_xz(ap, pos) > 0.6 {
                g.tab.guards_standoff[s].cornered = true;
                return Some(ap);
            }
        }
    }
    g.tab.guards_standoff[s].cornered = true;
    None
}

/// The point on this man's own nav path to `C` where it first comes within the ring, if they are in sight from there. (guards.ts approachPoint)
pub fn approach_point(g: &mut crate::game::Game, gd: GuardId, c_2: V3, need_sight: Option<bool>) -> Option<V3> {
    let need_sight = need_sight.unwrap_or(true);
    let pos = g.ch[g.gd[gd].ch].pos;
    let path = crate::scene_collision::nav_grid_find_path(g, pos, c_2, None, None)?;
    if path.is_empty() {
        return None;
    }
    let ring = STANDOFF_ORBIT_M;
    let mut a = pos;
    for &b in &path {
        if v3::dist_xz(b, c_2) <= ring {
            let (mut lo, mut hi) = (0.0, 1.0);
            for _ in 0..16 {
                let mid = (lo + hi) / 2.0;
                let q: V3 = [a[0] + (b[0] - a[0]) * mid, 0.0, a[2] + (b[2] - a[2]) * mid];
                if v3::dist_xz(q, c_2) > ring {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let q: V3 = [a[0] + (b[0] - a[0]) * hi, 0.0, a[2] + (b[2] - a[2]) * hi];
            let free = if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
                Some(q)
            } else {
                crate::scene_collision::nav_grid_nearest_free_point(g, q[0], q[2])
            };
            let free = match free {
                Some(f) if !(v3::dist_xz(f, c_2) < STANDOFF_MIN_M) => f,
                _ => return None,
            };
            return if need_sight && no_sight_at(g, free[0], free[2], c_2) { None } else { Some([free[0], 0.0, free[2]]) };
        }
        a = b;
    }
    None
}

/// retreatGoal's `open`: a free cell he can walk to straight over open floor.
fn open_floor(g: &mut Game, pos: V3, q: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
        && crate::scene_collision::nav_grid_walkable(g, pos, q)
        && !crate::scene_collision::solid_blocked(g, [pos[0], 0.5, pos[2]], [q[0], 0.5, q[2]], None)
}

/// Where a man too close to the pair backs off to; None = boxed in. (guards.ts retreatGoal)
pub fn retreat_goal(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> Option<V3> {
    let pos = g.ch[g.gd[gd].ch].pos;
    let c_2 = p.centre;
    let brg = m::atan2(pos[0] - c_2[0], pos[2] - c_2[2]);
    for da in [0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5] {
        for r in [STANDOFF_MIN_M + 0.7, STANDOFF_MIN_M + 0.2, STANDOFF_MIN_M + 1.4] {
            let a = brg + da;
            let q: V3 = [c_2[0] + m::sin(a) * r, 0.0, c_2[2] + m::cos(a) * r];
            if open_floor(g, pos, q) {
                return Some(q);
            }
        }
    }
    let d0 = v3::dist_xz(pos, c_2);
    let (mut best, mut bd) = (None, d0 + 0.8);
    for i in 0..16 {
        for st in [2.0, 1.2] {
            let a = i as f64 * m::PI / 8.0;
            let q: V3 = [pos[0] + m::sin(a) * st, 0.0, pos[2] + m::cos(a) * st];
            let d = v3::dist_xz(q, c_2);
            if d > bd && open_floor(g, pos, q) {
                bd = d;
                best = Some(q);
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Temperament;

    #[test]
    fn axis_measure_bits() {
        let cases: [(V3, V3, f64, f64, f64, f64, u64); 3] = AXIS_CASES;
        for (from, to, ax, az, y0, y1, bits) in cases {
            assert_eq!(axis_measure(from, to, ax, az, y0, y1).to_bits(), bits);
        }
    }

    #[test]
    fn pool_order_and_words() {
        let p = pool_of(StandoffMoment::NoShot, Temperament::Steady, false);
        assert_eq!(p.len(), 7);
        assert_eq!(p[0], "let {him} go and we talk about it");
        assert_eq!(p[1], "{held}, stay loose — I'm coming around");
        assert_eq!(p[2], "let {him} go!");
        let line = pool_of(StandoffMoment::Line, Temperament::Steady, false);
        assert_eq!(line, ["I've got him.", "got a line —"]);
    }

    #[test]
    fn nameless_pool_falls_to_its_own_line() {
        assert_eq!(pool_of(StandoffMoment::Dropped, Temperament::Steady, true), ["he's dropped {him} — TAKE HIM"]);
        assert_eq!(pool_of(StandoffMoment::Dropped, Temperament::Nervy, true), ["he's dropped {him} — TAKE HIM"]);
        let lost = pool_of(StandoffMoment::Lost, Temperament::Hard, true);
        assert_eq!(lost, ["lost them — where'd he take {him}?", "I've lost sight — moving up"]);
        assert!(pool_of(StandoffMoment::Enter, Temperament::Steady, true).contains(&"easy. easy. nobody has to get hurt here"));
    }

    /// This file's lines are the skeleton's table's (read from the same TypeScript), moment by moment.
    #[test]
    fn lines_are_the_skeleton_tables() {
        let table = standoff_lines();
        let moments = [StandoffMoment::Enter, StandoffMoment::NoShot, StandoffMoment::Line, StandoffMoment::Cornered, StandoffMoment::Dropped, StandoffMoment::Freed, StandoffMoment::Lost, StandoffMoment::HitHim];
        for moment in moments {
            let row = table.get(String::from(moment.as_str())).unwrap();
            let same = |ours: &[&str], theirs: &Option<Vec<String>>| {
                let theirs: Vec<&str> = theirs.iter().flatten().map(|l| l.as_str()).collect();
                assert_eq!(ours, theirs.as_slice(), "{}", moment.as_str());
            };
            same(lines_of(moment, Temperament::Steady), &row.steady);
            same(lines_of(moment, Temperament::Nervy), &row.nervy);
            same(lines_of(moment, Temperament::Hard), &row.hard);
            same(lines_any(moment), &row.under);
        }
        let nameless = standoff_nameless();
        assert_eq!(nameless.dropped.as_deref(), nameless_line(StandoffMoment::Dropped));
        assert_eq!(nameless.freed.as_deref(), nameless_line(StandoffMoment::Freed));
        assert_eq!(nameless.hit_him.as_deref(), nameless_line(StandoffMoment::HitHim));
    }

    /// from, to, the man's axis, soles, crown, and what the TypeScript's own arithmetic gives under EXACT (bun)
    const AXIS_CASES: [(V3, V3, f64, f64, f64, f64, u64); 3] = [
        ([1.1, 1.43, -2.7], [3.3, 1.42, 0.9], 2.13, -0.77, 0.05, 1.75, 0x3fc0528470dade40),
        ([-4.2, 0.3, 7.9], [-4.2, 2.6, 7.9], -3.61, 8.2, 0.55, 2.2, 0x3fe6a417e13e32c0),
        ([0.37, 2.9, 0.11], [9.41, 2.1, 5.73], 12.7, 7.03, 0.02, 1.71, 0x400c78c04b5bb24b),
    ];
}
