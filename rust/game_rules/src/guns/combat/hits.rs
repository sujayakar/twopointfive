use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

// ---------------------------------------------------------------- panels and contact

fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, or: V3) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => or }
}

/// `c.bones.headTop?.[1] ?? c.pos[1] + over`: the crown's height.
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

/// `a | b` of two box flags.
fn flags_or(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }

/// The pistol poked out through a panel: gun shoulder and muzzle on opposite sides of one.
pub fn arm_through(g: &mut crate::game::Game, c_2: CharId) -> bool {
    let or = v3::add(g.ch[c_2].pos, [0.0, 1.4, 0.0]);
    let from = bone_or(g, c_2, crate::character::Bone::UpperArmR, or);
    let to = g.ch[c_2].muzzle;
    solid_between(g, from, to)
}

/// A panel between `from` and `to`: a static, or a leaf standing shut or as good as (SoundSeal). Short segments only.
pub fn solid_between(g: &Game, from: V3, to: V3) -> bool {
    if crate::scene_collision::segment_blocked_dynamic(g, from, to, Some(crate::scene_boxes::box_flag().sound_seal)) { return true; }
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < 1e-3 { return false; }
    let rd = v3::scale(d, 1.0 / l);
    let mut scratch: Vec<i32> = solid_scratch();
    let n = crate::scene_collision::gather(g, m::min(from[0], to[0]) - 0.1, m::min(from[2], to[2]) - 0.1, m::max(from[0], to[0]) + 0.1, m::max(from[2], to[2]) + 0.1, m::min(from[1], to[1]) - 0.1, m::max(from[1], to[1]) + 0.1, &mut scratch);
    let mut i = 0usize;
    while (i as f64) < n {
        let b = g.col.boxes[scratch[i] as usize];   // gather answers places in g.col.boxes, not IDs
        if crate::scene_boxes::ray_box(g, from, rd, b.into(), l).is_some() { return true; }
        i += 1;
    }
    false
}

/// May a contact shot from `a` count on `b`: no panel between their chests.
fn contact_clear(g: &mut crate::game::Game, a: CharId, b: CharId) -> bool {
    if v3::dist_xz(g.ch[a].pos, g.ch[b].pos) > 1.2 { return true; }
    let from = v3::add(g.ch[a].pos, [0.0, 1.1, 0.0]);
    let or = g.ch[b].pos;
    let h = bone_or(g, b, crate::character::Bone::Hips, or);
    let to: V3 = [h[0], 1.1, h[2]];
    !solid_between(g, from, to)
}

/// rayHit plus the contact shot: a muzzle already inside his body cylinder is a hit where it is. `contact` None is the TypeScript's `true`.
pub fn body_hit(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, tmax: f64, contact: Option<crate::combat::BodyHitContact>) -> f64 {
    let t = crate::character::character_ray_hit(g, c_2, ro, rd, tmax);
    if t >= 0.0 || !g.ch[c_2].alive || matches!(contact, Some(BodyHitContact::Entering)) { return t; }
    if !crate::character::character_inside_body(g, c_2, ro, Some(0.25)) { return -1.0; }
    let counts = match contact {
        None => true,
        Some(BodyHitContact::Entering) => false,
        // fireWeapon's own test: nothing solid nearer along the bore than his middle, no panel between the two chests
        Some(BodyHitContact::Muzzle { st0_t, muzzle, glass_arm, shooter }) => {
            let or = g.ch[c_2].pos;
            let h = bone_or(g, c_2, crate::character::Bone::Hips, or);
            let nearer = match st0_t { None => true, Some(t0) => t0 > m::hyp2(h[0] - muzzle[0], h[2] - muzzle[2]) };
            nearer && (glass_arm || contact_clear(g, shooter, c_2))
        }
        // a later leg through glass
        Some(BodyHitContact::LegCrown) => ro[1] <= crown_or(g, c_2, 1.8),
        // a ring or a 40 mm round
        Some(BodyHitContact::Flight { radius }) => ro[1] < crown_or(g, c_2, 1.75) + radius,
    };
    if counts { 0.0 } else { -1.0 }
}

// ---------------------------------------------------------------- a man goes down

/// A man goes down: `how` he went, `by` whose hand, `with` what; `quiet` is the physical side.
pub fn kill_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: Option<bool>, how: Option<crate::game::DownKind>, by: Option<crate::game::DownBy>, with: crate::game::With) {
    use crate::game::{DownBy, DownKind, GuardState};
    let quiet = quiet.unwrap_or(false);
    let how = how.unwrap_or(if quiet { DownKind::Struck } else { DownKind::Shot });
    let by = by.unwrap_or(DownBy::Player);
    if g.gd[gd].civ.is_some() { crate::civilians::civilian_down(g, gd, dir, quiet, how, by, with); return; }
    if crate::condition::down(g, g.gd[gd].ch) { return; }
    if crate::rails::rails_spare_2(g, gd) { return; }
    g.gd[gd].script = None;
    let held = crate::guards::in_hand(g, gd);
    if by == DownBy::Player {
        if g.team(gd).theirs() {
            crate::mission::tally(g, if how == DownKind::Shot { crate::mission::TallyK::Kills } else { crate::mission::TallyK::Knockouts });
        }
        crate::chronicle::chron_down(g, gd, how, with);
    }
    crate::guards::witness_kill(g, gd, how, by);
    crate::morale::morale_man_down(g, gd);
    crate::netquiet::net_quiet_fall(g, gd);
    if g.gd[gd].torch_lost_at < 0.0 && g.gd[gd].gun_lost_at < 0.0 {
        let fl = g.gd[gd].flashlight;
        g.gd[gd].light_lit_at_loss = g.tab.light[fl].enabled;
    }
    let now = g.time;
    {
        crate::mind::set_state(g, gd, GuardState::Dead, "combat"); g.gd[gd].speed = 0.0; g.gd[gd].died_at = now; g.gd[gd].down_kind = Some(how); g.gd[gd].down_by = Some(by);
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
    crate::character::character_ragdollize(g, c, true, dir, quiet);
}

/// What a dying hand lets go of: an item at the hand, carrying the body's motion plus a nudge along the round.
pub fn drop_from_hand(g: &mut crate::game::Game, c_2: CharId, kind: crate::combat::DropFromHandKind, along: V3, dir: V3, suppressed: Option<bool>, quiet: Option<bool>, from: Option<V3>) -> ItemId {
    let suppressed = suppressed.unwrap_or(false);
    let quiet = quiet.unwrap_or(false);
    let h = match from {
        Some(f) => f,
        None => { let or = v3::add(g.ch[c_2].pos, [0.0, 1.0, 0.0]); bone_or(g, c_2, crate::character::Bone::HandR, or) }
    };
    let vel = g.ch[c_2].vel;
    let r0 = crate::dice::draw_body(g, "combat::drop_from_hand#1", c_2);
    let v0 = vel[0] * 0.6 + dir[0] * 1.1 + (r0 - 0.5) * 0.6;
    let r1 = crate::dice::draw_body(g, "combat::drop_from_hand#2", c_2);
    let v1 = 0.9 + r1 * 0.6;
    let r2 = crate::dice::draw_body(g, "combat::drop_from_hand#3", c_2);
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
        let rate = g.tab.item[it].tumble_rate * 0.55;
        g.tab.item[it].tumble_rate = rate;
        let spin = g.tab.item[it].spin * 0.5;
        g.tab.item[it].spin = spin;
    }
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Dropped { kind: item_kind, quiet });
    it
}

/// A round finds the player: one hit less, and at none he goes down.
pub fn hit_player(g: &mut crate::game::Game, from_dir: V3) {
    let p = g.player;
    if crate::condition::down(g, g.pl[p].ch) { return; }
    g.pl[p].hits_left = m::max(0.0, g.pl[p].hits_left - 1.0);
    if g.pl[p].hits_left == 0.0 {
        let in_hand = crate::game::player_gun_in_hand(g, p);
        if g.pl[p].dragging().is_some() { crate::game::game_toggle_drag(g, None); }
        if g.pl[p].holding().is_some() { crate::game::game_free_held(g, crate::player::FreeHeldWhy::Shot); }
        let c = g.pl[p].ch;
        g.cond[g.pl[p].ch].down = true; g.pl[p].sprinting = false; g.pl[p].crouch = false;
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
        g.pl[p].pending_throw = None;
        if let Some(gun) = in_hand {
            let def = g.fa[gun].def;
            let kind = drop_kind(g, def);
            let along = g.ch[c].gun_dir;
            let suppressed = row(g, def).suppressed;
            drop_from_hand(g, c, kind, along, from_dir, Some(suppressed), None, None);
            if gun == rifle { g.pl[p].rifle_gone = true; }
        }
        crate::character::character_ragdollize(g, c, true, from_dir, false);
    }
}

// ---------------------------------------------------------------- what the men carry, as boxes (world.raycast sees them) and the words the figure is told

/// Held and worn props as small boxes, and what each figure is told it holds; then every rig box's motion key.
pub fn hand_props(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let n0 = out.len() as f64;
    hand_props_raw(g, out);
    rig_keys(g, out, n0);
}

/// Motion identity: every rig box from `n0` on gets its man's id and its ordinal among his boxes this frame. Owner 0 stays on the index rule.
fn rig_keys(g: &mut crate::game::Game, out: &Vec<AnyBoxId>, n0: f64) {
    let mut ord = hand_ord();
    let mut k = n0 as usize;
    while k < out.len() {
        let b = out[k];
        k += 1;
        let o = js::to_i32(g.tab[b].owner) & 0xff;
        if o == 0 { continue; }
        let at = ord[o as usize];
        ord[o as usize] = at.wrapping_add(1);
        g.tab.edit(b).mkey = Some(crate::scene_boxes::mkey().rig + ((o << 8) as f64) + m::min(255.0, at as f64));
    }
}

/// Each player's own kit, then each man's, then the hostages' wrist ties.
fn hand_props_raw(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    crate::seats::each_seat(g, &mut |g| player_props(g, out));
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
        let tied = g.ch[c].anim.bound == 1.0 && g.ch[c].anim.mv2_shown.bound > 0.6 && !crate::condition::down(g, g.gd[cv].ch);
        if tied && !crate::fogofwar::fog_of_war_hidden(g, cv) { crate::rig_props::wrist_tie_boxes(g, c, out); }
    }
}

/// What the player carries and wears, as boxes, and the fist's and the figure's words for this frame.
fn player_props(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    use crate::rig_props::RifleAt as Rf;
    use crate::rig_props::{RigOpts};
    let p = g.player;
    let c = g.pl[p].ch;
    let busy = g.ch[c].anim.hide_held_item;
    let kind_sel = crate::game::player_canister_sel(g, p);
    let pistol_up = crate::game::player_pistol_up(g, p);
    let down = crate::condition::down(g, g.pl[p].ch);
    let sprinting = g.pl[p].sprinting;
    let held_now: Option<crate::game::ThrowKind> = match kind_sel {
        Some(ks) if !(down || busy || g.pl[p].holstered || g.pl[p].dragging().is_some() || sprinting) => {
            if crate::character::anim_in_throw(g, c) {
                if crate::character::anim_throwing(g, c) {
                    match g.pl[p].pending_throw.as_ref().map(|t| t.kind) {
                        Some(k) => Some(k),
                        None => if crate::character::anim_throw_time(g, c) >= crate::character::THROW_RELEASE { None } else { Some(ks) },
                    }
                } else { None }
            } else if crate::game::game_throw_count(g, &crate::seats::here(g), ks) > 0.0 { Some(ks) } else { None }
        }
        _ => None,
    };
    g.ch[c].anim.no_knife = true;
    let rifle_up = crate::game::player_rifle_up(g, p);
    let rifle_now: Option<Rf> = if !g.pl[p].has_rifle { None }
        else if down { if g.pl[p].rifle_gone { None } else { Some(Rf::Slung) } }
        else if rifle_up && !busy { Some(Rf::Hand) } else { Some(Rf::Slung) };
    let pistol_now = pistol_up && !down && !busy;
    let mut show = crate::kitanim::KitShowV { pistol_in_hand: pistol_now, rifle: rifle_now, held: held_now, knife: false };
    let kit = crate::kitanim::kit_show(g, &mut show);
    let crate::kitanim::KitShowV { pistol_in_hand, rifle, held, knife } = show;
    let lens_up = crate::kitanim::kit_gun_up(g, &crate::seats::here(g));
    g.ch[c].anim.grip_can = held.is_some();
    let big_s = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id: g.ch[c].id });
    let big_over = rifle == Some(Rf::Slung) && !crate::kitanim::kit_sling_front(g);
    let is_hammer = big_s == Some(crate::kit::BigTool::Hammer);
    let side = if is_hammer { hammer_side(g, c) } else { None };
    g.ch[c].anim.big_side = side.map(|s| crate::character::AnimBigSide { o: s.o, z: s.z });
    let clear = if is_hammer && big_over { crate::rig_props::big_clear_of(g, c, crate::kit::BigTool::Hammer, None) } else { 0.0 };
    g.ch[c].anim.big_clear = clear;
    let in_hand = !down;
    g.ch[c].in_hand = if !in_hand { None }
        else if pistol_in_hand { Some(crate::inhand::in_pistol()) }
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
        rifle: rifle,
        rifle_black: Some(rifle_black),
        rifle_light: Some(rifle_light),
        kit,
        fit: Some(true),
        in_hand: if in_hand { Some(true) } else { None },
        sling_front: if sling { Some(true) } else { None },
        sling_rest,
        nv_k: g.ch[c].anim.nv_shown,
        big: big_s,
        big_k: if big_s.is_some() { Some(if down { 0.0 } else { g.ch[c].anim.big_k }) } else { None },
        big_clear: if big_s.is_some() { Some(big_over) } else { None },
        ..Default::default()
    };
    crate::rig_props::rig_boxes(g, c, opts, out, None);
    crate::handwork::work_props(g, c, out);
    crate::irlaser::sam_laser_boxes(g, out);
}

/// Where the hammer's grip point and handle are at the side of the pack this man wears, in his chest joint's space. None: no pack.
fn hammer_side(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::HammerSide> {
    let p = crate::rig_props::pack_side_bind(g, c_2, crate::rig_props::PackSideWhat::Hammer, None)?;
    let rig = g.ch[c_2].rig;
    let l = crate::packside::side_local(g, rig, p)?;
    Some(HammerSide { o: l.o, z: l.z })
}

/// What one guard or unit carries and wears, as boxes.
fn guard_props(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<AnyBoxId>) {
    let c = g.gd[gd].ch;
    let big = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id: g.ch[c].id });
    let stow = crate::workstow::stow_hang(g, gd);
    let big = match big {
        Some(b) => b,
        None => { guard_kit(g, gd, out, stow); hung_side(g, gd, out); return; }
    };
    let alive = !crate::condition::down(g, g.gd[gd].ch);
    let side = if big == crate::kit::BigTool::Hammer { hammer_side(g, c) } else { None };
    g.ch[c].anim.big_side = side.map(|s| crate::character::AnimBigSide { o: s.o, z: s.z });
    let hang = stow || (alive && g.ch[c].anim.big_tool.is_some() && g.ch[c].anim.hide_held_item);
    guard_kit(g, gd, out, hang);
    let slung_out = g.gd[gd].slung.as_ref().filter(|s| s.out).map(|s| s.own);
    if let (crate::kit::BigTool::Shotgun, true, Some(own)) = (big, alive, slung_out) {
        let def = g.fa[own].def;
        let weapon = match row(g, def).rig {
            crate::weapons::WeaponDefRig::Carbine => Some(crate::rig_props::LongRig::Carbine),
            crate::weapons::WeaponDefRig::Sc20k => Some(crate::rig_props::LongRig::Sc20k),
            _ => None,
        };
        match weapon {
            Some(w) => { let black = crate::cast::sneak_suit(g, c); crate::rig_props::hung_boxes(g, c, w, out, Some(crate::rig_props::rig()), Some(black)); }
            None => hung_side(g, gd, out),
        }
        return;
    }
    hung_side(g, gd, out);
    let k = if alive { g.ch[c].anim.big_k } else { 0.0 };
    let kind = big;
    crate::rig_props::big_boxes(g, c, kind, k, out, None, None);
}

/// A man throwing with his free hand, the long gun still in the other: the grenade in the left fist until it leaves him.
fn free_can(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<AnyBoxId>) {
    let c = g.gd[gd].ch;
    if crate::character::anim_throw_off(g, c) && crate::frag::frag_in_fist(g, gd) {
        crate::rig_props::free_can_boxes(g, c, crate::game::ThrowKind::Frag, out);
    }
}

/// A teammate with his sidearm out: the long gun he let go of hangs down his front.
fn hung_side(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<AnyBoxId>) {
    if let Some(own) = crate::gunslots::side_hung(g, gd) {
        let c = g.gd[gd].ch;
        let w = own;
        let black = crate::cast::sneak_suit(g, c);
        crate::rig_props::hung_boxes(g, c, w, out, Some(crate::rig_props::rig()), Some(black));
    }
}

/// A man's row without the big slot's tool. `hang`: his own gun is out of his hands.
fn guard_kit(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<AnyBoxId>, hang: bool) {
    use crate::rig_props::{rig, RigOpts};
    let c = g.gd[gd].ch;
    g.ch[c].in_hand = None;
    let team = g.team(gd);
    let trained = g.gd[gd].trained;
    let ho = if crate::char_look::side_of_team(team, Some(trained)) == crate::anim_gltf::CastSide::Defender { g.k.inhand.holster_out } else { 0.0 };
    let has_ho = js::truthy(ho);
    let hr = if has_ho { let mut h = rig().holster; h.out = h.out + ho; Some(h) } else { None };
    let holster_out = if has_ho { Some(ho) } else { None };
    let rk = crate::guards::rig_of(g, gd);
    let carbine = rk == crate::rig_props::RigWeapon::Carbine;
    if crate::condition::down(g, g.gd[gd].ch) {
        if crate::nvg::nvg_worn(g, gd) { crate::rig_props::goggles_boxes(g, c, false, out, rig().goggles, None); }
        let dropped = g.gd[gd].dropped;
        let on_floor = match dropped {
            Some(it) => { let k = g.tab.item[it].kind; k == crate::throwables::ItemKind::Torch || k == crate::throwables::ItemKind::Rifle }
            None => false,
        };
        if on_floor && !crate::runner::runner_bare(g, gd) {
            let r = rig();
            if rk == crate::rig_props::RigWeapon::Sc20k {
                crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.pistol.slide, grip: r.pistol.grip, suppressor: Some(r.pistol.suppressor) }, None);
            } else {
                crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.guard_pistol.slide, grip: r.guard_pistol.grip, suppressor: None }, hr);
            }
        }
        return;
    }
    let fl = g.gd[gd].flashlight;
    let rule = crate::guards::in_hand(g, gd);
    let rule_word = rule;
    let held = crate::handwork::swap_held(g, gd, rule_word);
    let torch_held = held == Some(crate::guards::InHand::Torch);
    if rule.is_some() && !torch_held && rk == crate::rig_props::RigWeapon::Pistol && !crate::kit::kit_pistol(g, gd) && g.gd[gd].torch_lost_at < 0.0 {
        crate::handwork::belt_torch(g, c, out);
    }
    if held.is_none() && g.gd[gd].gun_lost_at >= 0.0 && rk == crate::rig_props::RigWeapon::Pistol && !crate::kit::kit_pistol(g, gd) && g.gd[gd].torch_lost_at < 0.0 {
        crate::handwork::belt_torch(g, c, out);
    }
    if held.is_none() && ((g.gd[gd].fled && (!carbine || crate::runner::runner_bare(g, gd))) || (rk == crate::rig_props::RigWeapon::Pistol && g.gd[gd].gun_lost_at >= 0.0)) { return; }
    if held.is_none() {
        let r = rig();
        crate::rig_props::holster_boxes(g, c, out, crate::rig_props::HolsterBoxesParts { slide: r.guard_pistol.slide, grip: r.guard_pistol.grip, suppressor: None }, hr);
        return;
    }
    let def = g.fa[g.gd[gd].weapon].def;
    if hang {
        if rk == crate::rig_props::RigWeapon::Sc20k {
            let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
            let black = crate::cast::sneak_suit(g, c);
            let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), rifle: Some(crate::rig_props::RifleAt::Slung), sling_front: Some(true), rifle_black: Some(black), ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None);
        } else if crate::kit::kit_pistol(g, gd) && row(g, def).rig == crate::weapons::WeaponDefRig::Pistol {
            let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
            let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None);
        } else {
            let o = RigOpts { goggles: false, nv: false, slot: 1.0, light_on: false, is_guard: true, weapon: Some(if carbine { crate::rig_props::RigWeapon::Carbine } else { crate::rig_props::RigWeapon::Pistol }), hang: Some(true), holster_out, ..Default::default() };
            crate::rig_props::rig_boxes(g, c, o, out, None);
            if js::truthy(crate::kit::kit_index(g, gd)) && crate::nvg::nvg_worn(g, gd) {
                let nv = crate::nvg::nvg_down(g, gd);
                crate::rig_props::goggles_boxes(g, c, nv, out, rig().goggles, None);
            }
        }
        return;
    }
    if rk == crate::rig_props::RigWeapon::Sc20k {
        g.ch[c].in_hand = Some(crate::rig_props::long_hold(crate::rig_props::LongRig::Sc20k, None));
        let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
        let black = crate::cast::sneak_suit(g, c);
        let light = g.tab.light[fl].enabled;
        let o = RigOpts { goggles, nv, slot: 0.0, light_on: false, is_guard: false, holster: Some(true), rifle: Some(crate::rig_props::RifleAt::Hand), rifle_black: Some(black), rifle_light: Some(light), ..Default::default() };
        crate::rig_props::rig_boxes(g, c, o, out, None);
        free_can(g, gd, out);
        return;
    }
    if crate::kit::kit_pistol(g, gd) && row(g, def).rig == crate::weapons::WeaponDefRig::Pistol {
        g.ch[c].in_hand = Some(crate::inhand::in_pistol());
        let (goggles, nv) = (crate::nvg::nvg_worn(g, gd), crate::nvg::nvg_down(g, gd));
        let light_on = g.tab.light[fl].enabled;
        let o = RigOpts { goggles, nv, slot: 1.0, light_on, is_guard: false, in_hand: Some(true), fit: Some(true), ..Default::default() };
        crate::rig_props::rig_boxes(g, c, o, out, None);
        return;
    }
    let ih = !carbine && !torch_held;
    let beam = g.gd[gd].beam_dir;
    g.ch[c].in_hand = Some(if carbine { crate::rig_props::long_hold(crate::rig_props::LongRig::Carbine, None) }
        else if torch_held { crate::rig_props::torch_hold(beam, None) }
        else { crate::inhand::in_pistol() });
    let light_on = g.tab.light[fl].enabled;
    let lens = g.tab.light[fl].color;
    let o = RigOpts {
        goggles: false, nv: false, slot: 1.0, light_on, is_guard: true,
        torch: Some(torch_held), beam_dir: Some(beam), lens_color: Some(lens),
        weapon: Some(if carbine { crate::rig_props::RigWeapon::Carbine } else { crate::rig_props::RigWeapon::Pistol }),
        in_hand: if ih { Some(true) } else { None },
        holster_out,
        ..Default::default()
    };
    crate::rig_props::rig_boxes(g, c, o, out, None);
    if js::truthy(crate::kit::kit_index(g, gd)) && crate::nvg::nvg_worn(g, gd) {
        let nv = crate::nvg::nvg_down(g, gd);
        crate::rig_props::goggles_boxes(g, c, nv, out, rig().goggles, None);
    }
    if carbine { free_can(g, gd, out); }
}

// ---------------------------------------------------------------- what only a viewer's picture asks for

/// What the men the fog of war leaves out carry, for a viewer's picture without the fog. Never part of a step.
pub fn hidden_props(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let n0 = out.len() as f64;
    let mut i = 0usize;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if crate::fogofwar::fog_of_war_hidden(g, gd) { guard_props(g, gd, out); }
    }
    for cv in crate::civilians::civilians_shown(g) {
        let c = g.gd[cv].ch;
        let tied = g.ch[c].anim.bound == 1.0 && g.ch[c].anim.mv2_shown.bound > 0.6 && !crate::condition::down(g, g.gd[cv].ch);
        if tied && crate::fogofwar::fog_of_war_hidden(g, cv) { crate::rig_props::wrist_tie_boxes(g, c, out); }
    }
    rig_keys(g, out, n0);
}

/// Where a weapon laser goes: from the muzzle along the bore to the first thing inside 30 m. None: no bore, or the muzzle is against what it meets.
pub fn laser_reach(g: &mut crate::game::Game, c_2: CharId, self_: Option<GuardId>, civ: Option<bool>) -> Option<crate::combat::LaserReach> {
    let civ = civ.unwrap_or(false);
    let dir = g.ch[c_2].gun_dir;
    let from = v3::mad(g.ch[c_2].muzzle, dir, 0.02);
    if !dir[0].is_finite() || v3::len(dir) < 0.5 { return None; }
    let mut l = 30.0;
    let flags = crate::scene_boxes::box_flag();
    let skip = flags_or(flags.no_shadow, flags.glass);
    let owner = (js::to_i32(g.ch[c_2].id) & 0xff) as f64;
    let wh = crate::scene_boxes::box_world_raycast(g, from, dir, Some(l), Some(skip), Some(owner), None);
    if let Some(h) = &wh { l = h.t; }
    let ph = crate::props::prop_system_raycast(g, from, dir, l);
    if let Some(h) = &ph { l = m::min(l, h.t); }
    for pl in crate::seats::men(g) {
        let pc = g.pl[pl].ch;
        if c_2 != pc {
            let tp = crate::character::character_ray_hit(g, pc, from, dir, l);
            if tp >= 0.0 && tp <= l { l = tp; }
        }
    }
    let mut i = 0usize;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if Some(o) == self_ || crate::condition::down(g, g.gd[o].ch) || crate::fogofwar::fog_of_war_hidden(g, o) { continue; }
        let oc = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, oc, from, dir, l);
        if t >= 0.0 && t <= l { l = t; }
    }
    if civ {
        for cv in crate::civilians::civilian_bodies(g) {
            let cc = g.gd[cv].ch;
            let t = crate::character::character_ray_hit(g, cc, from, dir, l);
            if t >= 0.0 && t <= l { l = t; }
        }
    }
    if l < 0.05 { return None; }
    Some(LaserReach { from, dir, l, dot: wh.is_some() || ph.is_some() || l < 30.0 })
}

/// A laser line from `from` along unit `dir` for `l` metres and (dot) the bead where it lands: emissive geometry no ray sees. The one place an infrared box is made.
pub fn beam_boxes(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, out: &mut Vec<AnyBoxId>, dot: bool, ir: Option<bool>, air: Option<bool>) {
    let ir = ir.unwrap_or(false);
    let air = air.unwrap_or(false);
    if ir && !crate::irlaser::ir_viewer(g) { return; }
    let red: V3 = if ir { crate::irlaser::ir_tint() } else { [1.0, 0.06, 0.03] };
    const HW: f64 = 0.0022;
    let up: V3 = if m::abs(dir[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let x = v3::normalize(v3::cross(up, dir));
    let y = v3::cross(dir, x);
    let flags = crate::scene_boxes::box_flag();
    let box_flags = flags_or(flags.no_shadow, flags.dynamic);
    let line = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
        c: v3::mad(from, dir, l / 2.0),
        h: [HW, HW, l / 2.0],
        yaw: Some(v3::yaw_of(dir)),
        rot: Some(v3::quat::from_basis(x, y, dir)),
        albedo: Some(if ir { [0.0, 0.0, 0.0] } else { [0.02, 0.0, 0.0] }),
        emissive: Some(v3::scale(red, 9.0)),
        flags: Some(box_flags),
        ..Default::default()
    });
    crate::thinline::thin_note(g, line.into(), HW, 0.45);
    let shown = if ir { crate::irlaser::ir_note(g, line.into()) } else { line.into() };
    out.push(shown);
    if dot {
        let end = v3::mad(from, dir, l - 0.006);
        let bead = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
            c: end,
            h: [0.011, 0.011, 0.011],
            yaw: Some(0.0),
            albedo: Some(if ir { [0.0, 0.0, 0.0] } else { [0.05, 0.0, 0.0] }),
            emissive: Some(v3::scale(red, 40.0)),
            flags: Some(box_flags),
            ..Default::default()
        });
        let shown = if ir { crate::irlaser::ir_note(g, bead.into()) } else { bead.into() };
        out.push(shown);
        if air { crate::thinline::thin_air(g, line.into(), Some(bead.into())); }
    } else if air {
        crate::thinline::thin_air(g, line.into(), None);
    }
}

// ---------------------------------------------------------------- the things in the air

/// The thrown things' step, then their fuses.
pub fn update_items(g: &mut crate::game::Game, dt: f64) {
    use crate::throwables::{self as t, ItemKind};
    t::throwables_update(g, dt);
    let mut i = 0usize;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        let kind = g.tab.item[it].kind;
        if kind == ItemKind::Smoke {
            if !g.tab.item[it].fired {
                let f = g.tab.item[it].fuse - dt;
                g.tab.item[it].fuse = f;
                if g.tab.item[it].fuse <= 0.0 { g.tab.item[it].fired = true; start_canister(g, it); }
            } else if g.tab.item[it].emit_t > 0.0 {
                let e = g.tab.item[it].emit_t - dt;
                g.tab.item[it].emit_t = e;
            }
        } else if kind == ItemKind::Flash && !g.tab.item[it].fired {
            let landed = g.tab.item[it].bounces > 0.0 || g.tab.item[it].settled;
            if landed { let f = g.tab.item[it].fuse - dt; g.tab.item[it].fuse = f; }
            if g.tab.item[it].fuse <= 0.0 || (!landed && g.tab.item[it].age > crate::effects::bang().max_air) {
                g.tab.item[it].fired = true;
                detonate_stun(g, it, None, None);
            }
        } else if kind == ItemKind::Frag {
            crate::frag::frag_fuse(g, it, dt);
        }
    }
}

/// The attached solver's canister tuning, or None behind a stand-in without any.
fn smoke_params(g: &crate::game::Game) -> Option<crate::smoke_types::CanisterParams> {
    g.smoke.params
}

/// What a smoke can lets out: the attached solver's tuning, or its own numbers behind a stand-in without any.
pub fn can_params(g: &crate::game::Game) -> crate::smoke_types::CanisterParams {
    smoke_params(g).unwrap_or(crate::smoke_types::CanisterParams { canister_duration: 32.0, canister_density: 95.0, canister_radius: 0.2, canister_temp: 1.6, canister_speed: 1.9 })
}

/// A smoke can's fuse is out: it starts to pour, hisses, and the pop draws attention nearby.
fn start_canister(g: &mut crate::game::Game, it: ItemId) {
    let seconds: f64;
    if crate::effects::flags().vent_canister {
        let yaw = g.tab.item[it].heading;
        let at = crate::effects::vent_mouth(g.tab.item[it].pos, yaw);
        seconds = crate::effects::spawn_vent(g, at, yaw, None);
    } else {
        let p = can_params(g);
        seconds = p.canister_duration;
        let pos = v3::add(g.tab.item[it].pos, [0.0, 0.1, 0.0]);
        // `track` (the can's top, while it rolls) has no arm in effects::Track: notes, Skeleton
        let e = crate::smoke_types::SmokeEmitter { pos, dir: [0.0, 1.0, 0.0], speed: p.canister_speed, radius: p.canister_radius, density: p.canister_density, temperature: p.canister_temp, ttl: p.canister_duration, age: 0.0, kind: crate::smoke_types::SmokeEmitterKind::Canister, ..crate::smoke_types::SmokeEmitter::default() };
        crate::smoke_types::smoke_system_like_emit(g, &e);
        crate::out_events::push_smoke_system_like_emit(g, &e);
    }
    g.tab.item[it].emit_t = seconds;
    let pos = g.tab.item[it].pos;
    // (its hiss is no event: a picture hears it for as long as the can pours, off the can itself)
    crate::audio_iface::game_audio_like_play(g, "smokePop", Some(pos), Some(0.9), None);
    let e = crate::game::GameEvent { kind: crate::director::EventKind::Shot, pos: v3::copy(pos), time: g.time, loud: false, serial: g.serial(), ..Default::default() };
    g.events.push(e);
}
