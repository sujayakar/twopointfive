use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- small lookups of the tick ----

fn hit(g: &Game, input: InputId, code: &str) -> bool { crate::input::input_hit(g, input, code) }
fn down(g: &Game, input: InputId, code: &str) -> bool { crate::input::input_down(g, input, code) }
fn shift_down(g: &Game, input: InputId) -> bool { down(g, input, "ShiftLeft") || down(g, input, "ShiftRight") }
/// g.audio.play(name, at, vol)
fn play(g: &mut Game, name: &str, at: Option<V3>, vol: f64) { crate::audio_iface::game_audio_like_play(g, name, at, Some(vol), None) }
/// a gun's def: its row of WEAPONS
fn def_row(g: &Game, def: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef { g.m.weapons.weapons.get(def).unwrap() }
/// a gun's def.auto?.interval
fn def_auto(g: &Game, def: crate::weapons::WeaponId) -> Option<f64> { def_row(g, def).auto.as_ref().map(|a| a.interval) }
/// what of a Legs the door's sprint rule reads
fn door_stamps(m: &crate::player::Legs) -> crate::sprintdoor::DoorStamps {
    crate::sprintdoor::DoorStamps { down: m.down, door_rip_until: m.door_rip_until, door_bash_at: m.door_bash_at, door_bash_lean: m.door_bash_lean }
}
/// The player's own fields as a Legs: the tick hands `g.player` to what a look ahead hands its copy.
fn legs_of(g: &Game, pl: PlayerId) -> crate::player::Legs {
    let (p, c) = (&g.pl[pl], &g.ch[g.pl[pl].ch]);
    crate::player::Legs {
        char: crate::player::LegsChar { pos: c.pos, vel: c.vel, body_yaw: c.body_yaw, aim_yaw: c.aim_yaw },
        safe: p.safe, crouch: p.crouch, sprinting: p.sprinting, dash: p.dash, backpedal: p.backpedal, speed_sm: p.speed_sm, pace: p.pace,
        tow: p.tow, shoved: p.shoved, shoved_t: p.shoved_t, shoved_from: p.shoved_from,
        down: g.cond[p.ch].down, door_rip_until: p.door_rip_until, door_bash_at: p.door_bash_at, door_bash_lean: p.door_bash_lean,
    }
}
/// A call on the player's legs: lent to it as a Legs, and what the Legs lines write put back. Only those fields: what a callee
/// changed on the player through `g` (a body let go of, a door's stamp) stays as it left it.
fn own_legs<R>(g: &mut Game, pl: PlayerId, f: impl FnOnce(&mut Game, &mut crate::player::Legs) -> R) -> R {
    let mut m = legs_of(g, pl);
    let r = f(g, &mut m);
    let c = g.pl[pl].ch;
    let ch = &mut g.ch[c];
    ch.pos = m.char.pos; ch.vel = m.char.vel; ch.body_yaw = m.char.body_yaw; ch.aim_yaw = m.char.aim_yaw;
    let p = &mut g.pl[pl];
    p.safe = m.safe; p.crouch = m.crouch; p.sprinting = m.sprinting; p.dash = m.dash; p.backpedal = m.backpedal; p.speed_sm = m.speed_sm;
    p.shoved = m.shoved; p.shoved_t = m.shoved_t; p.shoved_from = m.shoved_from;
    r
}
/// collideMan(g, m, y1) on a Legs: its stub takes the two fields it moves
fn collide_legs(g: &Game, m: &mut crate::player::Legs, y1: f64) {
    let mut pl = crate::player::CollideManPl { char: crate::player::CollideManPlChar { pos: m.char.pos }, safe: m.safe };
    collide_man(g, &mut pl, y1);
    m.char.pos = pl.char.pos; m.safe = pl.safe;
}

// ---- the part's functions ----

/// how a player steps to the launcher's next load

/// Is what moves him this tick his walk keys and nothing else. Asked, nothing written.
pub fn legs_own(g: &crate::game::Game) -> bool {
    let pl = g.player;
    let busy = crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].planted() || g.pl[pl].wall.is_some() || g.pl[pl].tow.is_some() || g.pl[pl].door_move().is_some();
    !(busy || crate::handwork::sam_work(g).is_some() || crate::stalk::stalk_on(g, &crate::seats::here(g)).is_some() || crate::rails::rails_allowed(g).is_some())
}

/// The top pace of a walk of his own this tick, by what his legs are doing and what his card weighs.
fn free_pace(g: &crate::game::Game, seat: &crate::seats::SeatRef, m: crate::player::Legs, ml: f64) -> f64 {
    let mut max_speed = if ml == 0.0 { 0.0 }
        else if m.sprinting { g.tune.player_sprint }
        else if let Some(t) = m.tow { if t.haul.is_some() { crate::hands::haul_speed(g, seat) } else { g.tune.player_crouch * 0.6 } }
        else if m.dash { g.k.crouchrun.pace }
        else { (if m.crouch { crouch_pace(g) } else { g.tune.player_walk }) * m.pace };
    crate::kitload::sam_load_pace(g, g.pl[seat.player].ch, max_speed, Some(m.sprinting))
}

/// the walk keys, ahead, left, back and right
pub const WALK_KEYS: [&str; 4] = ["KeyW", "KeyA", "KeyS", "KeyD"];
/// Which of them a hand holds to go from `from` toward `to`, in a view that looks along `forward`: the nearest of the eight ways they can go.
pub fn walk_keys(from: V3, to: [f64; 2], forward: V3) -> [bool; 4] {
    let (dx, dz) = (to[0] - from[0], to[1] - from[2]);
    let (d, l) = (m::hyp2(dx, dz), m::hyp2(forward[0], forward[2]));
    if d <= 0.0 || l <= 0.0 { return [false; 4]; }
    let (fx, fz) = (forward[0] / l, forward[2] / l);
    let (ahead, right) = ((dx * fx + dz * fz) / d, (dz * fx - dx * fz) / d);
    [ahead > 0.3827, right < -0.3827, ahead < -0.3827, right > 0.3827]
}

/// His move keys as a way across the floor, in the view's basis, not yet of unit length.
pub fn keys_way(fwd: V3, right: V3, w: bool, s: bool, d: bool, a: bool) -> [f64; 2] {
    let (mut mx, mut mz) = (0.0, 0.0);
    if w { mx += fwd[0]; mz += fwd[2]; }
    if s { mx -= fwd[0]; mz -= fwd[2]; }
    if d { mx += right[0]; mz += right[2]; }
    if a { mx -= right[0]; mz -= right[2]; }
    [mx, mz]
}

/// C: down, or up where there is the headroom; false: no room to stand.
pub fn crouch_key(g: &crate::game::Game, m: &mut crate::player::Legs) -> bool {
    if !m.crouch { m.crouch = true; }
    else if can_stand_at(g, m.char.pos) { m.crouch = false; }
    else { return false; }
    true
}

/// His stance for this stride: Shift and a direction stands him up to run, or keeps him down and quick; a body in his hands sets it.
pub fn sprint_stance(g: &crate::game::Game, m: &mut crate::player::Legs, want_sprint: bool, stalk: bool) {
    let dragging = m.tow.is_some();
    if dragging { m.crouch = crate::hands::haul_crouch(g); }
    m.sprinting = want_sprint && !m.crouch;
    m.dash = want_sprint && m.crouch && !dragging && !stalk;
}

/// One stride: his pace taken up or let go, his place moved by it, the walls and then the men given their say.
pub fn stride_step(g: &mut crate::game::Game, m: &mut crate::player::Legs, mx: f64, mz: f64, ml: f64, stalk: Option<crate::player::StrideStepStalk>, dt: f64, now: Option<f64>) {
    let now = now.unwrap_or(g.time);
    let mut max_speed = free_pace(g, &crate::seats::here(g), *m, ml);
    if let Some(s) = stalk { max_speed = if ml == 0.0 { 0.0 } else { s.pace }; }
    if m.sprinting { max_speed *= crate::sprintdoor::sprint_door_pace(g, door_stamps(m), Some(now)); }
    let rate = match stalk {
        Some(s) => if s.keep.unwrap_or(false) { 12.0 } else if max_speed > m.speed_sm { g.k.stalk.ease_up } else { g.k.stalk.ease_down },
        None => if max_speed > m.speed_sm { 6.0 } else { 12.0 },
    };
    m.speed_sm = v3::damp(m.speed_sm, max_speed, rate, dt);
    let moving = ml > 0.0;
    if moving { m.char.vel = [mx * m.speed_sm, 0.0, mz * m.speed_sm]; } else { m.char.vel = v3::scale(m.char.vel, m::exp(-14.0 * dt)); }
    m.char.pos = v3::mad(m.char.pos, m.char.vel, dt);
    let headroom = if m.crouch { 1.0 } else { 1.7 };
    collide_legs(g, m, headroom);
    keep_off_guards(g, dt, headroom, Some(m));
}

/// Is his gun up to aim: the pistol just fired or with its light on, the rifle braced.
pub fn gun_raised(g: &crate::game::Game, pl: PlayerId, now: Option<f64>, m: Option<crate::player::GunRaisedM>) -> bool {
    let now = now.unwrap_or(g.time);
    let sprinting = match m { Some(m) => m.sprinting, None => g.pl[pl].sprinting };
    let pistol = g.pl[pl].pistol;
    (crate::game::player_pistol_up(g, pl) && (now - g.pl[pl].last_fire_t < 1.6 || g.fa[pistol].light_on || g.sets[crate::seats::seat_of_man(g, pl)].hand.aim_held))
        || crate::game::player_braced_at(g, pl, sprinting)
}

/// The way a leaf he is pushing open turns him: square into its face, unless the gun is up or it merely rests on him.
pub fn leaf_facing(dp: Option<&crate::player::LeafFacingDp>, aiming: bool) -> Option<f64> {
    let dp = dp?;
    if dp.w >= 1.0 && !aiming { Some(m::atan2(-dp.n[0], -dp.n[2])) } else { None }
}

/// A sprint's facing: gun down, the body follows the run and the aim is the body's. Returns the shoulder's turn into a door just run open.
pub fn sprint_facing(g: &crate::game::Game, m: &mut crate::player::Legs, dt: f64, now: Option<f64>) -> Option<f64> {
    let now = now.unwrap_or(g.time);
    let lean = crate::sprintdoor::sprint_door_lean(g, door_stamps(m), Some(now));
    m.char.body_yaw = v3::approach_angle(m.char.body_yaw, v3::yaw_of(m.char.vel) + lean.unwrap_or(0.0), 14.0 * dt);
    m.char.aim_yaw = m.char.body_yaw;
    m.backpedal = false;
    lean
}

/// The hips of a walk of his own, his aim already turned; a leaf he is walking open or working by hand turns them instead.
pub fn hips_turn(m: &mut crate::player::Legs, speed: f64, aiming: bool, orbit: bool, into_door: Option<f64>, by_hand: bool, dt: f64) {
    if speed > 0.2 {
        let move_yaw = v3::yaw_of(m.char.vel);
        let d = m::cos(v3::wrap_angle(m.char.aim_yaw - move_yaw));
        if by_hand { m.backpedal = m::cos(v3::wrap_angle(into_door.unwrap() - move_yaw)) < 0.0; }
        else if into_door.is_some() { m.backpedal = false; }
        else if if m.backpedal { d > -0.05 } else { d < -0.35 } { m.backpedal = !m.backpedal; }
        let base = if m.backpedal { v3::wrap_angle(move_yaw + m::PI) } else { move_yaw };
        let body_target = if by_hand { into_door.unwrap() }
            else if let Some(door) = into_door { base + v3::clamp(v3::wrap_angle(door - base), -0.35, 0.35) }
            else { base + v3::clamp(v3::wrap_angle(m.char.aim_yaw - base), -0.8, 0.8) * (if aiming || orbit { 1.0 } else { 0.6 }) };
        m.char.body_yaw = v3::approach_angle(m.char.body_yaw, body_target, (if into_door.is_some() { 12.0 } else { 9.0 }) * dt);
    } else {
        m.backpedal = false;
        let toward = into_door.unwrap_or(m.char.aim_yaw);
        let off = v3::wrap_angle(toward - m.char.body_yaw);
        let limit = if by_hand { 0.1 } else if into_door.is_some() { 0.25 } else if aiming { 0.6 } else { 1.2 };
        if m::abs(off) > limit {
            m.char.body_yaw = v3::approach_angle(m.char.body_yaw, toward - m::sign(off) * limit * 0.7, (if by_hand { 12.0 } else { 7.0 }) * dt);
        }
    }
}

// ---- pieces of updatePlayer: its closures, and what it says twice ----

/// updatePlayer's `draw`: an item chosen, in the hand or waiting for the step off a wall.
fn draw(g: &mut Game, pl: PlayerId) {
    let on_plaster = match g.pl[pl].wall {
        Some(w) => g.pl[pl].slot != crate::game::SlotKind::Rifle,
        None => false,
    };
    if on_plaster {
        g.pl[pl].wall.as_mut().unwrap().drawn = true;
        let text = format!("{} comes out when you leave the wall", item_name(g, pl));
        crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), &text);
    } else {
        g.pl[pl].holstered = false;
    }
    play(g, "equip", None, 0.5);
}

/// updatePlayer's `cursorPitch`: gun pitch onto what the reticle has, else level along the cursor's heading.
fn cursor_pitch(g: &Game, seat: &crate::seats::SeatRef, pl: PlayerId, c: CharId, ap: V3, aim_dist: f64) -> f64 {
    let mut target_y = ap[1] + 0.03;
    if let Some(t) = seat.aim_target { target_y = v3::clamp(g.tab.fixture[t].pos[1], 0.3, 2.95); }
    else if let Some(gd) = seat.aim_guard {
        target_y = man_aim(g, seat, gd, 1.2)[1];
    }
    else if level_on(g) { return 0.0; }
    let gun_y = g.ch[c].pos[1] + (if g.pl[pl].crouch { 0.95 } else { 1.4 });
    v3::clamp(m::atan2(target_y - gun_y, m::max(aim_dist, 0.5)), -60.0 * v3::DEG, 70.0 * v3::DEG)
}

/// What a round goes at: a fixture under the reticle, a man under the cursor, else level along the heading.
fn shot_target(g: &Game, seat: &crate::seats::SeatRef) -> V3 {
    if let Some(t) = seat.aim_target { return v3::copy(g.tab.fixture[t].pos); }
    if let Some(gd) = seat.aim_guard {
        return man_aim(g, seat, gd, 1.25);
    }
    free_aim(g, seat, None)
}

/// updateWorkPlant moves `W.at`, in the TypeScript the job's own array (handwork.ts samWork hands it on): the spot goes back to the job it came from.
fn work_spot_back(g: &mut Game, pl: PlayerId, w: &crate::handwork::SamWork) {
    use crate::handwork::SamWorkKind;
    let Some(at) = w.at else { return };
    match w.kind {
        SamWorkKind::Wall => if let Some(s) = g.pl[pl].wall_work_mut() { s.at = at; },
        SamWorkKind::Door => if let Some(s) = g.pl[pl].charging_mut() { s.at = at; },
        SamWorkKind::Breaker | SamWorkKind::Drive | SamWorkKind::Switch => {}
    }
}

/// One tick of the player: his keys, his legs, his aim, his guns and throws, his footfalls.
pub fn update_player(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId) {
    use crate::game::{SlotKind, ThrowKind};
    use crate::input::{input_lmb, input_lmb_hit, input_rmb, input_rmb_hit};
    use crate::weapons::{firearm_automatic, firearm_cancel_reload, firearm_reloading, RifleAttachment};
    let pl = g.player;
    let c = g.pl[pl].ch;
    let gun = g.pl[pl].pistol;
    g.pl[pl].pace = 1.0;
    if !g.pl[pl].crouch && g.pl[pl].dragging().is_none() { g.pl[pl].knee = None; }   // (up again: how he went down is over)
    g.pl[pl].dash = false; g.ch[c].anim.dash = 0.0; g.ch[c].anim.run_ready = false;
    g.tab.camera[cam].peek_offset = None;
    crate::aimweight::aim_weigh_begin(g, pl);
    crate::noshove::no_shove_begin(g, None);
    let launch_was = g.pl[pl].launch_held; g.pl[pl].launch_held = None;
    if hit(g, input, "Enter") && (crate::condition::down(g, g.pl[pl].ch) || g.mission.stage == crate::game::MissionStage::Failed || shift_down(g, input)) {
        let shift = shift_down(g, input);
        let ck = if !shift && crate::condition::down(g, g.pl[pl].ch) { let e = g.mission.stats.elapsed; crate::savestate::checkpoints_newest(g, Some(true), Some(e)) } else { None };
        if let Some(ck) = ck {
            if crate::savestate::checkpoints_is_this_run(g, &ck) {
                if crate::savestate::checkpoints_load_in_step(g, &ck.key).is_none() { return; }
            }
        }
        crate::savestate::control::ask_restart(g);
        return;
    }
    crate::stealthfeed::stealth_feed(g);
    crate::bigkit::sam_big_tail_tick(g, dt);
    crate::workstow::sam_stow_tick(g, &mut stow_holster);
    if crate::condition::down(g, g.pl[pl].ch) {
        if g.pl[pl].kit_was.is_some() || crate::character::anim_kit_busy(g, c) { crate::kitanim::kit_tick(g, None); }
        release_wall(g);
        free_held(g, crate::player::FreeHeldWhy::Shot);
        crate::deed::put_down(g, pl);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        let light = g.pl[pl].light; g.tab.light[light].enabled = false;
        g.sets[g.seat].hand.throw_preview = None;
        g.ch[c].anim.lockpick = false; g.ch[c].anim.long_gun = false;
        crate::character::character_update(g, c, dt);
        crate::game::game_body_thud(g, c);
        return;
    }
    let rifle = g.pl[pl].rifle;
    crate::weapons::firearm_update(g, gun, dt);
    crate::weapons::firearm_update(g, rifle, dt);
    crate::weapons::ocp_update(&mut g.pl[pl].ocp, dt);
    g.fa[rifle].ring_cd = m::max(0.0, g.fa[rifle].ring_cd - dt);
    g.pl[pl].equip_t = m::max(0.0, g.pl[pl].equip_t - dt);
    g.pl[pl].noise *= m::exp(-2.2 * dt);
    crate::samstun::sam_stun_pose(g);
    if g.pl[pl].holding().is_some() && crate::samstun::sam_staggered(g, &crate::seats::here(g), None) { free_held(g, crate::player::FreeHeldWhy::Shot); }
    crate::whistle::whistle_key(g, input);
    crate::gunwall::gun_wall_stance(g, c, false);
    crate::gunwall::gun_wall_pose(g, c);
    let work = crate::handwork::sam_work(g);
    let held = g.pl[pl].holding().is_some();
    let beat_has_him = g.pl[pl].planted() && work.is_none() && !held;
    if g.pl[pl].wanding().is_some() { nv_key(g, Some(input), dt); }
    else if beat_has_him { nv_key(g, None, dt); }
    if g.pl[pl].planted() || work.is_some() || crate::samstun::sam_staggered(g, &crate::seats::here(g), None) { g.pl[pl].kit_owed = -1.0; }
    if beat_has_him || held { crate::kitanim::kit_tick(g, None); }
    if g.pl[pl].takedown().is_some() {
        update_takedown(g, dt);
        g.ch[c].anim.speed = 0.0; g.ch[c].anim.upper = crate::character::UpperMode::None;
        arm_rifle(g, false, None);
        crate::character::character_update(g, c, dt);
        let light = g.pl[pl].light; g.tab.light[light].enabled = false;
        return;
    }
    if held { update_hold(g, dt, input, cam); return; }
    if g.pl[pl].planted() || crate::handwork::sam_work(g).is_some() { throw_off(g, pl); }
    if g.pl[pl].kick().is_some() { update_kick(g, dt); return; }
    if g.pl[pl].picking().is_some() { update_picking(g, dt); return; }
    if g.pl[pl].wanding().is_some() { update_wanding(g, dt); return; }
    {
        let w = crate::handwork::sam_work(g);
        let on = match w { Some(w) => !(input_lmb_hit(g, input) && crate::handwork::sam_work_let_go(g, w)), None => false };
        if w.is_some() && !on && g.pl[pl].work_stow.is_some() { crate::workstow::sam_stow_tick(g, &mut stow_holster); }
        if on {
            if crate::workstow::sam_stowing(g, &crate::seats::here(g)) { crate::kitanim::kit_tick(g, None); }
            let mut w = w.unwrap();
            update_work_plant(g, dt, &mut w);
            work_spot_back(g, pl, &w);
            return;
        }
    }
    if g.pl[pl].coding().is_some() { update_coding_plant(g, dt); return; }
    g.ch[c].anim.lockpick = false;
    // the keys: the digits are KIT_ORDER's
    let was_in = crate::game::player_gun_in_hand(g, pl);
    crate::slotsetup::slot_setup_keys(g, input);
    if hit(g, input, "Digit1") {
        g.pl[pl].slot = SlotKind::Pistol;
        draw(g, pl);
    }
    if hit(g, input, "Digit2") {
        if !g.pl[pl].has_rifle {
            // (nothing to bring up)
        } else if crate::game::player_rifle_up(g, pl) {
            if g.pl[pl].equip_t <= 0.0 {
                crate::weapons::rifle_cycle_attachment(g, rifle);
                let muzzle = g.ch[c].muzzle; play(g, "click", Some(muzzle), 0.7);
            }
        } else {
            g.pl[pl].slot = SlotKind::Rifle;
            draw(g, pl);
        }
    }
    {
        let e_key = hit(g, input, "KeyE");
        let away = !e_key && !crate::game::player_choosing(g, pl) && hit(g, input, crate::handslot::HANDSLOT_KEY);
        if (e_key || away) && g.pl[pl].tow.is_none() {
            let plain_wall = match g.pl[pl].wall {
                Some(w) => w.kind != Some(crate::player::WallHoldKind::Stack) && !crate::game::player_rifle_sel(g, pl),
                None => false,
            };
            if plain_wall {
                let to = if e_key { !g.pl[pl].wall.unwrap().drawn } else { false };
                if to != g.pl[pl].wall.unwrap().drawn {
                    g.pl[pl].wall.as_mut().unwrap().drawn = to;
                    let text = if to { format!("{} comes out when you leave the wall", item_name(g, pl)) }
                        else { format!("it'll stay {} when you leave the wall", if crate::game::player_rifle_sel(g, pl) { "slung" } else { "holstered" }) };
                    crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), &text);
                }
            } else {
                let on = if e_key { !g.pl[pl].holstered } else { true };
                set_holstered(g, on, None);
            }
        }
    }
    if hit(g, input, "Digit3") {
        if g.pl[pl].canisters > 0.0 {
            g.pl[pl].slot = SlotKind::Smoke; g.pl[pl].throw_kind = ThrowKind::Smoke;
            draw(g, pl);
        }
    }
    if hit(g, input, "Digit5") {
        if g.pl[pl].frags > 0.0 {
            g.pl[pl].slot = SlotKind::Frag;
            draw(g, pl);
        }
    }
    if hit(g, input, "Digit4") {
        if g.pl[pl].flashbangs > 0.0 {
            g.pl[pl].slot = SlotKind::Flash; g.pl[pl].throw_kind = ThrowKind::Flash;
            draw(g, pl);
        }
    }
    {
        let now_in = crate::game::player_gun_in_hand(g, pl);
        if now_in != was_in && (now_in == Some(rifle) || was_in == Some(rifle)) {
            g.pl[pl].equip_t = RIFLE_EQUIP_SECS;
            if was_in == Some(rifle) && firearm_reloading(g, rifle) { firearm_cancel_reload(g, rifle); crate::character::anim_cancel_reload(g, c); }
            if was_in == Some(gun) && firearm_reloading(g, gun) { firearm_cancel_reload(g, gun); crate::character::anim_cancel_reload(g, c); }
        }
    }
    crate::kitanim::kit_tick(g, None);
    crate::kitanim::kit_click(g, input);
    if hit(g, input, "KeyL") && crate::kit::kit_you_light(g, pl) {
        let lit = if crate::game::player_rifle_sel(g, pl) { rifle } else { gun };
        g.fa[lit].light_on = !g.fa[lit].light_on;
        let muzzle = g.ch[c].muzzle; play(g, "click", Some(muzzle), 0.6);
    }
    if hit(g, input, "KeyZ") && (crate::game::player_rifle_sel(g, pl) || (g.pl[pl].slot == SlotKind::Pistol && { let def = g.fa[gun].def; def_row(g, def).modes.len() > 1 })) {
        let sel = if crate::game::player_rifle_sel(g, pl) { rifle } else { gun };
        let def = g.fa[sel].def;
        if def_row(g, def).modes.len() > 1 {
            crate::weapons::firearm_cycle_mode(g, sel);
            let muzzle = g.ch[c].muzzle; play(g, "click", Some(muzzle), 0.7);
        }
    }
    nv_key(g, Some(input), dt);
    let keys_on = down(g, input, "KeyW") || down(g, input, "KeyA") || down(g, input, "KeyS") || down(g, input, "KeyD");
    let sprint_key = !crate::condition::is_hurt(g, g.pl[pl].ch, None) && shift_down(g, input) && keys_on;
    if hit(g, input, "KeyR") || (crate::game::player_pistol_up(g, pl) && crate::autoreload::auto_reload_due(g, pl, gun)) || (crate::game::player_rifle_up(g, pl) && crate::autoreload::auto_reload_due(g, pl, rifle)) {
        let sprinting = sprint_key && !wall_ran_on(g, pl, input) && !(g.pl[pl].crouch);
        reload_in_hand(g, Some(sprinting), None);
    }
    // movement input: none while the selector is open
    let basis = crate::camera::follow_camera_planar_basis(g, cam);
    let (fwd, right) = (basis.fwd, basis.right);
    let (mut mx, mut mz) = (0.0, 0.0);
    let (w, s, d, a) = (down(g, input, "KeyW"), down(g, input, "KeyS"), down(g, input, "KeyD"), down(g, input, "KeyA"));
    let way = keys_way(fwd, right, w, s, d, a);
    mx = way[0]; mz = way[1];
    let stalk = {
        let (lmb, lmb_hit) = (input_lmb(g, input), input_lmb_hit(g, input));
        crate::stalk::stalk_step(g, lmb, lmb_hit, mx != 0.0 || mz != 0.0, Some(crate::stalk::StalkKeys { mx, mz, dt }))
    };
    if let Some(st) = stalk { mx = st.mx; mz = st.mz; g.pl[pl].crouch = true; g.pl[pl].knee = None; }
    if g.pl[pl].tow.is_none() { g.ch[c].anim.haul = None; }
    let stunned = crate::samstun::sam_staggered(g, &crate::seats::here(g), None);
    let rooted = crate::samstun::sam_rooted(g, &crate::seats::here(g), None);
    let own_move = !rooted && (mx != 0.0 || mz != 0.0);
    {
        let own = own_move;
        if let Some(hs) = crate::hands::hands_step(g, own, dt) { mx = hs[0]; mz = hs[1]; }
    }
    if rooted { mx = 0.0; mz = 0.0; }
    let ml = m::hyp2(mx, mz);
    if ml > 0.0 { mx /= ml; mz /= ml; }
    if hit(g, input, "KeyC") && g.pl[pl].dragging().is_none() { own_legs(g, pl, |g, m| crouch_key(g, m)); }
    let want_sprint = !crate::condition::is_hurt(g, g.pl[pl].ch, None) && shift_down(g, input)
        && (if g.pl[pl].haul().is_some() || g.pl[pl].door_move().is_some() { own_move } else { ml > 0.0 && stalk.is_none() });
    // back to the wall (Q)
    if rooted { /* folded: no reaching for a wall, no pushing off one */ }
    else if hit(g, input, "KeyQ") {
        if g.pl[pl].wall.is_some() { wall_step_off(g, mx, mz, ml, crate::player::WallStepOffHow::Q); }
        else if !want_sprint {
            let mut why = crate::player::WallSnapWhy { blocked: false };
            if !wall_snap(g, cam, Some(&mut why), None) && !wall_snap_later(g, ml, crate::player::WallSnapLaterWhy { blocked: why.blocked }) {
                let text = if g.pl[pl].dragging().is_some() { "your hands are full" } else if why.blocked { "can't get to that wall" } else { "no wall to take here" };
                crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), text);
            }
        }
    }
    else { wall_snap_pending(g, dt, cam, want_sprint); }
    if g.k.wallsnap.press && !rooted && stalk.is_none() && !crate::rails::rails_locks_wall(g) { wall_press(g, dt, input, cam, mx, mz, ml, want_sprint); }
    let ran_on = wall_ran_on(g, pl, input);
    if g.pl[pl].wall.is_some() && want_sprint && !ran_on { wall_step_off(g, mx, mz, ml, crate::player::WallStepOffHow::Run); }
    let on_wall = g.pl[pl].wall.is_some() && move_on_wall(g, dt, cam, mx, mz, ml);
    if !on_wall {
        if want_sprint && g.pl[pl].dragging().is_some() { crate::game::game_toggle_drag(g, None); }
        own_legs(g, pl, |g, m| sprint_stance(g, m, want_sprint, stalk.is_some()));
        if g.pl[pl].sprinting && firearm_reloading(g, gun) { firearm_cancel_reload(g, gun); crate::character::anim_cancel_reload(g, c); }
        if g.pl[pl].sprinting && firearm_reloading(g, rifle) { firearm_cancel_reload(g, rifle); crate::character::anim_cancel_reload(g, c); }
        let stalk_row = stalk.map(|s| crate::player::StrideStepStalk { pace: s.pace, keep: s.keep });
        // (the stride puts a NEW array in his `pos`'s place, player.ts:448 `c.pos = v3.mad(…)`, on the lent legs: said here, where the man is known)
        { let c = g.pl[pl].ch; crate::character::pos_replaced(g, c); }
        own_legs(g, pl, |g, m| stride_step(g, m, mx, mz, ml, stalk_row, dt, None));
    }
    let speed = m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]);
    // aim
    let ap = match stalk { Some(s) => s.at, None => g.sets[g.seat].aim_point };
    let to_aim: V3 = [ap[0] - g.ch[c].pos[0], 0.0, ap[2] - g.ch[c].pos[2]];
    let aim_dist = m::hyp2(to_aim[0], to_aim[2]);
    g.sets[g.seat].hand.aim_held = false;
    let gun_up = crate::game::player_pistol_up(g, pl);
    let rifle_up = crate::game::player_rifle_up(g, pl);
    let aiming = gun_raised(g, pl, None, None) && !g.pl[pl].sprinting && g.pl[pl].dragging().is_none() && !crate::hands::door_move_lowers(g, &crate::seats::here(g))
        && !(g.pl[pl].wall.map_or(false, |w| w.kind == Some(crate::player::WallHoldKind::Stack)) && !stack_up(g, dt, input));
    if let Some(mut tow) = g.pl[pl].tow {
        // dragging: the ragdoll's ankles pinned at his hands, facing its hips; torn out of his hands if it wedges
        let body = g.gd[tow.who].ch;
        let rag = g.ch[body].ragdoll;
        let has_rag = rag.is_some();
        let hips = match g.ch[body].bones.get(crate::character::Bone::Hips) { Some(h) => *h, None => g.ch[body].pos };
        let (dx, dz) = (hips[0] - g.ch[c].pos[0], hips[2] - g.ch[c].pos[2]);
        let d = m::hyp2(dx, dz);
        if d > 1e-4 { tow.yaw = m::atan2(dx, dz); }
        let strained = match rag { Some(r) => g.tab.ragdoll[r].strain > 0.2 || crate::hands::haul_slack(g), None => false };
        tow.stuck_t = if strained { tow.stuck_t + dt } else { 0.0 };
        g.pl[pl].tow = Some(tow);
        if !has_rag || d > 2.2 || tow.stuck_t > 0.25 {
            crate::game::game_toggle_drag(g, None);
        }
        else if crate::hands::haul_set_done(g) { release_body(g, Some(true)); }
        else {
            let f = crate::character::character_forward(g, c);
            let hx = g.ch[c].pos[0] + f[0] * 0.35;
            let hz = g.ch[c].pos[2] + f[2] * 0.35;
            let rag = rag.unwrap();
            if !crate::hands::haul_pins(g, dt) {
                let bones = crate::ragdoll::rag();
                crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_l, [hx + f[2] * 0.1, 0.42, hz - f[0] * 0.1]);
                crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_r, [hx - f[2] * 0.1, 0.42, hz + f[0] * 0.1]);
            }
            if crate::hands::haul_upright(g, &crate::seats::here(g)) {
                if speed > 0.2 { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, v3::yaw_of(g.ch[c].vel), g.k.hands.carry_turn * dt); }
                g.ch[c].aim_yaw = g.ch[c].body_yaw;
                g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 6.0, dt);
                g.pl[pl].backpedal = false; g.ch[c].anim.reverse = false;
            } else {
                let face_yaw = match crate::hands::haul_yaw(g, &crate::seats::here(g)) { Some(y) => y, None => tow.yaw };
                let hauling = g.pl[pl].haul().is_some();
                g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, if hauling { -0.45 } else { -0.9 }, 6.0, dt);
                if hauling {
                    let rel = v3::approach_angle(v3::wrap_angle(g.ch[c].aim_yaw - g.ch[c].body_yaw), 0.0, 8.0 * dt);
                    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face_yaw, 10.0 * dt);
                    g.ch[c].aim_yaw = g.ch[c].body_yaw + rel;
                } else {
                    g.ch[c].aim_yaw = face_yaw;
                    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face_yaw, 10.0 * dt);
                }
                g.pl[pl].backpedal = speed > 0.2 && m::cos(v3::wrap_angle(face_yaw - v3::yaw_of(g.ch[c].vel))) < 0.0;
                g.ch[c].anim.reverse = g.pl[pl].backpedal;
            }
            // a soft periodic scrape while the body is actually sliding
            let sleeping = g.tab.ragdoll[rag].sleeping;
            let carried = g.pl[pl].haul().map_or(false, |h| h.carry.is_some());
            if speed > 0.3 && !sleeping && !carried {
                let scrapes = match g.pl[pl].tow.as_mut() {
                    Some(t) => { t.noise_t -= dt; t.noise_t <= 0.0 }
                    None => false,
                };
                if scrapes {
                    if let Some(t) = g.pl[pl].tow.as_mut() { t.noise_t = 0.55; }
                    crate::audio_iface::game_audio_like_play(g, "propScrape", Some(hips), Some(0.25), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
                    let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
                    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: [hips[0], 0.0, hips[2]], time, loud: false, level: Some(0.22), who: Some(who), serial, ..Default::default() });
                    g.pl[pl].noise = m::max(g.pl[pl].noise, 0.22);
                }
            }
        }
    } else if g.pl[pl].sprinting {
        // sprint: gun down, body follows the run direction, no aiming
        let lean = own_legs(g, pl, |g, m| sprint_facing(g, m, dt, None));
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.3, 6.0, dt);
        if lean.is_some() {
            let head = crate::sprintdoor::sprint_door_head(g.ch[c].body_yaw - v3::yaw_of(g.ch[c].vel));
            g.ch[c].anim.look_yaw_extra = head; g.pl[pl].door_head = head;
            g.pl[pl].door_head_t = g.time;
        }
        g.ch[c].anim.reverse = false;
    } else if g.pl[pl].wall.map_or(false, |w| w.kind == Some(crate::player::WallHoldKind::Stack)) {
        // stacked on the wall with the rifle: he faces along it, turning up to 45° off that line to bring the rifle onto the cursor
        let w = g.pl[pl].wall.unwrap();
        let along = m::atan2(w.n[0], w.n[2]) - w.side * m::PI / 2.0;
        let moving = m::abs(w.v) > 0.3;
        let mut body_t = along;
        if aiming && aim_dist > 0.25 {
            g.ch[c].aim_yaw = aim_ease(g.ch[c].aim_yaw, to_aim, dt);
            let pitch = cursor_pitch(g, &crate::seats::here(g), pl, c, ap, aim_dist);
            g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, pitch, 12.0, dt);
            let off = v3::wrap_angle(v3::yaw_of(to_aim) - along);
            let mut flip_t = if m::abs(off) > 110.0 * v3::DEG && !moving && !w.lean.map_or(false, |l| l > 0.0) { w.flip_t.unwrap_or(0.0) + dt } else { 0.0 };
            let mut side = w.side;
            if flip_t > 0.2 { side = -side; flip_t = 0.0; }
            let held = g.pl[pl].wall.as_mut().unwrap();
            held.flip_t = Some(flip_t); held.side = side;
            body_t = along + v3::clamp(v3::wrap_angle(g.ch[c].aim_yaw - along), -0.9, 0.9) * 0.85;
        } else {
            g.ch[c].aim_yaw += v3::wrap_angle(along - g.ch[c].aim_yaw) * (1.0 - m::exp(-10.0 * dt));
            g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.05, 6.0, dt);
            g.pl[pl].wall.as_mut().unwrap().flip_t = Some(0.0);
        }
        if !wall_take_yaw(g, pl) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, body_t, 10.0 * dt); }
        let side = g.pl[pl].wall.unwrap().side;
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = moving && w.v * side < 0.0;
    } else if let Some(w) = g.pl[pl].wall {
        // back to the wall: flat against it when parked, turned toward the way he is going while he sidles
        let away_yaw = m::atan2(w.n[0], w.n[2]);
        let moving = m::abs(w.v) > 0.3;
        let pace = if g.pl[pl].crouch { g.tune.player_crouch } else { crate::player::WALL_PACE };
        let turn_k = v3::smoothstep(0.1, 0.9, m::abs(w.v) / m::max(0.3, pace));
        if !wall_take_yaw(g, pl) { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, away_yaw - w.side * crate::player::WALL_TURN * turn_k, 10.0 * dt); }
        let look = away_yaw - w.side * (if w.at_edge == w.side { crate::player::WALL_LOOK } else if moving { 15.0 * v3::DEG } else { 8.0 * v3::DEG });
        g.ch[c].aim_yaw += v3::wrap_angle(look - g.ch[c].aim_yaw) * (1.0 - m::exp(-8.0 * dt));
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.05, 6.0, dt);
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = moving && w.v * w.side < 0.0;
    } else {
        if !(stalk.is_some()) && crate::aimweight::aim_weigh_turn(g, dt) { /* the gun's heading swings to the cursor's bearing at a limited rate */ }
        else if aim_dist > 0.25 { g.ch[c].aim_yaw = aim_ease(g.ch[c].aim_yaw, to_aim, dt); }
        let pitch = cursor_pitch(g, &crate::seats::here(g), pl, c, ap, aim_dist);
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, pitch, 12.0, dt);
        // a leaf he is walking open is taken square on unless the gun is actually up
        let dp = g.ch[c].anim.door_push.clone();
        let by_hand = g.pl[pl].door_move().is_some() && dp.as_ref().map_or(false, |d| d.w >= 1.0) && !aiming;
        let into_door = if by_hand {
            let d = dp.as_ref().unwrap();
            Some(m::atan2(d.p[0] - g.ch[c].pos[0], d.p[2] - g.ch[c].pos[2]))
        } else {
            let leaf = dp.as_ref().map(|d| crate::player::LeafFacingDp { w: d.w, n: d.n.to_vec() });
            leaf_facing(leaf.as_ref(), aiming)
        };
        own_legs(g, pl, |g, m| hips_turn(m, speed, aiming, stalk.is_some(), into_door, by_hand, dt));
        g.ch[c].anim.reverse = g.pl[pl].backpedal;
    }
    g.ch[c].anim.speed = speed;
    g.ch[c].anim.crouch_target = if g.pl[pl].crouch { 1.0 } else { 0.0 };
    g.ch[c].anim.dash = if g.pl[pl].dash { 1.0 } else { 0.0 };
    if let Some(w) = g.pl[pl].wall {
        g.ch[c].anim.stance = crate::character::Stance::Wall;
        g.ch[c].anim.stack_side = w.side;
        g.ch[c].anim.wall_peek = w.peek;
        g.ch[c].anim.wall_yaw = v3::wrap_angle(m::atan2(w.n[0], w.n[2]) - g.ch[c].body_yaw);
    }
    {
        use crate::character::UpperMode;
        g.ch[c].anim.upper = if g.pl[pl].sprinting || g.pl[pl].dragging().is_some() { UpperMode::None } else if aiming { UpperMode::Aim } else if gun_up { UpperMode::Relaxed } else { UpperMode::None };
    }
    g.ch[c].anim.run_ready = g.pl[pl].sprinting && gun_up && !g.ch[c].anim.hide_held_item;
    {
        use crate::character::LongStance;
        let leaf_on_him = g.ch[c].anim.door_push.as_ref().map_or(false, |d| d.w >= 1.0);
        let in_hands = rifle_up && !g.ch[c].anim.hide_held_item;
        let carriage = if !in_hands { None } else if g.pl[pl].sprinting { Some(LongStance::LowLong) } else if leaf_on_him && !aiming { Some(LongStance::PortLong) } else { Some(LongStance::Shoulder) };
        arm_rifle(g, in_hands, carriage);
    }
    // a wall, a jamb or a leaf inside the raised gun's reach: the gun comes to the low ready and no round leaves it
    let rifle_aim = rifle_up && g.ch[c].anim.long_stance == Some(crate::character::LongStance::Shoulder);
    let lock_at = if rifle_aim { let def = g.fa[rifle].def; match def_row(g, def).pellets.as_ref().and_then(|p| p.breach.clone()) { Some(b) => crate::gunwall::breach_aim(g, c, b), None => None } } else { None };
    let friend_up = (rifle_aim || gun_up) && !g.pl[pl].sprinting && lock_at.is_none();
    let off_friend = crate::friends::sam_off_friend(g, c, friend_up, dt);
    let off_friend_on = !matches!(off_friend, Some(crate::friends::SamOffFriend::Bool(false)));
    let walled = (stunned && (rifle_aim || gun_up))
        || (g.pl[pl].wall.is_none() && crate::gunwall::gun_walled(g, c, if rifle_aim || gun_up { Some(crate::gunwall::GunUp::Aim) } else { None }, lock_at, dt, Some(false), Some(lock_at.is_some())))
        || off_friend_on;
    if walled {
        g.ch[c].anim.upper = crate::character::UpperMode::None;
        if rifle_aim { arm_rifle(g, true, Some(crate::character::LongStance::LowLong)); }
        if input_lmb(g, input) {   // (held, not only pressed: the word stays up while the trigger is down)
            let text = if stunned { String::from("you're still stunned") }
                else if off_friend_on {
                    match off_friend { Some(crate::friends::SamOffFriend::Guard(f)) => format!("{} is in your line of fire", g.gd[f].callsign), _ => String::from("a friend is in your line of fire") }
                }
                else { String::from("no room to raise your gun here") };
            crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), &text);
        }
    }
    crate::gunwall::gun_wall_stance(g, c, walled && !rifle_aim);
    // slot actions
    if g.pl[pl].wall.is_some() { wall_snap_pose(g, dt, aiming); }
    g.arms[g.pl[pl].ch].fire_cd -= dt;
    let gun_def = g.fa[gun].def;
    if def_auto(g, gun_def).is_some() { let trigger = gun_up && input_lmb(g, input); crate::weapons::firearm_auto_idle(g, gun, dt, trigger); }
    if gun_up && !g.pl[pl].sprinting && g.pl[pl].dragging().is_none() {
        let auto = firearm_automatic(g, gun);
        let pull = if auto { input_lmb(g, input) || input_lmb_hit(g, input) } else { input_lmb_hit(g, input) };
        if pull && !walled && g.arms[g.pl[pl].ch].fire_cd <= (if auto { 1e-4 } else { 0.0 }) && g.pl[pl].equip_t <= 0.0 {
            let grow = crate::weapons::firearm_auto_spread(g, gun);
            if crate::weapons::firearm_fire(g, gun) {
                g.arms[g.pl[pl].ch].fire_cd = if firearm_automatic(g, gun) { def_auto(g, gun_def).unwrap() } else { def_row(g, gun_def).cd };
                g.pl[pl].last_fire_t = g.time;
                if def_auto(g, gun_def).is_some() { crate::weapons::firearm_auto_fired(g, gun); }
                let tgt = shot_target(g, &crate::seats::here(g));
                let at = crate::aimweight::aim_weigh_shot(g, tgt);
                let spread = grow + crate::suppression::sam_spread(g, &crate::seats::here(g)) + crate::aim::sam_aim(g);
                crate::combat::fire_weapon(g, c, at, Some(&crate::combat::FireOpts { spread: Some(spread), ..Default::default() }));
            } else if input_lmb_hit(g, input) {
                g.arms[g.pl[pl].ch].fire_cd = 0.25;
                let muzzle = g.ch[c].muzzle; play(g, "dryFire", Some(muzzle), 0.7);
            }
        }
        // right click with the pistol out: the OCP zaps the light the reticle has snapped to
        if input_rmb_hit(g, input) && !(g.pl[pl].equip_t > 0.0) && g.ch[c].anim.big_tool.is_none() && (crate::lighting::ocpable(g, &crate::seats::here(g)) || (g.sets[g.seat].aim_target.is_none() && g.sets[g.seat].aim_guard.is_none())) {
            let t = g.sets[g.seat].aim_target;
            crate::game::game_ocp(g, t);
        }
    }
    // the rifle in the hands: the left button a round, the right button the launcher's load
    let mut ring_preview: Option<crate::throwables::ThrowSolution> = None;
    { let trigger = rifle_up && input_lmb(g, input); crate::weapons::firearm_auto_idle(g, rifle, dt, trigger); }
    if rifle_up && !g.pl[pl].sprinting {
        let pull = if firearm_automatic(g, rifle) { input_lmb(g, input) || input_lmb_hit(g, input) } else { input_lmb_hit(g, input) };
        if pull && !walled && g.arms[g.pl[pl].ch].fire_cd <= 1e-4 && g.pl[pl].equip_t <= 0.0 {
            let grow = crate::weapons::firearm_auto_spread(g, rifle);
            let broke = if input_lmb_hit(g, input) { shell_break(g) } else { None };
            let rifle_def = g.fa[rifle].def;
            if broke == Some(crate::player::ShellBreak::Bolt) { g.arms[g.pl[pl].ch].fire_cd = 0.1; }
            else if crate::weapons::firearm_fire(g, rifle) {
                g.arms[g.pl[pl].ch].fire_cd = if firearm_automatic(g, rifle) { def_auto(g, rifle_def).unwrap() } else { def_row(g, rifle_def).cd };
                g.pl[pl].last_fire_t = g.time;
                crate::weapons::firearm_auto_fired(g, rifle);
                let tgt = shot_target(g, &crate::seats::here(g));
                let at = crate::aimweight::aim_weigh_shot(g, tgt);
                let spread = rifle_spread(g, &crate::seats::here(g), speed) + grow + crate::suppression::sam_spread(g, &crate::seats::here(g)) + crate::aim::sam_aim(g);
                crate::combat::fire_weapon(g, c, at, Some(&crate::combat::FireOpts { weapon: Some(def_row(g, rifle_def).clone()), spread: Some(spread), ..Default::default() }));
            } else if input_lmb_hit(g, input) {
                g.arms[g.pl[pl].ch].fire_cd = 0.25;
                let muzzle = g.ch[c].muzzle; play(g, "dryFire", Some(muzzle), 0.7);
            }
        }
        // the launcher: the press begins the gesture, the arc is drawn while the button stays down, letting go sends the round
        let beat = g.pl[pl].equip_t > 0.0 && g.pl[pl].wall.is_some();
        let load = crate::weapons::rifle_attachment(g, rifle);
        let owed = walled && !stunned && launch_was.is_some() && crate::aimweight::aim_weigh_launch_keep(g, pl, dt);
        let mut held = None;
        if let Some(l) = load {
            if launch_was.is_some() && !beat && (!walled || owed) && !stunned && g.sets[g.seat].orders.menu.is_none() && crate::weapons::rifle_left(g, rifle, l) > 0.0 {
                if !crate::backkey::back_hit(g, input) && !hit(g, input, "Blur") && !hit(g, input, "Paused") { held = Some(l); }
            }
        }
        let is_down = input_rmb(g, input) || input_rmb_hit(g, input);
        if held.is_none() && input_rmb_hit(g, input) && !beat && !walled && !stunned {
            match load {
                None => {}
                Some(l) if crate::weapons::rifle_left(g, rifle, l) > 0.0 && g.fa[rifle].ring_cd <= 0.0 => held = Some(l),
                Some(_) => {
                    let muzzle = g.ch[c].muzzle; play(g, "dryFire", Some(muzzle), 0.4);
                }
            }
        }
        if let (Some(h), false) = (held, owed) {
            let from = crate::rig_props::sc20k_launcher_muzzle(g, c, None);
            let point = match h {
                RifleAttachment::Ring => crate::launcher::ring_aim_point(g, &crate::seats::here(g), from),
                RifleAttachment::Cam => crate::stickycam::cam_aim_point(g, &crate::seats::here(g), from),
                RifleAttachment::He40 => crate::he40::shell_aim_point(g, &crate::seats::here(g), from),
            };
            let at = crate::aimweight::aim_weigh_shot(g, point);
            ring_preview = Some(match h {
                RifleAttachment::Ring => crate::launcher::ring_solve(g, from, at),
                RifleAttachment::Cam => crate::stickycam::cam_solve(g, from, at),
                RifleAttachment::He40 => { let bodies = crate::he40::shell_bodies(g); crate::he40::shell_solve(g, from, at, Some(&bodies)) }
            });
            let up = !is_down && g.pl[pl].equip_t <= 0.0;
            if crate::aimweight::aim_weigh_launch(g, dt, up) {
                match h {
                    RifleAttachment::Ring => {
                        crate::launcher::fire_ring(g, c, from, at);
                        if !g.infinite_ammo { g.fa[rifle].rings -= 1.0; }
                        g.fa[rifle].ring_cd = g.k.launcher.cd;
                        g.arms[g.pl[pl].ch].fire_cd = m::max(g.arms[g.pl[pl].ch].fire_cd, 0.3);
                    }
                    RifleAttachment::Cam => {
                        crate::stickycam::fire_cam(g, pl, from, at);
                        let mut data = JsMap::new();
                        data.set(String::from("kind"), Some(crate::chronicle::ChronicleDataValue::Text(String::from("cam"))));
                        crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Thrown, Some(&data), None);
                        if !g.infinite_ammo { g.fa[rifle].cams -= 1.0; }
                        g.fa[rifle].ring_cd = g.k.sticky.cd;
                        g.arms[g.pl[pl].ch].fire_cd = m::max(g.arms[g.pl[pl].ch].fire_cd, 0.3);
                    }
                    RifleAttachment::He40 => {
                        crate::he40::fire_shell(g, c, from, at);
                        crate::chronicle::chron_thrown(g, "he40");
                        if !g.infinite_ammo { g.fa[rifle].shells -= 1.0; }
                        g.fa[rifle].ring_cd = g.k.launcher40.cd;
                        g.arms[g.pl[pl].ch].fire_cd = m::max(g.arms[g.pl[pl].ch].fire_cd, 0.4);
                    }
                }
                g.pl[pl].last_fire_t = g.time;
                crate::character::anim_fire(g, c);
                held = None;
                ring_preview = None;
            }
        }
        g.pl[pl].launch_held = held;
    }
    // throwables: preview while a canister slot is up; throw on LMB (QUICKTHROW is off on the page: no G gesture)
    let prev_preview = g.sets[g.seat].hand.throw_preview.take();
    {
        let tk = g.pl[pl].throw_kind;
        let other = if tk == ThrowKind::Flash { ThrowKind::Smoke } else { ThrowKind::Flash };
        if crate::game::game_throw_count(g, &crate::seats::here(g), tk) <= 0.0 && crate::game::game_throw_count(g, &crate::seats::here(g), other) > 0.0 { g.pl[pl].throw_kind = other; }
    }
    let slot_kind: Option<ThrowKind> = if g.pl[pl].holstered { None } else { crate::game::player_canister_sel(g, pl) };
    let (throw_held, held_kind, throw_kind) = (g.sets[g.seat].hand.throw_held, g.pl[pl].held_kind, g.pl[pl].throw_kind);
    let kind: ThrowKind = if slot_kind == Some(ThrowKind::Frag) && throw_held {
        if held_kind != ThrowKind::Frag { held_kind } else if throw_kind == ThrowKind::Frag { ThrowKind::Smoke } else { throw_kind }
    } else {
        match slot_kind { Some(k) => k, None => if throw_held { held_kind } else { throw_kind } }
    };
    g.pl[pl].held_kind = kind;
    if !g.pl[pl].sprinting && crate::game::game_throw_count(g, &crate::seats::here(g), kind) > 0.0 && slot_kind.is_some() {
        let tp = ap;
        let origin = throw_origin(g, &crate::seats::here(g));
        let sol = crate::throwables::throwables_solve(g, origin, [tp[0], m::max(0.05, m::min(tp[1], 2.6)), tp[2]], None);
        g.sets[g.seat].hand.throw_preview = Some(sol.clone());
        let hold_ok = match g.ch[c].anim.kit_hold {
            None => true,
            Some(Some(crate::character::KitItem::Can)) => g.pl[pl].kit_can == slot_kind,
            Some(_) => false,
        };
        if input_lmb_hit(g, input) && g.arms[g.pl[pl].ch].fire_cd <= 0.0 && hold_ok { do_throw(g, &sol, kind); g.arms[g.pl[pl].ch].fire_cd = 0.6; }
    }
    if g.sets[g.seat].hand.throw_held && !g.pl[pl].sprinting && !stunned {
        let sol = match g.sets[g.seat].hand.throw_preview.clone() { Some(s) => Some(s), None => prev_preview };
        if let Some(sol) = sol { do_throw(g, &sol, kind); }
    }
    g.sets[g.seat].hand.throw_held = false;
    if g.sets[g.seat].hand.throw_preview.is_none() && ring_preview.is_some() && !g.sets[g.seat].hand.throw_held { g.sets[g.seat].hand.throw_preview = ring_preview; }
    if let Some(p) = g.pl[pl].pending_throw.as_mut() { p.t -= dt; }
    if g.pl[pl].pending_throw.as_ref().map_or(false, |p| p.t <= 0.0) {
        let (sol, k2) = { let p = g.pl[pl].pending_throw.as_ref().unwrap(); (p.sol.clone(), p.kind) };
        let origin = throw_origin(g, &crate::seats::here(g));
        let who = g.ch[c].id;
        let (item_kind, fuse) = match k2 {
            ThrowKind::Flash => (crate::throwables::ItemKind::Flash, Some(crate::combat::flash_fuse(g))),
            ThrowKind::Frag => (crate::throwables::ItemKind::Frag, Some(g.k.frag.fuse)),
            ThrowKind::Smoke => (crate::throwables::ItemKind::Smoke, None),
        };
        let it = crate::throwables::throwables_spawn(g, item_kind, origin, sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse, who: Some(who), ..Default::default() }));
        g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::CanHeard);
        crate::chronicle::chron_thrown(g, k2.as_str());
        g.pl[pl].pending_throw = None;
        if g.infinite_ammo { can_back(g, pl, k2); }
        let slot = g.pl[pl].slot;
        if (slot == SlotKind::Smoke && g.pl[pl].canisters == 0.0) || (slot == SlotKind::Flash && g.pl[pl].flashbangs == 0.0) || (slot == SlotKind::Frag && g.pl[pl].frags == 0.0) { g.pl[pl].slot = SlotKind::Pistol; }
    }
    crate::kitanim::kit_tick(g, Some(true));
    crate::gunwall::gun_wall_pose(g, c);
    crate::character::character_update(g, c, dt);
    // the weapon light: its position and direction are the gun's
    let infinite = g.infinite_ammo;
    g.fa[gun].infinite = infinite; g.fa[rifle].infinite = infinite;
    let (light, power, radius) = (g.pl[pl].light, g.tune.player_light, crate::guards::FLASHLIGHT_RADIUS * 0.7);
    { let l = &mut g.tab.light[light]; l.peak_intensity = power; l.intensity = power; l.radius = radius; }
    pose_gun_light(g);
    // footsteps, for audio and the AI's hearing
    let phase = g.ch[c].anim.phase;
    let tread_speed = speed;
    let fall = footfall_due(g, dt, phase, tread_speed);
    if js::truthy(fall) {
        let base = if g.pl[pl].sprinting { 1.0 } else if g.pl[pl].dash { g.k.crouchrun.loud } else if g.pl[pl].crouch { 0.125 }
            else if crate::kitload::load_gait(g, g.pl[g.player].ch, speed) > 2.0 { 0.6 } else { 0.26 };
        let level = if g.pl[pl].sprinting { base } else { base * crate::kitload::load_heard(g, g.pl[g.player].ch) };
        let pos = g.ch[c].pos;
        let loud = crate::floor::tread(g, pos, level, true);
        let (time, serial) = (g.time, g.serial());
        g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos: v3::copy(pos), time, loud: false, level: Some(loud), serial, ..Default::default() });
        g.pl[pl].noise = m::max(g.pl[pl].noise, loud);
        if fall == 1.0 { let left = g.pl[pl].step_phase < 0.5; crate::wet::wet_footfall(g, left); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legs(body: f64, aim: f64, vx: f64, vz: f64, backpedal: bool) -> crate::player::Legs {
        let mut m = crate::player::Legs::default();
        m.char.vel = [vx, 0.0, vz]; m.char.body_yaw = body; m.char.aim_yaw = aim; m.backpedal = backpedal;
        m
    }

    const LONG: f64 = 1.0;   // a frame long enough that his hips get where they are going

    /// Walking on his own, hips go the way he walks and lean toward the aim, further with the gun up.
    #[test]
    fn walking_his_hips_follow_his_way_and_lean_to_his_aim() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let at = |aim: f64, aiming: bool| { let mut m = legs(0.0, aim, 0.0, 1.5, false); hips_turn(&mut m, 1.5, aiming, false, None, false, LONG); m.char.body_yaw };
        near(at(0.5, false), 0.3);
        near(at(0.5, true), 0.5);
        near(at(1.5, true), 0.8);     // no further round than the waist goes
        let mut m = legs(0.0, 0.5, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, true, false, None, false, 0.01);
        near(m.char.body_yaw, 0.09);  // and at a rate, not at once
    }

    /// A failure means he flickers between walking forward and backing up as the cursor crosses his flank: it takes a clear look behind to start backing up, and a clear look ahead to stop.
    #[test]
    fn he_backs_up_with_his_aim_behind_him_and_does_not_flicker() {
        let after = |aim: f64, back: bool| { let mut m = legs(0.0, aim, 0.0, 1.5, back); hips_turn(&mut m, 1.5, true, false, None, false, LONG); (m.backpedal, m.char.body_yaw) };
        let (back, body) = after(m::PI, false);
        assert!(back && (body.abs() - m::PI).abs() < 1e-9, "aim straight behind: back up facing it ({body})");
        assert!(!after(1.7, false).0, "a little behind the shoulder: still walking forward");
        assert!(after(1.7, true).0, "...and still backing up");
        assert!(!after(1.0, true).0, "a clear look ahead ends it");
    }

    /// A failure means he turns away from the leaf he works: walked open it pulls his hips a little and he never backs up; by hand he faces it, and backs up if he walks away from it.
    #[test]
    fn a_leaf_turns_his_hips_more_than_his_aim_does() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let mut m = legs(0.0, 2.9, 0.0, 1.5, true);
        hips_turn(&mut m, 1.5, false, true, Some(1.0), false, LONG);
        assert!(!m.backpedal);
        near(m.char.body_yaw, 0.35);
        let mut m = legs(0.0, 2.9, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, false, false, Some(1.3), true, LONG);
        assert!(!m.backpedal);
        near(m.char.body_yaw, 1.3);
        let mut m = legs(0.0, 2.9, 0.0, 1.5, false);
        hips_turn(&mut m, 1.5, false, false, Some(3.0), true, LONG);
        assert!(m.backpedal);
    }

    /// Standing, his hips stay until the aim has gone past what the waist takes, less with the gun up; then they come round short of it.
    #[test]
    fn standing_his_hips_stay_until_the_aim_passes_the_waist() {
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        let at = |aim: f64, aiming: bool, into: Option<f64>, hand: bool| { let mut m = legs(0.0, aim, 0.0, 0.0, true); hips_turn(&mut m, 0.0, aiming, false, into, hand, LONG); assert!(!m.backpedal); m.char.body_yaw };
        assert_eq!(at(1.0, false, None, false), 0.0);
        near(at(1.0, true, None, false), 1.0 - 0.6 * 0.7);
        near(at(2.0, false, None, false), 2.0 - 1.2 * 0.7);
        assert_eq!(at(0.0, false, Some(0.2), false), 0.0);
        near(at(0.0, false, Some(0.4), false), 0.4 - 0.25 * 0.7);
        near(at(0.0, false, Some(0.4), true), 0.4 - 0.1 * 0.7);
    }

    #[test]
    fn his_keys_add_up_and_opposite_keys_cancel() {
        let (fwd, right) = ([0.5, 0.0, 0.75], [0.75, 0.0, -0.5]);
        assert_eq!(keys_way(fwd, right, true, false, true, false), [1.25, 0.25]);
        assert_eq!(keys_way(fwd, right, false, true, false, true), [-1.25, -0.25]);
        assert_eq!(keys_way(fwd, right, true, true, true, true), [0.0, 0.0]);
        assert_eq!(keys_way(fwd, right, false, false, false, false), [0.0, 0.0]);
    }

    #[test]
    fn a_leaf_turns_him_square_into_it_only_when_it_pushes() {
        let dp = |w| crate::player::LeafFacingDp { w, n: vec![1.0, 0.0, 0.0] };
        assert!((leaf_facing(Some(&dp(1.0)), false).unwrap() + m::PI / 2.0).abs() < 1e-12, "a yaw of minus a quarter faces -x, into a face whose normal is +x");
        assert_eq!(leaf_facing(Some(&dp(0.5)), false), None);    // it merely rests on him
        assert_eq!(leaf_facing(Some(&dp(1.0)), true), None);     // the gun is up
        assert_eq!(leaf_facing(None, false), None);
    }
}
