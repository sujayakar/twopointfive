pub use ::game_types::slotsetup::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use alloc::{string::String, vec, vec::Vec};

/// a row's digit, the same on every gun
fn row_digit(kind: SetupKind) -> f64 {
    *row_key().get(String::from(kind.as_str())).unwrap()
}

/// a gun's own digit
fn slot_digit(gun: SetupGun) -> i32 {
    *slot_key().get(String::from(gun.as_str())).unwrap() as i32
}

/// the key's code of a digit (`Digit${d}`)
fn digit_code(d: i32) -> String {
    alloc::format!("Digit{}", d)
}

/// the gun of his named by the setup's word
fn gun_of(g: &Game, pl: PlayerId, gun: SetupGun) -> FirearmId {
    match gun {
        SetupGun::Rifle => g.pl[pl].rifle,
        SetupGun::Pistol => g.pl[pl].pistol,
    }
}

/// the gun in his hands, as the setup names it; None: none
fn setup_gun(g: &crate::game::Game, pl: PlayerId) -> Option<crate::slotsetup::SetupGun> {
    let f = crate::game::player_gun_in_hand(g, pl);
    let f = match f {
        Some(f) if !crate::condition::down(g, g.pl[pl].ch) => f,
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

/// what can be set on one of his guns, as it stands now
pub fn setup_rows(g: &crate::game::Game, pl: PlayerId, gun: crate::slotsetup::SetupGun) -> Vec<crate::slotsetup::SetupRow> {
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
        let up = crate::weapons::rifle_attachment(g, rf);
        let attachments = g.fa[rf].attachments.clone();
        let mut opts = Vec::new();
        for a in attachments {
            let left = crate::weapons::rifle_left(g, rf, a);
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

/// is the setup of the gun in his hands up; nothing touched
pub fn slot_setup_open(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let pl = seat.player;
    let s = match g.sets[seat.id].hand.setup {
        Some(s) => s,
        None => return false,
    };
    Some(s.gun) == setup_gun(g, pl) && g.time - s.seen <= STALE
}

/// the HUD's words for a load on the launcher and how many of it are left
pub(super) fn load_word(load: crate::weapons::RifleAttachment, left: f64, on: bool) -> crate::view::KitWord {
    use crate::weapons::RifleAttachment as A;
    let name = match load { A::Ring => crate::gear::ring_tag(), A::He40 => crate::gear::shell_tag(), A::Cam => "CAM" };
    crate::view::KitWord { text: alloc::format!("{} ×{}", name, left), on, out: left <= 0.0 }
}

/// the setup that is open, as the HUD shows it; None: none is
pub(super) fn setup_view(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::view::KitSetup> {
    if !slot_setup_open(g, seat) { return None; }
    let gun = setup_gun(g, seat.player)?;
    let word = |text: &str, on: bool| crate::view::KitWord { text: String::from(text), on, out: false };
    let rows = setup_rows(g, seat.player, gun).iter().map(|r| {
        let words = match r.kind {
            SetupKind::Mode => r.opts.iter().map(|o| word(&o.id.to_uppercase(), o.on)).collect(),
            SetupKind::Load => g.fa[g.pl[seat.player].rifle].attachments.iter().zip(r.opts.iter()).map(|(a, o)| load_word(*a, o.left.unwrap_or(0.0), o.on)).collect(),
            SetupKind::Light => vec![word("LIGHT", r.opts.iter().any(|o| o.id == "on" && o.on))],
        };
        crate::view::KitSetupRow { key: digit_code(r.key as i32), words }
    }).collect();
    Some(crate::view::KitSetup { rows, back: crate::backkey::BACK_KEY.to_uppercase() })
}

/// presses of the launcher's own step from the load that is up to the next one he has any of; 0: none but the one up
fn load_steps(g: &crate::game::Game, pl: PlayerId) -> f64 {
    let rf = g.pl[pl].rifle;
    let a = g.fa[rf].attachments.clone();
    let n = a.len();
    let i0: i64 = match crate::weapons::rifle_attachment(g, rf) {
        Some(up) => a.iter().position(|x| *x == up).map_or(-1, |i| i as i64),
        None => -1,
    };
    for k in 1..n {
        let at = a[((i0 + k as i64) % n as i64) as usize];
        if crate::weapons::rifle_left(g, rf, at) > 0.0 {
            return k as f64;
        }
    }
    0.0
}

/// a row's digit: step its position, and the click
fn step_row(g: &mut crate::game::Game, gun: crate::slotsetup::SetupGun, row: &crate::slotsetup::SetupRow) {
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
                crate::weapons::rifle_cycle_attachment(g, rf);
                k -= 1.0;
            }
        }
        SetupKind::Light => g.fa[f].light_on = !g.fa[f].light_on,
    }
    let muzzle = g.ch[g.pl[pl].ch].muzzle;
    let gain = if row.kind == SetupKind::Light { 0.6 } else { 0.7 };
    crate::audio_iface::game_audio_like_play(g, "click", Some(muzzle), Some(gain), None);
}

/// a gun's digit still in the frame that is not the gun in his hands: the press the slot lines will draw that gun on
fn draw_press(g: &mut crate::game::Game, input: InputId, in_hand: Option<crate::slotsetup::SetupGun>) {
    for gun in [SetupGun::Pistol, SetupGun::Rifle] {
        if Some(gun) != in_hand && crate::input::input_hit(g, input, &digit_code(slot_digit(gun))) {
            let (pl, t) = (g.player, g.time);
            g.sets[g.seat].hand.setup_key = Some(crate::game::PlayerSetupKey { gun, t });
        }
    }
}

/// player.ts updatePlayer, ahead of the digits' own lines: the digit of the gun in his hands opens its setup; with it up, a row's digit steps the row.
/// A digit used here is taken out of the frame
pub fn slot_setup_keys(g: &mut crate::game::Game, input: InputId) {
    let pl = g.player;
    let gun = setup_gun(g, pl);
    let came = match g.pl[pl].setup_hand {
        None => true,
        Some(h) => h.gun != gun || h.t > g.time,
    };
    if came {
        let t = g.time;
        let mine = match (gun, g.sets[g.seat].hand.setup_key) {
            (Some(gun), Some(k)) => k.gun == gun && t >= k.t && t - k.t <= KEYED,
            _ => false,
        };
        let own = if mine { t + m::max(g.pl[pl].equip_t, g.k.slotsetup.settle) + OWN_SLACK } else { 0.0 };
        g.pl[pl].setup_hand = Some(crate::game::PlayerSetupHand { gun, t, own: Some(own) });
    }
    let mut up = g.sets[g.seat].hand.setup;
    if let Some(s) = up {
        let shut = Some(s.gun) != gun
            || g.time - s.seen > STALE
            || crate::handwork::sam_work(g).is_some()
            || crate::samstun::sam_staggered(g, &crate::seats::here(g), None)
            || g.sets[g.seat].orders.menu.is_some()
            || crate::interact::backable_interact(g, &crate::seats::here(g));
        if shut {
            up = None;
            g.sets[g.seat].hand.setup = None;
        }
    }
    if let Some(s) = up {
        let now = g.time;
        if let Some(live) = g.sets[g.seat].hand.setup.as_mut() {
            live.seen = now;
        }
        let shut = crate::backkey::back_take(g, input)
            || crate::input::input_hit(g, input, "Paused")
            || crate::input::input_lmb_hit(g, input)
            || crate::input::input_rmb_hit(g, input)
            || crate::input::input_hit(g, input, "KeyR")
            || g.pl[pl].last_fire_t >= s.t;
        if shut {
            g.sets[g.seat].hand.setup = None;
            g.tab.input[input].pressed.delete(digit_code(slot_digit(s.gun)));
            draw_press(g, input, gun);
            return;
        }
        let rows = setup_rows(g, pl, s.gun);
        for d in 1..=5 {
            if g.sets[g.seat].hand.setup.is_none() {
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
                    g.sets[g.seat].hand.setup = None;
                }
                None => {
                    if d == slot_digit(s.gun) {
                        g.tab.input[input].pressed.delete(code);
                    }
                    g.sets[g.seat].hand.setup = None;
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
    g.sets[g.seat].hand.setup = Some(crate::game::PlayerSetup { gun, t, seen: t });
    let muzzle = g.ch[g.pl[pl].ch].muzzle;
    crate::audio_iface::game_audio_like_play(g, "click", Some(muzzle), Some(0.35), None);
}
