//! The team as it would go on a mission, as the screen before it shows it, and what a screen's presses do to the
//! cards. A read takes `&mut Game` only because the helpers do: it leaves the cards as they were.
use super::*;
use crate::game::Game;
use crate::js::{self, JsMap};
use crate::loadout::{any_size, kit_as_issued, issue_note, lesson_note, roster_of, size_words, squad_choices, LoadoutEntry, LoadoutKind, SquadChoice};
use crate::query::Query;
use crate::scene_scenarios::{IssueCard, IssueData};
use crate::view::{CardPouches, TeamKit, TeamMan, TeamSize, TeamThing, TeamView};
use crate::weapons::WeaponClass;
use alloc::{format, string::String, vec::Vec};

/// The mission the cards are for, as it would deploy.
#[derive(Clone, Debug)]
pub struct TeamFor {
    pub entry: LoadoutEntry,
    /// the words of the link it would be played by
    pub link: Query,
    /// the cards its level hands out, over the issue ones, by place
    pub issue: Option<JsMap<String, IssueCard>>,
    /// the wall charges its level hands out whatever the cards carry
    pub walls: f64,
}

impl TeamFor {
    /// The mission as the screen reads it: a lesson played as issued reads neither of its level's blocks, and a link with `noissue` wears the kept cards.
    pub fn new(g: &Game, entry: LoadoutEntry, link: Query, issue: Option<IssueData>, walls: f64) -> TeamFor {
        let lesson = kit_as_issued(&entry);
        let levelled = entry.level.is_some() && !lesson;
        let issue = issue.filter(|_| levelled && !link.has("noissue")).map(|d| d.members);
        let walls = if levelled && entry.kind == LoadoutKind::Roomclear { walls } else { 0.0 };
        TeamFor { entry, link, issue, walls }
    }
    /// The kit is not the cards': a lesson's, or the level's own.
    fn issued_only(&self) -> bool { self.issue.is_some() || kit_as_issued(&self.entry) }
}

/// There are cards to change.
fn cards_on(f: Option<&TeamFor>) -> bool { !f.is_some_and(|f| f.issued_only()) }

/// The cameras the link hands out where its fog runs.
fn link_cams(g: &Game, f: &TeamFor) -> f64 {
    if !crate::scenario_run::fog_of_query(g, &f.link) { return 0.0; }
    crate::scenario_run::quiet_cams_of_link(g, &f.link)
}

/// What the mission makes of a card.
pub fn team_card_for(g: &Game, f: &TeamFor) -> CardFor {
    CardFor { no_fog: !crate::scenario_run::fog_of_query(g, &f.link) }
}

/// The stealth card's kit, where it stands for yours.
pub fn team_hand(g: &Game, f: &TeamFor) -> Option<KitHand> {
    if !cards_on(Some(f)) || !matches!(f.entry.kind, LoadoutKind::Quiet | LoadoutKind::Solo) { return None; }
    let cams = link_cams(g, f);
    Some(KitHand { rings: g.k.kittree.lend, cams, wand: cams > 0.0 })
}

/// The size a mission opens on: the one that stands if it can take it, else its own, else the first it can.
pub fn team_opens_on(e: &LoadoutEntry, size: f64) -> f64 {
    let c = squad_choices(e);
    if c.iter().any(|x| x.n == size && x.ok) { return size; }
    if c.iter().any(|x| x.own && x.ok) { return e.squad; }
    c.iter().find(|x| x.ok).map_or(e.squad, |x| x.n)
}

/// How many come, or why not.
pub fn team_size(g: &mut Game, f: Option<&TeamFor>, n: f64) -> String {
    let choices = f.map_or_else(any_size, |f| squad_choices(&f.entry));
    let Some(c) = choices.iter().find(|c| c.n == n) else { return format!("no team of {}", js::num_str(n)) };
    if !c.ok { return c.note.clone(); }
    if n != kit_team(g, None) { kit_team(g, Some(n)); }
    String::new()
}

/// Who a named kit goes on: a man going alone wears it himself, a team on all five cards.
fn alone(g: &Game) -> Option<f64> { if g.m.kit.team == 0.0 { Some(0.0) } else { None } }

/// A named kit put on, or why not.
pub fn team_named(g: &mut Game, id: &str) -> Option<String> {
    let only = alone(g);
    kit_loadout(g, id, only)
}

/// What the mission hands man i, in a line.   (loadout.ts issueOf; a level's block has no cameras and no wand)
fn issue_words(c: &IssueCard) -> String {
    let count = |n: Option<f64>| n.filter(|n| *n > 0.0);
    let plain = |n: Option<f64>, w: &str| count(n).map(|k| format!("{} {w}", js::num_str(k)));
    let plural = |n: Option<f64>, w: &str| count(n).map(|k| format!("{} {w}{}", js::num_str(k), if k == 1.0 { "" } else { "s" }));
    let parts = [
        c.weapon.map(|w| String::from(w.as_str())),
        plain(c.bangs, "stun"),
        plural(c.charges, "charge"),
        plural(c.frags, "frag"),
        plain(c.smoke, "smoke"),
        plural(c.walls, "wall charge"),
        plural(c.shells, "shell"),
        plural(c.rings, "sponge round"),
    ];
    parts.into_iter().flatten().collect::<Vec<_>>().join(" · ")
}

/// Who the mission hands the shotgun.
fn issue_who(cards: &JsMap<String, IssueCard>, size: f64) -> String {
    let men = roster_of(size);
    let mut who = Vec::new();
    for i in 0..=(size as usize) {
        let Some(c) = cards.get(format!("{i}")) else { continue };
        if c.weapon != Some(KitWeapon::Shotgun) { continue; }
        who.push(if i == 0 { String::from("you carry the shotgun") } else { format!("{} carries the shotgun", men[i - 1].name) });
    }
    who.join(", ")
}

/// The note line.
fn note_line(f: Option<&TeamFor>, c: &SquadChoice, size: f64) -> String {
    let said = if !c.note.is_empty() { c.note.as_str() } else if c.own { "This mission's own team." } else { "" };
    let Some(f) = f else { return String::from(said) };
    if let Some(cards) = &f.issue { return issue_note(said, &issue_who(cards, size)); }
    if kit_as_issued(&f.entry) { return lesson_note(said); }
    String::from(said)
}

/// What is carried between them.
fn pool_line(g: &mut Game, f: Option<&TeamFor>, size: f64) -> String {
    let issued = f.map_or(0.0, |f| f.walls);
    let walls_word = |n: f64| format!("{} wall charge{}", js::num_str(n), if n == 1.0 { "" } else { "s" });
    if !cards_on(f) {
        if f.is_some_and(|f| f.issue.is_some()) && issued != 0.0 { return format!("{} · {} (issued for this mission)", if size != 0.0 { "Squad carries" } else { "You carry" }, walls_word(issued)); }
        return String::new();
    }
    let own = kit_walls(g, size);
    let top = own.max(issued);
    let walls = if top != 0.0 { format!("{}{}", walls_word(top), if issued > own { " (issued for this mission)" } else { "" }) } else { String::new() };
    if size == 0.0 { return if issued != 0.0 { format!("You carry · {walls}") } else { String::new() }; }
    let p = kit_pool(g, size, None);
    let smoke = kit_smokes(g, size);
    let mut line = format!("Squad carries · {} stun · {} charge{} · {} frag{}", js::num_str(p.bangs), js::num_str(p.charges), if p.charges == 1.0 { "" } else { "s" }, js::num_str(p.frags), if p.frags == 1.0 { "" } else { "s" });
    if !walls.is_empty() { line.push_str(&format!(" · {walls}")); }
    if smoke != 0.0 { line.push_str(&format!(" · {} smoke", js::num_str(smoke))); }
    line
}

/// The kind of drawing the gun of a weapon row is.
fn gun_kind(g: &Game, w: crate::weapons::WeaponId) -> &'static str {
    match g.m.weapons.weapons.get(w).expect("a card's row").cls {
        WeaponClass::Pistol => "pistol",
        WeaponClass::Shotgun => "shotgun",
        _ => "rifle",
    }
}

/// A chip of what a man carries, for one of his rows: what the row's value is on his chip, under the row his chip draws it by.
fn thing_of(g: &Game, r: &KitRow, m: &MemberKit) -> Option<TeamThing> {
    let count = |n: f64| if n == 0.0 { None } else { Some(TeamThing { field: String::from(r.field.as_str()), value: js::num_str(n), label: r.label.clone(), n, can: false, icon: String::new() }) };
    match r.field {
        KitField::Primary => {
            // (no long gun brought: his sidearm is the gun the chip draws)
            let (field, value) = if m.weapon == KitWeapon::Pistol { (KitField::Secondary, &m.secondary) } else { (KitField::Primary, &m.primary) };
            if value == "none" { return None; }
            Some(TeamThing { field: String::from(field.as_str()), value: value.clone(), label: r.label.clone(), n: 0.0, can: kit_row_can(g, m, field), icon: pick_kind(g, value) })
        }
        KitField::Wand => m.wand.then(|| TeamThing { field: String::from("wand"), value: String::from("true"), label: r.label.clone(), n: 0.0, can: false, icon: String::new() }),
        KitField::Big => match big_of(m) {
            BigPick::None => None,
            b => Some(TeamThing { field: String::from("big"), value: String::from(b.as_str()), label: r.label.clone(), n: 0.0, can: false, icon: String::new() }),
        },
        KitField::Bangs => count(m.bangs),
        KitField::Charges => count(m.charges),
        KitField::Smoke => count(m.smoke),
        KitField::Frags => count(m.frags),
        KitField::Shells => count(m.shells),
        KitField::Cams => count(m.cams),
        KitField::Rings => count(m.rings),
        _ => None,
    }
}

/// The five places as the screen draws them, with no worn card yet.
fn men_of(g: &mut Game, f: Option<&TeamFor>, size: f64, clip: Option<&MemberKit>) -> Vec<TeamMan> {
    let (men, all) = (roster_of(size), roster_of(4.0));
    let on = cards_on(f);
    let mut out = Vec::new();
    for i in 0..=4usize {
        let man = if i == 0 { None } else { men.get(i - 1) };
        let here = i == 0 || man.is_some();
        let mut t = TeamMan {
            here,
            title: String::from(if i == 0 { "You" } else { man.map_or("Empty", |m| m.name) }),
            sub: String::from(match (i, man) {
                (0, _) => "point man",
                (_, None) => "stays behind",
                (_, Some(m)) if size > 2.0 => if m.pair == crate::game::PairId::Red { "red pair" } else { "blue pair" },
                _ => if size == 2.0 { "the pair" } else { "with you" },
            }),
            pair: String::from(match man { Some(m) => m.pair.as_str(), None if i == 0 => "you", None => all[i - 1].pair.as_str() }),
            ..TeamMan::default()
        };
        if here && on {
            let who = i as f64;
            let m = kit_shown(g, who);
            let gun = kit_gun(g, &m);
            let cost = kit_cost(g, who, None);
            t.changed = kit_changed(g, who);
            t.words = kit_words(g, who, None);
            t.pouches = CardPouches { used: cost.used, total: cost.total };
            t.gun = String::from(gun.as_str());
            t.icon = String::from(gun_kind(g, gun));
            t.can = kit_row_can(g, &m, if m.weapon == KitWeapon::Pistol { KitField::Secondary } else { KitField::Primary });
            t.clip = clip.is_some_and(|c| kit_wears(g, who, c));
            t.things = kit_rows(g, who).iter().filter_map(|r| thing_of(g, r, &m)).collect();
        } else if here {
            if let Some(cards) = f.and_then(|f| f.issue.as_ref()) {
                let words = cards.get(format!("{i}")).map_or(String::new(), issue_words);
                t.words = if words.is_empty() { String::from("as issued") } else { words };
            }
        }
        out.push(t);
    }
    out
}

/// The team as it would go; the worn cards come apart (`team_worn`: a card is kept in a form only the machine's side knows). Changes nothing.
pub fn team_view(g: &mut Game, f: Option<&TeamFor>, clip: Option<&MemberKit>) -> TeamView {
    let size = kit_team(g, None);
    let choices = f.map_or_else(any_size, |f| squad_choices(&f.entry));
    let at = &choices[size as usize];   // (the size is always one of the five: `kit_team` holds it there)
    let on = cards_on(f);
    let sq = if on { squad_cost(g, None, None) } else { None };
    let squad = match sq { Some(s) if size != 0.0 => format!("Squad pouches {} / {}", js::num_str(s.used), js::num_str(s.total)), _ => String::new() };
    let mut kits = Vec::new();
    if on {
        let only = alone(g);
        let now = kit_loadout_of(g, only);
        let mine = if only.is_some() { Some(kit_loadouts(g, only)) } else { None };
        for l in kit_loadouts(g, None) {
            let offered = mine.as_ref().map_or(true, |m| m.iter().any(|x| x.id == l.id));
            kits.push(TeamKit { now: now.as_deref() == Some(l.id.as_str()), id: l.id, name: l.name, offered });
        }
    }
    let you = if on { Some(kit_worn(g, Some(size), None).swap_remove(0)) } else { None };
    TeamView {
        size,
        sizes: choices.iter().map(|c| TeamSize { n: c.n, ok: c.ok, own: c.own, words: size_words(c.n), note: c.note.clone() }).collect(),
        note: note_line(f, at, size),
        cards: on,
        as_issued: f.is_some_and(|f| kit_as_issued(&f.entry)),
        squad,
        pool: pool_line(g, f, size),
        walls: f.is_some_and(|f| f.walls != 0.0),
        kits,
        men: men_of(g, f, size, clip),
        frags: you.as_ref().map_or(0.0, |y| y.frags),
        smoke: you.as_ref().map_or(0.0, |y| y.smoke),
        bangs: if on { kit_pool(g, size, None).bangs } else { 0.0 },
        link: String::new(),
    }
}

/// The card each man who comes will be staged with: his kept card as the team wears it, or the issue card with the mission's fields over it, and what the mission hands you over
/// that where there is no card of yours to stand for it. None: he stays behind. Changes nothing.
pub fn team_worn(g: &mut Game, f: Option<&TeamFor>) -> Vec<Option<MemberKit>> {
    let size = kit_team(g, None);
    let worn = match f.filter(|f| f.issued_only()) {
        None => kit_worn(g, Some(size), None),
        Some(f) => {
            let cards: Vec<MemberKit> = (0..g.m.kit.kit.members.len()).map(|j| {
                let mut c = kit_default(g, j as f64);
                if let Some(k) = f.issue.as_ref().and_then(|d| d.get(format!("{j}"))) { assign_card(&mut c, k); }
                c
            }).collect();
            kit_worn(g, Some(size), Some(&cards))
        }
    };
    let (mut cams, mut wand) = (0.0, false);
    if let Some(f) = f.filter(|f| f.issued_only() && !kit_issue_whole(f.issue.as_ref())) {
        let n = link_cams(g, f);
        cams = if g.m.kit.kit.members[0].weapon == KitWeapon::Rifle { n } else { 0.0 };
        wand = n > 0.0;
    }
    worn.into_iter().enumerate().map(|(i, mut w)| {
        if i as f64 > size { return None; }
        if i == 0 && (cams > w.cams || (wand && !w.wand)) {
            w.cams = w.cams.max(cams);
            w.wand = w.wand || wand;
        }
        Some(w)
    }).collect()
}
