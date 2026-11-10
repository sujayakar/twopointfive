//! WHAT A SEAT'S HUD IS TOLD OF WHAT ITS MAN CARRIES: the kit's column (rust/game_rules/src/tick/view). A read of the game: nothing here is kept.

use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{format, string::String, vec, vec::Vec};

use crate::game::{InteractableKind, SlotKind, Verb};
use crate::slotsetup::SetupGun;
use crate::view::{KitBig, KitGun, KitOcp, KitSlot, KitThing, KitView};

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
    let name = match g.pl[seat.player].big_kit { BigKit::Shotgun => "breaching shotgun", BigKit::Hammer => "hammer", BigKit::None => return None };
    let offered = seat.nearby.iter().any(|r| r.kind == InteractableKind::Door && matches!(r.verb, Some(Verb::Breach | Verb::Hammer)));
    Some(KitBig { name: String::from(name), place: String::from("at a shut door"), offered })
}

/// WHAT THE KIT'S COLUMN SHOWS OF `seat`'s MAN: a place for each thing a key brings, the gun that is up, its setup while it is open, the big slot, and his condition beside them
pub fn kit_view(g: &Game, seat: &crate::seats::SeatRef) -> KitView {
    let (pl, p) = (seat.player, &g.pl[seat.player]);
    let up = |k: SlotKind| p.slot == k && !p.holstered;
    let gun_slot = |thing, key: &str, id: FirearmId, k: SlotKind| KitSlot { thing, key: String::from(key), name: crate::gear::gear_text(&g.m.weapons.weapons.get(g.fa[id].def).unwrap().short), count: crate::weapons::firearm_rounds_ready(g, id), up: up(k), ..KitSlot::default() };
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
        down: g.cond[p.ch].down,
        god: g.god_mode,
        night_vision: if crate::rails::rails_locks(g, "nvg") { None } else { Some(p.nv) },
        key_tag: if p.keys.is_empty() { None } else if named.is_empty() { Some(String::from("KEY")) } else { Some(format!("KEY · {}", named.join(" · ")).to_uppercase()) },
    }
}

// ---- private helpers

/// How many of a thing a card carries.
pub(in crate::outfit) fn amount(k: Thing, m: &MemberKit) -> f64 {
    let one = |has: bool| if has { 1.0 } else { 0.0 };
    match k {
        Thing::Shotgun => one(m.big == BigKit::Shotgun),
        Thing::Hammer => one(m.big == BigKit::Hammer),
        Thing::Wand => one(m.wand),
        _ => m.n(k),
    }
}

/// A card put to `n` of a thing.
pub(in crate::outfit) fn put(k: Thing, m: &mut MemberKit, n: f64) {
    let big = |m: &mut MemberKit, tool: BigKit| if n > 0.0 { m.big = tool; } else if m.big == tool { m.big = BigKit::None; };
    match k {
        Thing::Wand => m.wand = n > 0.0,
        Thing::Hammer => big(m, BigKit::Hammer),
        Thing::Shotgun => big(m, BigKit::Shotgun),
        _ => if let Some(had) = m.n_mut(k) { *had = n; },
    }
}

///  The pick in a slot of a card.
fn pick_of(m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> &str {
    match slot {
        crate::gunslots::GunSlot::Primary => &m.primary,
        crate::gunslots::GunSlot::Secondary => &m.secondary,
    }
}

// ---- the part

/// What member i is issued.
pub(super) fn issued_card(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit {
    if i == 0.0 {
        if let Some(h) = g.m.kit.hand {
            return stealth_issue(g, h);
        }
    }
    issued()[i as usize].clone()
}

/// A fresh copy of what member i is issued.
pub(super) fn kit_default(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit {
    issued_card(g, i)
}

/// The weapons.ts row a card names.
pub(in crate::outfit) fn kit_gun(g: &crate::game::Game, m: &crate::kit::MemberKit) -> crate::weapons::WeaponId {
    if m.weapon == KitWeapon::Pistol {
        return crate::gunslots::pick_row(g, &m.secondary, m.side_can).unwrap_or(crate::weapons::WeaponId::Fiveseven);
    }
    crate::gunslots::pick_row(g, &m.primary, m.suppressed).unwrap_or(crate::weapons::WeaponId::Fn2000)
}

/// A slot's pick comes with its suppressor on or off.
pub fn can_choice(g: &crate::game::Game, id_2: &str) -> bool {
    let a = crate::gunslots::pick_row(g, id_2, true);
    let b = crate::gunslots::pick_row(g, id_2, false);
    a.is_some() && b.is_some() && a != b
}

/// The gun in a slot of a card wears its suppressor.
pub(super) fn kit_slot_can(g: &crate::game::Game, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> bool {
    let can = if slot == crate::gunslots::GunSlot::Primary { m.suppressed } else { m.side_can };
    match crate::gunslots::pick_row(g, pick_of(m, slot), can) {
        Some(w) => g.m.weapons.weapons.get(w).unwrap().suppressed,
        None => false,
    }
}

/// The gun a card's man carries that is not in his hands, or None.
pub(in crate::outfit) fn kit_side(g: &crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> {
    if m.weapon == KitWeapon::Pistol {
        return None;
    }
    kit_second(g, m)
}

/// The row in a card's secondary slot.
pub(super) fn kit_second(g: &crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> {
    crate::gunslots::pick_row(g, &m.secondary, m.side_can)
}

/// The row of the gun in a slot of a card as he would deploy with it, or None.
pub(super) fn kit_slot_def(g: &crate::game::Game, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> Option<crate::weapons::WeaponDef> {
    if slot == crate::gunslots::GunSlot::Primary && m.weapon == KitWeapon::Pistol {
        return None;
    }
    let can = if slot == crate::gunslots::GunSlot::Primary { m.suppressed } else { m.side_can };
    let w = crate::gunslots::pick_row(g, pick_of(m, slot), can)?;
    g.m.weapons.weapons.get(w).cloned()
}

/// A card's two picks are ones the table offers. Writes `m` and returns a copy of it.
pub(super) fn guns_home(g: &crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
    let none = crate::gunslots::NO_PRIMARY;
    let long: Vec<String> = crate::gunslots::gun_picks(g, crate::gunslots::GunSlot::Primary).into_iter().filter(|p| p != none).collect();
    let side = crate::gunslots::gun_picks(g, crate::gunslots::GunSlot::Secondary);
    if !long.contains(&m.primary) {
        m.primary = long.first().cloned().unwrap_or_default();
    }
    if !side.contains(&m.secondary) {
        m.secondary = side.first().cloned().unwrap_or_default();
    }
    m.clone()
}

/// A card as the big slot reads it: the shotgun is no gun. Writes `m` and returns a copy of it.
pub(super) fn big_home(m: &mut crate::kit::MemberKit, i: f64) -> crate::kit::MemberKit {
    if m.weapon == KitWeapon::Shotgun {
        m.weapon = issued()[i as usize].weapon;
        if !js::truthy(m.walls) && m.big == BigKit::None {
            m.big = BigKit::Shotgun;
        }
    }
    m.clone()
}

/// What is in the big slot of a card, as its row words it.
pub(in crate::outfit) fn big_of(m: &crate::kit::MemberKit) -> crate::kit::BigPick {
    if js::truthy(m.walls) {
        return BigPick::Wall;
    }
    match m.big {
        BigKit::None => BigPick::None,
        BigKit::Shotgun => BigPick::Shotgun,
        BigKit::Hammer => BigPick::Hammer,
    }
}

/// The pouches one of an item fills: a big thing fills none.
pub(super) fn price_of(it: Option<&crate::kit::SlotItem>) -> f64 {
    match it {
        None => 0.0,
        Some(it) => {
            if it.big.unwrap_or(false) { 0.0 } else { it.pouches }
        }
    }
}

/// The pouches `n` of an item fill.
pub(super) fn pouches_of(it: Option<&crate::kit::SlotItem>, n: f64) -> f64 {
    let p = price_of(it);
    if !js::truthy(p) || n <= 0.0 {
        return 0.0;
    }
    // p is truthy, so `it` is there
    let holds = it.and_then(|it| it.holds).filter(|h| js::truthy(*h));
    match holds {
        Some(h) => p * m::ceil(n / h),
        None => p * n,
    }
}

/// How many big things a card carries.
pub(super) fn big_used(m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 {
    let mut n = 0.0;
    for k in Thing::ALL {
        if t.items.get(k).and_then(|it| it.big).unwrap_or(false) {
            n += amount(k, m);
        }
    }
    n
}

/// A whole number at or over `lo`.
pub(super) fn int(x: Option<&crate::kit::KitValue>, lo: f64) -> bool {
    match x {
        Some(KitValue::Num(n)) => n.is_finite() && m::floor(*n) == *n && *n >= lo,
        _ => false,
    }
}

/// `n` pouches, in a card's words.
pub(super) fn pouch_words(n: f64) -> String {
    if n == 1.0 {
        return String::from("a pouch");
    }
    alloc::format!("{} pouches", js::num_str(n))
}

/// Your card's long gun has the launcher under it.
pub(super) fn tubed(g: &crate::game::Game, m: &crate::kit::MemberKit) -> bool {
    if m.weapon != KitWeapon::Rifle {
        return false;
    }
    let w = crate::gunslots::pick_row(g, &m.primary, true).unwrap_or(crate::weapons::WeaponId::Fn2000);
    g.m.weapons.weapons.get(w).unwrap().launcher != Some(false)
}

/// A suppressor's flag holds a value only while its gun has the choice. Writes `m` and returns a copy of it.
pub(super) fn home_cans(g: &crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
    if !m.suppressed || !m.side_can {
        if m.weapon == KitWeapon::Pistol || !can_choice(g, &m.primary) {
            m.suppressed = true;
        }
        if !can_choice(g, &m.secondary) {
            m.side_can = true;
        }
    }
    m.clone()
}

/// The launcher's rounds go home with the rifle. Writes `m` and returns a copy of it.
pub(super) fn home_shells(g: &crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
    home_cans(g, m);
    if !tubed(g, m) {
        m.shells = 0.0;
        m.rings = 0.0;
        m.cams = 0.0;
    }
    m.clone()
}

/// The pouches a card fills.
pub(super) fn cost_of(m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 {
    let mut used = 0.0;
    for k in Thing::ALL {
        let n = amount(k, m);
        used += pouches_of(t.items.get(k), n);
    }
    used
}

/// The teammates who come, of five cards; copies, since the only readers only read.
pub(super) fn coming(cards: &[crate::kit::MemberKit], n: f64) -> Vec<crate::kit::MemberKit> {
    let end = (1 + n as usize).min(cards.len());
    if end <= 1 {
        return Vec::new();
    }
    cards[1..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card() -> MemberKit { issued()[1].clone() }

    #[test]
    fn your_issue_is_one_of_each_can() {
        let mut g = Game::blank();
        let c = issued_card(&mut g, 0.0);
        assert_eq!((c.smoke, c.bangs, c.frags, c.shells), (1.0, 1.0, 1.0, 1.0));
        assert!(c.primary == "mk18" && c.side_can);
        let c = kit_default(&mut g, 1.0);
        assert_eq!((c.smoke, c.bangs, c.charges, c.frags, c.shells), (0.0, 2.0, 1.0, 1.0, 0.0));
    }

    #[test]
    fn the_launchers_loads_leave_with_the_launcher() {
        let g = Game::blank();
        let loaded = || MemberKit { rings: 2.0, shells: 1.0, cams: 1.0, ..card() };
        let mut c = loaded();
        home_shells(&g, &mut c);
        assert!(tubed(&g, &c) && (c.rings, c.shells, c.cams) == (2.0, 1.0, 1.0));
        let pistol = MemberKit { weapon: KitWeapon::Pistol, ..loaded() };
        let no_tube = MemberKit { primary: String::from("mk46"), ..loaded() };
        for mut c in [pistol, no_tube] {
            home_shells(&g, &mut c);
            assert!(!tubed(&g, &c) && (c.rings, c.shells, c.cams) == (0.0, 0.0, 0.0), "{} {}", c.weapon.as_str(), c.primary);
        }
    }

    #[test]
    fn a_shotgun_card_goes_home_to_the_big_slot() {
        let mut c = card();
        c.weapon = KitWeapon::Shotgun;
        let back = big_home(&mut c, 1.0);
        assert!(c.weapon == KitWeapon::Rifle && c.big == BigKit::Shotgun && back.big == BigKit::Shotgun);
    }

    #[test]
    fn a_wall_charge_keeps_the_big_slot_from_the_shotgun() {
        let mut c = card();
        c.weapon = KitWeapon::Shotgun;
        c.walls = 1.0;
        big_home(&mut c, 1.0);
        assert!(c.weapon == KitWeapon::Rifle && c.big == BigKit::None);
        assert!(big_of(&c) == BigPick::Wall);
    }

    #[test]
    fn a_big_thing_fills_no_pouch() {
        let it = SlotItem { id: String::from("hammer"), pouches: 2.0, max: vec![1.0], squad_max: None, big: Some(true), holds: None };
        assert_eq!(price_of(Some(&it)), 0.0);
        assert_eq!(pouches_of(Some(&it), 1.0), 0.0);
        assert_eq!(price_of(None), 0.0);
        let it = SlotItem { big: None, ..it };
        assert_eq!(price_of(Some(&it)), 2.0);
        assert_eq!(pouches_of(Some(&it), 3.0), 6.0);
        assert_eq!(pouches_of(Some(&it), 0.0), 0.0);
    }

    #[test]
    fn your_issue_fills_your_four_pouches_on_the_pages_table() {
        let mut g = Game::blank();
        let t = g.k.slots.table.clone();
        let c = issued()[0].clone();
        assert_eq!(cost_of(&c, &t), 4.0);
        assert_eq!(big_used(&c, &t), 0.0);
        let mut c = card();
        c.big = BigKit::Hammer;
        assert_eq!(big_used(&c, &t), 1.0);
    }

    #[test]
    fn the_shotgun_is_the_big_slots_and_never_the_gun() {
        let mut c = card();
        put(Thing::Shotgun, &mut c, 1.0);
        assert!(c.big == BigKit::Shotgun && c.weapon == KitWeapon::Rifle);
        assert_eq!(amount(Thing::Shotgun, &c), 1.0);
        put(Thing::Hammer, &mut c, 0.0);
        assert!(c.big == BigKit::Shotgun);
        put(Thing::Shotgun, &mut c, 0.0);
        assert!(c.big == BigKit::None);
        assert!(c.weapon == KitWeapon::Rifle);
    }

    #[test]
    fn int_is_a_whole_number_at_or_over_lo() {
        assert!(int(Some(&KitValue::Num(3.0)), 0.0));
        assert!(!int(Some(&KitValue::Num(3.5)), 0.0));
        assert!(!int(Some(&KitValue::Num(-1.0)), 0.0));
        assert!(!int(Some(&KitValue::Num(f64::INFINITY)), 0.0));
        assert!(!int(Some(&KitValue::Text(String::from("3"))), 0.0));
        assert!(!int(None, 0.0));
    }

    #[test]
    fn the_men_who_come_are_a_slice_past_you() {
        let cards = issued();
        assert_eq!(coming(&cards, 0.0).len(), 0);
        assert_eq!(coming(&cards, 2.0).len(), 2);
        assert_eq!(coming(&cards, 2.0)[0].bangs, 2.0);
        assert_eq!(coming(&cards, 9.0).len(), 4);
        assert_eq!(coming(&[], 2.0).len(), 0);
    }

    #[test]
    fn pouches_say_their_number_in_words() {
        assert_eq!(pouch_words(1.0), "a pouch");
        assert_eq!(pouch_words(2.0), "2 pouches");
        assert_eq!(pouch_words(0.0), "0 pouches");
    }

    #[test]
    fn a_slots_key_fills_its_own_count_and_reads_it_back() {
        let mut c = card();
        for k in Thing::ALL {
            if k == Thing::Shotgun || k == Thing::Hammer { continue; }
            put(k, &mut c, 1.0);
        }
        assert_eq!((c.smoke, c.bangs, c.frags, c.shells, c.charges, c.walls, c.cams, c.rings), (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0));
        assert!(c.wand);
        put(Thing::Hammer, &mut c, 1.0);
        assert_eq!(amount(Thing::Hammer, &c), 1.0);
        put(Thing::Shotgun, &mut c, 1.0);
        assert_eq!((amount(Thing::Shotgun, &c), amount(Thing::Hammer, &c)), (1.0, 0.0));
        assert_eq!(amount(Thing::Smoke, &c), 1.0);
        assert_eq!(amount(Thing::Wand, &c), 1.0);
    }
}
