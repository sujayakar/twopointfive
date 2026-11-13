//! ALERT: the fight. A rule of the list's cover stage may have him first (control/); else he reacts, stands off, shoots, closes, or goes to the man he dropped.
use super::{Acts, Sensed};
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use crate::game::GuardState as St;
use crate::character::UpperMode;

/// What the stages of his fight share this tick.
#[derive(Clone, Copy)]
struct Fight {
    gd: GuardId,
    dt: f64,
    c: CharId,
    /// the man he is fighting
    pc: Option<CharId>,
    can_shoot: bool,
    staff: bool,
    dist: f64,
    dazzled: bool,
    staggered: bool,
}

pub(in crate::people) fn alert(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed) -> Acts {
    let &Sensed { c, staff, dist, pair, dazzled, staggered, .. } = s;
    let mut a = Acts { upper: UpperMode::Aim, ..Acts::still() };
    note_door_crossings(g, gd);
    let pc = g.gd[gd].foe;
    let down = crate::teams::foe_down(g, gd);
    let can_shoot = pc.is_some() && !down && !dazzled && !staggered && (g.time - g.gd[gd].last_seen_t < 0.5);
    if g.gd[gd].react_t > 0.0 || !can_shoot { g.gd[gd].shot_blocked_t = 0.0; g.gd[gd].line_in_t = 0.0; }
    if g.gd[gd].react_t > 0.0 || down || g.gd[gd].duty.pinned { g.gd[gd].stall_ref = None; g.gd[gd].chase_t = 0.0; }
    let f = Fight { gd, dt, c, pc, can_shoot, staff, dist, dazzled, staggered };
    // TAKING COVER (control/): a rule of the list's cover stage may have him before his fight does; the first is a man getting his head down
    let cover = crate::control::grant_cover(g, gd, dt);
    let in_standoff = g.gd[gd].standoff.is_some() && pair.is_some();
    if let Some((rule, want)) = cover {
        takes_cover(g, f, rule, want, &mut a);
    } else if g.gd[gd].react_t > 0.0 {
        still_reacting(g, f, &mut a);
    } else if in_standoff {
        let r = standoff_alert(g, gd, dt, pair.unwrap());
        a.move_speed = r.move_;
        a.aim_at = Some(r.aim);
        a.crouch = r.crouch;
        g.gd[gd].shot_blocked_t = 0.0;
        g.gd[gd].line_in_t = 0.0;
    } else if can_shoot {
        shoots_at_him(g, f, pc.unwrap(), &mut a);
    } else if let (true, Some(pc)) = (down, pc) {
        stands_over_him(g, f, pc, &mut a);
    } else {
        hunts_him(g, f, &mut a);
    }
    a
}

/// A rule has him getting his head down or going to cover: he aims at the danger, and on his way fires at the man he sees.
fn takes_cover(g: &mut Game, f: Fight, rule: crate::control::RuleId, want: crate::control::TakeCover, a: &mut Acts) {
    let Fight { gd, dt, c, pc, can_shoot, .. } = f;
    crate::control::record(g, gd, crate::control::Holder::Rule(rule));
    a.duck = true;
    g.gd[gd].shot_blocked_t = 0.0; g.gd[gd].line_in_t = 0.0; g.gd[gd].chase_t = 0.0;
    if want.to.is_none() { g.arms[g.gd[gd].ch].fire_cd = m::max(g.arms[g.gd[gd].ch].fire_cd, want.fire_wait); }
    if g.gd[gd].react_t > 0.0 { g.gd[gd].react_t -= dt; }
    let threat = match want.threat {
        Some(s) => s,
        None => match (crate::belief::gun_point(g, gd), pc) {
            (Some(gp), _) => gp,
            (None, Some(p)) => g.ch[p].pos,
            (None, None) => aim_ahead(g, c),
        },
    };
    a.aim_at = Some([threat[0], 1.1, threat[2]]);
    a.crouch = want.crouch;
    let outcome = legs_under_cover(g, f, want, a);
    if let (true, true, Some(pc)) = (want.to.is_some(), can_shoot && g.gd[gd].react_t <= 0.0, pc) {   // on his way he fires at the man he sees
        let s = fight_sight(g, gd, c, pc, dt);
        a.aim_at = Some(s.aim);
        a.duck = false;
        let ready = crate::aim::aim_ready_walking(g, gd);
        fight_trigger(g, gd, c, &s, a.move_speed, ready, dt);
    }
    crate::control::commit(g, gd, rule, outcome);
}

/// (A MAN WHO CANNOT SEE GOES NOWHERE: whatever spot a rule names, a dazzled or staggered man gets down where he is. Said here once, for
///  every rule of the cover stage; the rule keeps its spot and he goes when he can see)
fn legs_under_cover(g: &mut Game, f: Fight, want: crate::control::TakeCover, a: &mut Acts) -> crate::control::Outcome {
    let Fight { gd, dt, c, dazzled, staggered, .. } = f;
    let Some(spot) = want.to.filter(|_| !dazzled && !staggered) else {
        brake(g, gd, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].stall_ref = None;
        return crate::control::Outcome::Down;
    };
    go_to(g, gd, spot, None);
    let arrived = follow_path(g, gd, dt, want.pace);
    a.move_speed = g.gd[gd].speed;
    if arrived { crate::control::Outcome::Arrived }
    else if not_getting_anywhere(g, gd, dt, 1.5) { crate::control::Outcome::GoingNowhere }
    else { crate::control::Outcome::Moving }
}

/// His moment before he acts: he stops, looking where he believes the gun is or at the man.
fn still_reacting(g: &mut Game, f: Fight, a: &mut Acts) {
    let Fight { gd, dt, pc, dazzled, .. } = f;
    g.gd[gd].react_t -= dt;
    let gp = if dazzled || g.time - g.gd[gd].last_seen_t > 0.5 { crate::belief::gun_point(g, gd) } else { None };
    a.aim_at = if let Some(gp) = gp {
        Some(v3::add(gp, [0.0, 1.1, 0.0]))
    } else if let Some(p) = pc {
        Some(v3::add(g.ch[p].pos, [0.0, 1.2, 0.0]))
    } else { None };
    brake(g, gd, 8.0, dt);
}

/// He sees his man: his gun finds the point, his legs close or clear the line, and his finger does the rest; no shot for too long and he drops to a search.
fn shoots_at_him(g: &mut Game, f: Fight, pc: CharId, a: &mut Acts) {
    let Fight { gd, dt, c, dist, .. } = f;
    let s = fight_sight(g, gd, c, pc, dt);
    a.aim_at = Some(s.aim);
    let (closing, stepping) = fight_legs(g, gd, c, pc, &s, dist, dt, &mut a.move_speed);
    if !(s.blocked && !closing) {
        if !stepping { g.gd[gd].stall_ref = None; }
        g.gd[gd].chase_t = 0.0;
    } else if g.gd[gd].shot_blocked_t > 4.0 && !s.pinned {
        drop_to_search(g, gd, Some("no shot — lost him"));
        crate::mind::cap(g, gd, 0.5, "guards");
        return;
    }
    let ready = crate::aim::aim_ready(g, gd);
    fight_trigger(g, gd, c, &s, a.move_speed, ready, dt);
}

/// His man is down: he goes to him, covers him, calls it in once, and in time stands easy.
fn stands_over_him(g: &mut Game, f: Fight, pc: CharId, a: &mut Acts) {
    let Fight { gd, dt, c, staff, .. } = f;
    let body = g.ch[pc].pos;
    let here = g.ch[c].pos;
    let d_b = v3::dist_xz(body, here);
    if d_b > 2.2 {
        go_to(g, gd, body, None);
        follow_path(g, gd, dt, g.k.tune.guard_investigate);
        a.move_speed = g.gd[gd].speed;
    } else {
        brake(g, gd, 6.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
    }
    a.aim_at = Some(v3::add(body, [0.0, 0.3, 0.0]));
    g.gd[gd].search_t += dt;
    if g.gd[gd].search_t > 4.0 && !g.called_in && staff {
        g.called_in = true;
        let line = if !crate::dialogue::is_office(g) { "target is down. send someone over." } else { "target is down. send someone up." };
        crate::game::game_say(g, gd, line, Some(true), None);
        let at = head_or_pos(g, c);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, Some(at), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.95 }));
    }
    if g.gd[gd].search_t > 14.0 {
        crate::mind::set_state(g, gd, St::Suspicious, "guards");
        crate::mind::set(g, gd, 0.3, "guards");
        g.gd[gd].search_t = 0.0;
        g.gd[gd].react_t = 0.0;
        crate::mind::fix(g, gd, Some(body), "guards");
    }
}

/// He cannot see his man: covering fire, a door to force, his post's line to the gun, the chase to his fix, or with nothing left a search.
fn hunts_him(g: &mut Game, f: Fight, a: &mut Acts) {
    let Fight { gd, dt, c, .. } = f;
    a.aim_at = crate::coverfire::cover_blind(g, gd, dt);
    if a.aim_at.is_some() {
        brake(g, gd, 6.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
    } else if let Some(forced) = crate::doorforce::door_force(g, gd, dt) {
        a.aim_at = Some(forced.aim);
        a.move_speed = forced.move_;
    } else if let Some(gp) = crate::belief::gun_point(g, gd).filter(|_| g.gd[gd].duty.pinned) {
        let at = v3::add(gp, [0.0, 1.1, 0.0]);
        a.aim_at = Some(at);
        if crate::doorway::off_threshold(g, gd, Some(at), dt, false) {
            a.move_speed = g.gd[gd].speed;
        } else {
            brake(g, gd, 6.0, dt);
            g.ch[c].vel = [0.0, 0.0, 0.0];
        }
    } else if g.gd[gd].last_known.is_some() {
        let tac = if g.k.tune.tactics { crate::tactics::tactics_alert(g, gd, dt) } else { None };
        let (mv, at, pd, cr) = match tac {
            Some(r) => (r.move_, r.aim, r.pied, r.crouch),
            None => { let pace = g.k.tune.guard_chase; let r = chase_fix(g, gd, dt, pace); (r.move_, r.aim, r.pied, r.crouch) }
        };
        a.move_speed = mv; a.aim_at = at; a.pied = pd; a.crouch = cr;
    } else if g.gd[gd].duty.free() && g.time - g.gd[gd].last_seen_t > 2.0 {
        drop_to_search(g, gd, None);
    }
}

/// FIRING AS HE GOES, a man whose script has his legs: he turns to the man he sees, going backwards or sideways, and shoots, his aim what his own walk leaves of it.
pub fn fight_on_the_way(g: &mut Game, gd: GuardId, dt: f64) {
    let c = g.gd[gd].ch;
    let foe = g.gd[gd].foe.filter(|_| {
        !crate::teams::foe_down(g, gd) && g.time - g.gd[gd].last_seen_t < 0.5 && !crate::control::man::blind(g, gd) && !crate::control::man::staggered(g, gd)
    });
    let Some(pc) = foe else {
        if let Some(s) = g.gd[gd].script.as_mut() { s.over = None; }
        return;
    };
    if g.gd[gd].react_t > 0.0 { g.gd[gd].react_t -= dt; }
    let s = fight_sight(g, gd, c, pc, dt);
    let face = Some(v3::yaw_to(g.ch[c].pos, s.aim));
    if let Some(sc) = g.gd[gd].script.as_mut() { sc.over = Some(crate::game::GuardScriptOver { aim_at: s.aim, face, walk: Some(true) }); }
    let speed = g.gd[gd].speed;
    crate::aim::track_guard(g, gd, dt);
    let ready = crate::aim::aim_ready_walking(g, gd);
    if g.gd[gd].react_t <= 0.0 { fight_trigger(g, gd, c, &s, speed, ready, dt); }
}

/// What a fighting guard's gun can do about the man he sees, this tick: the point he aims at and what is in the way of the round.
struct Sight {
    aim: V3,
    /// no clear line to his trunk or his head, or his own bore is fouled (and no colleague's covering fire has the point)
    blocked: bool,
    /// his arm or his gun is through or against a wall
    poked: bool,
    /// how far his gun is from the line to the point, radians
    facing_err: f64,
    /// a colleague in the line of his muzzle, and whether that man is walking across it
    in_line: Option<GuardId>,
    crossing_man: bool,
    /// he is pinned to his post (read once, as the fight began this tick)
    pinned: bool,
}

/// AIM of a fighting guard: the point on his man he can hit (the trunk, else the head) and what stops the round. It keeps his
/// two clocks of a spoiled shot (`shot_blocked_t`, `line_in_t`); it moves nobody and fires nothing.
fn fight_sight(g: &mut Game, gd: GuardId, c: CharId, pc: CharId, dt: f64) -> Sight {
    use crate::character::Bone;
    let crouched = crate::teams::foe_crouched(g, gd);
    let trunk = match g.ch[pc].bone(Bone::Spine2) { Some(b) => Some(b), None => g.ch[pc].bone(Bone::Hips) };
    let pp = g.ch[pc].pos;
    let mut aim: V3 = [match trunk { Some(t) => t[0], None => pp[0] }, pp[1] + (if crouched { 0.8 } else { 1.25 }), match trunk { Some(t) => t[2], None => pp[2] }];
    let pinned = g.gd[gd].duty.pinned;
    let mut blocked = false;
    if (!pinned || pane_first(g, c, aim)) && !clear_shot(g, c, aim) {
        let head = match g.ch[pc].bone(Bone::Head) { Some(b) => b, None => v3::add(pp, [0.0, if crouched { 1.05 } else { 1.6 }, 0.0]) };
        if clear_shot(g, c, head) { aim = head; } else { blocked = true; }
    }
    if !blocked && !pinned && crate::aim::bore_fouled(g, gd) { blocked = true; }
    if blocked {
        let who = g.gd[gd].foe.clone();
        if crate::coverfire::cover_seen(g, gd, who, aim) { blocked = false; }
    }
    let here = g.ch[c].pos;
    let gun_yaw = crate::character::character_gun_yaw(g, c);
    let facing_err = m::abs(v3::wrap_angle(v3::yaw_to(here, aim) - gun_yaw));
    let walled = !pinned && crate::gunwall::gun_wall_check(g, gd, Some(crate::gunwall::GunUp::Aim), Some(aim), dt, Some(true));
    let poked = !pinned && (crate::combat::arm_through(g, c) || walled);
    if poked && facing_err < 0.3 { blocked = true; }
    g.gd[gd].shot_blocked_t = if blocked { g.gd[gd].shot_blocked_t + dt } else { 0.0 };
    let in_line = muzzle_check(g, gd, Some(aim), dt, None);
    let crossing_man = match in_line { Some(o) => { let v = g.ch[g.gd[o].ch].vel; m::hyp2(v[0], v[2]) > 0.4 } None => false };
    g.gd[gd].line_in_t = if in_line.is_some() { g.gd[gd].line_in_t + dt } else { 0.0 };
    Sight { aim, blocked, poked, facing_err, in_line, crossing_man, pinned }
}

/// LEGS of a fighting guard: close on a man who is far or whom he cannot hit, step out of a colleague's line, clear a doorway,
/// else stand. Returns (he is closing, he is stepping aside); `move_speed` is written only where he moved.
fn fight_legs(g: &mut Game, gd: GuardId, c: CharId, pc: CharId, s: &Sight, dist: f64, dt: f64, move_speed: &mut f64) -> (bool, bool) {
    let Sight { aim, blocked, in_line, crossing_man, pinned, .. } = *s;
    let (mut closing, mut stepping) = (false, false);
    if !pinned && (dist > 9.0 || blocked) && !crate::suppression::holding_cover(g, gd) {
        let braking = crate::aim::aim_brake(g, gd, dist, blocked);
        if braking && g.gd[gd].speed < 0.3 {
            brake(g, gd, 6.0, dt);
            g.ch[c].vel = [0.0, 0.0, 0.0];
            closing = true;
        } else {
            go_to(g, gd, g.ch[pc].pos, None);
            let pace = if braking { 0.0 } else if crate::suppression::flinching(g, gd) { g.k.tune.guard_investigate } else { g.k.tune.guard_chase };
            closing = !follow_path(g, gd, dt, pace);
            if braking { brake(g, gd, 6.0, dt); }
        }
        *move_speed = g.gd[gd].speed;
    } else if !pinned && g.gd[gd].muzzle_down && !(crossing_man && g.gd[gd].line_in_t < 0.45) {
        let st = line_step(g, gd, aim, in_line, dt);
        if matches!(st, crate::guards::LineStep::Step) {
            stepping = true;
            *move_speed = g.gd[gd].speed;
        } else if matches!(st, crate::guards::LineStep::None) && dist > LINE_PUSH_MIN_M {
            go_to(g, gd, g.ch[pc].pos, None);
            follow_path(g, gd, dt, g.k.tune.guard_investigate);
            *move_speed = g.gd[gd].speed;
        } else {
            brake(g, gd, 6.0, dt);
            g.ch[c].vel = [0.0, 0.0, 0.0];
        }
    } else if crate::doorway::off_threshold(g, gd, Some(aim), dt, true) {
        *move_speed = g.gd[gd].speed;
    } else {
        brake(g, gd, 6.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
    }
    (closing, stepping)
}

/// HANDS of a fighting guard: the magazine and the trigger. A shield in the way, a twisted body, a gun through a wall, a muzzle
/// held down for a colleague or a bore not yet on the point (`ready`) holds the round; an empty gun starts the change of magazine.
fn fight_trigger(g: &mut Game, gd: GuardId, c: CharId, s: &Sight, move_speed: f64, ready: bool, dt: f64) {
    let Sight { aim, blocked, poked, facing_err, .. } = *s;
    g.arms[g.gd[gd].ch].fire_cd -= dt;
    let twisted = move_speed > 0.3 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - g.ch[c].aim_yaw)) > 0.9;
    let shield = g.k.tune.muzzle_discipline && shield_up(g, &crate::seats::here(g));
    if shield { g.arms[g.gd[gd].ch].fire_cd = m::max(g.arms[g.gd[gd].ch].fire_cd, SHIELD_RELEASE_BEAT_MIN + dt); }
    let weapon = g.gd[gd].weapon;
    if crate::gunslots::side_locked(g, gd) {
        // his hands are between his two guns, or off his gun for a charge: no round, no magazine
    } else if crate::control::man::changing_magazine(g, gd) {
        // no round while it lasts (frame.rs `hands` counts it)
    } else if g.m.weapons.weapons.get(g.fa[weapon].def).expect("a gun's def is a row").burst.is_some() {
        let muzzle_down = g.gd[gd].muzzle_down;
        let start_ok = !shield && facing_err < 0.25 && !blocked && !twisted && !poked && !muzzle_down && ready;
        let round_ok = !shield && !twisted && !poked && !muzzle_down;
        burst_trigger(g, gd, dt, aim, start_ok, round_ok, None);
    } else if !shield && g.arms[g.gd[gd].ch].fire_cd <= 0.0 && facing_err < 0.25 && !blocked && !twisted && !poked && !g.gd[gd].muzzle_down
        && !round_past_colleague(g, gd, aim, None, None) && ready && crate::weapons::firearm_fire(g, weapon) {
        let (cd, jitter) = { let w = g.m.weapons.weapons.get(g.fa[weapon].def).expect("a gun's def is a row"); (w.cd, w.cd_jitter) };
        let mods = crate::suppression::guard_fire_mods(g, gd);
        g.arms[g.gd[gd].ch].fire_cd = (cd + crate::dice::draw_of(g, "guards::fight_trigger#1", gd) * jitter) * mods.gap;
        let round = crate::aim::guard_round(g, gd, mods.spread);
        crate::combat::fire_weapon(g, c, aim, Some(&round));
        crate::aim::note_bore(g, gd, aim);
        if crate::weapons::firearm_rounds_ready(g, weapon) == 0.0 { start_reload_beat(g, gd); }
    }
}
