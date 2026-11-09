//! WHAT A MACHINE KEEPS OF HOW IT PLAYS (`Kept`): one record, which crosses as one JSON text and is laid on a game before it is made (`host::lay`). And the cards as a machine keeps them: one text (JSON), read into a game's
//! cards and written from them.
use crate::json::{parse, Json};
use alloc::{format, string::String, vec::Vec};
use game::difficulty::LeverKey;
use game::game::Game;
use game::js::JsMap;
use game::kit::{self, KitValue, KitWeapon, MemberKit};

/// WHAT A MACHINE KEEPS, and nothing of an address. Each field absent is nothing kept. A rung or a row's name that is none is nothing, as a store's read takes it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Kept {
    /// hard, or not
    pub difficulty: Option<bool>,
    /// the levers moved, each with its rung
    pub levers: Vec<(LeverKey, String)>,
    /// the men's cards, as their own text (`read`)
    pub cards: Option<String>,
    /// the rung a quiet mission is played under
    pub quiet_fog: Option<String>,
    /// the cameras a quiet mission hands out
    pub quiet_cams: Option<bool>,
    /// the panel's kept rows, each by its name and as the row shows it (shown.rs)
    pub rows: Vec<(String, f64)>,
    /// the panel's picks, each by its action's name and the word picked (actions.rs)
    pub picks: Vec<(String, String)>,
}

impl Kept {
    /// the record in its text: `{"hard":true,"levers":{"fog":"own"},"cards":"…","quietFog":"own","quietCams":false,"rows":{"wind.def":0.5},"picks":{"nerve":"steady"}}`. An empty text keeps
    /// nothing. A text that is no such record is refused with the field's name.
    pub fn read(text: &str) -> Result<Kept, String> {
        let mut out = Kept::default();
        if text.is_empty() { return Ok(out); }
        let j = parse(text).map_err(|e| format!("what the machine keeps is no JSON: {e}"))?;
        let Json::Obj(..) = j else { return Err(String::from("what the machine keeps is no record")) };
        for (k, v) in j.members() {
            let wrong = |what: &str| format!("what the machine keeps says `{k}` as no {what}");
            match k.as_str() {
                "hard" => out.difficulty = Some(v.bool().ok_or_else(|| wrong("yes or no"))?),
                "levers" => for (key, rung) in members(v).ok_or_else(|| wrong("record"))? {
                    let rung = rung.str().ok_or_else(|| wrong("record of words"))?;
                    if let Some(key) = lever(key) { out.levers.push((key, String::from(rung))); }
                },
                "cards" => out.cards = Some(String::from(v.str().ok_or_else(|| wrong("text"))?)),
                "quietFog" => out.quiet_fog = Some(String::from(v.str().ok_or_else(|| wrong("word"))?)),
                "quietCams" => out.quiet_cams = Some(v.bool().ok_or_else(|| wrong("yes or no"))?),
                "rows" => for (name, n) in members(v).ok_or_else(|| wrong("record"))? { out.rows.push((name.clone(), n.num().ok_or_else(|| wrong("record of numbers"))?)); },
                "picks" => for (name, word) in members(v).ok_or_else(|| wrong("record"))? { out.picks.push((name.clone(), String::from(word.str().ok_or_else(|| wrong("record of words"))?))); },
                _ => return Err(format!("what the machine keeps has no field `{k}`")),
            }
        }
        Ok(out)
    }
}

fn members(j: &Json) -> Option<&[(String, Json)]> { if matches!(j, Json::Obj(..)) { Some(j.members()) } else { None } }
fn lever(key: &str) -> Option<LeverKey> { [LeverKey::Fog, LeverKey::Ai, LeverKey::Kit, LeverKey::Told].into_iter().find(|k| k.as_str() == key) }

/// the name the cards are kept under
pub const KIT: &str = "twopointsix.kit";

/// the fields of one kept card
fn fields(j: &Json) -> Option<JsMap<String, KitValue>> {
    let (Json::Obj(..) | Json::Arr(..)) = j else { return None };
    let mut out = JsMap::new();
    for (k, v) in j.members() {
        match v {
            Json::Bool(b) => out.set(k.clone(), KitValue::Bool(*b)),
            Json::Num(n) => out.set(k.clone(), KitValue::Num(*n)),
            Json::Str(s) => out.set(k.clone(), KitValue::Text(s.clone())),
            // (a null or a list is no row's value, but the key is there: a card with no `smoke` at all is one from before the pouches)
            _ => out.set(k.clone(), KitValue::Num(f64::NAN)),
        };
    }
    Some(out)
}

/// the cards read from what was kept: a text that is none, and a card that is none, are as issued. A kept card that does not fit is brought inside its pouches, with a
/// line of what was left behind
pub fn read(g: &mut Game, text: &str) {
    let m = &mut g.m.kit;
    m.hand = None;
    m.loud_you = None;
    m.stealth_you = None;
    m.kept0 = None;
    m.kept_wand = Vec::new();
    m.deal_own = false;
    m.team = (m.kit.members.len() - 1) as f64;
    let j = parse(text).unwrap_or(Json::Null);
    for i in 0..g.m.kit.kit.members.len() {
        let raw = fields(j.get("members").at(i));
        let c = kit::kit_sanitize(g, i as f64, raw.as_ref());
        g.m.kit.kit.members[i] = c;
        g.m.kit.kit_note[i] = String::new();
    }
    g.m.kit.deal_own = j.get("own").bool() == Some(true);
    let stealth = if j.get("stealth").bool() == Some(true) { j.get("members").at(0) } else { j.get("stealth") };
    if let Some(f) = fields(stealth) { kit::kit_stealth_set(g, Some(&f)); }
    kit::kit_built_home(g);
    let mut cards = g.m.kit.kit.members.clone();
    let left = kit::kit_fit_all(g, &mut cards, None, None);
    g.m.kit.kit.members = cards;
    for (i, what) in left.iter().enumerate() {
        if what.is_empty() { continue; }
        let gunless = i == 0 && g.m.kit.kit.members[0].weapon != KitWeapon::Rifle && what.len() == 1;
        g.m.kit.kit_note[i] = kit::kit_left_words(g, what, gunless);
    }
}

/// a text as JSON has it
pub fn text(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// one card as it is kept
pub fn written(m: &MemberKit) -> String {
    let bare = match m.bare { Some(b) => format!(",\"bare\":{b}"), None => String::new() };
    format!(
        "{{\"weapon\":{},\"suppressed\":{},\"light\":{},\"nvg\":{},\"bangs\":{},\"charges\":{},\"smoke\":{},\"frags\":{},\"walls\":{},\"shells\":{},\"suit\":{},\"trim\":{},\"wand\":{},\"cams\":{},\"big\":{},\"primary\":{},\"secondary\":{},\"sideCan\":{},\"rings\":{}{bare}}}",
        text(m.weapon.as_str()), m.suppressed, m.light, m.nvg, m.bangs, m.charges, m.smoke, m.frags, m.walls, m.shells, text(m.suit.as_str()), text(m.trim.as_str()), m.wand, m.cams, text(m.big.as_str()), text(&m.primary), text(&m.secondary), m.side_can, m.rings
    )
}

/// the cards as they are kept. While a stealth mission's card stands for yours, yours is the one parked, in the colors you wear now
pub fn write(g: &mut Game) -> String {
    let stealth = kit::stealth_kept(g).stealth;
    let k = &g.m.kit;
    let mut members = k.kit.members.clone();
    if let (Some(_), Some(loud)) = (&k.hand, &k.loud_you) { members[0] = MemberKit { suit: members[0].suit, trim: members[0].trim, ..loud.clone() }; }
    let own = if k.deal_own { ",\"own\":true" } else { "" };
    let stealth = match &stealth { Some(c) => format!(",\"stealth\":{}", written(c)), None => String::new() };
    format!("{{\"v\":2{own}{stealth},\"members\":[{}]}}", members.iter().map(written).collect::<Vec<_>>().join(","))
}
