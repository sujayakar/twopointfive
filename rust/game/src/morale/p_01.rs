//! src/game/morale.ts lines 1 to 452
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

type Pool = &'static [&'static str];
/// one kind's lines (morale.ts LINES); `now` is `say` as the writing guide has it
struct Lines { say: Pool, he: Option<Pool>, they: Option<Pool>, dark: Option<Pool>, lit: Option<Pool>, now: Option<Pool> }
/// (morale.ts LINES) in the order the object is written: moraleLines walks it
static LINES: [(LineKind, Lines); 9] = [
    (LineKind::Duck, Lines { say: &["down! get down!", "get down — stay down!", "heads down!"], he: None, they: None, dark: None, lit: None, now: None }),
    (LineKind::Call, Lines { say: &[], dark: Some(&["can't see a thing — somebody get in here!"]), lit: Some(&["right in my room — somebody get in here!"]), he: Some(&["he's coming in — I've got nobody in here!", "anyone on this net? he's right on top of me!"]),
        they: Some(&["they're coming in — I've got nobody in here!", "anyone on this net? they're right on top of me!"]), now: None }),
    (LineKind::Again, Lines { say: &["is anybody there? answer me!", "nobody's coming. nobody's coming—", "please — somebody answer!"], he: None, they: None, dark: None, lit: None, now: None }),
    (LineKind::Out, Lines { say: &["I'm out! I'm out!", "empty — I'm empty!", "I'm dry! changing!"], he: None, they: None, dark: None, lit: None, now: None }),
    (LineKind::Spent, Lines { say: &["no — no, stay back!", "keep away from me!", "don't — don't come in!"], now: Some(&["no — no, stay back!", "stay away from me!", "don't — don't come in!"]), he: None, they: None, dark: None, lit: None }),
    (LineKind::Clear, Lines { say: &["breach on my room. I'm alone in here."], he: Some(&["he's breached. I'm on my own in here.", "he's through. holding it on my own."]),
        they: Some(&["they've breached. I'm on my own in here.", "they're through. holding it on my own."]), dark: None, lit: None, now: None }),
    (LineKind::Lost, Lines { say: &["they got him! they got him!", "he's down. oh no, he's down.", "we've lost him! what do we do?"], he: None, they: None, dark: None, lit: None, now: None }),
    (LineKind::Order, Lines { say: &["get up. watch that hole. breathe.", "on your feet. eyes on that hole.", "up. gun on the hole. breathe."], he: None, they: None, dark: None, lit: None, now: None }),
    (LineKind::Okay, Lines { say: &["okay — okay.", "yeah. yeah, I'm up.", "I'm up. I'm up."], he: None, they: None, dark: None, lit: None, now: None }),
];
fn lines_of(kind: LineKind) -> &'static Lines {
    let mut i = 0;
    while LINES[i].0 != kind { i += 1; }
    &LINES[i].1
}
/// his row as it stands, a copy to read: a writer writes the row
fn now(g: &mut Game, gd: GuardId) -> Mind { *g.m.morale.state_of.get(gd).unwrap() }

/// this file's lines, every kind's or one kind's (morale.ts moraleLines)
pub fn morale_lines(g: &mut crate::game::Game, kind: Option<crate::morale::LineKind>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (k, l) in LINES.iter() {
        if let Some(want) = kind { if *k != want { continue; } }
        for t in l.say { out.push(String::from(*t)); }
        for t in l.now.unwrap_or(&[]) { if !l.say.contains(t) { out.push(String::from(*t)); } }
        for pool in [l.dark, l.lit, l.he, l.they] {
            for t in pool.unwrap_or(&[]) { out.push(String::from(*t)); }
        }
    }
    out
}

/// one line of a kind from this man, unless he said one inside `barkSecs` (morale.ts bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::morale::LineKind, radio: bool, force: Option<bool>) -> bool {
    let st = now(g, gd);
    if !force.unwrap_or(false) && st.bark < g.k.morale.bark_secs { return false; }
    let l = lines_of(kind);
    let mut pool: Vec<&'static str> = Vec::new();
    pool.extend_from_slice(l.now.unwrap_or(l.say));
    pool.extend_from_slice((if st.dark { l.dark } else { l.lit }).unwrap_or(&[]));
    pool.extend_from_slice((if g.gd[gd].team != crate::teams::Team::Guard { l.they } else { l.he }).unwrap_or(&[]));
    let n = pool.len() as f64;
    let i = js::rem(m::floor(crate::math_dice::sim_random(g) * n), n);
    crate::game::game_say(g, gd, pool[i as usize], Some(radio), None, None);
    g.m.morale.state_of.get_mut(gd).unwrap().bark = 0.0;
    if radio { g.stats.morale.calls += 1; } else { g.stats.morale.shouts += 1; }
    true
}

/// distance on the floor (morale.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// his chest, or where it would be (morale.ts chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    let c = &g.ch[c_2];
    match c.bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => [c.pos[0], c.pos[1] + 1.2, c.pos[2]] }
}

/// his head, or where it would be (morale.ts headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    let c = &g.ch[c_2];
    match c.bones.get(crate::character::Bone::Head) { Some(b) => *b, None => [c.pos[0], c.pos[1] + 1.6, c.pos[2]] }
}

/// a man this file is about at all: a living defender or a man of the night staff (morale.ts ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let m_ = &g.gd[gd];
    m_.state != crate::game::GuardState::Dead && m_.civ.is_none() && (m_.team == crate::teams::Team::Hostile || m_.team == crate::teams::Team::Guard)
}

/// a man of the hostage rescue's cast (morale.ts rescued)
pub fn rescued(g: &mut crate::game::Game, gd: GuardId) -> bool { g.rescue.active && g.rescue.hostiles.contains(&gd) }

/// may a sequence have him this frame (morale.ts reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.ai_enabled || g.touring || g.quiet_utility || g.gd[gd].held.is_some() { return false; }
    g.gd[gd].script.is_none() || g.fight.defs.iter().any(|d| g.tab.roomfight_def[*d].gd == gd)
}

/// his row, made the first time he is met: the row itself, for a few lines (morale.ts mind)
pub fn mind<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::morale::Mind {
    if !g.m.morale.state_of.has(gd) {
        let id = g.ch[g.gd[gd].ch].id;
        let s = Mind { shock: 0.0, alone: false, dark: false, lit: 0.0, ask: js::rem(id, 4.0) / 4.0 * g.k.morale.ask_secs, rung: 0.0, green: false, leader_down: false, fall: -1.0, lost: false, by: None, heard: -1.0, said: false, broke_t: 0.0,
            act: MoraleAct::None, t: 0.0, hole: None, goal: None, rounds: 0.0, next: 0.0, calls: 0.0, call_t: 0.0, baits: 0.0, at_hole: false, bark: g.k.morale.bark_secs, shout: false,
            drive: false, drive_t: -1.0, script: crate::game::GuardScript::zeroed() };
        g.m.morale.state_of.set(gd, s);
        stand(g, gd, Some(true));   // (the row is set first: stand writes it)
    }
    g.m.morale.state_of.get_mut(gd).unwrap()
}

/// is he blind-dark where he stands (morale.ts inTheDark)
pub fn in_the_dark(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.k.morale.dark_read == crate::knobs::MoraleKDarkRead::Watch { return crate::darkfoe::dark_foe_blind(g, gd); }
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let mut own = |_g: &mut Game, owner: f64| owner == id;
    let chest = chest_of(g, c);
    let a = crate::guards::light_estimate_at(g, chest, Some(&mut own), Some(true), None, None);
    let head = head_of(g, c);
    let b = crate::guards::light_estimate_at(g, head, Some(&mut own), Some(true), None, None);
    m::max(a, b) < g.tune.vis_floor
}

/// has he company (morale.ts hasMate)
pub fn has_mate(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let eye = head_of(g, c);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || !ours(g, o) || g.gd[o].team != g.gd[gd].team || g.gd[o].held.is_some() || crate::runner::runner_nobody(g, o) { continue; }
        let oc = g.gd[o].ch;
        if flat(g.ch[oc].pos, g.ch[c].pos) <= g.k.morale.alone_m {
            let chest = chest_of(g, oc);
            if !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { return true; }
        }
    }
    false
}

/// the men he is counted with: his squad's, else his room's (morale.ts bandOf)
pub fn band_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<GuardId>> {
    if let Some(q) = crate::enemysquads::squad_of(g, gd) { return Some(q.men); }
    if g.gd[gd].team != crate::teams::Team::Hostile { return None; }
    g.room_clear.rooms.iter().find(|r| r.defenders.contains(&gd)).map(|r| r.defenders.clone())
}

/// the difficulty's mix, read once a tick (morale.ts mixNow)
pub fn mix_now(g: &mut crate::game::Game) -> crate::morale::MixNow {
    let key = format!("{}|{}|{}", g.time, g.k.pressure.on, g.k.digin.on);
    if let Some(had) = &g.m.morale.mix_at {
        if had.key == key { return if had.mix == MoraleModMixAtValueMix::Hard { MixNow::Hard } else { MixNow::Mixed }; }
    }
    let ai = match crate::difficulty::levers_shown(g, None).ai { Some(d) => d.as_str(), None => if g.k.pressure.on { "hard" } else { "standard" } };
    let hard = crate::morale::morale_at().get(String::from(ai)).map_or(false, |v| v == "hard");
    g.m.morale.mix_at = Some(MoraleModMixAtValue { key, mix: if hard { MoraleModMixAtValueMix::Hard } else { MoraleModMixAtValueMix::Mixed } });
    if hard { MixNow::Hard } else { MixNow::Mixed }
}

/// is he green or steady (morale.ts trainingOf)
pub fn training_of(g: &mut crate::game::Game, gd: GuardId) -> crate::morale::TrainingOf {
    use crate::scene_scenarios::SquadDataTraining as Own;
    let k = g.k.morale.training;
    if g.gd[gd].trained || k == Training::Trained || k == Training::None { return TrainingOf::Steady; }
    if k == Training::Green { return TrainingOf::Green; }
    let own = if k == Training::Mixed { None } else { crate::enemysquads::squad_of(g, gd).and_then(|q| q.def.training) };
    if own == Some(Own::Trained) { return TrainingOf::Steady; }
    if own == Some(Own::Green) { return TrainingOf::Green; }
    let men = match band_of(g, gd) {
        Some(men) => men,
        None => {
            let t = crate::game::temperament().get(g.gd[gd].callsign.clone()).cloned();
            let green = match t {
                Some(t) => t == "nervy",
                None => g.guards.iter().position(|o| *o == gd).map_or(false, |i| i % 2 == 1),
            };
            return if green { TrainingOf::Green } else { TrainingOf::Steady };
        }
    };
    let mixed = k == Training::Mixed || own == Some(Own::Mixed) || mix_now(g) == MixNow::Mixed;
    let i = men.iter().position(|o| *o == gd);
    if mixed {
        if i == Some(0) { TrainingOf::Steady } else { TrainingOf::Green }
    } else if men.len() >= 3 && i == Some(men.len() - 1) { TrainingOf::Green } else { TrainingOf::Steady }
}

/// the first man of his band, where that man is steady (morale.ts leaderOf)
pub fn leader_of(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    if g.k.morale.training == Training::None { return None; }
    let l = band_of(g, gd).and_then(|men| men.first().copied())?;
    if training_of(g, l) == TrainingOf::Steady { Some(l) } else { None }
}

/// the first slice's sequence is a green man's (morale.ts folds)
pub fn folds(g: &mut crate::game::Game, gd: GuardId) -> bool { now(g, gd).green || g.k.morale.training == Training::None }

/// what a shock of `x` is to these men (morale.ts nerved)
pub fn nerved(g: &mut crate::game::Game, x: f64) -> f64 {
    let n = g.k.morale.nerve_k;
    x * match g.k.morale.nerve { Nerve::Brittle => n.brittle, Nerve::Normal => n.normal, Nerve::Steady => n.steady }
}

/// his standing terms read afresh, on the row (morale.ts stand)
pub fn stand(g: &mut crate::game::Game, gd: GuardId, first: Option<bool>) {
    let first = first.unwrap_or(false);
    let alone = !has_mate(g, gd);
    g.m.morale.state_of.get_mut(gd).unwrap().alone = alone;
    let dark = in_the_dark(g, gd);
    let lit_asks = g.k.morale.lit_asks;
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    if dark { r.dark = true; r.lit = 0.0; } else if first || { r.lit += 1.0; r.lit >= lit_asks } { r.dark = false; }
    if r.act == MoraleAct::None {
        let green = training_of(g, gd) == TrainingOf::Green;
        g.m.morale.state_of.get_mut(gd).unwrap().green = green;
    }
    let l = if g.m.morale.state_of.get(gd).unwrap().green { leader_of(g, gd) } else { None };
    let down = match l { Some(l) => g.gd[l].state == crate::game::GuardState::Dead, None => false };
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.leader_down = down;
    if !down { r.lost = false; }
}

/// does he know something is up (morale.ts roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId) -> bool { now(g, gd).shock > 0.0 || g.gd[gd].state != crate::game::GuardState::Patrol }

/// where he stands, as dread (morale.ts standing)
pub fn standing(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if !roused(g, gd) { return 0.0; }
    let s = now(g, gd);
    let k = &g.k.morale;
    (if s.alone { k.alone } else { 0.0 }) + (if s.dark { k.dark } else { 0.0 }) + (if s.leader_down { k.leader_down } else { 0.0 })
}

/// shock and standing, 0 to 1 (morale.ts dread)
pub fn dread(g: &mut crate::game::Game, gd: GuardId) -> f64 { let shock = now(g, gd).shock; v3::clamp(shock + standing(g, gd), 0.0, 1.0) }

/// may he break, whatever his shock (morale.ts mayBreak)
pub fn may_break(s: crate::morale::Mind) -> bool { if s.green { s.alone || (s.leader_down && s.lost) } else { s.alone && s.dark } }

/// the rung his dread reads, with hysteresis (morale.ts rungNow)
pub fn rung_now(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let s = now(g, gd);
    let d = dread(g, gd);
    let k = &g.k.morale;
    let br = if s.green { k.green_broken } else { k.broken };
    if may_break(s) && s.shock > k.hold_shock && (d >= br || (s.rung == 2.0 && d >= br - k.hyst)) { 2.0 }
    else if d >= k.jumpy || (s.rung >= 1.0 && d >= k.jumpy - k.hyst) { 1.0 }
    else { 0.0 }
}

/// a feed for any event: shock on one man (morale.ts moraleShock)
pub fn morale_shock(g: &mut crate::game::Game, gd: GuardId, amount: f64) {
    if !ours(g, gd) || rescued(g, gd) { return; }
    mind(g, gd);
    let n = nerved(g, amount);
    let cap = g.k.morale.shock_cap;
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.shock = m::min(cap, r.shock + n);
}

/// the leader's death: his green men, the ones who saw it, the nearest of those (morale.ts moraleManDown)
pub fn morale_man_down(g: &mut crate::game::Game, victim: GuardId) {
    if !ours(g, victim) { return; }
    let vc = g.gd[victim].ch;
    let at = g.ch[vc].pos;
    let (mut near, mut nd): (Option<GuardId>, f64) = (None, f64::INFINITY);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if gd == victim || !ours(g, gd) || rescued(g, gd) || training_of(g, gd) != TrainingOf::Green || leader_of(g, gd) != Some(victim) { continue; }
        mind(g, gd).leader_down = true;
        if g.time < g.gd[gd].dazzled_until || !crate::guards::can_see_char(g, gd, vc, false).visible { continue; }
        let n = nerved(g, g.k.morale.leader_shock);
        let cap = g.k.morale.shock_cap;
        let r = g.m.morale.state_of.get_mut(gd).unwrap();
        r.shock = m::min(cap, r.shock + n);
        r.fall = 0.0;
        g.stats.morale.falls += 1;
        let st = now(g, gd);
        if (st.act == MoraleAct::None || st.act == MoraleAct::Duck) && reacts(g, gd) {
            begin(g, gd, MoraleAct::Flinch, Some(at));
            g.stats.morale.flinches += 1;
        }
        let d = flat(g.ch[g.gd[gd].ch].pos, at);
        if d < nd { nd = d; near = Some(gd); }
    }
    if let Some(n) = near {
        mind(g, n).lost = true;
        bark(g, n, LineKind::Lost, false, Some(true));
    }
}

/// seconds since he saw his leader go down (morale.ts sawLeaderFall)
pub fn saw_leader_fall(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd) { Some(s) if s.fall >= 0.0 => s.fall, _ => f64::INFINITY }
}

/// a blast went off: only filed here, asked about on the next tick (morale.ts moraleBlast)
pub fn morale_blast(g: &mut crate::game::Game, at: V3, who: Option<f64>) {
    let b = Blast { at: v3::copy(at), t: g.time, who: who.unwrap_or(0.0) };
    g.m.morale.blasts_of.get_or_insert_with(Vec::new).push(b);
    g.stats.morale.blasts += 1;
}

/// a restart: blasts not yet asked about are the old floor's (morale.ts moraleReset)
pub fn morale_reset(g: &mut crate::game::Game, men: Option<bool>) {
    g.m.morale.blasts_of = None;
    if men.unwrap_or(false) {
        let mut i = 0;
        while i < g.guards.len() { let gd = g.guards[i]; i += 1; g.m.morale.state_of.delete(gd); }
    }
}

/// one filed blast, the frame after (morale.ts feed)
pub fn feed(g: &mut crate::game::Game, b: crate::morale::Blast) {
    let from: V3 = [b.at[0], m::max(b.at[1], 0.3), b.at[2]];
    let side = if js::truthy(b.who) { Some(crate::teams::team_of_id(g, b.who)) } else { None };
    let (mut caller, mut cd): (Option<GuardId>, f64) = (None, f64::INFINITY);
    let mut fed: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) || rescued(g, gd) || Some(g.gd[gd].team) == side || !reacts(g, gd) { continue; }
        let c = g.gd[gd].ch;
        let d = flat(g.ch[c].pos, b.at);
        if d > g.k.morale.blast_m { continue; }
        if d > g.k.morale.near_m {
            let chest = chest_of(g, c);
            if crate::scene_collision::segment_blocked(g, from, chest, None, None, None) { continue; }
        }
        mind(g, gd);
        let k = g.k.morale;
        let n = nerved(g, k.blast);
        let r = g.m.morale.state_of.get_mut(gd).unwrap();
        r.shock = m::min(k.shock_cap, r.shock + n * (if d <= k.near_m { 1.0 } else { v3::lerp(1.0, k.far_k, (d - k.near_m) / m::max(1e-3, k.blast_m - k.near_m)) }));
        g.stats.morale.fed += 1;
        let st = now(g, gd);
        let rung = rung_now(g, gd);
        g.m.morale.state_of.get_mut(gd).unwrap().rung = rung;
        fed.push(gd);
        if rung == 2.0 {
            begin(g, gd, MoraleAct::Down, Some(b.at));
            let r = g.m.morale.state_of.get_mut(gd).unwrap();
            r.baits = 0.0; r.at_hole = false; r.broke_t = 0.0;
            g.stats.morale.broke += 1;
        } else if rung == 1.0 && (st.act == MoraleAct::None || st.act == MoraleAct::Duck || st.act == MoraleAct::Stand || st.act == MoraleAct::Flinch) {
            begin(g, gd, MoraleAct::Duck, Some(b.at));
            g.stats.morale.ducks += 1;
        }
    }
    let order_m = g.k.morale.order_m;
    for &l in fed.iter() {
        let sl = now(g, l);
        let ducked = sl.act == MoraleAct::Duck && sl.t == 0.0;
        if sl.rung == 2.0 || !(ducked || sl.act == MoraleAct::None) || leader_of(g, l) != Some(l) { continue; }
        let lc = g.gd[l].ch;
        let mut his: Vec<GuardId> = Vec::new();
        for &o in fed.iter() {
            let so = now(g, o);
            if o == l || so.act != MoraleAct::Duck || so.t != 0.0 { continue; }
            let oc = g.gd[o].ch;
            if flat(g.ch[oc].pos, g.ch[lc].pos) > order_m || leader_of(g, o) != Some(l) { continue; }
            let (head, chest) = (head_of(g, lc), chest_of(g, oc));
            if !crate::scene_collision::segment_blocked(g, head, chest, None, None, None) { his.push(o); }
        }
        if !ducked && his.is_empty() { continue; }
        begin(g, l, MoraleAct::Stand, Some(b.at));
        if ducked { g.stats.morale.ducks -= 1; }
        for &o in his.iter() { g.m.morale.state_of.get_mut(o).unwrap().by = Some(l); }
    }
    for &gd in fed.iter() {
        let st = now(g, gd);
        let d = flat(g.ch[g.gd[gd].ch].pos, b.at);
        if st.act == MoraleAct::Duck && st.t == 0.0 && d < cd { cd = d; caller = Some(gd); }
    }
    if let Some(c) = caller { mind(g, c).shout = true; }
}

/// he starts an act (morale.ts begin)
pub fn begin(g: &mut crate::game::Game, gd: GuardId, act: crate::morale::MoraleAct, hole: Option<V3>) {
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.act = act; r.t = 0.0;
    if let Some(h) = hole { r.hole = Some(v3::copy(h)); }
    r.goal = None; r.rounds = 0.0; r.next = 0.0; r.by = None; r.heard = -1.0; r.said = false;
}

/// the sequence lets him go (morale.ts end)
pub fn end(g: &mut crate::game::Game, gd: GuardId) {
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.act = MoraleAct::None; r.t = 0.0; r.goal = None; r.rounds = 0.0; r.next = 0.0; r.drive = false; r.shout = false; r.by = None; r.heard = -1.0; r.said = false;
}

/// the enemies plainly in his eyes this frame (morale.ts seenBy)
pub fn seen_by(g: &mut crate::game::Game, gd: GuardId, hole: Option<V3>) -> Vec<CharId> {
    let mut out: Vec<CharId> = Vec::new();
    if g.time < g.gd[gd].dazzled_until { return out; }
    let gc = g.gd[gd].ch;
    let eye = head_of(g, gc);
    let ge = g.k.guard_eye;
    let e = hole.map(|h| crate::guards::Eye { pos: eye, look: v3::normalize([h[0] - eye[0], 0.0, h[2] - eye[2]]), range: ge.range, half_deg: ge.half_deg, near_m: ge.near_m, near_deg: ge.near_deg, v_half_deg: None });
    let smoke = g.gd[gd].smoke_trans;
    let team = g.gd[gd].team;
    let p = crate::seats::faced_man(g, gd);   // (the player he has in mind: asked as a side)
    let pc = g.pl[p].ch;
    if crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[p].down && !g.player_invisible {
        let v = crate::guards::can_see(g, gd);
        let (mut visible, mut dist) = (v.visible, v.dist);
        if !visible {
            if let Some(e) = e {
                let seat = crate::seats::faced_view(g, gd);
                let v = crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::guards::eye_contact(g, e, pc, true, false, smoke, None));
                visible = v.visible; dist = v.dist;
            }
        }
        if visible && crate::guards::sight_gain(g, gd, dist) > 0.25 { out.push(pc); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if g.gd[o].state == crate::game::GuardState::Dead || !crate::teams::is_hostile(g, team, ot) { continue; }
        let oc = g.gd[o].ch;
        let v = crate::guards::can_see_char(g, gd, oc, false);
        let (mut visible, mut dist) = (v.visible, v.dist);
        if !visible {
            if let Some(e) = e {
                let v = crate::guards::eye_contact(g, e, oc, false, false, smoke, None);
                visible = v.visible; dist = v.dist;
            }
        }
        if visible && crate::guards::sight_gain_on(g, gd, o, dist) > 0.25 { out.push(oc); }
    }
    out
}

/// a thing somebody else threw, on the move by the hole, in his view (morale.ts thrown)
pub fn thrown(g: &mut crate::game::Game, gd: GuardId, hole: V3) -> bool {
    let c = g.gd[gd].ch;
    let eye = head_of(g, c);
    let me = g.ch[c].id;
    let bait_m = g.k.morale.bait_m;
    let mut i = 0;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        let pos = crate::throwables::item_pos(g, it);
        if crate::throwables::item_settled(g, it) || crate::throwables::item_who(g, it) == me || flat(pos, hole) > bait_m { continue; }
        let vel = crate::throwables::item_vel(g, it);
        if m::hyp3(vel[0], vel[1], vel[2]) > 0.5 && !crate::scene_collision::segment_blocked(g, eye, pos, None, None, None) { return true; }
    }
    false
}

/// how far he can go from where he stands along (dx, dz) (morale.ts clearRun)
pub fn clear_run(g: &mut crate::game::Game, gd: GuardId, dx: f64, dz: f64, max: f64) -> f64 {
    let c = g.gd[gd].ch;
    let (p, radius) = (g.ch[c].pos, g.ch[c].radius);
    let prop = match crate::props::prop_system_raycast(g, [p[0], 0.45, p[2]], [dx, 0.0, dz], max + radius) { Some(h) => h.t, None => f64::INFINITY };
    let mut far = 0.0;
    let mut s = 0.25;
    while s <= max + 1e-6 {
        let (x, z, r) = (p[0] + dx * s, p[2] + dz * s, s + radius);
        if r > prop || crate::scene_collision::nav_grid_is_blocked(g, x, z) { break; }
        if crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [p[0] + dx * r, 0.45, p[2] + dz * r], None) || crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [p[0] + dx * r, 1.0, p[2] + dz * r], None) { break; }
        let mut blocked = false;
        for &o in g.guards.iter() {
            if o != gd && g.gd[o].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[o].ch].pos, [x, 0.0, z]) < 0.6 { blocked = true; break; }
        }
        if blocked { break; }
        far = s;
        s += 0.25;
    }
    far
}

/// the way off the hole from where he stands, a unit vector on the floor (morale.ts awayFrom)
pub fn away_from(g: &mut crate::game::Game, c_2: CharId, hole: V3) -> [f64; 2] {
    let p = g.ch[c_2].pos;
    let (dx, dz) = (p[0] - hole[0], p[2] - hole[2]);
    let l = m::hyp2(dx, dz);
    if l > 0.05 { [dx / l, dz / l] } else { let y = g.ch[c_2].body_yaw; [-m::sin(y), -m::cos(y)] }
}

/// where the scrabble ends: straight back, else the longest run of a small fan (morale.ts backGoal)
pub fn back_goal(g: &mut crate::game::Game, gd: GuardId, hole: V3) -> Option<V3> {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let [dx, dz] = away_from(g, c, hole);
    let k = g.k.morale;
    let mut far = clear_run(g, gd, dx, dz, k.back_m);
    let (mut bx, mut bz) = (dx, dz);
    if far < k.straight_m {
        let mut a = k.fan_step;
        while a <= k.fan_deg + 1e-6 && k.fan_step > 0.0 {
            for sgn in [1.0, -1.0] {
                let r = sgn * a * m::PI / 180.0;
                let x = dx * m::cos(r) + dz * m::sin(r);
                let z = dz * m::cos(r) - dx * m::sin(r);
                let run = clear_run(g, gd, x, z, k.back_m);
                if run > far + 1e-6 { far = run; bx = x; bz = z; }
            }
            a += k.fan_step;
        }
    }
    if far >= 0.5 { Some([p[0] + bx * far, 0.0, p[2] + bz * far]) } else { None }
}

/// the leader's one side step off the hole's line (morale.ts sideGoal)
pub fn side_goal(g: &mut crate::game::Game, gd: GuardId, hole: V3, to: Option<GuardId>) -> Option<V3> {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let [dx, dz] = away_from(g, c, hole);
    let side_m = g.k.morale.side_m;
    for sgn in [1.0, -1.0] {
        let (x, z) = (dz * sgn, -dx * sgn);
        if clear_run(g, gd, x, z, side_m) < side_m - 1e-6 { continue; }
        let at: V3 = [p[0] + x * side_m, 0.0, p[2] + z * side_m];
        match to {
            None => return Some(at),
            Some(t) => {
                let chest = chest_of(g, g.gd[t].ch);
                if !crate::scene_collision::segment_blocked(g, [at[0], 1.6, at[2]], chest, None, None, None) { return Some(at); }
            }
        }
    }
    None
}

/// one turn of a burst at the hole; false = the burst is over (morale.ts round)
pub fn round(g: &mut crate::game::Game, gd: GuardId, dt: f64, every: f64, of: crate::morale::RoundOf) -> bool {
    let c = g.gd[gd].ch;
    if g.gd[gd].reload_t >= 0.0 { return false; }
    let hole = g.m.morale.state_of.get(gd).unwrap().hole.unwrap();
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.next -= dt;
    if r.next > 0.0 { return true; }
    let at: V3 = [hole[0], 1.1, hole[2]];
    if g.gd[gd].muzzle_down || m::abs(v3::wrap_angle(v3::yaw_to(g.ch[c].pos, hole) - crate::character::character_gun_yaw(g, c))) > 0.5 { return true; }
    let w = g.gd[gd].weapon;
    if !crate::weapons::firearm_fire(g, w) { return false; }
    let spread = g.k.morale.spread;
    crate::combat::fire_weapon(g, c, at, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(spread), blind: Some(true), ..crate::combat::FireOpts::zeroed() }));
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.rounds -= 1.0;
    r.next = every;
    let left = r.rounds;
    g.stats.morale.rounds += 1;
    let item = MoraleModMORALELOGItem { id: g.ch[c].id, t: g.time, from: v3::copy(g.ch[c].muzzle), at, of: match of { RoundOf::Dust => MoraleModMORALELOGItemOf::Dust, RoundOf::Bait => MoraleModMORALELOGItemOf::Bait } };
    g.m.morale.morale_log.push(item);
    if g.m.morale.morale_log.len() > 96 { g.m.morale.morale_log.remove(0); }
    left > 0.0
}

/// the long burst is spent: the guards' own magazine change, stretched (morale.ts runDry)
pub fn run_dry(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    begin(g, gd, MoraleAct::Reload, None);
    let w = g.gd[gd].weapon;
    let row = if g.gd[gd].reload_t < 0.0 { crate::gunreload::feed_begin(g, w) } else { crate::weapons::firearm_reload_row(g, w) };
    if g.gd[gd].reload_t < 0.0 {
        g.fa[w].burst = None;
        crate::character::anim_reload(g, c, None, None);
        let pos = g.ch[c].pos;
        crate::audio_iface::game_audio_like_play(g, "magOut", Some(pos), Some(0.5), None);
    }
    g.gd[gd].reload_t = m::max_n(&[g.gd[gd].reload_t, row.dur, g.k.morale.reload_secs]);
    bark(g, gd, LineKind::Out, false, Some(true));
    g.stats.morale.reloads += 1;
}

/// a bait taken: the magazine, then a short burst, then a word (morale.ts bait)
pub fn bait(g: &mut crate::game::Game, gd: GuardId) {
    g.m.morale.state_of.get_mut(gd).unwrap().baits += 1.0;
    g.stats.morale.baits += 1;
    let st = now(g, gd);
    let bait_short = g.k.morale.bait_short;
    let w = g.gd[gd].weapon;
    if !folds(g, gd) {
        let n = m::min(bait_short, crate::weapons::firearm_rounds_ready(g, w) - 1.0);
        if st.baits <= 2.0 && n > 0.0 {
            begin(g, gd, MoraleAct::Bait, None);
            g.m.morale.state_of.get_mut(gd).unwrap().rounds = n;
        }
        return;
    }
    if st.baits == 1.0 {
        begin(g, gd, MoraleAct::Bait, None);
        let cap = g.m.weapons.weapons.get(g.fa[w].def).unwrap().mag_capacity;
        g.m.morale.state_of.get_mut(gd).unwrap().rounds = cap + 1.0;
    } else if st.baits == 2.0 {
        begin(g, gd, MoraleAct::Bait, None);
        g.m.morale.state_of.get_mut(gd).unwrap().rounds = bait_short;
    } else if st.baits == 3.0 {
        bark(g, gd, LineKind::Spent, false, Some(true));
    }
}

/// what this file's script holds him to this frame, on the row (morale.ts pose)
pub fn pose(g: &mut crate::game::Game, gd: GuardId, rooted: bool) {
    let st = now(g, gd);
    let hole = st.hole.unwrap();
    let low = rooted || st.act == MoraleAct::Duck || st.act == MoraleAct::Flinch || (st.act == MoraleAct::Down && folds(g, gd)) || st.act == MoraleAct::Reload;
    let walks = st.act == MoraleAct::Back || st.act == MoraleAct::Stand;
    let face = v3::yaw_to(g.ch[g.gd[gd].ch].pos, hole);
    let speed = g.k.morale.back_speed;
    let sc = &mut g.m.morale.state_of.get_mut(gd).unwrap().script;
    sc.goal = if walks && !rooted { st.goal } else { None };
    sc.speed = Some(speed);
    sc.strafe = Some(walks);
    sc.face = Some(face);
    sc.aim_at = if low { None } else { Some([hole[0], 1.1, hole[2]]) };
    sc.upper = Some(if low { crate::character::UpperMode::None } else { crate::character::UpperMode::Aim });
    sc.stance = Some(if low { crate::character::Stance::LowReady } else { crate::character::Stance::None });
    sc.crouch = Some(st.act != MoraleAct::Stand);
    sc.over = None;
}

/// once a frame: decay, standing, the blasts filed, the rungs, each sequence's step (morale.ts moraleTick)
pub fn morale_tick(g: &mut crate::game::Game, dt: f64) {
    let (half_life, ask_secs) = (g.k.morale.half_life, g.k.morale.ask_secs);
    let k = m::pow(2.0, -dt / half_life);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) || rescued(g, gd) { g.m.morale.state_of.delete(gd); continue; }
        let r = mind(g, gd);
        r.drive = false;
        r.shock *= k;
        if r.shock < 0.01 { r.shock = 0.0; }
        r.bark = m::min(r.bark + dt, 60.0);
        if r.fall >= 0.0 { r.fall += dt; }
        r.ask += dt;
        if r.ask >= ask_secs {
            r.ask = 0.0;
            stand(g, gd, None);
        }
    }
    let time = g.time;
    let due: Vec<Blast> = g.m.morale.blasts_of.iter().flatten().filter(|b| time > b.t).copied().collect();
    if !due.is_empty() {
        let keep: Vec<Blast> = g.m.morale.blasts_of.iter().flatten().filter(|b| time <= b.t).copied().collect();
        g.m.morale.blasts_of = Some(keep);
        for b in due { feed(g, b); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !g.m.morale.state_of.has(gd) || !ours(g, gd) { continue; }
        let rung = rung_now(g, gd);
        let r = g.m.morale.state_of.get_mut(gd).unwrap();
        r.rung = rung;
        if r.act != MoraleAct::None { step(g, gd, dt); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own truth: a steady man breaks alone AND dark; a green man alone, or lost with his leader down
    #[test]
    fn who_may_break() {
        let m_ = |green, alone, dark, leader_down, lost| Mind { green, alone, dark, leader_down, lost, ..Mind::zeroed() };
        assert!(may_break(m_(false, true, true, false, false)));
        assert!(!may_break(m_(false, true, false, false, false)));
        assert!(!may_break(m_(false, false, true, true, true)));
        assert!(may_break(m_(true, true, false, false, false)));
        assert!(may_break(m_(true, false, false, true, true)));
        assert!(!may_break(m_(true, false, true, true, false)));
    }
    /// the TypeScript's own bits (bun, EXACT on)
    #[test]
    fn flat_is_the_typescripts() {
        assert_eq!(flat([1.1, 0.0, 2.2], [3.3, 0.0, -4.4]).to_bits(), 0x401bd3faa7236476);
        assert_eq!(flat([0.1, 0.0, -9.3], [7.77, 0.0, 0.004]).to_bits(), 0x40281da70b81d837);
    }
    /// every kind has a table row, so lines_of cannot run off the end
    #[test]
    fn every_kind_has_lines() {
        for k in [LineKind::Duck, LineKind::Call, LineKind::Again, LineKind::Out, LineKind::Spent, LineKind::Clear, LineKind::Lost, LineKind::Order, LineKind::Okay] {
            assert_eq!(lines_of(k).say.len() == 0, k == LineKind::Call);
        }
    }
}
