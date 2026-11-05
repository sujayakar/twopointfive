//! src/game/combat.ts lines 342 to 663
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 3 still cannot give

/// `g.audio.play(name, at, gain, { duration })`: `opts` is a map of `crate::Opaque`, which holds no number.   (combat.ts:660)
fn seam_audio_play(g: &mut Game, name: &str, at: V3, gain: f64, duration: f64) { todo!("needs: game_audio_like_play's opts able to hold rate and duration") }
/// `gd.flashlight.enabled`: a LightId has no table and RtLight no accessors.   (combat.ts:399, 549)
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled of a LightId") }
/// `gd.flashlight.color`: a LightId has no table and RtLight no accessors.   (combat.ts:549)
fn seam_light_color(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.color of a LightId") }
/// `sideLocal(c.ch, p)`: it wants a `&PackChar` where a body holds a `RigId`.   (combat.ts:506)
fn seam_side_local(g: &mut Game, c: CharId, p: crate::packside::SidePlace) -> Option<crate::packside::SidePlace> { todo!("needs: side_local taking the body's RigId") }
/// `b.owner` of a box made this frame: Box has no table for a made box (`g.none`).   (combat.ts:456)
fn seam_box_owner(g: &mut Game, b: BoxId) -> f64 { todo!("needs: the table a made Box lives in") }
/// `b.mkey = k` of a box made this frame: the same.   (combat.ts:456)
fn seam_box_set_mkey(g: &mut Game, b: BoxId, k: f64) { todo!("needs: the table a made Box lives in") }

// ---------------------------------------------------------------- panels and contact

fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, or: V3) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => or }
}

/// `c.bones.headTop?.[1] ?? c.pos[1] + over`: the crown's height.   (combat.ts:197, he40.ts, launcher.ts)
fn crown_or(g: &Game, c: CharId, over: f64) -> f64 {
    match g.ch[c].bones.get(crate::character::Bone::HeadTop) { Some(p) => p[1], None => g.ch[c].pos[1] + over }
}

/// A weapon's row (`gd.weapon.def`).
fn row(g: &Game, def: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(def).unwrap()
}

/// `W.drop`, as dropFromHand's kind.
fn drop_kind(g: &Game, def: crate::weapons::WeaponId) -> DropFromHandKind {
    match row(g, def).drop { crate::slotsetup::SetupGun::Pistol => DropFromHandKind::Pistol, crate::slotsetup::SetupGun::Rifle => DropFromHandKind::Rifle }
}

/// `c.ragdollize(g.col, dir, quiet)`: the world is g.col, lent as a list of what is in it (a Ragdoll keeps only that it has a world).
fn ragdollize(g: &mut Game, c: CharId, dir: V3, quiet: bool) {
    let world = crate::ragdoll::RagdollWorld { boxes: g.col.boxes.ids().collect(), dynamic_boxes: g.col.dynamic_boxes.clone() };
    crate::character::character_ragdollize(g, c, Some(&world), dir, quiet);
}

/// `men(g)`: every seat's man, in seat order.
fn men_here(g: &mut Game) -> Vec<PlayerId> {
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    crate::seats::men(g, &of)
}

/// `a | b` of two box flags.
fn flags_or(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }

/// The pistol poked out through a panel: gun shoulder and muzzle on opposite sides of one.   (combat.ts armThrough)
pub fn arm_through(g: &mut crate::game::Game, c_2: CharId) -> bool {
    let or = v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]);
    let from = bone_or(g, c_2, crate::character::Bone::UpperArmR, or);
    let to = g.ch[c_2].muzzle;
    solid_between(g, from, to)
}

/// A panel between `from` and `to`: a static, or a leaf standing shut or as good as (SoundSeal). Short segments only.   (combat.ts solidBetween)
pub fn solid_between(g: &mut crate::game::Game, from: V3, to: V3) -> bool {
    if crate::scene_collision::segment_blocked_dynamic(g, from, to, Some(crate::scene_boxes::box_flag().sound_seal)) { return true; }
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < 1e-3 { return false; }
    let rd = v3::scale(d, 1.0 / l);
    let mut scratch: Vec<i32> = solid_scratch();
    let n = crate::scene_collision::gather(g, m::min(from[0], to[0]) - 0.1, m::min(from[2], to[2]) - 0.1, m::max(from[0], to[0]) + 0.1, m::max(from[2], to[2]) + 0.1, m::min(from[1], to[1]) - 0.1, m::max(from[1], to[1]) + 0.1, &mut scratch);
    let mut i = 0usize;
    while (i as f64) < n {
        let b = BoxId::from_ix(scratch[i] as usize);
        if crate::scene_boxes::ray_box(g, from, rd, b, l).is_some() { return true; }
        i += 1;
    }
    false
}

/// May a contact shot from `a` count on `b`: no panel between their chests.   (combat.ts contactClear)
pub fn contact_clear(g: &mut crate::game::Game, a: CharId, b: CharId) -> bool {
    if v3::dist_xz(g.ch[a].pos, g.ch[b].pos) > 1.2 { return true; }
    let from = v3::add(g.ch[a].pos, [0.0, 1.1, 0.0]);
    let or = g.ch[b].pos;
    let h = bone_or(g, b, crate::character::Bone::Hips, or);
    let to: V3 = [h[0], 1.1, h[2]];
    !solid_between(g, from, to)
}

/// rayHit plus the contact shot: a muzzle already inside his body cylinder is a hit where it is. `contact` None is the TypeScript's `true`.   (combat.ts bodyHit)
pub fn body_hit(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, tmax: f64, contact: Option<crate::combat::BodyHitContact>) -> f64 {
    let t = crate::character::character_ray_hit(g, c_2, ro, rd, tmax);
    if t >= 0.0 || !g.ch[c_2].alive || matches!(contact, Some(BodyHitContact::Entering)) { return t; }
    if !crate::character::character_inside_body(g, c_2, ro, Some(0.25)) { return -1.0; }
    let counts = match contact {
        None => true,
        Some(BodyHitContact::Entering) => false,
        // fireWeapon's own test (combat.ts:193): nothing solid nearer along the bore than his middle, no panel between the two chests
        Some(BodyHitContact::Muzzle { st0_t, muzzle, glass_arm, shooter }) => {
            let or = g.ch[c_2].pos;
            let h = bone_or(g, c_2, crate::character::Bone::Hips, or);
            let nearer = match st0_t { None => true, Some(t0) => t0 > m::hyp2(h[0] - muzzle[0], h[2] - muzzle[2]) };
            nearer && (glass_arm || contact_clear(g, shooter, c_2))
        }
        // a later leg through glass (combat.ts:197)
        Some(BodyHitContact::LegCrown) => ro[1] <= crown_or(g, c_2, 1.8),
        // a ring or a 40 mm round (launcher.ts:142, he40.ts)
        Some(BodyHitContact::Flight { radius }) => ro[1] < crown_or(g, c_2, 1.75) + radius,
    };
    if counts { 0.0 } else { -1.0 }
}

// ---------------------------------------------------------------- a man goes down

/// A man goes down: `how` he went and `by` whose hand; `quiet` is the physical side.   (combat.ts killGuard)
pub fn kill_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: Option<bool>, how: Option<crate::game::DownKind>, by: Option<crate::game::DownBy>, line: Option<&str>, zone: Option<crate::combat::KillGuardZone>) {
    use crate::game::{DownBy, DownKind, GuardState};
    let quiet = quiet.unwrap_or(false);
    let how = how.unwrap_or(if quiet { DownKind::Struck } else { DownKind::Shot });
    let by = by.unwrap_or(DownBy::Player);
    if g.gd[gd].civ.is_some() { crate::civilians::civilian_down(g, gd, dir, quiet, how, by, line); return; }
    if let Some(b) = g.gd[gd].bubble.clone() {
        if g.time - b.t < 0.8 && !g.messages.iter().any(|m| m.t == b.t && m.text.ends_with(b.text.as_str())) {
            let head = if g.gd[gd].unit.is_some() { alloc::format!("{}: ", g.gd[gd].callsign) } else { String::from("guard: ") };
            let text = head + b.text.as_str();
            crate::game::game_msg(g, &text);
        }
    }
    if g.gd[gd].state == GuardState::Dead { return; }
    if crate::rails::rails_spare_2(g, gd) { return; }
    g.gd[gd].script = None;
    let held = crate::guards::in_hand(g, gd);
    if by == DownBy::Player {
        let team = g.gd[gd].team;
        if !(crate::friendlyfire::ff_live(g) && !crate::teams::is_hostile(g, crate::teams::Team::Player, team)) {
            crate::mission::tally(g, if how == DownKind::Shot { crate::mission::TallyK::Kills } else { crate::mission::TallyK::Knockouts });
        }
        crate::chronicle::chron_down(g, gd, how, line);
    }
    crate::guards::witness_kill(g, gd, how, by);
    crate::morale::morale_man_down(g, gd);
    crate::netquiet::net_quiet_fall(g, gd);
    if g.gd[gd].torch_lost_at < 0.0 && g.gd[gd].gun_lost_at < 0.0 {
        let fl = g.gd[gd].flashlight;
        g.gd[gd].light_lit_at_loss = seam_light_enabled(g, fl);
    }
    let now = g.time;
    {
        let m = &mut g.gd[gd];
        m.state = GuardState::Dead; m.speed = 0.0; m.died_at = now; m.down_kind = Some(how); m.down_by = Some(by);
    }
    crate::tactics::tactics_man_down(g, gd);
    let c = g.gd[gd].ch;
    let w = g.fa[g.gd[gd].weapon].def;
    if let Some(h) = held {
        let torch = h == crate::guards::InHand::Torch;
        let kind = if torch { DropFromHandKind::Torch } else { drop_kind(g, w) };
        let along = if torch { g.gd[gd].beam_dir } else { g.ch[c].gun_dir };
        let suppressed = h == crate::guards::InHand::Weapon && row(g, w).suppressed;
        let it = drop_from_hand(g, c, kind, along, if quiet { [0.0, 0.0, 0.0] } else { dir }, Some(suppressed), Some(quiet), None);
        g.gd[gd].dropped = Some(it);
    }
    if how == DownKind::Wounded {
        let z = zone.map(|z| match z { KillGuardZone::Side => crate::wounds::WoundDownZone::Side, KillGuardZone::Leg => crate::wounds::WoundDownZone::Leg });
        crate::wounds::wound_down(g, gd, dir, z);
    } else {
        ragdollize(g, c, dir, quiet);
    }
    let text: String = match line {
        Some(l) => String::from(l),
        None => {
            let unit = g.gd[gd].unit.is_some();
            let team = g.gd[gd].team;
            let cs = g.gd[gd].callsign.clone();
            if how == DownKind::Choked { String::from("choked out \u{2014} he'll keep, breathing") }
            else if how == DownKind::Struck { String::from("takedown \u{2014} out cold, breathing (a knockout, not a kill)") }
            else if by == DownBy::Guard {
                if unit { alloc::format!("{} down \u{2014} hit by his own side", cs) } else { String::from("guard down \u{2014} hit by his own side") }
            }
            else if unit && team == crate::teams::Team::Hostile { alloc::format!("{} down", cs) }
            else if unit && team == crate::teams::Team::Operative { alloc::format!("{} down{}", cs, if by == DownBy::Player { " \u{2014} your round" } else { "" }) }
            else { String::from("guard down \u{2014} dead") }
        }
    };
    crate::game::game_msg(g, &text);
}

/// What a dying hand lets go of: an item at the hand, carrying the body's motion plus a nudge along the round.   (combat.ts dropFromHand)
pub fn drop_from_hand(g: &mut crate::game::Game, c_2: CharId, kind: crate::combat::DropFromHandKind, along: V3, dir: V3, suppressed: Option<bool>, quiet: Option<bool>, from: Option<V3>) -> ItemId {
    let suppressed = suppressed.unwrap_or(false);
    let quiet = quiet.unwrap_or(false);
    let h = match from {
        Some(f) => f,
        None => { let or = v3::add(g.ch[c_2].pos, [0.0, 1.0, 0.0]); bone_or(g, c_2, crate::character::Bone::HandR, or) }
    };
    let vel = g.ch[c_2].vel;
    let r0 = crate::math_dice::sim_random(g);
    let v0 = vel[0] * 0.6 + dir[0] * 1.1 + (r0 - 0.5) * 0.6;
    let r1 = crate::math_dice::sim_random(g);
    let v1 = 0.9 + r1 * 0.6;
    let r2 = crate::math_dice::sim_random(g);
    let v2 = vel[2] * 0.6 + dir[2] * 1.1 + (r2 - 0.5) * 0.6;
    let item_kind = match kind {
        DropFromHandKind::Pistol => crate::throwables::ItemKind::Pistol,
        DropFromHandKind::Rifle => crate::throwables::ItemKind::Rifle,
        DropFromHandKind::Torch => crate::throwables::ItemKind::Torch,
    };
    let yaw = v3::yaw_of(along);
    let opts = crate::throwables::ThrowablesSpawnOpts {
        who: Some(g.ch[c_2].id),
        fill: Some(if suppressed { 1.0 } else { 0.0 }),
        heading: Some(yaw),
        yaw: Some(yaw),
        life: Some(if kind == DropFromHandKind::Torch { 90.0 } else { GUN_ITEM_LIFE }),
        ..Default::default()
    };
    let it = crate::throwables::throwables_spawn(g, item_kind, [h[0], m::max(0.25, h[1]), h[2]], [v0, v1, v2], Some(opts));
    if kind == DropFromHandKind::Rifle {
        let rate = crate::throwables::item_tumble_rate(g, it) * 0.55;
        crate::throwables::item_set_tumble_rate(g, it, rate);
        let spin = crate::throwables::item_spin(g, it) * 0.5;
        crate::throwables::item_set_spin(g, it, spin);
    }
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Dropped { kind: item_kind, quiet });
    it
}

/// A round finds the player: one hit less, and at none he goes down.   (combat.ts hitPlayer)
pub fn hit_player(g: &mut crate::game::Game, from_dir: V3) {
    let p = g.player;
    if g.pl[p].down { return; }
    g.pl[p].hits_left = m::max(0.0, g.pl[p].hits_left - 1.0);
    if g.pl[p].hits_left == 0.0 {
        let in_hand = crate::game::player_gun_in_hand(g, p);
        if g.pl[p].dragging.is_some() { crate::game::game_toggle_drag(g, None); }
        if g.pl[p].holding.is_some() { crate::game::game_free_held(g, crate::game::GameFreeHeldWhy::Shot); }
        let c = g.pl[p].ch;
        g.pl[p].down = true; g.pl[p].sprinting = false; g.pl[p].crouch = false;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        if g.pl[p].wall.is_some() {
            g.pl[p].wall = None;
            let an = &mut g.ch[c].anim;
            an.stance = crate::character::Stance::None; an.look_yaw_extra = 0.0; an.wall_take = -1.0; an.wall_hit = 0.0; an.stack_press = 0.0; an.stance_in = 0.0;
        }
        let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
        crate::weapons::firearm_cancel_reload(g, pistol);
        crate::weapons::firearm_cancel_reload(g, rifle);
        crate::character::anim_cancel_reload(g, c);
        g.pl[p].throw_held = false;
        g.pl[p].pending_throw = None;
        if let Some(gun) = in_hand {
            let def = g.fa[gun].def;
            let kind = drop_kind(g, def);
            let along = g.ch[c].gun_dir;
            let suppressed = row(g, def).suppressed;
            drop_from_hand(g, c, kind, along, from_dir, Some(suppressed), None, None);
            if gun == rifle { g.pl[p].rifle_gone = true; }
        }
        ragdollize(g, c, from_dir, false);
        let elapsed = g.mission.stats.elapsed;
        let ck = crate::savestate::checkpoints_newest(g, Some(true), Some(elapsed));
        let back = match &ck { Some(info) => crate::savestate::checkpoints_is_this_run(g, info), None => false };
        crate::game::game_msg(g, if back { "you're down. press Enter to go back to the last checkpoint, or Shift+Enter to restart the mission" } else { "you're down. press Enter to restart the mission" });
    } else {
        let text = if g.pl[p].hits_left == 1.0 { "hit \u{2014} one more and you are down" } else { "you were hit" };
        crate::game::game_msg(g, text);
    }
}

// ---------------------------------------------------------------- what the men carry, as boxes (world.raycast sees them) and the words the figure is told

/// Held and worn props as small boxes, and what each figure is told it holds. The rigKeys call is left out while a made box has no table (see notes).   (combat.ts handProps)
pub fn hand_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    hand_props_raw(g, out);
}

/// Motion identity: every rig box from `n0` on gets its man's id and its ordinal among his boxes this frame. Owner 0 stays on the index rule.   (combat.ts rigKeys)
pub fn rig_keys(g: &mut crate::game::Game, out: &mut Vec<BoxId>, n0: f64) {
    let mut ord = hand_ord();
    let mut k = n0 as usize;
    while k < out.len() {
        let b = out[k];
        k += 1;
        let o = js::to_i32(seam_box_owner(g, b)) & 0xff;
        if o == 0 { continue; }
        let at = ord[o as usize];
        ord[o as usize] = at.wrapping_add(1);
        seam_box_set_mkey(g, b, crate::scene_boxes::mkey().rig + ((o << 8) as f64) + m::min(255.0, at as f64));
    }
}

/// Each player's own kit, then each man's, then the hostages' wrist ties.   (combat.ts handPropsRaw)
pub fn hand_props_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    if g.seats.len() == 1 {
        player_props(g, out);
    } else {
        for pl in men_here(g) { crate::seats::with_seat(g, pl, &mut |g| player_props(g, out)); }
    }
    let mut i = 0usize;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if crate::fogofwar::fog_of_war_hidden(g, gd) {
            let c = g.gd[gd].ch;
            g.ch[c].in_hand = None;
            continue;
        }
        guard_props(g, gd, out);
    }
    for cv in crate::civilians::civilians_shown(g) {
        let c = g.gd[cv].ch;
        let tied = g.ch[c].anim.bound == 1.0 && g.ch[c].anim.mv2_shown.bound > 0.6 && g.gd[cv].state != crate::game::GuardState::Dead;
        if tied && !crate::fogofwar::fog_of_war_hidden(g, cv) { crate::rig_props::wrist_tie_boxes(g, c, out); }
    }
}

/// What the player carries and wears, as boxes, and the fist's and the figure's words for this frame.   (combat.ts playerProps)
pub fn player_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    use crate::kitanim::KitShowVRifle as Rf;
    use crate::rig_props::{RigOpts, RigOptsRifle};
    let p = g.player;
    let mine0 = out.len();
    let c = g.pl[p].ch;
    let busy = g.ch[c].anim.hide_held_item;
    let kind_sel = crate::game::player_canister_sel(g, p);
    let pistol_up = crate::game::player_pistol_up(g, p);
    let pistol_up_now = pistol_up && !busy;
    let down = g.pl[p].down;
    let sprinting = g.pl[p].sprinting;
    let held_now: Option<crate::game::ThrowKind> = match kind_sel {
        Some(ks) if !(down || busy || g.pl[p].holstered || g.pl[p].dragging.is_some() || sprinting) => {
            if crate::character::anim_in_throw(g, c) {
                if crate::character::anim_throwing(g, c) {
                    match g.pl[p].pending_throw.as_ref().map(|t| t.kind) {
                        Some(k) => Some(k),
                        None => if crate::character::anim_throw_time(g, c) >= crate::character::THROW_RELEASE { None } else { Some(ks) },
                    }
                } else { None }
            } else if crate::game::game_throw_count(g, ks) > 0.0 { Some(ks) } else { None }
        }
        _ => None,
    };
    let bare = crate::close::close_bare(g);
    g.ch[c].anim.no_knife = bare;
    let holding = g.pl[p].holding.clone();
    let knife_now = !bare && {
        let hold_arm = match &holding {
            Some(h) if h.variant == crate::character::HoldVariant::Arm && !down && !g.ch[c].anim.hold_gun_side => !crate::disarm::disarm_hand(g, h),
            _ => false,
        };
        hold_arm || (g.ch[c].anim.knife_shown && !down && holding.is_none() && !pistol_up_now)
    };
    let rifle_up = crate::game::player_rifle_up(g, p);
    let rifle_now: Option<Rf> = if !g.pl[p].has_rifle { None }
        else if down { if g.pl[p].rifle_gone { None } else { Some(Rf::Slung) } }
        else if rifle_up && !busy { Some(Rf::Hand) } else { Some(Rf::Slung) };
    let pistol_now = pistol_up && !down && !busy;
    let mut show = crate::kitanim::KitShowV { pistol_in_hand: pistol_now, rifle: rifle_now, held: held_now, knife: knife_now };
    let kit = crate::kitanim::kit_show(g, &mut show);
    let crate::kitanim::KitShowV { pistol_in_hand, rifle, held, knife } = show;
    let lens_up = crate::kitanim::kit_gun_up(g);
    let fit = !g.touring;
    g.ch[c].anim.grip_can = fit && held.is_some();
    let big_s = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id: g.ch[c].id });
    let big_over = rifle == Some(Rf::Slung) && !crate::kitanim::kit_sling_front(g);
    let is_hammer = big_s == Some(crate::bigkit::BigCarried::Hammer);
    let side = if is_hammer { hammer_side(g, c) } else { None };
    g.ch[c].anim.big_side = side.map(|s| crate::character::AnimBigSide { o: s.o, z: s.z });
    let clear = if is_hammer && big_over { crate::rig_props::big_clear_of(g, c, crate::rig_props::BigClearOfKind::Hammer, None) } else { 0.0 };
    g.ch[c].anim.big_clear = clear;
    let in_hand = !down;
    g.ch[c].in_hand = if !in_hand { None }
        else if pistol_in_hand { Some(crate::inhand::InHandWord::Pistol) }
        else if rifle == Some(Rf::Hand) && !fit { Some(crate::rig_props::long_hold(crate::rig_props::LongHoldWeapon::Sc20k, None)) }
        else { None };
    g.ch[c].anim.grip[1] = held.is_some();

    let goggles = crate::kit::kit_you_nvg(g, p);
    let (pistol, gun_rifle) = (g.pl[p].pistol, g.pl[p].rifle);
    let light_on = g.fa[pistol].light_on && pistol_in_hand && !sprinting && lens_up;
    let rifle_black = crate::cast::sneak_suit(g, c);
    let rifle_light = rifle == Some(Rf::Hand) && g.fa[gun_rifle].light_on && !sprinting && lens_up;
    let sling = rifle == Some(Rf::Slung) && crate::kitanim::kit_sling_front(g);
    let sling_rest = if sling { crate::kitanim::kit_sling_rest(g) } else { None };
    let opts = RigOpts {
        goggles,
        nv: g.pl[p].nv,
        slot: if pistol_in_hand { 1.0 } else { 0.0 },
        light_on,
        is_guard: false,
        holster: Some(!(down && pistol_up)),
        held,
        knife: Some(knife),
        rifle: rifle.map(|r| match r { Rf::Hand => RigOptsRifle::Hand, Rf::Slung => RigOptsRifle::Slung }),
        rifle_black: Some(rifle_black),
        rifle_light: Some(rifle_light),
        kit,
        fit: if fit { Some(true) } else { None },
        in_hand: if in_hand { Some(true) } else { None },
        sling_front: if sling { Some(true) } else { None },
        sling_rest,
        nv_k: g.ch[c].anim.nv_shown,
        big: big_s.map(|b| match b { crate::bigkit::BigCarried::Shotgun => crate::rig_props::RigOptsBig::Shotgun, crate::bigkit::BigCarried::Hammer => crate::rig_props::RigOptsBig::Hammer }),
        big_k: if big_s.is_some() { Some(if down { 0.0 } else { g.ch[c].anim.big_k }) } else { None },
        big_clear: if big_s.is_some() { Some(big_over) } else { None },
        ..Default::default()
    };
    crate::rig_props::rig_boxes(g, c, opts, out, None, None);
    crate::handwork::work_props(g, c, out);
    if crate::rails::rails_sky_on(g) { out.truncate(mine0); }
}

/// Where the hammer's grip point and handle are at the side of the pack this man wears, in his chest joint's space. None: no pack.   (combat.ts hammerSide)
pub fn hammer_side(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::HammerSide> {
    let p = crate::rig_props::pack_side_bind(g, c_2, crate::rig_props::PackSideBindWhat::Hammer, None)?;
    let l = seam_side_local(g, c_2, p)?;
    Some(HammerSide { o: l.o, z: l.z })
}

/// What one guard or unit carries and wears, as boxes.   (combat.ts guardProps)
pub fn guard_props(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) {
    let c = g.gd[gd].ch;
    let big = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id: g.ch[c].id });
    let stow = crate::workstow::stow_hang(g, gd);
    let big = match big {
        Some(b) => b,
        None => { guard_kit(g, gd, out, stow); hung_side(g, gd, out); return; }
    };
    let alive = g.gd[gd].state != crate::game::GuardState::Dead;
    let side = if big == crate::bigkit::BigCarried::Hammer { hammer_side(g, c) } else { None };
    g.ch[c].anim.big_side = side.map(|s| crate::character::AnimBigSide { o: s.o, z: s.z });
    let hang = stow || (alive && g.ch[c].anim.big_tool.is_some() && g.ch[c].anim.hide_held_item);
    guard_kit(g, gd, out, hang);
    let slung_out = g.gd[gd].slung.as_ref().filter(|s| s.out).map(|s| s.own);
    if let (crate::bigkit::BigCarried::Shotgun, true, Some(own)) = (big, alive, slung_out) {
        let def = g.fa[own].def;
        let weapon = match row(g, def).rig {
            crate::weapons::WeaponDefRig::Carbine => Some(crate::rig_props::HungBoxesWeapon::Carbine),
            crate::weapons::WeaponDefRig::Sc20k => Some(crate::rig_props::HungBoxesWeapon::Sc20k),
            _ => None,
        };
        match weapon {
            Some(w) => { let black = crate::cast::sneak_suit(g, c); crate::rig_props::hung_boxes(g, c, w, out, Some(crate::rig_props::rig()), Some(black), None); }
            None => hung_side(g, gd, out),
        }
        return;
    }
    hung_side(g, gd, out);
    let k = if alive { g.ch[c].anim.big_k } else { 0.0 };
    let kind = match big { crate::bigkit::BigCarried::Shotgun => crate::rig_props::BigBoxesKind::Shotgun, crate::bigkit::BigCarried::Hammer => crate::rig_props::BigBoxesKind::Hammer };
    crate::rig_props::big_boxes(g, c, kind, k, out, None, None, None);
}

/// A man throwing with his free hand, the long gun still in the other: the grenade in the left fist until it leaves him.   (combat.ts freeCan)
pub fn free_can(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) {
    let c = g.gd[gd].ch;
    if crate::character::anim_throw_off(g, c) && crate::frag::frag_in_fist(g, gd) {
        crate::rig_props::free_can_boxes(g, c, crate::game::ThrowKind::Frag, out, None);
    }
}

/// A teammate with his sidearm out: the long gun he let go of hangs down his front.   (combat.ts hungSide)
pub fn hung_side(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) {
    if let Some(own) = crate::gunslots::side_hung(g, gd) {
        let c = g.gd[gd].ch;
        let w = match own { crate::gunslots::SideHung::Carbine => crate::rig_props::HungBoxesWeapon::Carbine, crate::gunslots::SideHung::Sc20k => crate::rig_props::HungBoxesWeapon::Sc20k };
        let black = crate::cast::sneak_suit(g, c);
        crate::rig_props::hung_boxes(g, c, w, out, Some(crate::rig_props::rig()), Some(black), None);
    }
}

/// A man's row without the big slot's tool. `hang`: his own gun is out of his hands.   (combat.ts guardKit)
pub fn guard_kit(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>, hang: bool) {
    use crate::guards::RigOf;
    use crate::handwork::{SwapHeld, SwapHeldHeld};
    use crate::rig_props::{rig, RigOpts, RigOptsRifle, RigOptsWeapon};
    let c = g.gd[gd].ch;
    g.ch[c].in_hand = None;
    let team = g.gd[gd].team;
    let trained = g.gd[gd].trained;
    let ho = if crate::char_look::side_of_team(team, Some(trained)) == crate::anim_gltf::CastSide::Defender { g.k.inhand.holster_out } else { 0.0 };
    let has_ho = js::truthy(ho);
    let hr = if has_ho { let mut h = rig().holster; h.out = h.out + ho; Some(h) } else { None };
    let holster_out = if has_ho { Some(ho) } else { None };
    let rk = crate::guards::rig_of(g, gd);
    let carbine = rk == RigOf::Carbine;
    if g.gd[gd].state == crate::game::GuardState::Dead {
        if crate::nvg::nvg_worn(g, gd) { crate::rig_props::goggles_boxes(g, c, false, out, &mut rig().goggles, None, None); }
        let dropped = g.gd[gd].dropped;
        let on_floor = match dropped {
            Some(it) => { let k = crate::throwables::item_kind(g, it); k == crate::throwables::ItemKind::Torch || k == crate::throwables::ItemKind::Rifle }
            None => false,
        };
        if on_floor && !crate::runner::runner_bare(g, gd) {
            let r = rig();
            if rk == RigOf::Sc20k {
                crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.pistol.slide, grip: r.pistol.grip, suppressor: Some(r.pistol.suppressor) }, None, None);
            } else {
                crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.guard_pistol.slide, grip: r.guard_pistol.grip, suppressor: None }, None, hr);
            }
        }
        return;
    }
    let fl = g.gd[gd].flashlight;
    let rule = crate::guards::in_hand(g, gd);
    let rule_word = rule.map(|r| match r { crate::guards::InHand::Torch => SwapHeldHeld::Torch, crate::guards::InHand::Weapon => SwapHeldHeld::Weapon });
    let held = crate::handwork::swap_held(g, gd, rule_word);
    let torch_held = held == Some(SwapHeld::Torch);
    if rule.is_some() && !torch_held && rk == RigOf::Pistol && !crate::kit::kit_pistol(g, gd) && g.gd[gd].torch_lost_at < 0.0 {
        crate::handwork::belt_torch(g, c, out);
    }
    if held.is_none() && g.gd[gd].gun_lost_at >= 0.0 && rk == RigOf::Pistol && !crate::kit::kit_pistol(g, gd) && g.gd[gd].torch_lost_at < 0.0 {
        crate::handwork::belt_torch(g, c, out);
    }
    if held.is_none() && ((crate::runner::runner_fled(g, gd) && (!carbine || crate::runner::runner_bare(g, gd))) || (rk == RigOf::Pistol && g.gd[gd].gun_lost_at >= 0.0)) { return; }
    if held.is_none() {
        let r = rig();
        crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.guard_pistol.slide, grip: r.guard_pistol.grip, suppressor: None }, None, hr);
        return;
    }
    let def = g.fa[g.gd[gd].weapon].def;
    if hang {
        if rk == RigOf::Sc20k {
            let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
            let black = crate::cast::sneak_suit(g, c);
            let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), rifle: Some(RigOptsRifle::Slung), sling_front: Some(true), rifle_black: Some(black), ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None, None);
        } else if crate::kit::kit_pistol(g, gd) && row(g, def).rig == crate::weapons::WeaponDefRig::Pistol {
            let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
            let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None, None);
        } else {
            let o = RigOpts { goggles: false, nv: false, slot: 1.0, light_on: false, is_guard: true, weapon: Some(if carbine { RigOptsWeapon::Carbine } else { RigOptsWeapon::Pistol }), hang: Some(true), holster_out, ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None, None);
            if js::truthy(crate::kit::kit_index(g, gd)) && crate::nvg::nvg_worn(g, gd) {
                let nv = crate::nvg::nvg_down(g, gd);
                crate::rig_props::goggles_boxes(g, c, nv, out, &mut rig().goggles, None, None);
            }
        }
        return;
    }
    if rk == RigOf::Sc20k {
        g.ch[c].in_hand = Some(crate::rig_props::long_hold(crate::rig_props::LongHoldWeapon::Sc20k, None));
        let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
        let black = crate::cast::sneak_suit(g, c);
        let light = seam_light_enabled(g, fl);
        let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), rifle: Some(RigOptsRifle::Hand), rifle_black: Some(black), rifle_light: Some(light), ..Default::default() };
        crate::rig_props::rig_boxes(g, c, o, out, None, None);
        free_can(g, gd, out);
        return;
    }
    if crate::kit::kit_pistol(g, gd) && row(g, def).rig == crate::weapons::WeaponDefRig::Pistol {
        g.ch[c].in_hand = Some(crate::inhand::InHandWord::Pistol);
        let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
        let light_on = seam_light_enabled(g, fl);
        let o = RigOpts { goggles, nv, slot: 1.0, light_on, is_guard: false, in_hand: Some(true), fit: Some(true), ..Default::default() };
        crate::rig_props::rig_boxes(g, c, o, out, None, None);
        return;
    }
    let ih = !carbine && !torch_held;
    let beam = g.gd[gd].beam_dir;
    g.ch[c].in_hand = Some(if carbine { crate::rig_props::long_hold(crate::rig_props::LongHoldWeapon::Carbine, None) }
        else if torch_held { crate::rig_props::torch_hold(beam, None) }
        else { crate::inhand::InHandWord::Pistol });
    let light_on = seam_light_enabled(g, fl);
    let lens = seam_light_color(g, fl);
    let o = RigOpts {
        goggles: false, nv: false, slot: 1.0, light_on, is_guard: true,
        torch: Some(torch_held), beam_dir: Some(beam), lens_color: Some(lens),
        weapon: Some(if carbine { RigOptsWeapon::Carbine } else { RigOptsWeapon::Pistol }),
        in_hand: if ih { Some(true) } else { None },
        holster_out,
        ..Default::default()
    };
    crate::rig_props::rig_boxes(g, c, o, out, None, None);
    if js::truthy(crate::kit::kit_index(g, gd)) && crate::nvg::nvg_worn(g, gd) {
        let nv = crate::nvg::nvg_down(g, gd);
        crate::rig_props::goggles_boxes(g, c, nv, out, &mut rig().goggles, None, None);
    }
    if carbine { free_can(g, gd, out); }
}

// ---------------------------------------------------------------- what only a viewer's picture asks for

/// What the men the fog of war leaves out carry, for a viewer's picture without the fog. Never part of a step.   (combat.ts hiddenProps)
pub fn hidden_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let n0 = out.len() as f64;
    let mut i = 0usize;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if crate::fogofwar::fog_of_war_hidden(g, gd) { guard_props(g, gd, out); }
    }
    for cv in crate::civilians::civilians_shown(g) {
        let c = g.gd[cv].ch;
        let tied = g.ch[c].anim.bound == 1.0 && g.ch[c].anim.mv2_shown.bound > 0.6 && g.gd[cv].state != crate::game::GuardState::Dead;
        if tied && crate::fogofwar::fog_of_war_hidden(g, cv) { crate::rig_props::wrist_tie_boxes(g, c, out); }
    }
    rig_keys(g, out, n0);
}

/// The infrared lines the run's men really have on, for a viewer's picture through a tube the run's own picture did not have. Never part of a step.   (combat.ts irBeams)
pub fn ir_beams(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    crate::irlaser::ir_view_as(g, true, &mut |g| {
        let mut i = 0usize;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if g.gd[gd].state == crate::game::GuardState::Dead || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
            if crate::irlaser::laser_band(g, gd) != crate::irlaser::LaserBand::Ir { continue; }
            let rule = crate::guards::in_hand(g, gd).map(|r| match r { crate::guards::InHand::Torch => crate::handwork::SwapHeldHeld::Torch, crate::guards::InHand::Weapon => crate::handwork::SwapHeldHeld::Weapon });
            if crate::handwork::swap_held(g, gd, rule).is_none() || !crate::irlaser::laser_on(g, gd) { continue; }
            laser_boxes(g, gd, out);
        }
        crate::irlaser::sam_laser_boxes(g, out);
    });
}

/// A hunting man's weapon laser: a thin emissive line from the muzzle to what it meets, and a dot there. LASER.enemy is off on the page: a man hostile to the player has none.   (combat.ts laserBoxes)
pub fn laser_boxes(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) {
    let team = g.gd[gd].team;
    if crate::teams::is_hostile(g, team, crate::teams::Team::Player) { return; }
    let ir = crate::irlaser::laser_band(g, gd) == crate::irlaser::LaserBand::Ir;
    if ir && !crate::irlaser::ir_viewer(g) { return; }
    let c = g.gd[gd].ch;
    let r = match laser_reach(g, c, Some(gd), Some(ir)) { Some(r) => r, None => return };
    let (from, dir, l, dot) = (r.from, r.dir, r.l, r.dot);
    if ir {
        if let Some(on) = r.on { if crate::irlaser::paints_own(g, gd, on) { return; } }
    }
    let b = crate::stature::drawn_beam(g, c, from, dir, l);
    if crate::fogofwar::fog_of_war_active(g) && g.fog_of_war.knobs.lasers && !crate::fogofwar::fog_of_war_on_team(g, gd) {
        if !crate::fogofwar::fog_of_war_seen(g, gd) { return; }
        for run in crate::fogofwar::fog_of_war_beam_runs(g, from, dir, l) {
            let (t0, t1) = (run[0], run[1]);
            if t1 - t0 > 0.02 { beam_boxes(g, v3::mad(b.from, b.dir, t0 * b.k), b.dir, (t1 - t0) * b.k, out, dot && t1 >= l - 1e-6, Some(ir), None); }
        }
        return;
    }
    beam_boxes(g, b.from, b.dir, l * b.k, out, dot, Some(ir), Some(true));
}

/// Where a weapon laser goes: from the muzzle along the bore to the first thing inside 30 m, and the man it ends on. None: no bore, or the muzzle is against what it meets.   (combat.ts laserReach)
pub fn laser_reach(g: &mut crate::game::Game, c_2: CharId, self_: Option<GuardId>, civ: Option<bool>) -> Option<crate::combat::LaserReach> {
    let civ = civ.unwrap_or(false);
    let dir = g.ch[c_2].gun_dir;
    let from = v3::mad(g.ch[c_2].muzzle, dir, 0.02);
    if !dir[0].is_finite() || v3::len(dir) < 0.5 { return None; }
    let mut l = 30.0;
    let mut on: Option<CharId> = None;
    let flags = crate::scene_boxes::box_flag();
    let skip = flags_or(flags.no_shadow, flags.glass);
    let owner = (js::to_i32(g.ch[c_2].id) & 0xff) as f64;
    let wh = crate::scene_boxes::box_world_raycast(g, from, dir, Some(l), Some(skip), Some(owner), None);
    if let Some(h) = &wh { l = h.t; }
    let ph = crate::props::prop_system_raycast(g, from, dir, l);
    if let Some(h) = &ph { l = m::min(l, h.t); }
    for pl in men_here(g) {
        let pc = g.pl[pl].ch;
        if c_2 != pc {
            let tp = crate::character::character_ray_hit(g, pc, from, dir, l);
            if tp >= 0.0 && tp <= l { l = tp; on = Some(pc); }
        }
    }
    let mut i = 0usize;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if Some(o) == self_ || g.gd[o].state == crate::game::GuardState::Dead || crate::fogofwar::fog_of_war_hidden(g, o) { continue; }
        let oc = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, oc, from, dir, l);
        if t >= 0.0 && t <= l { l = t; on = Some(oc); }
    }
    if civ {
        for cv in crate::civilians::civilian_bodies(g) {
            let cc = g.gd[cv].ch;
            let t = crate::character::character_ray_hit(g, cc, from, dir, l);
            if t >= 0.0 && t <= l { l = t; on = Some(cc); }
        }
    }
    if l < 0.05 { return None; }
    Some(LaserReach { from, dir, l, dot: wh.is_some() || ph.is_some() || l < 30.0, on })
}

/// A laser line from `from` along unit `dir` for `l` metres and (dot) the bead where it lands: emissive geometry no ray sees. The one place an infrared box is made.   (combat.ts beamBoxes)
pub fn beam_boxes(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, out: &mut Vec<BoxId>, dot: bool, ir: Option<bool>, air: Option<bool>) {
    let ir = ir.unwrap_or(false);
    let air = air.unwrap_or(false);
    if ir && !crate::irlaser::ir_viewer(g) { return; }
    let red: V3 = if ir { crate::irlaser::ir_tint() } else { [1.0, 0.06, 0.03] };
    let hw = m::max(0.0022, 0.45 * g.m_per_px);
    let gain = 0.0022 / hw;
    let up: V3 = if m::abs(dir[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let x = v3::normalize(v3::cross(up, dir));
    let y = v3::cross(dir, x);
    let flags = crate::scene_boxes::box_flag();
    let box_flags = flags_or(flags.no_shadow, flags.dynamic);
    let line = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: v3::mad(from, dir, l / 2.0),
        h: [hw, hw, l / 2.0],
        yaw: Some(v3::yaw_of(dir)),
        rot: Some(v3::quat::from_basis(x, y, dir)),
        albedo: Some(if ir { [0.0, 0.0, 0.0] } else { [0.02, 0.0, 0.0] }),
        emissive: Some(v3::scale(red, 9.0 * gain)),
        flags: Some(box_flags),
        ..Default::default()
    });
    crate::thinline::thin_note(g, line, 0.0022, 0.45, red[0] * 9.0, red[1] * 9.0, red[2] * 9.0);
    let shown = if ir { crate::irlaser::ir_note(g, line) } else { line };
    out.push(shown);
    if dot {
        let end = v3::mad(from, dir, l - 0.006);
        let bead = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: end,
            h: [0.011, 0.011, 0.011],
            yaw: Some(0.0),
            albedo: Some(if ir { [0.0, 0.0, 0.0] } else { [0.05, 0.0, 0.0] }),
            emissive: Some(v3::scale(red, 40.0)),
            flags: Some(box_flags),
            ..Default::default()
        });
        let shown = if ir { crate::irlaser::ir_note(g, bead) } else { bead };
        out.push(shown);
        if air { crate::thinline::thin_air(g, line, Some(bead)); }
    } else if air {
        crate::thinline::thin_air(g, line, None);
    }
}

// ---------------------------------------------------------------- the things in the air

/// The thrown things' step, then their fuses.   (combat.ts updateItems)
pub fn update_items(g: &mut crate::game::Game, dt: f64) {
    use crate::throwables::{self as t, ItemKind};
    t::throwables_update(g, dt);
    let mut i = 0usize;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        let kind = t::item_kind(g, it);
        if kind == ItemKind::Smoke {
            if !t::item_fired(g, it) {
                let f = t::item_fuse(g, it) - dt;
                t::item_set_fuse(g, it, f);
                if t::item_fuse(g, it) <= 0.0 { t::item_set_fired(g, it, true); start_canister(g, it); }
            } else if t::item_emit_t(g, it) > 0.0 {
                let e = t::item_emit_t(g, it) - dt;
                t::item_set_emit_t(g, it, e);
            }
        } else if kind == ItemKind::Flash && !t::item_fired(g, it) {
            let landed = t::item_bounces(g, it) > 0.0 || t::item_settled(g, it);
            if landed { let f = t::item_fuse(g, it) - dt; t::item_set_fuse(g, it, f); }
            if t::item_fuse(g, it) <= 0.0 || (!landed && t::item_age(g, it) > crate::effects::bang().max_air) {
                t::item_set_fired(g, it, true);
                detonate_stun(g, it, None, None);
            }
        } else if kind == ItemKind::Frag {
            crate::frag::frag_fuse(g, it, dt);
        }
    }
}

/// The attached solver's canister tuning, or None behind a stand-in without any.   (combat.ts smokeParams)
pub fn smoke_params(g: &mut crate::game::Game) -> Option<crate::smoke_types::CanisterParams> {
    g.smoke.params
}

/// A smoke can's fuse is out: it starts to pour, hisses, and the pop draws attention nearby.   (combat.ts startCanister)
pub fn start_canister(g: &mut crate::game::Game, it: ItemId) {
    let seconds: f64;
    if crate::effects::flags().vent_canister {
        let yaw = crate::throwables::item_heading(g, it);
        let at = crate::effects::vent_mouth(crate::throwables::item_pos(g, it), yaw);
        seconds = crate::effects::spawn_vent(g, at, yaw, None);
    } else {
        let p = smoke_params(g).unwrap_or(crate::smoke_types::CanisterParams { canister_duration: 32.0, canister_density: 95.0, canister_radius: 0.2, canister_temp: 1.6, canister_speed: 1.9 });
        seconds = p.canister_duration;
        let pos = v3::add(crate::throwables::item_pos(g, it), [0.0, 0.1, 0.0]);
        crate::smoke_types::smoke_system_like_emit(g, crate::smoke_types::SmokeEmitter { pos, radius: p.canister_radius, density: p.canister_density, ttl: p.canister_duration, look: None });
    }
    crate::throwables::item_set_emit_t(g, it, seconds);
    let pos = crate::throwables::item_pos(g, it);
    seam_audio_play(g, "smokeHiss", pos, 1.0, seconds);
    crate::game::game_msg(g, "smoke deployed");
    let e = crate::game::GameEvent { kind: crate::director::EventKind::Shot, pos: v3::copy(pos), time: g.time, loud: false, serial: g.serial(), ..Default::default() };
    g.events.push(e);
}
