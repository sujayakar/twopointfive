//! HIS GUNS: whether the one he has up may be raised here, its trigger, and what is under the rifle's barrel.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::JsMap;
use crate::m;
use crate::v3::V3;
use crate::weapons::RifleAttachment;
use alloc::{format, string::String};

/// A wall, a jamb or a leaf inside the raised gun's reach, a friend in its line, or a stun: the gun comes down to the low ready and no
/// round leaves it. With the trigger down he is told which.
pub(super) fn gun_blocked(g: &mut Game, t: Tick, go: Go, a: Aim) -> bool {
    use crate::character::{LongStance, UpperMode};
    let Tick { pl, c, dt, .. } = t;
    let rifle_aim = a.rifle_up && g.ch[c].anim.long_stance == Some(LongStance::Shoulder);
    let up = rifle_aim || a.gun_up;
    let lock_at = if rifle_aim { breach_at(g, t) } else { None };
    let friend = match crate::friends::sam_off_friend(g, c, up && !g.pl[pl].sprinting && lock_at.is_none(), dt) {
        Some(crate::friends::SamOffFriend::Bool(false)) => None,
        Some(crate::friends::SamOffFriend::Guard(f)) => Some(Some(f)),
        _ => Some(None),
    };
    let no_room = !(go.stunned && up) && g.pl[pl].wall.is_none()
        && crate::gunwall::gun_walled(g, c, if up { Some(crate::gunwall::GunUp::Aim) } else { None }, lock_at, dt, Some(false), Some(lock_at.is_some()));
    let blocked = (go.stunned && up) || no_room || friend.is_some();
    if blocked {
        g.ch[c].anim.upper = UpperMode::None;
        if rifle_aim { arm_rifle(g, true, Some(LongStance::LowLong)); }
        if t.lmb(g) {   // (held, not only pressed: the word stays up while the trigger is down)
            let text = match friend {
                _ if go.stunned => String::from("you're still stunned"),
                Some(Some(f)) => format!("{} is in your line of fire", g.gd[f].callsign),
                Some(None) => String::from("a friend is in your line of fire"),
                None => String::from("no room to raise your gun here"),
            };
            crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), &text);
        }
    }
    crate::gunwall::gun_wall_stance(g, c, blocked && !rifle_aim);
    blocked
}
/// The lock a breaching load would be put to.
fn breach_at(g: &Game, t: Tick) -> Option<V3> {
    let row = g.weapon(g.fa[t.rifle].def).pellets.as_ref().and_then(|p| p.breach.clone())?;
    crate::gunwall::breach_aim(g, t.c, row)
}

/// His guns this tick. Returns the arc of the launcher's load while its button is down.
pub(super) fn triggers(g: &mut Game, t: Tick, go: Go, a: Aim, blocked: bool, launch_was: Option<RifleAttachment>) -> Option<crate::throwables::ThrowSolution> {
    let Tick { pl, c, dt, .. } = t;
    g.arms[c].fire_cd -= dt;
    let lmb = t.lmb(g);
    if strings(g, t, t.gun) { crate::weapons::firearm_auto_idle(g, t.gun, dt, a.gun_up && lmb); }
    if a.gun_up && !g.pl[pl].sprinting && g.pl[pl].dragging().is_none() {
        if !blocked { pull(g, t, t.gun, a.speed); }
        if t.rmb_hit(g) && g.pl[pl].equip_t <= 0.0 && g.ch[c].anim.big_tool.is_none() { ocp_on_reticle(g); }
    }
    crate::weapons::firearm_auto_idle(g, t.rifle, dt, a.rifle_up && lmb);
    if !a.rifle_up || g.pl[pl].sprinting { return None; }
    if !blocked { pull(g, t, t.rifle, a.speed); }
    launcher(g, t, go, blocked, launch_was)
}

/// The string a gun is firing is kept: the rifle's always, a pistol's if it can fire on automatic.
fn strings(g: &Game, t: Tick, gun: FirearmId) -> bool { gun == t.rifle || g.weapon(g.fa[gun].def).auto.is_some() }

/// The trigger of the gun he has up: a round if it is pulled and the gun is ready, a click if it is pressed on an empty one.
fn pull(g: &mut Game, t: Tick, gun: FirearmId, speed: f64) {
    let Tick { pl, c, .. } = t;
    let (auto, pressed, rifle) = (crate::weapons::firearm_automatic(g, gun), t.lmb_hit(g), gun == t.rifle);
    let due = if auto || rifle { 1e-4 } else { 0.0 };
    if !(pressed || (auto && t.lmb(g))) || g.arms[c].fire_cd > due || g.pl[pl].equip_t > 0.0 { return; }
    let grow = crate::weapons::firearm_auto_spread(g, gun);
    if rifle && pressed && shell_break(g) == Some(crate::player::ShellBreak::Bolt) { g.arms[c].fire_cd = 0.1; return; }
    if !crate::weapons::firearm_fire(g, gun) {
        if pressed { dry_fire(g, c); }
        return;
    }
    let def = g.weapon(g.fa[gun].def);
    let (wait, weapon) = (def.auto.as_ref().filter(|_| auto).map_or(def.cd, |r| r.interval), if rifle { Some(def.clone()) } else { None });
    g.arms[c].fire_cd = wait;
    g.pl[pl].last_fire_t = g.time;
    if strings(g, t, gun) { crate::weapons::firearm_auto_fired(g, gun); }
    let to = shot_target(g, &crate::seats::here(g));
    let at = crate::aimweight::aim_weigh_shot(g, to);
    let seat = crate::seats::here(g);
    let own = if rifle { rifle_spread(g, &seat, speed) } else { 0.0 };
    let spread = own + grow + crate::suppression::sam_spread(g, &seat) + crate::aim::sam_aim(g);
    crate::combat::fire_weapon(g, c, at, Some(&crate::combat::FireOpts { weapon, spread: Some(spread), ..Default::default() }));
}
/// The trigger of an empty gun.
pub(super) fn dry_fire(g: &mut Game, c: CharId) {
    g.arms[c].fire_cd = 0.25;
    at_the_gun(g, c, "dryFire", 0.7);
}
/// What a round goes at: a fixture under the reticle, a man under the cursor, else level along the heading.
fn shot_target(g: &Game, seat: &crate::seats::SeatRef) -> V3 {
    if let Some(f) = seat.aim_target { return g.tab.fixture[f].pos; }
    if let Some(gd) = seat.aim_guard { return man_aim(g, seat, gd, 1.25); }
    free_aim(g, seat, None)
}
/// The pistol's right button: the light the reticle has snapped to, a man's flashlight too, is put out for a while. At nothing, it says no.
pub(super) fn ocp_on_reticle(g: &mut Game) {
    let (light, man) = { let s = &g.sets[g.seat]; (s.aim_target, s.aim_guard) };
    if crate::lighting::ocpable(g, &crate::seats::here(g)) || (light.is_none() && man.is_none()) { crate::game::game_ocp(g, light); }
}

/// The launcher: pressing begins the gesture, the arc is drawn while the button stays down, letting go sends the round. Returns the arc.
fn launcher(g: &mut Game, t: Tick, go: Go, blocked: bool, was: Option<RifleAttachment>) -> Option<crate::throwables::ThrowSolution> {
    let Tick { pl, c, rifle, dt, .. } = t;
    let beat = g.pl[pl].equip_t > 0.0 && g.pl[pl].wall.is_some();
    let owed = blocked && !go.stunned && was.is_some() && crate::aimweight::aim_weigh_launch_keep(g, pl, dt);
    let load = crate::weapons::rifle_attachment(g, rifle)?;
    let left = crate::weapons::rifle_left(g, rifle, load) > 0.0;
    let called_off = crate::backkey::back_hit(g, t.input) || t.hit(g, "Blur") || t.hit(g, "Paused");
    let kept = was.is_some() && !beat && (!blocked || owed) && !go.stunned && g.sets[g.seat].orders.menu.is_none() && left && !called_off;
    let pressed = !kept && t.rmb_hit(g) && !beat && !blocked && !go.stunned;
    let begun = pressed && left && g.fa[rifle].ring_cd <= 0.0;
    if pressed && !begun { at_the_gun(g, c, "dryFire", 0.4); }
    if !kept && !begun { return None; }
    if owed { g.pl[pl].launch_held = Some(load); return None; }
    let from = crate::rig_props::sc20k_launcher_muzzle(g, c);
    let seat = crate::seats::here(g);
    let point = match load {
        RifleAttachment::Ring => crate::launcher::ring_aim_point(g, &seat, from),
        RifleAttachment::Cam => crate::stickycam::cam_aim_point(g, &seat, from),
        RifleAttachment::He40 => crate::he40::shell_aim_point(g, &seat, from),
    };
    let at = crate::aimweight::aim_weigh_shot(g, point);
    let arc = match load {
        RifleAttachment::Ring => crate::launcher::ring_solve(g, from, at),
        RifleAttachment::Cam => crate::stickycam::cam_solve(g, from, at),
        RifleAttachment::He40 => { let bodies = crate::he40::shell_bodies(g); crate::he40::shell_solve(g, from, at, Some(&bodies)) }
    };
    let let_go = !(t.rmb(g) || t.rmb_hit(g)) && g.pl[pl].equip_t <= 0.0;
    if !crate::aimweight::aim_weigh_launch(g, dt, let_go) { g.pl[pl].launch_held = Some(load); return Some(arc); }
    send(g, t, load, from, at);
    None
}
/// The launcher's round goes.
fn send(g: &mut Game, t: Tick, load: RifleAttachment, from: V3, at: V3) {
    let Tick { pl, c, rifle, .. } = t;
    let (again, settle) = match load {
        RifleAttachment::Ring => { crate::launcher::fire_ring(g, c, from, at); (g.k.launcher.cd, 0.3) }
        RifleAttachment::Cam => {
            crate::stickycam::fire_cam(g, pl, from, at);
            let mut data = JsMap::new();
            data.set(String::from("kind"), Some(crate::chronicle::ChronicleDataValue::Text(String::from("cam"))));
            crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Thrown, Some(&data), None);
            (g.k.sticky.cd, 0.3)
        }
        RifleAttachment::He40 => { crate::he40::fire_shell(g, c, from, at); crate::chronicle::chron_thrown(g, "he40"); (g.k.launcher40.cd, 0.4) }
    };
    if !g.infinite_ammo {
        let fa = &mut g.fa[rifle];
        match load { RifleAttachment::Ring => fa.rings -= 1.0, RifleAttachment::Cam => fa.cams -= 1.0, RifleAttachment::He40 => fa.shells -= 1.0 }
    }
    g.fa[rifle].ring_cd = again;
    g.arms[c].fire_cd = m::max(g.arms[c].fire_cd, settle);
    g.pl[pl].last_fire_t = g.time;
    crate::character::anim_fire(g, c);
}
