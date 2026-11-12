//! ONE TICK OF A PLAYER, as the list of what is done in it. Each stage is below, or in the file its line names.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// Whose tick it is and what it is given: the same from its first stage to its last.
#[derive(Clone, Copy)]
pub(super) struct Tick { pub pl: PlayerId, pub c: CharId, pub gun: FirearmId, pub rifle: FirearmId, pub input: InputId, pub cam: CameraId, pub dt: f64 }
impl Tick {
    fn of(g: &Game, dt: f64, input: InputId, cam: CameraId) -> Tick {
        let pl = g.player;
        let p = &g.pl[pl];
        Tick { pl, c: p.ch, gun: p.pistol, rifle: p.rifle, input, cam, dt }
    }
    pub fn hit(&self, g: &Game, key: crate::input::Key) -> bool { crate::input::input_hit(g, self.input, key) }
    pub fn down(&self, g: &Game, key: crate::input::Key) -> bool { crate::input::input_down(g, self.input, key) }
    pub fn shift(&self, g: &Game) -> bool { self.down(g, crate::input::Key::ShiftLeft) || self.down(g, crate::input::Key::ShiftRight) }
    pub fn lmb(&self, g: &Game) -> bool { crate::input::input_lmb(g, self.input) }
    pub fn lmb_hit(&self, g: &Game) -> bool { crate::input::input_lmb_hit(g, self.input) }
    pub fn rmb(&self, g: &Game) -> bool { crate::input::input_rmb(g, self.input) }
    pub fn rmb_hit(&self, g: &Game) -> bool { crate::input::input_rmb_hit(g, self.input) }
    /// Shift is down and he is in a state to run.
    pub fn runs(&self, g: &Game) -> bool { !crate::condition::is_hurt(g, self.c, None) && self.shift(g) }
}

/// Where his keys send him this tick: the way across the floor (a unit, or nothing), and how far the keys asked of it.
#[derive(Clone, Copy)]
pub(super) struct Go {
    pub x: f64, pub z: f64, pub l: f64,
    pub sprint: bool,
    /// the sneak up on a man, while it has his feet
    pub stalk: Option<crate::stalk::StalkMove>,
    /// still stunned, and so badly that his feet are too
    pub stunned: bool, pub rooted: bool,
}

/// What he looks at this tick, and what is up to aim with.
#[derive(Clone, Copy)]
pub(super) struct Aim {
    pub at: V3,
    /// the way there across the floor, and how far
    pub to: V3, pub dist: f64,
    /// the pistol is in his hand; the rifle is in his hands
    pub gun_up: bool, pub rifle_up: bool,
    /// one of them is raised
    pub aiming: bool,
    /// how fast he goes
    pub speed: f64,
}

/// One tick of the player: his keys, his legs, his aim, his guns and throws, his footfalls.
pub fn update_player(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId) {
    let t = Tick::of(g, dt, input, cam);
    let launch_was = begin(g, t);
    if again_key(g, t) { return; }
    crate::stealthfeed::stealth_feed(g);
    crate::bigkit::sam_big_tail_tick(g, dt);
    crate::workstow::sam_stow_tick(g, &mut stow_holster);
    if crate::condition::down(g, t.c) { lies_down(g, t); return; }
    clocks(g, t);
    if deed_has_him(g, t) { return; }
    kit_keys(g, t);                                  // keys.rs
    let go = steer(g, t);
    wall_keys(g, t, go);
    move_him(g, t, go);
    let a = aim_now(g, t, go);
    face(g, t, go, a);                               // facing.rs
    pose(g, t, a);
    let blocked = gun_blocked(g, t, go, a);          // trigger.rs
    if g.pl[t.pl].wall.is_some() { wall_snap_pose(g, dt, a.aiming); }
    let arc = triggers(g, t, go, a, blocked, launch_was);
    throws(g, t, a, arc);                            // throws.rs
    crate::kitanim::kit_tick(g, Some(true));
    crate::gunwall::gun_wall_pose(g, t.c);
    crate::character::character_update(g, t.c, dt);
    gun_light(g, t);
    footfall(g, t, a.speed);
}

/// What the last tick wrote that this one writes again, put back. Returns the launcher's load he had the button down on.
fn begin(g: &mut Game, t: Tick) -> Option<crate::weapons::RifleAttachment> {
    let Tick { pl, c, .. } = t;
    let p = &mut g.pl[pl];
    p.pace = 1.0;
    if !p.crouch && p.dragging().is_none() { p.knee = None; }   // (up again: how he went down is over)
    p.dash = false;
    g.ch[c].anim.dash = 0.0;
    g.ch[c].anim.run_ready = false;
    g.tab.camera[t.cam].peek_offset = None;
    crate::aimweight::aim_weigh_begin(g, pl);
    crate::noshove::no_shove_begin(g, None);
    g.pl[pl].launch_held.take()
}

/// Enter, once he is down or the mission is lost (with Shift, at any time): the mission again, from this run's newest checkpoint if he is
/// down and has one, else, and with Shift, from its start. True: the rest of the tick is not played.
fn again_key(g: &mut Game, t: Tick) -> bool {
    if !t.hit(g, crate::input::Key::Enter) { return false; }
    let (down, shift) = (crate::condition::down(g, t.c), t.shift(g));
    if !down && !shift && g.mission.stage != crate::game::MissionStage::Failed { return false; }
    if down && !shift && newest_checkpoint_loaded(g) { return true; }
    crate::savestate::control::ask_restart(g);
    true
}
fn newest_checkpoint_loaded(g: &mut Game) -> bool {
    let elapsed = g.mission.stats.elapsed;
    let Some(ck) = crate::savestate::checkpoints_newest(g, Some(true), Some(elapsed)) else { return false };
    crate::savestate::checkpoints_is_this_run(g, &ck) && crate::savestate::checkpoints_load_in_step(g, &ck.key).is_none()
}

/// He is down: everything he had hold of is let go, and his body falls.
fn lies_down(g: &mut Game, t: Tick) {
    let Tick { pl, c, .. } = t;
    if g.pl[pl].kit_was.is_some() || crate::character::anim_kit_busy(g, c) { crate::kitanim::kit_tick(g, None); }
    release_wall(g);
    free_held(g, crate::player::FreeHeldWhy::Shot);
    crate::deed::put_down(g, pl);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    light_off(g, pl);
    g.sets[g.seat].hand.throw_preview = None;
    g.ch[c].anim.lockpick = false;
    g.ch[c].anim.long_gun = false;
    crate::character::character_update(g, c, t.dt);
    crate::game::game_body_thud(g, c);
}

/// What runs whatever he does: his guns' clocks, his noise dying away, a stun's pose (and it makes him let go of whom he holds), the whistle, the gun at
/// a wall.
fn clocks(g: &mut Game, t: Tick) {
    let Tick { pl, c, dt, .. } = t;
    crate::weapons::firearm_update(g, t.gun, dt);
    crate::weapons::firearm_update(g, t.rifle, dt);
    crate::weapons::ocp_update(&mut g.pl[pl].ocp, dt);
    g.fa[t.rifle].ring_cd = m::max(0.0, g.fa[t.rifle].ring_cd - dt);
    g.pl[pl].equip_t = m::max(0.0, g.pl[pl].equip_t - dt);
    g.pl[pl].noise *= m::exp(-2.2 * dt);
    crate::samstun::sam_stun_pose(g);
    if g.pl[pl].holding().is_some() && staggered(g) { free_held(g, crate::player::FreeHeldWhy::Shot); }
    crate::whistle::whistle_key(g, t.input);
    crate::gunwall::gun_wall_stance(g, c, false);
    crate::gunwall::gun_wall_pose(g, c);
}
fn staggered(g: &Game) -> bool { crate::samstun::sam_staggered(g, &crate::seats::here(g), None) }

/// The deed he is in the middle of, where it has the whole of him: its own tick is played. True: no more of this one is.
fn deed_has_him(g: &mut Game, t: Tick) -> bool {
    let Tick { pl, dt, .. } = t;
    let held = g.pl[pl].holding().is_some();
    let working = crate::handwork::sam_work(g).is_some();
    let beat = g.pl[pl].planted() && !working && !held;
    if g.pl[pl].wanding().is_some() { nv_key(g, Some(t.input), dt); } else if beat { nv_key(g, None, dt); }
    if g.pl[pl].planted() || working || staggered(g) { g.pl[pl].kit_owed = -1.0; }
    if beat || held { crate::kitanim::kit_tick(g, None); }
    if g.pl[pl].takedown().is_some() { strikes(g, t); return true; }
    if held { update_hold(g, t); return true; }
    if g.pl[pl].planted() || crate::handwork::sam_work(g).is_some() { throw_off(g, pl); }
    if g.pl[pl].kick().is_some() { update_kick(g, dt); return true; }
    if g.pl[pl].picking().is_some() { update_picking(g, dt); return true; }
    if g.pl[pl].wanding().is_some() { update_wanding(g, dt); return true; }
    if works(g, t) { return true; }
    if g.pl[pl].coding().is_some() { update_coding_plant(g, dt); return true; }
    g.ch[t.c].anim.lockpick = false;
    false
}

/// A man struck down: both his hands are on him.
fn strikes(g: &mut Game, t: Tick) {
    update_takedown(g, t.dt);
    g.ch[t.c].anim.speed = 0.0;
    g.ch[t.c].anim.upper = crate::character::UpperMode::None;
    arm_rifle(g, false, None);
    crate::character::character_update(g, t.c, t.dt);
    light_off(g, t.pl);
}

/// The job his hands are at (a charge, a lever, a drive, a switch), unless the trigger lets go of it. True: it had him this tick.
fn works(g: &mut Game, t: Tick) -> bool {
    let Some(mut w) = crate::handwork::sam_work(g) else { return false };
    if t.lmb_hit(g) && crate::handwork::sam_work_let_go(g, w) {
        if g.pl[t.pl].work_stow.is_some() { crate::workstow::sam_stow_tick(g, &mut stow_holster); }
        return false;
    }
    if crate::workstow::sam_stowing(g, &crate::seats::here(g)) { crate::kitanim::kit_tick(g, None); }
    update_work_plant(g, t.dt, &mut w);
    work_spot_back(g, t.pl, &w);
    true
}
/// Where his feet were put for the job, written on the job.
fn work_spot_back(g: &mut Game, pl: PlayerId, w: &crate::handwork::SamWork) {
    use crate::handwork::SamWorkKind;
    let Some(at) = w.at else { return };
    match w.kind {
        SamWorkKind::Wall => if let Some(s) = g.pl[pl].wall_work_mut() { s.at = at; },
        SamWorkKind::Door => if let Some(s) = g.pl[pl].charging_mut() { s.at = at; },
        SamWorkKind::Breaker | SamWorkKind::Drive | SamWorkKind::Switch => {}
    }
}

/// His walk keys as a way across the floor: the sneak's, the body's he hauls or the leaf's he works where one of them has his feet.
fn steer(g: &mut Game, t: Tick) -> Go {
    let Tick { pl, dt, .. } = t;
    let [mut x, mut z] = keys_held(g, t.input, t.cam);
    let (lmb, lmb_hit) = (t.lmb(g), t.lmb_hit(g));
    let stalk = crate::stalk::stalk_step(g, lmb, lmb_hit, x != 0.0 || z != 0.0, Some(crate::stalk::StalkKeys { mx: x, mz: z, dt }));
    if let Some(st) = stalk { (x, z) = (st.mx, st.mz); g.pl[pl].crouch = true; g.pl[pl].knee = None; }
    if g.pl[pl].tow.is_none() { g.ch[t.c].anim.haul = None; }
    let (stunned, rooted) = (staggered(g), crate::samstun::sam_rooted(g, &crate::seats::here(g), None));
    let own = !rooted && (x != 0.0 || z != 0.0);
    if let Some(hs) = crate::hands::hands_step(g, own, dt) { (x, z) = (hs[0], hs[1]); }
    if rooted { (x, z) = (0.0, 0.0); }
    let l = m::hyp2(x, z);
    if l > 0.0 { x /= l; z /= l; }
    if t.hit(g, crate::input::Key::C) && g.pl[pl].dragging().is_none() { own_legs(g, pl, |g, m| crouch_key(g, m)); }
    let led = g.pl[pl].haul().is_some() || g.pl[pl].door_move().is_some();   // (what he pulls goes its own way: it is his keys that ask for the run)
    let sprint = t.runs(g) && if led { own } else { l > 0.0 && stalk.is_none() };
    Go { x, z, l, sprint, stalk, stunned, rooted }
}

/// His back to a wall, and off it again: Q, a Q remembered, the stick kept into a face, a run.
fn wall_keys(g: &mut Game, t: Tick, go: Go) {
    let Tick { pl, dt, input, cam, .. } = t;
    let Go { x, z, l, .. } = go;
    if !go.rooted {
        if t.hit(g, crate::input::Key::Q) { wall_key(g, t, go); } else { wall_snap_pending(g, dt, cam, go.sprint); }
        if g.k.wallsnap.press && go.stalk.is_none() && !crate::rails::rails_locks_wall(g) { wall_press(g, dt, input, cam, x, z, l, go.sprint); }
    }
    let ran_on = wall_ran_on(g, pl, input);
    if g.pl[pl].wall.is_some() && go.sprint && !ran_on { wall_step_off(g, x, z, l, crate::player::WallStepOffHow::Run); }
}
/// Q: off the wall he is on, or onto the one in reach.
fn wall_key(g: &mut Game, t: Tick, go: Go) {
    if g.pl[t.pl].wall.is_some() { wall_step_off(g, go.x, go.z, go.l, crate::player::WallStepOffHow::Q); return; }
    if go.sprint { return; }
    let mut why = crate::player::WallSnapWhy { blocked: false };
    if wall_snap(g, t.cam, Some(&mut why), None) || wall_snap_later(g, go.l, crate::player::WallSnapLaterWhy { blocked: why.blocked }) { return; }
    if g.pl[t.pl].dragging().is_some() { crate::interact::refuse(g, t.pl, Some(crate::view::ThingKind::Wall), "your hands are full"); }
    else { wall_refused(g, why.blocked); }
}

/// He goes: along the wall he is on, or a stride across the floor.
fn move_him(g: &mut Game, t: Tick, go: Go) {
    let Tick { pl, c, dt, .. } = t;
    if g.pl[pl].wall.is_some() && move_on_wall(g, dt, t.cam, go.x, go.z, go.l) { return; }
    if go.sprint && g.pl[pl].dragging().is_some() { crate::game::game_toggle_drag(g, None); }
    own_legs(g, pl, |g, m| sprint_stance(g, m, go.sprint, go.stalk.is_some()));
    if g.pl[pl].sprinting { reload_dropped(g, c, t.gun); reload_dropped(g, c, t.rifle); }
    let stalk = go.stalk.map(|s| crate::player::StrideStepStalk { pace: s.pace, keep: s.keep });
    crate::character::pos_replaced(g, c);
    own_legs(g, pl, |g, m| stride_step(g, m, go.x, go.z, go.l, stalk, dt, None));
}

/// What he looks at, what is up, and whether it is raised.
fn aim_now(g: &mut Game, t: Tick, go: Go) -> Aim {
    let pl = t.pl;
    let vel = g.ch[t.c].vel;
    let speed = m::hyp2(vel[0], vel[2]);
    let at = match go.stalk { Some(s) => s.at, None => g.sets[g.seat].aim_point };
    let pos = g.ch[t.c].pos;
    let to: V3 = [at[0] - pos[0], 0.0, at[2] - pos[2]];
    let (gun_up, rifle_up) = (crate::game::player_pistol_up(g, pl), crate::game::player_rifle_up(g, pl));
    let aiming = gun_raised(g, pl, None, None) && !g.pl[pl].sprinting && g.pl[pl].dragging().is_none() && !crate::hands::door_move_lowers(g, &crate::seats::here(g))
        && !(on_stack(g, pl) && !stack_up(g, t.dt, t.input));
    Aim { at, to, dist: m::hyp2(to[0], to[2]), gun_up, rifle_up, aiming, speed }
}

/// His body's pose for it: how fast, how low, against what wall, and how he carries what is in his hands.
fn pose(g: &mut Game, t: Tick, a: Aim) {
    use crate::character::{LongStance, Stance, UpperMode};
    let p = &g.pl[t.pl];
    let (sprinting, drags, wall) = (p.sprinting, p.dragging().is_some(), p.wall);
    let ch = &mut g.ch[t.c];
    let an = &mut ch.anim;
    an.speed = a.speed;
    an.crouch_target = if p.crouch { 1.0 } else { 0.0 };
    an.dash = if p.dash { 1.0 } else { 0.0 };
    if let Some(w) = wall {
        an.stance = Stance::Wall;
        an.stack_side = w.side;
        an.wall_peek = w.peek;
        an.wall_yaw = v3::wrap_angle(m::atan2(w.n[0], w.n[2]) - ch.body_yaw);
    }
    an.upper = if sprinting || drags { UpperMode::None } else if a.aiming { UpperMode::Aim } else if a.gun_up { UpperMode::Relaxed } else { UpperMode::None };
    an.run_ready = sprinting && a.gun_up && !an.hide_held_item;
    let leaf_on_him = an.door_push.as_ref().map_or(false, |d| d.w >= 1.0);
    let in_hands = a.rifle_up && !an.hide_held_item;
    let carriage = if sprinting { LongStance::LowLong } else if leaf_on_him && !a.aiming { LongStance::PortLong } else { LongStance::Shoulder };
    arm_rifle(g, in_hands, if in_hands { Some(carriage) } else { None });
}

/// The weapon light: its place and direction are the gun's.
fn gun_light(g: &mut Game, t: Tick) {
    let infinite = g.infinite_ammo;
    g.fa[t.gun].infinite = infinite;
    g.fa[t.rifle].infinite = infinite;
    let power = g.k.tune.player_light;
    let light = g.pl[t.pl].light;
    let l = &mut g.tab.light[light];
    l.peak_intensity = power;
    l.intensity = power;
    l.radius = crate::guards::FLASHLIGHT_RADIUS * 0.7;
    pose_gun_light(g);
}
fn light_off(g: &mut Game, pl: PlayerId) { let light = g.pl[pl].light; g.tab.light[light].enabled = false; }

/// A footfall, when one is due: heard by the ear and by who is near.
fn footfall(g: &mut Game, t: Tick, speed: f64) {
    let Tick { pl, c, .. } = t;
    let phase = g.ch[c].anim.phase;
    let fall = footfall_due(g, t.dt, phase, speed);
    if fall == 0.0 { return; }
    let level = if g.pl[pl].sprinting { 1.0 } else {
        let base = if g.pl[pl].dash { g.k.crouchrun.loud } else if g.pl[pl].crouch { 0.125 } else if crate::kitload::load_gait(g, c, speed) > 2.0 { 0.6 } else { 0.26 };
        base * crate::kitload::load_heard(g, c)
    };
    tread_heard(g, c, level);
    if fall == 1.0 { let left = g.pl[pl].step_phase < 0.5; crate::wet::wet_footfall(g, left); }
}

/// His foot on the floor, `level` loud on a hard one: what it is worth here, told to who is near.
pub(super) fn tread_heard(g: &mut Game, c: CharId, level: f64) {
    let pos = g.ch[c].pos;
    let loud = crate::floor::tread(g, pos, level, true);
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos, time, loud: false, level: Some(loud), serial, ..Default::default() });
    let p = &mut g.pl[g.player];
    p.noise = m::max(p.noise, loud);
}
