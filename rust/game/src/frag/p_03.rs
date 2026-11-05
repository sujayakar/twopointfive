//! src/game/frag.ts lines 678 to 992
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One live grenade's book, by its place in the list (the row is never held across a call that takes `g`).
macro_rules! live { ($g:ident, $i:ident) => { $g.m.frag.books.as_mut().unwrap()[$i] }; }
/// The defenders' book, and one carrier of it by place.
macro_rules! book { ($g:ident) => { $g.m.frag.def_books.as_mut().unwrap() }; }
macro_rules! car { ($g:ident, $ci:ident) => { $g.m.frag.def_books.as_mut().unwrap().carriers[$ci] }; }

/// `c.bones.head ?? v3.add(c.pos, [0, 1.6, 0])`, the head of a body.
fn head_of(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) }
}
/// `gd.weapon.def`: the row of his gun, a copy, since `shot_energy` is lent it beside `g`.
fn gun_row(g: &Game, gd: GuardId) -> Option<crate::weapons::WeaponDef> {
    g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).cloned()
}
/// `s.arrived` as a condition.
fn arrived(s: &crate::game::GuardScript) -> bool { s.arrived == Some(true) }
/// A man's script off this book, null and absent alike.
fn flee_of(g: &Game, i: usize, gd: GuardId) -> Option<crate::game::GuardScript> {
    g.m.frag.books.as_ref().unwrap()[i].flee.get(gd).and_then(|s| s.clone())
}

/// The floor others of his side have a claim to while this grenade burns: where a mate is going, where a mate is staying.   (frag.ts takenBy)
pub fn taken_by(g: &mut crate::game::Game, l: Option<&crate::frag::Live>, gd: GuardId, dz: V3) -> crate::frag::TakenBy {
    let (mut stay, mut go): (Vec<V3>, Vec<V3>) = (Vec::new(), Vec::new());
    let from: V3 = [dz[0], dz[1] + g.k.frag.lift, dz[2]];
    let team = g.gd[gd].team;
    let me = g.ch[g.gd[gd].ch].pos;
    let mut n = 0;
    while n < g.guards.len() {
        let o = g.guards[n]; n += 1;
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead || g.gd[o].team != team { continue; }
        let s = l.and_then(|l| l.flee.get(o)).and_then(|s| s.as_ref());
        let oc = g.gd[o].ch;
        let p = g.ch[oc].pos;
        let on_leg = s.map_or(false, |s| s.goal.is_some() && !arrived(s));
        let q = if on_leg { s.unwrap().goal.unwrap() } else { p };
        if flat(q, me) > 8.5 { continue; }
        if on_leg { go.push(q); }
        else if s.is_some() || g.time < g.gd[o].stagger_until || g.time < g.gd[o].dazzled_until || flat(p, dz) > g.k.frag.flee_m || { let h = head_of(g, oc); exposed_to(g, from, p, h, None) <= 0.0 } { stay.push(p); }
    }
    if team == crate::teams::Team::Operative {
        let mg = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
        for pl in crate::seats::men(g, &mg) {
            let pp = g.ch[g.pl[pl].ch].pos;
            if flat(pp, me) <= 8.5 { stay.push(pp); }
        }
    }
    crate::frag::TakenBy { stay, go }
}

/// The list the deal would hand `flee_spot` for this man off a grenade at rest at `dz` that has dealt nobody yet.   (frag.ts fleeMates)
pub fn flee_mates(g: &mut crate::game::Game, gd: GuardId, dz: V3) -> Option<crate::squadthrow::SquadThrowKeptDuckM> {
    let t = taken_by(g, None, gd, dz);
    Some(crate::squadthrow::SquadThrowKeptDuckM { stay: t.stay, go: t.go })
}

/// Once a frame, before the men move: each live grenade's book.   (frag.ts fragTick)
pub fn frag_tick(g: &mut crate::game::Game, _dt: f64) {
    g.m.frag.books.get_or_insert_with(Vec::new);
    let mut n = 0;
    while n < g.items.items.len() {
        let it = g.items.items[n]; n += 1;
        if crate::throwables::item_kind(g, it) != crate::throwables::ItemKind::Frag || crate::throwables::item_fired(g, it) { continue; }
        if g.m.frag.books.as_ref().unwrap().iter().any(|l| l.it == it) { continue; }
        let who = crate::throwables::item_who(g, it);
        let side = if js::truthy(who) { crate::teams::team_of_id(g, who) } else { crate::teams::Team::Guard };
        g.m.frag.books.as_mut().unwrap().push(crate::frag::Live {
            it, side, knows: JsMap::new(), shouted: JsSet::new(), flee: JsMap::new(), via: None, flee_at: JsMap::new(),
            deal_t: JsMap::new(), counted: JsSet::new(), cued: false, rest: None, made: None,
        });
    }
    let mut i = g.m.frag.books.as_ref().unwrap().len();
    while i > 0 {
        i -= 1;
        tick_book(g, i);
    }
}

/// One grenade's turn of `frag_tick`; a grenade that is gone is struck off here.
fn tick_book(g: &mut Game, i: usize) {
    let it = live!(g, i).it;
    // (the list first: the accessors read a row that a thing off the list no longer has)
    if !g.items.items.contains(&it) || crate::throwables::item_fired(g, it) || crate::throwables::item_life(g, it) <= 0.0 {
        for gd in live!(g, i).flee.keys() {
            g.gd[gd].path = Vec::new();
            g.gd[gd].path_goal = None;
            if g.gd[gd].state != crate::game::GuardState::Dead { let c = g.gd[gd].ch; g.ch[c].anim.crouch_target = 0.0; }
        }
        g.m.frag.books.as_mut().unwrap().remove(i);
        return;
    }
    let at = crate::throwables::item_pos(g, it);
    let eye_at: V3 = [at[0], at[1] + 0.12, at[2]];
    let mut lifted = live!(g, i).clone();
    let wise = rest_of(g, &mut lifted);
    live!(g, i) = lifted;
    let dz = wise.at;
    if !live!(g, i).cued && live!(g, i).side == crate::teams::Team::Hostile {
        let mut lifted = live!(g, i).clone();
        sam_cue(g, &mut lifted, eye_at);
        live!(g, i) = lifted;
    }
    if !g.ai_enabled { return; }
    let mut n = 0;
    while n < g.guards.len() {
        let gd = g.guards[n]; n += 1;
        tick_man(g, i, it, at, eye_at, &wise, dz, gd);
    }
}

/// One man's turn under one grenade: noticing, the shout, and his legs.
fn tick_man(g: &mut Game, i: usize, it: ItemId, at: V3, eye_at: V3, wise: &crate::frag::Rest, dz: V3, gd: GuardId) {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.gd[gd].civ.is_some() { return; }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let team = g.gd[gd].team;
    let serial = crate::throwables::item_serial(g, it);
    let d = m::hyp2(pos[0] - at[0], pos[2] - at[2]);
    if !live!(g, i).knows.has(gd) {
        if d > m::max(g.k.frag.see_m, g.k.frag.flee_m) { return; }
        let mut when = -1.0;
        if same_side(live!(g, i).side, team) { when = g.time + 0.1; }
        else if g.time >= g.gd[gd].dazzled_until {
            let head = head_of(g, c);
            let to = m::atan2(at[0] - pos[0], at[2] - pos[2]);
            let mut off = to - g.ch[c].aim_yaw;
            off = m::atan2(m::sin(off), m::cos(off));
            let told = g.m.frag.warned.get(gd).copied().unwrap_or(-1.0) > g.time;
            let hear_m = if told { g.k.frag.hear_m * 2.0 } else { g.k.frag.hear_m };
            let sees = d <= g.k.frag.see_m && (told || d < 3.0 || m::abs(off) < 1.9) && !crate::scene_collision::segment_blocked(g, head, eye_at, None, None, None);
            let mut hears = false;
            if !sees && crate::throwables::item_bounces(g, it) > 0.0 && d <= hear_m {
                let pr = crate::scene_collision::sound_paths_propagate(g, eye_at, head, None);
                hears = pr.muffle < 0.5 && pr.path_len <= hear_m + 1.0;
            }
            if sees || hears {
                let react = if g.gd[gd].trained || team == crate::teams::Team::Operative { g.k.frag.react[0] } else { g.k.frag.react[1] };
                when = g.time + react + hash01(serial, g.ch[c].id) * g.k.frag.react_jit;
            }
        }
        if when >= 0.0 { live!(g, i).knows.set(gd, when); }
    }
    match live!(g, i).knows.get(gd).copied() { Some(k) if g.time >= k => {} _ => return }
    if !live!(g, i).shouted.has(team) && !same_side(live!(g, i).side, team) && g.time >= g.gd[gd].stagger_until {
        live!(g, i).shouted.add(team);
        crate::game::game_say(g, gd, "grenade!", None, None, None);
        g.stats.frag.shouts += 1;
        let mouth = head_of(g, c);
        let mut n = 0;
        while n < g.guards.len() {
            let o = g.guards[n]; n += 1;
            if o == gd || g.gd[o].state == crate::game::GuardState::Dead || g.gd[o].team != team || live!(g, i).knows.has(o) { continue; }
            let oc = g.gd[o].ch;
            if v3::dist(g.ch[oc].pos, pos) > g.k.frag.shout_m { continue; }
            let oh = head_of(g, oc);
            let pr = crate::scene_collision::sound_paths_propagate(g, mouth, oh, None);
            if pr.muffle > 0.6 || pr.path_len > g.k.frag.shout_m + 2.0 { continue; }
            let when = g.time + 0.15 + hash01(serial + 7.0, g.ch[oc].id) * 0.15;
            live!(g, i).knows.set(o, when);
        }
    }
    let mut keep: Option<crate::game::GuardScript> = None;
    let mut not: Option<V3> = None;
    let fuse = crate::throwables::item_fuse(g, it);
    {   // on a leg and no ground made for a second: another leg, not that spot
        let cur = flee_of(g, i, gd);
        let time = g.time;
        let l = &mut live!(g, i);
        let made = l.made.get_or_insert_with(JsMap::new);
        let was = made.get(gd).copied();
        let on_leg = cur.as_ref().map_or(false, |s| s.goal.is_some() && !arrived(s));
        if !on_leg { made.delete(gd); }
        else if was.map_or(true, |w| flat(w.at, pos) > 0.3) { made.set(gd, crate::frag::LiveMadeValue { at: pos, t: time }); }
        else if time - was.unwrap().t > 1.0 && fuse > 0.6 {
            not = cur.unwrap().goal;
            made.delete(gd);
            l.flee.delete(gd);
            l.flee_at.delete(gd);
        }
    }
    if live!(g, i).via.as_ref().map_or(false, |v| v.has(gd)) {   // round the leaf's end: the way out is dealt from here, in stride
        let cur = flee_of(g, i, gd);
        let dealt = live!(g, i).deal_t.get(gd).copied().unwrap_or(-1.0);
        if let Some(goal) = cur.and_then(|s| s.goal) {
            if g.time - dealt > 0.3 && flat(pos, goal) < g.k.frag.flee_door_via_m {
                let l = &mut live!(g, i);
                l.via.as_mut().unwrap().delete(gd);
                l.flee.delete(gd);
                l.flee_at.delete(gd);
            }
        }
    }
    {   // dealt again when it has gone on, or when he has got there and it turns out to have a line to him after all
        let cur = flee_of(g, i, gd);
        let was = live!(g, i).flee_at.get(gd).copied();
        let dealt = live!(g, i).deal_t.get(gd).copied().unwrap_or(-1.0);
        if let (Some(cur), Some(was)) = (cur, was) {
            if g.time - dealt > 0.3 {
                let again = flat(was, dz) > 0.4
                    || ((match cur.goal { None => true, Some(goal) => arrived(&cur) && !(flat(pos, goal) > 0.06 && g.gd[gd].speed > 0.05) })
                        && fuse > 0.6
                        && { let h = head_of(g, c); let from = [dz[0], dz[1] + g.k.frag.lift, dz[2]]; exposed_to(g, from, pos, h, None) > 0.0 });
                if again {
                    if cur.goal.is_some() && !arrived(&cur) { keep = Some(cur); }
                    let l = &mut live!(g, i);
                    l.flee.delete(gd);
                    l.flee_at.delete(gd);
                }
            }
        }
    }
    if flee_of(g, i, gd).is_some() { return; }
    // in its line and close enough to matter: move. (Blind or folded, he cannot.)
    if flat(pos, dz) > g.k.frag.flee_m || g.time < g.gd[gd].stagger_until || g.time < g.gd[gd].dazzled_until { return; }
    let head = head_of(g, c);
    let burst: V3 = [dz[0], dz[1] + g.k.frag.lift, dz[2]];
    if exposed_to(g, burst, pos, head, None) <= 0.0 {   // a wall already between him and it: he stays behind it, down
        if g.gd[gd].script.is_none() && !g.gd[gd].hold && !g.gd[gd].pinned && flat(pos, dz) <= g.k.frag.lethal_m + 2.0 {
            let time = g.time;
            let l = &mut live!(g, i);
            l.flee.set(gd, Some(crate::game::GuardScript { goal: None, upper: Some(crate::character::UpperMode::None), crouch: Some(true), ..Default::default() }));
            l.flee_at.set(gd, dz);
            l.deal_t.set(gd, time);
            g.gd[gd].path = Vec::new();
            g.gd[gd].path_goal = None;
        }
        return;
    }
    let run = crate::teampace::team_blast_pace(g, gd, g.k.frag.run_speed);
    let speed = crate::condition::condition_pace(g, gd, run);
    let acc = if team == crate::teams::Team::Operative && g.gd[gd].speed < speed * 0.6 { g.k.pace.accel } else { m::INF };
    let keep_goal = keep.as_ref().and_then(|k| k.goal);
    let lifted = live!(g, i).clone();
    let apart = taken_by(g, Some(&lifted), gd, dz);
    let w_spot = crate::frag::FleeSpotWise { path: wise.path.clone(), keep: keep_goal, not, apart: Some(crate::frag::FleeSpotWiseApart { stay: apart.stay.clone(), go: apart.go.clone() }) };
    let legs = crate::kitload::load_pace(g, gd, speed);
    let pick = acc * crate::kitload::load_pace_on(g, crate::kitload::LoadPaceOnWho::Guard(gd));
    let mut spot = flee_spot(g, crate::frag::FleeSpotC { pos }, dz, legs, fuse, Some(pick), Some(&w_spot), Some(gd));
    {
        let time = g.time;
        let l = &mut live!(g, i);
        l.flee_at.set(gd, dz);
        l.deal_t.set(gd, time);
        if let Some(v) = l.via.as_mut() { v.delete(gd); }
    }
    // the straight leg leaves him in its line, or there is none: out by an open doorway his legs make
    let by_door = match spot {
        None => true,
        Some(s) => exposed_to(g, burst, s, [s[0], 1.15, s[2]], None) > 0.0 || in_mouth(g, s),
    };
    if by_door {
        let w_door = crate::frag::FleeByDoorWise { path: wise.path.clone(), keep: keep_goal, not, apart: Some(crate::frag::FleeByDoorWiseApart { stay: apart.stay, go: apart.go }) };
        if let Some(out) = flee_by_door(g, crate::frag::FleeByDoorC { pos }, dz, legs, fuse, pick, &w_door, Some(gd)) {
            if Some(out.goal) != keep_goal { g.stats.frag.by_door += 1; }
            spot = Some(out.goal);
            if out.via { live!(g, i).via.get_or_insert_with(JsSet::new).add(gd); }
        }
    }
    if let Some(k) = keep {   // the leg he has: his path and his stride are left as they are
        if spot == k.goal { live!(g, i).flee.set(gd, Some(k)); return; }
    }
    let script = match spot {
        Some(s) => crate::game::GuardScript { goal: Some(s), speed: Some(speed), upper: Some(crate::character::UpperMode::None), crouch: Some(false), ..Default::default() },
        None => crate::game::GuardScript { goal: None, upper: Some(crate::character::UpperMode::None), crouch: Some(true), ..Default::default() },
    };
    live!(g, i).flee.set(gd, Some(script));
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    if !live!(g, i).counted.has(gd) { live!(g, i).counted.add(gd); g.stats.frag.fled += 1; }
}

/// The script his legs follow while a live grenade he knows of has a line to him (None = none). Once there he gets down.   (frag.ts fragFlee)
pub fn frag_flee(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> {
    let n = match g.m.frag.books.as_ref() { Some(b) if !b.is_empty() => b.len(), _ => return None };
    for i in 0..n {
        if let Some(s) = flee_of(g, i, gd) {
            if g.time < g.gd[gd].stagger_until { return None; }
            if s.goal.is_some() && arrived(&s) {
                if let Some(Some(row)) = live!(g, i).flee.get_mut(gd) { row.crouch = Some(true); }
                return flee_of(g, i, gd);
            }
            return Some(s);
        }
    }
    None
}

/// Is he on his legs off a live grenade this frame.   (frag.ts fragRunning)
pub fn frag_running(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(b) = g.m.frag.books.as_ref() else { return false };
    b.iter().any(|l| l.flee.get(gd).and_then(|s| s.as_ref()).map_or(false, |s| s.goal.is_some()))
}

/// A grenade's bang that his OWN side threw.   (frag.ts ownBang)
pub fn own_bang(g: &mut crate::game::Game, gd: GuardId, e: crate::frag::OwnBangE) -> bool {
    e.frag.unwrap_or(false) && own_noise(g, gd, e.who)
}

/// A noise filed under the id of a man of his own side.   (frag.ts ownNoise)
pub fn own_noise(g: &mut crate::game::Game, gd: GuardId, who: Option<f64>) -> bool {
    let Some(who) = who else { return false };
    if who == 0.0 { return false; }
    let t = crate::teams::team_of_id(g, who);
    same_side(t, g.gd[gd].team)
}

/// A defender's live grenade Sam can see, or hear land, inside cueM: one HUD line, once.   (frag.ts samCue)
pub fn sam_cue(g: &mut crate::game::Game, l: &mut crate::frag::Live, eye_at: V3) {
    let pl = g.player;
    let pc = g.pl[pl].ch;
    if g.pl[pl].down { return; }
    let pos = g.ch[pc].pos;
    let d = m::hyp2(pos[0] - eye_at[0], pos[2] - eye_at[2]);
    if d > g.k.frag.cue_m { return; }
    let head = head_of(g, pc);
    let mut knows = !crate::scene_collision::segment_blocked(g, head, eye_at, None, None, None);
    if !knows && crate::throwables::item_bounces(g, l.it) > 0.0 {
        let pr = crate::scene_collision::sound_paths_propagate(g, eye_at, head, None);
        knows = pr.muffle < 0.6 && pr.path_len <= g.k.frag.cue_m;
    }
    if !knows { return; }
    l.cued = true;
    g.stats.frag.cues += 1;
    crate::game::game_msg(g, "GRENADE — get clear");
}

/// The overlay's: every hostile grenade Sam has been cued on, where it lies and the seconds left.   (frag.ts fragMarks)
pub fn frag_marks(g: &mut crate::game::Game) -> Vec<crate::frag::FragMarksItem> {
    let cued: Vec<ItemId> = match g.m.frag.books.as_ref() { Some(b) => b.iter().filter(|l| l.cued).map(|l| l.it).collect(), None => return Vec::new() };
    let mut out = Vec::new();
    for it in cued {
        if !crate::throwables::item_fired(g, it) {
            out.push(crate::frag::FragMarksItem { at: crate::throwables::item_pos(g, it), left: crate::throwables::item_fuse(g, it) });
        }
    }
    out
}

/// Where a player who has read the cue would go, off the nearest cued grenade with a line to him; None when none.   (frag.ts fragStepOff)
pub fn frag_step_off(g: &mut crate::game::Game, speed: Option<f64>) -> Option<V3> {
    let speed = speed.unwrap_or(3.2);
    let n = match g.m.frag.books.as_ref() { Some(b) if !b.is_empty() => b.len(), _ => return None };
    let pc = g.pl[g.player].ch;
    for i in 0..n {
        let it = live!(g, i).it;
        if !live!(g, i).cued || crate::throwables::item_fired(g, it) { continue; }
        let mut lifted = live!(g, i).clone();
        let wise = rest_of(g, &mut lifted);
        live!(g, i) = lifted;
        let at = wise.at;
        let pos = g.ch[pc].pos;
        if m::hyp2(pos[0] - at[0], pos[2] - at[2]) > g.k.frag.wound_m { continue; }
        let head = head_of(g, pc);
        if exposed_to(g, [at[0], at[1] + g.k.frag.lift, at[2]], pos, head, None) <= 0.0 { continue; }
        let fuse = crate::throwables::item_fuse(g, it);
        let w = crate::frag::FleeSpotWise { path: wise.path, keep: None, not: None, apart: None };
        let spot = flee_spot(g, crate::frag::FleeSpotC { pos }, at, speed, fuse, Some(m::INF), Some(&w), None);
        if spot.is_some() { return spot; }
    }
    None
}

/// The men of the floor who may carry a frag.   (frag.ts defendersOf)
pub fn defenders_of(g: &mut crate::game::Game) -> Vec<GuardId> {
    g.guards.iter().copied().filter(|&gd| g.gd[gd].team == crate::teams::Team::Hostile && g.gd[gd].unit.is_some()).collect()
}

/// Deal the floor's frags: one defender in defShare, at most defMax, by a hash of the men's ids.   (frag.ts dealCarriers)
pub fn deal_carriers(g: &mut crate::game::Game, was: Option<&crate::frag::DefBook>) {
    let men = defenders_of(g);
    let n = m::min(g.k.frag.def_max, m::ceil(men.len() as f64 / m::max(1.0, g.k.frag.def_share)));
    let mut sorted: Vec<(GuardId, f64)> = men.iter().map(|&gd| (gd, g.ch[g.gd[gd].ch].id)).collect();
    js::sort_by(&mut sorted, |a, b| js::or(hash01(a.1, 9173.0) - hash01(b.1, 9173.0), a.1 - b.1));
    let take = (n as usize).min(sorted.len());
    let carriers: Vec<crate::frag::Carrier> = sorted[..take].iter().map(|&(gd, _)| crate::frag::Carrier {
        gd, frags: 1.0, blocked_t: 0.0, bunch_t: 0.0, try_at: 0.0, wind: None, away: None, spots: None,
    }).collect();
    let mut b = crate::frag::DefBook { men, carriers, last_t: -100.0, serial: 0.0, thrown: 0.0 };
    if let Some(was) = was {   // the roster changed under a run: who had one keeps it, what was thrown stays thrown
        b.last_t = was.last_t;
        b.serial = was.serial;
        b.thrown = was.thrown;
        let still = was.carriers.iter().filter(|c| c.frags > 0.0 && b.men.contains(&c.gd)).count() as f64;
        let mut fresh = m::max(0.0, g.k.frag.def_max - was.thrown - still);
        let dealt = core::mem::take(&mut b.carriers);
        for c in dealt {
            if let Some(o) = was.carriers.iter().find(|o| o.gd == c.gd) { b.carriers.push(o.clone()); continue; }
            let give = fresh > 0.0;
            fresh -= 1.0;
            if give { b.carriers.push(c); } else { b.carriers.push(crate::frag::Carrier { frags: 0.0, ..c }); }
        }
    }
    g.m.frag.def_books = Some(b);
}

/// Who on the defenders' side still carries a frag.   (frag.ts fragCarriers)
pub fn frag_carriers(g: &mut crate::game::Game) -> Vec<GuardId> {
    let Some(b) = g.m.frag.def_books.as_ref() else { return Vec::new() };
    b.carriers.iter().filter(|c| c.frags > 0.0 && g.gd[c.gd].state != crate::game::GuardState::Dead).map(|c| c.gd).collect()
}

/// `FRAG_WHY[w]++`: why a throw was not looked for.
fn no(g: &mut Game, w: &str) {
    let n = g.m.frag.frag_why.get(String::from(w)).copied().unwrap_or(0.0) + 1.0;
    g.m.frag.frag_why.set(String::from(w), n);
}

/// The defenders' frags, once a frame beside `frag_tick`.   (frag.ts fragThrowTick)
pub fn frag_throw_tick(g: &mut crate::game::Game, dt: f64) {
    if !g.ai_enabled || !crate::roomfight::fight_live(g) || crate::director::director_their_frags_off(g) { return; }
    let men = defenders_of(g);
    let stale = match g.m.frag.def_books.as_ref() { None => true, Some(b) => b.men != men };
    if stale {
        if men.is_empty() { return; }
        let was = g.m.frag.def_books.clone();
        deal_carriers(g, was.as_ref());
    }
    let n = book!(g).carriers.len();
    for ci in 0..n { tick_carrier(g, ci, dt); }
}

/// The line of a throw of his own ends here: the two clocks go back to nothing.
fn clear_clocks(g: &mut Game, ci: usize) { car!(g, ci).blocked_t = 0.0; car!(g, ci).bunch_t = 0.0; }

/// One carrier's turn of `frag_throw_tick`.
fn tick_carrier(g: &mut Game, ci: usize, dt: f64) {
    let gd = car!(g, ci).gd;
    let c = g.gd[gd].ch;
    if !js::truthy(car!(g, ci).frags) { return; }
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || crate::runner::runner_owns(g, gd) { car!(g, ci).wind = None; return; }
    let out = g.time < g.gd[gd].stagger_until || g.time < g.gd[gd].dazzled_until;
    let head = head_of(g, c);
    if car!(g, ci).wind.is_some() { tick_wind(g, ci, gd, c, out); return; }
    if out || frag_flee(g, gd).is_some() { clear_clocks(g, ci); return; }
    let state = g.gd[gd].state;
    let Some(fix) = g.gd[gd].last_known.filter(|_| state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search) else { clear_clocks(g, ci); no(g, "calm"); return };
    if g.time - m::max(g.gd[gd].last_seen_t, g.gd[gd].fix_t) > g.k.frag.def_fresh { clear_clocks(g, ci); no(g, "stale"); return; }
    let pos = g.ch[c].pos;
    let d = flat(pos, fix);
    if d < g.k.frag.def_near || d > g.k.frag.def_far { clear_clocks(g, ci); let why = if d < g.k.frag.def_near { "near" } else { "far" }; no(g, why); return; }
    // (a) the man he was on has gone where his rifle cannot follow; (b) two or more of them bunched in his view
    let chest: V3 = [fix[0], fix[1] + 1.1, fix[2]];
    let def = gun_row(g, gd);
    let energy = crate::glasspen::shot_energy(g, c, def.as_ref());
    let blocked = crate::scene_collision::shot_blocked(g, head, chest, Some(0.02), energy);
    car!(g, ci).blocked_t = if blocked { car!(g, ci).blocked_t + dt } else { 0.0 };
    let mut bunch = 0;
    if g.time - g.gd[gd].last_seen_t < 0.5 {
        if g.seats.len() == 1 { if seat_in_bunch(g, gd, c, head, fix) { bunch += 1; } }
        else {
            let mg = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
            for pl in crate::seats::men(g, &mg) {
                if crate::seats::with_seat(g, pl, &mut |v: &mut Game| seat_in_bunch(v, gd, c, head, fix)) { bunch += 1; }
            }
        }
        let mut n = 0;
        while n < g.guards.len() {
            let o = g.guards[n]; n += 1;
            if g.gd[o].team != crate::teams::Team::Operative || g.gd[o].state == crate::game::GuardState::Dead || crate::rails::rails_down(g, o) { continue; }
            let oc = g.gd[o].ch;
            let op = g.ch[oc].pos;
            if flat(op, fix) > g.k.frag.def_bunch { continue; }
            if !(matches!(g.gd[gd].foe, Some(crate::game::Foe::Guard(f)) if f == o) || g.gd[o].visibility > 0.2) { continue; }
            let oh = g.ch[oc].bones.get(crate::character::Bone::Head).copied();
            if crate::guards::can_see_char(g, gd, oc, false).visible || sights(g, gd, c, head, op, oh) { bunch += 1; }
        }
    }
    car!(g, ci).bunch_t = if bunch >= 2 { car!(g, ci).bunch_t + dt } else { 0.0 };
    if car!(g, ci).blocked_t < g.k.frag.def_blocked && car!(g, ci).bunch_t < g.k.frag.def_bunch_secs { no(g, "line"); return; }
    if g.time - book!(g).last_t < g.k.frag.def_gap { no(g, "gap"); return; }
    if g.time < car!(g, ci).try_at { return; }
    car!(g, ci).try_at = g.time + 0.25;
    if hostage_near(g, fix) { no(g, "hostage"); return; }
    // WHERE HE MEANS IT: then off that by an untrained arm's scatter
    let yaw = g.ch[c].aim_yaw;
    let from: V3 = [pos[0] + m::sin(yaw) * 0.35, pos[1] + 1.5, pos[2] + m::cos(yaw) * 0.35];
    // a fix his ears gave him is the doorway on his own side: he throws past it or not at all
    let seen = g.time - g.gd[gd].last_seen_t <= g.k.frag.def_fresh;
    let spots = aim_spots(g, gd, Some(from), fix, if seen { 17.0 } else { 0.0 });
    let why = match spots.first() { Some(&aim) => wind_up(g, ci, head, from, aim, None), None => Some(crate::frag::WindUp::Arc) };
    if let Some(why) = why { no(g, why.as_str()); }
}

/// The second it takes him: shot, folded or blinded in it and the grenade stays on his belt; else the release.
fn tick_wind(g: &mut Game, ci: usize, gd: GuardId, c: CharId, out: bool) {
    if out { car!(g, ci).wind = None; return; }
    let mut wd = car!(g, ci).wind.unwrap();
    let pos = g.ch[c].pos;
    if !wd.swung && g.time >= wd.until - crate::character::THROW_RELEASE {
        wd.swung = true;
        car!(g, ci).wind = Some(wd);
        crate::character::anim_throw_item(g, c, Some(flat(pos, wd.at)), None);
    }
    if g.time < wd.until { return; }
    car!(g, ci).wind = None;
    let by_hand = match wd.hand { Some(h) => flat(h, pos) < 1.0 && !crate::scene_collision::solid_blocked(g, [pos[0], h[1], pos[2]], h, None), None => false };
    let mut from: V3 = if by_hand { wd.hand.unwrap() }
        else if let Some(t) = wd.toward { arm_round(g, pos, t) }
        else { let yaw = g.ch[c].aim_yaw; [pos[0] + m::sin(yaw) * 0.35, pos[1] + 1.5, pos[2] + m::cos(yaw) * 0.35] };
    let mut sol = crate::throwables::throwables_solve(g, from, wd.at, Some(11.0));
    if wd.toward.is_none() && crate::character::anim_throw_off(g, c) {   // thrown with the free hand, it leaves THAT hand, where the hand has the same arc to the same spot
        if let Some(h) = g.ch[c].bones.get(crate::character::Bone::HandL).copied() {
            if flat(h, pos) < 1.0 && !crate::scene_collision::solid_blocked(g, [pos[0], h[1], pos[2]], h, None) {
                let sh = crate::throwables::throwables_solve(g, h, wd.at, Some(11.0));
                if lands(g, Some(crate::frag::LandsSol { land: sh.land }), wd.at, 2.0) { from = h; sol = sh; }
            }
        }
    }
    if !lands(g, Some(crate::frag::LandsSol { land: sol.land }), wd.at, 2.0) || !own_clear(g, gd, sol.land, Some(true)) || hostage_near(g, sol.land) {
        book!(g).last_t = wd.last_t;
        car!(g, ci).try_at = g.time + 2.0;
        let line = crate::words::w(g, &"no — holding it!", &"no — hold it!");
        crate::game::game_say(g, gd, line, None, None, None);
        g.stats.frag.held += 1;
        return;
    }
    let opts = crate::throwables::ThrowablesSpawnOpts { fuse: Some(g.k.frag.def_fuse), who: Some(g.ch[c].id), heading: Some(v3::yaw_of(sol.v0)), ..Default::default() };
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Frag, from, sol.v0, Some(opts));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    car!(g, ci).frags -= 1.0;
    car!(g, ci).away = Some(car!(g, ci).away.unwrap_or(0.0) + 1.0);
    book!(g).thrown += 1.0;
    g.stats.frag.theirs += 1;
}

/// Along his own gun line: within 45 degrees of his aim, a line to the man, his eyes not full of magnesium.
fn sights(g: &mut Game, gd: GuardId, c: CharId, head: V3, p: V3, hd: Option<V3>) -> bool {
    let pos = g.ch[c].pos;
    let mut off = m::atan2(p[0] - pos[0], p[2] - pos[2]) - g.ch[c].aim_yaw;
    off = m::atan2(m::sin(off), m::cos(off));
    m::abs(off) < 0.785 && g.time >= g.gd[gd].dazzled_until && flat(p, pos) < 16.0 && !crate::scene_collision::segment_blocked(g, head, hd.unwrap_or_else(|| v3::add(p, [0.0, 1.5, 0.0])), None, None, None)
}

/// Does this seat's man count in the bunch (`v` is the game with that seat's set in).
fn seat_in_bunch(v: &mut Game, gd: GuardId, c: CharId, head: V3, fix: V3) -> bool {
    let pl = v.player;
    let pc = v.pl[pl].ch;
    let p = v.ch[pc].pos;
    if v.pl[pl].down || v.player_invisible || flat(p, fix) > v.k.frag.def_bunch { return false; }
    if !(matches!(v.gd[gd].foe, Some(crate::game::Foe::Player)) || v.pl[pl].visibility > 0.2) { return false; }
    let ph = v.ch[pc].bones.get(crate::character::Bone::Head).copied();
    crate::guards::can_see_char(v, gd, pc, true).visible || (v.gd[gd].smoke_trans >= 0.3 && sights(v, gd, c, head, p, ph))
}

/// Of `n` spots by the fix, those of open floor with the burst's line to the man's chest; `from` given: only the first his arm lands.   (frag.ts aimSpots)
pub fn aim_spots(g: &mut crate::game::Game, gd: GuardId, from: Option<V3>, fix: V3, n: f64) -> Vec<V3> {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let d = flat(pos, fix);
    let ux = (fix[0] - pos[0]) / js::or(d, 1.0);
    let uz = (fix[2] - pos[2]) / js::or(d, 1.0);
    let mut out: Vec<V3> = Vec::new();
    let chest: V3 = [fix[0], fix[1] + 1.1, fix[2]];
    let def = gun_row(g, gd);
    let glass = if crate::glasspen::shot_energy(g, c, def.as_ref()).unwrap_or(0.0) > 0.0 { crate::frag::GlassView::Counted } else { crate::frag::GlassView::Whole };
    let mut q = -2.0;
    while q < n {
        let rr = if q <= 0.0 { 0.0 } else if q <= 8.0 { 1.3 } else { 2.4 };
        let aa = q * (m::PI / 4.0);
        let on = if q == -2.0 { g.k.frag.def_past } else if q == -1.0 { g.k.frag.def_past * 0.55 } else { 0.0 };
        let p: V3 = [fix[0] + ux * on + m::sin(aa) * rr, 0.0, fix[2] + uz * on + m::cos(aa) * rr];
        q += 1.0;
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || !burst_line(g, [p[0], g.k.frag.lift + 0.05, p[2]], chest, Some(glass)) { continue; }
        match from {
            None => out.push(p),
            Some(f) => {
                let sol = crate::throwables::throwables_solve(g, f, p, Some(11.0));
                if lands(g, Some(crate::frag::LandsSol { land: sol.land }), p, 1.0) { return vec![p]; }
            }
        }
    }
    out
}

/// The wind-up begins: the scatter, none of his own in reach, then the shout, the floor's clock and the cue. None = he is getting it out; else why not.   (frag.ts windUp)
pub fn wind_up(g: &mut crate::game::Game, c_2: usize, head: V3, from: V3, aim: V3, toward: Option<V3>) -> Option<crate::frag::WindUp> {
    let gd = car!(g, c_2).gd;
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let k = book!(g).serial + 1.0;
    let r = v3::lerp(g.k.frag.def_scatter[0], g.k.frag.def_scatter[1], hash01(id, 31.0 * k));
    let a = hash01(id + 5.0, 17.0 * k) * m::PI * 2.0;
    let mut land: V3 = [aim[0] + m::sin(a) * r, 0.0, aim[2] + m::cos(a) * r];
    if crate::scene_collision::nav_grid_is_blocked(g, land[0], land[2]) { land = aim; }
    let mut sol = crate::throwables::throwables_solve(g, from, land, Some(11.0));
    if !lands(g, Some(crate::frag::LandsSol { land: sol.land }), land, 1.6) { sol = crate::throwables::throwables_solve(g, from, aim, Some(11.0)); }
    if !own_clear(g, gd, sol.land, None) { return Some(crate::frag::WindUp::Own); }
    let now_t = g.time;
    let wind = crate::frag::CarrierWind { until: now_t + g.k.frag.def_wind, at: sol.land, swung: false, last_t: book!(g).last_t, hand: if toward.is_some() { Some(from) } else { None }, toward };
    car!(g, c_2).wind = Some(wind);
    book!(g).serial = k;
    book!(g).last_t = now_t;
    car!(g, c_2).blocked_t = 0.0;
    car!(g, c_2).bunch_t = 0.0;
    let all = shouts();
    let now: Vec<String> = all[1..].to_vec();
    let said = crate::words::w(g, &now, &all);
    let len = said.len() as f64;
    let pick = js::rem(m::floor(hash01(id, k) * len), len) as usize;
    crate::game::game_say(g, gd, said[pick].as_str(), None, None, None);
    let mut heard = false;
    for pl in crate::seats::men_up(g) {
        let ph = head_of(g, g.pl[pl].ch);
        let pr = crate::scene_collision::sound_paths_propagate(g, head, ph, None);
        if pr.path_len <= g.k.frag.cue_m * 1.5 && pr.muffle < 0.75 { heard = true; break; }
    }
    if heard {
        crate::game::game_msg(g, "they're shouting a warning. a grenade is coming");
        g.stats.frag.cues += 1;
    }
    let team = g.gd[gd].team;
    let mut n = 0;
    while n < g.guards.len() {   // the team's, the same shout in the same ears: for the next few seconds they are looking for it
        let o = g.guards[n]; n += 1;
        if g.gd[o].state == crate::game::GuardState::Dead || g.gd[o].team == team || same_side(g.gd[o].team, team) { continue; }
        let oh = head_of(g, g.gd[o].ch);
        let po = crate::scene_collision::sound_paths_propagate(g, head, oh, None);
        if po.path_len <= g.k.frag.cue_m * 1.5 && po.muffle < 0.75 {
            let until = g.time + g.k.frag.def_wind + g.k.frag.def_fuse + 1.0;
            g.m.frag.warned.set(o, until);
        }
    }
    None
}

/// The grenade of a throw under way is still in his fist: swung, not yet let go.   (frag.ts fragInFist)
pub fn frag_in_fist(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(b) = g.m.frag.def_books.as_ref() else { return false };
    b.carriers.iter().find(|c| c.gd == gd).and_then(|c| c.wind).map_or(false, |w| w.swung)
}

/// An arm round a corner: the hand half a pace from a man at `p` toward `to`, chest high; no arm goes through a wall.   (frag.ts armRound)
pub fn arm_round(g: &mut crate::game::Game, p: V3, to: V3) -> V3 {
    let (dx, dz) = (to[0] - p[0], to[2] - p[2]);
    let d = js::or(m::hyp2(dx, dz), 1.0);
    let out = m::min(0.7, d);
    let from: V3 = [p[0] + dx / d * out, p[1] + 1.5, p[2] + dz / d * out];
    let chest: V3 = [p[0], p[1] + 1.5, p[2]];
    if crate::scene_collision::solid_blocked(g, chest, from, None) { chest } else { from }
}

/// The place in the book of the carrier who may start a throw this moment (the book is `g.m.frag.def_books`).   (frag.ts free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> Option<usize> {
    if !g.ai_enabled || !crate::roomfight::fight_live(g) || crate::director::director_their_frags_off(g) { return None; }
    let b = g.m.frag.def_books.as_ref()?;
    let ci = b.carriers.iter().position(|x| x.gd == gd)?;
    if !js::truthy(b.carriers[ci].frags) || b.carriers[ci].wind.is_some() { return None; }
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || crate::runner::runner_owns(g, gd) || g.time < g.gd[gd].stagger_until || g.time < g.gd[gd].dazzled_until || frag_flee(g, gd).is_some() || g.time - book!(g).last_t < g.k.frag.def_gap { return None; }
    Some(ci)
}

/// May he start a throw at the place `at` this moment: everything `frag_throw_at` asks but the arc and his own side.   (frag.ts fragReady)
pub fn frag_ready(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool {
    free(g, gd).is_some() && !hostage_near(g, at)
}

/// A throw another rule asks for: only WHERE is the caller's.   (frag.ts fragThrowAt)
pub fn frag_throw_at(g: &mut crate::game::Game, gd: GuardId, at: V3, stand: V3, toward: V3) -> crate::frag::FragThrowAt {
    use crate::frag::FragThrowAt as R;
    let Some(ci) = free(g, gd) else { return R::Busy };
    if hostage_near(g, at) { return R::Busy; }
    let c = g.gd[gd].ch;
    let d = flat(stand, at);
    if d < g.k.frag.def_near || d > g.k.frag.def_far { return R::Arc; }
    let pos = g.ch[c].pos;
    let time = g.time;
    let s = match car!(g, ci).spots.clone() {
        Some(s) if !(time - s.t > 1.0 || flat(s.at, at) > 0.1 || flat(s.from, pos) > 1.5) => s,
        _ => {
            let list = aim_spots(g, gd, None, at, 17.0);
            let s = crate::frag::CarrierSpots { at, from: pos, t: time, list };
            car!(g, ci).spots = Some(s.clone());
            s
        }
    };
    let hand = arm_round(g, stand, toward);
    let mut aim = None;
    for &p in &s.list {
        let sol = crate::throwables::throwables_solve(g, hand, p, Some(11.0));
        if lands(g, Some(crate::frag::LandsSol { land: sol.land }), p, 1.0) { aim = Some(p); break; }
    }
    let Some(aim) = aim else { return R::Arc };
    let head = head_of(g, c);
    match wind_up(g, ci, head, hand, aim, Some(toward)) {
        None => R::Thrown,
        Some(crate::frag::WindUp::Arc) => R::Arc,
        Some(crate::frag::WindUp::Own) => R::Own,
    }
}

/// He has shouted and the grenade is not yet out of his hand.   (frag.ts fragWinding)
pub fn frag_winding(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.frag.def_books.as_ref().map_or(false, |b| b.carriers.iter().any(|x| x.gd == gd && x.wind.is_some()))
}

/// How many grenades have left his hand.   (frag.ts fragAway)
pub fn frag_away(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    g.m.frag.def_books.as_ref().and_then(|b| b.carriers.iter().find(|x| x.gd == gd)).and_then(|x| x.away).unwrap_or(0.0)
}

/// The arc comes DOWN ON THE FLOOR within `tol` of where he means it.   (frag.ts lands)
pub fn lands(g: &mut crate::game::Game, sol: Option<crate::frag::LandsSol>, p: V3, tol: f64) -> bool {
    match sol { Some(s) => s.land[1] <= g.k.frag.lift + 0.1 && flat(s.land, p) <= tol, None => false }
}

/// A live hostage in what a grenade there would reach.   (frag.ts hostageNear)
pub fn hostage_near(g: &mut crate::game::Game, p: V3) -> bool {
    let reach = g.k.frag.wound_m + g.k.frag.def_scatter[1];
    g.rescue.hostages.iter().any(|&h| g.gd[h].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[h].ch].pos, p) < reach)
}

/// Nobody of the thrower's own side within defOwnM of where it would land with a line to it, and he not within his own arm's error.   (frag.ts ownClear)
pub fn own_clear(g: &mut crate::game::Game, gd: GuardId, land: V3, release: Option<bool>) -> bool {
    let release = release.unwrap_or(false);
    let from: V3 = [land[0], land[1] + g.k.frag.lift, land[2]];
    let team = g.gd[gd].team;
    let mut n = 0;
    while n < g.guards.len() {
        let o = g.guards[n]; n += 1;
        if g.gd[o].team != team { continue; }
        if g.gd[o].state == crate::game::GuardState::Dead {   // a mate down and breathing is still one of his
            if down_asked(g, o) {
                let b = lying(g, o);
                if flat(b[0], land) <= g.k.frag.def_own_m && lying_shows(g, from, &b, crate::frag::GlassView::Wary) > 0.0 { return false; }
            }
            continue;
        }
        let oc = g.gd[o].ch;
        let op = g.ch[oc].pos;
        let od = flat(op, land);
        if od > g.k.frag.def_own_m { continue; }
        if o == gd && release {
            if od < g.k.frag.def_near - 1.5 { return false; }
            continue;
        }
        if o == gd && od < g.k.frag.def_near - 0.5 { return false; }
        let oh = head_of(g, oc);
        if exposed_to(g, from, op, oh, None) > 0.0 { return false; }
    }
    true
}
