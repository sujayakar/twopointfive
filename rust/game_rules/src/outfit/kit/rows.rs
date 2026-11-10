use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- what a row's stored closures became, and small helpers (private)

/// A counted row's values.
fn counts(max: f64) -> Vec<KitValue> {
    up_to(max).into_iter().map(KitValue::Num).collect()
}

fn text_value(w: &str) -> KitValue {
    KitValue::Text(String::from(w))
}

/// A yes/no row, in the order the TypeScript writes its two values.
fn flags(first: bool) -> Vec<KitValue> {
    vec![KitValue::Bool(first), KitValue::Bool(!first)]
}

fn row(field: KitField, label: &str, item: Option<Thing>, values: Vec<KitValue>) -> KitRow {
    KitRow { field, label: String::from(label), values, item, get: None }
}

/// `a === b` of two row values.
pub(super) fn same_value(a: &KitValue, b: &KitValue) -> bool {
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
pub(super) fn value_text(v: &KitValue) -> String {
    match v {
        KitValue::Text(s) => s.clone(),
        KitValue::Num(n) => js::num_str(*n),
        KitValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
    }
}

/// The pick in a card's gun slot: `m[slot]`.
fn slot_pick(m: &MemberKit, slot: crate::gunslots::GunSlot) -> &str {
    match slot { crate::gunslots::GunSlot::Primary => &m.primary, crate::gunslots::GunSlot::Secondary => &m.secondary }
}

/// A row's value in words.
pub(super) fn row_word(g: &Game, r: &KitRow, v: &KitValue) -> String {
    match r.field {
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
        KitField::Big => {
            let t = value_text(v);
            big_word().get(t.clone()).cloned().unwrap_or(t)
        }
        KitField::Primary | KitField::Secondary => {
            let word = crate::gunslots::pick_word(g, &value_text(v), None);
            crate::gear::gear_text(&word)
        }
        _ => value_text(v),
    }
}

/// Why a suppressor's row cannot be stepped, '' = it can.
pub(super) fn row_fixed(g: &Game, r: &KitRow, m: &MemberKit) -> String {
    let slot = match r.field {
        KitField::SideCan => crate::gunslots::GunSlot::Secondary,
        KitField::Suppressed => crate::gunslots::GunSlot::Primary,
        _ => return String::new(),
    };
    let id = slot_pick(m, slot);
    if can_choice(g, id) { return String::new(); }
    let word = crate::gunslots::pick_word(g, id, None);
    let name = crate::gear::gear_text(&word);
    let says = if kit_slot_can(g, m, slot) { "made with it" } else { "none fits it" };
    alloc::format!("{}: {}", name, says)
}

/// `rowGet(r, KIT.members[at])`: a read may home the card's picks (gunsHome), and that lands on the member.
fn row_get_member(g: &mut Game, r: &KitRow, at: usize) -> KitValue {
    let mut card = g.m.kit.kit.members[at].clone();
    let v = row_get(g, r, &mut card);
    g.m.kit.kit.members[at] = card;
    v
}

// ---- the rows

/// The big slot's row offers this thing to member i.
fn big_can(g: &Game, k: Thing, i: f64) -> bool {
    let at = i as usize;
    let most = g.k.slots.table.items.get(k).and_then(|it| it.max.get(at).copied()).unwrap_or(0.0);
    most > 0.0 && g.k.slots.table.big.get(at).copied().unwrap_or(1.0) > 0.0
}

/// The big slot's one row, the last of a man's card.
fn big_row(g: &crate::game::Game, i: f64) -> crate::kit::KitRow {
    let mut values = vec![text_value("none")];
    if big_can(g, Thing::Shotgun, i) { values.push(text_value("shotgun")); }
    if big_can(g, Thing::Hammer, i) { values.push(text_value("hammer")); }
    if wall_row(g, i) { values.push(text_value("wall")); }
    KitRow { get: Some(KitRowGet::Big), ..row(KitField::Big, "Big slot", None, values) }
}

/// A gun's row, and its suppressor's under it.
fn gun_row(g: &crate::game::Game, slot: crate::gunslots::GunSlot) -> crate::kit::KitRow {
    let (field, label, get) = match slot {
        crate::gunslots::GunSlot::Primary => (KitField::Primary, "Primary", KitRowGet::Primary),
        crate::gunslots::GunSlot::Secondary => (KitField::Secondary, "Secondary", KitRowGet::Secondary),
    };
    let values = crate::gunslots::gun_picks(g, slot).into_iter().map(KitValue::Text).collect();
    KitRow { get: Some(get), ..row(field, label, None, values) }
}
fn can_row(slot: crate::gunslots::GunSlot) -> crate::kit::KitRow {
    let field = match slot {
        crate::gunslots::GunSlot::Primary => KitField::Suppressed,
        crate::gunslots::GunSlot::Secondary => KitField::SideCan,
    };
    KitRow { get: Some(KitRowGet::SlotCan { slot }), ..row(field, "Suppressor", None, flags(true)) }
}

/// A row of a thing he may carry up to so many of; a most of none is no row.
fn counted(g: &crate::game::Game, out: &mut Vec<crate::kit::KitRow>, i: f64, field: KitField, label: &str, thing: Thing) {
    let most = slot_max(g, thing, i);
    if most > 0.0 { out.push(row(field, label, Some(thing), counts(most))); }
}

/// The rows of member i's card, top to bottom, one list for every man: each gun with what rides on it, what fills the pouches, what is worn, the big slot.
pub fn kit_rows(g: &crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> {
    use crate::gunslots::GunSlot::{Primary, Secondary};
    let mut out = vec![gun_row(g, Primary), can_row(Primary), row(KitField::Light, "Weapon light", None, flags(true))];
    counted(g, &mut out, i, KitField::Rings, "Sponge rounds", Thing::Rings);
    counted(g, &mut out, i, KitField::Shells, "40 mm grenades", Thing::Shells);
    counted(g, &mut out, i, KitField::Cams, "Sticky cameras", Thing::Cams);
    out.push(gun_row(g, Secondary));
    out.push(can_row(Secondary));
    counted(g, &mut out, i, KitField::Smoke, "Smoke canisters", Thing::Smoke);
    counted(g, &mut out, i, KitField::Bangs, "Stun canisters", Thing::Bangs);
    counted(g, &mut out, i, KitField::Frags, "Frag grenades", Thing::Frags);
    counted(g, &mut out, i, KitField::Charges, "Door charges", Thing::Charges);
    if wand_row(g, i) { out.push(row(KitField::Wand, "Optical wand", Some(Thing::Wand), flags(false))); }
    out.push(row(KitField::Nvg, "Night vision", None, flags(true)));
    out.push(suit_row());
    out.push(trim_row());
    out.push(big_row(g, i));
    out
}

/// What a row costs member i's pouches as his card stands, and its rate in plain words; null for a row that is no pouch.
pub(super) fn kit_row_cost(g: &mut crate::game::Game, i: f64, r: &crate::kit::KitRow, m: Option<&crate::kit::MemberKit>) -> Option<crate::kit::KitRowCost> {
    let m = match m { Some(c) => c.clone(), None => kit_shown(g, i) };
    let item = r.item?;
    let it = g.k.slots.table.items.get(item).cloned();
    let it = it?;
    if !js::truthy(price_of(Some(&it))) { return None; }
    let p = it.pouches;
    let holds = it.holds.filter(|h| js::truthy(*h));
    let rate = match holds {
        Some(h) => alloc::format!("{} to a pouch", h),
        _ if item == Thing::Wand => alloc::format!("the wand takes {}", pouch_words(p)),
        _ if p == 1.0 => String::from("a pouch each"),
        _ => alloc::format!("{} pouches each", p),
    };
    let n = amount(item, &m);
    Some(KitRowCost { pouches: slot_cost(g, item, n), rate })
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

/// A gun by its pick's name, 'bare' after one whose suppressor the card has taken off.
fn gun_words(g: &Game, m: &MemberKit, slot: crate::gunslots::GunSlot) -> String {
    let id = slot_pick(m, slot);
    let word = crate::gunslots::pick_word(g, id, None);
    let name = crate::gear::gear_text(&word);
    let bare = can_choice(g, id) && !kit_slot_can(g, m, slot);
    alloc::format!("{}{}", name, if bare { " bare" } else { "" })
}

/// The card's guns in words.
fn guns_words(g: &Game, m: &MemberKit) -> String {
    if m.weapon == KitWeapon::Pistol {
        return alloc::format!("{} only", gun_words(g, m, crate::gunslots::GunSlot::Secondary));
    }
    let primary = gun_words(g, m, crate::gunslots::GunSlot::Primary);
    let secondary = gun_words(g, m, crate::gunslots::GunSlot::Secondary);
    alloc::format!("{} + {}", primary, secondary)
}

/// A card in a line.
pub(super) fn kit_words(g: &mut crate::game::Game, i: f64, m: Option<&crate::kit::MemberKit>) -> String {
    let m = match m { Some(c) => c.clone(), None => kit_shown(g, i) };
    kit_card_words(g, i, &m)
}
/// A card he has been handed, in a line
pub fn kit_card_words(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> String {
    let suit = if m.suit != SuitId::Issued { suits().get(String::from(m.suit.as_str())).map(|s| s.label.clone()).unwrap_or_default() } else { String::new() };
    let trim = if m.trim != TrimId::Issued { alloc::format!("{} trim", trims().get(String::from(m.trim.as_str())).map(|s| s.label.clone()).unwrap_or_default()) } else { String::new() };
    let look = line_of(&[suit, trim], ", ");
    let frags = if js::truthy(m.frags) { alloc::format!(", {} frag{}", m.frags, plural(m.frags)) } else { String::new() };
    if i == 0.0 {
        let guns = guns_words(g, &m);
        let or_no = |x: f64| if js::truthy(x) { alloc::format!("{}", x) } else { String::from("no") };
        let shells = if js::truthy(m.shells) { alloc::format!(", {}\u{a0}×\u{a0}40\u{a0}mm", m.shells) } else { String::new() };
        let sponge = if js::truthy(m.rings) { alloc::format!(", {}\u{a0}sponge", m.rings) } else { String::new() };
        let charges = if charge_row(g, 0.0) && js::truthy(m.charges) { alloc::format!(", {} charge{}", m.charges, plural(m.charges)) } else { String::new() };
        let cams = if js::truthy(m.cams) && cam_row(g) { alloc::format!(", {}\u{a0}cam{}", m.cams, plural(m.cams)) } else { String::new() };
        let wand = if m.wand && wand_row(g, 0.0) { String::from(", wand") } else { String::new() };
        let carried = alloc::format!("{} smoke, {} stun{}{}{}{}{}{}", or_no(m.smoke), or_no(m.bangs), frags, shells, sponge, charges, cams, wand);
        let walls = wall_words(g, 0.0, &m);
        let big = big_words(&m);
        return line_of(&[guns, carried, walls, big, look], " · ");
    }
    let guns = guns_words(g, &m);
    let smoke = if js::truthy(m.smoke) { alloc::format!(", {} smoke", m.smoke) } else { String::new() };
    let shells = if js::truthy(m.shells) { alloc::format!(", {}\u{a0}×\u{a0}40\u{a0}mm", m.shells) } else { String::new() };
    let carried = alloc::format!("{} stun, {} charge{}{}{}{}", m.bangs, m.charges, plural(m.charges), frags, smoke, shells);
    let walls = wall_words(g, i, &m);
    let big = big_words(&m);
    line_of(&[
        guns,
        String::from(if m.light { "light" } else { "no light" }),
        String::from(if m.nvg { "goggles" } else { "no goggles" }),
        carried,
        walls,
        big,
        look,
    ], " · ")
}

/// The big slot's shotgun or hammer of a card's line.
fn big_words(m: &crate::kit::MemberKit) -> String {
    if !js::truthy(m.walls) && m.big != BigKit::None {
        return big_word().get(String::from(m.big.as_str())).cloned().unwrap_or_default();
    }
    String::new()
}

/// The wall charges of a card's line.
fn wall_words(g: &crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> String {
    if !(js::truthy(m.walls) && wall_row(g, i)) { return String::new(); }
    if m.walls == 1.0 { String::from("wall charge") } else { alloc::format!("{} wall charges", m.walls) }
}

/// Member i's card differs from what he is issued (only the rows his card has).
pub(super) fn kit_changed(g: &mut crate::game::Game, i: f64) -> bool {
    let mut m = kit_shown(g, i);
    let mut d = kit_issued_shown(g, i, None);
    kit_rows(g, i).iter().any(|r| row_has(g, r, &m) && { let (a, b) = (row_get(g, r, &mut m), row_get(g, r, &mut d)); !same_value(&a, &b) })
}

/// Why member i's card with one row set to `v` is refused, or null: it fits.
fn over_why(g: &mut crate::game::Game, i: f64, row: &crate::kit::KitRow, v: &crate::kit::KitValue) -> Option<String> {
    let mut m = g.m.kit.kit.members[i as usize].clone();
    row_set(g, row, &mut m, v);
    card_why(g, i, &m)
}

/// A row of member i's card by its key.
fn row_of(g: &crate::game::Game, i: f64, field: crate::kit::KitField) -> Option<crate::kit::KitRow> {
    kit_rows(g, i).into_iter().find(|r| r.field == field)
}

/// What a step of one row of member i's card would do, nothing written, as his card is worn.
pub fn kit_peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitPeek {
    let p = on_worn(g, i, &mut |g: &mut Game| peek(g, i, field, dir), None);
    KitPeek { value: p.value, why: p.why }
}

/// A step of one row: the value it lands on, wrapping; or `why` and the value as it stands.
fn peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::Peek {
    let row = match row_of(g, i, field) {
        Some(r) => r,
        None => panic!("kit: member {} has no '{}' row", i, field.as_str()),
    };
    let cur = row_get_member(g, &row, i as usize);
    let card = g.m.kit.kit.members[i as usize].clone();
    let n = row.values.len();
    let at = row.values.iter().position(|x| same_value(x, &cur)).unwrap_or(0);
    let counted = matches!(row.values.first(), Some(KitValue::Num(_)));
    let stuck = if !row_has(g, &row, &card) {
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
        if passed.is_empty() { passed = alloc::format!("{}: {}", row_word(g, &row, &to), w); }
    }
    Peek { value: cur, why, passed: String::new() }
}

/// Step one row of member i's card; kept. `ok` false: refused (`why`), nothing written.
pub fn kit_try(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitTry {
    on_worn(g, i, &mut |g: &mut Game| {
        let p = peek(g, i, field, dir);
        if !p.why.is_empty() { return KitTry { ok: false, why: p.why, passed: p.passed }; }
        let row = row_of(g, i, field).expect("peek found the row");
        write_row(g, i, &row, &p.value);
        KitTry { ok: true, why: p.why, passed: p.passed }
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

/// Set one row outright; refused for a value the row does not offer, or one his pouches have no room for.
pub fn kit_set(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool {
    on_worn(g, i, &mut |g: &mut Game| set_row(g, i, field, v), Some(&mut |_g: &mut Game, ok: bool| ok))
}

fn set_row(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool {
    let at = i as usize;
    let Some(row) = row_of(g, i, field) else { return false; };
    if !offers(&row.values, v) { return false; }
    let card = g.m.kit.kit.members[at].clone();
    if (!row_has(g, &row, &card) || !row_fixed(g, &row, &card).is_empty()) && !same_value(v, &row_get_member(g, &row, at)) { return false; }
    if !over_why(g, i, &row, v).unwrap_or_default().is_empty() { return false; }
    write_row(g, i, &row, v);
    true
}

/// Put what he is issued on member i's card; kept. `ok` false: refused (`why`), nothing written.
pub fn kit_reset(g: &mut crate::game::Game, i: f64) -> crate::kit::KitReset {
    on_worn(g, i, &mut |g: &mut Game| {
        let c = issued_dealt(g, i);
        let why = card_why(g, i, &c).unwrap_or_default();
        if !why.is_empty() { return KitReset { ok: false, why }; }
        let at = i as usize;
        g.m.kit.kit.members[at] = c;
        g.m.kit.kit_note[at] = String::new();
        KitReset { ok: true, why: String::new() }
    }, Some(&mut |_g: &mut Game, r: KitReset| r.ok))
}

/// Why a reset of member i's card would be refused as the cards stand ('' = it would be taken).
pub(super) fn kit_reset_why(g: &mut crate::game::Game, i: f64) -> String {
    on_worn(g, i, &mut |g: &mut Game| {
        let c = issued_dealt(g, i);
        card_why(g, i, &c).unwrap_or_default()
    }, None)
}

/// Member i's card is what he is issued.
pub(super) fn kit_as_issued(g: &mut crate::game::Game, i: f64) -> bool {
    let mut m = kit_shown(g, i);
    let mut c = issued_dealt(g, i);
    kit_rows(g, i).iter().all(|r| (!row_has(g, r, &c) && !row_has(g, r, &m)) || { let (a, b) = (row_get(g, r, &mut c), row_get(g, r, &mut m)); same_value(&a, &b) })
}

/// Member i's card as another man's card makes it: each row of his own takes the copied value where it offers it, a count over his row's most comes down.
fn kit_paste_card(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::MemberKit {
    let mut m = kit_shown(g, i);
    let mut from = from.clone();   // (a read may home its picks: the caller's card is not ours to write)
    for r in kit_rows(g, i) {
        if r.field == KitField::Suit || r.field == KitField::Trim { continue; }
        let v = row_get(g, &r, &mut from);
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
    home_shells(g, &mut m)
}

/// rowsMoved's walk, over the cards themselves: a read may home a card's picks and that lands on them.
fn moved_rows(g: &Game, rows: &[KitRow], a: &mut MemberKit, b: &mut MemberKit) -> Vec<KitField> {
    rows.iter()
        .filter(|r| (row_has(g, r, a) || row_has(g, r, b)) && { let (x, y) = (row_get(g, r, a), row_get(g, r, b)); !same_value(&x, &y) })
        .map(|r| r.field)
        .collect()
}

/// What a paste would do, nothing written, as his card is worn.
pub(super) fn kit_paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::KitPastePeek {
    let p = on_worn(g, i, &mut |g: &mut Game| paste_peek(g, i, from), None);
    KitPastePeek { changed: p.changed, why: p.why }
}

/// The card a paste would make, the rows of his it would move, why it would be refused ('' = taken).
fn paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::PastePeek {
    let mut card = kit_paste_card(g, i, from);
    let rows = kit_rows(g, i);
    let mut worn = g.m.kit.kit.members[i as usize].clone();
    let changed = moved_rows(g, &rows, &mut worn, &mut card);
    g.m.kit.kit.members[i as usize] = worn;
    let why = card_why(g, i, &card).unwrap_or_default();
    PastePeek { card, changed, why }
}

/// Paste a copied card onto member i's; kept, in one write of his card. `ok` false: refused (`why`), nothing written.
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

/// Member i already wears a copied card: a paste of it would move no row of his.
pub(super) fn kit_wears(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> bool {
    kit_paste_peek(g, i, from).changed.is_empty()
}

/// `out[field] = v` of a stored value, for a row with no reader of its own (one its value was found on, so the value's kind fits the field).
fn put_field(card: &mut MemberKit, field: KitField, v: &KitValue) {
    match (field, v) {
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
        (KitField::Shells, KitValue::Num(n)) => card.shells = *n,
        (KitField::Cams, KitValue::Num(n)) => card.cams = *n,
        (KitField::Rings, KitValue::Num(n)) => card.rings = *n,
        _ => {}
    }
}

/// A stored (or hand-typed) card made safe: every field one of its row's values, anything else the issued one.
pub fn kit_sanitize(g: &mut crate::game::Game, i: f64, raw: Option<&JsMap<String, crate::kit::KitValue>>) -> crate::kit::MemberKit {
    let mut card = kit_default(g, i);
    let Some(raw) = raw else { return card; };
    let at = |k: &str| raw.get(String::from(k));
    let word = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Text(s)) => Some(s.as_str()), _ => None };
    let flag = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Bool(b)) => Some(*b), _ => None };
    let num = |k: &str| match raw.get(String::from(k)) { Some(KitValue::Num(n)) => *n, _ => 0.0 };

    let you = i == 0.0;
    let rows = kit_rows(g, i);
    for r in &rows {
        if r.get.is_some() { continue; }
        let key = r.field.as_str();
        let v = at(key);
        if let Some(v) = v.filter(|v| offers(&r.values, v)) {
            put_field(&mut card, r.field, v);
        } else if you && matches!(r.field, KitField::Smoke | KitField::Bangs | KitField::Frags | KitField::Shells) && int(v, 0.0) {
            put_field(&mut card, r.field, &KitValue::Num(m::min(num(key), Thing::of(key).expect("a counted thing").most())));
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
    let big_values = rows.iter().find(|r| r.field == KitField::Big).map(|r| r.values.clone()).unwrap_or_default();
    if card.big != BigKit::None && !offers(&big_values, &text_value(card.big.as_str())) { card.big = BigKit::None; }
    if int(at("walls"), 1.0) && offers(&big_values, &text_value("wall")) { card.walls = m::min(num("walls"), WALLS_MAX); }
    // THE TWO GUN SLOTS: rows of choices, not read by the loop above.
    match word("weapon") {
        Some("rifle") => card.weapon = KitWeapon::Rifle,
        Some("pistol") => card.weapon = KitWeapon::Pistol,
        _ => {}
    }
    if let Some(p) = word("primary") { card.primary = String::from(p); }
    if let Some(p) = word("secondary") { card.secondary = String::from(p); }
    guns_home(g, &mut card);
    // THE TWO SUPPRESSORS: rows that read the gun, not the field, so not read by the loop above.
    match flag("sideCan") {
        Some(side_can) => {
            card.side_can = side_can;
            if let Some(b) = flag("suppressed") { card.suppressed = b; }
        }
        None => {
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
    home_cans(g, &mut card);
    if you && flag("wand") == Some(true) && !wand_row(g, 0.0) { card.wand = true; }
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A card's rows top to bottom, each with how many values it steps through: yours, and a teammate's, who has no sponge rounds or cameras.
    #[test]
    fn a_cards_rows_top_to_bottom() {
        let g = Game::blank();
        let rows = |i: f64| kit_rows(&g, i).iter().map(|r| alloc::format!("{} {}", r.field.as_str(), r.values.len())).collect::<Vec<_>>().join(", ");
        assert_eq!(rows(0.0), "primary 6, suppressed 2, light 2, rings 6, shells 5, cams 4, secondary 3, sideCan 2, smoke 5, bangs 5, frags 4, charges 3, wand 2, nvg 2, suit 8, trim 7, big 4");
        for i in 1..5 { assert_eq!(rows(i as f64), "primary 6, suppressed 2, light 2, shells 2, secondary 3, sideCan 2, smoke 3, bangs 4, frags 3, charges 3, wand 2, nvg 2, suit 8, trim 7, big 4", "{i}"); }
    }
}
