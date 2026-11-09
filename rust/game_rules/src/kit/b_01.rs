//! WHAT A SEAT'S HUD IS TOLD OF WHAT ITS MAN CARRIES: the kit's column (rust/game_rules/src/view). A read of the game: nothing here is kept.
use super::*;
use crate::game::{Game, InteractableKind, SlotKind, Verb};
use crate::ids::*;
use crate::slotsetup::SetupGun;
use crate::view::{KitBig, KitGun, KitOcp, KitSlot, KitThing, KitView};
use alloc::{format, string::String, vec, vec::Vec};

/// the gun that is up, as its box shows it
fn gun_view(g: &Game, pl: PlayerId, gun: SetupGun) -> KitGun {
    let p = &g.pl[pl];
    let id = if gun == SetupGun::Rifle { p.rifle } else { p.pistol };
    let (f, def) = (&g.fa[id], g.m.weapons.weapons.get(g.fa[id].def).unwrap());
    let ready = crate::weapons::firearm_rounds_ready(g, id);
    let load = if gun == SetupGun::Rifle { crate::weapons::rifle_attachment(g, id).map(|a| crate::slotsetup::load_word(a, crate::weapons::rifle_left(g, id, a), true)) } else { None };
    let fed_by_shell = def.reload.shell.is_some();
    KitGun {
        mag: f.mag.unwrap_or(0.0),
        capacity: def.mag_capacity,
        chamber: f.chamber != 0.0,
        ready,
        dry: ready == 0.0,
        reloading: crate::weapons::firearm_reloading(g, id),
        selector: if def.modes.len() > 1 { Some(String::from(f.mode.as_str()).to_uppercase()) } else { None },
        load,
        light: kit_you_light(g, pl) && f.light_on,
        ocp: if gun == SetupGun::Pistol { Some(KitOcp { charge: p.ocp.charge, ready: crate::weapons::ocp_ready(&p.ocp) }) } else { None },
        spares: if fed_by_shell || !def.spare_mags.is_finite() { Vec::new() } else { crate::weapons::firearm_spare_sorted(g, id) },
        loose: if fed_by_shell { Some(f.spare.iter().sum()) } else { None },
    }
}

/// a place in the strip for a can
fn can_slot(thing: KitThing, key: &str, name: &str, count: f64, up: bool) -> KitSlot {
    KitSlot { thing, key: String::from(key), name: String::from(name), count, spent: count <= 0.0, up, ..KitSlot::default() }
}

/// the big slot's row: what he carries and whether a door in reach offers it
fn big_view(g: &Game, seat: &crate::seats::SeatRef) -> Option<KitBig> {
    if !g.k.bigslot.on { return None; }
    let name = match g.pl[seat.player].big_kit { BigKit::Shotgun => "breaching shotgun", BigKit::Hammer => "hammer", BigKit::None => return None };
    let offered = seat.nearby.iter().any(|r| r.kind == InteractableKind::Door && matches!(r.verb, Some(Verb::Breach | Verb::Hammer)));
    Some(KitBig { name: String::from(name), place: String::from("at a shut door"), offered })
}

/// WHAT THE KIT'S COLUMN SHOWS OF `seat`'s MAN: a place for each thing a key brings, the gun that is up, its setup while it is open, the big slot, and his condition beside them
pub fn kit_view(g: &Game, seat: &crate::seats::SeatRef) -> KitView {
    let (pl, p) = (seat.player, &g.pl[seat.player]);
    let up = |k: SlotKind| p.slot == k && !p.holstered;
    let gun_slot = |thing, key: &str, id: FirearmId, k: SlotKind| KitSlot { thing, key: String::from(key), name: crate::gear::gear_text(g, &g.m.weapons.weapons.get(g.fa[id].def).unwrap().short), count: crate::weapons::firearm_rounds_ready(g, id), up: up(k), ..KitSlot::default() };
    let rifle = if p.has_rifle { gun_slot(KitThing::Rifle, "Digit2", p.rifle, SlotKind::Rifle) } else { KitSlot { thing: KitThing::Rifle, key: String::from("Digit2"), name: String::from("—"), absent: true, ..KitSlot::default() } };
    let mut slots = vec![
        KitSlot { thing: KitThing::Hands, key: String::from(crate::handslot::HANDSLOT_KEY), up: p.holstered, ..KitSlot::default() },
        gun_slot(KitThing::Pistol, "Digit1", p.pistol, SlotKind::Pistol),
        rifle,
        can_slot(KitThing::Smoke, "Digit3", "smoke", p.canisters, up(SlotKind::Smoke)),
        can_slot(KitThing::Flash, "Digit4", "stun", p.flashbangs, up(SlotKind::Flash)),
        can_slot(KitThing::Frag, "Digit5", "frag", p.frags, up(SlotKind::Frag)),
    ];
    let setup = crate::slotsetup::setup_view(g, seat);
    if let Some(s) = &setup { for slot in slots.iter_mut() { slot.taken = !slot.up && s.rows.iter().any(|r| r.key == slot.key); } }
    let gun = if up(SlotKind::Pistol) { Some(gun_view(g, pl, SetupGun::Pistol)) } else if up(SlotKind::Rifle) && p.has_rifle { Some(gun_view(g, pl, SetupGun::Rifle)) } else { None };
    let named: Vec<String> = p.keys.keys().into_iter().filter(|k| k != crate::doors::FLOOR_KEY).collect();
    KitView {
        slots,
        gun,
        setup,
        big: big_view(g, seat),
        hits_left: p.hits_left,
        hits_max: crate::game::PLAYER_MAX_HITS,
        down: p.down,
        god: g.god_mode,
        night_vision: if crate::rails::rails_locks(g, "nvg") { None } else { Some(p.nv) },
        key_tag: if p.keys.is_empty() { None } else if named.is_empty() { Some(String::from("KEY")) } else { Some(format!("KEY · {}", named.join(" · ")).to_uppercase()) },
    }
}
