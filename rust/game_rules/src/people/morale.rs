#![allow(unused_imports, unused_variables)]
pub use ::game_types::morale::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

// None back: the run ended the script, nothing to put home.
pub(super) fn morale_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(s)) = (ran, g.m.morale.state_of.get_mut(gd)) { s.script = ran; }
}

type Pool = &'static [&'static str];
/// one kind's lines
struct Lines { say: Pool, he: Option<Pool>, they: Option<Pool>, dark: Option<Pool>, lit: Option<Pool> }
///  in the order the object is written: moraleLines walks it
static LINES: [(LineKind, Lines); 9] = [
    (LineKind::Duck, Lines { say: &["down! get down!", "get down — stay down!", "heads down!"], he: None, they: None, dark: None, lit: None }),
    (LineKind::Call, Lines { say: &[], dark: Some(&["can't see a thing — somebody get in here!"]), lit: Some(&["right in my room — somebody get in here!"]), he: Some(&["he's coming in — I've got nobody in here!", "anyone on this net? he's right on top of me!"]),
        they: Some(&["they're coming in — I've got nobody in here!", "anyone on this net? they're right on top of me!"]) }),
    (LineKind::Again, Lines { say: &["is anybody there? answer me!", "nobody's coming. nobody's coming—", "please — somebody answer!"], he: None, they: None, dark: None, lit: None }),
    (LineKind::Out, Lines { say: &["I'm out! I'm out!", "empty — I'm empty!", "I'm dry! changing!"], he: None, they: None, dark: None, lit: None }),
    (LineKind::Spent, Lines { say: &["no — no, stay back!", "stay away from me!", "don't — don't come in!"], he: None, they: None, dark: None, lit: None }),
    (LineKind::Clear, Lines { say: &["breach on my room. I'm alone in here."], he: Some(&["he's breached. I'm on my own in here.", "he's through. holding it on my own."]),
        they: Some(&["they've breached. I'm on my own in here.", "they're through. holding it on my own."]), dark: None, lit: None }),
    (LineKind::Lost, Lines { say: &["they got him! they got him!", "he's down. oh no, he's down.", "we've lost him! what do we do?"], he: None, they: None, dark: None, lit: None }),
    (LineKind::Order, Lines { say: &["get up. watch that hole. breathe.", "on your feet. eyes on that hole.", "up. gun on the hole. breathe."], he: None, they: None, dark: None, lit: None }),
    (LineKind::Okay, Lines { say: &["okay — okay.", "yeah. yeah, I'm up.", "I'm up. I'm up."], he: None, they: None, dark: None, lit: None }),
];
fn lines_of(kind: LineKind) -> &'static Lines {
    let mut i = 0;
    while LINES[i].0 != kind { i += 1; }
    &LINES[i].1
}
/// his row as it stands, a copy to read: a writer writes the row
fn now(g: &Game, gd: GuardId) -> Mind { *g.m.morale.state_of.get(gd).unwrap() }

/// one line of a kind from this man, unless he said one inside `barkSecs`
fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::morale::LineKind, radio: bool, force: Option<bool>) -> bool {
    let st = now(g, gd);
    if !force.unwrap_or(false) && st.bark < g.k.morale.bark_secs { return false; }
    let l = lines_of(kind);
    let mut pool: Vec<&'static str> = Vec::new();
    pool.extend_from_slice(l.say);
    pool.extend_from_slice((if st.dark { l.dark } else { l.lit }).unwrap_or(&[]));
    pool.extend_from_slice((if g.team(gd) != crate::teams::Team::Guard { l.they } else { l.he }).unwrap_or(&[]));
    let n = pool.len() as f64;
    let i = js::rem(m::floor(crate::dice::draw_of(g, "morale::bark#1", gd) * n), n);
    crate::game::game_say(g, gd, pool[i as usize], Some(radio), None);
    g.m.morale.state_of.get_mut(gd).unwrap().bark = 0.0;
    true
}

/// a man this file is about at all: a living defender or a man of the night staff
pub(super) fn ours(g: &crate::game::Game, gd: GuardId) -> bool {
    let m_ = &g.gd[gd];
    !g.cond[m_.ch].down && g.ch[m_.ch].team.theirs()
}

/// may a sequence have him this frame
pub(super) fn reacts(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.ai_enabled || g.gd[gd].held.is_some() { return false; }
    g.gd[gd].script.is_none() || g.fight.defs.iter().any(|d| g.tab.roomfight_def[*d].gd == gd)
}

/// his row, made the first time he is met: the row itself, for a few lines
fn mind<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::morale::Mind {
    if !g.m.morale.state_of.has(gd) {
        let id = g.char_id(gd);
        let s = Mind { shock: 0.0, alone: false, dark: false, lit: 0.0, ask: js::rem(id, 4.0) / 4.0 * g.k.morale.ask_secs, rung: 0.0, green: false, leader_down: false, fall: -1.0, lost: false, by: None, heard: -1.0, said: false, broke_t: 0.0,
            act: MoraleAct::None, t: 0.0, hole: None, goal: None, rounds: 0.0, next: 0.0, calls: 0.0, call_t: 0.0, baits: 0.0, at_hole: false, bark: g.k.morale.bark_secs, shout: false,
            drive: false, drive_t: -1.0, script: crate::game::GuardScript::default() };
        g.m.morale.state_of.set(gd, s);
        stand(g, gd, Some(true));   // (the row is set first: stand writes it)
    }
    g.m.morale.state_of.get_mut(gd).unwrap()
}

/// is he blind-dark where he stands
fn in_the_dark(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.k.morale.dark_read == crate::knobs::MoraleKDarkRead::Watch { return crate::darkfoe::dark_foe_blind(g, gd); }
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let mut own = |_g: &Game, owner: f64| owner == id;
    let chest = g.ch[c].bone_or(crate::character::Bone::Chest, 1.2);
    let a = crate::guards::light_estimate_at(g, chest, Some(&mut own), Some(true), None);
    let head = g.ch[c].bone_or(crate::character::Bone::Head, 1.6);
    let b = crate::guards::light_estimate_at(g, head, Some(&mut own), Some(true), None);
    m::max(a, b) < g.tune.vis_floor
}

/// has he company
pub fn has_mate(g: &crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.6);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || !ours(g, o) || g.team(o) != g.team(gd) || g.gd[o].held.is_some() || crate::runner::runner_nobody(g, o) { continue; }
        let oc = g.gd[o].ch;
        if dist_xz(g.ch[oc].pos, g.ch[c].pos) <= g.k.morale.alone_m {
            let chest = g.ch[oc].bone_or(crate::character::Bone::Chest, 1.2);
            if !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { return true; }
        }
    }
    false
}

/// the men he is counted with: his squad's, else his room's
fn band_of(g: &crate::game::Game, gd: GuardId) -> Option<Vec<GuardId>> {
    if let Some(q) = crate::enemysquads::squad_of(g, gd) { return Some(q.men); }
    if g.team(gd) != crate::teams::Team::Hostile { return None; }
    g.staged.rooms.iter().find(|r| r.defenders.contains(&gd)).map(|r| r.defenders.clone())
}

/// the difficulty's mix, read once a tick
fn mix_now(g: &mut crate::game::Game) -> crate::morale::MixNow {
    let key = format!("{}|{}|{}", g.time, g.k.pressure.on, g.k.digin.on);
    if let Some(had) = &g.m.morale.mix_at {
        if had.key == key { return if had.mix == crate::morale::MixNow::Hard { MixNow::Hard } else { MixNow::Mixed }; }
    }
    let ai = match crate::difficulty::levers_shown(g, None).ai { Some(d) => d.as_str(), None => if g.k.pressure.on { "hard" } else { "standard" } };
    let hard = crate::morale::morale_at().of(ai).map_or(false, |v| v == "hard");
    g.m.morale.mix_at = Some(MoraleModMixAtValue { key, mix: if hard { crate::morale::MixNow::Hard } else { crate::morale::MixNow::Mixed } });
    if hard { MixNow::Hard } else { MixNow::Mixed }
}

/// is he green or steady
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
            let t = crate::game::temperament().iter().find(|(name, _)| *name == g.gd[gd].callsign.as_str()).map(|(_, t)| *t);
            let green = match t {
                Some(t) => t == crate::game::Temperament::Nervy,
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

/// the first man of his band, where that man is steady
fn leader_of(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    if g.k.morale.training == Training::None { return None; }
    let l = band_of(g, gd).and_then(|men| men.first().copied())?;
    if training_of(g, l) == TrainingOf::Steady { Some(l) } else { None }
}

/// the first slice's sequence is a green man's
fn folds(g: &crate::game::Game, gd: GuardId) -> bool { now(g, gd).green || g.k.morale.training == Training::None }

/// what a shock of `x` is to these men
fn nerved(g: &crate::game::Game, x: f64) -> f64 {
    let n = g.k.morale.nerve_k;
    x * match g.k.morale.nerve { Nerve::Brittle => n.brittle, Nerve::Normal => n.normal, Nerve::Steady => n.steady }
}

/// his standing terms read afresh, on the row
fn stand(g: &mut crate::game::Game, gd: GuardId, first: Option<bool>) {
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
    let down = match l { Some(l) => crate::condition::down(g, g.gd[l].ch), None => false };
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.leader_down = down;
    if !down { r.lost = false; }
}

/// does he know something is up
fn roused(g: &crate::game::Game, gd: GuardId) -> bool { now(g, gd).shock > 0.0 || g.gd[gd].state != crate::game::GuardState::Patrol }

/// where he stands, as dread
fn standing(g: &crate::game::Game, gd: GuardId) -> f64 {
    if !roused(g, gd) { return 0.0; }
    let s = now(g, gd);
    let k = &g.k.morale;
    (if s.alone { k.alone } else { 0.0 }) + (if s.dark { k.dark } else { 0.0 }) + (if s.leader_down { k.leader_down } else { 0.0 })
}

/// shock and standing, 0 to 1
fn dread(g: &crate::game::Game, gd: GuardId) -> f64 { let shock = now(g, gd).shock; v3::clamp(shock + standing(g, gd), 0.0, 1.0) }

/// may he break, whatever his shock
fn may_break(s: crate::morale::Mind) -> bool { if s.green { s.alone || (s.leader_down && s.lost) } else { s.alone && s.dark } }

/// the rung his dread reads, with hysteresis
pub fn rung_now(g: &crate::game::Game, gd: GuardId) -> f64 {
    let s = now(g, gd);
    let d = dread(g, gd);
    let k = &g.k.morale;
    let br = if s.green { k.green_broken } else { k.broken };
    if may_break(s) && s.shock > k.hold_shock && (d >= br || (s.rung == 2.0 && d >= br - k.hyst)) { 2.0 }
    else if d >= k.jumpy || (s.rung >= 1.0 && d >= k.jumpy - k.hyst) { 1.0 }
    else { 0.0 }
}

/// a feed for any event: shock on one man
pub fn morale_shock(g: &mut crate::game::Game, gd: GuardId, amount: f64) {
    if !ours(g, gd) || crate::mission::leads(g, gd) { return; }
    mind(g, gd);
    let n = nerved(g, amount);
    let cap = g.k.morale.shock_cap;
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.shock = m::min(cap, r.shock + n);
}

/// the leader's death: his green men, the ones who saw it, the nearest of those
pub fn morale_man_down(g: &mut crate::game::Game, victim: GuardId) {
    if !ours(g, victim) { return; }
    let vc = g.gd[victim].ch;
    let at = g.ch[vc].pos;
    let (mut near, mut nd): (Option<GuardId>, f64) = (None, f64::INFINITY);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if gd == victim || !ours(g, gd) || crate::mission::leads(g, gd) || training_of(g, gd) != TrainingOf::Green || leader_of(g, gd) != Some(victim) { continue; }
        mind(g, gd).leader_down = true;
        if crate::control::man::blind(g, gd) || !crate::guards::can_see_char(g, gd, vc).visible { continue; }
        let n = nerved(g, g.k.morale.leader_shock);
        let cap = g.k.morale.shock_cap;
        let r = g.m.morale.state_of.get_mut(gd).unwrap();
        r.shock = m::min(cap, r.shock + n);
        r.fall = 0.0;
        g.stats.morale.falls += 1;
        let st = now(g, gd);
        if (st.act == MoraleAct::None || st.act == MoraleAct::Duck) && reacts(g, gd) {
            begin(g, gd, MoraleAct::Flinch, Some(at));
        }
        let d = dist_xz(g.pos_of(gd), at);
        if d < nd { nd = d; near = Some(gd); }
    }
    if let Some(n) = near {
        mind(g, n).lost = true;
        bark(g, n, LineKind::Lost, false, Some(true));
    }
}

/// seconds since he saw his leader go down
pub fn saw_leader_fall(g: &crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd) { Some(s) if s.fall >= 0.0 => s.fall, _ => f64::INFINITY }
}

/// a blast went off: only filed here, asked about on the next tick
pub fn morale_blast(g: &mut crate::game::Game, at: V3, who: Option<f64>) {
    let b = Blast { at: v3::copy(at), t: g.time, who: who.unwrap_or(0.0) };
    g.m.morale.blasts_of.get_or_insert_with(Vec::new).push(b);
}

/// a restart: blasts not yet asked about are the old floor's
pub fn morale_reset(g: &mut crate::game::Game, men: Option<bool>) {
    g.m.morale.blasts_of = None;
    if men.unwrap_or(false) {
        let mut i = 0;
        let rule = crate::owner::by::MORALE;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            g.m.morale.state_of.delete(gd);
            let t = g.time;
            crate::owner::release(g, gd, rule, t);
        }
    }
}

/// one filed blast, the frame after
fn feed(g: &mut crate::game::Game, b: crate::morale::Blast) {
    let from: V3 = [b.at[0], m::max(b.at[1], 0.3), b.at[2]];
    let side = if js::truthy(b.who) { Some(crate::teams::team_of_id(g, b.who)) } else { None };
    let (mut caller, mut cd): (Option<GuardId>, f64) = (None, f64::INFINITY);
    let mut fed: Vec<GuardId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) || crate::mission::leads(g, gd) || Some(g.team(gd)) == side || !reacts(g, gd) { continue; }
        let c = g.gd[gd].ch;
        let d = dist_xz(g.ch[c].pos, b.at);
        if d > g.k.morale.blast_m { continue; }
        if d > g.k.morale.near_m {
            let chest = g.ch[c].bone_or(crate::character::Bone::Chest, 1.2);
            if crate::scene_collision::segment_blocked(g, from, chest, None, None, None) { continue; }
        }
        mind(g, gd);
        let k = g.k.morale;
        let n = nerved(g, k.blast);
        let r = g.m.morale.state_of.get_mut(gd).unwrap();
        r.shock = m::min(k.shock_cap, r.shock + n * (if d <= k.near_m { 1.0 } else { v3::lerp(1.0, k.far_k, (d - k.near_m) / m::max(1e-3, k.blast_m - k.near_m)) }));
        let st = now(g, gd);
        let rung = rung_now(g, gd);
        g.m.morale.state_of.get_mut(gd).unwrap().rung = rung;
        fed.push(gd);
        if rung == 2.0 {
            begin(g, gd, MoraleAct::Down, Some(b.at));
            let r = g.m.morale.state_of.get_mut(gd).unwrap();
            r.baits = 0.0; r.at_hole = false; r.broke_t = 0.0;
        } else if rung == 1.0 && (st.act == MoraleAct::None || st.act == MoraleAct::Duck || st.act == MoraleAct::Stand || st.act == MoraleAct::Flinch) {
            begin(g, gd, MoraleAct::Duck, Some(b.at));
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
            if dist_xz(g.ch[oc].pos, g.ch[lc].pos) > order_m || leader_of(g, o) != Some(l) { continue; }
            let (head, chest) = (g.ch[lc].bone_or(crate::character::Bone::Head, 1.6), g.ch[oc].bone_or(crate::character::Bone::Chest, 1.2));
            if !crate::scene_collision::segment_blocked(g, head, chest, None, None, None) { his.push(o); }
        }
        if !ducked && his.is_empty() { continue; }
        begin(g, l, MoraleAct::Stand, Some(b.at));
        for &o in his.iter() { g.m.morale.state_of.get_mut(o).unwrap().by = Some(l); }
    }
    for &gd in fed.iter() {
        let st = now(g, gd);
        let d = dist_xz(g.pos_of(gd), b.at);
        if st.act == MoraleAct::Duck && st.t == 0.0 && d < cd { cd = d; caller = Some(gd); }
    }
    if let Some(c) = caller { mind(g, c).shout = true; }
}

/// he starts an act
fn begin(g: &mut crate::game::Game, gd: GuardId, act: crate::morale::MoraleAct, hole: Option<V3>) {
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.act = act; r.t = 0.0;
    if let Some(h) = hole { r.hole = Some(v3::copy(h)); }
    r.goal = None; r.rounds = 0.0; r.next = 0.0; r.by = None; r.heard = -1.0; r.said = false;
}

/// the sequence lets him go
fn end(g: &mut crate::game::Game, gd: GuardId) {
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.act = MoraleAct::None; r.t = 0.0; r.goal = None; r.rounds = 0.0; r.next = 0.0; r.drive = false; r.shout = false; r.by = None; r.heard = -1.0; r.said = false;
}

/// the enemies plainly in his eyes this frame
fn seen_by(g: &mut crate::game::Game, gd: GuardId, hole: Option<V3>) -> Vec<CharId> {
    let mut out: Vec<CharId> = Vec::new();
    if crate::control::man::blind(g, gd) { return out; }
    let gc = g.gd[gd].ch;
    let eye = g.ch[gc].bone_or(crate::character::Bone::Head, 1.6);
    let ge = g.k.guard_eye;
    let e = hole.map(|h| crate::guards::Eye { pos: eye, look: v3::normalize([h[0] - eye[0], 0.0, h[2] - eye[2]]), range: ge.range, half_deg: ge.half_deg, near_m: ge.near_m, near_deg: ge.near_deg, v_half_deg: None });
    let smoke = g.gd[gd].smoke_trans;
    let team = g.team(gd);
    let p = crate::seats::faced_man(g, gd);   // (the player he has in mind: asked as a side)
    let pc = g.pl[p].ch;
    if team.theirs() && !crate::condition::down(g, g.pl[p].ch) && !g.player_invisible {
        let v = crate::guards::can_see(g, gd);
        let (mut visible, mut dist) = (v.visible, v.dist);
        if !visible {
            if let Some(e) = e {
                let seat = crate::seats::faced_view(g, gd);
                let v = crate::seats::with_seat(g, seat, &mut |g: &mut Game| crate::guards::eye_contact(g, e, pc, false, smoke, None));
                visible = v.visible; dist = v.dist;
            }
        }
        if visible && crate::guards::sight_gain(g, gd, dist) > 0.25 { out.push(pc); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[o].ch) || !team.foe_of(g.team(o)) { continue; }
        let oc = g.gd[o].ch;
        let v = crate::guards::can_see_char(g, gd, oc);
        let (mut visible, mut dist) = (v.visible, v.dist);
        if !visible {
            if let Some(e) = e {
                let v = crate::guards::eye_contact(g, e, oc, false, smoke, None);
                visible = v.visible; dist = v.dist;
            }
        }
        if visible && crate::guards::sight_gain_on(g, gd, o, dist) > 0.25 { out.push(oc); }
    }
    out
}

/// a thing somebody else threw, on the move by the hole, in his view
fn thrown(g: &crate::game::Game, gd: GuardId, hole: V3) -> bool {
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.6);
    let me = g.ch[c].id;
    let bait_m = g.k.morale.bait_m;
    let mut i = 0;
    while i < g.items.items.len() {
        let it = g.items.items[i];
        i += 1;
        let pos = g.tab.item[it].pos;
        if g.tab.item[it].settled || g.tab.item[it].who == me || dist_xz(pos, hole) > bait_m { continue; }
        let vel = g.tab.item[it].vel;
        if m::hyp3(vel[0], vel[1], vel[2]) > 0.5 && !crate::scene_collision::segment_blocked(g, eye, pos, None, None, None) { return true; }
    }
    false
}

/// how far he can go from where he stands along (dx, dz)
fn clear_run(g: &crate::game::Game, gd: GuardId, dx: f64, dz: f64, max: f64) -> f64 {
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
            if o != gd && !crate::condition::down(g, g.gd[o].ch) && dist_xz(g.pos_of(o), [x, 0.0, z]) < 0.6 { blocked = true; break; }
        }
        if blocked { break; }
        far = s;
        s += 0.25;
    }
    far
}

/// the way off the hole from where he stands, a unit vector on the floor
fn away_from(g: &crate::game::Game, c_2: CharId, hole: V3) -> [f64; 2] {
    let p = g.ch[c_2].pos;
    let (dx, dz) = (p[0] - hole[0], p[2] - hole[2]);
    let l = m::hyp2(dx, dz);
    if l > 0.05 { [dx / l, dz / l] } else { let y = g.ch[c_2].body_yaw; [-m::sin(y), -m::cos(y)] }
}

/// where the scrabble ends: straight back, else the longest run of a small fan
fn back_goal(g: &crate::game::Game, gd: GuardId, hole: V3) -> Option<V3> {
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

/// the leader's one side step off the hole's line
fn side_goal(g: &crate::game::Game, gd: GuardId, hole: V3, to: Option<GuardId>) -> Option<V3> {
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
                let chest = g.ch[g.gd[t].ch].bone_or(crate::character::Bone::Chest, 1.2);
                if !crate::scene_collision::segment_blocked(g, [at[0], 1.6, at[2]], chest, None, None, None) { return Some(at); }
            }
        }
    }
    None
}

/// one turn of a burst at the hole; false = the burst is over
fn round(g: &mut crate::game::Game, gd: GuardId, dt: f64, every: f64) -> bool {
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
    crate::combat::fire_weapon(g, c, at, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(spread), blind: Some(true), ..crate::combat::FireOpts::default() }));
    let r = g.m.morale.state_of.get_mut(gd).unwrap();
    r.rounds -= 1.0;
    r.next = every;
    let left = r.rounds;
    left > 0.0
}

/// the long burst is spent: the guards' own magazine change, stretched
fn run_dry(g: &mut crate::game::Game, gd: GuardId) {
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
}

/// a bait taken: the magazine, then a short burst, then a word
fn bait(g: &mut crate::game::Game, gd: GuardId) {
    g.m.morale.state_of.get_mut(gd).unwrap().baits += 1.0;
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
        let cap = g.weapon(g.fa[w].def).mag_capacity;
        g.m.morale.state_of.get_mut(gd).unwrap().rounds = cap + 1.0;
    } else if st.baits == 2.0 {
        begin(g, gd, MoraleAct::Bait, None);
        g.m.morale.state_of.get_mut(gd).unwrap().rounds = bait_short;
    } else if st.baits == 3.0 {
        bark(g, gd, LineKind::Spent, false, Some(true));
    }
}

/// what this file's script holds him to this frame, on the row
fn pose(g: &mut crate::game::Game, gd: GuardId, rooted: bool) {
    let st = now(g, gd);
    let hole = st.hole.unwrap();
    let low = rooted || st.act == MoraleAct::Duck || st.act == MoraleAct::Flinch || (st.act == MoraleAct::Down && folds(g, gd)) || st.act == MoraleAct::Reload;
    let walks = st.act == MoraleAct::Back || st.act == MoraleAct::Stand;
    let face = v3::yaw_to(g.pos_of(gd), hole);
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

/// once a frame: decay, standing, the blasts filed, the rungs, each sequence's step
pub fn morale_tick(g: &mut crate::game::Game, dt: f64) {
    let (half_life, ask_secs) = (g.k.morale.half_life, g.k.morale.ask_secs);
    let k = m::pow(2.0, -dt / half_life);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) || crate::mission::leads(g, gd) { g.m.morale.state_of.delete(gd); continue; }
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
    // who this file's script has this frame, as it stands when the sequences have all been stepped
    let rule = crate::owner::by::MORALE;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let t = g.time;
        if g.m.morale.state_of.get(gd).map_or(false, |s| s.drive) { crate::owner::claim(g, gd, rule, t, None); } else { crate::owner::release(g, gd, rule, t); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// a steady man breaks alone AND dark; a green man alone, or lost with his leader down
    #[test]
    fn who_may_break() {
        let m_ = |green, alone, dark, leader_down, lost| Mind { green, alone, dark, leader_down, lost, ..Mind::default() };
        assert!(may_break(m_(false, true, true, false, false)));
        assert!(!may_break(m_(false, true, false, false, false)));
        assert!(!may_break(m_(false, false, true, true, true)));
        assert!(may_break(m_(true, true, false, false, false)));
        assert!(may_break(m_(true, false, false, true, true)));
        assert!(!may_break(m_(true, false, true, true, false)));
    }
    /// pinned bits
    #[test]
    fn flat_is_the_typescripts() {
        assert_eq!(dist_xz([1.1, 0.0, 2.2], [3.3, 0.0, -4.4]).to_bits(), 0x401bd3faa7236476);
        assert_eq!(dist_xz([0.1, 0.0, -9.3], [7.77, 0.0, 0.004]).to_bits(), 0x40281da70b81d837);
    }
    /// every kind has a table row, so lines_of cannot run off the end
    #[test]
    fn every_kind_has_lines() {
        for k in [LineKind::Duck, LineKind::Call, LineKind::Again, LineKind::Out, LineKind::Spent, LineKind::Clear, LineKind::Lost, LineKind::Order, LineKind::Okay] {
            assert_eq!(lines_of(k).say.len() == 0, k == LineKind::Call);
        }
    }
}

/// The row as it stands now, for reading: every read comes fresh from the table, every write goes through it. `step` and `drive` are called on a man who has one.
fn row(g: &Game, gd: GuardId) -> Mind {
    *g.m.morale.state_of.get(gd).unwrap()
}

/// His men whose duck waits on his order, nearest first.
fn his(g: &Game, gd: GuardId) -> Vec<GuardId> {
    let mut men: Vec<GuardId> = Vec::new();
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if let Some(mm) = g.m.morale.state_of.get(o) {
            if mm.by == Some(gd) && mm.act == MoraleAct::Duck { men.push(o); }
        }
    }
    let at = g.pos_of(gd);
    let mut keyed: Vec<(GuardId, V3)> = Vec::new();
    for &o in men.iter() { let p = g.pos_of(o); keyed.push((o, p)); }
    js::sort_by(&mut keyed, |a, b| dist_xz(a.1, at) - dist_xz(b.1, at));
    keyed.into_iter().map(|x| x.0).collect()
}

/// one frame of a man's sequence: decides the act, fires its rounds, says its lines, and whether this file's script has him this frame
fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    if !reacts(g, gd) || row(g, gd).hole.is_none() { end(g, gd); return; }
    if g.gd[gd].script.is_none() && g.gd[gd].state != crate::game::GuardState::Alert && g.gd[gd].awareness >= ALERT_AW { return; }
    let hole = row(g, gd).hole.unwrap();
    let st = g.m.morale.state_of.get_mut(gd).unwrap();
    if st.calls > 0.0 { st.call_t += dt; }
    let rooted = crate::control::man::staggered(g, gd);
    let act = row(g, gd).act;
    let goes_on = match act {
        MoraleAct::Duck => ducks_on(g, gd, dt, rooted),
        MoraleAct::Flinch => flinches_on(g, gd, dt, rooted),
        MoraleAct::Stand => stands_up_on(g, gd, hole, dt, rooted),
        _ => return broken_step(g, gd, act, hole, dt, rooted),
    };
    if goes_on { drive(g, gd, dt, rooted); }
}

/// He stays ducked until his leader's word has had its time, the duck has lasted, or he sees an enemy; false = it is over.
fn ducks_on(g: &mut Game, gd: GuardId, dt: f64, rooted: bool) -> bool {
    let st = g.m.morale.state_of.get_mut(gd).unwrap();
    if st.heard >= 0.0 { st.heard += dt; }
    if rooted { return true; }
    g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
    let now = row(g, gd);
    if now.heard >= g.k.morale.order_secs + now.next {
        if now.said { bark(g, gd, LineKind::Okay, false, Some(true)); }
        end(g, gd);
        return false;
    }
    let lasts = if folds(g, gd) { g.k.morale.duck_secs } else { g.k.morale.steady_duck };
    if now.t >= lasts || !seen_by(g, gd, None).is_empty() { end(g, gd); return false; }
    true
}

/// The flinch lasts `flinchSecs`; false = it is over.
fn flinches_on(g: &mut Game, gd: GuardId, dt: f64, rooted: bool) -> bool {
    if rooted { return true; }
    g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
    if row(g, gd).t >= g.k.morale.flinch_secs { end(g, gd); return false; }
    true
}

/// The leader steps off the hole's line and gets his men up, until his time is up or he sees an enemy; false = it is over.
fn stands_up_on(g: &mut Game, gd: GuardId, hole: V3, dt: f64, rooted: bool) -> bool {
    if rooted { return true; }
    if row(g, gd).t == 0.0 {
        let to = his(g, gd).first().copied();
        let goal = side_goal(g, gd, hole, to);
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        st.goal = goal;
        st.script.arrived = Some(false);
    }
    g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
    if row(g, gd).t >= g.k.morale.stand_secs || !seen_by(g, gd, None).is_empty() { end(g, gd); return false; }
    let st = g.m.morale.state_of.get_mut(gd).unwrap();
    if st.goal.is_some() && st.script.arrived == Some(true) { st.goal = None; }
    let now = row(g, gd);
    if !now.said && now.t >= g.k.morale.order_beat { gives_the_word(g, gd); }
    true
}

/// Once, at its beat: he tells his ducked men to get up, and they answer one after another, the nearest first.
fn gives_the_word(g: &mut Game, gd: GuardId) {
    g.m.morale.state_of.get_mut(gd).unwrap().said = true;
    let men = his(g, gd);
    if men.is_empty() { return; }
    bark(g, gd, LineKind::Order, false, Some(true));
    g.stats.morale.orders += 1;
    let gap = g.k.morale.order_gap;
    for (i, &o) in men.iter().enumerate() {
        let mm = g.m.morale.state_of.get_mut(o).unwrap();
        mm.heard = 0.0;
        mm.next = (i as f64) * gap;
        mm.said = i == 0;
    }
}

/// A broken man's sequence: over if he is no longer broken or sees an enemy (a bait or a reload runs its course), else one step of the act he is in.
fn broken_step(g: &mut Game, gd: GuardId, act: MoraleAct, hole: V3, dt: f64, rooted: bool) {
    let owned = act == MoraleAct::Bait || act == MoraleAct::Reload;
    if row(g, gd).rung < 2.0 && !owned { end(g, gd); return; }
    if rooted { drive(g, gd, dt, true); return; }
    let seen: Vec<CharId> = if owned { Vec::new() } else { seen_by(g, gd, if act == MoraleAct::Hold { Some(hole) } else { None }) };
    if !seen.is_empty() && act != MoraleAct::Hold { end(g, gd); return; }
    let st = g.m.morale.state_of.get_mut(gd).unwrap();
    st.t += dt;
    st.broke_t += dt;
    if act == MoraleAct::Hold && row(g, gd).broke_t >= g.k.morale.hold_max { end(g, gd); return; }
    match act {
        MoraleAct::Down => stays_down(g, gd, hole),
        MoraleAct::Back => scrabbles_back(g, gd, hole),
        MoraleAct::Burst => {
            let every = g.k.morale.burst_every;
            let over = !round(g, gd, dt, every)
                || row(g, gd).t > g.k.morale.burst_rounds * g.k.morale.burst_every + 2.0;
            if over { begin(g, gd, MoraleAct::Hold, Some(hole)); }
        }
        MoraleAct::Hold => {
            if !holds_the_hole(g, gd, hole, &seen) { return; }
        }
        MoraleAct::Bait => shoots_at_the_bait(g, gd, hole, dt),
        MoraleAct::Reload => {
            if g.gd[gd].reload_t < 0.0 { begin(g, gd, MoraleAct::Hold, Some(hole)); }
        }
        _ => {}
    }
    drive(g, gd, dt, false);
}

/// Down for `downSecs`, then he scrabbles back and makes his first call: for help if he folds and is alone, a report if he doesn't fold.
fn stays_down(g: &mut Game, gd: GuardId, hole: V3) {
    if row(g, gd).t >= g.k.morale.down_secs {
        let t = row(g, gd).t;
        begin(g, gd, MoraleAct::Back, Some(hole));
        g.m.morale.state_of.get_mut(gd).unwrap().t = t;
        let goal = back_goal(g, gd, hole);
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        st.goal = goal;
        st.script.arrived = Some(false);
        let now = row(g, gd);
        if now.calls == 0.0 {
            if !folds(g, gd) { bark(g, gd, LineKind::Clear, true, Some(true)); }
            else if now.alone { bark(g, gd, LineKind::Call, true, Some(true)); }
            let st = g.m.morale.state_of.get_mut(gd).unwrap();
            st.calls = 1.0;
            st.call_t = 0.0;
        }
    }
}

/// When the scrabble is done he fires a burst at the hole if he folds, else he just holds it.
fn scrabbles_back(g: &mut Game, gd: GuardId, hole: V3) {
    let now = row(g, gd);
    if now.t >= g.k.morale.back_until || now.goal.is_none() || now.script.arrived == Some(true) {
        if folds(g, gd) {
            begin(g, gd, MoraleAct::Burst, Some(hole));
            let n = g.k.morale.burst_rounds;
            g.m.morale.state_of.get_mut(gd).unwrap().rounds = n;
        } else {
            begin(g, gd, MoraleAct::Hold, Some(hole));
        }
    }
}

/// Gun on the hole: he calls a second time if nobody came, and takes the bait of a thing thrown or a man at the hole; a man seen elsewhere ends it (false).
fn holds_the_hole(g: &mut Game, gd: GuardId, hole: V3, seen: &[CharId]) -> bool {
    let now = row(g, gd);
    if now.calls == 1.0 && now.call_t >= g.k.morale.call2_secs && now.alone && folds(g, gd) {
        bark(g, gd, LineKind::Again, true, Some(true));
        g.m.morale.state_of.get_mut(gd).unwrap().calls = 2.0;
    }
    if seen.is_empty() {
        let at = thrown(g, gd, hole);
        let now = row(g, gd);
        if at && !now.at_hole { bait(g, gd); }
        g.m.morale.state_of.get_mut(gd).unwrap().at_hole = at;
        return true;
    }
    let now = row(g, gd);
    let bait_m = g.k.morale.bait_m;
    let at_it = now.baits == 0.0 && seen.iter().any(|&c| dist_xz(g.ch[c].pos, hole) <= bait_m);
    if !at_it {
        end(g, gd);
        return false;
    }
    bait(g, gd);
    g.m.morale.state_of.get_mut(gd).unwrap().at_hole = true;
    true
}

/// Rounds at the bait until they are spent or a magazine's time has gone: a folding man's first bait runs him dry, else back to holding.
fn shoots_at_the_bait(g: &mut Game, gd: GuardId, hole: V3, dt: f64) {
    let every = g.k.morale.bait_every;
    let mut over = !round(g, gd, dt, every);
    if !over {
        let w = g.fa[g.gd[gd].weapon].def;
        let cap = g.weapon(w).mag_capacity;
        over = row(g, gd).t > (cap + 1.0) * g.k.morale.bait_every + 2.0;
    }
    if over {
        if row(g, gd).baits == 1.0 && folds(g, gd) { run_dry(g, gd); }
        else { begin(g, gd, MoraleAct::Hold, Some(hole)); }
    }
}

/// this file's script has him this frame: posed, gun out, his reaction beat run down
fn drive(g: &mut crate::game::Game, gd: GuardId, dt: f64, rooted: bool) {
    pose(g, gd, rooted);
    let time = g.time;
    {
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        st.drive = true; st.drive_t = time;
    }
    g.gd[gd].drawn = true;
    {
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        if st.shout && st.act != MoraleAct::Duck { st.shout = false; }
    }
    let now = row(g, gd);
    if now.shout {
        let time = g.time;
        let gap = g.k.morale.shout_gap;
        let talking = match &g.gd[gd].bubble { Some(b) => time - b.t < gap, None => false };
        if !talking {
            g.m.morale.state_of.get_mut(gd).unwrap().shout = false;
            bark(g, gd, LineKind::Duck, false, Some(true));
        }
    }
    if g.gd[gd].react_t > 0.0 { g.gd[gd].react_t -= dt; }
}

/// THE FIRST OF A RULE'S THREE PARTS (control/), for one man: a man the rule drives who is down and folds clutches himself. The rest of its state is
/// moved by `morale_tick`.
pub(super) fn morale_transition(g: &mut crate::game::Game, gd: GuardId) {
    let Some(s) = g.m.morale.state_of.get(gd).copied() else { return };
    if !s.drive || s.drive_t != g.time { return; }
    if s.act == MoraleAct::Down && folds(g, gd) {
        let c = g.gd[gd].ch;
        g.ch[c].anim.clutch = 1.0;
    }
}

/// WHAT IT WANTS OF HIM (the second part; it only reads): the script his body follows this tick, None = he is nobody's but his own.
pub(super) fn morale_propose(g: &crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> {
    let s = g.m.morale.state_of.get(gd)?;
    if !s.drive || s.drive_t != g.time { return None; }
    Some(s.script)
}

/// his dread while an event's shock is still on him, 0 otherwise
pub fn shaken_of(g: &crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd).copied() {
        Some(s) if s.shock > 0.0 => dread(g, gd),
        _ => 0.0,
    }
}

/// his rung: 0 steady, 1 jumpy, 2 broken
pub fn rung_of(g: &crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd) { Some(s) => s.rung, None => 0.0 }
}

/// QA: what the rule has of a man, None = never met
pub(super) fn morale_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::morale::MoraleOf> {
    let s = g.m.morale.state_of.get(gd)?;
    Some(MoraleOf {
        t: s.t, act: s.act,
        alone: s.alone, green: s.green,
    })
}

/// The man is that file's from here: whatever sequence has him ends where it stands, and this rule lets go of him; dread and rung stay
pub(super) fn morale_release(g: &mut crate::game::Game, gd: GuardId) {
    if let Some(s) = g.m.morale.state_of.get(gd).copied() {
        if s.act != MoraleAct::None || s.drive { end(g, gd); }
    }
    let t = g.time;
    crate::owner::release(g, gd, crate::owner::by::MORALE, t);
}
