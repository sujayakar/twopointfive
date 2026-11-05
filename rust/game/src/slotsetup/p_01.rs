//! src/game/slotsetup.ts lines 1 to 171
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// a row's digit, the same on every gun (slotsetup.ts ROW_KEY[kind])
fn row_digit(kind: SetupKind) -> f64 {
    *row_key().get(String::from(kind.as_str())).unwrap()
}

/// a gun's own digit (slotsetup.ts SLOT_KEY[gun])
fn slot_digit(gun: SetupGun) -> i32 {
    *slot_key().get(String::from(gun.as_str())).unwrap() as i32
}

/// the key's code of a digit (`Digit${d}`)
fn digit_code(d: i32) -> String {
    alloc::format!("Digit{}", d)
}

/// `/^Digit[1-5]$/`
fn is_digit_1_to_5(code: &str) -> bool {
    let b = code.as_bytes();
    b.len() == 6 && code.starts_with("Digit") && (b'1'..=b'5').contains(&b[5])
}

/// the gun of his named by the setup's word
fn gun_of(g: &Game, pl: PlayerId, gun: SetupGun) -> FirearmId {
    match gun {
        SetupGun::Rifle => g.pl[pl].rifle,
        SetupGun::Pistol => g.pl[pl].pistol,
    }
}

/// the gun in his hands, as the setup names it; None: none (slotsetup.ts setupGun)
pub fn setup_gun(g: &mut crate::game::Game, pl: PlayerId) -> Option<crate::slotsetup::SetupGun> {
    let f = crate::game::player_gun_in_hand(g, pl);
    let f = match f {
        Some(f) if !g.pl[pl].down => f,
        _ => return None,
    };
    if f == g.pl[pl].rifle {
        Some(SetupGun::Rifle)
    } else if f == g.pl[pl].pistol {
        Some(SetupGun::Pistol)
    } else {
        None
    }
}

/// what can be set on one of his guns, as it stands now (slotsetup.ts setupRows)
pub fn setup_rows(g: &mut crate::game::Game, pl: PlayerId, gun: crate::slotsetup::SetupGun) -> Vec<crate::slotsetup::SetupRow> {
    let mut rows: Vec<SetupRow> = Vec::new();
    let f = gun_of(g, pl, gun);
    let def = g.fa[f].def;
    let modes = g.m.weapons.weapons.get(def).unwrap().modes.clone();
    if modes.len() > 1 {
        let mode = g.fa[f].mode;
        let opts = modes
            .iter()
            .map(|m| SetupRowOptsItem { id: String::from(m.as_str()), on: *m == mode, left: None })
            .collect();
        rows.push(SetupRow { key: row_digit(SetupKind::Mode), kind: SetupKind::Mode, opts });
    }
    if gun == SetupGun::Rifle && load_steps(g, pl) > 0.0 {
        let rf = g.pl[pl].rifle;
        let up = seam_rifle_attachment(g, rf);
        let attachments = g.fa[rf].attachments.clone();
        let mut opts = Vec::new();
        for a in attachments {
            let left = seam_rifle_left(g, rf, a);
            opts.push(SetupRowOptsItem { id: String::from(a.as_str()), on: Some(a) == up, left: Some(left) });
        }
        rows.push(SetupRow { key: row_digit(SetupKind::Load), kind: SetupKind::Load, opts });
    }
    if crate::kit::kit_you_light(g, pl) {
        let on = g.fa[f].light_on;
        rows.push(SetupRow {
            key: row_digit(SetupKind::Light),
            kind: SetupKind::Light,
            opts: vec![
                SetupRowOptsItem { id: String::from("off"), on: !on, left: None },
                SetupRowOptsItem { id: String::from("on"), on, left: None },
            ],
        });
    }
    rows
}

/// is the setup of the gun in his hands up; nothing touched (slotsetup.ts slotSetupOpen)
pub fn slot_setup_open(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let s = match g.pl[pl].setup {
        Some(s) => s,
        None => return false,
    };
    Some(s.gun) == setup_gun(g, pl) && g.time - s.seen <= STALE
}

/// how much of its time the open setup has left: 1 with no clock on it, 0 = not up (slotsetup.ts slotSetupLeft)
pub fn slot_setup_left(g: &mut crate::game::Game) -> f64 {
    let pl = g.player;
    if g.pl[pl].setup.is_none() || !slot_setup_open(g) {
        return 0.0;
    }
    1.0
}

/// Esc with the back key's rule off: shut it, and say whether there was one to shut (slotsetup.ts slotSetupShut)
pub fn slot_setup_shut(g: &mut crate::game::Game) -> bool {
    if !slot_setup_open(g) {
        return false;
    }
    let pl = g.player;
    g.pl[pl].setup = None;
    true
}

/// the presses that put the next load on the launcher, in words (slotsetup.ts loadKeys)
pub fn load_keys(g: &mut crate::game::Game) -> String {
    String::from("press 2, then 2")
}

/// where a kit's weapon light is worked, in words (slotsetup.ts lightKeys)
pub fn light_keys(g: &mut crate::game::Game) -> String {
    String::from("its gun's key again, then 3")
}

/// would this key, pressed now, be the setup's own? Nothing touched (slotsetup.ts slotSetupTakes)
pub fn slot_setup_takes(g: &mut crate::game::Game, code: &str) -> bool {
    if !is_digit_1_to_5(code) {
        return false;
    }
    let pl = g.player;
    let gun = match setup_gun(g, pl) {
        Some(gun) => gun,
        None => return false,
    };
    if code == digit_code(slot_digit(gun)) {
        return true;
    }
    slot_setup_open(g) && setup_rows(g, pl, gun).iter().any(|r| code == digit_code(r.key as i32))
}

/// does the open setup run down by itself: never, it is a mode (slotsetup.ts clocked; SLOTSETUP.pop is on)
pub fn clocked(g: &mut crate::game::Game) -> bool {
    false
}

/// has the open setup's clock run out: it has none (slotsetup.ts lapsed; SLOTSETUP.pop is on)
pub fn lapsed(g: &mut crate::game::Game, t: f64) -> bool {
    false
}

/// presses of the launcher's own step from the load that is up to the next one he has any of; 0: none but the one up (slotsetup.ts loadSteps)
pub fn load_steps(g: &mut crate::game::Game, pl: PlayerId) -> f64 {
    let rf = g.pl[pl].rifle;
    let a = g.fa[rf].attachments.clone();
    let n = a.len();
    let i0: i64 = match seam_rifle_attachment(g, rf) {
        Some(up) => a.iter().position(|x| *x == up).map_or(-1, |i| i as i64),
        None => -1,
    };
    for k in 1..n {
        let at = a[((i0 + k as i64) % n as i64) as usize];
        if seam_rifle_left(g, rf, at) > 0.0 {
            return k as f64;
        }
    }
    0.0
}

/// a row's digit: step its position, and the click (slotsetup.ts stepRow)
pub fn step_row(g: &mut crate::game::Game, gun: crate::slotsetup::SetupGun, row: &crate::slotsetup::SetupRow) {
    let pl = g.player;
    let f = gun_of(g, pl, gun);
    match row.kind {
        SetupKind::Mode => {
            crate::weapons::firearm_cycle_mode(g, f);
        }
        SetupKind::Load => {
            let rf = g.pl[pl].rifle;
            let mut k = load_steps(g, pl);
            while k > 0.0 {
                seam_rifle_cycle_attachment(g, rf);
                k -= 1.0;
            }
        }
        SetupKind::Light => g.fa[f].light_on = !g.fa[f].light_on,
    }
    let muzzle = g.ch[g.pl[pl].ch].muzzle;
    let gain = if row.kind == SetupKind::Light { 0.6 } else { 0.7 };
    crate::audio_iface::game_audio_like_play(g, "click", Some(muzzle), Some(gain), None);
}

/// a gun's digit still in the frame that is not the gun in his hands: the press the slot lines will draw that gun on (slotsetup.ts drawPress)
pub fn draw_press(g: &mut crate::game::Game, input: InputId, in_hand: Option<crate::slotsetup::SetupGun>) {
    for gun in [SetupGun::Pistol, SetupGun::Rifle] {
        if Some(gun) != in_hand && crate::input::input_hit(g, input, &digit_code(slot_digit(gun))) {
            let (pl, t) = (g.player, g.time);
            g.pl[pl].setup_key = Some(crate::game::PlayerSetupKey { gun, t });
        }
    }
}

/// player.ts updatePlayer, ahead of the digits' own lines: the digit of the gun in his hands opens its setup; with it up, a row's digit steps the row.
/// A digit used here is taken out of the frame (slotsetup.ts slotSetupKeys)
pub fn slot_setup_keys(g: &mut crate::game::Game, input: InputId) {
    let pl = g.player;
    let gun = setup_gun(g, pl);
    let came = match g.pl[pl].setup_hand {
        None => true,
        Some(h) => h.gun != gun || h.t > g.time,
    };
    if came {
        let t = g.time;
        let mine = match (gun, g.pl[pl].setup_key) {
            (Some(gun), Some(k)) => k.gun == gun && t >= k.t && t - k.t <= KEYED,
            _ => false,
        };
        let own = if mine { t + m::max(g.pl[pl].equip_t, g.k.slotsetup.settle) + OWN_SLACK } else { 0.0 };
        g.pl[pl].setup_hand = Some(crate::game::PlayerSetupHand { gun, t, own: Some(own) });
    }
    let mut up = g.pl[pl].setup;
    if let Some(s) = up {
        let shut = Some(s.gun) != gun
            || g.time - s.seen > STALE
            || crate::handwork::sam_work(g).is_some()
            || crate::samstun::sam_staggered(g, None)
            || g.orders.menu.is_some()
            || crate::interact::backable_interact(g);
        if shut {
            up = None;
            g.pl[pl].setup = None;
        }
    }
    if let Some(s) = up {
        let now = g.time;
        if let Some(live) = g.pl[pl].setup.as_mut() {
            live.seen = now;
        }
        let shut = back_take_lent(g, input)
            || crate::input::input_hit(g, input, "Paused")
            || crate::input::input_lmb_hit(g, input)
            || crate::input::input_rmb_hit(g, input)
            || crate::input::input_hit(g, input, "KeyR")
            || g.pl[pl].last_fire_t >= s.t;
        if shut {
            g.pl[pl].setup = None;
            g.tab.input[input].pressed.delete(digit_code(slot_digit(s.gun)));
            draw_press(g, input, gun);
            return;
        }
        let rows = setup_rows(g, pl, s.gun);
        for d in 1..=5 {
            if g.pl[pl].setup.is_none() {
                break;
            }
            let code = digit_code(d);
            if !crate::input::input_hit(g, input, &code) {
                continue;
            }
            match rows.iter().find(|r| r.key == d as f64) {
                Some(row) => {
                    g.tab.input[input].pressed.delete(code);
                    step_row(g, s.gun, row);
                    g.pl[pl].setup = None;
                }
                None => {
                    if d == slot_digit(s.gun) {
                        g.tab.input[input].pressed.delete(code);
                    }
                    g.pl[pl].setup = None;
                }
            }
        }
        draw_press(g, input, gun);
        return;
    }
    draw_press(g, input, gun);
    let gun = match gun {
        Some(gun) => gun,
        None => return,
    };
    let code = digit_code(slot_digit(gun));
    if !crate::input::input_hit(g, input, &code) {
        return;
    }
    g.tab.input[input].pressed.delete(code);
    let (hand_t, hand_own) = match g.pl[pl].setup_hand {
        Some(h) => (h.t, h.own),
        None => (0.0, None), // never: the first lines of this function set it
    };
    let coming = g.pl[pl].equip_t > 0.0 || g.time - hand_t < g.k.slotsetup.settle;
    if (coming && !(g.time < hand_own.unwrap_or(0.0))) || setup_rows(g, pl, gun).is_empty() {
        return;
    }
    let t = g.time;
    g.pl[pl].setup = Some(crate::game::PlayerSetup { gun, t, seen: t });
    let muzzle = g.ch[g.pl[pl].ch].muzzle;
    crate::audio_iface::game_audio_like_play(g, "click", Some(muzzle), Some(0.35), None);
}

/// backTake(input) (slotsetup.ts:151): the frame's Input lends its pressed set as the back key's `Keys`, and gets back what the call left of it
fn back_take_lent(g: &mut Game, input: InputId) -> bool {
    let pressed = core::mem::take(&mut g.tab.input[input].pressed);
    let mut keys = crate::backkey::Keys { pressed };
    let taken = crate::backkey::back_take(g, &mut keys);
    g.tab.input[input].pressed = keys.pressed;
    taken
}

// The skeleton's gaps, each one step (notes.md, ## Skeleton).

/// rf.attachment (slotsetup.ts:93, 122): rifle_attachment takes neither g nor the gun
fn seam_rifle_attachment(g: &mut Game, rf: FirearmId) -> Option<crate::weapons::RifleAttachment> { let i = g.fa[rf].att_idx; if i >= 0 { g.fa[rf].attachments.get(i as usize).copied() } else { None } } // SKELETON-GAP: crate::weapons::rifle_attachment taking g and the FirearmId (weapons.ts:533-538)

/// rf.left(a) (slotsetup.ts:93, 122): rifle_left takes neither g nor the gun
fn seam_rifle_left(g: &mut Game, rf: FirearmId, a: crate::weapons::RifleAttachment) -> f64 { match a { crate::weapons::RifleAttachment::Ring => g.fa[rf].rings, crate::weapons::RifleAttachment::Cam => g.fa[rf].cams, _ => g.fa[rf].shells } } // SKELETON-GAP: crate::weapons::rifle_left taking g and the FirearmId (weapons.ts:533-538)

/// rf.cycleAttachment() (slotsetup.ts:127): rifle_cycle_attachment takes neither g nor the gun
fn seam_rifle_cycle_attachment(g: &mut Game, rf: FirearmId) -> Option<crate::weapons::RifleAttachment> { let n = g.fa[rf].attachments.len() as i32; g.fa[rf].att_idx = if n > 0 { (g.fa[rf].att_idx + 1) % n } else { -1 }; let i = g.fa[rf].att_idx; if i >= 0 { g.fa[rf].attachments.get(i as usize).copied() } else { None } } // SKELETON-GAP: crate::weapons::rifle_cycle_attachment taking g and the FirearmId (weapons.ts:533-538)
