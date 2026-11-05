//! src/game/kitanim.ts lines 1 to 286
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Pushes one row on the module's log, keeping the last 200.   (kitanim.ts note)
pub fn note(g: &mut crate::game::Game, row: crate::kitanim::NoteRow) {
    let what = match row.what {
        crate::kitanim::NoteRowWhat::Reload => crate::kitanim::KitanimModKITANIMLOGItemWhat::Reload,
        crate::kitanim::NoteRowWhat::Swap => crate::kitanim::KitanimModKITANIMLOGItemWhat::Swap,
    };
    let t = g.time;
    g.m.kitanim.kitanim_log.push(crate::kitanim::KitanimModKITANIMLOGItem { t, what, from: row.from, to: row.to, kind: row.kind, secs: row.secs });
    if g.m.kitanim.kitanim_log.len() > 200 {
        g.m.kitanim.kitanim_log.remove(0);
    }
}

/// The kit's part runs: nothing of it under the tour.   (kitanim.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    !g.touring
}

/// Is Sam's left hand free to go up to his goggles? Else it is a nod.   (kitanim.ts nvHandFree)
pub fn nv_hand_free(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    if g.pl[p].down
        || g.pl[p].holding.is_some()
        || g.pl[p].takedown.is_some()
        || g.pl[p].dragging.is_some()
        || g.pl[p].picking.is_some()
        || g.pl[p].wanding.is_some()
        || g.pl[p].coding.is_some()
        || g.pl[p].easing.is_some()
        || g.pl[p].door_move.is_some()
        || g.pl[p].kick.is_some()
        || g.pl[p].wall.is_some()
        || g.pl[p].sprinting
        || crate::handwork::sam_work(g).is_some()
        || g.ch[c].anim.big_tool.is_some()
    {
        return false;
    }
    if crate::game::player_rifle_up(g, p) || (crate::game::player_pistol_up(g, p) && g.ch[c].anim.upper == crate::character::UpperMode::Aim) {
        return false;
    }
    let (pistol, rifle) = (g.pl[p].pistol, g.pl[p].rifle);
    if g.pl[p].pending_throw.is_some()
        || g.pl[p].throw_held
        || crate::character::anim_in_throw(g, c)
        || crate::character::anim_reloading(g, c)
        || crate::weapons::firearm_reloading(g, pistol)
        || crate::weapons::firearm_reloading(g, rifle)
        || crate::character::anim_kit_busy(g, c)
        || g.ch[c].anim.ready.is_some()
    {
        return false;
    }
    true
}

/// N was pressed and the switch has changed: the goggles go by the left hand, or by a nod.   (kitanim.ts kitNv)
pub fn kit_nv(g: &mut crate::game::Game) {
    if !live(g) {
        return;
    }
    let p = g.player;
    let hand = nv_hand_free(g);
    let (c, nv) = (g.pl[p].ch, g.pl[p].nv);
    crate::character::anim_nv_flip(g, c, nv, hand);
    *g.stats.kitanim.nv.get_or_insert_with(hand, || 0) += 1;
}

/// The tube is at his eyes: what the picture follows; a flip no longer live is let go of here.   (kitanim.ts kitNvEye)
pub fn kit_nv_eye(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    let k = match g.ch[c].anim.nv_shown {
        Some(k) => k,
        None => return g.pl[p].nv,
    };
    if !live(g) {
        crate::character::anim_nv_cancel(g, c);
        return g.pl[p].nv;
    }
    if g.pl[p].nv { k > 0.0 } else { k >= 1.0 }
}

/// A pose has his hands off his own gun: the animator's word, or what the big tool's beat will say this frame.   (kitanim.ts handsOff)
pub fn hands_off(g: &mut crate::game::Game, an: &crate::character::Anim) -> bool {
    match &an.big_tool {
        Some(b) if crate::bigkit::big_weighed(g) => crate::character::big_off(big_off_of(b)),
        _ => an.hide_held_item,
    }
}

/// `handsOff(pl.char.anim)` from inside this file: the stub's `an` cannot be lent beside `g`.
fn hands_off_c(g: &mut crate::game::Game, c: CharId) -> bool {
    let b = g.ch[c].anim.big_tool.as_ref().map(big_off_of);
    match b {
        Some(b) if crate::bigkit::big_weighed(g) => crate::character::big_off(b),
        _ => g.ch[c].anim.hide_held_item,
    }
}

/// The part of the big tool's record `bigOff` reads.
fn big_off_of(b: &crate::character::AnimBigTool) -> crate::character::BigOffB {
    crate::character::BigOffB { t: b.t, equip: b.equip, stow: b.stow, end: b.end, snug: b.snug }
}

/// What the game's own rules put in Sam's right hand this frame.   (kitanim.ts kitInHand)
pub fn kit_in_hand(g: &mut crate::game::Game) -> Option<crate::character::KitItem> {
    let p = g.player;
    let c = g.pl[p].ch;
    if g.pl[p].down {
        return None;
    }
    if let Some(variant) = g.pl[p].holding.as_ref().map(|h| h.variant) {
        if variant == crate::character::HoldVariant::Arm && !g.ch[c].anim.hold_gun_side && !g.ch[c].anim.no_knife {
            return Some(crate::character::KitItem::Knife);
        }
        return if crate::game::player_pistol_up(g, p) { Some(crate::character::KitItem::Pistol) } else { None };
    }
    if hands_off_c(g, c) {
        return None;
    }
    if crate::bigkit::sam_big_carry(g) {
        return None;
    }
    if crate::game::player_pistol_up(g, p) {
        return Some(crate::character::KitItem::Pistol);
    }
    if crate::game::player_rifle_up(g, p) {
        return Some(crate::character::KitItem::Rifle);
    }
    let k = crate::game::player_canister_sel(g, p);
    match k {
        Some(k) if !g.pl[p].holstered && g.pl[p].dragging.is_none() && crate::game::game_throw_count(g, k) > 0.0 => Some(crate::character::KitItem::Can),
        _ => None,
    }
}

/// The rifle hangs in front of him on its sling when not in his hands; a fact of the mission.   (kitanim.ts kitSlingFront)
pub fn kit_sling_front(g: &mut crate::game::Game) -> bool {
    if !live(g) {
        return false;
    }
    if js::truthy(g.k.kitanim.sling) {
        return g.k.kitanim.sling == 1.0;
    }
    match crate::scenario_run::active_scenario(g) {
        None | Some(crate::scenario_run::ActiveScenario::Quiet) => false,
        Some(_) => true,
    }
}

/// How far the rifle in front is swung onto his thighs, eased on the game's clock; None = not in front.   (kitanim.ts kitSlingRest)
pub fn kit_sling_rest(g: &mut crate::game::Game) -> Option<f64> {
    if !kit_sling_front(g) {
        return None;
    }
    let p = g.player;
    let c = g.pl[p].ch;
    let want = crate::rig_props::sling_rest_want(g, c);
    let s = g.pl[p].sling_rest;
    let dt = match s {
        Some(s) => g.time - s.t,
        None => -1.0,
    };
    let (sling_up, sling_down) = (g.k.kitanim.sling_up, g.k.kitanim.sling_down);
    let mut s = match s {
        Some(s) if !(dt < 0.0 || dt > 0.25 || !(sling_up > 0.0)) => s,
        _ => {
            g.pl[p].sling_rest = Some(crate::game::PlayerSlingRest { a: want, t: g.time });
            return Some(want);
        }
    };
    if dt > 0.0 {
        let k = if want > s.a { sling_up } else if sling_down > 0.0 { sling_down } else { sling_up };
        s.a += (want - s.a) * (1.0 - m::exp(-k * dt));
        s.t = g.time;
        g.pl[p].sling_rest = Some(s);
    }
    Some(s.a)
}

/// How close a man `d` metres off is: 0 at the close-in mode's edge, 1 in the knife's reach.   (kitanim.ts kitCloseness)
pub fn kit_closeness(g: &mut crate::game::Game, d: f64) -> f64 {
    let mode_m = g.k.close.mode_m;
    m::max(0.0, m::min(1.0, (mode_m - d) / m::max(0.1, mode_m - g.k.kitanim.close_near)))
}

/// Once a frame from updatePlayer: deals the animator its closeness and runs the equip watch.   (kitanim.ts kitTick)
pub fn kit_tick(g: &mut crate::game::Game, late: Option<bool>) {
    let late = late.unwrap_or(false);
    let p = g.player;
    let c = g.pl[p].ch;
    let on = live(g);
    // closer than ever, by distance (the frame's second look, `late`, is the equip half alone)
    if late {
    } else if on {
        if let (true, Some(at)) = (g.ch[c].anim.creep, g.ch[c].anim.creep_at) {
            let pos = g.ch[c].pos;
            let d = m::hyp2(at[0] - pos[0], at[2] - pos[2]);
            let k = kit_closeness(g, d);
            g.ch[c].anim.close_k = k;
            g.stats.kitanim.close_frames += 1;
        } else if g.ch[c].anim.close_k < 0.0 {
            g.ch[c].anim.close_k = 0.0;
        }
    } else {
        g.ch[c].anim.close_k = -1.0;
    }
    // equip / put away
    if !on {
        if crate::character::anim_kit_busy(g, c) {
            crate::character::anim_kit_cancel(g, c);
        }
        g.pl[p].kit_was = None;
        return;
    }
    let rest = kit_sling_rest(g).unwrap_or(0.0);
    g.ch[c].anim.kit_rest = rest;
    if !crate::character::anim_kit_busy(g, c) && g.ch[c].anim.big_tool.is_none() && g.pl[p].equip_t > g.k.kitanim.plain_lock {
        g.pl[p].equip_t = g.k.kitanim.plain_lock;
    }
    let cur = kit_in_hand(g);
    let sel = crate::game::player_canister_sel(g, p);
    // something else has his arms, or the item is not his to carry about: no errand
    let carry = crate::bigkit::sam_big_carry(g);
    let hard = g.pl[p].down
        || g.pl[p].holding.is_some()
        || g.pl[p].takedown.is_some()
        || g.pl[p].dragging.is_some()
        || (g.pl[p].kick.is_some() && !carry)
        || g.pl[p].picking.is_some()
        || g.pl[p].wanding.is_some()
        || g.pl[p].coding.is_some()
        || (crate::handwork::sam_work(g).is_some() && !crate::workstow::sam_stowing(g))
        || crate::character::anim_in_throw(g, c);
    let quiet = hard || (hands_off_c(g, c) && !carry);
    let was = g.pl[p].kit_was;
    if was.is_none() || quiet {
        if quiet && crate::character::anim_kit_busy(g, c) {
            crate::character::anim_kit_cancel(g, c);
        }
        // (a can thrown: the next is fetched once the arm is back; a held man let go of: the knife stays until the pose lets go; the last can: the pistol is drawn afterwards)
        let mut now = was;
        if !(was == Some(Some(crate::character::KitItem::Knife)) && !hard) {
            now = if crate::character::anim_in_throw(g, c) && (cur == Some(crate::character::KitItem::Can) || was == Some(None)) { Some(None) } else { Some(cur) };
            g.pl[p].kit_was = now;
        }
        if crate::character::anim_in_throw(g, c) && now == Some(None) && (cur == Some(crate::character::KitItem::Pistol) || cur == Some(crate::character::KitItem::Rifle)) {
            g.pl[p].equip_t = m::max(g.pl[p].equip_t, 0.05);
        }
        if cur == Some(crate::character::KitItem::Can) && sel.is_some() {
            g.pl[p].kit_can = sel;
        }
        return;
    }
    let was = was.unwrap();
    // which can the fist shows: the selected kind, taken up only while no can is in the fist; another kind picked with one in it is a swap of its own
    let mut same = false;
    if cur == Some(crate::character::KitItem::Can) && sel.is_some() {
        if g.ch[c].anim.kit_hold == Some(None) || (!crate::character::anim_kit_busy(g, c) && was != Some(crate::character::KitItem::Can)) {
            g.pl[p].kit_can = sel;
        } else if !crate::character::anim_kit_busy(g, c) && g.pl[p].kit_can != sel {
            same = true;
        }
    }
    if cur == was && !same {
        return;
    }
    // a gun leaving the hand takes its magazine change with it
    let pistol = g.pl[p].pistol;
    if was == Some(crate::character::KitItem::Pistol) && crate::weapons::firearm_reloading(g, pistol) {
        crate::weapons::firearm_cancel_reload(g, pistol);
        crate::character::anim_cancel_reload(g, c);
    }
    let front = kit_sling_front(g);
    g.ch[c].anim.kit_front = front;
    let r = *g.ch[c].bones.get(crate::character::Bone::HandR).unwrap();
    let l = *g.ch[c].bones.get(crate::character::Bone::HandL).unwrap();
    let frame = crate::character::character_hand_frame_r(g, c, Some(true));
    let shown = crate::character::AnimKitSwapShown { r, l, fingers: frame.fingers, back: frame.back };
    let secs = crate::character::anim_kit_swap(g, c, was, cur, Some(shown), Some(same));
    if secs > 0.0 {
        if cur == Some(crate::character::KitItem::Pistol) || cur == Some(crate::character::KitItem::Rifle) {
            g.pl[p].equip_t = m::max(0.0, secs - g.k.kitanim.early);
        }
        g.stats.kitanim.swaps += 1;
        if let Some(it) = was {
            *g.stats.kitanim.stows.get_or_insert_with(it, || 0) += 1;
        }
        if let Some(it) = cur {
            *g.stats.kitanim.draws.get_or_insert_with(it, || 0) += 1;
        }
        note(g, crate::kitanim::NoteRow { what: crate::kitanim::NoteRowWhat::Swap, from: was, to: cur, kind: None, secs });
    }
    g.pl[p].kit_was = Some(cur);
}

/// A click that came a moment early is kept and put back in this tick's input the frame the thing is in hand.   (kitanim.ts kitClick)
pub fn kit_click(g: &mut crate::game::Game, input: InputId) {
    let p = g.player;
    let c = g.pl[p].ch;
    let sel = crate::game::player_canister_sel(g, p);
    let want = if crate::game::player_pistol_up(g, p) {
        Some(crate::character::KitItem::Pistol)
    } else if crate::game::player_rifle_up(g, p) {
        Some(crate::character::KitItem::Rifle)
    } else if sel.is_some() && !g.pl[p].holstered {
        Some(crate::character::KitItem::Can)
    } else {
        None
    };
    if !live(g) || g.pl[p].down || want.is_none() || g.orders.menu.is_some() || g.pl[p].holding.is_some() || g.pl[p].dragging.is_some() {
        g.pl[p].kit_owed = -1.0;
        return;
    }
    let want = want.unwrap();
    let what = owed_word(want, sel);
    if g.pl[p].kit_owed >= 0.0 && g.pl[p].kit_owed_for != what {
        g.pl[p].kit_owed = -1.0;
    }
    let is_can = want == crate::character::KitItem::Can;
    let keep = if is_can { g.k.kitanim.throw_keep } else { g.k.kitanim.click_keep };
    if !(keep > 0.0) {
        g.pl[p].kit_owed = -1.0;
        return;
    }
    let not_yet = if is_can {
        let h = g.ch[c].anim.kit_hold;
        h.is_some() && !(h == Some(Some(crate::character::KitItem::Can)) && g.pl[p].kit_can == sel)
    } else {
        g.pl[p].equip_t > 0.0
    };
    if not_yet {
        if !crate::character::anim_kit_busy(g, c) {
            g.pl[p].kit_owed = -1.0;
        } else if crate::input::input_lmb_hit(g, input) && (is_can || g.pl[p].equip_t <= keep) {
            g.pl[p].kit_owed = g.time;
            g.pl[p].kit_owed_for = what;
        }
        return;
    }
    if g.pl[p].kit_owed >= 0.0 {
        let age = g.time - g.pl[p].kit_owed;
        g.pl[p].kit_owed = -1.0;
        if age >= 0.0 && age <= keep + 0.05 && !crate::input::input_lmb_hit(g, input) {
            g.tab.input[input].clicked = (js::to_i32(g.tab.input[input].clicked) | 1) as f64;
            g.stats.kitanim.owed += 1;
        }
    }
}

/// The thing a kept click is for, in the Player's own word: a can by its kind.
fn owed_word(want: crate::character::KitItem, sel: Option<crate::game::ThrowKind>) -> Option<crate::game::PlayerKitOwedFor> {
    match want {
        crate::character::KitItem::Pistol => Some(crate::game::PlayerKitOwedFor::Pistol),
        crate::character::KitItem::Rifle => Some(crate::game::PlayerKitOwedFor::Rifle),
        crate::character::KitItem::Knife => Some(crate::game::PlayerKitOwedFor::Knife),
        crate::character::KitItem::Can => sel.map(|k| match k {
            crate::game::ThrowKind::Smoke => crate::game::PlayerKitOwedFor::Smoke,
            crate::game::ThrowKind::Flash => crate::game::PlayerKitOwedFor::Flash,
            crate::game::ThrowKind::Frag => crate::game::PlayerKitOwedFor::Frag,
        }),
    }
}

/// The longest a kit change can hold a gun's fire.   (kitanim.ts kitLockMax)
pub fn kit_lock_max(kit: &crate::kitanim::KitLockMaxKIT) -> f64 {
    // (the rifle on its back sling, then on the front one: its own three times)
    fn t(fr: bool, row: &JsMap<crate::character::KitItem, f64>, it: crate::character::KitItem, f: f64) -> f64 {
        if fr && it == crate::character::KitItem::Rifle { f } else { *row.get(it).unwrap() }
    }
    let mut mx = 0.0;
    let rows: &[bool] = if kit.front.is_some() { &[false, true] } else { &[false] };
    let (f_stow, f_reach, f_draw) = match kit.front {
        Some(f) => (f.stow, f.reach, f.draw),
        None => (0.0, 0.0, 0.0),
    };
    for &fr in rows {
        for to in [crate::character::KitItem::Pistol, crate::character::KitItem::Rifle] {
            let r = t(fr, &kit.reach, to, f_reach);
            let d = t(fr, &kit.draw, to, f_draw);
            mx = m::max(mx, r + d);
            for from in [crate::character::KitItem::Pistol, crate::character::KitItem::Rifle, crate::character::KitItem::Can, crate::character::KitItem::Knife] {
                mx = m::max(mx, (t(fr, &kit.stow, from, f_stow) + r + (if fr { 0.0 } else { kit.far }) + d) * kit.swap_k);
            }
        }
    }
    mx
}

/// What the drawer shows in his hands this frame: while a swap runs the hand's word replaces the slot's.   (kitanim.ts kitShow)
pub fn kit_show(g: &mut crate::game::Game, v: &mut crate::kitanim::KitShowV) -> Option<crate::rig_props::KitDraw> {
    if !live(g) {
        return None;
    }
    let p = g.player;
    let c = g.pl[p].ch;
    let h = g.ch[c].anim.kit_hold;
    if g.pl[p].down {
        return None;
    }
    let mut kit: Option<crate::rig_props::KitDraw> = None;
    let was = g.pl[p].kit_was;
    if h.is_none()
        && was == Some(Some(crate::character::KitItem::Knife))
        && g.pl[p].holding.is_none()
        && g.pl[p].takedown.is_none()
        && g.pl[p].kick.is_none()
        && g.pl[p].picking.is_none()
        && g.pl[p].wanding.is_none()
        && g.pl[p].coding.is_none()
    {
        v.knife = true;
    }
    if h.is_none() && was == Some(None) && crate::character::anim_in_throw(g, c) {
        v.pistol_in_hand = false;
    }
    if let Some(hand) = h {
        v.pistol_in_hand = hand == Some(crate::character::KitItem::Pistol);
        if v.rifle.is_some() {
            v.rifle = Some(if hand == Some(crate::character::KitItem::Rifle) { crate::kitanim::KitShowVRifle::Hand } else { crate::kitanim::KitShowVRifle::Slung });
        }
        v.held = if hand == Some(crate::character::KitItem::Can) && !g.pl[p].sprinting { g.pl[p].kit_can } else { None };
        if hand == Some(crate::character::KitItem::Knife) {
            v.knife = true;
        }
        let (roll, stow, front) = (g.ch[c].anim.kit_roll, g.ch[c].anim.kit_stow, g.ch[c].anim.kit_front);
        let rest = if front { kit_sling_rest(g) } else { None };
        kit = Some(crate::rig_props::KitDraw {
            roll: Some(roll),
            pistol: Some(if hand == Some(crate::character::KitItem::Pistol) { stow } else { 0.0 }),
            rifle: Some(if hand == Some(crate::character::KitItem::Rifle) { stow } else { 0.0 }),
            front: Some(front),
            rest,
            ..Default::default()
        });
    }
    let rs = crate::character::anim_reload_show(g, c);
    let long = v.rifle == Some(crate::kitanim::KitShowVRifle::Hand);
    if let Some(rs) = rs {
        let (rifle, pistol) = (g.pl[p].rifle, g.pl[p].pistol);
        if v.pistol_in_hand || (long && g.m.weapons.weapons.get(g.fa[rifle].def).unwrap().pellets.is_none()) {
            let mut draw = kit.unwrap_or_default();
            draw.no_mag = Some(rs.out && long);
            draw.mag_l = if rs.hand {
                Some(if long { crate::slotsetup::SetupGun::Rifle } else { crate::slotsetup::SetupGun::Pistol })
            } else {
                None
            };
            // (a gun with a model of its own carries its own thing in the hand; its magazine / belt box is off the gun when its feeding says so)
            let def = g.m.weapons.weapons.get(g.fa[if long { rifle } else { pistol }].def).unwrap().clone();
            if crate::gunreload::feed_of(g, &def).is_some() {
                draw.mag_l = None;
                draw.no_mag = Some(long && crate::gunreload::reload_show(g, c, def.id).map_or(false, |s| s.no_mag));
            }
            kit = Some(draw);
        }
    }
    kit
}

/// No errand has the hands: the gun the light rides is up.   (kitanim.ts kitGunUp)
pub fn kit_gun_up(g: &mut crate::game::Game) -> bool {
    let c = g.pl[g.player].ch;
    g.ch[c].anim.kit_hold.is_none()
}

/// The magazine change for the gun in his hands, off its state; None = a shell-fed gun or no hands layer: the row's own reload.   (kitanim.ts kitReload)
pub fn kit_reload(g: &mut crate::game::Game, gun: FirearmId, long: bool) -> Option<crate::kitanim::KitReload> {
    let c = g.pl[g.player].ch;
    if !live(g) || g.m.weapons.weapons.get(g.fa[gun].def).unwrap().pellets.is_some() || !crate::character::anim_kit_hands_on(g, c) {
        return None;
    }
    // by what the weapon will do: a magazine with rounds in it is kept, an empty one dropped; the slide / handle is run iff nothing is chambered
    let keeps = matches!(g.fa[gun].mag, Some(n) if n > 0.0);
    let racks = g.fa[gun].chamber == 0.0;
    let table = if long {
        let t = g.k.kitanim.beats.rifle;
        BeatTable { ref_: t.ref_, empty: t.empty, dry: t.dry, tactical: t.tactical }
    } else {
        let t = g.k.kitanim.beats.pistol;
        BeatTable { ref_: t.ref_, empty: t.empty, dry: t.dry, tactical: t.tactical }
    };
    let row_dur = g.m.weapons.weapons.get(g.fa[gun].def).unwrap().reload.dur;
    Some(reload_plan(&table, row_dur, long, keeps, racks))
}

/// `KITANIM.beats.pistol` / `.rifle`: two shapes in the skeleton, one here.
struct BeatTable {
    ref_: f64,
    empty: crate::kitanim::BeatRow,
    dry: crate::kitanim::BeatRow,
    tactical: crate::kitanim::BeatRow,
}

/// kitReload's arithmetic: `row_dur` is the weapon row's own reload length.
fn reload_plan(t: &BeatTable, row_dur: f64, long: bool, keeps: bool, racks: bool) -> crate::kitanim::KitReload {
    let kind = if keeps { crate::kitanim::ReloadKind::Tactical } else if racks { crate::kitanim::ReloadKind::Empty } else { crate::kitanim::ReloadKind::Dry };
    let b = match kind {
        crate::kitanim::ReloadKind::Tactical => t.tactical,
        crate::kitanim::ReloadKind::Empty => t.empty,
        crate::kitanim::ReloadKind::Dry => t.dry,
    };
    let k = row_dur / t.ref_;
    let e = t.empty;
    let rack = if let Some(r) = b.rack {
        Some(r * k)
    } else if keeps && racks {
        Some((b.in_ + (e.rack.unwrap() - e.in_)) * k)
    } else {
        None
    };
    // (the animator's own floors, so the weapon and the hands end together whatever row this was scaled to)
    let dur = m::max(
        if keeps && racks { (b.in_ + (e.dur - e.in_)) * k } else { b.dur * k },
        match rack {
            Some(r) => r + 0.25,
            None => b.in_ * k + 0.5,
        },
    );
    let (out, in_) = (b.out * k, b.in_ * k);
    let row = crate::weapons::WeaponDefReload { dur, out: Some(out), in_: Some(in_), rack, shell: None };
    let beat = crate::character::ReloadBeat { dur, out: Some(out), in_: Some(in_), rack: Some(rack), keep: b.keep.map(|x| x * k), hand: Some(!long), exact: None };
    crate::kitanim::KitReload { kind, row, beat }
}

/// One line for the panel.   (kitanim.ts kitSummary)
pub fn kit_summary(g: &mut crate::game::Game) -> String {
    let s = &g.stats.kitanim;
    let n = |k| s.reloads.get(k).copied().unwrap_or(0);
    alloc::format!(
        "on · {} swaps · reloads {} empty / {} dry / {} tactical",
        s.swaps,
        n(crate::kitanim::ReloadKind::Empty),
        n(crate::kitanim::ReloadKind::Dry),
        n(crate::kitanim::ReloadKind::Tactical),
    )
}

/// A reload began on a kitReload plan: counted.   (kitanim.ts kitReloadNote)
pub fn kit_reload_note(g: &mut crate::game::Game, kind: crate::kitanim::ReloadKind, secs: f64) {
    *g.stats.kitanim.reloads.get_or_insert_with(kind, || 0) += 1;
    note(g, crate::kitanim::NoteRow { what: crate::kitanim::NoteRowWhat::Reload, kind: Some(kind), from: None, to: None, secs });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn beats(long: bool) -> BeatTable {
        let row = |dur, out, in_, rack, keep| crate::kitanim::BeatRow { dur, out, in_, rack, keep };
        if long {
            BeatTable { ref_: 2.4, empty: row(2.4, 0.4, 1.45, Some(1.95), None), dry: row(1.95, 0.4, 1.45, None, None), tactical: row(2.75, 0.55, 2.1, None, Some(1.2)) }
        } else {
            BeatTable { ref_: 1.6, empty: row(1.6, 0.25, 0.95, Some(1.3), None), dry: row(1.45, 0.25, 0.95, None, None), tactical: row(2.0, 0.45, 1.5, None, Some(0.95)) }
        }
    }

    /// The bits of kitanim.ts kitReload itself, run under bun with EXACT on: (long, row's dur, keeps, racks, kind, dur, out, in_, rack, keep).
    #[test]
    fn reload_plan_has_the_typescripts_bits() {
        const ROWS: &[(bool, f64, bool, bool, &str, u64, u64, u64, Option<u64>, Option<u64>)] = &[
            (false, 1.6, false, false, "dry", 0x3ff7333333333333, 0x3fd0000000000000, 0x3fee666666666666, None, None),
            (false, 1.6, false, true, "empty", 0x3ff999999999999a, 0x3fd0000000000000, 0x3fee666666666666, Some(0x3ff4cccccccccccd), None),
            (false, 1.6, true, false, "tactical", 0x4000000000000000, 0x3fdccccccccccccd, 0x3ff8000000000000, None, Some(0x3fee666666666666)),
            (false, 1.6, true, true, "tactical", 0x4001333333333334, 0x3fdccccccccccccd, 0x3ff8000000000000, Some(0x3ffd99999999999a), Some(0x3fee666666666666)),
            (false, 2.4, false, false, "dry", 0x4001666666666666, 0x3fd7ffffffffffff, 0x3ff6cccccccccccc, None, None),
            (false, 2.4, false, true, "empty", 0x4003333333333333, 0x3fd7ffffffffffff, 0x3ff6cccccccccccc, Some(0x3fff333333333332), None),
            (false, 2.4, true, false, "tactical", 0x4007ffffffffffff, 0x3fe5999999999999, 0x4001ffffffffffff, None, Some(0x3ff6cccccccccccc)),
            (false, 2.4, true, true, "tactical", 0x4009cccccccccccd, 0x3fe5999999999999, 0x4001ffffffffffff, Some(0x4006333333333333), Some(0x3ff6cccccccccccc)),
            (false, 1.73, false, false, "dry", 0x3ff915c28f5c28f4, 0x3fd14ccccccccccc, 0x3ff06f5c28f5c28e, None, None),
            (false, 1.73, false, true, "empty", 0x3ffbae147ae147ad, 0x3fd14ccccccccccc, 0x3ff06f5c28f5c28e, Some(0x3ff67d70a3d70a3d), None),
            (false, 1.73, true, false, "tactical", 0x40014ccccccccccc, 0x3fdf23d70a3d70a3, 0x3ff9f33333333332, None, Some(0x3ff06f5c28f5c28e)),
            (false, 1.73, true, true, "tactical", 0x400298f5c28f5c29, 0x3fdf23d70a3d70a3, 0x3ff9f33333333332, Some(0x400000a3d70a3d70), Some(0x3ff06f5c28f5c28e)),
            (false, 3.1, false, false, "dry", 0x4006799999999999, 0x3fdf000000000000, 0x3ffd733333333333, None, None),
            (false, 3.1, false, true, "empty", 0x4008cccccccccccd, 0x3fdf000000000000, 0x3ffd733333333333, Some(0x4004266666666667), None),
            (false, 3.1, true, false, "tactical", 0x400f000000000000, 0x3febe66666666667, 0x4007400000000000, None, Some(0x3ffd733333333333)),
            (false, 3.1, true, true, "tactical", 0x4010a9999999999a, 0x3febe66666666667, 0x4007400000000000, Some(0x400caccccccccccd), Some(0x3ffd733333333333)),
            (true, 1.6, false, false, "dry", 0x3ff7777777777778, 0x3fd1111111111112, 0x3feeeeeeeeeeeef0, None, None),
            (true, 1.6, false, true, "empty", 0x3ff999999999999a, 0x3fd1111111111112, 0x3feeeeeeeeeeeef0, Some(0x3ff4cccccccccccd), None),
            (true, 1.6, true, false, "tactical", 0x3ffe666666666667, 0x3fd7777777777779, 0x3ff6666666666667, None, Some(0x3fe999999999999a)),
            (true, 1.6, true, true, "tactical", 0x4000444444444445, 0x3fd7777777777779, 0x3ff6666666666667, Some(0x3ffbbbbbbbbbbbbd), Some(0x3fe999999999999a)),
            (true, 2.4, false, false, "dry", 0x3fff333333333333, 0x3fd999999999999a, 0x3ff7333333333333, None, None),
            (true, 2.4, false, true, "empty", 0x4003333333333333, 0x3fd999999999999a, 0x3ff7333333333333, Some(0x3fff333333333333), None),
            (true, 2.4, true, false, "tactical", 0x4006000000000000, 0x3fe199999999999a, 0x4000cccccccccccd, None, Some(0x3ff3333333333333)),
            (true, 2.4, true, true, "tactical", 0x4008666666666666, 0x3fe199999999999a, 0x4000cccccccccccd, Some(0x4004cccccccccccd), Some(0x3ff3333333333333)),
            (true, 1.73, false, false, "dry", 0x3ff8b92c5f92c5f9, 0x3fd2740da740da74, 0x3ff0b92c5f92c5f9, None, None),
            (true, 1.73, false, true, "empty", 0x3ffbae147ae147ae, 0x3fd2740da740da74, 0x3ff0b92c5f92c5f9, Some(0x3ff67d70a3d70a3d), None),
            (true, 1.73, true, false, "tactical", 0x40001c28f5c28f5c, 0x3fd95f92c5f92c60, 0x3ff83851eb851eb9, None, Some(0x3febae147ae147ae)),
            (true, 1.73, true, true, "tactical", 0x4001969d0369d036, 0x3fd95f92c5f92c60, 0x3ff83851eb851eb9, Some(0x3ffdfc962fc962fd), Some(0x3febae147ae147ae)),
            (true, 3.1, false, false, "dry", 0x4004266666666667, 0x3fe0888888888889, 0x3ffdf77777777778, None, None),
            (true, 3.1, false, true, "empty", 0x4008cccccccccccd, 0x3fe0888888888889, 0x3ffdf77777777778, Some(0x4004266666666667), None),
            (true, 3.1, true, false, "tactical", 0x400c6aaaaaaaaaab, 0x3fe6bbbbbbbbbbbd, 0x4005b33333333334, None, Some(0x3ff8cccccccccccd)),
            (true, 3.1, true, true, "tactical", 0x400f844444444444, 0x3fe6bbbbbbbbbbbd, 0x4005b33333333334, Some(0x400adddddddddddf), Some(0x3ff8cccccccccccd)),
        ];
        for &(long, row_dur, keeps, racks, kind, dur, out, in_, rack, keep) in ROWS {
            let p = reload_plan(&beats(long), row_dur, long, keeps, racks);
            assert_eq!(p.kind.as_str(), kind);
            assert_eq!(p.row.dur.to_bits(), dur);
            assert_eq!(p.beat.dur.to_bits(), dur);
            assert_eq!(p.row.out.map(f64::to_bits), Some(out));
            assert_eq!(p.beat.out.map(f64::to_bits), Some(out));
            assert_eq!(p.row.in_.map(f64::to_bits), Some(in_));
            assert_eq!(p.beat.in_.map(f64::to_bits), Some(in_));
            assert_eq!(p.row.rack.map(f64::to_bits), rack);
            assert_eq!(p.beat.rack.map(|r| r.map(f64::to_bits)), Some(rack));
            assert_eq!(p.beat.keep.map(f64::to_bits), keep);
            assert_eq!(p.beat.hand, Some(!long));
        }
    }

    fn row_of(v: [f64; 4]) -> JsMap<crate::character::KitItem, f64> {
        let mut t = JsMap::new();
        t.set(crate::character::KitItem::Pistol, v[0]);
        t.set(crate::character::KitItem::Rifle, v[1]);
        t.set(crate::character::KitItem::Can, v[2]);
        t.set(crate::character::KitItem::Knife, v[3]);
        t
    }

    /// kitanim.ts kitLockMax over character.ts KIT, under bun, with its front row and without.
    #[test]
    fn lock_max_has_the_typescripts_bits() {
        let mut kit = crate::kitanim::KitLockMaxKIT {
            stow: row_of([0.3, 0.54, 0.26, 0.34]),
            reach: row_of([0.17, 0.28, 0.16, 0.2]),
            draw: row_of([0.26, 0.43, 0.2, 0.2]),
            far: 0.28,
            swap_k: 0.8,
            front: Some(crate::kitanim::KitLockMaxKITFront { stow: 0.36, reach: 0.2, draw: 0.32 }),
        };
        assert_eq!(kit_lock_max(&kit).to_bits(), 0x3ff395810624dd30);
        kit.front = None;
        assert_eq!(kit_lock_max(&kit).to_bits(), 0x3ff395810624dd30);
    }
}
