//! src/game/morale.ts lines 452 to 604
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The row as it stands now, for reading: every read comes fresh from the table, every write goes through it. `step` and `drive` are called on a man who has one.
fn row(g: &mut Game, gd: GuardId) -> Mind {
    *g.m.morale.state_of.get(gd).unwrap()
}

/// Where a man's body stands.
fn pos_of(g: &mut Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].pos
}

/// His men whose duck waits on his order, nearest first. (the closure `his`, morale.ts:473)
fn his(g: &mut Game, gd: GuardId) -> Vec<GuardId> {
    let mut men: Vec<GuardId> = Vec::new();
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if let Some(mm) = g.m.morale.state_of.get(o) {
            if mm.by == Some(gd) && mm.act == MoraleAct::Duck { men.push(o); }
        }
    }
    let at = pos_of(g, gd);
    let mut keyed: Vec<(GuardId, V3)> = Vec::new();
    for &o in men.iter() { let p = pos_of(g, o); keyed.push((o, p)); }
    js::sort_by(&mut keyed, |a, b| flat(a.1, at) - flat(b.1, at));
    keyed.into_iter().map(|x| x.0).collect()
}

/// one frame of a man's sequence: decides the act, fires its rounds, says its lines, and whether this file's script has him this frame (morale.ts step)
pub fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    if !reacts(g, gd) || row(g, gd).hole.is_none() { end(g, gd); return; }
    if g.gd[gd].script.is_none() && g.gd[gd].state != crate::game::GuardState::Alert && g.gd[gd].awareness >= ALERT_AW { return; }
    let hole = row(g, gd).hole.unwrap();
    {
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        if st.calls > 0.0 { st.call_t += dt; }
    }
    let rooted = g.time < g.gd[gd].stagger_until;
    let act = row(g, gd).act;
    if act == MoraleAct::Duck {
        {
            let st = g.m.morale.state_of.get_mut(gd).unwrap();
            if st.heard >= 0.0 { st.heard += dt; }
        }
        if !rooted {
            g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
            let now = row(g, gd);
            if now.heard >= g.k.morale.order_secs + now.next {
                if now.said { bark(g, gd, LineKind::Okay, false, Some(true)); }
                end(g, gd);
                return;
            }
            let lasts = if folds(g, gd) { g.k.morale.duck_secs } else { g.k.morale.steady_duck };
            if now.t >= lasts || !seen_by(g, gd, None).is_empty() { end(g, gd); return; }
        }
        drive(g, gd, dt, rooted);
        return;
    }
    if act == MoraleAct::Flinch {
        if !rooted {
            g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
            if row(g, gd).t >= g.k.morale.flinch_secs { end(g, gd); return; }
        }
        drive(g, gd, dt, rooted);
        return;
    }
    if act == MoraleAct::Stand {
        if !rooted {
            if row(g, gd).t == 0.0 {
                let to = his(g, gd).first().copied();
                let goal = side_goal(g, gd, hole, to);
                let st = g.m.morale.state_of.get_mut(gd).unwrap();
                st.goal = goal; st.script.arrived = Some(false);
            }
            g.m.morale.state_of.get_mut(gd).unwrap().t += dt;
            if row(g, gd).t >= g.k.morale.stand_secs || !seen_by(g, gd, None).is_empty() { end(g, gd); return; }
            {
                let st = g.m.morale.state_of.get_mut(gd).unwrap();
                if st.goal.is_some() && st.script.arrived == Some(true) { st.goal = None; }
            }
            let now = row(g, gd);
            if !now.said && now.t >= g.k.morale.order_beat {
                g.m.morale.state_of.get_mut(gd).unwrap().said = true;
                let men = his(g, gd);
                if !men.is_empty() {
                    bark(g, gd, LineKind::Order, false, Some(true));
                    g.stats.morale.orders += 1;
                    let gap = g.k.morale.order_gap;
                    for (i, &o) in men.iter().enumerate() {
                        let mm = g.m.morale.state_of.get_mut(o).unwrap();
                        mm.heard = 0.0; mm.next = (i as f64) * gap; mm.said = i == 0;
                    }
                }
            }
        }
        drive(g, gd, dt, rooted);
        return;
    }
    let owned = act == MoraleAct::Bait || act == MoraleAct::Reload;
    if row(g, gd).rung < 2.0 && !owned { end(g, gd); return; }
    if rooted { drive(g, gd, dt, true); return; }
    let seen: Vec<CharId> = if owned { Vec::new() } else { seen_by(g, gd, if act == MoraleAct::Hold { Some(hole) } else { None }) };
    if !seen.is_empty() && act != MoraleAct::Hold { end(g, gd); return; }
    {
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        st.t += dt; st.broke_t += dt;
    }
    if act == MoraleAct::Hold && row(g, gd).broke_t >= g.k.morale.hold_max { end(g, gd); return; }
    match act {
        MoraleAct::Down => {
            if row(g, gd).t >= g.k.morale.down_secs {
                let t = row(g, gd).t;
                begin(g, gd, MoraleAct::Back, Some(hole));
                g.m.morale.state_of.get_mut(gd).unwrap().t = t;
                let goal = back_goal(g, gd, hole);
                {
                    let st = g.m.morale.state_of.get_mut(gd).unwrap();
                    st.goal = goal; st.script.arrived = Some(false);
                }
                let now = row(g, gd);
                if now.calls == 0.0 {
                    if !folds(g, gd) { bark(g, gd, LineKind::Clear, true, Some(true)); }
                    else if now.alone { bark(g, gd, LineKind::Call, true, Some(true)); }
                    let st = g.m.morale.state_of.get_mut(gd).unwrap();
                    st.calls = 1.0; st.call_t = 0.0;
                }
            }
        }
        MoraleAct::Back => {
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
        MoraleAct::Burst => {
            let every = g.k.morale.burst_every;
            let over = !round(g, gd, dt, every, RoundOf::Dust)
                || row(g, gd).t > g.k.morale.burst_rounds * g.k.morale.burst_every + 2.0;
            if over { begin(g, gd, MoraleAct::Hold, Some(hole)); }
        }
        MoraleAct::Hold => {
            let now = row(g, gd);
            if now.calls == 1.0 && now.call_t >= g.k.morale.call2_secs && now.alone && folds(g, gd) {
                bark(g, gd, LineKind::Again, true, Some(true));
                g.m.morale.state_of.get_mut(gd).unwrap().calls = 2.0;
            }
            if !seen.is_empty() {
                let now = row(g, gd);
                let bait_m = g.k.morale.bait_m;
                let at_it = now.baits == 0.0 && seen.iter().any(|&c| flat(g.ch[c].pos, hole) <= bait_m);
                if at_it {
                    bait(g, gd);
                    g.m.morale.state_of.get_mut(gd).unwrap().at_hole = true;
                } else {
                    end(g, gd);
                    return;
                }
            } else {
                let at = thrown(g, gd, hole);
                let now = row(g, gd);
                if at && !now.at_hole { bait(g, gd); }
                g.m.morale.state_of.get_mut(gd).unwrap().at_hole = at;
            }
        }
        MoraleAct::Bait => {
            let every = g.k.morale.bait_every;
            let mut over = !round(g, gd, dt, every, RoundOf::Bait);
            if !over {
                let w = g.fa[g.gd[gd].weapon].def;
                let cap = g.m.weapons.weapons.get(w).unwrap().mag_capacity;
                over = row(g, gd).t > (cap + 1.0) * g.k.morale.bait_every + 2.0;
            }
            if over {
                if row(g, gd).baits == 1.0 && folds(g, gd) { run_dry(g, gd); }
                else { begin(g, gd, MoraleAct::Hold, Some(hole)); }
            }
        }
        MoraleAct::Reload => {
            if g.gd[gd].reload_t < 0.0 { begin(g, gd, MoraleAct::Hold, Some(hole)); }
        }
        _ => {}
    }
    drive(g, gd, dt, false);
}

/// this file's script has him this frame: posed, gun out, ears open, a magazine change run on, his reaction beat run down (morale.ts drive)
pub fn drive(g: &mut crate::game::Game, gd: GuardId, dt: f64, rooted: bool) {
    pose(g, gd, rooted);
    let time = g.time;
    {
        let st = g.m.morale.state_of.get_mut(gd).unwrap();
        st.drive = true; st.drive_t = time;
    }
    g.gd[gd].drawn = true;
    crate::guards::hearing_check(g, gd, None, None, None);
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
    if g.gd[gd].reload_t >= 0.0 { crate::guards::tick_reload_beat(g, gd, dt); }
    if g.gd[gd].react_t > 0.0 { g.gd[gd].react_t -= dt; }
}

/// THE BODY'S SEAM (guards.ts updateGuard): the script his body follows this frame, None = he is nobody's but his own (morale.ts moraleScript)
pub fn morale_script(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> {
    let s = g.m.morale.state_of.get(gd).copied()?;
    if !s.drive || s.drive_t != g.time { return None; }
    if s.act == MoraleAct::Down && folds(g, gd) {
        let c = g.gd[gd].ch;
        g.ch[c].anim.clutch = 1.0;
    }
    Some(s.script)
}

/// has a sequence of this file's got him, as of the last tick (morale.ts moraleOwns)
pub fn morale_owns(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.m.morale.state_of.get(gd) { Some(s) => s.drive, None => false }
}

/// his dread, 0 to 1 (0 for a man never met) (morale.ts dreadOf)
pub fn dread_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd).copied() { Some(s) => dread(g, gd), None => 0.0 }
}

/// his dread while an event's shock is still on him, 0 otherwise (morale.ts shakenOf)
pub fn shaken_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd).copied() {
        Some(s) if s.shock > 0.0 => dread(g, gd),
        _ => 0.0,
    }
}

/// his rung: 0 steady, 1 jumpy, 2 broken (morale.ts rungOf)
pub fn rung_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.morale.state_of.get(gd) { Some(s) => s.rung, None => 0.0 }
}

/// QA: what the rule has of a man, None = never met (morale.ts moraleOf)
pub fn morale_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::morale::MoraleOf> {
    let s = g.m.morale.state_of.get(gd)?;
    Some(MoraleOf {
        drive: s.drive, heard: s.heard, t: s.t, lost: s.lost, next: s.next, hole: s.hole, goal: s.goal, act: s.act, lit: s.lit, said: s.said,
        alone: s.alone, ask: s.ask, by: s.by, green: s.green, dark: s.dark, rounds: s.rounds, fall: s.fall, drive_t: s.drive_t, calls: s.calls,
        shock: s.shock, rung: s.rung, leader_down: s.leader_down, broke_t: s.broke_t, call_t: s.call_t, baits: s.baits, at_hole: s.at_hole,
        bark: s.bark, shout: s.shout,
    })
}

/// `x.toFixed(2)` of a number at or above 0. Only a multiple of 1/8 sits exactly on a tie at two places: JavaScript takes the larger there, Rust the even.
fn fixed2(x: f64) -> String {
    let e = x * 8.0;
    if e == m::floor(e) && js::rem(e, 2.0) != 0.0 { format!("{:.2}", x + 0.001) } else { format!("{:.2}", x) }
}

/// 'morale ▸ Tango 2 0.85 BROKEN hold (alone, dark) · Tango 4 0.35 jumpy duck': every living man with anything on him (morale.ts moraleSummary)
pub fn morale_summary(g: &mut crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !ours(g, gd) { continue; }
        let Some(s) = g.m.morale.state_of.get(gd).copied() else { continue };
        if !(s.rung > 0.0 || s.act != MoraleAct::None || s.shock > 0.005) { continue; }
        let d = dread(g, gd);
        let word = if s.rung == 2.0 { "BROKEN" } else if s.rung == 1.0 { "jumpy" } else { "steady" };
        let mut line = format!("{} {} {}", g.gd[gd].callsign, fixed2(d), word);
        if s.act != MoraleAct::None { line.push(' '); line.push_str(s.act.as_str()); }
        if s.alone || s.dark || s.leader_down {
            let mut why: Vec<&str> = Vec::new();
            if s.alone { why.push("alone"); }
            if s.dark { why.push("dark"); }
            if s.leader_down { why.push("leader down"); }
            line.push_str(" (");
            line.push_str(&why.join(", "));
            line.push(')');
        }
        parts.push(line);
    }
    let body = parts.join(" · ");
    format!("morale ▸ {}", if body.is_empty() { "every man steady" } else { body.as_str() })
}

/// 'men ▸ S g g g · S g g': who is steady and who green, a band at a time, the living only; men with no band as one last group (morale.ts moraleRoster)
pub fn morale_roster(g: &mut crate::game::Game) -> String {
    let mut groups: Vec<Vec<GuardId>> = Vec::new();
    let mut loose: Vec<GuardId> = Vec::new();
    let mut seen: Vec<Vec<GuardId>> = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !ours(g, gd) || rescued(g, gd) { continue; }
        match band_of(g, gd) {
            None => loose.push(gd),
            Some(b) => {
                if !seen.contains(&b) {
                    let mut living: Vec<GuardId> = Vec::new();
                    for &o in b.iter() { if ours(g, o) { living.push(o); } }
                    seen.push(b);
                    groups.push(living);
                }
            }
        }
    }
    if !loose.is_empty() { groups.push(loose); }
    let mut shown: Vec<String> = Vec::new();
    for b in groups.iter() {
        let mut marks: Vec<&str> = Vec::new();
        for &o in b.iter() { marks.push(if training_of(g, o) == TrainingOf::Steady { "S" } else { "g" }); }
        shown.push(marks.join(" "));
    }
    let body = shown.join(" · ");
    format!("men ▸ {}", if body.is_empty() { "nobody" } else { body.as_str() })
}

/// HOST WORK, the tab's sessionStorage: not translated (conventions 6). Only the host's address reader reaches it. (morale.ts kept)
pub fn kept() -> crate::morale::Kept {
    todo!("host work: the browser's sessionStorage, read by the host")
}

/// HOST WORK, the tab's sessionStorage: not translated (conventions 6). Only the debugger's panel (ui/bindings.ts) calls it. (morale.ts moraleKeep)
pub fn morale_keep(g: &mut crate::game::Game, row: crate::morale::MoraleKeepRow) {
    todo!("host work: the browser's sessionStorage, written by the host")
}

/// HOST WORK, the address and the tab's store: not translated (conventions 6). Only host/profile.ts calls it; `q` is `Opaque`. (morale.ts moraleFromQuery)
pub fn morale_from_query(g: &mut crate::game::Game, q: crate::Opaque) {
    todo!("host work: the host reads the address and sets g.k.morale")
}

/// runner.ts: the man is that file's from here: whatever sequence has him ends where it stands; dread and rung stay (morale.ts moraleRelease)
pub fn morale_release(g: &mut crate::game::Game, gd: GuardId) {
    if let Some(s) = g.m.morale.state_of.get(gd).copied() {
        if s.act != MoraleAct::None || s.drive { end(g, gd); }
    }
}

/// A position as three numbers, NaN for none. (morale.ts n3)
pub fn n3(p: Option<V3>) -> Vec<f64> {
    match p { Some(p) => vec![p[0], p[1], p[2]], None => vec![f64::NAN, f64::NAN, f64::NAN] }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected strings are Number.prototype.toFixed(2) as printed by the JavaScript engine.
    #[test]
    fn fixed2_matches_to_fixed() {
        for (x, want) in [(0.0, "0.00"), (1.0, "1.00"), (0.125, "0.13"), (0.375, "0.38"), (0.625, "0.63"), (0.875, "0.88"), (0.25, "0.25"), (0.85, "0.85"), (0.005, "0.01"), (0.015, "0.01"), (0.345, "0.34")] {
            assert_eq!(fixed2(x), want, "{}", x);
        }
    }
}
