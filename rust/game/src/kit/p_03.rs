//! src/game/kit.ts lines 770 to 1138
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- what a row's stored closures became, and small helpers (private)

/// A number's row of values, none up to `max`.   (kit.ts upTo)
pub fn up_to(max: f64) -> Vec<f64> {
    // Array.from({ length: max + 1 }): the length is truncated, and a NaN or anything under one makes no row
    let length = max + 1.0;
    if length.is_nan() || length < 1.0 { return Vec::new(); }
    let n = m::floor(length) as usize;
    let mut out = Vec::with_capacity(n);
    for k in 0..n { out.push(k as f64); }
    out
}

/// A counted row's values.
fn counts(max: f64) -> Vec<KitValue> {
    up_to(max).into_iter().map(KitValue::Num).collect()
}

fn words(list: &[&str]) -> Vec<KitValue> {
    list.iter().map(|w| text_value(w)).collect()
}

fn text_value(w: &str) -> KitValue {
    KitValue::Text(String::from(w))
}

/// A yes/no row, in the order the TypeScript writes its two values.
fn flags(first: bool) -> Vec<KitValue> {
    vec![KitValue::Bool(first), KitValue::Bool(!first)]
}

fn row(field: KitField, label: &str, item: Option<&str>, values: Vec<KitValue>) -> KitRow {
    KitRow { field, label: String::from(label), values, item: item.map(String::from) }
}

/// `a === b` of two row values.
fn same_value(a: &KitValue, b: &KitValue) -> bool {
    match (a, b) {
        (KitValue::Text(x), KitValue::Text(y)) => x == y,
        (KitValue::Num(x), KitValue::Num(y)) => x == y,
        (KitValue::Bool(x), KitValue::Bool(y)) => x == y,
        _ => false,
    }
}

/// `values.includes(v)`: unlike `===` it finds a NaN, and a row's values are never NaN.
fn offers(values: &[KitValue], v: &KitValue) -> bool {
    values.iter().any(|x| same_value(x, v))
}

/// A row value as a condition.
fn value_truthy(v: &KitValue) -> bool {
    match v {
        KitValue::Text(s) => !s.is_empty(),
        KitValue::Num(n) => js::truthy(*n),
        KitValue::Bool(b) => *b,
    }
}

/// `String(v)`.
fn value_text(v: &KitValue) -> String {
    match v {
        KitValue::Text(s) => s.clone(),
        KitValue::Num(n) => alloc::format!("{}", n),
        KitValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
    }
}

/// OWN_MAX[k][mate ? 1 : 0] (kit.ts:270).
fn own_most(k: &str, mate: bool) -> f64 {
    own_max().get(String::from(k)).expect("OWN_MAX prices every row's thing")[if mate { 1 } else { 0 }]
}

/// The pick in a card's gun slot: `m[slot]`.
fn slot_pick(m: &MemberKit, slot: crate::gunslots::GunSlot) -> &str {
    match slot { crate::gunslots::GunSlot::Primary => &m.primary, crate::gunslots::GunSlot::Secondary => &m.secondary }
}

/// SLOT_AMOUNT[item](m) (kit.ts:252): a row's `item` is always one of its keys.
fn amount_of(g: &mut Game, item: &str, m: &MemberKit) -> f64 {
    match item {
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
        _ => panic!("kit: no amount for '{}'", item),
    }
}

/// `row.word(v)` (kit.ts KitRow.word): the closures are gone from a row, and its field and the card's owner decide which one it was. A gun slot's is
/// the tree's (kit.ts:916): KITTREE.on is settled on, and the two rows are only ever built with the big slot on.
fn row_word(g: &mut Game, i: f64, r: &KitRow, v: &KitValue) -> String {
    match r.field {
        KitField::Weapon if i == 0.0 => {
            let t = value_text(v);
            let line = you_guns().get(t).cloned().unwrap_or_default();
            crate::gear::gear_text(g, &line)
        }
        KitField::Suppressed | KitField::SideCan => String::from(if value_truthy(v) { "on" } else { "off" }),
        KitField::Light => String::from(if value_truthy(v) { "fitted" } else { "none" }),
        KitField::Nvg => String::from(if value_truthy(v) { "goggles" } else { "none" }),
        KitField::Wand => String::from(if value_truthy(v) { "carried" } else { "none" }),
        KitField::Suit => {
            let t = value_text(v);
            if t == "issued" { return String::from("as issued"); }
            suits().get(t).map(|s| s.label.clone()).unwrap_or_default()
        }
        KitField::Trim => {
            let t = value_text(v);
            if t == "issued" { return String::from("as issued"); }
            trims().get(t).map(|s| s.label.clone()).unwrap_or_default()
        }
        KitField::Cans => match can_mixes().get(value_text(v)) {
            Some(x) => x.label.clone(),
            None => String::from("your own split"),
        },
        KitField::Big => {
            let t = value_text(v);
            big_word().get(t.clone()).cloned().unwrap_or(t)
        }
        KitField::Primary | KitField::Secondary => {
            let word = crate::gunslots::pick_word(g, &value_text(v), None);
            crate::gear::gear_text(g, &word)
        }
        _ => value_text(v),
    }
}

/// `row.fixed?.(m)` (kit.ts:905): why a suppressor's row cannot be stepped, '' = it can. Only the tree's two suppressor rows have one.
fn row_fixed(g: &mut Game, r: &KitRow, m: &MemberKit) -> String {
    let slot = match r.field {
        KitField::SideCan => crate::gunslots::GunSlot::Secondary,
        KitField::Suppressed if tree_up(g) => crate::gunslots::GunSlot::Primary,
        _ => return String::new(),
    };
    let id = slot_pick(m, slot);
    if can_choice(g, id) { return String::new(); }
    let word = crate::gunslots::pick_word(g, id, None);
    let name = crate::gear::gear_text(g, &word);
    let says = if kit_slot_can(g, m, slot) { "made with it" } else { "none fits it" };
    alloc::format!("{}: {}", name, says)
}

/// `r.get` is present (kit.ts:1087): the row reads and writes the card itself, so a stored value is not simply its field.
fn row_reads_itself(g: &mut Game, r: &KitRow) -> bool {
    match r.field {
        KitField::Cans | KitField::Big | KitField::Primary | KitField::Secondary | KitField::SideCan => true,
        KitField::Suppressed => tree_up(g),
        _ => false,
    }
}

// ---- the rows

/// Your two counted canister rows, the pouches rule on.   (kit.ts youCanRows)
pub fn you_can_rows(g: &mut crate::game::Game) -> Vec<crate::kit::KitRow> {
    let smoke = slot_max(g, "smoke", 0.0, SMOKE_MAX);
    let bangs = slot_max(g, "bangs", 0.0, STUN_MAX);
    vec![
        row(KitField::Smoke, "Smoke canisters", Some("smoke"), counts(smoke)),
        row(KitField::Bangs, "Stun canisters", Some("bangs"), counts(bangs)),
    ]
}

/// The pistol carried alone, named for what it is, or null.   (kit.ts youGunsCan)
pub fn you_guns_can(g: &mut crate::game::Game, v: &crate::kit::KitValue) -> Option<String> {
    let table = you_guns_can_table();
    let w = match v {
        KitValue::Text(s) if s == "pistol" => table.pistol,
        KitValue::Text(s) if s == "rifle" => table.rifle,
        KitValue::Text(s) if s == "shotgun" => table.shotgun,
        _ => None,
    };
    match w {
        Some(w) if !w.is_empty() => Some(crate::gear::gear_text(g, &w)),
        _ => None,
    }
}

/// The gun in member i's hands wears a suppressor.   (kit.ts kitGunCan)
pub fn kit_gun_can(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> bool {
    if tree_up(g) {
        let slot = if m.weapon == KitWeapon::Pistol { crate::gunslots::GunSlot::Secondary } else { crate::gunslots::GunSlot::Primary };
        return kit_slot_can(g, m, slot);
    }
    if i == 0.0 { return m.weapon != KitWeapon::Shotgun; }
    let w = kit_gun(g, m);
    g.m.weapons.weapons.get(w).expect("WEAPONS has every id").suppressed
}

/// The split of six a card's two canister counts are, if they are one.   (kit.ts mixOf)
pub fn mix_of(m: &crate::kit::MemberKit) -> Option<crate::kit::CanMix> {
    let table = can_mixes();
    for k in [CanMix::Issued, CanMix::Smoke, CanMix::Even, CanMix::Stun] {   // (the table's own order)
        let x = table.get(String::from(k.as_str())).expect("CAN_MIXES has every split");
        if x.smoke == m.smoke && x.stun == m.bangs { return Some(k); }
    }
    None
}

/// The charges' rows of a card.   (kit.ts chargeRows)
pub fn charge_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    let mut rows = Vec::new();
    if charge_row(g, i) {
        let most = slot_max(g, "charges", i, CHARGES_MAX);
        rows.push(row(KitField::Charges, "Door charges", Some("charges"), counts(most)));
    }
    if wall_row(g, i) && !g.m.kit.big_on {
        let most = slot_max(g, "walls", i, WALLS_MAX);
        rows.push(row(KitField::Walls, "Wall charges", Some("walls"), counts(most)));
    }
    rows
}

/// The big slot's row offers this thing to member i (the `can` of kit.ts bigRow).
fn big_can(g: &mut Game, k: &str, i: f64) -> bool {
    let at = i as usize;
    let on = g.k.slots.on;
    let most = if on {
        g.k.slots.table.items.get(String::from(k)).and_then(|it| it.max.get(at).copied()).unwrap_or(0.0)
    } else {
        own_most(k, i != 0.0)
    };
    most > 0.0 && (!on || g.k.slots.table.big.get(at).copied().unwrap_or(1.0) > 0.0)
}

/// The big slot's one row, the last of a man's card.   (kit.ts bigRow)
pub fn big_row(g: &mut crate::game::Game, i: f64) -> crate::kit::KitRow {
    let mut values = vec![text_value("none")];
    if big_can(g, "shotgun", i) { values.push(text_value("shotgun")); }
    if big_can(g, "hammer", i) { values.push(text_value("hammer")); }
    if wall_row(g, i) { values.push(text_value("wall")); }
    row(KitField::Big, "Big slot", None, values)
}

/// A card's rows, top to bottom. KITTREE.on and GUNSLOTS.on are settled on: with the big slot on, the card is the tree.   (kit.ts buildRows)
pub fn build_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    if g.m.kit.big_on { return build_rows_tree(g, i); }
    if i == 0.0 {
        let mut rows = vec![row(KitField::Weapon, "Guns", Some("shotgun"), words(&["rifle", "pistol", "shotgun"]))];
        if g.k.slots.on { rows.extend(you_can_rows(g)); } else { rows.push(cans_row()); }
        let frags = slot_max(g, "frags", 0.0, FRAGS_MAX);
        rows.push(row(KitField::Frags, "Frag grenades", Some("frags"), counts(frags)));
        let shells = slot_max(g, "shells", 0.0, SHELLS_MAX);
        rows.push(row(KitField::Shells, "40 mm grenades", Some("shells"), counts(shells)));
        rows.extend(charge_rows(g, 0.0));
        if cam_row(g) {
            let cams = slot_max(g, "cams", 0.0, CAMS_MAX);
            rows.push(row(KitField::Cams, "Sticky cameras", Some("cams"), counts(cams)));
        }
        if wand_row(g, None) { rows.push(row(KitField::Wand, "Optical wand", Some("wand"), flags(false))); }
        rows.push(suit_row());
        rows.push(trim_row());
        return rows;
    }
    let mut rows = vec![
        row(KitField::Weapon, "Weapon", Some("shotgun"), words(&["rifle", "pistol", "shotgun"])),
        row(KitField::Suppressed, "Suppressor", None, flags(true)),
        row(KitField::Light, "Weapon light", None, flags(true)),
        row(KitField::Nvg, "Night vision", None, flags(true)),
    ];
    let bangs = slot_max(g, "bangs", i, BANGS_MAX);
    rows.push(row(KitField::Bangs, "Stun canisters", Some("bangs"), counts(bangs)));
    if slot_max(g, "smoke", i, SMOKE_MATE_MAX) > 0.0 {
        let smoke = slot_max(g, "smoke", i, SMOKE_MATE_MAX);
        rows.push(row(KitField::Smoke, "Smoke canisters", Some("smoke"), counts(smoke)));
    }
    rows.extend(charge_rows(g, i));
    let frags = slot_max(g, "frags", i, FRAGS_MAX - 1.0);
    rows.push(row(KitField::Frags, "Frag grenades", Some("frags"), counts(frags)));
    if wand_row(g, Some(i)) { rows.push(row(KitField::Wand, "Optical wand", Some("wand"), flags(false))); }
    rows.push(suit_row());
    rows.push(trim_row());
    rows
}

/// The rows with the big slot on: the build before's, asked with the switch held off for the call.   (kit.ts buildRowsBig)
pub fn build_rows_big(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    g.m.kit.big_on = false;
    let mut rows = build_rows(g, i);
    g.m.kit.big_on = true;
    rows.retain(|r| r.field != KitField::Walls);
    for r in rows.iter_mut() {
        if r.field != KitField::Weapon { continue; }
        r.item = None;
        r.values = words(&["rifle", "pistol"]);
    }
    let big = big_row(g, i);
    rows.push(big);
    rows
}

/// The big slot's card with its one gun row parted in two.   (kit.ts buildRowsGuns)
pub fn build_rows_guns(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    let picks_primary = crate::gunslots::gun_picks(g, crate::gunslots::GunSlot::Primary);
    let primary = KitRow { field: KitField::Primary, label: String::from("Primary"), values: picks_primary.into_iter().map(KitValue::Text).collect(), item: None };
    let picks_secondary = crate::gunslots::gun_picks(g, crate::gunslots::GunSlot::Secondary);
    let secondary = KitRow { field: KitField::Secondary, label: String::from("Secondary"), values: picks_secondary.into_iter().map(KitValue::Text).collect(), item: None };
    let mut out = Vec::new();
    for r in build_rows_big(g, i) {
        if r.field == KitField::Weapon {
            out.push(primary.clone());
            out.push(secondary.clone());
        } else {
            out.push(r);
        }
    }
    out
}

/// The first row of `base` for a field (the `by` of kit.ts buildRowsTree).
fn by(base: &[KitRow], f: KitField) -> Option<KitRow> {
    base.iter().find(|r| r.field == f).cloned()
}

/// A suppressor's row under its gun (the `canRow` of kit.ts buildRowsTree).
fn can_row(slot: crate::gunslots::GunSlot) -> KitRow {
    let field = match slot {
        crate::gunslots::GunSlot::Primary => KitField::Suppressed,
        crate::gunslots::GunSlot::Secondary => KitField::SideCan,
    };
    KitRow { field, label: String::from("Suppressor"), values: flags(true), item: None }
}

/// A launcher's load: a most of none is no row (the `tube` of kit.ts buildRowsTree).
fn tube(r: Option<KitRow>) -> Vec<KitRow> {
    match r {
        Some(r) if r.values.len() > 1 => vec![r],
        _ => Vec::new(),
    }
}

/// One list for every man: each gun with what rides on it, what fills the pouches, what is worn, the big slot.   (kit.ts buildRowsTree)
pub fn build_rows_tree(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    let base = build_rows_guns(g, i);
    let you = i == 0.0;
    let light = by(&base, KitField::Light).unwrap_or_else(|| row(KitField::Light, "Weapon light", None, flags(true)));
    let nvg = by(&base, KitField::Nvg).unwrap_or_else(|| row(KitField::Nvg, "Night vision", None, flags(true)));
    let rings_most = slot_max(g, "rings", i, own_most("rings", !you));
    let rings = row(KitField::Rings, "Sponge rounds", Some("rings"), counts(rings_most));
    let shells = match by(&base, KitField::Shells) {
        Some(r) => r,
        None => {
            let most = if mate_shells(g) { slot_max(g, "shells", i, own_most("shells", true)) } else { 0.0 };
            row(KitField::Shells, "40 mm grenades", Some("shells"), counts(most))
        }
    };
    let mut out = Vec::new();
    out.push(by(&base, KitField::Primary).expect("the gun slots' card has a primary row"));
    out.push(can_row(crate::gunslots::GunSlot::Primary));
    out.push(light);
    out.extend(tube(Some(rings)));
    out.extend(tube(Some(shells)));
    out.extend(tube(by(&base, KitField::Cams)));
    out.push(by(&base, KitField::Secondary).expect("the gun slots' card has a secondary row"));
    out.push(can_row(crate::gunslots::GunSlot::Secondary));
    for f in [KitField::Smoke, KitField::Bangs, KitField::Frags, KitField::Charges, KitField::Wand] {
        if let Some(r) = by(&base, f) { out.push(r); }
    }
    out.push(nvg);
    out.push(by(&base, KitField::Suit).expect("every card has a suit row"));
    out.push(by(&base, KitField::Trim).expect("every card has a trim row"));
    out.push(by(&base, KitField::Big).expect("the big slot's card has its row"));
    out
}

/// The rows of member i's card, top to bottom; built once per member and per way the rules stand.   (kit.ts kitRows)
pub fn kit_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    let shells = mate_shells(g);
    let key = alloc::format!("{}:{}:{}:{}:{}", i, g.k.slots.on, wall_here(), g.m.kit.big_on, shells);
    if let Some(r) = g.m.kit.row_cache.get(key.clone()) { return r.clone(); }
    let r = build_rows(g, i);
    g.m.kit.row_cache.set(key, r.clone());
    r
}

/// What a row costs member i's pouches as his card stands, and its rate in plain words; null for a row that is no pouch.   (kit.ts kitRowCost)
pub fn kit_row_cost(g: &mut crate::game::Game, i: f64, r: &crate::kit::KitRow, m: Option<&crate::kit::MemberKit>) -> Option<crate::kit::KitRowCost> {
    let m = match m { Some(c) => c.clone(), None => kit_shown(g, i) };
    let item = r.item.as_ref()?;
    let it = g.k.slots.table.items.get(item.clone()).cloned();
    if !g.k.slots.on { return None; }
    let it = it?;
    if !js::truthy(price_of(g, Some(&it))) { return None; }
    let p = it.pouches;
    let holds = it.holds.filter(|h| js::truthy(*h));
    let rate = match holds {
        Some(h) if stealth_on(g) => alloc::format!("{} to a pouch", h),
        _ if item == "wand" => alloc::format!("the wand takes {}", pouch_words(p)),
        _ if item == "shotgun" => alloc::format!("the shotgun's shells take {}", pouch_words(p)),
        _ if p == 1.0 => String::from("a pouch each"),
        _ => alloc::format!("{} pouches each", p),
    };
    let n = amount_of(g, item, &m);
    Some(KitRowCost { pouches: slot_cost(g, item, n, None), rate })
}

/// Who the card is for: 'You', or the callsigns the man in that place answers to.   (kit.ts kitName)
pub fn kit_name(i: f64) -> crate::kit::KitName {
    if i == 0.0 { return KitName { title: String::from("You"), sub: String::from("point man") }; }
    let at = (i - 1.0) as usize;
    let four = ["Red One", "Red Two", "Blue One", "Blue Two"].get(at).copied().unwrap_or("undefined");
    let pair = ["Blue One", "Blue Two"].get(at).copied();
    let sub = match pair {
        Some(p) => alloc::format!("{} · {} in a pair", four, p),
        None => String::from(four),
    };
    KitName { title: alloc::format!("Teammate {}", i), sub }
}

/// `[…].filter(Boolean).join(sep)`.
fn line_of(parts: &[String], sep: &str) -> String {
    let kept: Vec<&str> = parts.iter().map(|s| s.as_str()).filter(|s| !s.is_empty()).collect();
    kept.join(sep)
}

/// The plural's letter of a count.
fn plural(n: f64) -> &'static str {
    if n == 1.0 { "" } else { "s" }
}

/// A gun by its pick's name, 'bare' after one whose suppressor the card has taken off (the `gun` of kit.ts kitWords).
fn gun_words(g: &mut Game, m: &MemberKit, slot: crate::gunslots::GunSlot) -> String {
    let id = slot_pick(m, slot);
    let word = crate::gunslots::pick_word(g, id, None);
    let name = crate::gear::gear_text(g, &word);
    let bare = can_choice(g, id) && !kit_slot_can(g, m, slot);
    alloc::format!("{}{}", name, if bare { " bare" } else { "" })
}

/// The card's guns in the tree's words (the `guns` of kit.ts kitWords).
fn guns_words(g: &mut Game, m: &MemberKit) -> String {
    if m.weapon == KitWeapon::Pistol {
        return alloc::format!("{} only", gun_words(g, m, crate::gunslots::GunSlot::Secondary));
    }
    let primary = gun_words(g, m, crate::gunslots::GunSlot::Primary);
    let secondary = gun_words(g, m, crate::gunslots::GunSlot::Secondary);
    alloc::format!("{} + {}", primary, secondary)
}

/// A pick's name in a card's words.
fn pick_words(g: &mut Game, id: &str) -> String {
    let word = crate::gunslots::pick_word(g, id, None);
    crate::gear::gear_text(g, &word)
}

/// GUN_SHORT[w] in a card's words.
fn gun_short_words(g: &mut Game, w: crate::weapons::WeaponId) -> String {
    let short = gun_short().get(String::from(w.as_str())).cloned().unwrap_or_default();
    crate::gear::gear_text(g, &short)
}

/// A card in a line.   (kit.ts kitWords)
pub fn kit_words(g: &mut crate::game::Game, i: f64, m: Option<&crate::kit::MemberKit>) -> String {
    let m = match m { Some(c) => c.clone(), None => kit_shown(g, i) };
    let suit = if m.suit != SuitId::Issued { suits().get(String::from(m.suit.as_str())).map(|s| s.label.clone()).unwrap_or_default() } else { String::new() };
    let trim = if m.trim != TrimId::Issued { alloc::format!("{} trim", trims().get(String::from(m.trim.as_str())).map(|s| s.label.clone()).unwrap_or_default()) } else { String::new() };
    let look = line_of(&[suit, trim], ", ");
    let slots_on = g.k.slots.on;
    let frags = if slots_on && js::truthy(m.frags) { alloc::format!(", {} frag{}", m.frags, plural(m.frags)) } else { String::new() };
    if i == 0.0 {
        let guns = if tree_up(g) {
            guns_words(g, &m)
        } else if guns_on(g) {
            if m.weapon == KitWeapon::Pistol {
                alloc::format!("{} only", pick_words(g, &m.secondary))
            } else {
                let primary = pick_words(g, &m.primary);
                let secondary = pick_words(g, &m.secondary);
                alloc::format!("{} + {}", primary, secondary)
            }
        } else {
            let line = you_guns().get(String::from(m.weapon.as_str())).cloned().unwrap_or_default();
            crate::gear::gear_text(g, &line)
        };
        let or_no = |x: f64| if js::truthy(x) { alloc::format!("{}", x) } else { String::from("no") };
        let shells = if slots_on && js::truthy(m.shells) { alloc::format!(", {}\u{a0}×\u{a0}40\u{a0}mm", m.shells) } else { String::new() };
        let sponge = if js::truthy(m.rings) && tree_up(g) { alloc::format!(", {}\u{a0}sponge", m.rings) } else { String::new() };
        let charges = if charge_row(g, 0.0) && js::truthy(m.charges) { alloc::format!(", {} charge{}", m.charges, plural(m.charges)) } else { String::new() };
        let cams = if js::truthy(m.cams) && cam_row(g) { alloc::format!(", {}\u{a0}cam{}", m.cams, plural(m.cams)) } else { String::new() };
        let wand = if m.wand && wand_row(g, None) { String::from(", wand") } else { String::new() };
        let carried = alloc::format!("{} smoke, {} stun{}{}{}{}{}{}", or_no(m.smoke), or_no(m.bangs), frags, shells, sponge, charges, cams, wand);
        let walls = wall_words(g, 0.0, &m);
        let big = big_words(g, &m);
        return line_of(&[guns, carried, walls, big, look], " · ");
    }
    // (GUNSLOTS.bare is settled on)
    let bare = if !tree_up(g) && guns_on(g) && !m.suppressed {
        let mut on = m.clone();
        on.suppressed = true;
        if kit_gun(g, &m) != kit_gun(g, &on) || kit_side(g, &m) != kit_side(g, &on) { String::from("no suppressor") } else { String::new() }
    } else {
        String::new()
    };
    let guns = if tree_up(g) {
        guns_words(g, &m)
    } else {
        let w = kit_gun(g, &m);
        let first = gun_short_words(g, w);
        match kit_side(g, &m) {
            Some(side) => alloc::format!("{} + {}", first, gun_short_words(g, side)),
            None => first,
        }
    };
    let smoke = if js::truthy(m.smoke) { alloc::format!(", {} smoke", m.smoke) } else { String::new() };
    let shells = if js::truthy(m.shells) && mate_shells(g) { alloc::format!(", {}\u{a0}×\u{a0}40\u{a0}mm", m.shells) } else { String::new() };
    let carried = alloc::format!("{} stun, {} charge{}{}{}{}", m.bangs, m.charges, plural(m.charges), frags, smoke, shells);
    let walls = wall_words(g, i, &m);
    let big = big_words(g, &m);
    line_of(&[
        guns,
        bare,
        String::from(if m.light { "light" } else { "no light" }),
        String::from(if m.nvg { "goggles" } else { "no goggles" }),
        carried,
        walls,
        big,
        look,
    ], " · ")
}

/// The big slot's shotgun or hammer of a card's line.   (kit.ts bigWords)
pub fn big_words(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> String {
    if g.m.kit.big_on && !js::truthy(m.walls) && m.big != BigKit::None {
        return big_word().get(String::from(m.big.as_str())).cloned().unwrap_or_default();
    }
    String::new()
}

/// The wall charges of a card's line.   (kit.ts wallWords)
pub fn wall_words(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> String {
    if !(js::truthy(m.walls) && wall_row(g, i)) { return String::new(); }
    if m.walls == 1.0 { String::from("wall charge") } else { alloc::format!("{} wall charges", m.walls) }
}

/// Member i's card differs from what he is issued (only the rows his card has).   (kit.ts kitChanged)
pub fn kit_changed(g: &mut crate::game::Game, i: f64) -> bool {
    let m = kit_shown(g, i);
    let d = kit_issued_shown(g, i, None);
    kit_rows(g, i).iter().any(|r| row_has(r, &m) && !same_value(&row_get(r, &m), &row_get(r, &d)))
}

/// Why member i's card with one row set to `v` is refused, or null: it fits.   (kit.ts overWhy)
pub fn over_why(g: &mut crate::game::Game, i: f64, row: &crate::kit::KitRow, v: &crate::kit::KitValue) -> Option<String> {
    if !g.k.slots.on { return None; }
    let mut m = g.m.kit.kit.members[i as usize].clone();
    row_set(g, row, &mut m, v);
    card_why(g, i, &m)
}

/// A row of member i's card by its key: your canister rows of the other way the rule stands too.   (kit.ts rowOf)
pub fn row_of(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField) -> Option<crate::kit::KitRow> {
    if let Some(r) = kit_rows(g, i).into_iter().find(|r| r.field == field) { return Some(r); }
    if i != 0.0 { return None; }
    let mut other = vec![cans_row()];
    other.extend(you_can_rows(g));
    other.into_iter().find(|r| r.field == field)
}

/// What a step of one row of member i's card would do, nothing written, as his card is worn.   (kit.ts kitPeek)
pub fn kit_peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitPeek {
    let p = on_worn(g, i, &mut |g: &mut Game| peek(g, i, field, dir), None);
    KitPeek { value: p.value, why: p.why, passed: p.passed }
}

/// A step of one row: the value it lands on, wrapping; or `why` and the value as it stands.   (kit.ts peek)
pub fn peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::Peek {
    let row = match row_of(g, i, field) {
        Some(r) => r,
        None => panic!("kit: member {} has no '{}' row", i, field.as_str()),
    };
    let card = g.m.kit.kit.members[i as usize].clone();
    let cur = row_get(&row, &card);
    let n = row.values.len();
    let at = row.values.iter().position(|x| same_value(x, &cur)).unwrap_or(0);
    let counted = matches!(row.values.first(), Some(KitValue::Num(_)));
    let stuck = if !row_has(&row, &card) {
        alloc::format!("no {} on this card as it stands", row.label.to_lowercase())
    } else {
        row_fixed(g, &row, &card)
    };
    if !stuck.is_empty() { return Peek { value: cur, why: stuck, passed: String::new() }; }
    let mut why = String::new();
    let mut passed = String::new();
    for k in 1..n {
        let nf = n as f64;
        let to = row.values[js::rem(js::rem(at as f64 + dir * k as f64, nf) + nf, nf) as usize].clone();
        let w = over_why(g, i, &row, &to).unwrap_or_default();
        if w.is_empty() { return Peek { value: to, why: String::new(), passed }; }
        if counted && dir < 0.0 && at == 0 { why = w; continue; }
        if why.is_empty() { why = w.clone(); }
        if counted { break; }
        if passed.is_empty() { passed = alloc::format!("{}: {}", row_word(g, i, &row, &to), w); }
    }
    Peek { value: cur, why, passed: String::new() }
}

/// Step one row of member i's card; kept. `ok` false: refused (`why`), nothing written.   (kit.ts kitTry)
pub fn kit_try(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitTry {
    on_worn(g, i, &mut |g: &mut Game| {
        let p = peek(g, i, field, dir);
        if !p.why.is_empty() { return KitTry { ok: false, value: p.value, why: p.why, passed: p.passed }; }
        let row = row_of(g, i, field).expect("peek found the row");
        write_row(g, i, &row, &p.value);
        KitTry { ok: true, value: p.value, why: p.why, passed: p.passed }
    }, Some(&mut |_g: &mut Game, r: KitTry| r.ok))
}

/// `rowSet(row, KIT.members[i], v); KIT_NOTE[i] = ''; kitSave()`: row_set works on the card out of the table and it goes back. (kitSave is not in the game.)
fn write_row(g: &mut Game, i: f64, row: &KitRow, v: &KitValue) {
    let at = i as usize;
    let mut card = g.m.kit.kit.members[at].clone();
    row_set(g, row, &mut card, v);
    g.m.kit.kit.members[at] = card;
    g.m.kit.kit_note[at] = String::new();
}

/// Step one row to its next or last value, wrapping; kept. Returns the value the row now has.   (kit.ts kitStep)
pub fn kit_step(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitValue {
    kit_try(g, i, field, dir).value
}

/// Set one row outright; refused for a value the row does not offer, or one his pouches have no room for.   (kit.ts kitSet)
pub fn kit_set(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool {
    on_worn(g, i, &mut |g: &mut Game| set_row(g, i, field, v), Some(&mut |_g: &mut Game, ok: bool| ok))
}

/// (kit.ts setRow)
pub fn set_row(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool {
    let at = i as usize;
    let is_word = |w: &str| matches!(v, KitValue::Text(s) if s == w);
    if g.m.kit.big_on && (field == KitField::Walls || (field == KitField::Weapon && is_word("shotgun"))) {
        // (BIGSLOT: a hand that still names the rows of the build before is given the one big row)
        let now = big_of(&g.m.kit.kit.members[at]);
        let mine = if field == KitField::Walls { BigPick::Wall } else { BigPick::Shotgun };
        let pick = if value_truthy(v) { mine } else if now == mine { BigPick::None } else { now };
        return set_row(g, i, KitField::Big, &text_value(pick.as_str()));
    }
    if field == KitField::Weapon && guns_on(g) && (is_word("rifle") || is_word("pistol")) {
        // (GUNSLOTS: the same for the one gun row; gunsHome writes the card it is handed)
        let pick = if is_word("pistol") {
            String::from(crate::gunslots::NO_PRIMARY)
        } else {
            let mut card = g.m.kit.kit.members[at].clone();
            let home = guns_home(g, &mut card);
            g.m.kit.kit.members[at] = card;
            home.primary
        };
        return set_row(g, i, KitField::Primary, &KitValue::Text(pick));
    }
    let Some(row) = row_of(g, i, field) else { return false; };
    if !offers(&row.values, v) { return false; }
    let card = g.m.kit.kit.members[at].clone();
    if (!row_has(&row, &card) || !row_fixed(g, &row, &card).is_empty()) && !same_value(v, &row_get(&row, &card)) { return false; }
    if !over_why(g, i, &row, v).unwrap_or_default().is_empty() { return false; }
    write_row(g, i, &row, v);
    true
}

/// SLOTS.on ? SLOTS.table : null, out of the game for the length of a call.
fn table_in_force(g: &Game) -> Option<SlotTable> {
    if g.k.slots.on { Some(g.k.slots.table.clone()) } else { None }
}

/// Member i's card as a set makes it.   (kit.ts kitPresetCard)
pub fn kit_preset_card(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> crate::kit::MemberKit {
    let t = table_in_force(g);
    set_card(g, i, p, t.as_ref())
}

/// Put a set on member i's card; kept. `ok` false: refused (`why`), nothing written.   (kit.ts kitPreset)
pub fn kit_preset(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> crate::kit::KitPreset2 {
    on_worn(g, i, &mut |g: &mut Game| {
        let c = kit_preset_card(g, i, p);
        let why = card_why(g, i, &c).unwrap_or_default();
        if !why.is_empty() { return KitPreset2 { ok: false, why }; }
        let at = i as usize;
        g.m.kit.kit.members[at] = c;
        g.m.kit.kit_note[at] = String::new();
        KitPreset2 { ok: true, why: String::new() }
    }, Some(&mut |_g: &mut Game, r: KitPreset2| r.ok))
}

/// Why a press of a set on member i's card would be refused as the cards stand ('' = it would be taken).   (kit.ts kitPresetWhy)
pub fn kit_preset_why(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> String {
    on_worn(g, i, &mut |g: &mut Game| {
        let c = kit_preset_card(g, i, p);
        card_why(g, i, &c).unwrap_or_default()
    }, None)
}

/// A set on all five at once (always a legal fill).   (kit.ts kitPresetAll)
pub fn kit_preset_all(g: &mut crate::game::Game, p: crate::kit::KitPreset) {
    for at in 0..(KIT_MEMBERS as usize) {
        let t = table_in_force(g);
        let c = set_card_base(g, at as f64, p, t.as_ref());
        g.m.kit.kit.members[at] = c;
        g.m.kit.kit_note[at] = String::new();
    }
    g.m.kit.deal_own = false;
}

/// Which set member i's card is, if it is exactly one.   (kit.ts kitPresetOf)
pub fn kit_preset_of(g: &mut crate::game::Game, i: f64) -> Option<crate::kit::KitPreset> {
    let m = kit_shown(g, i);
    for p in [KitPreset::Issued, KitPreset::Quiet, KitPreset::Loud] {   // (KIT_PRESETS' order)
        let c = kit_preset_card(g, i, p);
        let same = kit_rows(g, i).iter().all(|r| (!row_has(r, &c) && !row_has(r, &m)) || same_value(&row_get(r, &c), &row_get(r, &m)));
        if same { return Some(p); }
    }
    None
}

/// Member i's card as another man's card makes it: each row of his own takes the copied value where it offers it, a count over his row's most comes down.   (kit.ts kitPasteCard)
pub fn kit_paste_card(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::MemberKit {
    let mut m = kit_shown(g, i);
    for r in kit_rows(g, i) {
        if r.field == KitField::Suit || r.field == KitField::Trim { continue; }
        let v = row_get(&r, from);
        if offers(&r.values, &v) {
            row_set(g, &r, &mut m, &v);
        } else if let (KitValue::Num(n), Some(KitValue::Num(first))) = (&v, r.values.first()) {
            let mut most: Vec<f64> = Vec::new();
            for x in &r.values {
                if let KitValue::Num(x) = x { if *x <= *n { most.push(*x); } }
            }
            most.push(*first);
            row_set(g, &r, &mut m, &KitValue::Num(m::max_n(&most)));
        }
    }
    if g.k.slots.on { home_shells(g, &mut m) } else { m }
}

/// The rows of member i's card that read differently on two cards of his.   (kit.ts rowsMoved)
pub fn rows_moved(g: &mut crate::game::Game, i: f64, a: &crate::kit::MemberKit, b: &crate::kit::MemberKit) -> Vec<crate::kit::KitField> {
    kit_rows(g, i).iter()
        .filter(|r| (row_has(r, a) || row_has(r, b)) && !same_value(&row_get(r, a), &row_get(r, b)))
        .map(|r| r.field)
        .collect()
}

/// What a paste would do, nothing written, as his card is worn.   (kit.ts kitPastePeek)
pub fn kit_paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::KitPastePeek {
    let p = on_worn(g, i, &mut |g: &mut Game| paste_peek(g, i, from), None);
    KitPastePeek { card: p.card, changed: p.changed, why: p.why }
}

/// The card a paste would make, the rows of his it would move, why it would be refused ('' = taken).   (kit.ts pastePeek)
pub fn paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::PastePeek {
    let card = kit_paste_card(g, i, from);
    let now = g.m.kit.kit.members[i as usize].clone();
    let changed = rows_moved(g, i, &now, &card);
    let why = card_why(g, i, &card).unwrap_or_default();
    PastePeek { card, changed, why }
}

/// Paste a copied card onto member i's; kept, in one write of his card. `ok` false: refused (`why`), nothing written.   (kit.ts kitPaste)
pub fn kit_paste(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::KitPaste {
    on_worn(g, i, &mut |g: &mut Game| {
        let p = paste_peek(g, i, from);
        if !p.why.is_empty() { return KitPaste { ok: false, why: p.why, changed: Vec::new() }; }
        if !p.changed.is_empty() {
            let at = i as usize;
            g.m.kit.kit.members[at] = p.card;
            g.m.kit.kit_note[at] = String::new();
        }
        KitPaste { ok: true, why: String::new(), changed: p.changed }
    }, Some(&mut |_g: &mut Game, r: KitPaste| r.ok && !r.changed.is_empty()))
}

/// Member i already wears a copied card: a paste of it would move no row of his.   (kit.ts kitWears)
pub fn kit_wears(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> bool {
    kit_paste_peek(g, i, from).changed.is_empty()
}

/// The browser's storage: an interface with no body in the TypeScript.   (kit.ts Store.getItem)
pub fn store_get_item(g: &mut crate::game::Game, k: &str) -> Option<String> { todo!("needs: the host's storage (kit.ts:1072 is an interface)") }

/// The browser's storage: an interface with no body in the TypeScript.   (kit.ts Store.setItem)
pub fn store_set_item(g: &mut crate::game::Game, k: &str, v: &str) { todo!("needs: the host's storage (kit.ts:1072 is an interface)") }

/// The browser's storage: an interface with no body in the TypeScript.   (kit.ts Store.removeItem)
pub fn store_remove_item(g: &mut crate::game::Game, k: &str) { todo!("needs: the host's storage (kit.ts:1072 is an interface)") }

/// `out[field] = v` of a stored value, for a row with no reader of its own (one its value was found on, so the value's kind fits the field).
fn put_field(card: &mut MemberKit, field: KitField, v: &KitValue) {
    match (field, v) {
        (KitField::Weapon, KitValue::Text(s)) => {
            if let Some(w) = [KitWeapon::Pistol, KitWeapon::Rifle, KitWeapon::Shotgun].into_iter().find(|w| w.as_str() == s) { card.weapon = w; }
        }
        (KitField::Suit, KitValue::Text(s)) => {
            if let Some(w) = [SuitId::Issued, SuitId::Black, SuitId::Grey, SuitId::Navy, SuitId::Olive, SuitId::Jungle, SuitId::Khaki, SuitId::Desert].into_iter().find(|w| w.as_str() == s) { card.suit = w; }
        }
        (KitField::Trim, KitValue::Text(s)) => {
            if let Some(w) = [TrimId::Red, TrimId::Blue, TrimId::Green, TrimId::Issued, TrimId::Black, TrimId::Amber, TrimId::White].into_iter().find(|w| w.as_str() == s) { card.trim = w; }
        }
        (KitField::Suppressed, KitValue::Bool(b)) => card.suppressed = *b,
        (KitField::Light, KitValue::Bool(b)) => card.light = *b,
        (KitField::Nvg, KitValue::Bool(b)) => card.nvg = *b,
        (KitField::Wand, KitValue::Bool(b)) => card.wand = *b,
        (KitField::Bangs, KitValue::Num(n)) => card.bangs = *n,
        (KitField::Charges, KitValue::Num(n)) => card.charges = *n,
        (KitField::Smoke, KitValue::Num(n)) => card.smoke = *n,
        (KitField::Frags, KitValue::Num(n)) => card.frags = *n,
        (KitField::Walls, KitValue::Num(n)) => card.walls = *n,
        (KitField::Shells, KitValue::Num(n)) => card.shells = *n,
        (KitField::Cams, KitValue::Num(n)) => card.cams = *n,
        (KitField::Rings, KitValue::Num(n)) => card.rings = *n,
        _ => {}
    }
}

/// A stored (or hand-typed) card made safe: every field one of its row's values, anything else the issued one.   (kit.ts kitSanitize)
pub fn kit_sanitize(g: &mut crate::game::Game, i: f64, raw: Option<&JsMap<String, crate::kit::KitValue>>) -> crate::kit::MemberKit {
    let mut card = kit_default(g, i);
    let Some(raw) = raw else { return card; };
    let at = |k: &str| raw.get(String::from(k));
    let word = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Text(s)) => Some(s.as_str()), _ => None };
    let flag = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Bool(b)) => Some(*b), _ => None };
    let num = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Num(n)) => *n, _ => 0.0 };

    let you = i == 0.0;
    let old = you && at("smoke").is_none();
    if old {
        if let Some(x) = word("cans").and_then(|c| can_mixes().get(String::from(c)).cloned()) { card.smoke = x.smoke; card.bangs = x.stun; }
    }
    let mut rows = kit_rows(g, i);
    if you && !g.k.slots.on { rows.extend(you_can_rows(g)); }   // (rule off: your two counts have no row of their own, and are still checked)
    for r in &rows {
        if row_reads_itself(g, r) || (old && (r.field == KitField::Smoke || r.field == KitField::Bangs)) { continue; }
        let key = r.field.as_str();
        let v = at(key);
        if let Some(v) = v.filter(|v| offers(&r.values, v)) {
            put_field(&mut card, r.field, v);
        } else if you && matches!(r.field, KitField::Smoke | KitField::Bangs | KitField::Frags | KitField::Shells) && int(v, 0.0) {
            put_field(&mut card, r.field, &KitValue::Num(m::min(num(key), own_most(key, false))));
        }
    }
    if you && int(at("cams"), 1.0) && !cam_row(g) { card.cams = m::min(num("cams"), CAMS_MAX); }
    // THE BIG SLOT'S TWO FIELDS: a row of choices over them, not read by the loop above.
    match word("big") {
        Some("shotgun") => card.big = BigKit::Shotgun,
        Some("hammer") => card.big = BigKit::Hammer,
        _ => {}
    }
    if i > 0.0 && flag("bare") == Some(true) { card.bare = Some(true); }
    if g.m.kit.big_on {
        let big_values = rows.iter().find(|r| r.field == KitField::Big).map(|r| r.values.clone()).unwrap_or_default();
        if card.big != BigKit::None && !offers(&big_values, &text_value(card.big.as_str())) { card.big = BigKit::None; }
        if int(at("walls"), 1.0) && offers(&big_values, &text_value("wall")) { card.walls = m::min(num("walls"), WALLS_MAX); }
        if word("weapon") == Some("shotgun") && offers(&big_values, &text_value("shotgun")) {
            card.weapon = KitWeapon::Shotgun;
            big_home(&mut card, i);
        }
    }
    // THE TWO GUN SLOTS: rows of choices, not read by the loop above.
    if guns_on(g) {
        match word("weapon") {
            Some("rifle") => card.weapon = KitWeapon::Rifle,
            Some("pistol") => card.weapon = KitWeapon::Pistol,
            _ => {}
        }
    }
    if let Some(p) = word("primary") { card.primary = String::from(p); }
    if let Some(p) = word("secondary") { card.secondary = String::from(p); }
    guns_home(g, &mut card);
    // THE TREE'S TWO SUPPRESSORS: rows that read the gun, not the field, so not read by the loop above.
    match flag("sideCan") {
        Some(side_can) => {
            card.side_can = side_can;
            if let Some(b) = flag("suppressed") { card.suppressed = b; }
            if !tree_up(g) {
                if int(at("rings"), 1.0) { card.rings = m::min(num("rings"), RINGS_MAX); }
                if you {
                    if let Some(b) = flag("light") { card.light = b; }
                    if let Some(b) = flag("nvg") { card.nvg = b; }
                }
            }
        }
        None => {
            if tree_up(g) {
                if you {
                    let d = kit_default(g, 0.0);
                    card.suppressed = d.suppressed;
                    card.side_can = d.side_can;
                    card.light = d.light;
                    card.nvg = d.nvg;
                } else if let Some(b) = flag("suppressed") {
                    card.suppressed = b;
                    card.side_can = b;
                }
            }
        }
    }
    home_cans(g, &mut card);
    if you && flag("wand") == Some(true) && !wand_row(g, None) { card.wand = true; }
    card
}

/// `new URLSearchParams(search).has(word)`: the words kit_load asks for need no decoding.
fn search_has(search: &str, word: &str) -> bool {
    let body = search.strip_prefix('?').unwrap_or(search);
    body.split('&').any(|pair| !pair.is_empty() && pair.split('=').next() == Some(word))
}

/// SLOTS.on's setter (kit.ts:294): what you are issued follows the switch, so your card crosses when its four counts are as issued the way the rule stood.
fn slots_set_on(g: &mut Game, v: bool) {
    g.k.slots.on = v;   // (the TypeScript has one store, read through the getter: `ruleOn`)
    if v == g.m.kit.rule_on { return; }
    let m = g.m.kit.kit.members[0].clone();
    let was = issued_card(g, 0.0);
    let same = m.smoke == was.smoke && m.bangs == was.bangs && m.frags == was.frags && m.shells == was.shells;
    g.m.kit.rule_on = v;
    if same {
        let now = issued_card(g, 0.0);
        let card = &mut g.m.kit.kit.members[0];
        card.smoke = now.smoke;
        card.bangs = now.bangs;
        card.frags = now.frags;
        card.shells = now.shells;
    }
    if v {
        let mut card = g.m.kit.kit.members[0].clone();
        let home = home_shells(g, &mut card);
        g.m.kit.kit.members[0] = home;
    }
}

/// needs: JSON.parse of the store's text (kit.ts:1131): the crate has no JSON. None = unreadable. Gives `members` (a member not an object: None), `own === true`, `stealth === true`, and `stealth` as an object.
fn seam_parse_store(raw: &str) -> Option<(Vec<Option<JsMap<String, KitValue>>>, bool, bool, Option<JsMap<String, KitValue>>)> {
    todo!("needs: a parse of the stored kit's JSON")
}

/// needs: kitFitAll(KIT.members) writing the table's own cards (kit.ts:661, read back at 1136): kit_fit_all takes `cards: &[MemberKit]`, so its writes cannot land.
fn seam_fit_members(g: &mut Game) -> Vec<Vec<String>> {
    todo!("needs: kit_fit_all taking &mut [MemberKit]")
}

/// Read the table from the store and the address: `?nokit`, `?noslots`; a stored kit over a man's pouches is brought inside them here, in memory.   (kit.ts kitLoad)
pub fn kit_load(g: &mut crate::game::Game, search: Option<&str>) {
    let search = search.unwrap_or("");
    g.m.kit.hand = None;
    g.m.kit.loud_you = None;
    g.m.kit.stealth_you = None;
    g.m.kit.kit.on = !search_has(search, "nokit");
    let on = g.k.slots.ok && !search_has(search, "noslots");
    slots_set_on(g, on);
    g.m.kit.team = KIT_MEMBERS - 1.0;
    let count = KIT_MEMBERS as usize;
    for at in 0..count {
        g.m.kit.kit.members[at] = kit_default(g, at as f64);
        g.m.kit.kit_note[at] = String::new();
    }
    g.m.kit.kept0 = None;
    g.m.kit.kept_wand = Vec::new();
    g.m.kit.deal_own = false;
    let Some(raw) = store_get_item(g, KIT_LS) else { return; };
    if raw.is_empty() { return; }
    let Some((members, own, stealth_true, stealth)) = seam_parse_store(&raw) else { return; };   // (an unreadable store: as issued)
    g.m.kit.deal_own = own;
    for at in 0..count {
        let raw = members.get(at).and_then(|r| r.as_ref());
        g.m.kit.kit.members[at] = kit_sanitize(g, at as f64, raw);
    }
    let first = members.first().and_then(|r| r.as_ref());
    g.m.kit.kept0 = kept_of(g, first);
    let st = if stealth_true { first } else { stealth.as_ref() };
    if st.is_some() {
        g.m.kit.stealth_you = Some(kit_sanitize(g, 0.0, st));
    }
    g.m.kit.kept_wand = members.iter().enumerate()
        .map(|(at, r)| at > 0 && matches!(r.as_ref().and_then(|r| r.get(String::from("wand"))), Some(KitValue::Bool(true))))
        .collect();
    // (as issued the other way the rule stands = as issued)
    let (smoke, bangs, frags, shells) = if g.m.kit.rule_on {
        let b = built();
        (b.smoke, b.bangs, b.frags, b.shells)
    } else {
        let b = &issued()[0];
        (b.smoke, b.bangs, b.frags, b.shells)
    };
    let now = issued_card(g, 0.0);
    let card = &mut g.m.kit.kit.members[0];
    if card.smoke == smoke && card.bangs == bangs && card.frags == frags && card.shells == shells {
        card.smoke = now.smoke;
        card.bangs = now.bangs;
        card.frags = now.frags;
        card.shells = now.shells;
    }
    if !g.k.slots.on { return; }
    let shells_word = slot_word().get(String::from("shells")).cloned().unwrap_or_default();
    for (at, left) in seam_fit_members(g).iter().enumerate() {
        if left.is_empty() { continue; }
        let list = left.join(", ");
        let note = if at == 0 && g.m.kit.kit.members[0].weapon != KitWeapon::Rifle && left.len() == 1 && left[0].contains(shells_word.as_str()) {
            alloc::format!("No FN2000 in your saved kit: {} left behind.", left[0])
        } else {
            let joined = left.join(",");
            let pouch_thing = ["canister", "frag", "grenade", "door charge", "camera", "wand", "sponge"].iter().any(|w| joined.contains(w));
            let what = if g.m.kit.big_on && !pouch_thing { "big slot" } else { "pouches" };
            alloc::format!("Your saved kit was over the {}: {} left behind.", what, list)
        };
        g.m.kit.kit_note[at] = note;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(smoke: f64, bangs: f64) -> MemberKit {
        let mut m = MemberKit::zeroed();
        m.smoke = smoke;
        m.bangs = bangs;
        m
    }

    #[test]
    fn up_to_truncates_as_array_from_does() {
        assert_eq!(up_to(2.0), vec![0.0, 1.0, 2.0]);
        assert_eq!(up_to(0.5), vec![0.0]);
        assert!(up_to(-1.0).is_empty());
        assert!(up_to(f64::NAN).is_empty());
    }

    #[test]
    fn a_card_is_a_split_only_on_both_counts() {
        assert_eq!(mix_of(&card(4.0, 2.0)), Some(CanMix::Issued));
        assert_eq!(mix_of(&card(2.0, 4.0)), Some(CanMix::Stun));
        assert_eq!(mix_of(&card(1.0, 1.0)), None);
    }

    #[test]
    fn a_card_is_named_by_its_place() {
        assert_eq!(kit_name(0.0).title, "You");
        assert_eq!(kit_name(1.0).sub, "Red One · Blue One in a pair");
        assert_eq!(kit_name(3.0).sub, "Blue One");
        assert_eq!(kit_name(4.0).title, "Teammate 4");
    }

    #[test]
    fn the_address_words_are_whole_words() {
        assert!(search_has("?nokit&x=1", "nokit"));
        assert!(search_has("a=1&noslots=0", "noslots"));
        assert!(!search_has("?nokitdeal", "nokit"));
        assert!(!search_has("", "nokit"));
    }
}
