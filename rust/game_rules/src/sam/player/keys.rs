//! THE KEYS OF WHAT HE CARRIES: which item is in his hands, its light, its fire mode, its reload.
use super::*;
use crate::game::{Game, SlotKind, ThrowKind};
use crate::ids::*;
use alloc::format;
use alloc::string::String;

/// the cans' keys, in the order they are read
const CAN_KEYS: [(&str, SlotKind, ThrowKind); 3] = [("Digit3", SlotKind::Smoke, ThrowKind::Smoke), ("Digit5", SlotKind::Frag, ThrowKind::Frag), ("Digit4", SlotKind::Flash, ThrowKind::Flash)];

/// The keys of what he carries, read once a tick: what is chosen, put away and brought up, then what is done with it.
pub(super) fn kit_keys(g: &mut Game, t: Tick) {
    let pl = t.pl;
    let was_in = crate::game::player_gun_in_hand(g, pl);
    crate::slotsetup::slot_setup_keys(g, t.input);
    if t.hit(g, "Digit1") { choose(g, pl, SlotKind::Pistol); }
    if t.hit(g, "Digit2") && g.pl[pl].has_rifle { rifle_key(g, t); }
    put_away_keys(g, t);
    for (key, slot, kind) in CAN_KEYS {
        if !t.hit(g, key) || throw_count(g, &crate::seats::here(g), kind) <= 0.0 { continue; }
        if kind != ThrowKind::Frag { g.pl[pl].throw_kind = kind; }
        choose(g, pl, slot);
    }
    guns_changed(g, t, was_in);
    crate::kitanim::kit_tick(g, None);
    crate::kitanim::kit_click(g, t.input);
    let sel = if crate::game::player_rifle_sel(g, pl) { t.rifle } else { t.gun };
    if t.hit(g, "KeyL") { gun_light_key(g, pl, sel); }
    if t.hit(g, "KeyZ") && (sel == t.rifle || g.pl[pl].slot == SlotKind::Pistol) { mode_key(g, t, sel); }
    nv_key(g, Some(t.input), t.dt);
    if t.hit(g, "KeyR") || empty_in_hand(g, t) {
        let sprinting = t.runs(g) && WALK_KEYS.iter().any(|k| t.down(g, k)) && !wall_ran_on(g, pl, t.input) && !g.pl[pl].crouch;
        reload_in_hand(g, Some(sprinting), None);
    }
}

/// An item chosen: in his hand now, or once he steps off a wall he has only a free hand on.
fn choose(g: &mut Game, pl: PlayerId, slot: SlotKind) {
    g.pl[pl].slot = slot;
    if g.pl[pl].wall.is_some() && slot != SlotKind::Rifle { off_the_wall(g, pl, true); } else { g.pl[pl].holstered = false; }
    crate::audio_iface::game_audio_like_play(g, "equip", None, Some(0.5), None);
}
/// On a wall: what is chosen comes out, or stays put away, when he leaves it.
fn off_the_wall(g: &mut Game, pl: PlayerId, out: bool) {
    let Some(w) = g.pl[pl].wall.as_mut() else { return };
    w.drawn = out;
    let text = if out { format!("{} comes out when you leave the wall", item_name(g, pl)) } else { String::from("it'll stay holstered when you leave the wall") };
    crate::interact::refuse(g, pl, Some(crate::view::ThingKind::Wall), &text);
}

/// 2: the rifle, or with it up already the next thing under its barrel.
fn rifle_key(g: &mut Game, t: Tick) {
    if !crate::game::player_rifle_up(g, t.pl) { choose(g, t.pl, SlotKind::Rifle); return; }
    if g.pl[t.pl].equip_t > 0.0 { return; }
    crate::weapons::rifle_cycle_attachment(g, t.rifle);
    at_the_gun(g, t.c, "click", 0.7);
}

/// E puts what is in his hand away or brings it out; the empty hand's own key puts it away.
fn put_away_keys(g: &mut Game, t: Tick) {
    let pl = t.pl;
    let e_key = t.hit(g, "KeyE");
    let away = !e_key && !crate::game::player_choosing(g, pl) && t.hit(g, crate::handslot::HANDSLOT_KEY);
    if !(e_key || away) || g.pl[pl].tow.is_some() { return; }
    match g.pl[pl].wall {
        Some(w) if wall_not_stack(g, pl) && !crate::game::player_rifle_sel(g, pl) => {
            let out = e_key && !w.drawn;
            if out != w.drawn { off_the_wall(g, pl, out); }
        }
        _ => { let on = away || !g.pl[pl].holstered; set_holstered(g, on, None); }
    }
}

/// A gun came into his hands or left them: the rifle takes its time, and a reload of the one put down is dropped.
fn guns_changed(g: &mut Game, t: Tick, was: Option<FirearmId>) {
    let now = crate::game::player_gun_in_hand(g, t.pl);
    if now == was || (now != Some(t.rifle) && was != Some(t.rifle)) { return; }
    g.pl[t.pl].equip_t = RIFLE_EQUIP_SECS;
    if let Some(put_down) = was { reload_dropped(g, t.c, put_down); }
}

/// L: the light on a gun of his, where his kit has one.
pub(super) fn gun_light_key(g: &mut Game, pl: PlayerId, gun: FirearmId) {
    if !crate::kit::kit_you_light(g, pl) { return; }
    g.fa[gun].light_on = !g.fa[gun].light_on;
    at_the_gun(g, g.pl[pl].ch, "click", 0.6);
}

/// Z: the gun's next way of firing, where it has more than one.
fn mode_key(g: &mut Game, t: Tick, gun: FirearmId) {
    if def_of(g, g.fa[gun].def).modes.len() <= 1 { return; }
    crate::weapons::firearm_cycle_mode(g, gun);
    at_the_gun(g, t.c, "click", 0.7);
}

/// The gun he has up is empty and is to be reloaded without being asked.
fn empty_in_hand(g: &Game, t: Tick) -> bool {
    (crate::game::player_pistol_up(g, t.pl) && crate::autoreload::auto_reload_due(g, t.pl, t.gun))
        || (crate::game::player_rifle_up(g, t.pl) && crate::autoreload::auto_reload_due(g, t.pl, t.rifle))
}

/// A sound of his gun's, from its muzzle.
pub(super) fn at_the_gun(g: &mut Game, c: CharId, name: &str, vol: f64) {
    let muzzle = g.ch[c].muzzle;
    crate::audio_iface::game_audio_like_play(g, name, Some(muzzle), Some(vol), None);
}
