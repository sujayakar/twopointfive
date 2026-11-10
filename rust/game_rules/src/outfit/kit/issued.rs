use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// how many cards the table holds, as an index
const CARDS: usize = KIT_MEMBERS as usize;

// ------------------------------------------------------------ small asks of this file

/// `issuing` as a condition (true and 'own' both hold)
fn issuing_on(g: &Game) -> bool { !matches!(g.m.kit.issuing, KitModIssuing::Bool(false)) }
/// `WEAPONS[w]`, a copy to hand on while `g` is lent
fn weapon_row(g: &Game, w: crate::weapons::WeaponId) -> crate::weapons::WeaponDef { g.m.weapons.weapons.get(w).unwrap().clone() }
/// `gd.weapon.def.cls === 'pistol'`, of a gun's row
fn is_pistol(g: &Game, w: crate::weapons::WeaponId) -> bool { g.m.weapons.weapons.get(w).unwrap().cls == crate::weapons::WeaponClass::Pistol }
/// `SUITS[s].tint`
fn suit_tint(s: SuitId) -> V3 { suits().get(String::from(s.as_str())).unwrap().tint }
/// `TRIMS[t].tint`
fn trim_tint(t: TrimId) -> V3 { trims().get(String::from(t.as_str())).unwrap().tint }
/// `g.guards.some(gd => !!gd.unit && gd.team === 'operative')`
fn team_on_floor(g: &Game) -> bool {
    g.guards.iter().any(|&gd| g.gd[gd].unit.is_some() && g.gd[gd].team == crate::teams::Team::Operative)
}
/// `kitIssueWhole(g.level.scenarios?.issue?.members)`
fn issue_names_you(g: &Game) -> bool {
    let members = g.level.scenarios.as_ref().and_then(|s| s.issue.as_ref()).map(|d| d.members.clone());
    kit_issue_whole(members.as_ref())
}
/// `const { loaded, ...card } = c; Object.assign(m, card)`: the fields a level's block names
pub(super) fn assign_card(m: &mut MemberKit, c: &crate::scene_scenarios::IssueCard) {
    if let Some(x) = c.weapon { m.weapon = x; }
    if let Some(x) = c.suppressed { m.suppressed = x; }
    if let Some(x) = c.light { m.light = x; }
    if let Some(x) = c.nvg { m.nvg = x; }
    if let Some(x) = c.bangs { m.bangs = x; }
    if let Some(x) = c.charges { m.charges = x; }
    if let Some(x) = c.smoke { m.smoke = x; }
    if let Some(x) = c.frags { m.frags = x; }
    if let Some(x) = c.walls { m.walls = x; }
    if let Some(x) = c.shells { m.shells = x; }
    if let Some(x) = c.rings { m.rings = x; }
}

// ------------------------------------------------------------ the page's kind, a mission's issue

/// The page of a lesson says so: its stagings wear the issue.
pub fn kit_lesson_page(g: &mut crate::game::Game, on: bool) { g.m.kit.lesson_page = on; }

/// Every staging on this page wears the issue cards.
fn page_issued(g: &crate::game::Game) -> bool { g.m.kit.issued_page || g.m.kit.lesson_page }

/// The level's block names a card for you: nothing is handed over it.
pub(super) fn kit_issue_whole(members: Option<&JsMap<String, crate::scene_scenarios::IssueCard>>) -> bool {
    members.map_or(false, |m| m.has(String::from("0")))
}

/// A mission's own issue: `f` (the staging, asked again) runs under the mission's cards. False: no such block, or already issuing.
fn kit_issue(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) -> bool {
    if issuing_on(g) { return false; }
    let Some(d) = g.level.scenarios.as_ref().and_then(|s| s.issue.clone()) else { return false; };
    as_issued(g, &mut |g| {
        g.m.kit.issuing = KitModIssuing::Own;
        for (k, c) in d.members.iter() {
            let Ok(i) = k.parse::<usize>() else { continue; };
            if let Some(m) = g.m.kit.kit.members.get_mut(i) { assign_card(m, c); }
        }
        f(g);
    });
    true
}

/// The issue cards stand in the table for the length of `f`; the table, notes, store and team size are put back.
fn as_issued(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) {
    let cards = g.m.kit.kit.members.clone();
    let notes = g.m.kit.kit_note.clone();
    let (t, own, h) = (g.m.kit.team, g.m.kit.deal_own, g.m.kit.hand);
    g.m.kit.issuing = KitModIssuing::Bool(true);
    g.m.kit.deal_own = false;
    g.m.kit.hand = None;
    for i in 0..CARDS {
        let c = kit_default(g, i as f64);
        g.m.kit.kit.members[i] = c;
        g.m.kit.kit_note[i] = String::new();
    }
    f(g);
    for i in 0..CARDS {
        g.m.kit.kit.members[i] = cards[i].clone();
        g.m.kit.kit_note[i] = notes[i].clone();
    }
    g.m.kit.team = t;
    g.m.kit.deal_own = own;
    g.m.kit.hand = h;
    g.m.kit.issuing = KitModIssuing::Bool(false);
}

// ------------------------------------------------------------ on the floor

/// The card a staged teammate was staged with (a copy taken then).
pub fn kit_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::kit::MemberKit> { g.m.kit.man_of.get(gd).map(|e| e.kit.clone()) }

/// The card he was staged with in a line, for the HUD ('' = none): worked out as he is staged, ten microseconds a squad, so no tick asks it
pub fn kit_line(g: &crate::game::Game, gd: GuardId) -> &str { g.m.kit.man_of.get(gd).map_or("", |e| e.words.as_str()) }
/// Each staged man's line said again, for a game read in from the wire, which does not carry them
pub fn kit_words_again(g: &mut crate::game::Game) {
    let men = g.m.kit.man_of.keys();
    for gd in men {
        let Some(e) = g.m.kit.man_of.get(gd).cloned() else { continue };
        let words = if e.i != 0.0 { kit_card_words(g, e.i, &e.kit) } else { String::new() };
        g.m.kit.man_of.set(gd, crate::kit::KitModManOfValue { words, ..e });
    }
}

/// Which member a staged teammate is (1 to 4), or 0.
pub fn kit_index(g: &crate::game::Game, gd: GuardId) -> f64 { g.m.kit.man_of.get(gd).map_or(0.0, |e| e.i) }

/// He wears goggles.
pub fn kit_goggles(g: &crate::game::Game, gd: GuardId) -> bool { !g.m.kit.bare_eyed.has(gd) }

/// One of a staged team, and his card has him carry a pistol.
pub fn kit_pistol(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.m.kit.man_of.has(gd) { return false; }
    let w = g.fa[g.gd[gd].weapon].def;
    is_pistol(g, w)
}

/// The breaching shotgun or the hammer in a staged teammate's big slot.
pub fn kit_big(g: &crate::game::Game, gd: GuardId) -> crate::kit::BigKit {
    match g.m.kit.man_of.get(gd) {
        Some(e) if !js::truthy(e.kit.walls) => e.kit.big,
        _ => BigKit::None,
    }
}

/// Your card left you the weapon light (L works).
pub fn kit_you_light(g: &crate::game::Game, pl: PlayerId) -> bool {
    !g.m.kit.you_bare.get(pl).map_or(false, |b| b.light)
}

/// Your card left you the goggles (N works).
pub fn kit_you_nvg(g: &crate::game::Game, pl: PlayerId) -> bool {
    !g.m.kit.you_bare.get(pl).map_or(false, |b| b.nvg)
}

/// A run that is played quietly: a quiet mission, or the plain night alone off the loadout screen.
pub(super) fn stealth_run(g: &crate::game::Game) -> bool {
    g.staged.facts.unseen || (g.solo_card && !team_on_floor(g))
}

/// Your card onto the fresh Player.
pub(in crate::outfit) fn kit_player(g: &mut crate::game::Game) {
    if !g.m.kit.kit.on { return; }
    if kit_issue(g, &mut |g| kit_player(g)) { return; }
    if page_issued(g) && !issuing_on(g) { return as_issued(g, &mut |g| kit_player(g)); }
    let own = !issuing_on(g);
    if own && !g.m.kit.staged { return for_run(g, &mut |g| kit_player(g)); }
    let lent = !own && !(matches!(g.m.kit.issuing, KitModIssuing::Own) && issue_names_you(g));
    let mut m = g.m.kit.kit.members[0].clone();
    let pl = g.player;
    if !g.m.kit.issued_player.has(pl) {
        let (rifle, side, c) = (g.pl[pl].rifle, g.pl[pl].pistol, g.pl[pl].ch);
        let cams = if g.fa[rifle].attachments.contains(&crate::weapons::RifleAttachment::Cam) { g.fa[rifle].cams } else { -1.0 };
        let was = KitModIssuedPlayerValue {
            rifle: g.pl[pl].has_rifle,
            gun: rifle,
            side,
            slot: g.pl[pl].slot,
            smoke: g.pl[pl].canisters,
            stun: g.pl[pl].flashbangs,
            frags: g.pl[pl].frags,
            shells: g.fa[rifle].shells,
            rings: g.fa[rifle].rings,
            wand: g.pl[pl].wand,
            big: g.pl[pl].big_kit,
            cams,
            tint: g.ch[c].tint,
            tint2: g.ch[c].tint2,
        };
        g.m.kit.issued_player.set(pl, was);
    }
    let counts = !(page_issued(g) && matches!(g.m.kit.issuing, KitModIssuing::Bool(true)));
    big_home(&mut m, 0.0);
    g.m.kit.kit.members[0] = m.clone();
    g.pl[pl].big_kit = if !js::truthy(m.walls) { m.big } else { BigKit::None };
    if m.weapon == KitWeapon::Pistol && g.pl[pl].has_rifle {
        g.pl[pl].has_rifle = false;
        if g.pl[pl].slot == crate::game::SlotKind::Rifle { g.pl[pl].slot = crate::game::SlotKind::Pistol; }
    }
    let long = crate::gunslots::pick_row(g, &m.primary, true);
    let side = crate::gunslots::pick_row(g, &m.secondary, true);
    if let Some(long) = long {
        if g.pl[pl].has_rifle {
            let want = mine(g, long, &m, crate::gunslots::GunSlot::Primary);
            if g.fa[g.pl[pl].rifle].def != want.id { g.pl[pl].rifle = crate::weapons::rifle_new(g, Some(&want)); }
        }
    }
    if let Some(side) = side {
        let want = mine(g, side, &m, crate::gunslots::GunSlot::Secondary);
        if g.fa[g.pl[pl].pistol].def != want.id { g.pl[pl].pistol = crate::weapons::pistol_new(g, Some(&want)); }
    }
    if counts {
        g.pl[pl].canisters = m.smoke;
        g.pl[pl].flashbangs = m.bangs;
        let (slot, can, fb) = (g.pl[pl].slot, g.pl[pl].canisters, g.pl[pl].flashbangs);
        if (slot == crate::game::SlotKind::Smoke && !js::truthy(can)) || (slot == crate::game::SlotKind::Flash && !js::truthy(fb)) {
            g.pl[pl].slot = crate::game::SlotKind::Pistol;
        }
        g.pl[pl].frags = m.frags;
        if g.pl[pl].slot == crate::game::SlotKind::Frag && !js::truthy(g.pl[pl].frags) { g.pl[pl].slot = crate::game::SlotKind::Pistol; }
        let rf = g.pl[pl].rifle;
        if g.fa[rf].attachments.contains(&crate::weapons::RifleAttachment::He40) { g.fa[rf].shells = m::min(m.shells, SHELLS_MAX); }
    }
    if counts && cam_row(g) && g.pl[pl].has_rifle {
        let n = if tubed(g, &m) { m::max(m.cams, if lent { crate::stickycam::sticky_visit(g) } else { 0.0 }) } else { 0.0 };
        crate::stickycam::sticky_issue(g, pl, n, None);
    }
    if counts && g.fa[g.pl[pl].rifle].attachments.contains(&crate::weapons::RifleAttachment::Ring) {
        let rf = g.pl[pl].rifle;
        let handed = if lent && stealth_run(g) { g.k.kittree.lend } else { 0.0 };
        g.fa[rf].rings = m::min(RINGS_MAX, m::max(m.rings, handed));
        if !js::truthy(g.fa[rf].rings) && crate::weapons::rifle_attachment(g, rf) == Some(crate::weapons::RifleAttachment::Ring) {
            let (shells, cams) = (g.fa[rf].shells, g.fa[rf].cams);
            let k = g.fa[rf].attachments.iter().position(|&a| match a {
                crate::weapons::RifleAttachment::He40 => shells > 0.0,
                crate::weapons::RifleAttachment::Cam => cams > 0.0,
                crate::weapons::RifleAttachment::Ring => false,
            });
            if let Some(k) = k { g.fa[rf].att_idx = k as i32; }
        }
    }
    g.m.kit.you_bare.set(pl, KitModYouBareValue { light: !m.light, nvg: !m.nvg });
    if !m.light {
        let (rf, sd) = (g.pl[pl].rifle, g.pl[pl].pistol);
        g.fa[rf].light_on = false;
        g.fa[sd].light_on = false;
    }
    if !m.nvg { g.pl[pl].nv = false; }
    if matches!(g.m.kit.issuing, KitModIssuing::Own) {
        let up = g.level.scenarios.as_ref().and_then(|s| s.issue.as_ref()).and_then(|d| d.members.get(String::from("0"))).and_then(|c| c.loaded);
        if let Some(up) = up {
            let a = match up {
                crate::scene_scenarios::IssueCardLoaded::Ring => crate::weapons::RifleAttachment::Ring,
                crate::scene_scenarios::IssueCardLoaded::He40 => crate::weapons::RifleAttachment::He40,
            };
            let rf = g.pl[pl].rifle;
            if let Some(k) = g.fa[rf].attachments.iter().position(|&x| x == a) {
                if crate::weapons::rifle_left(g, rf, a) > 0.0 { g.fa[rf].att_idx = k as i32; }
            }
        }
    }
    let wand = (m.wand && wand_row(g, 0.0)) || (lent && crate::quietfog::quiet_wand(g));
    g.pl[pl].wand = wand;
    let c = g.pl[pl].ch;
    if m.suit != SuitId::Issued { g.ch[c].tint = suit_tint(m.suit); }
    if m.trim != TrimId::Issued { g.ch[c].tint2 = trim_tint(m.trim); }
    let worn = if issuing_on(g) { None } else { Some(&m) };
    crate::kitload::load_man(g, g.pl[g.player].ch, 0.0, worn, None);
    kit_alone(g);
}

/// kitPlayer's `mine`: the gun your card picks for this slot.
fn mine(g: &mut Game, w: crate::weapons::WeaponId, m: &MemberKit, slot: crate::gunslots::GunSlot) -> crate::weapons::WeaponDef {
    if let Some(d) = kit_slot_def(g, m, slot) { return d; }
    weapon_row(g, w)
}

/// Nobody came with you: the counts the door tools and the wall charge spend are your card's.
fn kit_alone(g: &mut crate::game::Game) {
    if team_on_floor(g) { return; }
    crate::wallcharge::wall_charge_raise_carried(g, 0.0);
    let n = if charge_row(g, 0.0) { g.m.kit.kit.members[0].charges } else { 0.0 };
    crate::doortools::door_tools_alone(g, n);
    let w = kit_walls(g, 0.0);
    crate::wallcharge::wall_charge_set_carried(g, Some(w));
}

/// The solo night wears your card.
fn kit_solo(g: &mut crate::game::Game) {
    g.solo_card = true;
    crate::seats::each_seat(g, &mut |v| kit_player(v));
}

/// The page's own question: the plain night off the loadout screen, or a quiet mission alone, and no squad scenario up.
pub fn kit_solo_if(g: &mut crate::game::Game, solo: bool) {
    if !g.staged.facts.fight && (solo || (g.staged.facts.unseen && g.staged.team.is_empty())) { kit_solo(g); }
}

/// A mission started lowers the card on every host, and the harness's shipped stand asks the question too.
pub fn kit_solo_stand(g: &mut crate::game::Game, plain: Option<bool>) {
    let plain = plain.unwrap_or(true);
    g.solo_card = false;
    if !g.stood.solo_stand { return; }
    let lesson = g.level.lesson().is_some_and(|l| !l.steps.is_empty());
    if !lesson { kit_solo_if(g, plain); }
}

/// The kit picture is about to be staged on this Player: his card comes off.
pub(in crate::outfit) fn kit_show(g: &mut crate::game::Game) {
    let pl = g.player;
    let Some(was) = g.m.kit.issued_player.get(pl).copied() else { return; };
    g.pl[pl].has_rifle = was.rifle;
    g.pl[pl].rifle = was.gun;
    g.pl[pl].pistol = was.side;
    g.pl[pl].slot = was.slot;
    g.pl[pl].canisters = was.smoke;
    g.pl[pl].flashbangs = was.stun;
    g.pl[pl].frags = was.frags;
    g.fa[was.gun].shells = was.shells;
    g.fa[was.gun].rings = was.rings;
    g.m.kit.you_bare.delete(pl);
    crate::stickycam::sticky_issue(g, pl, m::max(0.0, was.cams), Some(was.cams >= 0.0));
    g.pl[pl].wand = was.wand;
    g.pl[pl].big_kit = was.big;
    let c = g.pl[pl].ch;
    g.ch[c].tint = was.tint;
    g.ch[c].tint2 = was.tint2;
    g.m.kit.issued_player.delete(pl);
    crate::kitload::load_man(g, g.pl[g.player].ch, 0.0, None, None);
}

/// A scenario has just staged its team: each man's card onto him, yours onto you, and the squad's counts moved by what the cards changed.
pub fn kit_stage(g: &mut crate::game::Game, ops: &[GuardId]) {
    if kit_issue(g, &mut |g| kit_stage(g, ops)) { return; }
    if page_issued(g) && !issuing_on(g) { return as_issued(g, &mut |g| kit_stage(g, ops)); }
    if !issuing_on(g) && !g.m.kit.staged { return for_run(g, &mut |g| kit_stage(g, ops)); }
    let came: Vec<GuardId> = ops.iter().take(CARDS - 1).copied().collect();
    g.m.kit.kit_staged.ops = came.clone();
    if !g.m.kit.kit.on {
        crate::seats::each_seat(g, &mut |v| {
            crate::squadsmoke::squad_smoke_stage(v, 0.0);
            crate::squad40::squad40_stage(v, &[]);
        });
        return;
    }
    let n = m::min(ops.len() as f64, KIT_MEMBERS - 1.0);
    kit_team(g, Some(n));
    if g.seats.len() > 1 { crate::seats::deal_seats(g); }
    let cans = kit_smokes(g, ops.len() as f64);
    crate::seats::each_seat(g, &mut |v| {
        let share = crate::seats::seat_share(v, cans);
        crate::squadsmoke::squad_smoke_stage(v, share);
    });
    crate::seats::each_seat(g, &mut |v| kit_player(v));
    let mut worn = kit_worn(g, Some(n), None);
    let worn_is_table = !dealing(g) || n < 3.0;   // (kitWorn handed back KIT.members itself: bigHome below writes through to the table)
    let issued_cards = issued();
    for (k, &gd) in came.iter().enumerate() {
        let i = k + 1;
        let d = &issued_cards[i];
        let m = big_home(&mut worn[i], i as f64);
        if worn_is_table { g.m.kit.kit.members[i] = m.clone(); }
        let words = if i != 0 { kit_card_words(g, i as f64, &m) } else { String::new() };
        g.m.kit.man_of.set(gd, KitModManOfValue { i: i as f64, kit: m.clone(), words });
        let held = if issuing_on(g) { None } else { Some(&m) };
        crate::kitload::load_man(g, g.gd[gd].ch, i as f64, held, Some(n));
        let gun = kit_gun(g, &m);
        let changed = m.weapon != d.weapon || m.suppressed != d.suppressed || m.primary != d.primary;
        if changed && gun != g.fa[g.gd[gd].weapon].def && g.m.weapons.weapons.get(gun).unwrap().enabled {
            let route = g.gd[gd].route;
            g.tab.route[route].weapon = Some(gun);
            let def = weapon_row(g, gun);
            let fresh = crate::weapons::firearm_new(g, &def);
            g.gd[gd].weapon = fresh;
            g.gd[gd].reload_t = -1.0;
            let c = g.gd[gd].ch;
            g.ch[c].muzzle_ahead = def.muzzle_ahead;
        }
        let w = g.fa[g.gd[gd].weapon].def;
        let second = if is_pistol(g, w) { None } else { kit_side(g, &m) };
        crate::gunslots::side_stage(g, gd, second);
        if !m.light { g.gd[gd].light_dead_until = f64::INFINITY; }
        if !m.nvg { g.m.kit.bare_eyed.add(gd); }
        let c = g.gd[gd].ch;
        if m.suit != SuitId::Issued { g.ch[c].tint = suit_tint(m.suit); }
        if m.trim != TrimId::Issued { g.ch[c].tint2 = trim_tint(m.trim); }
    }
    let rows: Vec<(GuardId, f64)> = came.iter().map(|&gd| (gd, g.m.kit.man_of.get(gd).map_or(0.0, |e| e.kit.shells))).collect();
    let one = g.seats.len() == 1;
    crate::seats::each_seat(g, &mut |v| {
        if one {
            crate::squad40::squad40_stage(v, &rows);
        } else {
            let s = v.seat;
            let mine: Vec<(GuardId, f64)> = rows.iter().copied().filter(|r| crate::seats::in_seat(v, s, r.0)).collect();
            crate::squad40::squad40_stage(v, &mine);
        }
    });
    let pool = kit_pool(g, n, None);
    if js::truthy(n) && (pool.bangs != g.k.doortools.bangs || pool.charges != g.k.doortools.charges || pool.frags != g.k.frag.squad) {
        crate::seats::each_seat(g, &mut |v| {
            crate::doortools::door_tools_sync(v);
            v.sets[v.seat].door_tools.bangs = crate::seats::seat_share(v, pool.bangs);
            v.sets[v.seat].door_tools.charges = crate::seats::seat_share(v, pool.charges);
            v.sets[v.seat].door_tools.frags = crate::seats::seat_share(v, pool.frags);
        });
    }
    if js::truthy(n) {
        let walls = kit_walls(g, n);
        crate::seats::each_seat(g, &mut |v| {
            let share = crate::seats::seat_share(v, walls);
            crate::wallcharge::wall_charge_set_carried(v, Some(share));
        });
    }
}

/// The wall charges you and the `n` teammates who come carry between you.
pub(super) fn kit_walls(g: &crate::game::Game, n: f64) -> f64 {
    if !g.m.kit.kit.on { return 0.0; }
    let mut w = 0.0;
    let top = m::min(n, KIT_MEMBERS - 1.0);
    let mut i = 0usize;
    while (i as f64) <= top {
        if wall_row(g, i as f64) { w += g.m.kit.kit.members[i].walls; }
        i += 1;
    }
    w
}

/// The squad's counted stun canisters and charges with `n` teammates coming.
pub(super) fn kit_pool(g: &mut crate::game::Game, n: f64, issued: Option<crate::kit::KitPoolIssued>) -> crate::kit::KitPool {
    let issued = issued.unwrap_or(KitPoolIssued { bangs: g.k.doortools.bangs, charges: g.k.doortools.charges });
    let (mut b, mut c, mut f) = (issued.bangs, issued.charges, g.k.frag.squad);
    let on = g.m.kit.kit.on;
    if on {
        let worn = kit_worn(g, Some(n), None);
        let (cards, frags) = (crate::kit::issued(), issued_frags());
        for i in 1..CARDS {
            if (i as f64) <= n {
                b += worn[i].bangs - cards[i].bangs;
                c += worn[i].charges - cards[i].charges;
                f += worn[i].frags - frags[i];
            } else {
                b -= cards[i].bangs;
                c -= cards[i].charges;
                f -= frags[i];
            }
        }
    }
    if on && charge_row(g, 0.0) { c += g.m.kit.kit.members[0].charges; }
    KitPool { bangs: m::max(0.0, b), charges: m::max(0.0, c), frags: m::max(0.0, f) }
}

/// The smoke canisters the squad brings to a run of `n` teammates.
pub(super) fn kit_smokes(g: &crate::game::Game, n: f64) -> f64 {
    let mut s = 0.0;
    if g.m.kit.kit.on {
        let mut i = 1usize;
        while i < CARDS && (i as f64) <= n {
            s += g.m.kit.kit.members[i].smoke;
            i += 1;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// kitSmokes counts the men who come, never your card; kitBig yields to a wall charge.
    #[test]
    fn counts_and_the_big_slot() {
        let mut g = Box::new(Game::blank());
        for i in 0..CARDS { g.m.kit.kit.members[i].smoke = (i + 1) as f64; }
        assert_eq!(kit_smokes(&mut g, 0.0), 0.0);
        assert_eq!(kit_smokes(&mut g, 2.0), 2.0 + 3.0);
        assert_eq!(kit_smokes(&mut g, 9.0), 2.0 + 3.0 + 4.0 + 5.0);
        g.m.kit.kit.on = false;
        assert_eq!(kit_smokes(&mut g, 4.0), 0.0);

        let gd = GuardId(0);
        let mut kit = g.m.kit.kit.members[1].clone();
        kit.big = BigKit::Hammer;
        g.m.kit.man_of.set(gd, KitModManOfValue { i: 1.0, kit: kit.clone(), words: String::new() });
        assert_eq!(kit_big(&mut g, gd), BigKit::Hammer);
        kit.walls = 1.0;
        g.m.kit.man_of.set(gd, KitModManOfValue { i: 1.0, kit, words: String::new() });
        assert_eq!(kit_big(&mut g, gd), BigKit::None);
    }

    /// A SideMap's "no entry" leaves your light and goggles working; a card that took them off says so.
    #[test]
    fn your_light_and_goggles() {
        let mut g = Box::new(Game::blank());
        let pl = PlayerId(0);
        assert!(kit_you_light(&mut g, pl) && kit_you_nvg(&mut g, pl));
        g.m.kit.you_bare.set(pl, KitModYouBareValue { light: true, nvg: false });
        assert!(!kit_you_light(&mut g, pl) && kit_you_nvg(&mut g, pl));
    }
}
