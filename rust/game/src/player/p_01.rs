//! src/game/player.ts lines 1 to 392
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ what skeleton 4 still blocks (each is a line in notes.md)

/// player.ts:266 Rifle.attachment: the getter's stub `rifle_attachment()` takes neither `g` nor the rifle. The fields are there, so this is the getter's own line (weapons.ts:533)
fn seam_rifle_attachment(g: &mut crate::game::Game, rifle: FirearmId) -> Option<crate::weapons::RifleAttachment> {
    let i = g.fa[rifle].att_idx;
    if i >= 0 { g.fa[rifle].attachments.get(i as usize).copied() } else { None }
}
/// player.ts:265 `rf.attachments[(rf.attIdx + 1) % Math.max(1, rf.attachments.length)]`: next_load's signature has `rf` and no `g`
fn seam_next_attachment(rf: FirearmId) -> Option<crate::weapons::RifleAttachment> { todo!("needs: next_load(g, rf)") }

/// a gun's row of WEAPONS (TypeScript: `gun.def`, the object itself)
fn def_of(g: &crate::game::Game, def: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(def).unwrap()
}

// ------------------------------------------------------------ the sums

/// how long his line hangs over him before the man answers (player.ts samBeat)
pub fn sam_beat(chars: f64) -> f64 { v3::clamp(0.7 + 0.018 * chars, 0.9, 1.4) }

/// his collision circle's radius against the statics with his centre at `p` (player.ts playerBodyR)
pub fn player_body_r(g: &mut crate::game::Game, p: V3) -> f64 { body_r(g, p) }

/// the player's own push-out of the statics for a spot he is not standing on yet (player.ts settleSpot)
pub fn settle_spot(g: &mut crate::game::Game, p: &mut V3) {
    let y1 = if g.pl[g.player].crouch { 1.0 } else { 1.7 };
    for _k in 0..4 {
        let r = body_r(g, *p);
        crate::scene_collision::collide_circle(g, p, r, 0.2, y1, Some(6.0));
        if !crate::scene_collision::clamp_to_world(g, p, BODY_R) { break; }
    }
}

/// he is as wide as his shoulders in a door frame: BODY_R narrows to DOOR_R on a doorway's middle line (player.ts bodyR)
pub fn body_r(g: &mut crate::game::Game, p: V3) -> f64 {
    let fit = door_fit();
    let mut k: f64 = 0.0;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        let fc = g.dr[d].frame_centre;
        let dx = p[0] - fc[0];
        let dz = p[2] - fc[2];
        if dx * dx + dz * dz > 1.3 { continue; }
        let c = g.dr[d].def.closed_dir;
        let ux = m::cos(c);
        let uz = m::sin(c);
        let lat = m::abs(dx * ux + dz * uz);
        let out = m::abs(dz * ux - dx * uz);
        let kl = 1.0 - v3::clamp((lat - fit.lat[0]) / (fit.lat[1] - fit.lat[0]), 0.0, 1.0);
        let ko = 1.0 - v3::clamp((out - fit.out[0]) / (fit.out[1] - fit.out[0]), 0.0, 1.0);
        k = m::max(k, kl * kl * (3.0 - 2.0 * kl) * ko * ko * (3.0 - 2.0 * ko));
    }
    BODY_R - (BODY_R - DOOR_R) * k
}

/// the player's own collision: a sweep from the last settled spot, then the push-out (player.ts collidePlayer)
pub fn collide_player(g: &mut crate::game::Game, y1: f64) {
    let id = g.player;
    let c = g.pl[id].ch;
    let mut pl = crate::player::CollideManPl { char: crate::player::CollideManPlChar { pos: g.ch[c].pos }, safe: g.pl[id].safe };
    collide_man(g, &mut pl, y1);
    g.ch[c].pos = pl.char.pos;
    g.pl[id].safe = pl.safe;
}

/// the same settle for any body with a place and the spot it last stood clear at (player.ts collideMan)
pub fn collide_man(g: &mut crate::game::Game, pl: &mut crate::player::CollideManPl, y1: f64) {
    let mut p = pl.char.pos;
    if let Some(s) = pl.safe {
        let dx = p[0] - s[0];
        let dz = p[2] - s[2];
        let n = m::min(32.0, m::ceil(m::hyp2(dx, dz) / SWEEP_STEP));
        if n > 1.0 {
            p[0] = s[0];
            p[2] = s[2];
            let mut i = 1.0;
            while i < n {
                p[0] += dx / n;
                p[2] += dz / n;
                let r = body_r(g, p);
                crate::scene_collision::collide_circle(g, &mut p, r, 0.2, y1, Some(4.0));
                crate::scene_collision::clamp_to_world(g, &mut p, BODY_R);
                i += 1.0;
            }
            p[0] += dx / n;
            p[2] += dz / n;
        }
    }
    for _k in 0..4 {
        let r = body_r(g, p);
        crate::scene_collision::collide_circle(g, &mut p, r, 0.2, y1, Some(6.0));
        if !crate::scene_collision::clamp_to_world(g, &mut p, BODY_R) { break; }
    }
    pl.char.pos = p;
    pl.safe = Some([p[0], 0.0, p[2]]);
}

/// may he stand up here (player.ts canStand)
pub fn can_stand(g: &mut crate::game::Game) -> bool {
    let p = g.ch[g.pl[g.player].ch].pos;
    can_stand_at(g, p)
}

/// may he stand up at a spot: settles a copy against the band between crouched and standing head height (player.ts canStandAt)
pub fn can_stand_at(g: &mut crate::game::Game, p: V3) -> bool {
    let mut q: V3 = [p[0], p[1], p[2]];
    let r = body_r(g, p);
    crate::scene_collision::collide_circle(g, &mut q, r, 1.0, 1.7, Some(2.0));
    m::hyp2(q[0] - p[0], q[2] - p[2]) < 0.01
}

/// settle the player against the statics now, swept from where he last stood clear (player.ts settlePlayer)
pub fn settle_player(g: &mut crate::game::Game) {
    let pl = g.player;
    let p = g.ch[g.pl[pl].ch].pos;
    let still = match g.pl[pl].safe { Some(s) => s[0] == p[0] && s[2] == p[2], None => false };
    if g.pl[pl].down || still { return; }
    let y1 = if g.pl[pl].crouch { 1.0 } else { 1.7 };
    collide_player(g, y1);
}

/// the crouch-walk's top speed: the sneak's own (player.ts crouchPace; TWOGEARS.on settled)
pub fn crouch_pace(g: &mut crate::game::Game) -> f64 { g.k.twogears.crouch }

/// one notch of the wheel on his pace: under TWOGEARS.on (settled) the wheel is nobody's pace (player.ts wheelPace)
pub fn wheel_pace(g: &mut crate::game::Game, wheel: f64) {}

/// asked once a frame by both places a footfall is filed: 0 none, 1 at an edge of the cycle, 2 one the legs owe (player.ts footfallDue; FOOTFALL.once and .owed settled)
pub fn footfall_due(g: &mut crate::game::Game, dt: f64, phase: f64, speed: f64) -> f64 {
    let (time, f) = (g.time, g.k.footfall);
    let id = g.player;
    footfall_on(&mut g.pl[id], time, f.travel, f.owe, dt, phase, speed)
}

/// footfallDue on the man's own numbers: it keeps stepPhase, stepTravel, stepOwed and stepAt (player.ts:248-256)
fn footfall_on(p: &mut crate::game::Player, time: f64, travel: f64, owe: f64, dt: f64, phase: f64, speed: f64) -> f64 {
    let prev = p.step_phase;
    let edge = (prev < 0.5) != (phase < 0.5);
    p.step_phase = phase;
    let d = m::abs(phase - prev);
    let turn = if d > 0.5 { 1.0 - d } else { d };
    p.step_travel = m::min(1.0, p.step_travel + turn);
    if edge || speed <= 0.4 || time - p.step_at > dt * 1.5 { p.step_owed = 0.0; } else { p.step_owed += turn; }
    p.step_at = time;
    if speed <= 0.4 || (if edge { p.step_travel < travel } else { !(p.step_owed >= owe) }) { return 0.0; }
    p.step_travel = 0.0;
    p.step_owed = 0.0;
    if edge { 1.0 } else { 2.0 }
}

/// what the selected slot puts in his hand, for the HUD line (player.ts itemName)
pub fn item_name(g: &mut crate::game::Game, pl: PlayerId) -> String {
    match g.pl[pl].slot {
        crate::game::SlotKind::Pistol => { let def = g.fa[g.pl[pl].pistol].def; def_of(g, def).short.clone() }
        crate::game::SlotKind::Smoke => String::from("smoke canister"),
        crate::game::SlotKind::Flash => String::from("stun canister"),
        crate::game::SlotKind::Frag => String::from("frag grenade"),
        _ => rifle_name(g, pl),
    }
}

/// what 2 pressed again puts on the right button after the load that is up (player.ts nextLoad)
pub fn next_load(rf: FirearmId) -> String {
    let a = seam_next_attachment(rf);
    String::from(match a {
        Some(crate::weapons::RifleAttachment::Cam) => "the sticky cameras",
        Some(crate::weapons::RifleAttachment::He40) => "the 40 mm",
        _ => "the ring airfoil",
    })
}

/// the rifle as the HUD line names it (player.ts rifleName)
pub fn rifle_name(g: &mut crate::game::Game, pl: PlayerId) -> String {
    let rifle = g.pl[pl].rifle;
    let def = g.fa[rifle].def;
    let short = def_of(g, def).short.clone();
    let mode = g.fa[rifle].mode.as_str().to_uppercase();
    let att = seam_rifle_attachment(g, rifle);
    let tail = match att {
        Some(crate::weapons::RifleAttachment::Ring) => { let tag = crate::gear::ring_tag(g); format!(" · {} ×{}", tag, g.fa[rifle].rings) }
        Some(crate::weapons::RifleAttachment::He40) => format!(" · 40 MM ×{}", g.fa[rifle].shells),
        Some(crate::weapons::RifleAttachment::Cam) => format!(" · CAM ×{}", g.fa[rifle].cams),
        None => String::new(),
    };
    format!("{} · {}{}", short, mode, tail)
}

/// put the selected item away or draw it again: a reload in hand is abandoned on holstering; the rifle takes RIFLE_EQUIP_SECS to come up (player.ts setHolstered)
pub fn set_holstered(g: &mut crate::game::Game, on: bool, quiet: Option<bool>) {
    let quiet = quiet.unwrap_or(false);
    let pl = g.player;
    if g.pl[pl].holstered == on { return; }
    g.pl[pl].holstered = on;
    let (c, pistol, rifle) = (g.pl[pl].ch, g.pl[pl].pistol, g.pl[pl].rifle);
    if on && crate::weapons::firearm_reloading(g, pistol) {
        crate::weapons::firearm_cancel_reload(g, pistol);
        crate::character::anim_cancel_reload(g, c);
    }
    if on && crate::weapons::firearm_reloading(g, rifle) {
        crate::weapons::firearm_cancel_reload(g, rifle);
        crate::character::anim_cancel_reload(g, c);
    }
    if g.pl[pl].slot == crate::game::SlotKind::Rifle { g.pl[pl].equip_t = RIFLE_EQUIP_SECS; }
    if !quiet {
        let text = if on {
            String::from(if crate::game::player_rifle_sel(g, pl) { "slung" } else { "holstered" })
        } else {
            item_name(g, pl)
        };
        crate::game::game_msg(g, &text);
        crate::audio_iface::game_audio_like_play(g, "equip", None, Some(0.45), None);
    }
}

/// the stow's way to his holster: quiet (player.ts stowHolster)
pub fn stow_holster(g: &mut crate::game::Game, on: bool) { set_holstered(g, on, Some(true)); }

/// every gun's reload in hand abandoned: the hands are wanted (player.ts cancelReloads)
pub fn cancel_reloads(g: &mut crate::game::Game, pl: PlayerId) {
    let (c, pistol, rifle) = (g.pl[pl].ch, g.pl[pl].pistol, g.pl[pl].rifle);
    crate::weapons::firearm_cancel_reload(g, pistol);
    crate::weapons::firearm_cancel_reload(g, rifle);
    crate::character::anim_cancel_reload(g, c);
}

/// where the round and the gun's pitch go on a man: `h` over his feet, brought down onto him when his head is low (player.ts manAim; AIM.player.low settled)
pub fn man_aim(g: &mut crate::game::Game, gd: GuardId, h: f64) -> V3 {
    use crate::character::Bone;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let p: V3 = [pos[0], pos[1] + h, pos[2]];
    let top = g.ch[c].bones.get(Bone::HeadTop).map(|b| b[1]);
    let chest = g.ch[c].bones.get(Bone::Chest).copied();
    let (top, chest) = match (top, chest) { (Some(t), Some(ch)) => (t, ch), _ => return p };
    if pos[1] + LOW_AT <= top - LOW_UNDER { return p; }
    let y = m::max(pos[1] + 0.15, m::min(p[1], top - LOW_TOP));
    let at: V3 = [chest[0], y, chest[2]];
    let from = g.ch[g.pl[g.player].ch].muzzle;
    let d = v3::sub(at, from);
    let l = v3::len(d);
    if l > 1e-3 && crate::character::character_ray_hit(g, c, from, v3::scale(d, 1.0 / l), l + 1.0) >= 0.0 { return at; }
    let pos = g.ch[c].pos;
    let hips = g.ch[c].bones.get(Bone::Hips).copied();
    match hips { Some(b) => [b[0], y, b[2]], None => [pos[0], y, pos[2]] }
}

/// the level aim holds this frame: no script holds his aim (player.ts levelOn; AIM.player.level settled)
pub fn level_on(g: &mut crate::game::Game) -> bool {
    match &g.puppet { Some(p) => p.aim.is_none(), None => true }
}

/// the point his fire sites send a round at with nothing under the reticle (player.ts freeAim)
pub fn free_aim(g: &mut crate::game::Game, keep: Option<f64>) -> V3 {
    let keep = keep.unwrap_or(0.25);
    let c = g.pl[g.player].ch;
    let ap = g.aim_point;
    if !level_on(g) { return [ap[0], ap[1] + 0.03, ap[2]]; }
    let (pos, muzzle, aim_yaw) = (g.ch[c].pos, g.ch[c].muzzle, g.ch[c].aim_yaw);
    let dx = ap[0] - pos[0];
    let dz = ap[2] - pos[2];
    let d = m::hyp2(dx, dz);
    let yaw = if d > keep { m::atan2(dx, dz) } else { aim_yaw };
    let r = m::max(d, LEVEL_REACH);
    [pos[0] + m::sin(yaw) * r, pos[1] + v3::clamp(muzzle[1] - pos[1], 0.6, 1.6), pos[2] + m::cos(yaw) * r]
}

/// the one weapon light he owns, posed off whichever gun is in the hand and lit by that gun's switch (player.ts poseGunLight; KITANIM.on settled)
pub fn pose_gun_light(g: &mut crate::game::Game) {
    use crate::character::Bone;
    let pl = g.player;
    let (c, l, pistol, rifle) = (g.pl[pl].ch, g.pl[pl].light, g.pl[pl].pistol, g.pl[pl].rifle);
    let pistol_up = crate::game::player_pistol_up(g, pl);
    let rifle_up = crate::game::player_rifle_up(g, pl);
    let enabled = ((pistol_up && g.fa[pistol].light_on) || (rifle_up && g.fa[rifle].light_on))
        && !g.pl[pl].sprinting && !g.ch[c].anim.hide_held_item && !g.pl[pl].down && crate::kitanim::kit_gun_up(g);
    g.tab.light[l].enabled = enabled;
    if enabled {
        let (lens, dir) = if rifle_up {
            let lp = crate::rig_props::sc20k_light_pose(g, c, None);
            (lp.pos, lp.dir)
        } else {
            let lp = crate::rig_props::rig_light_pose(g, c, None);
            (lp.pos, lp.dir)
        };
        let sh = g.ch[c].bones.get(Bone::UpperArmR).copied();
        let pos = match sh {
            Some(sh) => { let r = g.tab.light[l].radius; crate::lighting::lens_clear(g, sh, lens, r, Some(l)) }
            None => lens,
        };
        g.tab.light[l].pos = pos;
        g.tab.light[l].dir = dir;
    }
}

/// arm or disarm the animator's long gun for the rifle, each frame (player.ts armRifle; GEAR.on, KITANIM.on, KITANIM.grips settled)
pub fn arm_rifle(g: &mut crate::game::Game, in_hands: bool, carriage: Option<crate::character::LongStance>) {
    let pl = g.player;
    let (c, pistol, rifle) = (g.pl[pl].ch, g.pl[pl].pistol, g.pl[pl].rifle);
    let (pistol_def, rifle_def) = (g.fa[pistol].def, g.fa[rifle].def);
    let pellets = def_of(g, rifle_def).pellets.is_some();
    {
        let an = &mut g.ch[c].anim;
        an.long_gun = in_hands;
        an.long_stance = if in_hands { carriage } else { None };
        an.long_steady = 1.0;
        an.reload_back = false;
        an.reload_mag = !pellets;
    }
    crate::gunreload::feed_deal(g, c, if in_hands { Some(rifle) } else { None }, None);
    let into = g.ch[c].gun_look;
    let (held, other) = if in_hands { (rifle_def, pistol_def) } else { (pistol_def, rifle_def) };
    let (held, other) = (def_of(g, held).clone(), def_of(g, other).clone());
    let look = crate::gunlook::gun_look_of(g, into, Some(&held), Some(&other), None);
    g.ch[c].gun_look = look;
    let own = if in_hands { g.ch[c].gun_look.and_then(|l| l.hand) } else { None };
    let fit = !g.touring;
    if in_hands {
        let model = match own { Some(id) => crate::rig_props::model_grip(g, id, Some(fit), None), None => None };
        let (wrist, fingers, back) = match model {
            Some(h) => (h.wrist, h.fingers, h.back),
            None => { let h = crate::rig_props::sc20k_grip(None, Some(fit)); (h.wrist, h.fingers, h.back) }
        };
        g.ch[c].anim.long_grip = crate::character::AnimLongGrip { wrist, fingers, back };
        let ahead = match own { Some(id) => crate::rig_props::model_muzzle_ahead(g, id, None), None => None };
        g.ch[c].muzzle_ahead = match ahead { Some(a) => a, None => crate::rig_props::sc20k_muzzle_ahead(None) };
    } else {
        g.ch[c].muzzle_ahead = def_of(g, pistol_def).muzzle_ahead;
    }
}

/// the trigger pressed over a tube gun being fed by the shell: what was fed is in the gun, so the feeding stops there (player.ts shellBreak)
pub fn shell_break(g: &mut crate::game::Game) -> Option<crate::player::ShellBreak> {
    let pl = g.player;
    let (rifle, c) = (g.pl[pl].rifle, g.pl[pl].ch);
    let reloading = crate::weapons::firearm_reloading(g, rifle);
    let feeding = reloading && crate::weapons::firearm_shell_fed(g, rifle);
    if reloading { if !feeding { return None; } } else if crate::gunreload::shell_plan(g, rifle, None, None).is_none() { return None; }
    if g.fa[rifle].chamber == 1.0 || (g.fa[rifle].infinite && g.fa[rifle].mag.is_some()) {
        if !feeding { return None; }
        crate::weapons::firearm_cancel_reload(g, rifle);
        crate::character::anim_cancel_reload(g, c);
        return Some(crate::player::ShellBreak::Fire);
    }
    if !g.fa[rifle].mag.map_or(false, js::truthy) { return None; }
    if feeding && g.fa[rifle].feed_n == 0.0 { return Some(crate::player::ShellBreak::Bolt); }
    if crate::character::anim_kit_busy(g, c) || !crate::game::player_rifle_up(g, pl) || wall_not_stack(g, pl) { return None; }
    if feeding {
        crate::weapons::firearm_cancel_reload(g, rifle);
        crate::character::anim_cancel_reload(g, c);
    }
    if reload_in_hand(g, Some(false), Some(true)) { Some(crate::player::ShellBreak::Bolt) } else { None }
}

/// `pl.wall && pl.wall.kind !== 'stack'` (player.ts:347,369)
fn wall_not_stack(g: &crate::game::Game, pl: PlayerId) -> bool {
    match &g.pl[pl].wall { Some(w) => w.kind != Some(crate::player::WallHoldKind::Stack), None => false }
}

/// a tube gun's beat as the weapon's row (TypeScript: the one object, ShellRow being WeaponDef['reload'] & { exact: true })
fn shell_row_as_weapon_row(r: &crate::gunreload::ShellRow) -> crate::weapons::WeaponDefReload {
    crate::weapons::WeaponDefReload { dur: r.dur, out: r.out, in_: r.in_, rack: r.rack, shell: r.shell }
}

/// a tube gun's beat as the animator's (the same object in the TypeScript; `exact` is what tells it from a magazine change's)
fn shell_row_as_beat(r: &crate::gunreload::ShellRow) -> crate::character::ReloadBeat {
    crate::character::ReloadBeat { dur: r.dur, out: r.out, in_: r.in_, rack: r.rack.map(Some), keep: None, hand: None, exact: Some(r.exact) }
}

/// feedPlan's answer as the weapon's events hold it (the closures of player.ts:381-384 read it; they are ReloadEvents::Rifle now)
fn feed_for_events(f: &crate::gunreload::FeedPlan) -> crate::weapons::FeedPlan {
    let cues = f.cues.as_ref().map(|cs| cs.iter().map(|q| crate::weapons::FeedCue { t: q.t, sfx: q.sfx.clone(), gain: q.gain, rate: q.rate }).collect());
    crate::weapons::FeedPlan { cues, times: f.times.clone(), drop: f.drop }
}

/// reload the gun in his hands, the one way a reload starts for the player: true when a reload began (player.ts reloadInHand; KITANIM.on settled)
pub fn reload_in_hand(g: &mut crate::game::Game, sprinting: Option<bool>, bolt_only: Option<bool>) -> bool {
    let sprinting = sprinting.unwrap_or(false);
    let bolt_only = bolt_only.unwrap_or(false);
    let pl = g.player;
    let (c, gun, rifle) = (g.pl[pl].ch, g.pl[pl].pistol, g.pl[pl].rifle);
    if crate::character::anim_kit_busy(g, c) || g.pl[pl].equip_t > 0.0 { return false; }
    if crate::game::player_pistol_up(g, pl) && !sprinting && g.pl[pl].wall.is_none() {
        let plan = crate::kitanim::kit_reload(g, gun, false);
        let row = match plan { Some(p) => p.row, None => { let def = g.fa[gun].def; def_of(g, def).reload.clone() } };
        let fill = crate::gunreload::feed_plan(g, gun, row, None).and_then(|f| f.drop).unwrap_or(0.0);
        let ev = crate::weapons::ReloadEvents::Pistol { ch: c, fill };
        if crate::weapons::firearm_start_reload(g, gun, Some(&ev), plan.map(|p| p.row), None, None) {
            match plan {
                Some(p) => {
                    crate::character::anim_reload(g, c, Some(p.beat.dur), Some(p.beat));
                    crate::kitanim::kit_reload_note(g, p.kind, p.beat.dur);
                }
                None => crate::character::anim_reload(g, c, None, None),
            }
            crate::game::game_msg(g, "reloading");
            return true;
        }
        if g.fa[gun].spare.is_empty() { crate::game::game_msg(g, "no spare magazines"); }
        return false;
    }
    if crate::game::player_rifle_up(g, pl) && !sprinting && !wall_not_stack(g, pl) {
        let def = g.fa[rifle].def;
        let rack_it = g.fa[rifle].chamber == 0.0;
        let sp = crate::gunreload::shell_plan(g, rifle, Some(false), Some(if bolt_only { 0.0 } else { m::INF }));
        if sp.as_ref().map_or(false, |s| s.idle) {
            if g.fa[rifle].spare.is_empty() && !bolt_only {
                let short = def_of(g, def).short.clone();
                crate::game::game_msg(g, &format!("no more shells for the {}", short));
            }
            return false;
        }
        let plan = if sp.is_some() { None } else { crate::kitanim::kit_reload(g, rifle, true) };
        let beat = match &sp { Some(s) => Some(shell_row_as_weapon_row(&s.row)), None => plan.map(|p| p.row) };
        let row = match beat { Some(r) => r, None => def_of(g, def).reload.clone() };
        let asked = sp.as_ref().map(|s| crate::gunreload::FeedPlanShells { n: s.n, pull: s.pull });
        let fp = crate::gunreload::feed_plan(g, rifle, row, asked);
        let ev = crate::weapons::ReloadEvents::Rifle { ch: c, feed: fp.as_ref().map(feed_for_events) };
        let shells = sp.as_ref().map(|s| crate::weapons::FirearmStartReloadShells { times: s.times.clone(), port: s.port });
        let cues = fp.as_ref().map(|f| f.times.as_slice());
        if crate::weapons::firearm_start_reload(g, rifle, Some(&ev), beat, cues, shells.as_ref()) {
            if let Some(s) = &sp {
                crate::character::anim_reload(g, c, Some(s.row.dur), Some(shell_row_as_beat(&s.row)));
                if js::truthy(s.n) {
                    let short = def_of(g, def).short.clone();
                    crate::game::game_msg(g, &format!("reloading the {}", short));
                }
                return true;
            }
            match plan {
                Some(p) => {
                    crate::character::anim_reload(g, c, Some(p.beat.dur), Some(p.beat));
                    crate::kitanim::kit_reload_note(g, p.kind, p.beat.dur);
                }
                None => {
                    let r = def_of(g, def).reload.clone();
                    let b = crate::character::ReloadBeat { dur: r.dur, out: r.out, in_: r.in_, rack: if rack_it { r.rack.map(Some) } else { Some(None) }, keep: None, hand: None, exact: None };
                    crate::character::anim_reload(g, c, Some(r.dur), Some(b));
                }
            }
            let short = def_of(g, def).short.clone();
            crate::game::game_msg(g, &format!("reloading the {}", short));
            return true;
        }
        if g.fa[rifle].spare.is_empty() {
            let short = def_of(g, def).short.clone();
            let text = if def_of(g, def).pellets.is_some() { format!("no more shells for the {}", short) } else { format!("no spare magazines for the {}", short) };
            crate::game::game_msg(g, &text);
        }
        return false;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f64 = 1.0 / 60.0;

    /// (FOOTFALL's travel and owe, the page's) one frame's question at game time `t`
    fn ask(p: &mut crate::game::Player, t: f64, phase: f64, speed: f64) -> f64 { footfall_on(p, t, 0.25, 0.5, DT, phase, speed) }

    fn walker(phase: f64, t: f64) -> crate::game::Player {
        let mut p = crate::game::Player::blank();
        p.step_phase = phase;
        p.step_travel = 1.0;
        p.step_at = t - DT;
        p
    }

    /// the TypeScript's own bits, from bun: 0.7 + 0.018 * 20 is 0x3ff0f5c28f5c28f6, and the two ends of the clamp
    #[test]
    fn sam_beat_bits() {
        assert_eq!(sam_beat(0.0).to_bits(), 0.9f64.to_bits());
        assert_eq!(sam_beat(20.0).to_bits(), 0x3ff0f5c28f5c28f6);
        assert_eq!(sam_beat(100.0).to_bits(), 1.4f64.to_bits());
    }

    /// a plain walk files at each edge of the cycle, none between
    #[test]
    fn footfall_plain_walk_files_at_edges() {
        let mut p = walker(0.3, 10.0);
        assert_eq!(ask(&mut p, 10.0, 0.4, 1.5), 0.0);
        assert_eq!(ask(&mut p, 10.0 + DT, 0.55, 1.5), 1.0);
        assert_eq!(p.step_travel, 0.0);
    }

    /// a cycle rocking on the edge files once, then nothing until it has come a quarter turn
    #[test]
    fn footfall_rocking_on_an_edge_files_once() {
        let mut p = walker(0.49, 10.0);
        assert_eq!(ask(&mut p, 10.0, 0.51, 1.5), 1.0);
        assert_eq!(ask(&mut p, 10.0 + DT, 0.49, 1.5), 0.0);
        assert_eq!(ask(&mut p, 10.0 + 2.0 * DT, 0.51, 1.5), 0.0);
    }

    /// a cycle turned about short of its edges files the step the legs owe at half a turn of travel; slowing clears the debt
    #[test]
    fn footfall_the_legs_owe() {
        let mut p = walker(0.1, 10.0);
        assert_eq!(ask(&mut p, 10.0, 0.25, 1.5), 0.0);
        assert_eq!(ask(&mut p, 10.0 + DT, 0.4, 1.5), 0.0);
        assert_eq!(ask(&mut p, 10.0 + 2.0 * DT, 0.25, 1.5), 0.0);
        assert_eq!(ask(&mut p, 10.0 + 3.0 * DT, 0.1, 1.5), 2.0);
        assert_eq!(p.step_owed, 0.0);
        p.step_owed = 0.4;
        assert_eq!(ask(&mut p, 10.0 + 4.0 * DT, 0.11, 0.2), 0.0);
        assert_eq!(p.step_owed, 0.0);
    }

    /// a debt is not kept over frames the test was not asked
    #[test]
    fn footfall_a_gap_clears_the_debt() {
        let mut p = walker(0.1, 10.0);
        p.step_owed = 0.4;
        assert_eq!(ask(&mut p, 11.0, 0.3, 1.5), 0.0);
        assert_eq!(p.step_owed, 0.0);
    }
}
