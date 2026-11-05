//! src/game/guards.ts lines 2903 to 3293
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::character::Bone;
use crate::game::{GuardBodyDuty, GuardBodyDutyRole, GuardBodyDutyStage, GuardBubble, GuardState, GuardTask, GuardTaskCheckOn};
use crate::teams::Team;

// ---- seams: what skeleton 3 still cannot do

/// guards.ts 2914 and on: an RtLight's fields have no accessor (LightId has neither table nor properties).
fn seam_light_set_pos(g: &mut Game, l: LightId, p: V3) { todo!("needs: RtLight.pos accessor for a LightId") }
fn seam_light_set_dir(g: &mut Game, l: LightId, d: V3) { todo!("needs: RtLight.dir accessor for a LightId") }
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled accessor for a LightId") }
fn seam_light_set_enabled(g: &mut Game, l: LightId, on: bool) { todo!("needs: RtLight.enabled accessor for a LightId") }
fn seam_light_peak(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.peakIntensity accessor for a LightId") }
fn seam_light_set_peak(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.peakIntensity accessor for a LightId") }
fn seam_light_set_intensity(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.intensity accessor for a LightId") }
fn seam_light_set_inner(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.innerDeg accessor for a LightId") }
fn seam_light_set_outer(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.outerDeg accessor for a LightId") }
fn seam_light_set_radius(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.radius accessor for a LightId") }
/// guards.ts 2979 `manLeaves(gd.char, gd)`: ManCols came out empty, and a Character / a Guard do not turn into one.
fn seam_man_leaves(g: &mut Game, gd: GuardId) { todo!("needs: man_leaves by his CharId and GuardId") }
/// guards.ts 3118, 3138 `g.audio.play(name, at, vol, { rate })`: game_audio_like_play's opts hold `crate::Opaque`, which cannot carry a number.
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry the number `rate`") }

// ---- small readers

/// `g.audio.play(name, at, vol)`
fn play(g: &mut Game, name: &str, at: V3, vol: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(at), Some(vol), None);
}
/// `gd.weapon.def.rig === 'pistol'`
fn weapon_rig_is_pistol(g: &Game, gd: GuardId) -> bool {
    let def = g.fa[g.gd[gd].weapon].def;
    matches!(g.m.weapons.weapons.get(def), Some(d) if d.rig == crate::weapons::WeaponDefRig::Pistol)
}
/// `ordersOf(g, gd)`'s `sams && mine(gd)`: orders_of returns nothing in the skeleton, so its choice of book is made here: this game's, or his seat's.
fn orders_sams_mine(g: &mut Game, gd: GuardId) -> bool {
    if crate::seats::one_book(g) { return crate::orders::squad_orders_sams(g) && crate::orders::squad_orders_mine(g, gd); }
    let s = crate::seats::seat_of(g, gd);
    let man = crate::cmd::seat_man(g, s);
    crate::seats::with_seat(g, man, &mut |g: &mut Game| crate::orders::squad_orders_sams(g) && crate::orders::squad_orders_mine(g, gd))
}
/// `eachSeat(g, v => …)`: each_seat's closure takes two `&mut Game` and cannot be lent, so its body is here, through with_seat.
fn on_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    if g.seats.len() == 1 { f(g); return; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let man = crate::cmd::seat_man(g, s);
        crate::seats::with_seat(g, man, &mut |g: &mut Game| f(g));
    }
}

/// his head bone, or his feet (`x.char.bones.head ?? x.char.pos`: where a radio crackle sounds)
fn head_or_pos(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => g.ch[c].pos }
}
/// `g.level.routes[gd.routeI]`'s points, if that route exists
fn route_points(g: &Game, gd: GuardId) -> Option<Vec<V3>> {
    g.level.routes.get(g.gd[gd].route_i).map(|r| g.tab.route[*r].points.clone())
}
/// `g.level.routes[M.routeI]?.points[0] ?? M.char.pos`
fn post_of(g: &Game, m: GuardId) -> V3 {
    match route_points(g, m).and_then(|p| p.first().copied()) { Some(p) => p, None => g.ch[g.gd[m].ch].pos }
}
/// a number as the TypeScript's template prints it (no minus on a zero)
fn num(x: f64) -> String { format!("{}", x + 0.0) }

// ---- the flashlight

/// The light at its lens, its beam and whether it burns; returns what the lens hangs off.   (guards.ts poseFlashlight)
pub fn pose_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> {
    let c = g.gd[gd].ch;
    let fl = g.gd[gd].flashlight;
    if g.ch[c].bones.get(Bone::HandR).is_none() { return None; }
    let mut dir = g.ch[c].gun_dir;
    let rk = rig_of(g, gd);
    let carbine = rk == RigOf::Carbine;
    let torch = hand_carry(g, gd) == HandCarry::Torch;
    let hangs = torch && g.ch[c].alive && torch_low(g, gd);
    if torch && g.ch[c].alive {
        let low = if hangs { Some(crate::rig_props::TorchCarryDirLow { tan: g.k.torch.low_tan, rate: g.k.torch.low_rate }) } else { None };
        let prev = g.gd[gd].beam_dir;
        let beam = crate::rig_props::torch_carry_dir(g, c, prev, Some(dt), low);
        g.gd[gd].beam_dir = beam;
        dir = beam;
    } else {
        g.gd[gd].beam_dir = v3::copy(dir);
    }
    let lens = if rk == RigOf::Sc20k {
        crate::rig_props::sc20k_light_pose(g, c, None).pos
    } else if crate::kit::kit_pistol(g, gd) && weapon_rig_is_pistol(g, gd) {
        crate::rig_props::rig_light_pose(g, c, None).pos
    } else {
        let want = if torch { crate::handwork::SwapHeldHeld::Torch } else { crate::handwork::SwapHeldHeld::Weapon };
        let in_torch_hand = crate::handwork::swap_held(g, gd, Some(want)) == Some(crate::handwork::SwapHeld::Torch);
        let weapon = if carbine { crate::rig_props::RigTorchPosWeapon::Carbine } else { crate::rig_props::RigTorchPosWeapon::Pistol };
        crate::rig_props::rig_torch_pos(g, c, dir, in_torch_hand, Some(crate::rig_props::rig()), Some(weapon))
    };
    seam_light_set_pos(g, fl, lens);
    seam_light_set_dir(g, fl, dir);
    if crate::runner::runner_fled(g, gd) {
        seam_light_set_enabled(g, fl, false);
        return g.ch[c].bones.get(Bone::UpperArmR).copied();
    }
    let mut held: Option<V3> = g.ch[c].bones.get(Bone::UpperArmR).copied();
    let lost_live = if torch && g.gd[gd].torch_lost_at >= 0.0 { g.gd[gd].torch_lost_at } else { g.gd[gd].gun_lost_at };
    if g.gd[gd].state == GuardState::Dead || lost_live >= 0.0 {
        if let Some(it) = g.gd[gd].dropped {
            let (yaw, tumble) = (crate::throwables::item_yaw(g, it), crate::throwables::item_tumble(g, it));
            let hd: V3 = [m::sin(yaw) * m::cos(tumble), -m::sin(tumble), m::cos(yaw) * m::cos(tumble)];
            let at = crate::throwables::item_pos(g, it);
            seam_light_set_pos(g, fl, v3::mad(at, hd, 0.1));
            seam_light_set_dir(g, fl, hd);
            held = Some(at);
        }
    }
    if g.gd[gd].state != GuardState::Dead && lost_live >= 0.0 {
        if !g.gd[gd].light_lit_at_loss { seam_light_set_enabled(g, fl, false); return held; }
        sputter(g, fl, g.time - lost_live);
        return held;
    }
    if g.gd[gd].state == GuardState::Dead && g.time >= g.gd[gd].light_dead_until {
        if !g.gd[gd].light_lit_at_loss { seam_light_set_enabled(g, fl, false); return held; }
        let (tl, gl) = (g.gd[gd].torch_lost_at, g.gd[gd].gun_lost_at);
        let lost_at = m::min(m::min(if tl >= 0.0 { tl } else { f64::INFINITY }, if gl >= 0.0 { gl } else { f64::INFINITY }), g.gd[gd].died_at);
        sputter(g, fl, g.time - lost_at);
        return held;
    }
    let until = g.gd[gd].light_dead_until;
    if g.time < until {
        seam_light_set_enabled(g, fl, false);
    } else if until > 0.0 && g.time < until + 1.0 {
        let on = m::sin(g.time * 50.0) > 0.2;
        seam_light_set_enabled(g, fl, on);
    } else {
        let unit_light = g.gd[gd].unit.as_ref().map(|u| u.light);
        let on = (match unit_light {
            None => true,
            Some(lit) => (lit || crate::darkfoe::dark_foe_lit(g, gd) || crate::powerout::power_out_lit(g, gd))
                && (!light_ruled(g, gd) || team_lit(g, gd))
                && !crate::nvg::nvg_dark(g, gd)
                && !crate::silent::silent_dark(g, gd),
        }) && !(torch && crate::calmtorch::calm_torch_dark(g, gd, dt));
        seam_light_set_enabled(g, fl, on);
    }
    if wall_dark(g, gd, held) && seam_light_enabled(g, fl) {
        seam_light_set_enabled(g, fl, false);
        if let Some(s) = g.m.guards.wall_seen.get_mut(gd) { s.out = Some(true); }
    }
    if seam_light_enabled(g, fl) && crate::doze::doze_dark(g, gd) { seam_light_set_enabled(g, fl, false); }
    let (peak, inner, outer) = (g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
    seam_light_set_peak(g, fl, peak);
    seam_light_set_intensity(g, fl, peak);
    seam_light_set_inner(g, fl, inner);
    seam_light_set_outer(g, fl, outer);
    let radius = g.engine.settings.flashlight_radius;
    seam_light_set_radius(g, fl, radius);
    held
}
/// A light on the floor `t` seconds after it fell: a couple of sputters, then dark.   (guards.ts 2921, 2929)
fn sputter(g: &mut Game, fl: LightId, t: f64) {
    let on = t < 0.9 && (t < 0.35 || m::sin(t * 60.0) > 0.3);
    seam_light_set_enabled(g, fl, on);
    let peak = seam_light_peak(g, fl);
    seam_light_set_intensity(g, fl, peak * v3::clamp(1.0 - t / 0.9, 0.0, 1.0));
}

// ---- on a screen, and a man's line

/// On a player's screen: with two, on either's.   (guards.ts onScreen)
pub fn on_screen(g: &mut crate::game::Game, p: V3) -> bool {
    if g.seats.len() > 1 {
        let seats = g.seats.clone();
        for s in seats {
            let man = crate::cmd::seat_man(g, s);
            if crate::seats::with_seat(g, man, &mut |g: &mut Game| in_game_view(g, p)) { return true; }
        }
        return false;
    }
    in_game_view(g, p)
}
/// inView of the view on Game: of the seat this pass is about.
fn in_game_view(g: &Game, p: V3) -> bool {
    match &g.view_proj {
        None => false,
        Some(crate::game::GameViewProj::Array(v)) => in_view_of(|i| v[i], p),
        Some(crate::game::GameViewProj::Float32Array(v)) => in_view_of(|i| v[i] as f64, p),
    }
}
/// Is a point inside the frame of this view-projection?   (guards.ts inView)
pub fn in_view(vp: Option<&crate::guards::InViewVp>, p: V3) -> bool {
    match vp {
        None => false,
        Some(InViewVp::Array(v)) => in_view_of(|i| v[i], p),
        Some(InViewVp::Float32Array(v)) => in_view_of(|i| v[i] as f64, p),
    }
}
fn in_view_of(vp: impl Fn(usize) -> f64, p: V3) -> bool {
    let w = vp(3) * p[0] + vp(7) * p[1] + vp(11) * p[2] + vp(15);
    if w <= 0.0 { return false; }
    let x = (vp(0) * p[0] + vp(4) * p[1] + vp(8) * p[2] + vp(12)) / w;
    let y = (vp(1) * p[0] + vp(5) * p[1] + vp(9) * p[2] + vp(13)) / w;
    x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9
}

/// How long a line stands over a man's head, by its length (the language's: UTF-16 units).   (guards.ts bubbleSecs)
pub fn bubble_secs(text: &str) -> f64 {
    m::min(4.0, m::max(2.0, 1.5 + text.encode_utf16().count() as f64 * 0.055))
}
/// His head, or where it would be.   (guards.ts headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.7, 0.0]) }
}

/// A man says something: a bubble over his head, and a log row if it went over the radio or is out of sight.   (guards.ts say)
pub fn say(g: &mut crate::game::Game, gd: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) {
    let radio = radio.unwrap_or(false);
    let force = force.unwrap_or(false);
    let row = row.unwrap_or(true);
    if g.gd[gd].held.is_some() && !force { return; }
    let net = radio || crate::silent::silent_hush(g, gd);
    let dur = bubble_secs(text);
    g.gd[gd].bubble = Some(GuardBubble { text: String::from(text), t: g.time, dur, radio: net });
    if net { crate::handwork::radio_hand(g, gd, dur); }
    if !net && row { crate::cues::cues_overheard(g, gd, text); }
    if radio && g.gd[gd].unit.is_some() { crate::cues::cues_heard(g, gd, text); }
    if radio { crate::chronicle::chron_radio(g, gd, text); }
    if row && (radio || crate::fogofwar::fog_of_war_hidden(g, gd) || { let h = head_of(g, gd); !on_screen(g, h) }) {
        let unit = g.gd[gd].unit.is_some();
        let cs = &g.gd[gd].callsign;
        let who = if net {
            if unit { format!("{} (radio): ", cs) } else { String::from("radio: ") }
        } else if force || unit {
            format!("{}: ", cs)
        } else {
            String::from("guard: ")
        };
        let line = format!("{}{}", who, text);
        crate::game::game_msg(g, &line);
    }
    if orders_sams_mine(g, gd) && { let h = head_of(g, gd); !on_screen(g, h) } {
        crate::squadnet::squad_net_line(g);
    }
}

/// Fresh guards at the start of their routes; spawned units are not stood up again.   (guards.ts resetGuards)
pub fn reset_guards(g: &mut crate::game::Game) {
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &of) {
        g.pl[pl].dragging = None;
        g.pl[pl].takedown = None;
        g.pl[pl].holding = None;
        let c = g.pl[pl].ch;
        g.ch[c].anim.hold_pose = None;
    }
    let old = g.guards.clone();
    for &gd in &old {
        if g.gd[gd].unit.is_some() { release_unit(g, gd); }
    }
    end_engagement(g, EndEngagementOutcome::Reset);
    let old = g.guards.clone();
    let mut fresh: Vec<(f64, RouteId, f64, LightId, V3)> = Vec::new();
    for &gd in &old {
        if g.gd[gd].unit.is_some() { continue; }
        let c = g.gd[gd].ch;
        fresh.push((g.ch[c].id, g.gd[gd].route, g.gd[gd].route_i as f64, g.gd[gd].flashlight, g.ch[c].tint));
    }
    for &gd in &old { seam_man_leaves(g, gd); }
    let mut shift: Vec<GuardId> = Vec::with_capacity(fresh.len());
    let (men, rig) = (g.men, g.rig);
    for (id, route, route_i, fl, tint) in fresh { shift.push(crate::game::guard_new(g, men, rig, id, route, route_i, fl, tint, None)); }
    g.guards = shift.clone();
    for gd in shift {
        let c = g.gd[gd].ch;
        crate::character::character_update(g, c, 0.0);
    }
    crate::opening::opening_start(g);
    crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().fix, None, Some(0.0), None, None, None);
    on_each_seat(g, &mut |v: &mut Game| { v.aim_guard = None; });
    g.escalation = 0.0;
    g.escalation_t = -1.0;
    g.alarm.episode = false;
    g.alarm.placed = false;
    g.alarm.apart_t = 0.0;
    g.clearing = None;
    g.cleared_at.clear();
    let r = rollcall_state(g);
    r.stage = RollcallStage::Idle;
    r.next_at = -1.0;
    r.caller = None;
    r.missing = None;
    crate::game::game_note(g, "guards reset");
}

// ---- alarm escalation

/// The alarm level the guards act on: the game's, except under the showcase tour.   (guards.ts escalationOf)
pub fn escalation_of(g: &mut crate::game::Game) -> f64 {
    if g.quiet_utility { 0.0 } else { g.escalation }
}
/// (guards.ts escalationName)
pub fn escalation_name(l: f64) -> String {
    String::from(if l >= 2.0 { "lockdown" } else if l >= 1.0 { "heightened" } else { "calm" })
}

/// `g.level.routes[gd.routeI]?.name ?? guard n`: a man's name on the panel
fn route_name(g: &Game, gd: GuardId) -> String {
    match g.level.routes.get(g.gd[gd].route_i) {
        Some(r) => g.tab.route[*r].name.clone(),
        None => format!("guard {}", num(g.ch[g.gd[gd].ch].id - 10.0)),
    }
}
/// One line for the Tab panel: level, its clock, the episode, the lockdown duties, the radio check.   (guards.ts escalationSummary)
pub fn escalation_summary(g: &mut crate::game::Game) -> String {
    let lvl = g.escalation;
    let mut s = format!("alarm level: {}{}", escalation_name(lvl), if g.quiet_utility && js::truthy(lvl) { " (tour: ignored)" } else { "" });
    if lvl > 0.0 { s += &format!(" · {} s", num(m::max(0.0, m::ceil(g.escalation_t - g.time)))); }
    if g.alarm.episode { s += " · episode running"; }
    for &gd in &g.guards {
        if g.gd[gd].state == GuardState::Dead { continue; }
        if let Some(l) = g.gd[gd].leader { s += &format!(" · pair {} ▸ {}", route_name(g, l), route_name(g, gd)); }
        if let Some(p) = &g.gd[gd].post { s += &format!(" · {} holds the {}", route_name(g, gd), p.name); }
    }
    let (stage, missing, next_at) = { let r = rollcall_state(g); (r.stage, r.missing, r.next_at) };
    if stage != RollcallStage::Idle {
        let what = if stage == RollcallStage::Called {
            String::from("counting in")
        } else {
            format!("asking after {}", match missing { Some(x) => g.gd[x].callsign.as_str(), None => "?" })
        };
        s += &format!(" · radio check: {}", what);
    } else if next_at >= 0.0 && !g.quiet_utility {
        s += &format!(" · radio check in {} s", num(m::max(0.0, m::ceil(next_at - g.time))));
    }
    for &gd in &g.guards {
        if g.gd[gd].state == GuardState::Dead { continue; }
        if let Some(GuardTask::CheckOn(k)) = &g.gd[gd].task {
            s += &format!(" · {} walks {}'s route ({} to go)", route_name(g, gd), g.gd[k.who].callsign, num(k.left));
        }
    }
    s
}
/// The second panel line: what the pair is clearing right now, or nothing.   (guards.ts clearingSummary)
pub fn clearing_summary(g: &mut crate::game::Game) -> String {
    if g.clearing.is_some() { crate::squad::clearing_summary(g) } else { String::new() }
}

/// The living man best placed to radio something about `pos`: the nearest not mid-fight, else the nearest at all, never `except`.   (guards.ts spokesman)
pub fn spokesman(g: &mut crate::game::Game, pos: Option<V3>, except: Option<GuardId>) -> Option<GuardId> {
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    let guards = g.guards.clone();
    let mut pass = 0;
    while pass < 2 && best.is_none() {
        for &gd in &guards {
            if off_net(g, gd)
                || Some(gd) == except
                || (pass == 0 && (g.gd[gd].state == GuardState::Alert || crate::doze::doze_asleep(g, gd)))
                || g.gd[gd].team != Team::Guard
                || g.gd[gd].unit.is_some()
            { continue; }
            let d = match pos { Some(p) => v3::dist(g.ch[g.gd[gd].ch].pos, p), None => 0.0 };
            if d < bd { bd = d; best = Some(gd); }
        }
        pass += 1;
    }
    best
}

/// Raise the floor to at least `to` and (re)arm that level's clock: one radio line per actual change.   (guards.ts raiseEscalation)
pub fn raise_escalation(g: &mut crate::game::Game, to: f64, except: Option<GuardId>, cause: Option<crate::guards::RaiseEscalationCause>, another: Option<bool>) {
    use crate::guards::RaiseEscalationCause as C;
    let cause = cause.unwrap_or(C::Search);
    let another = another.unwrap_or(false);
    let prev = g.escalation;
    let lvl = if to > prev { to } else { prev };
    g.escalation = lvl;
    g.escalation_t = g.time + if lvl >= 2.0 { LOCKDOWN_SECS } else { HEIGHTENED_SECS };
    if lvl > prev { crate::chronicle::chron_alarm(g, lvl, true, Some(cause.as_str())); }
    let again = g.time - g.alarm.raised_at < 1.5;
    g.alarm.raised_at = g.time;
    if again && lvl <= prev && cause != C::Body { return; }
    let at = if g.alarm.placed { Some(g.alarm.pos) } else { None };
    let sp = spokesman(g, at, except);
    let line: String = if lvl <= prev {
        match cause {
            C::Body => format!("{} — stay locked down, clear everything around him", if another { "another man down" } else { "man down confirmed" }),
            C::Missing => String::from("and a man we can't raise — it stays locked down, find him"),
            C::Hostage => String::from("copy — he's got one of ours. stay locked down. box him in. no shot unless it's clean"),
            C::Device => String::from("copy, control — it stays locked down, get to it"),
            C::Civilian => String::from("copy — another one saw him. stay on it, get over there"),
            C::Search => String::from("still nothing — it stays locked down"),
        }
    } else if lvl >= 2.0 {
        if cause == C::Hostage {
            String::from("he's got one of ours — lock it down. weapons out, everyone box him in and hold your fire")
        } else {
            let lead = match cause {
                C::Body => if another { "another man down — " } else { "man down — " },
                C::Missing => "a man missing on top of it — ",
                C::Device => "copy, control — ",
                C::Civilian => "copy — that's a witness. ",
                _ => "",
            };
            format!("{}lock it down: pairs, weapons out, nobody wanders", lead)
        }
    } else {
        match cause {
            C::Body => String::from("we have a man down — weapons out, everyone sharp"),
            C::Missing => String::from("we've got a man off the net and off his route — weapons out, treat it as real"),
            C::Device => String::from("copy, control — weapons out, everyone sharp, somebody get eyes on it"),
            C::Civilian => String::from(if !crate::dialogue::is_office(g) { "copy — somebody's here who shouldn't be. weapons out, treat it as real" } else { "copy — somebody's on the floor who shouldn't be. weapons out, treat it as real" }),
            _ => String::from("nothing here — stay sharp, check your corners"),
        }
    };
    if let Some(sp) = sp {
        crate::game::game_say(g, sp, &line, Some(true), None, None);
        let at = head_or_pos(g, sp);
        play(g, "radio", at, 0.7);
    }
    if lvl >= 2.0 {
        assign_lockdown(g, prev >= 2.0, except);
        if cause != C::Hostage {
            let why = match cause {
                C::Body => PlanClearsCause::Body,
                C::Missing => PlanClearsCause::Missing,
                _ => PlanClearsCause::Search,
            };
            plan_clears(g, Some(false), Some(why));
        }
    }
}

/// Panel: the floor calm again at once, without touching the men themselves.   (guards.ts standDownEscalation)
pub fn stand_down_escalation(g: &mut crate::game::Game) {
    g.escalation = 0.0;
    g.escalation_t = -1.0;
    g.alarm.episode = false;
    g.alarm.placed = false;
    if g.clearing.is_some() { crate::squad::clearing_cancel(g); }
    g.clearing = None;
    g.cleared_at.clear();
    clear_lockdown(g);
}

// ---- the net's radio check

/// A man who cannot answer the net: down, or held with Sam's arm across his throat.   (guards.ts offNet)
pub fn off_net(g: &mut crate::game::Game, x: GuardId) -> bool {
    g.gd[x].state == GuardState::Dead || g.gd[x].held.is_some()
}
/// The net's radio-check state for this game, made on first touch; the row itself.   (guards.ts rollcallState)
pub fn rollcall_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::guards::Rollcall {
    g.m.guards.rollcalls.get_or_insert_with(|| Rollcall { next_at: -1.0, stage: RollcallStage::Idle, t0: 0.0, caller: None, missing: None, answered: false, checks: 0.0 })
}
/// Seconds to the next check, jittered: a die.   (guards.ts rollcallInterval)
pub fn rollcall_interval(g: &mut crate::game::Game) -> f64 {
    let secs = if escalation_of(g) >= 1.0 { ROLLCALL_ALARMED_SECS } else { ROLLCALL_CALM_SECS };
    secs * (1.0 + (crate::math_dice::sim_random(g) * 2.0 - 1.0) * ROLLCALL_JITTER)
}
/// Who calls the check: a living man with his hands free for the radio, one on his route for choice.   (guards.ts radioCaller)
pub fn radio_caller(g: &mut crate::game::Game) -> Option<GuardId> {
    let mut best: Option<GuardId> = None;
    let mut br = -1;
    let guards = g.guards.clone();
    for x in guards {
        if off_net(g, x)
            || g.gd[x].state == GuardState::Alert
            || g.gd[x].script.is_some()
            || matches!(&g.gd[x].body_duty, Some(b) if b.stage != GuardBodyDutyStage::Fan)
            || g.gd[x].team != Team::Guard
            || crate::doze::doze_asleep(g, x)
        { continue; }
        let r = if g.gd[x].state == GuardState::Patrol { 2 } else { 1 };
        if r > br { br = r; best = Some(x); }
    }
    best
}
/// Somebody already out walking this man's route.   (guards.ts checkerFor)
pub fn checker_for(g: &mut crate::game::Game, m: GuardId) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.gd[x].state != GuardState::Dead && matches!(&g.gd[x].task, Some(GuardTask::CheckOn(k)) if k.who == m))
}
/// A silent man is being looked for right now: the floor's alarm level holds meanwhile.   (guards.ts stillLooking)
pub fn still_looking(g: &mut crate::game::Game) -> bool {
    rollcall_state(g).stage == RollcallStage::Asked
        || g.guards.iter().any(|&x| g.gd[x].state != GuardState::Dead && matches!(&g.gd[x].task, Some(GuardTask::CheckOn(_))))
}
/// calm, free of orders, errands and bodies, and of Sam's arm   (guards.ts pickChecker's `free`)
fn checker_free(g: &Game, x: GuardId, m: GuardId) -> bool {
    let d = &g.gd[x];
    x != m && d.state == GuardState::Patrol && !d.hold && d.script.is_none() && d.task.is_none() && d.body_duty.is_none() && d.held.is_none() && d.team == Team::Guard
}
/// The living man best placed to go round M's route, nearest any point of it; `prefer` first.   (guards.ts pickChecker)
pub fn pick_checker(g: &mut crate::game::Game, m: GuardId, prefer: Option<GuardId>) -> Option<GuardId> {
    let pts = match route_points(g, m) { Some(p) => p, None => vec![g.ch[g.gd[m].ch].pos] };
    if let Some(p) = prefer { if checker_free(g, p, m) { return Some(p); } }
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    for &x in &g.guards {
        if !checker_free(g, x, m) { continue; }
        let mut d = f64::INFINITY;
        let at = g.ch[g.gd[x].ch].pos;
        for &p in &pts { d = m::min(d, v3::dist_xz(p, at)); }
        if d < bd { bd = d; best = Some(x); }
    }
    best
}
/// Send `gd` round M's route: the checkOn errand from the waypoint of M's nearest him.   (guards.ts sendCheckOn)
pub fn send_check_on(g: &mut crate::game::Game, gd: GuardId, m: GuardId, line: &str) {
    let pts = route_points(g, m).unwrap_or_default();
    let mut i0 = 0usize;
    let mut bd = f64::INFINITY;
    let me = g.ch[g.gd[gd].ch].pos;
    for (i, &p) in pts.iter().enumerate() {
        let d = v3::dist_xz(p, me);
        if d < bd { bd = d; i0 = i; }
    }
    let pos: V3 = if !pts.is_empty() { v3::copy(pts[i0]) } else { v3::copy(post_of(g, m)) };
    let t0 = g.time;
    let d = &mut g.gd[gd];
    d.task = Some(GuardTask::CheckOn(GuardTaskCheckOn { pos, who: m, wp: i0 as f64, left: m::max(1.0, pts.len() as f64), t0 }));
    d.last_known = Some(v3::copy(pos));
    d.awareness = m::max(d.awareness, 0.42);
    if d.state == GuardState::Patrol { d.state = GuardState::Suspicious; d.react_t = 0.8; }
    d.search_t = 0.0;
    d.path = Vec::new();
    d.path_goal = None;
    d.body_duty = None;
    d.stall_ref = None;
    crate::game::game_say(g, gd, line, Some(true), None, None);
    let at = head_or_pos(g, gd);
    seam_audio_play_rate(g, "radio", at, 0.5, 1.05);
}
/// One frame of the errand's bookkeeping: over if the man has been found, capped, or on to the next waypoint.   (guards.ts checkOnTick)
pub fn check_on_tick(g: &mut crate::game::Game, gd: GuardId) {
    let k = match &g.gd[gd].task { Some(GuardTask::CheckOn(k)) => *k, _ => return };
    let who = k.who;
    if g.gd[who].found || !off_net(g, who) || !g.guards.contains(&who) {
        g.gd[gd].task = None;
        if g.gd[who].found && g.gd[who].state == GuardState::Dead && g.guards.contains(&who) && g.gd[gd].body_duty.is_none() && !g.quiet_utility {
            let t0 = g.time;
            g.gd[gd].body_duty = Some(GuardBodyDuty { body: who, role: GuardBodyDutyRole::Called, stage: GuardBodyDutyStage::To, t: 0.0, t0, spot: None });
            let body = body_pos(g, who);
            let d = &mut g.gd[gd];
            d.last_known = Some(body);
            d.awareness = m::max(d.awareness, 0.45);
            d.react_t = 0.5;
            d.search_t = 0.0;
            d.path = Vec::new();
            d.path_goal = None;
        }
        return;
    }
    if g.time - k.t0 > CHECK_CAP_SECS { check_on_done(g, gd, who, true); return; }
    let near = match g.gd[gd].last_known { Some(l) => v3::dist_xz(l, k.pos) < 0.6, None => false };
    if g.gd[gd].react_t <= 0.0 && near && g.gd[gd].search_t > CHECK_LOOK_SECS { advance_check_on(g, gd); }
}
/// The next of the missing man's waypoints, or, the last one looked at, he is not on his route.   (guards.ts advanceCheckOn)
pub fn advance_check_on(g: &mut crate::game::Game, gd: GuardId) {
    let who = match &g.gd[gd].task { Some(GuardTask::CheckOn(k)) => k.who, _ => return };
    let pts = route_points(g, who).unwrap_or_default();
    let left = match &mut g.gd[gd].task {
        Some(GuardTask::CheckOn(k)) => { k.left -= 1.0; k.left }
        _ => return,
    };
    if left <= 0.0 || pts.is_empty() { check_on_done(g, gd, who, false); return; }
    let d = &mut g.gd[gd];
    let mut pos = [0.0; 3];
    if let Some(GuardTask::CheckOn(k)) = &mut d.task {
        k.wp = js::rem(k.wp + 1.0, pts.len() as f64);
        pos = v3::copy(pts[k.wp as usize]);
        k.pos = pos;
    }
    d.last_known = Some(v3::copy(pos));
    d.search_t = 0.0;
    d.path = Vec::new();
    d.path_goal = None;
    d.stall_ref = None;
    if left == m::floor(pts.len() as f64 / 2.0) {
        let line = format!("halfway around — nothing yet. {}, if you can hear this, sound off", g.gd[who].callsign);
        crate::game::game_say(g, gd, &line, Some(true), None, None);
        let at = head_or_pos(g, gd);
        seam_audio_play_rate(g, "radio", at, 0.4, 1.05);
    }
}
/// The walk is over without him: the line, and, once per missing man, the floor goes up a step on it.   (guards.ts checkOnDone)
pub fn check_on_done(g: &mut crate::game::Game, gd: GuardId, m: GuardId, capped: bool) {
    let d = &mut g.gd[gd];
    d.task = None;
    d.search_t = 0.0;
    d.awareness = v3::clamp(d.awareness, 0.2, 0.3);
    let line = if capped {
        format!("walked most of it — no sign of {}. he's not at his post", g.gd[m].callsign)
    } else {
        format!("walked his whole route — {} is not at his post, no sign of him anywhere", g.gd[m].callsign)
    };
    crate::game::game_say(g, gd, &line, Some(true), None, None);
    let at = head_or_pos(g, gd);
    play(g, "radio", at, 0.7);
    if !g.gd[m].missing_raised {
        g.gd[m].missing_raised = true;
        let post = post_of(g, m);
        g.alarm.pos[0] = post[0];
        g.alarm.pos[1] = 0.0;
        g.alarm.pos[2] = post[2];
        g.alarm.placed = true;
        let to = if escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
        raise_escalation(g, to, Some(gd), Some(RaiseEscalationCause::Missing), None);
    }
}
/// (guards.ts updateRollcall's `callerOk`)
fn caller_ok(g: &mut Game, x: Option<GuardId>) -> bool {
    match x {
        None => false,
        Some(x) => !off_net(g, x) && g.gd[x].state != GuardState::Alert && g.gd[x].script.is_none() && g.guards.contains(&x),
    }
}
/// (guards.ts updateRollcall's `silent`)
fn silent(g: &mut Game, x: GuardId) -> bool {
    off_net(g, x) && !g.gd[x].found && checker_for(g, x).is_none()
}
/// The check is over for now: idle, the next one on its clock.
fn rollcall_idle(g: &mut Game) {
    let next = g.time + rollcall_interval(g);
    let r = rollcall_state(g);
    r.stage = RollcallStage::Idle;
    r.next_at = next;
}
/// Once a frame: the radio check's clock and its short exchange.   (guards.ts updateRollcall)
pub fn update_rollcall(g: &mut crate::game::Game, dt: f64) {
    if g.quiet_utility || !g.ai_enabled {
        let r = rollcall_state(g);
        r.stage = RollcallStage::Idle;
        r.next_at = -1.0;
        return;
    }
    if g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()).and_then(|q| q.radio_check) == Some(false) {
        let r = rollcall_state(g);
        r.stage = RollcallStage::Idle;
        r.next_at = -1.0;
        return;
    }
    if rollcall_state(g).next_at < 0.0 {
        let next = g.time + rollcall_interval(g);
        rollcall_state(g).next_at = next;
    }
    let staff: Vec<GuardId> = g.guards.iter().copied().filter(|&x| g.gd[x].team == Team::Guard).collect();
    let living: Vec<GuardId> = staff.iter().copied().filter(|&x| g.gd[x].state != GuardState::Dead).collect();
    match rollcall_state(g).stage {
        RollcallStage::Idle => {
            if escalation_of(g) >= 1.0 {
                let soon = g.time + ROLLCALL_ALARMED_SECS * (1.0 + ROLLCALL_JITTER);
                let r = rollcall_state(g);
                r.next_at = m::min(r.next_at, soon);
            }
            if g.time < rollcall_state(g).next_at { return; }
            if living.is_empty() || g.pl[g.player].down {
                let next = g.time + rollcall_interval(g);
                rollcall_state(g).next_at = next;
                return;
            }
            if g.alarm.episode
                || living.iter().any(|&x| g.gd[x].state == GuardState::Alert)
                || matches!(&g.clearing, Some(c) if c.stage != crate::squad::ClearStage::Done)
                || g.time - g.alarm.raised_at < ROLLCALL_QUIET_AFTER_SECS
            {
                let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                rollcall_state(g).next_at = next;
                return;
            }
            let caller = match radio_caller(g) {
                Some(c) => c,
                None => {
                    let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                    rollcall_state(g).next_at = next;
                    return;
                }
            };
            let mut news = false;
            for &x in &staff {
                if x != caller && (!off_net(g, x) || (!g.gd[x].found && !g.gd[x].missing_raised && checker_for(g, x).is_none())) { news = true; break; }
            }
            if !news {
                let next = g.time + rollcall_interval(g);
                rollcall_state(g).next_at = next;
                return;
            }
            let now = g.time;
            let r = rollcall_state(g);
            r.stage = RollcallStage::Called;
            r.t0 = now;
            r.caller = Some(caller);
            r.missing = None;
            r.answered = false;
            r.checks += 1.0;
            crate::game::game_say(g, caller, "radio check — sound off", Some(true), None, None);
            let at = head_or_pos(g, caller);
            play(g, "radio", at, 0.5);
        }
        RollcallStage::Called => {
            let el = g.time - rollcall_state(g).t0;
            if !rollcall_state(g).answered && el >= ROLLCALL_ANSWER_SECS {
                rollcall_state(g).answered = true;
                let caller = rollcall_state(g).caller;
                let mut answers: Vec<String> = Vec::new();
                for &x in &staff {
                    if Some(x) != caller && !off_net(g, x) { answers.push(format!("…{}.", g.gd[x].callsign)); }
                }
                if !answers.is_empty() {
                    let line = format!("radio: {}", answers.join(" "));
                    crate::game::game_msg(g, &line);
                }
                let mut missing: Option<GuardId> = None;
                for &x in &staff {
                    if silent(g, x) && !g.gd[x].missing_raised { missing = Some(x); break; }
                }
                if missing.is_none() {
                    for &x in &staff {
                        if silent(g, x) { missing = Some(x); break; }
                    }
                }
                rollcall_state(g).missing = missing;
                if missing.is_none() { rollcall_idle(g); }
                return;
            }
            let missing = rollcall_state(g).missing;
            if let (Some(who), true) = (missing, el >= ROLLCALL_ASK_SECS) {
                let caller = rollcall_state(g).caller;
                if !caller_ok(g, caller) {
                    let c = radio_caller(g);
                    rollcall_state(g).caller = c;
                }
                if g.gd[who].found || !off_net(g, who) || !g.guards.contains(&who) { rollcall_idle(g); return; }
                let caller = rollcall_state(g).caller;
                if g.gd[who].missing_raised {
                    if let Some(c) = caller {
                        let line = format!("and still nothing from {} — keep looking for him", g.gd[who].callsign);
                        crate::game::game_say(g, c, &line, Some(true), None, None);
                        let at = head_or_pos(g, c);
                        play(g, "radio", at, 0.5);
                    }
                    rollcall_idle(g);
                    return;
                }
                if let Some(c) = caller {
                    let cs = g.gd[who].callsign.clone();
                    let line = format!("{}? … {}, sound off", cs, cs);
                    crate::game::game_say(g, c, &line, Some(true), None, None);
                    let at = head_or_pos(g, c);
                    play(g, "radio", at, 0.55);
                }
                crate::netquiet::net_quiet_asked(g, caller, who, &staff);
                if g.gd[who].missed_at < 0.0 { g.gd[who].missed_at = g.time; }
                rollcall_state(g).stage = RollcallStage::Asked;
            }
        }
        RollcallStage::Asked => {
            let missing = rollcall_state(g).missing;
            let el = g.time - rollcall_state(g).t0;
            let who = match missing {
                Some(w) if off_net(g, w) && !g.gd[w].found && g.guards.contains(&w) && checker_for(g, w).is_none() => w,
                _ => { rollcall_idle(g); return; }
            };
            if el < ROLLCALL_SEND_SECS { return; }
            let caller = rollcall_state(g).caller;
            if !caller_ok(g, caller) {
                let c = radio_caller(g);
                rollcall_state(g).caller = c;
            }
            let checker = match pick_checker(g, who, None) {
                Some(c) => c,
                None => {
                    if el > ROLLCALL_SEND_GIVEUP_SECS {
                        let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                        let r = rollcall_state(g);
                        r.stage = RollcallStage::Idle;
                        r.next_at = next;
                    }
                    return;
                }
            };
            let caller = rollcall_state(g).caller;
            let cs = g.gd[who].callsign.clone();
            if let Some(c) = caller {
                if c != checker {
                    let line = format!("nothing from {}. {}, go walk his route and check on him", cs, g.gd[checker].callsign);
                    crate::game::game_say(g, c, &line, Some(true), None, None);
                    let at = head_or_pos(g, c);
                    play(g, "radio", at, 0.55);
                }
            }
            let line = if caller == Some(checker) {
                format!("nothing from {} — going to walk his route and check on him", cs)
            } else {
                format!("copy — going to check on {}", cs)
            };
            send_check_on(g, checker, who, &line);
            rollcall_idle(g);
        }
    }
}
/// Lockdown follower, each frame on patrol: when he last had his leader in plain sight, and what he does about losing him.   (guards.ts partnerWatch)
pub fn partner_watch(g: &mut crate::game::Game, gd: GuardId) {
    let l = match g.gd[gd].leader { Some(l) => l, None => return };
    let c = g.gd[gd].ch;
    let lc = g.gd[l].ch;
    let eye = match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) };
    let at = if g.gd[l].state != GuardState::Dead {
        match g.ch[lc].bones.get(Bone::Chest) { Some(b) => *b, None => v3::add(g.ch[lc].pos, [0.0, 1.1, 0.0]) }
    } else {
        v3::add(body_pos(g, l), [0.0, 0.4, 0.0])
    };
    let seen = v3::dist(eye, at) < 16.0 && !crate::scene_collision::segment_blocked(g, eye, at, None, None, None);
    if seen || g.gd[gd].leader_seen_t < 0.0 {
        g.gd[gd].leader_seen_t = g.time;
        g.gd[gd].leader_seen_pos = v3::copy(g.ch[lc].pos);
        return;
    }
    let unseen = g.time - g.gd[gd].leader_seen_t;
    let at_spot = v3::dist_xz(g.ch[c].pos, g.gd[gd].leader_seen_pos) < PARTNER_SPOT_M;
    if !g.ai_enabled || unseen < PARTNER_UNSEEN_SECS || (!at_spot && unseen < 2.0 * PARTNER_UNSEEN_SECS) { return; }
    g.gd[gd].leader_seen_t = g.time;
    let (me, him) = (g.gd[gd].callsign.clone(), g.gd[l].callsign.clone());
    if !off_net(g, l) {
        let ask = format!("{}, where'd you go?", him);
        crate::game::game_say(g, gd, &ask, Some(true), None, None);
        let answer = format!("{} — on me, close up", me);
        crate::game::game_say(g, l, &answer, Some(true), None, None);
        play(g, "radio", eye, 0.5);
        return;
    }
    if rollcall_state(g).stage != RollcallStage::Idle || g.gd[l].found || checker_for(g, l).is_some() { return; }
    let line = format!("{}? …I've lost {} — he was just ahead of me. {}, sound off", him, him, him);
    crate::game::game_say(g, gd, &line, Some(true), None, None);
    play(g, "radio", eye, 0.6);
    if g.gd[l].missed_at < 0.0 { g.gd[l].missed_at = g.time; }
    let now = g.time;
    let r = rollcall_state(g);
    r.stage = RollcallStage::Asked;
    r.t0 = now - ROLLCALL_ASK_SECS;
    r.caller = Some(gd);
    r.missing = Some(l);
    r.answered = true;
    r.checks += 1.0;
}

// ---- room clearing

/// planClears' row of a room: its plans (one per door it can be taken from), how near any of its doors is, how it ranks
#[derive(Clone)]
struct RoomRow {
    plans: Vec<crate::scene_level::RoomClearPlan>,
    d: f64,
    tampered: bool,
    inside: bool,
}
/// Locked down: hand the pair the rooms whose doors are nearest the last fix, the tampered ones first.   (guards.ts planClears)
pub fn plan_clears(g: &mut crate::game::Game, force: Option<bool>, cause: Option<crate::guards::PlanClearsCause>) -> bool {
    let force = force.unwrap_or(false);
    let cause = cause.unwrap_or(PlanClearsCause::Search);
    if g.quiet_utility || !g.ai_enabled || escalation_of(g) < 2.0 { return false; }
    if g.clearing.is_some() {
        if !force && cause != PlanClearsCause::Body { return false; }
        crate::squad::clearing_cancel(g);
        g.clearing = None;
    }
    let mut pair: (Option<GuardId>, Option<GuardId>) = (None, None);
    for &gd in &g.guards {
        if g.gd[gd].state == GuardState::Dead || g.gd[gd].held.is_some() { continue; }
        if let Some(l) = g.gd[gd].leader {
            if g.gd[l].state != GuardState::Dead && g.gd[l].held.is_none() { pair = (Some(l), Some(gd)); }
        }
    }
    let (a, b) = pair;
    let a = match a {
        Some(a) => a,
        None => {
            let alive: Vec<GuardId> = g.guards.iter().copied()
                .filter(|&x| g.gd[x].state != GuardState::Dead && g.gd[x].held.is_none() && g.gd[x].team == Team::Guard && g.gd[x].unit.is_none())
                .collect();
            if alive.len() != 1 { return false; }
            alive[0]
        }
    };
    let at = if g.alarm.placed { g.alarm.pos } else { g.ch[g.gd[a].ch].pos };
    let mut by_room: JsMap<String, RoomRow> = JsMap::default();
    for p in g.level.clear_plans.clone().unwrap_or_default() {
        let d = match crate::doors::doors_by_name(g, &p.door) { Some(d) => d, None => continue };
        let k = match &p.room { Some(r) => r.clone(), None => p.door.clone() };
        let dist = v3::dist(g.dr[d].frame_centre, at);
        let tampered = door_tamper(g, d) == Some(DoorTamper::Forced)
            || (g.dr[d].def.locked == Some(true) && crate::doors::door_noticed(g, d) && g.time - crate::doors::door_noticed_t(g, d) < TAMPER_MEMORY_SECS);
        let inside = match p.bounds.clone() { Some(b) => crate::squad::in_bounds(at, Some(b), None), None => false };
        let r = by_room.get_or_insert_with(k, || RoomRow { plans: Vec::new(), d: f64::INFINITY, tampered: false, inside: false });
        r.plans.push(p);
        r.d = m::min(r.d, dist);
        if tampered { r.tampered = true; }
        if inside { r.inside = true; }
    }
    let n = if g.clear_rooms > 0.0 { g.clear_rooms } else if cause == PlanClearsCause::Body { 3.0 } else { 2.0 };
    let now = g.time;
    let mut ranked: Vec<RoomRow> = Vec::new();
    for (k, r) in by_room.iter() {
        let recent = match g.cleared_at.get(k) { Some(t) => now - *t < RECLEAR_SECS, None => false };
        if !recent || r.inside { ranked.push(r.clone()); }
    }
    js::sort_by(&mut ranked, |x, y| js::or((y.tampered as i32 - x.tampered as i32) as f64, x.d - y.d));
    let rooms: Vec<Vec<crate::scene_level::RoomClearPlan>> = ranked.into_iter().take(n as usize).map(|r| r.plans).collect();
    if rooms.is_empty() {
        crate::game::game_note(g, "nothing near to clear — those rooms were done a minute ago");
        return false;
    }
    let c = crate::squad::clearing_new(g, &rooms, a, b);
    g.clearing = Some(c);
    true
}
/// `v[0].room ?? v[0].door`
fn room_label(v: &[crate::scene_level::RoomClearPlan]) -> String {
    match &v[0].room { Some(r) => r.clone(), None => v[0].door.clone() }
}
/// Panel: the rooms still ahead of the pair and the ones called clear lately.   (guards.ts clearQueueSummary)
pub fn clear_queue_summary(g: &mut crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(c) = &g.clearing {
        if c.stage != crate::squad::ClearStage::Done {
            let ahead: Vec<String> = c.rooms.iter().skip(c.room_i as usize).map(|v| room_label(v)).collect();
            parts.push(format!("clear queue: {}", ahead.join(" ▸ ")));
        }
    }
    let mut entries: Vec<(String, f64)> = g.cleared_at.iter().map(|(k, t)| (k, *t)).collect();
    js::sort_by(&mut entries, |x, y| y.1 - x.1);
    let done: Vec<String> = entries.iter().map(|(k, t)| {
        format!("{} ({} s ago{})", k, num(m::round(g.time - *t)), if g.time - *t < RECLEAR_SECS { ", holds" } else { "" })
    }).collect();
    if !done.is_empty() { parts.push(format!("cleared this lockdown: {}", done.join(", "))); }
    parts.join(" · ")
}
/// Panel: 'clear nearest room now': the fix where the player stands, the floor locked down if it was not.   (guards.ts forceClear)
pub fn force_clear(g: &mut crate::game::Game) {
    if g.quiet_utility { crate::game::game_msg(g, "no clear under the tour"); return; }
    let p = g.ch[g.pl[g.player].ch].pos;
    g.alarm.pos[0] = p[0];
    g.alarm.pos[1] = 0.0;
    g.alarm.pos[2] = p[2];
    g.alarm.placed = true;
    if g.escalation < 2.0 {
        raise_escalation(g, 2.0, None, None, None);
    } else {
        plan_clears(g, Some(true), Some(PlanClearsCause::Panel));
    }
    let line = match &g.clearing {
        None => String::from(if !g.ai_enabled { "no clear: guard AI is off" } else { "no clear: nobody to send / no plans" }),
        Some(c) => {
            let names: Vec<String> = c.rooms.iter().map(|r| room_label(r)).collect();
            format!("clear ordered: {}", names.join(", "))
        }
    };
    crate::game::game_note(g, &line);
}

#[cfg(test)]
mod tests {
    use super::*;

    // bits from the TypeScript's own expression, under bun
    #[test]
    fn a_line_stands_by_its_length_in_utf16_units() {
        assert_eq!(bubble_secs("radio check — sound off").to_bits(), 0x40061eb851eb851e);
        assert_eq!(bubble_secs("x").to_bits(), 0x4000000000000000);
        assert_eq!(bubble_secs("and still nothing from Bravo — keep looking for him, all of you, all of you now right away").to_bits(), 0x4010000000000000);
    }

    #[test]
    fn the_frame_test_takes_either_shape_of_matrix() {
        let doubles: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let singles: Vec<f32> = doubles.iter().map(|x| *x as f32).collect();
        for vp in [InViewVp::Array(doubles.clone()), InViewVp::Float32Array(singles)] {
            assert!(in_view(Some(&vp), [0.5, 0.5, 0.0]));
            assert!(!in_view(Some(&vp), [0.95, 0.0, 0.0]));
            assert!(!in_view(Some(&vp), [0.0, -0.91, 0.0]));
        }
        assert!(!in_view(None, [0.0, 0.0, 0.0]));
        let behind = InViewVp::Array(vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0]);
        assert!(!in_view(Some(&behind), [0.0, 0.0, 1.0]));
    }

    #[test]
    fn a_write_through_the_rollcall_row_lands() {
        let mut g = Game::blank();
        assert_eq!(rollcall_state(&mut g).next_at, -1.0);
        let r = rollcall_state(&mut g);
        r.stage = RollcallStage::Asked;
        r.checks += 1.0;
        assert!(rollcall_state(&mut g).stage == RollcallStage::Asked);
        assert_eq!(g.m.guards.rollcalls.as_ref().unwrap().checks, 1.0);
    }

    #[test]
    fn the_levels_have_their_names() {
        assert_eq!(escalation_name(0.0), "calm");
        assert_eq!(escalation_name(1.0), "heightened");
        assert_eq!(escalation_name(2.0), "lockdown");
    }
}
