//! A man's card as the page shows it (src/ui/kitCard.ts draw, drawDense; kitNoise.ts; gunStats.ts; the rows' words of src/game/kit.ts buildRowsTree), and the
//! few presses and store lines only the desk asks for. Only the card as a tree stands here: its other forms are gone from the rules.
//!
//! A read takes `&mut Game` only because the helpers do: it leaves the cards as they were.
use super::rows::{same_value, value_text};
use super::*;
use crate::game::Game;
use crate::gunslots::GunSlot;
use crate::js;
use crate::m;
use crate::view::{CardBar, CardKind, CardLoad, CardPick, CardPouches, CardRow, CardView};
use alloc::{format, string::String, vec::Vec};

/// What the mission the cards are for makes of a card.
#[derive(Clone, Debug, Default)]
pub struct CardFor {
    /// the mission has no fog of war, so a carried wand or cameras do nothing on it
    pub no_fog: bool,
}

/// Where the launcher's load is worked from (slotsetup.ts loadKeys; the words of the build's own keys, as player.rs words them).
const LOAD_KEYS: &str = "press 2, then 2";
/// Where your light is worked from.
const LIGHT_KEYS: &str = "its gun's key again, then 3";

/// A key's code as a line says it.
fn key_word(code: &str) -> &str { code.strip_prefix("Key").unwrap_or(code) }

/// The gun slot a row is.
fn gun_slot(field: KitField) -> Option<GunSlot> {
    match field {
        KitField::Primary => Some(GunSlot::Primary),
        KitField::Secondary => Some(GunSlot::Secondary),
        _ => None,
    }
}

/// The gun row a row hangs under as the card stands, if any.
fn row_under(r: &KitRow, m: &MemberKit) -> Option<GunSlot> {
    match r.field {
        KitField::Suppressed | KitField::Rings | KitField::Shells | KitField::Cams => Some(GunSlot::Primary),
        KitField::SideCan => Some(GunSlot::Secondary),
        KitField::Light => Some(if m.weapon != KitWeapon::Pistol { GunSlot::Primary } else { GunSlot::Secondary }),
        _ => None,
    }
}

/// What the value now set changes in the run, in a line.
fn row_note(g: &Game, i: f64, r: &KitRow, m: &MemberKit) -> String {
    let you = i == 0.0;
    let pick = |g: &Game, id: &str| crate::gear::gear_text(g, &crate::gunslots::pick_word(g, id, None));
    let line: &str = match r.field {
        KitField::Primary | KitField::Secondary => {
            let slot = gun_slot(r.field).expect("a gun's row");
            if slot == GunSlot::Primary && m.weapon == KitWeapon::Pistol {
                return crate::gear::gear_text(g, if !you { "no long gun: his sidearm is his gun" } else if g.k.slots.on { "no long gun, no launcher: nothing you fire reads as gunfire · the 40 mm rounds stay home with it" } else { "no long gun, no launcher: nothing you fire reads as gunfire" });
            }
            let id = if slot == GunSlot::Primary { &m.primary } else { &m.secondary };
            return format!("{}{}", pick(g, id), if kit_slot_can(g, m, slot) { " (suppressed)" } else { "" });
        }
        KitField::Suppressed | KitField::SideCan => {
            let slot = if r.field == KitField::Suppressed { GunSlot::Primary } else { GunSlot::Secondary };
            if !kit_slot_can(g, m, slot) { "a bright flash, the hard report" }
            else if slot == GunSlot::Secondary || primary_quiet(g, m) { "the floor does not hear it as gunfire" }
            else { "a small flash · still heard as gunfire" }
        }
        KitField::Light if you => if m.light { LIGHT_KEYS } else { "none" },
        KitField::Light => if m.light { "burns when the light rule or the fight wants it" } else if m.nvg { "never lit: nothing gives him away, and black rooms stay black" } else { "no light and no goggles: he sees by the room alone" },
        KitField::Nvg if you => if m.nvg { "on N" } else { "bare eyes: N does nothing" },
        KitField::Nvg => if m.nvg { "goes dark with you, sees through the tube" } else if m.light { "bare eyes: his light does the seeing" } else { "bare eyes, no light" },
        KitField::Rings => if js::truthy(m.rings) { "2 with the rifle up, then the right button: out cold to the head, doubled over anywhere else" } else { "none for the launcher" },
        KitField::Shells if you => {
            if !tubed(g, m) { return crate::gear::gear_text(g, if g.k.slots.on { "no FN2000, no launcher: none carried, and no pouch spent on them" } else { "no FN2000, no launcher: these stay home" }); }
            if !js::truthy(m.shells) { return crate::gear::gear_text(g, "none for the launcher: rings only"); }
            return format!("{LOAD_KEYS} with the rifle up, then the right button: a frag's blast where it lands, past {} m", js::num_str(g.k.he40.arm_m));
        }
        KitField::Shells => if js::truthy(m.shells) { "fired on your order" } else { "none for his launcher" },
        KitField::Cams => {
            if !tubed(g, m) { return crate::gear::gear_text(g, "no FN2000, no launcher: these stay home"); }
            if !js::truthy(m.cams) { return String::from("none for the launcher"); }
            return format!("{LOAD_KEYS} with the rifle up, then the right button: it sticks where it lands and watches");
        }
        KitField::Smoke if you => if js::truthy(m.smoke) { "on 3: a cloud to cross a lane in, or to leave a room under" } else { "none: nothing to hide in" },
        KitField::Smoke => {
            if !js::truthy(m.smoke) { return String::from("none: the squad has no smoke to throw on order"); }
            return format!("added to the squad's smoke: X on a spot of floor, then {} — the nearest man with a throw tosses one there", key_word(&crate::squadsmoke::squadsmoke().order_key));
        }
        KitField::Bangs if you => if js::truthy(m.bangs) { "on 4: blinds a room before you go in" } else { "none of your own: the squad's are on the door tools" },
        KitField::Bangs | KitField::Charges => "added to the squad count the door tools spend",
        KitField::Frags if you => if js::truthy(m.frags) { "on 5, then a click: kills what it has a line to — you included" } else { "none on your belt" },
        KitField::Frags => "added to the squad count: frag and clear, on the door tools",
        KitField::Wand if you => if m.wand { "at a shut door, Space: a look under it" } else { "none in your pouches" },
        KitField::Wand => {
            if !m.wand { return String::from("none in his pouches"); }
            return format!("X on a shut door, then {}: he looks under it", key_word(&g.k.wand.order_key));
        }
        KitField::Suit => "looks only: nobody sees a colour",
        KitField::Trim if you => if m.trim == TrimId::Issued { "your own green joints" } else { "the joints: how you pick yourself out from above" },
        KitField::Trim => if m.trim == TrimId::Issued { "the pair colour on a four (red, blue)" } else { "the joints: how you tell him from above" },
        KitField::Big => {
            let notes = big_note().get(String::from(big_of(m).as_str())).cloned().expect("every big thing has its line");
            return notes[if you { 0 } else { 1 }].clone();
        }
        KitField::Weapon | KitField::Walls | KitField::Bare | KitField::Cans => "",
    };
    String::from(line)
}

/// The long gun, suppressed, is under the level the floor takes for gunfire.
fn primary_quiet(g: &Game, m: &MemberKit) -> bool {
    let w = crate::gunslots::pick_row(g, &m.primary, true).unwrap_or(crate::weapons::WeaponId::Fn2000);
    g.m.weapons.weapons.get(w).expect("a pick's row").report.level < 1.0
}

/// The gun of a row wears a suppressor.
pub(super) fn kit_row_can(g: &Game, m: &MemberKit, field: KitField) -> bool {
    match field {
        KitField::Primary => m.weapon != KitWeapon::Pistol && kit_slot_can(g, m, GunSlot::Primary),
        KitField::Secondary => kit_slot_can(g, m, GunSlot::Secondary),
        _ => false,
    }
}

/// The range at which a gun's cone is half a man wide, in metres: r · spread(r) / 2 = half, the spread opening past `range_from`.
fn reach_m(def: &crate::weapons::WeaponDef) -> f64 {
    const HALF: f64 = 0.25;
    let a = &def.ai_spread;
    let flat = 2.0 * HALF / a.base;
    let mut r = flat;
    if flat > a.range_from && a.range_k > 0.0 {
        let b = a.base - a.range_from * a.range_k;
        r = (-b + m::sqrt(b * b + 8.0 * HALF * a.range_k)) / (2.0 * a.range_k);
    }
    match &def.pellets {
        Some(p) => m::min(r, HALF / p.cone),
        None => r,
    }
}

/// Rounds out in ten seconds from a full gun, magazine changes and all.
fn rounds_out(def: &crate::weapons::WeaponDef) -> f64 {
    const SECS: f64 = 10.0;
    let gap = def.auto.as_ref().map_or(def.cd, |a| a.interval);
    let empty_in = def.mag_capacity * gap;
    let (mut t, mut n) = (0.0, 0.0);
    while t < SECS {
        let left = SECS - t;
        if left <= empty_in {
            n += m::floor(left / gap);
            break;
        }
        n += def.mag_capacity;
        t += empty_in + def.reload.dur;
    }
    n
}

/// What a gun is good at, each 0 to 1 off its own row: reach, volume, handling, quiet.
fn gun_stats(def: &crate::weapons::WeaponDef) -> [f64; 4] {
    let unit = |x: f64| m::min(1.0, m::max(0.0, x));
    [
        unit(reach_m(def) / 20.0),
        unit(rounds_out(def) / 100.0),
        unit((crate::weapons::handling_of(def) - 0.4) / (1.0 - 0.4)),
        unit((m::max(0.0, 1.0 - def.report.level) + if def.suppressed { 0.2 } else { 0.0 }) / 0.8),
    ]
}

/// How loud a row's thing is when used: 0 none, 1 nearby, 2 the floor hears it, 3 a blast. A count of none still has its step.
fn kit_noise(g: &Game, field: KitField, m: &MemberKit) -> f64 {
    let gun = |w: crate::weapons::WeaponId| if g.m.weapons.weapons.get(w).is_some_and(|d| d.report.loud) { 2.0 } else { 1.0 };
    match field {
        KitField::Primary => if m.weapon == KitWeapon::Pistol { 0.0 } else { gun(kit_gun(g, m)) },
        KitField::Secondary => kit_second(g, m).map_or(0.0, gun),
        KitField::Big => match big_of(m) {
            BigPick::Shotgun => gun(crate::weapons::WeaponId::M870),
            BigPick::Hammer | BigPick::Wall => 2.0,
            BigPick::None => 0.0,
        },
        KitField::Smoke | KitField::Cams | KitField::Rings => 1.0,
        KitField::Bangs | KitField::Charges | KitField::Walls => 2.0,
        KitField::Frags | KitField::Shells => 3.0,
        _ => 0.0,
    }
}

/// The value means he carries none of it.
fn carries_none(g: &Game, field: KitField, v: &KitValue) -> bool {
    if gun_slot(field).is_some() { return crate::gunslots::pick_icon(g, &value_text(v)).is_none(); }
    match v {
        KitValue::Num(n) => *n == 0.0,
        KitValue::Bool(b) => !*b,
        KitValue::Text(s) => s == "none",
    }
}

/// A priced row's cost in the card's words: what a counted row fills and its rate, or the rate alone.
fn cost_words(c: &KitRowCost) -> String {
    if !(c.pouches > 0.0 && c.rate.ends_with("each")) { return c.rate.clone(); }
    if c.pouches == 1.0 { return format!("1 pouch · {}", c.rate); }
    format!("{} pouches · {}", js::num_str(c.pouches), c.rate)
}

/// The rows the card has as it stands, in the order they are drawn: a gun, then what hangs under it.
fn seq(g: &Game, rows: &[KitRow], m: &MemberKit) -> Vec<KitRow> {
    let mut out = Vec::new();
    for r in rows {
        if row_under(r, m).is_some() || !row_has(g, r, m) { continue; }
        out.push(r.clone());
        let Some(slot) = gun_slot(r.field) else { continue };
        out.extend(rows.iter().filter(|c| row_under(c, m) == Some(slot) && row_has(g, c, m)).cloned());
    }
    out
}

/// The kind of gun a pick is drawn as (empty: no gun).
pub(super) fn pick_kind(g: &Game, id: &str) -> String { crate::gunslots::pick_icon(g, id).map_or(String::new(), |k| String::from(k.as_str())) }

/// A gun's picks: every value of its row, tried on a copy of the card.
fn card_picks(g: &mut Game, i: f64, r: &KitRow, m: &MemberKit, now: &KitValue, slot: GunSlot) -> Vec<CardPick> {
    let mut out = Vec::new();
    for v in &r.values {
        let mut c = m.clone();
        row_set(g, r, &mut c, v);
        let on = same_value(v, now);
        let why = if on { String::new() } else { kit_paste_peek(g, i, &c).why };
        let bars = kit_slot_def(g, i, &c, slot).map_or(Vec::new(), |d| gun_stats(&d).to_vec());
        out.push(CardPick { id: value_text(v), word: row_word(g, i, r, v), note: row_note(g, i, r, &c), why, now: on, can: kit_row_can(g, &c, r.field), icon: pick_kind(g, &value_text(v)), bars });
    }
    out
}

/// One row of the card as it stands.
fn card_row(g: &mut Game, i: f64, r: &KitRow, m: &mut MemberKit, issued: &mut MemberKit, load: &crate::kitload::KitLoad, on: &CardFor) -> CardRow {
    let v = row_get(g, r, m);
    let none = carries_none(g, r.field, &v);
    let idle = !none && on.no_fog && matches!(r.field, KitField::Wand | KitField::Cams);
    let (kind, n) = match &v {
        KitValue::Num(n) => (CardKind::Count, *n),
        KitValue::Bool(b) => (CardKind::Flag, if *b { 1.0 } else { 0.0 }),
        KitValue::Text(_) => (CardKind::Choice, if none { 0.0 } else { 1.0 }),
    };
    let note = row_note(g, i, r, m);
    let cost = kit_row_cost(g, i, r, Some(m)).map_or(String::new(), |c| cost_words(&c));
    let share = crate::kitload::load_share(g, i, m, r.field.as_str());
    let mut would = None;
    if share.is_none() {
        // what one more would take: the step the right arrow would make, weighed on a copy of the card; nothing where that step has no room
        let p = kit_peek(g, i, r.field, 1.0);
        if p.why.is_empty() {
            let mut c = m.clone();
            row_set(g, r, &mut c, &p.value);
            let up = crate::kitload::load_of(g, i, &c);
            if up.mob < load.mob || up.con < load.con { would = Some(CardLoad { mob: load.mob - up.mob, con: load.con - up.con }); }
        }
    }
    let was = row_get(g, r, issued);
    let (mut bars, mut picks) = (Vec::new(), Vec::new());
    if let Some(slot) = gun_slot(r.field) {
        if let Some(d) = kit_slot_def(g, i, m, slot) {
            let now = gun_stats(&d);
            let from = kit_slot_def(g, i, issued, slot).map_or(now, |d0| gun_stats(&d0));
            bars = now.iter().zip(from.iter()).map(|(a, z)| CardBar { now: *a, issued: *z }).collect();
        }
        picks = card_picks(g, i, r, m, &v, slot);
    }
    CardRow {
        field: String::from(r.field.as_str()),
        label: r.label.clone(),
        kind,
        value: value_text(&v),
        n,
        word: row_word(g, i, r, &v),
        note,
        cost,
        under: row_under(r, m).map_or(String::new(), |s| String::from(s.as_str())),
        look: matches!(r.field, KitField::Suit | KitField::Trim),
        none,
        fixed: !row_fixed(g, r, m).is_empty(),
        changed: !same_value(&v, &was),
        idle,
        less: kit_peek(g, i, r.field, -1.0).why,
        more: kit_peek(g, i, r.field, 1.0).why,
        can: kit_row_can(g, m, r.field),
        icon: if gun_slot(r.field).is_some() { pick_kind(g, &value_text(&v)) } else { String::new() },
        noise: kit_noise(g, r.field, m),
        takes: share.map(|s| CardLoad { mob: s.mob, con: s.con }),
        would,
        bars,
        picks,
    }
}

/// A man's card as it is shown; `clip` is the copied card. Changes nothing.
pub fn card_view(g: &mut Game, i: f64, clip: Option<&MemberKit>, on: &CardFor) -> CardView {
    let mut m = kit_shown(g, i);
    let mut issued = kit_issued_shown(g, i, None);
    let load = crate::kitload::load_of(g, i, &m);
    let load_issued = crate::kitload::load_ref(g, i, None);
    let all = kit_rows(g, i);
    let mut rows = Vec::new();
    for r in seq(g, &all, &m) { rows.push(card_row(g, i, &r, &mut m, &mut issued, &load, on)); }
    let cost = kit_cost(g, i, Some(&m));
    let squad = if i > 0.0 && i <= kit_team(g, None) { squad_cost(g, None, None) } else { None };
    let (mut worn, mut paste_why) = (false, String::new());
    if let Some(c) = clip {
        let p = kit_paste_peek(g, i, c);
        worn = p.changed.is_empty();
        if !worn { paste_why = p.why; }
    }
    let mut words = kit_words(g, i, Some(&m));
    let note = g.m.kit.kit_note[i as usize].clone();
    if !note.is_empty() { words = format!("{words} {}", crate::gear::gear_text(g, &note)); }
    CardView {
        who: i,
        changed: kit_changed(g, i),
        pouches: CardPouches { used: cost.used, total: cost.total },
        big: cost.big.map(|b| CardPouches { used: b.used, total: b.total }),
        squad: squad.map(|s| CardPouches { used: s.used, total: s.total }),
        load: CardLoad { mob: load.mob, con: load.con },
        load_issued: CardLoad { mob: load_issued.mob, con: load_issued.con },
        rows,
        issued: kit_preset_of(g, i) == Some(KitPreset::Issued),
        reset_why: kit_preset_why(g, i, KitPreset::Issued),
        copied: clip.is_some(),
        clip: worn,
        paste_why,
        words,
        suit: suits().get(String::from(m.suit.as_str())).map(|s| s.tint),
        trim: trims().get(String::from(m.trim.as_str())).map(|s| s.tint),
    }
}

/// A row's key by the word the page and the kept text have for it.
pub fn field_of(word: &str) -> Option<KitField> {
    use KitField::*;
    [Smoke, Light, Wand, Bangs, Frags, Shells, Charges, Cams, Walls, Suit, Bare, Big, Primary, Secondary, SideCan, Rings, Weapon, Suppressed, Nvg, Trim, Cans].into_iter().find(|f| f.as_str() == word)
}

/// One of a row's values put on the card at once, fitted and refused as a paste is; one that is on already is taken and changes nothing.
pub fn kit_choose(g: &mut Game, i: f64, field: &str, value: &str) -> (bool, String) {
    let Some(r) = kit_rows(g, i).into_iter().find(|r| r.field.as_str() == field) else { return (false, format!("no row {field}")) };
    let Some(v) = r.values.iter().find(|v| value_text(v) == value).cloned() else { return (false, format!("{field} has no {value}")) };
    let mut m = kit_shown(g, i);
    if same_value(&row_get(g, &r, &mut m), &v) { return (true, String::new()); }
    let mut c = m;
    row_set(g, &r, &mut c, &v);
    let p = kit_paste(g, i, &c);
    if p.ok { return (true, String::new()); }
    (false, format!("{}: {}", row_word(g, i, &r, &v), p.why))
}

/// Your four counts as issued the other way the rule stands are as issued.
pub fn kit_built_home(g: &mut Game) {
    let now = issued_card(g, 0.0);
    let other = if g.k.slots.on {
        let b = built();
        [b.smoke, b.bangs, b.frags, b.shells]
    } else {
        let c = issued().swap_remove(0);
        [c.smoke, c.bangs, c.frags, c.shells]
    };
    let m = &mut g.m.kit.kit.members[0];
    if [m.smoke, m.bangs, m.frags, m.shells] != other { return; }
    (m.smoke, m.bangs, m.frags, m.shells) = (now.smoke, now.bangs, now.frags, now.shells);
}

/// What a kept card had to leave behind to fit, in a line for his card; `gunless`: yours, with no long gun. Worded as issued where it is shown.
pub fn kit_left_words(g: &Game, left: &[String], gunless: bool) -> String {
    let shells = slot_word().get(String::from("shells")).cloned().expect("the launcher's rounds have a word");
    if gunless && left.len() == 1 && left[0].contains(&shells) { return format!("No FN2000 in your saved kit: {} left behind.", left[0]); }
    let small = ["canister", "frag", "grenade", "door charge", "camera", "wand", "sponge"];
    let in_the_pouches = left.iter().any(|w| small.iter().any(|s| w.contains(s)));
    let what = if g.k.bigslot.on && !in_the_pouches { "big slot" } else { "pouches" };
    format!("Your saved kit was over the {what}: {} left behind.", left.join(", "))
}
