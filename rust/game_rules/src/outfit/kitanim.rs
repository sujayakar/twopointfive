pub use ::game_types::kitanim::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;

/// Is Sam's left hand free to go up to his goggles? Else it is a nod.
fn nv_hand_free(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    if crate::condition::down(g, g.pl[p].ch)
        || g.pl[p].hands_full()
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

/// N was pressed and the switch has changed: the goggles go by the left hand, or by a nod.
pub fn kit_nv(g: &mut crate::game::Game) {
    let p = g.player;
    let hand = nv_hand_free(g);
    let (c, nv) = (g.pl[p].ch, g.pl[p].nv);
    crate::character::anim_nv_flip(g, c, nv, hand);
    *g.stats.kitanim.nv.get_or_insert_with(hand, || 0) += 1;
}

/// The tube is at his eyes: what the picture follows; a flip no longer live is let go of here.
pub fn kit_nv_eye(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    let k = match g.ch[c].anim.nv_shown {
        Some(k) => k,
        None => return g.pl[p].nv,
    };
    if g.pl[p].nv { k > 0.0 } else { k >= 1.0 }
}

/// `handsOff(pl.char.anim)` from inside this file: the stub's `an` cannot be lent beside `g`.
fn hands_off_c(g: &crate::game::Game, c: CharId) -> bool {
    let b = g.ch[c].anim.big_tool.as_ref().map(big_off_of);
    match b {
        Some(b) => crate::character::big_off(b),
        _ => g.ch[c].anim.hide_held_item,
    }
}

/// The part of the big tool's record `bigOff` reads.
fn big_off_of(b: &crate::character::AnimBigTool) -> crate::character::BigOffB {
    crate::character::BigOffB { t: b.t, equip: b.equip, stow: b.stow, end: b.end }
}

/// What the game's own rules put in Sam's right hand this frame.
pub fn kit_in_hand(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::character::KitItem> {
    let p = seat.player;
    let c = g.pl[p].ch;
    if crate::condition::down(g, g.pl[p].ch) {
        return None;
    }
    if let Some(variant) = g.pl[p].holding().map(|h| h.variant) {
        if variant == crate::character::HoldVariant::Arm && !g.ch[c].anim.hold_gun_side && !g.ch[c].anim.no_knife {
            return Some(crate::character::KitItem::Knife);
        }
        return if crate::game::player_pistol_up(g, p) { Some(crate::character::KitItem::Pistol) } else { None };
    }
    if hands_off_c(g, c) {
        return None;
    }
    if crate::bigkit::sam_big_carry(g, seat) {
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
        Some(k) if !g.pl[p].holstered && g.pl[p].dragging().is_none() && crate::game::game_throw_count(g, seat, k) > 0.0 => Some(crate::character::KitItem::Can),
        _ => None,
    }
}

/// The rifle hangs in front of him on its sling when not in his hands; a fact of the mission.
pub fn kit_sling_front(g: &mut crate::game::Game) -> bool {
    if js::truthy(g.k.kitanim.sling) {
        return g.k.kitanim.sling == 1.0;
    }
    g.staged.facts.fight
}

/// How far the rifle in front is swung onto his thighs, eased on the game's clock; None = not in front.
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

/// How close a man `d` metres off is: 0 at the close-in mode's edge, 1 in the knife's reach.
fn kit_closeness(g: &crate::game::Game, d: f64) -> f64 {
    let mode_m = g.k.close.mode_m;
    m::max(0.0, m::min(1.0, (mode_m - d) / m::max(0.1, mode_m - g.k.kitanim.close_near)))
}

/// Once a frame from updatePlayer: deals the animator its closeness and runs the equip watch.
pub fn kit_tick(g: &mut crate::game::Game, late: Option<bool>) {
    let late = late.unwrap_or(false);
    let p = g.player;
    let c = g.pl[p].ch;
    // closer than ever, by distance (the frame's second look, `late`, is the equip half alone)
    if late {
    } else {
        if let (true, Some(at)) = (g.ch[c].anim.creep, g.ch[c].anim.creep_at) {
            let pos = g.ch[c].pos;
            let d = m::hyp2(at[0] - pos[0], at[2] - pos[2]);
            let k = kit_closeness(g, d);
            g.ch[c].anim.close_k = k;
        } else if g.ch[c].anim.close_k < 0.0 {
            g.ch[c].anim.close_k = 0.0;
        }
    }
    // equip / put away
    let rest = kit_sling_rest(g).unwrap_or(0.0);
    g.ch[c].anim.kit_rest = rest;
    if !crate::character::anim_kit_busy(g, c) && g.ch[c].anim.big_tool.is_none() && g.pl[p].equip_t > g.k.kitanim.plain_lock {
        g.pl[p].equip_t = g.k.kitanim.plain_lock;
    }
    let cur = kit_in_hand(g, &crate::seats::here(g));
    let sel = crate::game::player_canister_sel(g, p);
    // something else has his arms, or the item is not his to carry about: no errand
    let carry = crate::bigkit::sam_big_carry(g, &crate::seats::here(g));
    let hard = crate::condition::down(g, g.pl[p].ch)
        || g.pl[p].tow.is_some()
        || ((g.pl[p].planted() || crate::handwork::sam_work(g).is_some()) && !carry && !crate::workstow::sam_stowing(g, &crate::seats::here(g)))
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
        if let Some(it) = was {
            *g.stats.kitanim.stows.get_or_insert_with(it, || 0) += 1;
        }
        if let Some(it) = cur {
            *g.stats.kitanim.draws.get_or_insert_with(it, || 0) += 1;
        }
    }
    g.pl[p].kit_was = Some(cur);
}

/// A click that came a moment early is kept and put back in this tick's input the frame the thing is in hand.
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
    if crate::condition::down(g, g.pl[p].ch) || want.is_none() || g.sets[g.seat].orders.menu.is_some() || g.pl[p].holding().is_some() || g.pl[p].tow.is_some() {
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

/// What the drawer shows in his hands this frame: while a swap runs the hand's word replaces the slot's.
pub fn kit_show(g: &mut crate::game::Game, v: &mut crate::kitanim::KitShowV) -> Option<crate::rig_props::KitDraw> {
    let p = g.player;
    let c = g.pl[p].ch;
    let h = g.ch[c].anim.kit_hold;
    if crate::condition::down(g, g.pl[p].ch) {
        return None;
    }
    let mut kit: Option<crate::rig_props::KitDraw> = None;
    let was = g.pl[p].kit_was;
    if h.is_none()
        && was == Some(Some(crate::character::KitItem::Knife))
        && !g.pl[p].planted()
    {
        v.knife = true;
    }
    if h.is_none() && was == Some(None) && crate::character::anim_in_throw(g, c) {
        v.pistol_in_hand = false;
    }
    if let Some(hand) = h {
        v.pistol_in_hand = hand == Some(crate::character::KitItem::Pistol);
        if v.rifle.is_some() {
            v.rifle = Some(if hand == Some(crate::character::KitItem::Rifle) { crate::rig_props::RifleAt::Hand } else { crate::rig_props::RifleAt::Slung });
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
    let long = v.rifle == Some(crate::rig_props::RifleAt::Hand);
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
            if crate::gunreload::feed_of(&def).is_some() {
                draw.mag_l = None;
                draw.no_mag = Some(long && crate::gunreload::reload_show(g, c, def.id).map_or(false, |s| s.no_mag));
            }
            kit = Some(draw);
        }
    }
    kit
}

/// No errand has the hands: the gun the light rides is up.
pub fn kit_gun_up(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let c = g.pl[seat.player].ch;
    g.ch[c].anim.kit_hold.is_none()
}

/// The magazine change for the gun in his hands, off its state; None = a shell-fed gun or no hands layer: the row's own reload.
pub fn kit_reload(g: &mut crate::game::Game, gun: FirearmId, long: bool) -> Option<crate::kitanim::KitReload> {
    if g.m.weapons.weapons.get(g.fa[gun].def).unwrap().pellets.is_some() {
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
    let row = crate::weapons::WeaponDefReload { dur, out: Some(out), in_: Some(in_), rack, shell: None, exact: None };
    let beat = crate::character::ReloadBeat { dur, out: Some(out), in_: Some(in_), rack: Some(rack), keep: b.keep.map(|x| x * k), hand: Some(!long), exact: None };
    crate::kitanim::KitReload { kind, row, beat }
}

/// A reload began on a kitReload plan: counted.
pub fn kit_reload_note(g: &mut crate::game::Game, kind: crate::kitanim::ReloadKind) {
    *g.stats.kitanim.reloads.get_or_insert_with(kind, || 0) += 1;
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

    /// A FAILURE MEANS: the magazine in his hand says something else than the hands do: he keeps a part mag on a tactical, and racks only when the gun asks.
    #[test]
    fn a_reload_is_tactical_if_he_keeps_the_mag_empty_if_he_racks_and_dry_if_neither() {
        let kind = |keeps, racks| reload_plan(&beats(true), 2.4, true, keeps, racks).kind;
        assert_eq!(kind(true, false), crate::kitanim::ReloadKind::Tactical);
        assert_eq!(kind(true, true), crate::kitanim::ReloadKind::Tactical);
        assert_eq!(kind(false, true), crate::kitanim::ReloadKind::Empty);
        assert_eq!(kind(false, false), crate::kitanim::ReloadKind::Dry);
    }

    /// A FAILURE MEANS: the hands and the gun part company: the old mag comes out, the new goes in, then the rack, in that order, the whole scaled to the gun's own reload; the row and the beat tell the same, and a pistol is one hand's.
    #[test]
    fn a_reloads_beats_run_in_order_and_scale_with_the_guns_own_length() {
        for long in [true, false] { for row in [1.6, 2.4, 3.1] { for (keeps, racks) in [(false, false), (false, true), (true, false), (true, true)] {
            let p = reload_plan(&beats(long), row, long, keeps, racks);
            let (out, in_) = (p.beat.out.unwrap(), p.beat.in_.unwrap());
            assert!(0.0 < out && out < in_ && in_ < p.beat.dur);
            assert_eq!(p.beat.rack.unwrap().is_some(), racks);
            if let Some(r) = p.beat.rack.unwrap() { assert!(in_ < r && r < p.beat.dur); }
            assert_eq!(p.beat.keep.is_some(), keeps);
            if let Some(k) = p.beat.keep { assert!(out < k && k < in_); }
            assert_eq!((p.row.dur, p.row.out, p.row.in_, p.row.rack), (p.beat.dur, p.beat.out, p.beat.in_, p.beat.rack.unwrap()));
            assert_eq!(p.beat.hand, Some(!long));
            let twice = reload_plan(&beats(long), row * 2.0, long, keeps, racks);
            assert!((twice.beat.in_.unwrap() - in_ * 2.0).abs() < 1e-9);
            if row >= beats(long).ref_ { assert!((twice.beat.dur - p.beat.dur * 2.0).abs() < 1e-9, "(below the gun's reference the last beat's floor can hold the end)"); }
        } } }
    }

    /// A FAILURE MEANS: a tactical reload that must also rack skips the rack's time: it goes as long after the new mag as an empty one does.
    #[test]
    fn a_tactical_reload_that_racks_takes_the_racks_time_after_the_new_mag() {
        let t = beats(true);
        let p = reload_plan(&t, t.ref_, true, true, true);
        let in_ = p.beat.in_.unwrap();
        assert!((p.beat.rack.unwrap().unwrap() - (in_ + t.empty.rack.unwrap() - t.empty.in_)).abs() < 1e-9);
        assert!((p.beat.dur - (in_ + t.empty.dur - t.empty.in_)).abs() < 1e-9);
    }

    /// A FAILURE MEANS: on a very quick gun the weapon comes back while his hands are still on it: a reload never ends before its last beat has had its time.
    #[test]
    fn a_reload_scaled_small_still_ends_after_its_last_beat() {
        let t = beats(true);
        let dry = reload_plan(&t, 0.1, true, false, false);
        assert!(dry.beat.dur >= dry.beat.in_.unwrap() + 0.5 - 1e-9);
        let empty = reload_plan(&t, 0.1, true, false, true);
        assert!(empty.beat.dur >= empty.beat.rack.unwrap().unwrap() + 0.25 - 1e-9);
    }
}
