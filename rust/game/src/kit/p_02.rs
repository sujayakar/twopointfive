//! src/game/kit.ts lines 377 to 770
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- private helpers and the seams

/// `Object.keys(SLOT_AMOUNT)`, in its written order (kit.ts:252)
const SLOT_KEYS: [&str; 11] = ["smoke", "bangs", "frags", "shells", "charges", "walls", "shotgun", "hammer", "cams", "rings", "wand"];
/// what is dealt, in the order it is handed out (kit.ts:450 DEALT)
const DEALT: [&str; 4] = ["charges", "bangs", "frags", "smoke"];
/// who is handed to, in turn: Red One, Blue One, Red Two, Blue Two (kit.ts:450 DEAL_TURN)
const DEAL_TURN: [usize; 4] = [1, 3, 2, 4];
/// who is lent which big tool, of the men who come (kit.ts:383 DEAL_TOOL)
const DEAL_TOOL: [(usize, BigKit, &str); 2] = [(2, BigKit::Shotgun, "shotgun"), (4, BigKit::Hammer, "hammer")];
const MEMBERS: usize = KIT_MEMBERS as usize;

/// `ISSUED[i]` (kit.ts:147)
fn issued_at(i: usize) -> MemberKit { issued().swap_remove(i) }
/// `SLOT_WORD[k]` (kit.ts:267)
fn word_of(k: &str) -> String { slot_word().get(String::from(k)).cloned().expect("a thing the card can carry") }
/// `SET_LOADOUT[p]` (kit.ts:277); only ever asked of the two sets
fn set_loadout_of(p: KitPreset) -> String { set_loadout().get(String::from(p.as_str())).cloned().expect("the quiet or the loud set") }
/// smoke, stun of a split: `CAN_MIXES[mix]` (kit.ts:197)
fn can_mix(mix: CanMix) -> (f64, f64) { let x = can_mixes().get(String::from(mix.as_str())).cloned().expect("a split of six"); (x.smoke, x.stun) }
/// `lend`'s stub takes the cards as `&mut Vec` and nothing in it writes them (kit.ts:483 `out` is a slice before its first write): a copy lent, for the callers that hand the same cards as `of`
fn lend_of(g: &mut Game, cards: &[MemberKit], t: &SlotTable, n: f64, of: &[MemberKit], mine: bool) -> Vec<MemberKit> {
    let mut cards = cards.to_vec();
    lend(g, &mut cards, t, n, of, mine)
}
// Seams: KitRow's functions are gone from the skeleton and these three stubs have no `g`.
fn seam_row_has(r: &KitRow, m: &MemberKit) -> bool { todo!("needs: g in row_has (kit.ts:759, 904, 910: whether the card is a tree, and `tubed`)") }
fn seam_row_under(r: &KitRow, m: &MemberKit) -> Option<crate::gunslots::GunSlot> { todo!("needs: g in row_under (kit.ts:757, 904, 908, 910: whether the card is a tree)") }
fn seam_row_get(r: &KitRow, m: &MemberKit) -> KitValue { todo!("needs: g and a card it may write in row_get (kit.ts:889, 892, 907: gunsHome, kitSlotCan, whether the card is a tree)") }
/// `SLOT_AMOUNT[k](c)` of a row that reads the switches
fn seam_squad_amount(k: &str, c: &MemberKit) -> f64 { todo!("needs: g in squad_has (kit.ts:546; SLOT_AMOUNT's shotgun, hammer, cams, rings, wand read the switches); inside this file the body is squad_has_g") }
/// `cards` left as `fit_all` left them
fn seam_fit_write(cards: &[MemberKit], fitted: &[MemberKit]) { todo!("needs: cards: &mut [MemberKit] in kit_fit_all (kit.ts:661 writes the cards in place, callers at 315, 641, 684, 1136 read them after)") }

/// `SLOTS.on`: the getter's `ruleOn` (kit.ts:293). One variable; the page's value is on the row.
fn rule_is_on(g: &Game) -> bool { g.k.slots.on }
/// `BIGSLOT.on`: the getter's `bigOn` (kit.ts:314)
fn big_is_on(g: &Game) -> bool { g.k.bigslot.on }
/// `bigOn = v` itself, the setter's work left out (kit.ts:614)
fn big_write(g: &mut Game, v: bool) { g.k.bigslot.on = v; g.m.kit.big_on = v; }
fn team_of(g: &Game) -> f64 { g.m.kit.team }
fn item<'a>(t: &'a SlotTable, k: &str) -> Option<&'a SlotItem> { t.items.get(String::from(k)) }
/// `t.items[k]?.max[i] ?? d`
fn max_or(t: &SlotTable, k: &str, i: usize, d: f64) -> f64 { item(t, k).and_then(|it| it.max.get(i).copied()).unwrap_or(d) }
/// `t.big[i] ?? 1`
fn big_slots(t: &SlotTable, i: usize) -> f64 { t.big.get(i).copied().unwrap_or(1.0) }
/// `t.pouches[i] ?? 0`
fn pouches_at(t: &SlotTable, i: usize) -> f64 { t.pouches.get(i).copied().unwrap_or(0.0) }
/// a whole number as the language writes it
fn ns(x: f64) -> String { if x == 0.0 { String::from("0") } else { format!("{}", x) } }
fn plural(n: f64) -> &'static str { if n == 1.0 { "" } else { "s" } }
/// `SLOT_AMOUNT[k](m)` (kit.ts:252)
fn amount(g: &mut Game, k: &str, m: &MemberKit) -> f64 {
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
/// `SLOT_PUT[k](m, n, d)` (kit.ts:261)
fn put(g: &mut Game, k: &str, m: &mut MemberKit, n: f64, d: &MemberKit) {
    match k {
        "smoke" => slot_put::smoke(m, n),
        "bangs" => slot_put::bangs(m, n),
        "frags" => slot_put::frags(m, n),
        "shells" => slot_put::shells(m, n),
        "charges" => slot_put::charges(m, n),
        "walls" => slot_put::walls(m, n),
        "wand" => slot_put::wand(m, n),
        "cams" => slot_put::cams(m, n),
        "rings" => slot_put::rings(m, n),
        "shotgun" => slot_put::shotgun(g, m, n, d),
        "hammer" => slot_put::hammer(g, m, n),
        _ => unreachable!(),
    }
}
/// `c[k]` of a dealt thing
fn dealt_get(c: &MemberKit, k: &str) -> f64 {
    match k { "charges" => c.charges, "bangs" => c.bangs, "frags" => c.frags, "smoke" => c.smoke, _ => unreachable!() }
}
fn dealt_mut<'a>(c: &'a mut MemberKit, k: &str) -> &'a mut f64 {
    match k { "charges" => &mut c.charges, "bangs" => &mut c.bangs, "frags" => &mut c.frags, "smoke" => &mut c.smoke, _ => unreachable!() }
}
/// `{ ...c, ...Object.fromEntries(DEALT.map(k => [k, from[k]])) }`
fn with_dealt_of(c: &MemberKit, from: &MemberKit) -> MemberKit {
    let mut o = c.clone();
    for k in DEALT { *dealt_mut(&mut o, k) = dealt_get(from, k); }
    o
}
/// `JSON.stringify(a') === JSON.stringify({ ...a', ...b })`: every field alike; `bare` only where `b` has one, since a spread keeps `a`'s where `b` has none
fn alike_over(a: &MemberKit, b: &MemberKit, looks: bool) -> bool {
    a.weapon == b.weapon && a.suppressed == b.suppressed && a.light == b.light && a.nvg == b.nvg && a.bangs == b.bangs && a.charges == b.charges
        && a.smoke == b.smoke && a.frags == b.frags && a.walls == b.walls && a.shells == b.shells && a.wand == b.wand && a.cams == b.cams
        && (b.bare.is_none() || a.bare == b.bare) && a.big == b.big && a.primary == b.primary && a.secondary == b.secondary && a.side_can == b.side_can
        && a.rings == b.rings && (!looks || (a.suit == b.suit && a.trim == b.trim))
}
/// `JSON.stringify({ ...c, suit: '', trim: '' })`, as a key
fn card_key(c: &MemberKit) -> String {
    format!("{:?}", MemberKit { suit: SuitId::Issued, trim: TrimId::Issued, ..c.clone() })
}
fn issuing_truthy(g: &Game) -> bool { match g.m.kit.issuing { KitModIssuing::Bool(b) => b, KitModIssuing::Own => true } }
/// the table's own cards: `KIT.members`
fn members(g: &Game) -> Vec<MemberKit> { g.m.kit.kit.members.clone() }
fn table(g: &Game) -> SlotTable { g.k.slots.table.clone() }

// ---------------------------------------------------------------- lines 384 to 460

/// A check's word in the environment (kit.ts envIs): the game has no environment and no such word is ever set
pub fn env_is(name: &str) -> bool { false }

/// (kit.ts moreOn; KITDEAL.more settled on, the check's break word never set in the game)
pub fn more_on(g: &mut crate::game::Game) -> bool { true }

/// (kit.ts stealthOn; KITDEAL.stealth settled on)
pub fn stealth_on(g: &mut crate::game::Game) -> bool { tree_up(g) }

/// (kit.ts kitStealthOn)
pub fn kit_stealth_on(g: &mut crate::game::Game) -> bool { stealth_on(g) }

/// A run tape's stealth card put back for its replay (kit.ts kitStealthSet)
pub fn kit_stealth_set(g: &mut crate::game::Game, raw: Option<&JsMap<String, crate::kit::KitValue>>) {
    g.m.kit.stealth_you = if raw.is_some() { Some(kit_sanitize(g, 0.0, raw)) } else { None };
}

/// Your stealth card as kept, for a run tape (kit.ts kitStealthCard)
pub fn kit_stealth_card(g: &mut crate::game::Game) -> Option<crate::kit::MemberKit> {
    if !stealth_on(g) { return None; }
    let Some(h) = g.m.kit.hand else { return g.m.kit.stealth_you.clone(); };
    let m0 = g.m.kit.kit.members[0].clone();
    let iss = stealth_issue(g, h);
    if same_kit(&m0, &iss) { None } else { Some(m0) }
}

/// The loud card as issued, whatever mission the table is for (kit.ts loudIssue)
pub fn loud_issue(g: &mut crate::game::Game) -> crate::kit::MemberKit {
    let mut m = issued_at(0);
    if !rule_is_on(g) {
        let b = built();
        m.smoke = b.smoke; m.bangs = b.bangs; m.frags = b.frags; m.shells = b.shells;
    }
    m
}

/// What a stealth mission issues you (kit.ts stealthIssue)
pub fn stealth_issue(g: &mut crate::game::Game, h: crate::kit::KitHand) -> crate::kit::MemberKit {
    if !rule_is_on(g) {
        let mut m = loud_issue(g);
        m.rings = m::min(RINGS_MAX, m::max(0.0, h.rings));
        m.cams = if cam_row(g) { m::min(CAMS_MAX, m::max(0.0, h.cams)) } else { 0.0 };
        m.wand = h.wand && wand_row(g, None);
        return m;
    }
    let mut m = issued_at(0);
    m.smoke = 0.0; m.bangs = 0.0; m.frags = 0.0; m.shells = 0.0;
    let t = table(g);
    m.rings = m::min_n(&[RINGS_MAX, m::max(0.0, h.rings), max_or(&t, "rings", 0, 0.0)]);
    m.wand = h.wand && wand_row(g, None);
    m.cams = if cam_row(g) { m::min_n(&[CAMS_MAX, m::max(0.0, h.cams), max_or(&t, "cams", 0, 0.0)]) } else { 0.0 };
    while m.cams > 0.0 && !(cost_of(g, &m, &t) <= pouches_at(&t, 0)) { m.cams -= 1.0; }
    if !(cost_of(g, &m, &t) <= pouches_at(&t, 0)) { m.wand = false; }
    for k in ["smoke", "bangs", "smoke", "bangs"] {
        if dealt_get(&m, k) >= max_or(&t, k, 0, 0.0) { continue; }
        *dealt_mut(&mut m, k) += 1.0;
        if !(cost_of(g, &m, &t) <= pouches_at(&t, 0)) { *dealt_mut(&mut m, k) -= 1.0; }
    }
    m
}

/// Two cards alike, the two colours aside (kit.ts sameKit)
pub fn same_kit(a: &crate::kit::MemberKit, b: &crate::kit::MemberKit) -> bool { alike_over(a, b, false) }

/// The stealth card for a mission (kit.ts stealthCard)
pub fn stealth_card(g: &mut crate::game::Game, h: crate::kit::KitHand) -> crate::kit::MemberKit {
    if let Some(you) = g.m.kit.stealth_you.clone() {
        let fits = if rule_is_on(g) { let t = table(g); cost_of(g, &you, &t) <= pouches_at(&t, 0) } else { true };
        if fits { return you; }
    }
    stealth_issue(g, h)
}

/// The stealth card as the store keeps it (kit.ts stealthKept)
pub fn stealth_kept(g: &mut crate::game::Game) -> crate::kit::StealthKept {
    if let Some(h) = g.m.kit.hand {
        let m = g.m.kit.kit.members[0].clone();
        let iss = stealth_issue(g, h);
        g.m.kit.stealth_you = if same_kit(&m, &iss) { None } else { Some(m) };
    }
    crate::kit::StealthKept { stealth: g.m.kit.stealth_you.clone() }
}

/// `{ ...c, suit: KIT.members[0].suit, trim: KIT.members[0].trim }` (kitMission's `look`)
fn look(g: &Game, c: &MemberKit) -> MemberKit {
    let m0 = &g.m.kit.kit.members[0];
    MemberKit { suit: m0.suit, trim: m0.trim, ..c.clone() }
}

/// The mission the cards are for (kit.ts kitMission)
pub fn kit_mission(g: &mut crate::game::Game, h: Option<crate::kit::KitHand>) {
    let to = match h { Some(x) if stealth_on(g) => Some(x), _ => None };
    let Some(to) = to else {
        if g.m.kit.hand.is_some() {
            if let Some(loud) = g.m.kit.loud_you.clone() {
                stealth_kept(g);
                g.m.kit.kit.members[0] = look(g, &loud);
            }
        }
        g.m.kit.hand = None;
        g.m.kit.loud_you = None;
        return;
    };
    let Some(hand) = g.m.kit.hand else {
        let loud = g.m.kit.kit.members[0].clone();
        let card = stealth_card(g, to);
        g.m.kit.kit.members[0] = look(g, &card);
        g.m.kit.loud_you = Some(loud);
        g.m.kit.hand = Some(to);
        return;
    };
    if hand.rings == to.rings && hand.cams == to.cams && hand.wand == to.wand { return; }
    let m0 = g.m.kit.kit.members[0].clone();
    let iss = stealth_issue(g, hand);
    let follows = same_kit(&m0, &iss);
    g.m.kit.hand = Some(to);
    if follows {
        let card = stealth_issue(g, to);
        g.m.kit.kit.members[0] = look(g, &card);
    }
}

/// A stealth mission's kit off the mission's link (kit.ts kitStealthHand)
pub fn kit_stealth_hand(g: &mut crate::game::Game, cams: f64, wand: bool) -> crate::kit::KitHand {
    crate::kit::KitHand { rings: g.k.kittree.lend, cams, wand }
}

/// A staging reads the card its run wears and leaves the table as it found it (kit.ts forRun)
pub fn for_run(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) {
    let was = g.m.kit.hand;
    g.m.kit.staged = true;
    let h = hand_of(g);
    kit_mission(g, h);
    f(g);
    g.m.kit.staged = false;
    kit_mission(g, was);
}

/// The kit a run's stealth mission is played with (kit.ts handOf; WAND.on settled on)
pub fn hand_of(g: &mut crate::game::Game) -> Option<crate::kit::KitHand> {
    let rings = if stealth_run(g) { g.k.kittree.lend } else { 0.0 };
    let cams = if cam_row(g) { crate::stickycam::sticky_visit(g) } else { 0.0 };
    let wand = crate::quietfog::quiet_wand(g);
    if js::truthy(rings) || js::truthy(cams) || wand { Some(crate::kit::KitHand { rings, cams, wand }) } else { None }
}

/// (kit.ts dealing; KITDEAL.on settled on)
pub fn dealing(g: &mut crate::game::Game) -> bool { rule_is_on(g) && g.m.kit.kit.on }

/// The teammates' counted things, as one word (kit.ts dealWord)
pub fn deal_word(cards: &[crate::kit::MemberKit]) -> String {
    let rows: Vec<String> = cards.iter().skip(1).map(|c| DEALT.map(|k| ns(dealt_get(c, k))).join(",")).collect();
    rows.join("|")
}

/// The issue's and every named kit's cards (kit.ts dealTable). The TypeScript kept them once a table (`dealKits`); the skeleton has no such field and they are a pure function of the table, so they are made at each ask
pub fn deal_table(g: &mut crate::game::Game) -> crate::kit::DealTable {
    let t = table(g);
    let issued = issued();
    let mut kits = alloc::vec![issued.clone()];
    for l in t.loadouts.iter() {
        kits.push(issued.iter().enumerate().map(|(i, m)| {
            let mut o = m.clone();
            for k in DEALT { *dealt_mut(&mut o, k) = l.counts[i].get(String::from(k)).copied().unwrap_or(0.0); }
            o
        }).collect());
    }
    let mut words = JsSet::new();
    for c in kits.iter() { words.add(deal_word(c)); }
    crate::kit::DealTable { kits, words }
}

/// (kit.ts dealNamed)
pub fn deal_named(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit]) -> bool {
    deal_table(g).words.has(deal_word(cards))
}

// ---------------------------------------------------------------- lines 463 to 559

/// Five cards as `n` teammates carry them, shared out (kit.ts deal; the check's break word dropped)
pub fn deal(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: f64) -> Vec<crate::kit::MemberKit> {
    if n < 3.0 { return cards.to_vec(); }
    let mut out: Vec<MemberKit> = cards.to_vec();
    let men: Vec<usize> = DEAL_TURN.iter().copied().filter(|&i| i as f64 <= n).collect();
    let mut bag: Vec<&'static str> = Vec::new();
    for k in DEALT {
        for &i in men.iter() {
            let mut c = dealt_get(&out[i], k);
            while c > 0.0 { bag.push(k); c -= 1.0; }
            *dealt_mut(&mut out[i], k) = 0.0;
        }
    }
    let load = |out: &[MemberKit], i: usize| -> f64 { DEALT.iter().fold(0.0, |x, k| x + dealt_get(&out[i], k)) };
    for k in bag {
        let mut can: Vec<usize> = Vec::new();
        for &i in men.iter() {
            if dealt_get(&out[i], k) < max_or(t, k, i, m::INF) && cost_of(g, &out[i], t) + price_of(g, item(t, k)) <= t.pouches[i] { can.push(i); }
        }
        if can.is_empty() { return cards.to_vec(); }
        let mut to = can[0];
        for &b in can.iter().skip(1) {
            let a = to;
            if load(&out, b) < load(&out, a) || (load(&out, b) == load(&out, a) && !js::truthy(dealt_get(&out[b], k)) && dealt_get(&out[a], k) > 0.0) { to = b; }
        }
        *dealt_mut(&mut out[to], k) += 1.0;
    }
    if more_on(g) {
        for &i in men.iter() {
            let Some(it) = item(t, "bangs") else { continue; };
            if js::truthy(load(&out, i)) || out[i].bangs >= it.max[i] || cost_of(g, &out[i], t) + price_of(g, Some(it)) > t.pouches[i] { continue; }
            if let Some(sm) = it.squad_max { if squad_has_g(g, &out, "bangs", n) >= sm { continue; } }
            out[i].bangs += 1.0;
        }
    }
    out
}

/// Five cards with the tools this team is lent (kit.ts lend)
pub fn lend(g: &mut crate::game::Game, cards: &mut Vec<crate::kit::MemberKit>, t: &crate::kit::SlotTable, n: f64, of: &[crate::kit::MemberKit], mine: bool) -> Vec<crate::kit::MemberKit> {
    if !more_on(g) || !big_is_on(g) || n < 3.0 || !lendable(g, of, mine) { return cards.to_vec(); }
    let mut out = cards.to_vec();
    for (i, tool, key) in DEAL_TOOL {
        if i as f64 <= n && big_of(&cards[i]) == BigPick::None && cards[i].bare != Some(true) && max_or(t, key, i, 0.0) > 0.0 && big_slots(t, i) > 0.0 {
            out[i] = MemberKit { big: tool, ..cards[i].clone() };
        }
    }
    out
}

/// May the cards be lent at all (kit.ts lendable)
pub fn lendable(g: &mut crate::game::Game, of: &[crate::kit::MemberKit], mine: bool) -> bool {
    if deal_word(of) == deal_word(&deal_table(g).kits[0]) { return true; }
    mine && !deal_named(g, of)
}

/// The five cards as the team wears them (kit.ts kitWorn). `cards: None` IS `KIT.members`: the TypeScript's `cards === KIT.members`.
pub fn kit_worn(g: &mut crate::game::Game, n: Option<f64>, cards: Option<&[crate::kit::MemberKit]>) -> Vec<crate::kit::MemberKit> {
    let n = n.unwrap_or(team_of(g));
    let is_table = cards.is_none();
    let cards: Vec<MemberKit> = match cards { Some(c) => c.to_vec(), None => members(g) };
    if !dealing(g) || n < 3.0 { return cards; }
    let t = table(g);
    let base = if !(g.m.kit.deal_own && is_table) && deal_named(g, &cards) { deal(g, &cards, &t, n) } else { cards.clone() };
    let mine = is_table && !issuing_truthy(g);
    lend_of(g, &base, &t, n, &cards, mine)
}

/// The cards are the player's own layout: read, or put back as a recording had it (kit.ts kitOwn)
pub fn kit_own(g: &mut crate::game::Game, v: Option<bool>) -> bool {
    if let Some(v) = v { g.m.kit.deal_own = v; }
    g.m.kit.deal_own
}

/// Member i's card as he wears it (kit.ts kitShown)
pub fn kit_shown(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit {
    kit_worn(g, None, None).swap_remove(i as usize)
}

/// What he is issued, as he would wear that (kit.ts kitIssuedShown)
pub fn kit_issued_shown(g: &mut crate::game::Game, i: f64, n: Option<f64>) -> crate::kit::MemberKit {
    let n = n.unwrap_or(team_of(g));
    if i > 0.0 && dealing(g) && n >= 3.0 {
        let cards: Vec<MemberKit> = (0..MEMBERS).map(|j| issued_card(g, j as f64)).collect();
        let t = table(g);
        let dealt = deal(g, &cards, &t, n);
        return lend_of(g, &dealt, &t, n, &issued(), false).swap_remove(i as usize);
    }
    issued_card(g, i)
}

/// A card is read and changed as it is worn (kit.ts onWorn; `kitSave` and with it `store` dropped, no exception to catch)
pub fn on_worn<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, i: f64, f: &mut dyn FnMut(&mut crate::game::Game) -> T, taken: Option<&mut dyn FnMut(&mut crate::game::Game, T) -> bool>) -> T {
    if !(i > 0.0 && dealing(g) && team_of(g) >= 3.0) { return f(g); }
    let w = kit_worn(g, None, None);   // (`now === KIT.members ? was : now`: by value the two hold the same cards)
    let was = members(g);
    for j in 1..MEMBERS { g.m.kit.kit.members[j] = w[j].clone(); }
    let r = f(g);
    let took = match taken { Some(t) => t(g, r.clone()), None => false };
    if !took {
        for j in 1..MEMBERS { g.m.kit.kit.members[j] = was[j].clone(); }
        return r;
    }
    let now = members(g);
    let counted = now.iter().enumerate().any(|(j, c)| j > 0 && DEALT.iter().any(|k| dealt_get(c, k) != dealt_get(&w[j], k)));
    let laid: Option<Vec<MemberKit>> = if counted { None } else {
        Some(now.iter().enumerate().map(|(j, c)| if j > 0 { with_dealt_of(c, &was[j]) } else { c.clone() }).collect())
    };
    let t = table(g);
    let own = match &laid { None => true, Some(l) => illegal(g, l, &t, None).is_some() };
    let mut kept: Vec<MemberKit> = match laid { Some(l) if !own => l, _ => now };
    if more_on(g) && big_is_on(g) {
        let leaving = own && !g.m.kit.deal_own && !lendable(g, &was, true);
        for j in 0..kept.len() {
            if !DEAL_TOOL.iter().any(|d| d.0 == j) { continue; }
            let c_bare = kept[j].bare == Some(true);
            let mark = big_of(&kept[j]) == BigPick::None && (c_bare || big_of(&w[j]) != BigPick::None || leaving);
            if c_bare == mark { continue; }
            kept[j].bare = if mark { Some(true) } else { None };
        }
        let bare: Vec<MemberKit> = kept.iter().enumerate().map(|(j, c)| {
            if DEAL_TOOL.iter().any(|d| d.0 == j && c.big == d.1 && !js::truthy(c.walls) && big_of(&was[j]) == BigPick::None) { MemberKit { big: BigKit::None, ..c.clone() } } else { c.clone() }
        }).collect();
        let team = team_of(g);
        let lent = lend_of(g, &bare, &t, team, &bare, true);
        // (`lent[j] !== bare[j]` was by identity: lend replaces a card only where its slot stood empty and fills it, so `big` tells)
        for j in 0..kept.len() { if lent[j].big != bare[j].big { kept[j] = bare[j].clone(); } }
    }
    if own { g.m.kit.deal_own = true; }
    for j in 1..MEMBERS { g.m.kit.kit.members[j] = kept[j].clone(); }
    deal_settle(g);
    r
}

/// True: the cards were put back in a named kit's layout, and kept (kit.ts dealSettle; `kitSave` dropped)
pub fn deal_settle(g: &mut crate::game::Game) -> bool {
    if !dealing(g) || team_of(g) < 3.0 { return false; }
    let now = members(g);
    if deal_named(g, &now) && !g.m.kit.deal_own { return false; }
    let t = table(g);
    let team = team_of(g);
    let worn = lend_of(g, &now, &t, team, &now, true);
    let word_now = deal_word(&now);
    let layouts: Vec<Vec<MemberKit>> = deal_table(g).kits.iter().map(|kit| {
        now.iter().enumerate().map(|(j, c)| if j > 0 { with_dealt_of(c, &kit[j]) } else { c.clone() }).collect()
    }).collect();
    let mut found: Option<Vec<MemberKit>> = None;
    for cards in layouts {
        if illegal(g, &cards, &t, None).is_some() { continue; }
        let dealt = deal(g, &cards, &t, team);
        let a = lend_of(g, &dealt, &t, team, &cards, false);
        let same = deal_word(&a) == word_now && a.iter().enumerate().all(|(j, c)| j == 0 || big_of(c) == big_of(&worn[j]));
        if same { found = Some(cards); break; }
    }
    let Some(kit) = found else { return false; };
    let named: Vec<MemberKit> = if lendable(g, &kit, false) { kit } else {
        kit.into_iter().map(|mut c| { if c.bare == Some(true) { c.bare = None; } c }).collect()
    };
    for j in 1..MEMBERS { g.m.kit.kit.members[j] = named[j].clone(); }
    g.m.kit.deal_own = false;
    true
}

/// The pouches the teammates who come fill between them (kit.ts squadUsed)
pub fn squad_used(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: f64) -> f64 {
    let mut s = 0.0;
    for c in coming(cards, n).iter() { s += cost_of(g, c, t); }
    s
}

/// How many of one thing they carry (kit.ts squadHas). Five of SLOT_AMOUNT's rows read the switches and this stub has no `g`: inside this file the body is squad_has_g
pub fn squad_has(cards: &[crate::kit::MemberKit], k: &str, n: f64) -> f64 {
    let mut s = 0.0;
    for c in coming(cards, n).iter() {
        s += match k {
            "smoke" => slot_amount::smoke(c),
            "bangs" => slot_amount::bangs(c),
            "frags" => slot_amount::frags(c),
            "shells" => slot_amount::shells(c),
            "charges" => slot_amount::charges(c),
            "walls" => slot_amount::walls(c),
            _ => seam_squad_amount(k, c),
        };
    }
    s
}

fn squad_has_g(g: &mut Game, cards: &[MemberKit], k: &str, n: f64) -> f64 {
    let mut s = 0.0;
    for c in coming(cards, n).iter() { s += amount(g, k, c); }
    s
}

/// Why five cards are not a legal squad of `n` teammates, or None (kit.ts illegal)
pub fn illegal(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: Option<f64>) -> Option<String> {
    let n = n.unwrap_or(KIT_MEMBERS - 1.0);
    if cards[0].shells > 0.0 && cards[0].weapon != KitWeapon::Rifle { return Some(crate::gear::gear_text(g, "member 0 carries launcher grenades and no FN2000 to fire them")); }
    for i in 0..MEMBERS {
        for k in SLOT_KEYS {
            let most = max_or(t, k, i, m::INF);
            let has = amount(g, k, &cards[i]);
            if has > most { return Some(format!("member {} carries {} of \"{}\", over the most he may ({})", i, ns(has), item(t, k).unwrap().id, ns(most))); }
        }
        let used = cost_of(g, &cards[i], t);
        if used > t.pouches[i] { return Some(format!("member {} needs {} pouches and has {}", i, ns(used), ns(t.pouches[i]))); }
        let bigs = big_used(g, &cards[i], t);
        let slots = big_slots(t, i);
        if bigs > slots { return Some(format!("member {} carries {} big things and has {} big slot{}", i, ns(bigs), ns(slots), if slots == 1.0 { "" } else { "s" })); }
    }
    let squad = squad_used(g, cards, t, n);
    if let Some(sq) = t.squad { if squad > sq { return Some(format!("the teammates together need {} pouches and the squad has {}", ns(squad), ns(sq))); } }
    for (k, it) in t.items.iter() {
        let has = squad_has_g(g, cards, &k, n);
        if let Some(sm) = it.squad_max { if has > sm { return Some(format!("the teammates carry {} of \"{}\", over the most a squad may ({})", ns(has), it.id, ns(sm))); } }
    }
    None
}

// ---------------------------------------------------------------- lines 561 to 704

/// Member i's card as a set makes it under a table (kit.ts setCard)
pub fn set_card(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset, t: Option<&crate::kit::SlotTable>) -> crate::kit::MemberKit {
    let tt = match t { Some(tt) if i > 0.0 && dealing(g) => tt, _ => return set_card_base(g, i, p, t) };
    let base: Vec<MemberKit> = (0..MEMBERS).map(|j| set_card_base(g, j as f64, p, t)).collect();
    let team = team_of(g);
    let dealt = deal(g, &base, tt, team);
    lend_of(g, &dealt, tt, team, &base, false).swap_remove(i as usize)
}

/// … in the pair's layout (kit.ts setCardBase)
pub fn set_card_base(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset, t: Option<&crate::kit::SlotTable>) -> crate::kit::MemberKit {
    let mut d = if i == 0.0 { loud_issue(g) } else { kit_default(g, i) };
    if p == KitPreset::Quiet {
        if i == 0.0 {
            let (smoke, stun) = can_mix(CanMix::Smoke);
            d.weapon = KitWeapon::Pistol; d.smoke = smoke; d.bangs = stun; d.suit = SuitId::Black; d.trim = TrimId::Black;
        } else {
            d.weapon = KitWeapon::Pistol; d.suppressed = true; d.light = false; d.nvg = true; d.suit = SuitId::Black; d.trim = TrimId::Black;
        }
    }
    if p == KitPreset::Loud {
        if i == 0.0 {
            let (smoke, stun) = can_mix(CanMix::Stun);
            d.weapon = KitWeapon::Rifle; d.smoke = smoke; d.bangs = stun; d.suit = SuitId::Olive; d.trim = TrimId::Amber;
        } else {
            d.weapon = KitWeapon::Rifle; d.suppressed = false; d.light = true; d.nvg = true; d.bangs = m::min(BANGS_MAX, d.bangs + 1.0); d.suit = SuitId::Olive; d.trim = TrimId::Amber;
        }
    }
    let Some(t) = t else { return d; };
    if p == KitPreset::Issued { return d; }
    let iu = i as usize;
    let want = set_loadout_of(p);
    let l = t.loadouts.iter().find(|l| l.id == want && l.missing.is_empty());
    let iss = issued_at(iu);
    for k in ["smoke", "bangs", "frags", "shells", "charges"] {
        let n = match l { Some(l) => l.counts[iu].get(String::from(k)).copied().unwrap_or(0.0), None => amount(g, k, &iss) };
        put(g, k, &mut d, n, &iss);
    }
    home_shells(g, &mut d)
}

/// The table in force is safe to run, or why not (kit.ts slotsCheck). The stub has no `raw`: the table comes over as data (`g.k.slots.table`), so reading slots.json's shape and its own account of the issue (kit.ts:580-612) stay TypeScript.
pub fn slots_check(g: &mut crate::game::Game) -> crate::kit::SlotsCheck {
    let why = |w: String| crate::kit::SlotsCheck::Arm1(crate::kit::SlotsCheckArm1 { why: w });
    let table = table(g);
    // (… both ways the big slot stands: the plain variable written, none of the setter's work)
    let (was_k, was_m) = (g.k.bigslot.on, g.m.kit.big_on);
    let issued = issued();
    let mut bad: Option<String> = None;
    'ways: for way in [false, true] {
        big_write(g, way);
        let word = if way { " with the big slot on" } else { "" };
        if let Some(b) = illegal(g, &issued, &table, None) { bad = Some(format!("{}: the issued kit must stay a legal fill{}", b, word)); break 'ways; }
        for p in [KitPreset::Quiet, KitPreset::Loud] {
            let cards: Vec<MemberKit> = (0..MEMBERS).map(|i| set_card_base(g, i as f64, p, Some(&table))).collect();
            if let Some(w) = illegal(g, &cards, &table, None) { bad = Some(format!("the {} set (loadout \"{}\") is not a legal fill{}: {}", p.as_str(), set_loadout_of(p), word, w)); break 'ways; }
        }
    }
    g.k.bigslot.on = was_k;
    g.m.kit.big_on = was_m;
    if let Some(b) = bad { return why(b); }
    crate::kit::SlotsCheck::Arm0(crate::kit::SlotsCheckArm0 { table })
}

/// The pouches `n` of one item fill (kit.ts slotCost)
pub fn slot_cost(g: &mut crate::game::Game, item: &str, n: f64, t: Option<&crate::kit::SlotTable>) -> f64 {
    let it = match t { Some(t) => self::item(t, item).cloned(), None => self::item(&g.k.slots.table, item).cloned() };
    pouches_of(g, it.as_ref(), n)
}

/// Member i's pouches: used of total, and what fills them (kit.ts kitCost)
pub fn kit_cost(g: &mut crate::game::Game, i: f64, m: Option<&crate::kit::MemberKit>) -> crate::kit::KitCost {
    let m: MemberKit = match m { Some(m) => m.clone(), None => kit_shown(g, i) };
    let iu = i as usize;
    let mut lines = Vec::new();
    for k in SLOT_KEYS {
        let n = amount(g, k, &m);
        let p = slot_cost(g, k, n, None);
        if js::truthy(p) { lines.push(crate::kit::KitCostLinesItem { item: String::from(k), n, pouches: p }); }
    }
    let t = table(g);
    let used = cost_of(g, &m, &t);
    let total = pouches_at(&t, iu);
    let big = if big_is_on(g) { Some(crate::kit::KitCostBig { used: big_used(g, &m, &t), total: big_slots(&t, iu) }) } else { None };
    crate::kit::KitCost { used, total, free: total - used, lines, big }
}

/// The pouches the teammates who come fill between them, of the squad's total (kit.ts squadCost)
pub fn squad_cost(g: &mut crate::game::Game, cards: Option<&[crate::kit::MemberKit]>, n: Option<f64>) -> Option<crate::kit::SquadCost> {
    let cards: Vec<MemberKit> = match cards { Some(c) => c.to_vec(), None => members(g) };
    let n = n.unwrap_or(team_of(g));
    let t = table(g);
    let total = t.squad?;
    if !rule_is_on(g) { return None; }
    let used = squad_used(g, &cards, &t, n);
    Some(crate::kit::SquadCost { used, total, free: total - used })
}

/// Why the five cards as they stand are not a legal squad of the team that comes (kit.ts kitLegal)
pub fn kit_legal(g: &mut crate::game::Game, cards: Option<&[crate::kit::MemberKit]>, n: Option<f64>) -> Option<String> {
    let cards: Vec<MemberKit> = match cards { Some(c) => c.to_vec(), None => members(g) };
    let n = n.unwrap_or(team_of(g));
    let t = table(g);
    illegal(g, &cards, &t, Some(n))
}

/// How many teammates come: asked, or set (kit.ts kitTeam)
pub fn kit_team(g: &mut crate::game::Game, n: Option<f64>) -> f64 {
    let Some(n) = n else { return team_of(g); };
    g.m.kit.team = m::min(KIT_MEMBERS - 1.0, m::max(0.0, m::round(n)));
    if rule_is_on(g) {
        let t = table(g);
        let team = team_of(g);
        let left = fit_table_cards(g, &t, team);
        for (i, l) in left.iter().enumerate() {
            if !l.is_empty() {
                let head = if t.squad.is_some() { "No room in the squad's pouches" } else { "More than a squad may carry" };
                g.m.kit.kit_note[i] = format!("{} with this team: {} left behind.", head, l.join(", "));
            }
        }
    }
    team_of(g)
}

/// The table's tools this build has no row for (kit.ts slotsUnknown)
pub fn slots_unknown(g: &mut crate::game::Game) -> Vec<String> {
    let mut out = g.k.slots.table.unknown.clone();
    if item(&g.k.slots.table, "walls").is_some() && !wall_here() { out.push(String::from("wallCharge")); }
    out
}

/// The most of a counted row member i may carry (kit.ts slotMax)
pub fn slot_max(g: &mut crate::game::Game, item: &str, i: f64, own: f64) -> f64 {
    if rule_is_on(g) { max_or(&g.k.slots.table, item, i as usize, own) } else { own }
}

/// The wall charge has a row on member i's card (kit.ts wallRow; OWNCHARGE.on settled on)
pub fn wall_row(g: &mut crate::game::Game, i: f64) -> bool {
    let iu = i as usize;
    rule_is_on(g) && wall_here() && max_or(&g.k.slots.table, "walls", iu, 0.0) > 0.0 && (!big_is_on(g) || big_slots(&g.k.slots.table, iu) > 0.0)
}

/// Door charges have a row on member i's card (kit.ts chargeRow; OWNCHARGE.on settled on)
pub fn charge_row(g: &mut crate::game::Game, i: f64) -> bool {
    i > 0.0 || slot_max(g, "charges", 0.0, CHARGES_MAX) > 0.0
}

/// The optical wand has a row on a card (kit.ts wandRow; WAND.on and WAND.mate settled on)
pub fn wand_row(g: &mut crate::game::Game, i: Option<f64>) -> bool {
    let i = i.unwrap_or(0.0) as usize;
    !rule_is_on(g) || max_or(&g.k.slots.table, "wand", i, 0.0) > 0.0
}

/// The sticky cameras have a row on your card (kit.ts camRow; STICKY.on settled on)
pub fn cam_row(g: &mut crate::game::Game) -> bool {
    !rule_is_on(g) || max_or(&g.k.slots.table, "cams", 0, 0.0) > 0.0
}

/// Five cards brought inside the rule (kit.ts kitFitAll). The TypeScript writes `cards` in place and this signature cannot: the step that does is the seam
pub fn kit_fit_all(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: Option<&crate::kit::SlotTable>, n: Option<f64>) -> Vec<Vec<String>> {
    let t: SlotTable = match t { Some(t) => t.clone(), None => table(g) };
    let n = n.unwrap_or(KIT_MEMBERS - 1.0);
    let mut fitted = cards.to_vec();
    let left = fit_all(g, &mut fitted, &t, n);
    seam_fit_write(cards, &fitted);
    left
}

/// kitFitAll over `KIT.members`: nothing fit_all calls reads the table's cards, so a copy written back is the same
fn fit_table_cards(g: &mut Game, t: &SlotTable, n: f64) -> Vec<Vec<String>> {
    let mut cards = members(g);
    let left = fit_all(g, &mut cards, t, n);
    g.m.kit.kit.members = cards;
    left
}

fn over(g: &mut Game, k: &str, cards: &[MemberKit], i: usize) -> bool {
    let has = amount(g, k, &cards[i]);
    let iss = issued_at(i);
    has > amount(g, k, &iss)
}
fn less(g: &mut Game, k: &str, cards: &mut [MemberKit], i: usize) {
    let has = amount(g, k, &cards[i]);
    let iss = issued_at(i);
    put(g, k, &mut cards[i], has - 1.0, &iss);
}
/// kitFitAll's `give`: one of what his card added, in TRIM's order
fn give(g: &mut Game, cards: &mut [MemberKit], t: &SlotTable, i: usize) -> bool {
    for k in trim() {
        let k = k.as_str();
        if price_of(g, item(t, k)) > 0.0 && over(g, k, cards, i) { less(g, k, cards, i); return true; }
    }
    false
}
/// kitFitAll's `giveBig`
fn give_big(g: &mut Game, cards: &mut [MemberKit], t: &SlotTable, i: usize) -> bool {
    for k in big_trim() {
        let k = k.as_str();
        if item(t, k).and_then(|it| it.big).unwrap_or(false) && over(g, k, cards, i) { less(g, k, cards, i); return true; }
    }
    false
}

/// kitFitAll's whole body, in place (kit.ts:661)
fn fit_all(g: &mut Game, cards: &mut [MemberKit], t: &SlotTable, n: f64) -> Vec<Vec<String>> {
    let before: Vec<MemberKit> = cards.to_vec();
    home_shells(g, &mut cards[0]);
    if big_is_on(g) { for (i, c) in cards.iter_mut().enumerate() { big_home(c, i as f64); } }
    for i in 0..MEMBERS {
        for k in trim() {
            let k = k.as_str();
            let most = max_or(t, k, i, m::INF);
            while amount(g, k, &cards[i]) > most && over(g, k, cards, i) { less(g, k, cards, i); }
        }
        while cost_of(g, &cards[i], t) > t.pouches[i] && give(g, cards, t, i) {}
        while big_used(g, &cards[i], t) > big_slots(t, i) && give_big(g, cards, t, i) {}
    }
    let last = n as usize;
    if let Some(sq) = t.squad {
        while squad_used(g, cards, t, n) > sq {
            let mut gave = false;
            for i in (1..=last).rev() { if give(g, cards, t, i) { gave = true; break; } }
            if !gave { break; }
        }
    }
    for (k, it) in t.items.iter() {
        let Some(sm) = it.squad_max else { continue; };
        while squad_has_g(g, cards, &k, n) > sm {
            let mut gave = false;
            for i in (1..=last).rev() { if over(g, &k, cards, i) { less(g, &k, cards, i); gave = true; break; } }
            if !gave { break; }
        }
    }
    let mut out = Vec::new();
    for (i, c) in cards.iter().enumerate() {
        let mut row = Vec::new();
        for k in SLOT_KEYS {
            let n = amount(g, k, &before[i]) - amount(g, k, c);
            if n > 0.0 {
                row.push(if k == "shotgun" || k == "wand" || k == "hammer" { format!("the {}", word_of(k)) } else { format!("{} {}{}", ns(n), word_of(k), plural(n)) });
            }
        }
        out.push(row);
    }
    out
}

/// The table in force is taken: None, else why not (kit.ts slotsSet). The stub has no `raw`: whoever swaps the table has written it to `g.k.slots.table`, and on a refusal puts the old one back (the TypeScript's stands untouched).
pub fn slots_set(g: &mut crate::game::Game) -> Option<String> {
    let t = match slots_check(g) {
        crate::kit::SlotsCheck::Arm1(c) => return Some(c.why),
        crate::kit::SlotsCheck::Arm0(c) => c.table,
    };
    g.k.slots.table = t.clone();
    g.k.slots.ok = true;
    g.k.slots.why = String::new();
    g.m.kit.row_cache.clear();
    if rule_is_on(g) { let team = team_of(g); fit_table_cards(g, &t, team); }
    None
}

/// Why member i's card may not become `m`, in the card's words, or None (kit.ts cardWhy; STICKY.on settled on)
pub fn card_why(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> Option<String> {
    if !rule_is_on(g) { return None; }
    let iu = i as usize;
    let you = iu == 0;
    if you && !tubed(g, m) {
        let m0 = &g.m.kit.kit.members[0];
        if m.shells > m0.shells || m.cams > m0.cams {
            let s = if m.weapon == KitWeapon::Rifle {
                let a = crate::gunslots::pick_word(g, &m.primary, None);
                let picks = crate::gunslots::gun_picks(g, crate::gunslots::GunSlot::Primary);
                let b = crate::gunslots::pick_word(g, picks.first().map(|s| s.as_str()).unwrap_or(""), None);
                format!("no launcher under the {} to fire them: the {} has one", a, b)
            } else { String::from("no FN2000, no launcher to fire them: take the rifle first") };
            return Some(crate::gear::gear_text(g, &s));
        }
    }
    let t = table(g);
    let cur = g.m.kit.kit.members[iu].clone();
    let now = kit_cost(g, i, None);
    let then = kit_cost(g, i, Some(m));
    let need = then.used - now.used;
    for (k, it) in t.items.iter() {
        let more = amount(g, &k, m) - amount(g, &k, &cur);
        if more > 0.0 && amount(g, &k, m) > it.max[iu] {
            let word = word_of(&k);
            return Some(if js::truthy(it.max[iu]) { format!("{} {}{} is the most {} may carry", ns(it.max[iu]), word, plural(it.max[iu]), if you { "you" } else { "he" }) } else { format!("the {} is not {} to carry", word, if you { "yours" } else { "his" }) });
        }
    }
    if need > 0.0 && then.used > then.total {
        let room = if now.free > 0.0 { format!("only {} free", ns(now.free)) } else { String::from("none free") };
        return Some(format!("needs {}, {} ({} of {} full): give something up first", pouch_words(need), room, ns(now.used), ns(now.total)));
    }
    if let (Some(tb), Some(nb)) = (then.big, now.big) {
        if tb.used > nb.used && tb.used > tb.total { return Some(format!("{} big slot is full: one big thing to a man", if you { "your" } else { "his" })); }
    }
    let team = team_of(g);
    if you || i > team { return None; }
    if let Some(sq) = squad_cost(g, None, None) {
        if need > 0.0 && sq.used + need > sq.total {
            let room = if sq.free > 0.0 { format!("only {} free", ns(sq.free)) } else { String::from("none free") };
            return Some(format!("needs {}, the squad has {} ({} of {} full): take something off a teammate first", pouch_words(need), room, ns(sq.used), ns(sq.total)));
        }
    }
    let now_cards = members(g);
    for (k, it) in t.items.iter() {
        let all = squad_has_g(g, &now_cards, &k, team);
        let more = amount(g, &k, m) - amount(g, &k, &cur);
        if let Some(sm) = it.squad_max {
            if more > 0.0 && all + more > sm {
                let one = sm == 1.0;
                return Some(format!("{} {}{} to a squad, and a teammate carries {}", if one { String::from("one") } else { ns(sm) }, word_of(&k), plural(sm), if one { "it" } else { "them" }));
            }
        }
    }
    None
}

// ---------------------------------------------------------------- lines 707 to 770

/// loadoutCards' `wear`: the looks are handed on
fn wear(mut c: MemberKit, i: usize, looks: Option<&[MemberKit]>) -> MemberKit {
    if let Some(l) = looks { c.suit = l[i].suit; c.trim = l[i].trim; }
    c
}

/// The five cards of one of the table's named loadouts, or why they cannot be had here (kit.ts loadoutCards)
pub fn loadout_cards(g: &mut crate::game::Game, id_2: &str, looks: Option<&[crate::kit::MemberKit]>) -> crate::kit::LoadoutCards {
    if id_2 == "issued" {
        let cards: Vec<MemberKit> = (0..MEMBERS).map(|i| { let c = kit_default(g, i as f64); wear(c, i, looks) }).collect();
        return LoadoutCards::Array(cards);
    }
    let t = table(g);
    let Some(l) = t.loadouts.iter().find(|l| l.id == id_2) else {
        let ids: Vec<&str> = t.loadouts.iter().map(|l| l.id.as_str()).collect();
        let names = if ids.is_empty() { String::from("it names none") } else { ids.join(", ") };
        return LoadoutCards::Text(format!("no loadout \"{}\" in the table ({})", id_2, names));
    };
    let missing = loadout_missing(g, l);
    if !missing.is_empty() { return LoadoutCards::Text(format!("loadout \"{}\" carries {}, which this build has no row for", id_2, missing.join(", "))); }
    let mut cards: Vec<MemberKit> = Vec::new();
    for i in 0..MEMBERS {
        let mut c = kit_sanitize(g, i as f64, Some(&l.cards[i]));
        let iss = issued_at(i);
        for k in SLOT_KEYS {
            let n = l.counts[i].get(String::from(k)).copied().unwrap_or(0.0);
            put(g, k, &mut c, n, &iss);
        }
        cards.push(wear(c, i, looks));
    }
    if id_2 == "quiet" && stealth_on(g) {
        let h = match g.m.kit.hand { Some(h) => h, None => quiet_hand(g) };
        let c = stealth_issue(g, h);
        cards[0] = wear(c, 0, looks);
    }
    match illegal(g, &cards, &t, None) { Some(why) => LoadoutCards::Text(why), None => LoadoutCards::Array(cards) }
}

/// The stealth kit where no mission says (kit.ts QUIET_HAND; WAND.on settled on)
pub fn quiet_hand(g: &mut crate::game::Game) -> crate::kit::KitHand {
    crate::kit::KitHand { rings: g.k.kittree.lend, cams: 2.0, wand: true }
}

/// Put one of the table's named loadouts on all five cards, or on one man's (kit.ts kitLoadout; `kitSave` dropped)
pub fn kit_loadout(g: &mut crate::game::Game, id_2: &str, only: Option<f64>) -> Option<String> {
    let looks = members(g);
    let cards = match loadout_cards(g, id_2, Some(&looks)) { LoadoutCards::Text(why) => return Some(why), LoadoutCards::Array(c) => c };
    if let Some(only) = only {
        let ou = only as usize;
        let card = cards[ou].clone();
        return on_worn(g, only, &mut |g: &mut Game| -> Option<String> {
            if let Some(why) = card_why(g, only, &card) { return Some(why); }
            g.m.kit.kit.members[ou] = card.clone();
            g.m.kit.kit_note[ou] = String::new();
            None
        }, Some(&mut |_g: &mut Game, r: Option<String>| r.is_none()));
    }
    for (i, c) in cards.into_iter().enumerate() {
        g.m.kit.kit.members[i] = c;
        g.m.kit.kit_note[i] = String::new();
    }
    g.m.kit.deal_own = false;
    None
}

/// The squad kits a press can put on, in the screen's order (kit.ts kitLoadouts)
pub fn kit_loadouts(g: &mut crate::game::Game, only: Option<f64>) -> Vec<crate::kit::KitLoadoutsItem> {
    if let Some(only) = only {
        let ou = only as usize;
        let mut seen: JsSet<String> = JsSet::new();
        let mut out = Vec::new();
        for l in kit_loadouts(g, None) {
            let key = match loadout_cards(g, &l.id, None) { LoadoutCards::Text(_) => continue, LoadoutCards::Array(c) => card_key(&c[ou]) };
            if seen.has(key.clone()) { continue; }
            seen.add(key);
            out.push(l);
        }
        return out;
    }
    let mut out = alloc::vec![crate::kit::KitLoadoutsItem { id: String::from("issued"), name: String::from("As issued") }];
    if rule_is_on(g) {
        let t = table(g);
        for l in t.loadouts.iter() {
            if !loadout_missing(g, l).is_empty() { continue; }
            if let LoadoutCards::Text(_) = loadout_cards(g, &l.id, None) { continue; }
            out.push(crate::kit::KitLoadoutsItem { id: l.id.clone(), name: l.name.clone() });
        }
    }
    out
}

/// kitLoadoutOf's `same`
fn loadout_same(g: &mut Game, id: &str, only: Option<usize>) -> bool {
    let now = members(g);
    match loadout_cards(g, id, Some(&now)) {
        LoadoutCards::Text(_) => false,
        LoadoutCards::Array(cards) => cards.iter().enumerate().all(|(i, c)| only.map_or(false, |o| i != o) || alike_over(c, &now[i], true)),
    }
}

/// Which of them the five cards are now, colours aside (kit.ts kitLoadoutOf)
pub fn kit_loadout_of(g: &mut crate::game::Game, only: Option<f64>) -> Option<String> {
    if only.is_none() && g.m.kit.deal_own && dealing(g) && team_of(g) >= 3.0 { return None; }
    let ou = only.map(|o| o as usize);
    for l in kit_loadouts(g, only) {
        if loadout_same(g, &l.id, ou) { return Some(l.id); }
    }
    None
}

/// A row is on a card as it stands (kit.ts rowHas). The rows with a `has` are the tree's suppressor over a long gun and the launcher's three loads
pub fn row_has(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> bool {
    match r.field {
        KitField::Suppressed | KitField::Rings | KitField::Shells | KitField::Cams => seam_row_has(r, m),
        _ => true,
    }
}

/// The gun row it hangs under there, if any (kit.ts rowUnder). A second suppressor and the sponge rounds are rows of the tree alone; the other four hang only there
pub fn row_under(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> Option<crate::gunslots::GunSlot> {
    match r.field {
        KitField::SideCan => Some(crate::gunslots::GunSlot::Secondary),
        KitField::Rings => Some(crate::gunslots::GunSlot::Primary),
        KitField::Suppressed | KitField::Light | KitField::Shells | KitField::Cams => seam_row_under(r, m),
        _ => None,
    }
}

/// A row's value on a card (kit.ts rowGet). The field stands for the row: 'cans' and 'big' read through their own get (kit.ts:808, 837), the gun rows through theirs (the seam)
pub fn row_get(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> crate::kit::KitValue {
    let t = |s: &str| KitValue::Text(String::from(s));
    match r.field {
        KitField::Smoke => KitValue::Num(m.smoke),
        KitField::Light => KitValue::Bool(m.light),
        KitField::Wand => KitValue::Bool(m.wand),
        KitField::Bangs => KitValue::Num(m.bangs),
        KitField::Frags => KitValue::Num(m.frags),
        KitField::Shells => KitValue::Num(m.shells),
        KitField::Charges => KitValue::Num(m.charges),
        KitField::Cams => KitValue::Num(m.cams),
        KitField::Walls => KitValue::Num(m.walls),
        KitField::Suit => t(m.suit.as_str()),
        KitField::Bare => KitValue::Bool(m.bare == Some(true)),
        KitField::Big => t(big_of(m).as_str()),
        KitField::Rings => KitValue::Num(m.rings),
        KitField::Weapon => t(m.weapon.as_str()),
        KitField::Nvg => KitValue::Bool(m.nvg),
        KitField::Trim => t(m.trim.as_str()),
        KitField::Cans => t(mix_of(m).map_or("own", |x| x.as_str())),
        KitField::Primary | KitField::Secondary | KitField::Suppressed | KitField::SideCan => seam_row_get(r, m),
    }
}

fn v_num(v: &KitValue) -> f64 { match v { KitValue::Num(n) => *n, _ => unreachable!("a counted row's value is a number") } }
fn v_bool(v: &KitValue) -> bool { match v { KitValue::Bool(b) => *b, _ => unreachable!("a yes/no row's value is a boolean") } }
fn v_text(v: &KitValue) -> &str { match v { KitValue::Text(s) => s.as_str(), _ => unreachable!("a row of words has a word") } }

/// (kit.ts rowSet) The field stands for the row: 'cans', 'big' and the gun rows write through their own set (kit.ts:808, 837, 889, 892, 907, 917)
pub fn row_set(g: &mut crate::game::Game, r: &crate::kit::KitRow, m: &mut crate::kit::MemberKit, v: &crate::kit::KitValue) {
    match r.field {
        KitField::Smoke => m.smoke = v_num(v),
        KitField::Light => m.light = v_bool(v),
        KitField::Wand => m.wand = v_bool(v),
        KitField::Bangs => m.bangs = v_num(v),
        KitField::Frags => m.frags = v_num(v),
        KitField::Shells => m.shells = v_num(v),
        KitField::Charges => m.charges = v_num(v),
        KitField::Cams => m.cams = v_num(v),
        KitField::Walls => m.walls = v_num(v),
        KitField::Suit => {
            let w = v_text(v);
            m.suit = [SuitId::Issued, SuitId::Black, SuitId::Grey, SuitId::Navy, SuitId::Olive, SuitId::Jungle, SuitId::Khaki, SuitId::Desert].into_iter().find(|s| s.as_str() == w).expect("a suit's word");
        }
        KitField::Bare => m.bare = if v_bool(v) { Some(true) } else { None },
        KitField::Big => {
            let w = v_text(v);
            m.walls = if w == "wall" { 1.0 } else { 0.0 };
            m.big = if w == "shotgun" { BigKit::Shotgun } else if w == "hammer" { BigKit::Hammer } else { BigKit::None };
        }
        KitField::Primary => {
            let w = v_text(v);
            if w == crate::gunslots::NO_PRIMARY { m.weapon = KitWeapon::Pistol; } else { m.primary = String::from(w); m.weapon = KitWeapon::Rifle; }
            if rule_is_on(g) { home_shells(g, m); }
            if tree_up(g) { home_cans(g, m); }
        }
        KitField::Secondary => {
            m.secondary = String::from(v_text(v));
            if tree_up(g) { home_cans(g, m); }
        }
        KitField::SideCan => {
            if !tree_up(g) || can_choice(g, &m.secondary) { m.side_can = v_bool(v); }
        }
        KitField::Rings => m.rings = v_num(v),
        KitField::Weapon => {
            let w = v_text(v);
            m.weapon = [KitWeapon::Rifle, KitWeapon::Pistol, KitWeapon::Shotgun].into_iter().find(|s| s.as_str() == w).expect("a gun's word");
        }
        KitField::Suppressed => {
            if !tree_up(g) || can_choice(g, &m.primary) { m.suppressed = v_bool(v); }
        }
        KitField::Nvg => m.nvg = v_bool(v),
        KitField::Trim => {
            let w = v_text(v);
            m.trim = [TrimId::Issued, TrimId::Black, TrimId::Red, TrimId::Blue, TrimId::Green, TrimId::Amber, TrimId::White].into_iter().find(|s| s.as_str() == w).expect("a trim's word");
        }
        KitField::Cans => {
            if let KitValue::Text(w) = v {
                if let Some(x) = can_mixes().get(w.clone()) { m.smoke = x.smoke; m.bangs = x.stun; }
            }
        }
    }
    if rule_is_on(g) && r.field == KitField::Weapon { home_shells(g, m); }
}

/// (kit.ts yesNo) Its closure went only to KitRow.word, which the skeleton dropped with the menu: nothing to hand back
pub fn yes_no(yes: &str, no: &str) {}

/// (kit.ts upTo)
pub fn up_to(max: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut n = 0.0;
    while n < max + 1.0 { out.push(n); n += 1.0; }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(charges: f64, bangs: f64, frags: f64, smoke: f64) -> MemberKit {
        MemberKit { charges, bangs, frags, smoke, ..MemberKit::zeroed() }
    }

    #[test]
    fn deal_word_is_the_teammates_counts_in_dealt_order() {
        let cards = [card(9.0, 9.0, 9.0, 9.0), card(1.0, 2.0, 1.0, 0.0), card(1.0, 1.0, 1.0, 0.0)];
        assert_eq!(deal_word(&cards), "1,2,1,0|1,1,1,0");
    }

    #[test]
    fn same_kit_leaves_out_the_colours_and_keeps_a_spread_of_bare() {
        let a = MemberKit { suit: SuitId::Black, trim: TrimId::Red, ..card(1.0, 1.0, 0.0, 0.0) };
        let b = MemberKit { suit: SuitId::Olive, trim: TrimId::Amber, ..card(1.0, 1.0, 0.0, 0.0) };
        assert!(same_kit(&a, &b));
        assert!(!same_kit(&a, &card(2.0, 1.0, 0.0, 0.0)));
        // `{ ...a, ...b }` keeps a's `bare` where b has none: alike; b's own `bare` on a card without one is a key the other lacks: not alike
        let barred = MemberKit { bare: Some(true), ..a.clone() };
        assert!(same_kit(&barred, &b));
        assert!(!same_kit(&a, &MemberKit { bare: Some(true), ..b.clone() }));
    }

    #[test]
    fn this_files_word_lists_are_the_skeletons_tables() {
        assert_eq!(slot_amount().keys(), SLOT_KEYS.map(String::from).to_vec());
        assert_eq!(dealt().to_vec(), DEALT.map(String::from).to_vec());
        assert_eq!(deal_turn(), DEAL_TURN.map(|i| i as f64).to_vec());
        assert_eq!(deal_tool(), DEAL_TOOL.iter().map(|d| (d.0 as f64, String::from(d.2))).collect::<Vec<_>>());
        for d in DEAL_TOOL { assert_eq!(d.1.as_str(), d.2); }
    }

    #[test]
    fn up_to_counts_from_nothing_to_the_most() {
        assert_eq!(up_to(3.0), alloc::vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(up_to(0.0), alloc::vec![0.0]);
    }
}
