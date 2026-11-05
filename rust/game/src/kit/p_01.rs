//! src/game/kit.ts lines 1 to 377
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- private helpers

/// (kit.ts SLOT_AMOUNT[k](m)) The table as a call by its key.
fn slot_amount_of(g: &mut crate::game::Game, k: &str, m: &crate::kit::MemberKit) -> f64 {
    match k {
        "smoke" => slot_amount::smoke(m),
        "bangs" => slot_amount::bangs(m),
        "frags" => slot_amount::frags(m),
        "shells" => slot_amount::shells(m),
        "charges" => slot_amount::charges(m),
        "walls" => slot_amount::walls(m),
        "shotgun" => slot_amount::shotgun(g, m),
        "hammer" => slot_amount::hammer(g, m),
        "cams" => slot_amount::cams(g, m),
        "rings" => slot_amount::rings(g, m),
        "wand" => slot_amount::wand(g, m),
        _ => unreachable!(),
    }
}

/// (kit.ts m[slot]) The pick in a slot of a card.
fn pick_of(m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> &str {
    match slot {
        crate::gunslots::GunSlot::Primary => &m.primary,
        crate::gunslots::GunSlot::Secondary => &m.secondary,
    }
}

// ---- the part

/// What member i is issued as the rule stands.   (kit.ts issuedCard)
pub fn issued_card(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit {
    if i == 0.0 {
        if let Some(h) = g.m.kit.hand {
            return stealth_issue(g, h);
        }
    }
    if i == 0.0 && !g.k.slots.on {
        let b = built();
        let mut c = issued()[0].clone();
        c.smoke = b.smoke;
        c.bangs = b.bangs;
        c.frags = b.frags;
        c.shells = b.shells;
        return c;
    }
    issued()[i as usize].clone()
}

/// A fresh copy of what member i is issued.   (kit.ts kitDefault)
pub fn kit_default(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit {
    issued_card(g, i)
}

/// The weapons.ts row a card names.   (kit.ts kitGun)
pub fn kit_gun(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> crate::weapons::WeaponId {
    if guns_on(g) {
        let w = if m.weapon == KitWeapon::Pistol {
            let can = if tree_up(g) { m.side_can } else { m.suppressed };
            crate::gunslots::pick_row(g, &m.secondary, can)
        } else {
            crate::gunslots::pick_row(g, &m.primary, m.suppressed)
        };
        if let Some(w) = w {
            return w;
        }
    }
    match m.weapon {
        KitWeapon::Shotgun => crate::weapons::WeaponId::M870,
        KitWeapon::Rifle => {
            if m.suppressed { crate::weapons::WeaponId::Fn2000 } else { crate::weapons::WeaponId::Aks74u }
        }
        KitWeapon::Pistol => {
            if m.suppressed { crate::weapons::WeaponId::Fiveseven } else { crate::weapons::WeaponId::Guard9mm }
        }
    }
}

/// The card has two gun rows.   (kit.ts gunsOn)
pub fn guns_on(g: &mut crate::game::Game) -> bool {
    g.k.bigslot.on && !crate::gunslots::guns_broken()
}

/// The card is laid out as a tree.   (kit.ts treeUp)
pub fn tree_up(g: &mut crate::game::Game) -> bool {
    guns_on(g) && crate::kittree::tree_on(g)
}

/// A slot's pick comes with its suppressor on or off.   (kit.ts canChoice)
pub fn can_choice(g: &mut crate::game::Game, id_2: &str) -> bool {
    let a = crate::gunslots::pick_row(g, id_2, true);
    let b = crate::gunslots::pick_row(g, id_2, false);
    a.is_some() && b.is_some() && a != b
}

/// The gun in a slot of a card wears its suppressor.   (kit.ts kitSlotCan)
pub fn kit_slot_can(g: &mut crate::game::Game, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> bool {
    let can = if slot == crate::gunslots::GunSlot::Primary { m.suppressed } else { m.side_can };
    match crate::gunslots::pick_row(g, pick_of(m, slot), can) {
        Some(w) => g.m.weapons.weapons.get(w).unwrap().suppressed,
        None => false,
    }
}

/// The gun a card's man carries that is not in his hands, or None.   (kit.ts kitSide)
pub fn kit_side(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> {
    if m.weapon == KitWeapon::Pistol {
        return None;
    }
    kit_second(g, m)
}

/// The row in a card's secondary slot; None with the switch off.   (kit.ts kitSecond)
pub fn kit_second(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> {
    if !guns_on(g) {
        return None;
    }
    let can = if tree_up(g) { m.side_can } else { m.suppressed };
    crate::gunslots::pick_row(g, &m.secondary, can)
}

/// The weapons.ts row of the gun in a slot of member i's card as he would deploy with it, or None.   (kit.ts kitSlotDef)
pub fn kit_slot_def(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> Option<crate::weapons::WeaponDef> {
    if slot == crate::gunslots::GunSlot::Primary && m.weapon == KitWeapon::Pistol {
        return None;
    }
    let tree = tree_up(g);
    let can = if i == 0.0 && !tree {
        true
    } else if slot == crate::gunslots::GunSlot::Primary || !tree {
        m.suppressed
    } else {
        m.side_can
    };
    let w = crate::gunslots::pick_row(g, pick_of(m, slot), can)?;
    g.m.weapons.weapons.get(w).cloned()
}

/// The gun on member i's row `field` wears a suppressor.   (kit.ts kitRowCan)
pub fn kit_row_can(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, field: &str) -> bool {
    if field == "weapon" {
        return kit_gun_can(g, i, m);
    }
    if tree_up(g) {
        return if field == "primary" {
            m.weapon != KitWeapon::Pistol && kit_slot_can(g, m, crate::gunslots::GunSlot::Primary)
        } else {
            field == "secondary" && kit_slot_can(g, m, crate::gunslots::GunSlot::Secondary)
        };
    }
    let w = if field == "primary" {
        if m.weapon == KitWeapon::Pistol { None } else { crate::gunslots::pick_row(g, &m.primary, i == 0.0 || m.suppressed) }
    } else if field == "secondary" {
        crate::gunslots::pick_row(g, &m.secondary, i == 0.0 || m.suppressed)
    } else {
        None
    };
    match w {
        Some(w) => g.m.weapons.weapons.get(w).unwrap().suppressed,
        None => false,
    }
}

/// A card's two picks are ones the table offers. Writes `m` and returns a copy of it.   (kit.ts gunsHome)
pub fn guns_home(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
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

/// The wall charge is a row only in a build that has it: this one does.   (kit.ts wallHere)
pub fn wall_here() -> bool {
    true
}

/// A loadout's tools this build cannot put on.   (kit.ts loadoutMissing)
pub fn loadout_missing(g: &mut crate::game::Game, l: &crate::kit::SlotLoadout) -> Vec<String> {
    let walls = l.counts.iter().any(|c| c.get(String::from("walls")).copied().unwrap_or(0.0) > 0.0);
    let mut missing = l.missing.clone();
    if walls && !(g.k.slots.on && wall_here()) {
        missing.push(String::from("wallCharge"));
    }
    missing
}

/// A card as the big slot reads it: the shotgun is no gun. Writes `m` and returns a copy of it.   (kit.ts bigHome)
pub fn big_home(m: &mut crate::kit::MemberKit, i: f64) -> crate::kit::MemberKit {
    if m.weapon == KitWeapon::Shotgun {
        m.weapon = issued()[i as usize].weapon;
        if !js::truthy(m.walls) && m.big == BigKit::None {
            m.big = BigKit::Shotgun;
        }
    }
    m.clone()
}

/// What is in the big slot of a card, as its row words it.   (kit.ts bigOf)
pub fn big_of(m: &crate::kit::MemberKit) -> crate::kit::BigPick {
    if js::truthy(m.walls) {
        return BigPick::Wall;
    }
    match m.big {
        BigKit::None => BigPick::None,
        BigKit::Shotgun => BigPick::Shotgun,
        BigKit::Hammer => BigPick::Hammer,
    }
}

/// The pouches one of an item fills as the rule stands.   (kit.ts priceOf)
pub fn price_of(g: &mut crate::game::Game, it: Option<&crate::kit::SlotItem>) -> f64 {
    match it {
        None => 0.0,
        Some(it) => {
            if g.k.bigslot.on && it.big.unwrap_or(false) { 0.0 } else { it.pouches }
        }
    }
}

/// The pouches `n` of an item fill.   (kit.ts pouchesOf)
pub fn pouches_of(g: &mut crate::game::Game, it: Option<&crate::kit::SlotItem>, n: f64) -> f64 {
    let p = price_of(g, it);
    if !js::truthy(p) || n <= 0.0 {
        return 0.0;
    }
    // p is truthy, so `it` is there
    let holds = it.and_then(|it| it.holds).filter(|h| js::truthy(*h));
    match holds {
        Some(h) if stealth_on(g) => p * m::ceil(n / h),
        _ => p * n,
    }
}

/// How many big things a card carries (0 with the big slot off).   (kit.ts bigUsed)
pub fn big_used(g: &mut crate::game::Game, m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 {
    if !g.k.bigslot.on {
        return 0.0;
    }
    let mut n = 0.0;
    for k in slot_amount().keys() {
        if t.items.get(k.clone()).and_then(|it| it.big).unwrap_or(false) {
            n += slot_amount_of(g, &k, m);
        }
    }
    n
}

/// A whole number at or over `lo`.   (kit.ts int)
pub fn int(x: Option<&crate::kit::KitValue>, lo: f64) -> bool {
    match x {
        Some(KitValue::Num(n)) => n.is_finite() && m::floor(*n) == *n && *n >= lo,
        _ => false,
    }
}

/// `n` pouches, in a card's words.   (kit.ts pouchWords)
pub fn pouch_words(n: f64) -> String {
    if n == 1.0 {
        return String::from("a pouch");
    }
    alloc::format!("{} pouches", n)
}

/// Which of a table's two most-a-man columns member i reads.   (kit.ts kindOf)
pub fn kind_of(i: f64) -> crate::kit::KindOf {
    if i == 0.0 { KindOf::You } else { KindOf::Mate }
}

/// Your card's long gun has the launcher under it.   (kit.ts tubed)
pub fn tubed(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> bool {
    if m.weapon != KitWeapon::Rifle {
        return false;
    }
    if !guns_on(g) {
        return true;
    }
    let w = crate::gunslots::pick_row(g, &m.primary, true).unwrap_or(crate::weapons::WeaponId::Fn2000);
    g.m.weapons.weapons.get(w).unwrap().launcher != Some(false)
}

/// A teammate's card has the 40 mm row.   (kit.ts mateShells)
pub fn mate_shells(g: &mut crate::game::Game) -> bool {
    tree_up(g) && crate::squad40::squad40_here(g)
}

/// A suppressor's flag holds a value only while its gun has the choice. Writes `m` and returns a copy of it.   (kit.ts homeCans)
pub fn home_cans(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
    if (!m.suppressed || !m.side_can) && tree_up(g) {
        if m.weapon == KitWeapon::Pistol || !can_choice(g, &m.primary) {
            m.suppressed = true;
        }
        if !can_choice(g, &m.secondary) {
            m.side_can = true;
        }
    }
    m.clone()
}

/// The launcher's rounds go home with the rifle. Writes `m` and returns a copy of it.   (kit.ts homeShells)
pub fn home_shells(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit {
    home_cans(g, m);
    if !tubed(g, m) {
        m.shells = 0.0;
        if js::truthy(m.rings) && tree_up(g) {
            m.rings = 0.0;
        }
        if js::truthy(m.cams) {
            m.cams = 0.0;
        }
    }
    m.clone()
}

/// The pouches a card fills.   (kit.ts costOf)
pub fn cost_of(g: &mut crate::game::Game, m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 {
    let mut used = 0.0;
    for k in slot_amount().keys() {
        let n = slot_amount_of(g, &k, m);
        used += pouches_of(g, t.items.get(k.clone()), n);
    }
    used
}

/// The teammates who come, of five cards; copies, since the only readers only read.   (kit.ts coming)
pub fn coming(cards: &[crate::kit::MemberKit], n: f64) -> Vec<crate::kit::MemberKit> {
    let end = (1 + n as usize).min(cards.len());
    if end <= 1 {
        return Vec::new();
    }
    cards[1..end].to_vec()
}

/// (kit.ts SLOT_AMOUNT) How much of each priced thing a card carries.
pub mod slot_amount {
    use crate::js;
    use crate::kit::{BigKit, KitWeapon};

    /// (kit.ts SLOT_AMOUNT.smoke)
    pub fn smoke(m: &crate::kit::MemberKit) -> f64 { m.smoke }

    /// (kit.ts SLOT_AMOUNT.bangs)
    pub fn bangs(m: &crate::kit::MemberKit) -> f64 { m.bangs }

    /// (kit.ts SLOT_AMOUNT.frags)
    pub fn frags(m: &crate::kit::MemberKit) -> f64 { m.frags }

    /// (kit.ts SLOT_AMOUNT.shells)
    pub fn shells(m: &crate::kit::MemberKit) -> f64 { m.shells }

    /// (kit.ts SLOT_AMOUNT.charges)
    pub fn charges(m: &crate::kit::MemberKit) -> f64 { m.charges }

    /// (kit.ts SLOT_AMOUNT.walls)
    pub fn walls(m: &crate::kit::MemberKit) -> f64 { m.walls }

    /// With the big slot on the shotgun is what is in it, never the gun.   (kit.ts SLOT_AMOUNT.shotgun)
    pub fn shotgun(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 {
        let has = if g.k.bigslot.on { m.big == BigKit::Shotgun } else { m.weapon == KitWeapon::Shotgun };
        if has { 1.0 } else { 0.0 }
    }

    /// (kit.ts SLOT_AMOUNT.hammer)
    pub fn hammer(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 {
        if g.k.bigslot.on && m.big == BigKit::Hammer { 1.0 } else { 0.0 }
    }

    /// (kit.ts SLOT_AMOUNT.cams)
    pub fn cams(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 {
        if js::truthy(m.cams) { m.cams } else { 0.0 }
    }

    /// (kit.ts SLOT_AMOUNT.rings)
    pub fn rings(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 {
        if js::truthy(m.rings) && super::tree_up(g) { m.rings } else { 0.0 }
    }

    /// (kit.ts SLOT_AMOUNT.wand)
    pub fn wand(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 {
        if m.wand { 1.0 } else { 0.0 }
    }
}

/// (kit.ts SLOT_PUT) How a card is put to `n` of one thing.
pub mod slot_put {
    use crate::kit::{BigKit, KitWeapon};

    /// (kit.ts SLOT_PUT.smoke)
    pub fn smoke(m: &mut crate::kit::MemberKit, n: f64) { m.smoke = n; }

    /// (kit.ts SLOT_PUT.bangs)
    pub fn bangs(m: &mut crate::kit::MemberKit, n: f64) { m.bangs = n; }

    /// (kit.ts SLOT_PUT.frags)
    pub fn frags(m: &mut crate::kit::MemberKit, n: f64) { m.frags = n; }

    /// (kit.ts SLOT_PUT.shells)
    pub fn shells(m: &mut crate::kit::MemberKit, n: f64) { m.shells = n; }

    /// (kit.ts SLOT_PUT.charges)
    pub fn charges(m: &mut crate::kit::MemberKit, n: f64) { m.charges = n; }

    /// (kit.ts SLOT_PUT.walls)
    pub fn walls(m: &mut crate::kit::MemberKit, n: f64) { m.walls = n; }

    /// (kit.ts SLOT_PUT.wand)
    pub fn wand(m: &mut crate::kit::MemberKit, n: f64) { m.wand = n > 0.0; }

    /// (kit.ts SLOT_PUT.cams)
    pub fn cams(m: &mut crate::kit::MemberKit, n: f64) { m.cams = n; }

    /// (kit.ts SLOT_PUT.rings)
    pub fn rings(m: &mut crate::kit::MemberKit, n: f64) { m.rings = n; }

    /// A shotgun given up is the issued gun again (`d`: what he is issued).   (kit.ts SLOT_PUT.shotgun)
    pub fn shotgun(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit, n: f64, d: &crate::kit::MemberKit) {
        if g.k.bigslot.on {
            if n > 0.0 {
                m.big = BigKit::Shotgun;
            } else if m.big == BigKit::Shotgun {
                m.big = BigKit::None;
            }
        } else if n > 0.0 {
            m.weapon = KitWeapon::Shotgun;
        } else if m.weapon == KitWeapon::Shotgun {
            m.weapon = d.weapon;
        }
    }

    /// (kit.ts SLOT_PUT.hammer)
    pub fn hammer(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit, n: f64) {
        if !g.k.bigslot.on {
            return;
        }
        if n > 0.0 {
            m.big = BigKit::Hammer;
        } else if m.big == BigKit::Hammer {
            m.big = BigKit::None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card() -> MemberKit { issued()[1].clone() }

    /// the page's table, with the big slot and the rule as the test asks
    fn game(big: bool, rule: bool) -> Game {
        let mut g = Game::blank();
        g.k.bigslot.on = big;
        g.k.slots.on = rule;
        g
    }

    #[test]
    fn the_issue_follows_the_pouches_rule() {
        let mut g = game(true, true);
        let c = issued_card(&mut g, 0.0);
        assert_eq!((c.smoke, c.bangs, c.frags, c.shells), (1.0, 1.0, 1.0, 1.0));
        g.k.slots.on = false;
        let c = issued_card(&mut g, 0.0);
        assert_eq!((c.smoke, c.bangs, c.frags, c.shells), (4.0, 2.0, 2.0, 3.0));
        assert!(c.primary == "mk18" && c.side_can);
        // a teammate's issue does not follow it
        let c = kit_default(&mut g, 1.0);
        assert_eq!((c.smoke, c.bangs, c.charges, c.frags, c.shells), (0.0, 2.0, 1.0, 1.0, 0.0));
    }

    #[test]
    fn with_the_big_slot_off_a_card_names_the_four_old_rows() {
        let mut g = game(false, true);
        let mut c = card();
        assert!(kit_gun(&mut g, &c) == crate::weapons::WeaponId::Fn2000);
        c.suppressed = false;
        assert!(kit_gun(&mut g, &c) == crate::weapons::WeaponId::Aks74u);
        c.weapon = KitWeapon::Pistol;
        assert!(kit_gun(&mut g, &c) == crate::weapons::WeaponId::Guard9mm);
        c.suppressed = true;
        assert!(kit_gun(&mut g, &c) == crate::weapons::WeaponId::Fiveseven);
        c.weapon = KitWeapon::Shotgun;
        assert!(kit_gun(&mut g, &c) == crate::weapons::WeaponId::M870);
        assert!(kit_side(&mut g, &card()).is_none() && kit_second(&mut g, &card()).is_none());
        assert!(tubed(&mut g, &card()) && !tubed(&mut g, &c));
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
    fn the_big_slot_decides_what_a_big_thing_costs() {
        let it = SlotItem { id: String::from("hammer"), pouches: 2.0, max: vec![1.0], squad_max: None, big: Some(true), holds: None };
        let mut g = game(true, true);
        assert_eq!(price_of(&mut g, Some(&it)), 0.0);
        assert_eq!(pouches_of(&mut g, Some(&it), 1.0), 0.0);
        assert_eq!(price_of(&mut g, None), 0.0);
        g.k.bigslot.on = false;
        assert_eq!(price_of(&mut g, Some(&it)), 2.0);
        assert_eq!(pouches_of(&mut g, Some(&it), 3.0), 6.0);
        assert_eq!(pouches_of(&mut g, Some(&it), 0.0), 0.0);
    }

    #[test]
    fn your_issue_fills_your_four_pouches_on_the_pages_table() {
        let mut g = game(true, true);
        let t = g.k.slots.table.clone();
        let c = issued()[0].clone();
        assert_eq!(cost_of(&mut g, &c, &t), 4.0);
        assert_eq!(big_used(&mut g, &c, &t), 0.0);
        let mut c = card();
        c.big = BigKit::Hammer;
        assert_eq!(big_used(&mut g, &c, &t), 1.0);
        g.k.bigslot.on = false;
        assert_eq!(big_used(&mut g, &c, &t), 0.0);
    }

    #[test]
    fn a_loadout_with_a_wall_charge_is_missing_it_with_the_rule_off() {
        let mut counts: JsMap<String, f64> = JsMap::new();
        counts.set(String::from("walls"), 1.0);
        let l = SlotLoadout { counts: vec![counts], missing: vec![String::from("x")], ..Default::default() };
        let mut g = game(true, true);
        assert_eq!(loadout_missing(&mut g, &l), vec![String::from("x")]);
        g.k.slots.on = false;
        assert_eq!(loadout_missing(&mut g, &l), vec![String::from("x"), String::from("wallCharge")]);
    }

    #[test]
    fn the_shotgun_is_the_big_slots_with_it_on_and_the_guns_with_it_off() {
        let d = card();
        let mut g = game(true, true);
        let mut c = card();
        slot_put::shotgun(&mut g, &mut c, 1.0, &d);
        assert!(c.big == BigKit::Shotgun && c.weapon == KitWeapon::Rifle);
        assert_eq!(slot_amount::shotgun(&mut g, &c), 1.0);
        slot_put::hammer(&mut g, &mut c, 0.0);
        assert!(c.big == BigKit::Shotgun);
        slot_put::shotgun(&mut g, &mut c, 0.0, &d);
        assert!(c.big == BigKit::None);
        g.k.bigslot.on = false;
        slot_put::shotgun(&mut g, &mut c, 1.0, &d);
        assert!(c.big == BigKit::None && c.weapon == KitWeapon::Shotgun);
        slot_put::hammer(&mut g, &mut c, 1.0);
        assert!(c.big == BigKit::None);
        assert_eq!(slot_amount::hammer(&mut g, &c), 0.0);
        slot_put::shotgun(&mut g, &mut c, 0.0, &d);
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
    fn pouches_say_their_number_as_javascript_does() {
        assert_eq!(pouch_words(1.0), "a pouch");
        assert_eq!(pouch_words(2.0), "2 pouches");
        assert_eq!(pouch_words(0.0), "0 pouches");
    }
}
