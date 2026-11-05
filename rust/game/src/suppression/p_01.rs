//! src/game/suppression.ts lines 1 to 325
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Everything the reactions keep, back to rest; the scalar itself stays.   (suppression.ts Sup.clearReactions)
pub fn sup_clear_reactions(this: &mut crate::suppression::Sup) {
    this.down_t = 0.0;
    this.latched = false;
    this.pinned_t = 0.0;
    this.cover = None;
    this.cover_until = -1.0;
}

/// How readily this body accrues suppression, by his temperament.   (suppression.ts keenOf)
pub fn keen_of(g: &mut crate::game::Game, c_2: CharId) -> f64 {
    if c_2 == g.pl[g.player].ch { return 1.0; }
    let mut t: Option<String> = None;
    for &x in &g.guards {
        if g.gd[x].ch == c_2 {
            t = crate::game::temperament().get(g.gd[x].callsign.clone()).cloned();
            break;
        }
    }
    match t {
        Some(t) => match g.k.suppression.keen.get(t) {
            Some(&k) => js::or(k, 1.0),
            None => 1.0,
        },
        None => 1.0,
    }
}

/// One round has just been traced: every OTHER living, free man it passed close to or struck close to accrues.   (suppression.ts noteRound)
pub fn note_round(g: &mut crate::game::Game, shooter: CharId, from: V3, to: V3, hit: Option<CharId>) {
    let seg = v3::sub(to, from);
    let t_hit = v3::len(seg);
    if t_hit < 1e-3 { return; }
    let dir = v3::scale(seg, 1.0 / t_hit);
    let shooter_team = crate::teams::team_of_char(g, shooter);
    let round = Round { shooter, hit, from, to, dir, t_hit, shooter_team };
    for pl in all_men(g) {
        let c = g.pl[pl].ch;
        note_one(g, &round, c, crate::condition::Man::Player(pl), false, crate::teams::Team::Player);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state != crate::game::GuardState::Dead {
            let (c, held, team) = (g.gd[gd].ch, g.gd[gd].held.is_some(), g.gd[gd].team);
            note_one(g, &round, c, crate::condition::Man::Guard(gd), held, team);
        }
    }
}

/// What `noteRound`'s closure captured.
struct Round {
    shooter: CharId,
    hit: Option<CharId>,
    from: V3,
    to: V3,
    dir: V3,
    t_hit: f64,
    shooter_team: crate::teams::Team,
}

/// One man's share of one round.   (suppression.ts noteRound's `each`)
fn note_one(g: &mut Game, r: &Round, c: CharId, who: crate::condition::Man, held: bool, team: crate::teams::Team) {
    let gd = match who { crate::condition::Man::Guard(x) => Some(x), crate::condition::Man::Player(_) => None };
    if c == r.shooter || Some(c) == r.hit || !g.ch[c].alive || held { return; }
    let chest = match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(&b) => b,
        None => v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]),
    };
    let s = v3::clamp(v3::dot(v3::sub(chest, r.from), r.dir), 0.0, r.t_hit);
    let foe = crate::teams::is_hostile(g, r.shooter_team, team);
    let min_travel = g.k.suppression.min_travel;
    if s < min_travel && !(foe && crate::flinch::flinch_inside(g, s)) { return; }
    let closest = v3::mad(r.from, r.dir, s);
    let d_near = v3::dist(closest, chest);
    let d_hit = v3::dist(r.to, chest);
    let mut add = g.k.suppression.w_near * m::max(0.0, 1.0 - d_near / g.k.suppression.near_r)
        + g.k.suppression.w_hit * m::max(0.0, 1.0 - d_hit / g.k.suppression.hit_r);
    if s < min_travel {
        crate::flinch::flinch_round(g, c, gd, add, r.from, true);
        return;
    }
    if add <= 0.0 { return; }
    crate::flinch::flinch_round(g, c, gd, add, r.from, foe);
    let sup = match who {
        crate::condition::Man::Player(p) => g.pl[p].sup,
        crate::condition::Man::Guard(x) => g.gd[x].sup,
    };
    if sup.down_t > 0.0 || sup.latched || sup.cover.is_some() { add *= g.k.suppression.down_accrual; }
    if !foe { add *= g.k.suppression.friendly_k; }
    add *= keen_of(g, c);
    let (cap, shooter_id, now) = (g.k.suppression.cap, g.ch[r.shooter].id, g.time);
    let sup = match who {
        crate::condition::Man::Player(p) => &mut g.pl[p].sup,
        crate::condition::Man::Guard(x) => &mut g.gd[x].sup,
    };
    sup.v = m::min(cap, sup.v + add);
    sup.kick = m::max(sup.kick, add);
    sup.last_add = add;
    sup.src = Some(v3::copy(r.from));
    sup.src_id = shooter_id;
    sup.src_t = now;
}

/// A checkpoint loaded puts the time back: the floor's clocks must not sit in the future.   (suppression.ts forgetSuppressionClocks)
pub fn forget_suppression_clocks(g: &mut crate::game::Game) {
    g.m.suppression.floor_line_t = None;
    g.m.suppression.floor_bark_t = None;
}

/// Which held-back line outranks which.   (suppression.ts RANK)
fn rank(moment: crate::suppression::SupMoment) -> i32 {
    match moment {
        SupMoment::Recover => 0,
        SupMoment::Friendly => 1,
        SupMoment::Flinch => 1,
        SupMoment::Pinned => 2,
        SupMoment::Displace => 3,
    }
}

/// The lines, by moment and temperament ('*' for anybody), in the written order.   (suppression.ts SUP_LINES)
static SUP_LINES: [(SupMoment, &[(&str, &[&str])]); 5] = [
    (SupMoment::Flinch, &[
        ("*", &["taking fire!", "rounds coming in!", "he's shooting — get down!", "that was close—"]),
        ("nervy", &["christ— CHRIST—", "he's shooting at me!"]),
        ("hard", &["he's found his nerve.", "incoming. stay on him."]),
    ]),
    (SupMoment::Friendly, &[
        ("*", &["watch your fire!", "check fire — that's past MY head!", "easy! I'm in front of you!"]),
        ("nervy", &["stop— stop shooting past me!"]),
    ]),
    (SupMoment::Pinned, &[
        ("*", &["I'm pinned!", "can't get my head up—", "he's got me pinned here!", "head down— head down!"]),
        ("nervy", &["I can't— I can't move—", "jesus, he's right on me—"]),
        ("hard", &["he's got the angle. wait him out.", "let him spend them."]),
    ]),
    (SupMoment::Displace, &[
        ("*", &["moving!", "shifting — cover me!", "can't stay here — moving!"]),
        ("nervy", &["I'm getting out of here—"]),
        ("hard", &["relocating."]),
    ]),
    (SupMoment::Recover, &[
        ("hard", &["that all you've got?", "my turn."]),
    ]),
];

/// One moment's row of the lines, by key; none = nothing.
fn lines_of(moment: SupMoment, key: &str) -> &'static [&'static str] {
    for row in SUP_LINES.iter() {
        if row.0 == moment {
            for pool in row.1.iter() {
                if pool.0 == key { return pool.1; }
            }
        }
    }
    &[]
}

/// A byte JavaScript's `\w` holds (every byte of a non-ASCII character is outside it, as a non-unicode regex sees that character).
fn is_word(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' }

/// `/\b(he|he's|him|his)\b/.test(l)`.   (suppression.ts:169)
fn mentions_him(l: &str) -> bool {
    let b = l.as_bytes();
    for w in ["he", "he's", "him", "his"] {
        let w = w.as_bytes();
        let mut at = 0;
        while at + w.len() <= b.len() {
            if &b[at..at + w.len()] == w
                && (at == 0 || !is_word(b[at - 1]))
                && (at + w.len() == b.len() || !is_word(b[at + w.len()]))
            {
                return true;
            }
            at += 1;
        }
    }
    false
}

/// A teammate with the player's round going past him says what a man says of a colleague's.   (suppression.ts watchYourFire)
pub fn watch_your_fire(g: &mut crate::game::Game, gd: GuardId) -> bool {
    sup_bark(g, gd, SupMoment::Friendly, None)
}

/// One suppression line from this man if the clocks allow; a line held back is kept `pending`.   (suppression.ts supBark)
pub fn sup_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::suppression::SupMoment, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    let now = g.time;
    if crate::runner::runner_mute(g, gd) { return false; }
    let floor_bark = g.m.suppression.floor_bark_t.unwrap_or(-100.0);
    let held = (!force && (now - g.gd[gd].sup.bark_t < g.k.suppression.bark_secs || now - floor_bark < g.k.suppression.floor_bark_secs))
        || match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 2.5, None => false };
    if held {
        let sup = &mut g.gd[gd].sup;
        if moment != SupMoment::Recover && rank(moment) > (match sup.pending { Some(p) => rank(p), None => -1 }) {
            sup.pending = Some(moment);
            sup.pending_t = now;
        }
        return false;
    }
    let temperament = crate::dialogue::temperament_of(g, gd);
    let mut pool: Vec<&'static str> = Vec::new();
    pool.extend_from_slice(lines_of(moment, temperament.as_str()));
    pool.extend_from_slice(lines_of(moment, "*"));
    if g.gd[gd].team != crate::teams::Team::Guard {
        pool.retain(|l| !mentions_him(l));
    }
    if pool.is_empty() { return false; }
    let n = pool.len() as f64;
    let mut i = js::rem(m::floor(crate::math_dice::sim_random(g) * n), n) as usize;
    if pool.len() > 1 && g.m.suppression.last_line.get(gd).map(|l| l.as_str()) == Some(pool[i]) {
        i = (i + 1) % pool.len();
    }
    g.m.suppression.last_line.set(gd, String::from(pool[i]));
    let sup = &mut g.gd[gd].sup;
    sup.bark_t = now;
    sup.bark_was = Some(moment);
    sup.pending = None;
    g.m.suppression.floor_bark_t = Some(now);
    crate::game::game_say(g, gd, pool[i], None, None, None);
    true
}

/// The suppression lines, every moment's or one moment's.   (suppression.ts suppressionLines)
pub fn suppression_lines(moment: Option<crate::suppression::SupMoment>) -> Vec<String> {
    let mut out = Vec::new();
    for row in SUP_LINES.iter() {
        if let Some(wanted) = moment {
            if row.0 != wanted { continue; }
        }
        for pool in row.1.iter() {
            for l in pool.1.iter() {
                out.push(String::from(*l));
            }
        }
    }
    out
}

/// May this man's behaviour react at all this frame.   (suppression.ts reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.tune.suppression && g.ai_enabled && g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].held.is_none() && g.gd[gd].script.is_none()
}

/// Once a frame's decay of one man's scalar and pulse.   (suppression.ts updateSuppression's `decay`)
fn decay(sup: &mut Sup, k: f64, kk: f64) {
    sup.v *= k;
    sup.kick *= kk;
    if sup.v < 1e-5 { sup.v = 0.0; }
    if sup.kick < 1e-5 { sup.kick = 0.0; }
}

/// His band, with hysteresis: a scalar hovering about a line under a steady volley is ONE episode.   (suppression.ts:196)
fn band_of(v: f64, was: f64, pinned: f64, release: f64, flinch: f64) -> f64 {
    if v >= pinned || (was == 2.0 && v >= release) {
        2.0
    } else if v >= flinch || (was >= 1.0 && v >= flinch - 0.05) {
        1.0
    } else {
        0.0
    }
}

/// Where his start jolts him from: the muzzle of the last round, off his own feet.   (suppression.ts:197, :201)
fn start_from_round(g: &mut Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    let toward = match g.gd[gd].sup.src { Some(src) => Some(v3::sub(src, g.ch[c].pos)), None => None };
    let scale = crate::flinch::flinch_scale(g, Some(gd));
    crate::character::anim_flinch_from(g, c, toward, Some(scale), None);
}

/// Once a frame, ahead of the player and the guards: decay everybody's scalar and pulse; the guards' edges.   (suppression.ts updateSuppression)
pub fn update_suppression(g: &mut crate::game::Game, dt: f64) {
    let k = m::pow(2.0, -dt / g.k.suppression.half_life);
    let kk = m::exp(-g.k.suppression.kick_decay * dt);
    for pl in all_men(g) {
        let down = g.pl[pl].down;
        let p = &mut g.pl[pl].sup;
        if down { p.v = 0.0; p.kick = 0.0; } else { decay(p, k, kk); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() {
            let sup = &mut g.gd[gd].sup;
            sup.v = 0.0;
            sup.kick = 0.0;
            sup.band = 0.0;
            sup.was_pinned = false;
            sup_clear_reactions(sup);
            continue;
        }
        decay(&mut g.gd[gd].sup, k, kk);
        if g.gd[gd].state != crate::game::GuardState::Alert || !reacts(g, gd) { sup_clear_reactions(&mut g.gd[gd].sup); }
        let v = g.gd[gd].sup.v;
        let was = g.gd[gd].sup.band;
        let band = band_of(v, was, g.k.suppression.pinned, g.k.suppression.release, g.k.suppression.flinch);
        if band > was && was < 1.0 && !reacts(g, gd)
            && g.gd[gd].script.as_ref().map_or(false, |s| s.flinch == Some(true))
            && g.tune.suppression
        {
            start_from_round(g, gd);
        }
        if band > was && reacts(g, gd) {
            let src_id = g.gd[gd].sup.src_id;
            let friendly = !crate::seats::is_man_id(g, src_id) && src_id >= 0.0 && {
                let team = g.gd[gd].team;
                let theirs = crate::teams::team_of_id(g, src_id);
                !crate::teams::is_hostile(g, team, theirs)
            };
            if was < 1.0 {
                g.gd[gd].sup.flinch_at = g.time;
                start_from_round(g, gd);
                g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, g.k.suppression.flinch_delay);
                if band < 2.0 {
                    sup_bark(g, gd, if friendly { SupMoment::Friendly } else { SupMoment::Flinch }, None);
                }
            }
            if band == 2.0 {
                g.gd[gd].sup.pinned_at = g.time;
                g.gd[gd].sup.was_pinned = true;
                let mut other = false;
                let mut o_i = 0;
                while o_i < g.guards.len() {
                    let o = g.guards[o_i];
                    o_i += 1;
                    if o != gd
                        && g.gd[o].state != crate::game::GuardState::Dead
                        && g.time - g.gd[o].sup.pinned_at < g.k.suppression.pair_secs
                        && g.gd[o].team == g.gd[gd].team
                        && !crate::runner::runner_mute(g, o)
                    {
                        other = true;
                        break;
                    }
                }
                if other && !crate::runner::runner_mute(g, gd)
                    && g.time - g.m.suppression.floor_line_t.unwrap_or(-100.0) > g.k.suppression.floor_line_secs
                {
                    let now = g.time;
                    g.m.suppression.floor_line_t = Some(now);
                    g.gd[gd].sup.bark_t = now;
                    g.gd[gd].sup.bark_was = Some(SupMoment::Pinned);
                    g.m.suppression.floor_bark_t = Some(now);
                    let line = if g.gd[gd].team != crate::teams::Team::Guard { "they've got us pinned!" } else { "he's got us pinned!" };
                    crate::game::game_say(g, gd, line, Some(true), None, None);
                } else {
                    let force = g.gd[gd].sup.bark_was == Some(SupMoment::Flinch) && g.time - g.gd[gd].sup.bark_t > 1.5;
                    sup_bark(g, gd, if friendly { SupMoment::Friendly } else { SupMoment::Pinned }, Some(force));
                }
            }
        }
        if g.gd[gd].sup.was_pinned && g.gd[gd].sup.v < 0.02 {
            g.gd[gd].sup.was_pinned = false;
            if reacts(g, gd) && g.gd[gd].state == crate::game::GuardState::Alert
                && crate::game::temperament().get(g.gd[gd].callsign.clone()).map(String::as_str) == Some("hard")
            {
                sup_bark(g, gd, SupMoment::Recover, None);
            }
        }
        if let Some(p) = g.gd[gd].sup.pending {
            let still = reacts(g, gd)
                && g.time - g.gd[gd].sup.pending_t < 2.0
                && match p {
                    SupMoment::Displace => g.gd[gd].sup.cover.is_some(),
                    SupMoment::Pinned => band == 2.0,
                    _ => band >= 1.0,
                };
            if !still {
                g.gd[gd].sup.pending = None;
            } else {
                sup_bark(g, gd, p, Some(p == SupMoment::Displace));
            }
        }
        g.gd[gd].sup.band = band;
    }
}

/// The alert branch's question each frame: is this man's head down, or is he shifting to cover.   (suppression.ts headDown)
pub fn head_down(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::suppression::HeadDown {
    if !reacts(g, gd) {
        sup_clear_reactions(&mut g.gd[gd].sup);
        return HeadDown::None;
    }
    if g.gd[gd].sup.cover.is_some() { return HeadDown::Displace; }
    if g.gd[gd].sup.v < g.k.suppression.flinch { g.gd[gd].sup.pinned_t = 0.0; }
    if g.gd[gd].sup.v >= g.k.suppression.pinned {
        if !g.gd[gd].sup.latched && g.gd[gd].sup.down_t <= 0.0 {
            let span = g.k.suppression.head_down;
            let drawn = span[0] + crate::math_dice::sim_random(g) * (span[1] - span[0]);
            g.gd[gd].sup.down_t = drawn;
        }
        g.gd[gd].sup.latched = true;
    } else if g.gd[gd].sup.v < g.k.suppression.release {
        g.gd[gd].sup.latched = false;
    }
    let down = g.gd[gd].sup.down_t > 0.0 || g.gd[gd].sup.latched;
    if !down {
        let release = g.k.suppression.release;
        let sup = &mut g.gd[gd].sup;
        if sup.down_t <= 0.0 && sup.pinned_t > 0.0 && sup.v < release {
            sup.pinned_t = m::max(0.0, sup.pinned_t - dt);
        }
        return HeadDown::None;
    }
    {
        let sup = &mut g.gd[gd].sup;
        sup.down_t = m::max(0.0, sup.down_t - dt);
        sup.pinned_t += dt;
    }
    if g.gd[gd].sup.pinned_t > g.k.suppression.displace_after
        && !g.gd[gd].pinned
        && g.gd[gd].standoff.is_none()
        && g.time - g.gd[gd].sup.displaced_at > g.k.suppression.displace_cooldown
    {
        let threat = match g.gd[gd].sup.src {
            Some(src) if g.time - g.gd[gd].sup.src_t < g.k.suppression.src_memory_secs => Some(src),
            _ => g.gd[gd].last_known,
        };
        let p = match threat { Some(t) => cover_from(g, gd, t), None => None };
        if let Some(p) = p {
            let now = g.time;
            let sup = &mut g.gd[gd].sup;
            sup.cover = Some(p);
            sup.displaced_at = now;
            sup.displaces += 1.0;
            sup.pinned_t = 0.0;
            sup.down_t = 0.0;
            sup.latched = false;
            g.gd[gd].path = Vec::new();
            g.gd[gd].path_goal = None;
            g.gd[gd].stall_ref = None;
            sup_bark(g, gd, SupMoment::Displace, Some(true));
            return HeadDown::Displace;
        }
        g.gd[gd].sup.pinned_t = g.k.suppression.displace_after - 1.5;
    }
    HeadDown::Down
}

/// The shift is over: hold the spot a few seconds, the first round from there a beat late.   (suppression.ts endDisplace)
pub fn end_displace(g: &mut crate::game::Game, gd: GuardId, why: &str) {
    let until = g.time + g.k.suppression.cover_hold;
    let delay = g.k.suppression.head_down_delay;
    let sup = &mut g.gd[gd].sup;
    sup.cover = None;
    sup.cover_until = until;
    sup.pinned_t = 0.0;
    sup.down_t = 0.0;
    sup.latched = false;
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    g.gd[gd].stall_ref = None;
    g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, delay);
}

/// May the plain fight close on the player right now, or is he holding the cover he has just reached.   (suppression.ts holdingCover)
pub fn holding_cover(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.tune.suppression && g.time < g.gd[gd].sup.cover_until
}

/// No place hidden from everybody: the kept one, the hard place in another man's line over the thin one.   (suppression.ts coverFrom's `fall`)
fn cover_fall(g: &mut Game, kept_open: Option<V3>, as_was: Option<V3>) -> Option<V3> {
    if kept_open.is_some() { g.stats.wallpen.harder += 1; }
    kept_open.or(as_was)
}

/// Where a pinned man shifts to: first fit in the preference order, a dozen path plans at most.   (suppression.ts coverFrom)
pub fn cover_from(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> {
    let all = crate::picture::ideas_of(g, gd).len() > 1;
    let mut as_was: Option<V3> = None;
    let hard = crate::wallpen::wall_men_live(g);
    let mut soft = false;
    let mut was_open = false;
    let mut kept_thin: Option<V3> = None;
    let mut kept_open: Option<V3> = None;
    let c = g.gd[gd].ch;
    let from = g.ch[c].pos;
    let t: V3 = [threat[0], 1.15, threat[2]];
    let d0 = v3::dist_xz(from, threat);
    let away = m::atan2(from[0] - threat[0], from[2] - threat[2]);
    let mut plans: i32 = 0;
    let (displace_min, displace_max, min_travel) = (g.k.suppression.displace_min, g.k.suppression.displace_max, g.k.suppression.min_travel);
    for da in [1.2, -1.2, 0.8, -0.8, 1.65, -1.65, 0.4, -0.4, 0.0, 2.1, -2.1] {
        for r in [2.6, 3.4, displace_min, displace_max] {
            let a = away + da;
            let p: V3 = [from[0] + m::sin(a) * r, 0.0, from[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
            if v3::dist_xz(p, threat) < m::max(d0 - 1.5, min_travel + 0.5) { continue; }
            let chest: V3 = [p[0], 0.9, p[2]];
            if !crate::scene_collision::segment_blocked(g, t, chest, None, None, None) { continue; }
            let thin = hard && {
                let energy = crate::wallpen::top_energy(g);
                crate::wallpen::thin_line(g, t, chest, energy, Some(0.0), None).is_some()
            };
            if thin && as_was.is_some() { continue; }
            let open = all && crate::picture::exposed_at(g, gd, p, Some(true)) > 0.15;
            if open && (kept_open.is_some() || (as_was.is_some() && !(kept_thin.is_some() && plans < 8))) { continue; }
            let mut taken = false;
            for &o in &g.guards {
                if o != gd && g.gd[o].state != crate::game::GuardState::Dead
                    && (v3::dist_xz(g.ch[g.gd[o].ch].pos, p) < 0.9
                        || match g.gd[o].sup.cover { Some(cover) => v3::dist_xz(cover, p) < 1.2, None => false })
                {
                    taken = true;
                    break;
                }
            }
            if taken { continue; }
            plans += 1;
            if plans > 12 { return cover_fall(g, kept_open, as_was); }
            let path = match crate::scene_collision::nav_grid_find_path(g, from, p, None, None) {
                Some(path) if !path.is_empty() => path,
                _ => continue,
            };
            let mut l = 0.0;
            let mut q = from;
            let mut toward = false;
            for &w in &path {
                l += v3::dist_xz(q, w);
                q = w;
                if v3::dist_xz(w, threat) < d0 - 1.5 { toward = true; }
            }
            if l > 2.5 * r + 2.0 || toward { continue; }
            let end = path[path.len() - 1];
            if v3::dist_xz(end, p) > 0.6 { continue; }
            if thin {
                let kept: V3 = [end[0], 0.0, end[2]];
                as_was = Some(kept);
                kept_thin = Some(kept);
                soft = true;
                continue;
            }
            if open {
                was_open = true;
                if kept_thin.is_some() { kept_open = Some([end[0], 0.0, end[2]]); } else { as_was = Some([end[0], 0.0, end[2]]); }
                continue;
            }
            if soft { g.stats.wallpen.harder += 1; }
            if was_open { g.stats.picture.moved += 1; }
            return Some([end[0], 0.0, end[2]]);
        }
    }
    cover_fall(g, kept_open, as_was)
}

/// What suppression does to a guard's next round: extra cone, and x the cadence he draws after it.   (suppression.ts guardFireMods)
pub fn guard_fire_mods(g: &mut crate::game::Game, gd: GuardId) -> crate::suppression::GuardFireMods {
    if !g.tune.suppression { return GuardFireMods { spread: 0.0, gap: 1.0 }; }
    let v = g.gd[gd].sup.v;
    let spread = g.k.suppression.guard_spread * crate::condition::condition_sup(g, crate::condition::Man::Guard(gd), v);
    GuardFireMods { spread, gap: 1.0 + g.k.suppression.gap_k * g.gd[gd].sup.v }
}

/// Is he rattled (over the flinch line, reactions on).   (suppression.ts flinching)
pub fn flinching(g: &mut crate::game::Game, gd: GuardId) -> bool {
    reacts(g, gd) && g.gd[gd].sup.v >= g.k.suppression.flinch
}

/// Extra cone on the player's own round for his own suppression.   (suppression.ts samSpread)
pub fn sam_spread(g: &mut crate::game::Game) -> f64 {
    if g.tune.suppression && g.puppet.is_none() {
        let p = g.player;
        let v = g.pl[p].sup.v;
        g.k.suppression.sam_spread * crate::condition::condition_sup(g, crate::condition::Man::Player(p), v)
    } else {
        0.0
    }
}

/// `x.toFixed(2)`, for the panel's text only.
fn f2(x: f64) -> String { alloc::format!("{:.2}", x) }
/// `x.toFixed(1)`, for the panel's text only.
fn f1(x: f64) -> String { alloc::format!("{:.1}", x) }

/// Panel / QA: the player, then every living man with anything on him (or the nearest, at rest).   (suppression.ts suppressionSummary)
pub fn suppression_summary(g: &mut crate::game::Game) -> String {
    let (pv, pk) = (g.pl[g.player].sup.v, g.pl[g.player].sup.kick);
    let kick = if pk > 0.01 { alloc::format!(" (kick {})", f2(pk)) } else { String::new() };
    let mut parts: Vec<String> = vec![alloc::format!("Player {}{}", f2(pv), kick)];
    let now = g.time;
    let mut shown: Vec<GuardId> = Vec::new();
    for &x in &g.guards {
        if g.gd[x].state == crate::game::GuardState::Dead { continue; }
        let s = &g.gd[x].sup;
        if s.v >= 0.005 || s.cover.is_some() || s.down_t > 0.0 || now < s.cover_until { shown.push(x); }
    }
    if shown.is_empty() {
        if let Some(n) = crate::game::game_nearest_living_guard(g, None, None) { shown = vec![n]; }
    }
    let flinch = g.k.suppression.flinch;
    for gd in shown {
        let s = g.gd[gd].sup;
        let mut bits: Vec<String> = vec![alloc::format!("{} {}", g.gd[gd].callsign, f2(s.v))];
        if let Some(cover) = s.cover {
            bits.push(alloc::format!("→ cover ({}, {})", f1(cover[0]), f1(cover[2])));
        } else if s.down_t > 0.0 || s.latched {
            bits.push(alloc::format!("head-down {} s (pinned {} s)", f1(m::max(s.down_t, 0.0)), f1(s.pinned_t)));
        } else if now < s.cover_until {
            bits.push(alloc::format!("holding cover {} s", f1(s.cover_until - now)));
        } else if s.v >= flinch {
            bits.push(String::from("flinching"));
        }
        if js::truthy(s.displaces) {
            bits.push(alloc::format!("{} shift{}", s.displaces, if s.displaces > 1.0 { "s" } else { "" }));
        }
        parts.push(bits.join(" "));
    }
    alloc::format!("suppression{} ▸ {}", if g.tune.suppression { "" } else { " (reactions OFF)" }, parts.join(" · "))
}

/// Every seat's man, in seat order.   (suppression.ts's `men(g)`: the stub takes the seats and the player as a value)
fn all_men(g: &mut Game) -> Vec<PlayerId> {
    let view = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    crate::seats::men(g, &view)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The lines the regex lets through for a side other than the night staff: printed by the TypeScript's own regex under bun.
    const KEPT: &[&str] = &[
        "taking fire!",
        "rounds coming in!",
        "that was close—",
        "christ— CHRIST—",
        "watch your fire!",
        "check fire — that's past MY head!",
        "easy! I'm in front of you!",
        "stop— stop shooting past me!",
        "I'm pinned!",
        "can't get my head up—",
        "head down— head down!",
        "I can't— I can't move—",
        "moving!",
        "shifting — cover me!",
        "can't stay here — moving!",
        "I'm getting out of here—",
        "relocating.",
        "that all you've got?",
        "my turn.",
    ];

    #[test]
    fn the_word_filter_keeps_what_the_regex_keeps() {
        let kept: Vec<String> = suppression_lines(None).into_iter().filter(|l| !mentions_him(l)).collect();
        let want: Vec<String> = KEPT.iter().map(|l| String::from(*l)).collect();
        assert_eq!(kept, want);
    }

    #[test]
    fn lines_walk_in_the_written_order() {
        let all = suppression_lines(None);
        assert_eq!(all.len(), 27);
        assert_eq!(all[0], "taking fire!");
        assert_eq!(all[26], "my turn.");
        assert_eq!(suppression_lines(Some(SupMoment::Recover)), vec![String::from("that all you've got?"), String::from("my turn.")]);
        assert_eq!(lines_of(SupMoment::Friendly, "hard").len(), 0);
        assert_eq!(lines_of(SupMoment::Friendly, "nervy").len(), 1);
    }

    #[test]
    fn the_tables_say_what_the_skeletons_say() {
        let mut theirs: Vec<String> = Vec::new();
        for row in crate::suppression::sup_lines().values() {
            for pool in row.values() {
                theirs.extend(pool.clone().unwrap());
            }
        }
        assert_eq!(suppression_lines(None), theirs);
        for moment in [SupMoment::Recover, SupMoment::Friendly, SupMoment::Flinch, SupMoment::Pinned, SupMoment::Displace] {
            assert_eq!(rank(moment) as f64, *crate::suppression::rank().get(String::from(moment.as_str())).unwrap());
        }
    }

    #[test]
    fn worse_news_outranks() {
        assert!(rank(SupMoment::Displace) > rank(SupMoment::Pinned));
        assert!(rank(SupMoment::Pinned) > rank(SupMoment::Flinch));
        assert_eq!(rank(SupMoment::Flinch), rank(SupMoment::Friendly));
        assert!(rank(SupMoment::Friendly) > rank(SupMoment::Recover));
    }

    #[test]
    fn a_band_holds_with_hysteresis() {
        let b = |v, was| band_of(v, was, 0.6, 0.45, 0.3);
        assert_eq!(b(0.0, 0.0), 0.0);
        assert_eq!(b(0.3, 0.0), 1.0);
        assert_eq!(b(0.26, 0.0), 0.0);
        assert_eq!(b(0.26, 1.0), 1.0);
        assert_eq!(b(0.24, 1.0), 0.0);
        assert_eq!(b(0.6, 0.0), 2.0);
        assert_eq!(b(0.5, 2.0), 2.0);
        assert_eq!(b(0.44, 2.0), 1.0);
        assert_eq!(b(0.5, 1.0), 1.0);
        assert_eq!(b(0.26, 2.0), 1.0);
    }

    #[test]
    fn a_decay_goes_to_zero_under_the_floor() {
        let mut s = Sup::blank();
        s.v = 1.0;
        s.kick = 0.5;
        decay(&mut s, 0.5, 0.25);
        assert_eq!((s.v, s.kick), (0.5, 0.125));
        s.v = 1.5e-5;
        s.kick = 1.5e-5;
        decay(&mut s, 0.5, 0.5);
        assert_eq!((s.v, s.kick), (0.0, 0.0));
    }

    #[test]
    fn clearing_leaves_the_scalar_and_the_barks() {
        let mut s = Sup::blank();
        s.v = 0.7;
        s.down_t = 1.0;
        s.latched = true;
        s.pinned_t = 2.0;
        s.cover = Some([1.0, 0.0, 2.0]);
        s.cover_until = 9.0;
        s.displaces = 2.0;
        sup_clear_reactions(&mut s);
        assert_eq!((s.v, s.down_t, s.latched, s.pinned_t, s.cover, s.cover_until, s.displaces), (0.7, 0.0, false, 0.0, None, -1.0, 2.0));
    }
}
