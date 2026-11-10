pub use ::game_types::suppression::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// Everything the reactions keep, back to rest; the scalar itself stays.
fn sup_clear_reactions(this: &mut crate::suppression::Sup) {
    this.down_t = 0.0;
    this.latched = false;
    this.pinned_t = 0.0;
    this.cover = None;
    this.cover_until = -1.0;
}

/// `TEMPERAMENT[callsign]`: the skeleton's table is a slice of pairs, walked for the name.
fn temperament_row(callsign: &str) -> Option<crate::game::Temperament> {
    crate::game::temperament().iter().find(|row| row.0 == callsign).map(|row| row.1)
}

/// How readily this body accrues suppression, by his temperament.
fn keen_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: CharId) -> f64 {
    if c_2 == g.pl[seat.player].ch { return 1.0; }
    let mut t: Option<crate::game::Temperament> = None;
    for &x in &g.guards {
        if g.gd[x].ch == c_2 {
            t = temperament_row(&g.gd[x].callsign);
            break;
        }
    }
    match t {
        Some(t) => match g.k.suppression.keen.get(String::from(t.as_str())) {
            Some(&k) => js::or(k, 1.0),
            None => 1.0,
        },
        None => 1.0,
    }
}

/// One round has just been traced: every OTHER living, free man it passed close to or struck close to accrues.
pub fn note_round(g: &mut crate::game::Game, shooter: CharId, from: V3, to: V3, hit: Option<CharId>) {
    let seg = v3::sub(to, from);
    let t_hit = v3::len(seg);
    if t_hit < 1e-3 { return; }
    let dir = v3::scale(seg, 1.0 / t_hit);
    let shooter_team = g.ch[shooter].team;
    let round = Round { shooter, hit, from, to, dir, t_hit, shooter_team };
    for pl in crate::seats::men(g) {
        let c = g.pl[pl].ch;
        note_one(g, &round, c, None, false, crate::teams::Team::Ours);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !crate::condition::down(g, g.gd[gd].ch) {
            let (c, held, team) = (g.gd[gd].ch, g.gd[gd].held.is_some(), g.team(gd));
            note_one(g, &round, c, Some(gd), held, team);
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

/// One man's share of one round.
fn note_one(g: &mut Game, r: &Round, c: CharId, gd: Option<GuardId>, held: bool, team: crate::teams::Team) {
    if c == r.shooter || Some(c) == r.hit || !g.ch[c].alive || held { return; }
    let chest = g.ch[c].bone_or(crate::character::Bone::Chest, 1.1);
    let s = v3::clamp(v3::dot(v3::sub(chest, r.from), r.dir), 0.0, r.t_hit);
    let foe = r.shooter_team.foe_of(team);
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
    let sup = g.cond[c].sup;
    if sup.down_t > 0.0 || sup.latched || sup.cover.is_some() { add *= g.k.suppression.down_accrual; }
    if !foe { add *= g.k.suppression.friendly_k; }
    add *= keen_of(g, &crate::seats::here(g), c);
    let (cap, shooter_id, now) = (g.k.suppression.cap, g.ch[r.shooter].id, g.time);
    let sup = &mut g.cond[c].sup;
    sup.v = m::min(cap, sup.v + add);
    sup.kick = m::max(sup.kick, add);
    sup.src = Some(v3::copy(r.from));
    sup.src_id = shooter_id;
    sup.src_t = now;
}

/// Which held-back line outranks which.
fn rank_of(moment: crate::suppression::SupMoment) -> i32 {
    match moment {
        SupMoment::Recover => 0,
        SupMoment::Friendly => 1,
        SupMoment::Flinch => 1,
        SupMoment::Pinned => 2,
        SupMoment::Displace => 3,
    }
}

/// The lines, by moment and temperament ('*' for anybody), in the written order.
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

/// `/\b(he|he's|him|his)\b/.test(l)`.
fn mentions_him(l: &str) -> bool {
    let b = l.as_bytes();
    for w in ["he", "he's", "him", "his"] {
        let w = w.as_bytes();
        let mut at = 0;
        while at + w.len() <= b.len() {
            if &b[at..at + w.len()] == w
                && (at == 0 || !crate::js::is_word(b[at - 1]))
                && (at + w.len() == b.len() || !crate::js::is_word(b[at + w.len()]))
            {
                return true;
            }
            at += 1;
        }
    }
    false
}

/// A teammate with the player's round going past him says what a man says of a colleague's.
pub fn watch_your_fire(g: &mut crate::game::Game, gd: GuardId) -> bool {
    sup_bark(g, gd, SupMoment::Friendly, None)
}

/// One suppression line from this man if the clocks allow; a line held back is kept `pending`.
fn sup_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::suppression::SupMoment, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    let now = g.time;
    if crate::runner::runner_mute(g, gd) { return false; }
    let floor_bark = g.m.suppression.floor_bark_t.unwrap_or(-100.0);
    let held = (!force && (now - g.cond[g.gd[gd].ch].sup.bark_t < g.k.suppression.bark_secs || now - floor_bark < g.k.suppression.floor_bark_secs))
        || match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 2.5, None => false };
    if held {
        let sup = &mut g.cond[g.gd[gd].ch].sup;
        if moment != SupMoment::Recover && rank_of(moment) > (match sup.pending { Some(p) => rank_of(p), None => -1 }) {
            sup.pending = Some(moment);
            sup.pending_t = now;
        }
        return false;
    }
    let temperament = crate::dialogue::temperament_of(g, gd);
    let mut pool: Vec<&'static str> = Vec::new();
    pool.extend_from_slice(lines_of(moment, temperament.as_str()));
    pool.extend_from_slice(lines_of(moment, "*"));
    if g.team(gd) != crate::teams::Team::Guard {
        pool.retain(|l| !mentions_him(l));
    }
    if pool.is_empty() { return false; }
    let n = pool.len() as f64;
    let mut i = js::rem(m::floor(crate::dice::draw_of(g, "suppression::sup_bark#1", gd) * n), n) as usize;
    if pool.len() > 1 && g.m.suppression.last_line.get(gd).map(|l| l.as_str()) == Some(pool[i]) {
        i = (i + 1) % pool.len();
    }
    g.m.suppression.last_line.set(gd, String::from(pool[i]));
    let sup = &mut g.cond[g.gd[gd].ch].sup;
    sup.bark_t = now;
    sup.bark_was = Some(moment);
    sup.pending = None;
    g.m.suppression.floor_bark_t = Some(now);
    crate::game::game_say(g, gd, pool[i], None, None);
    true
}

/// May this man's behaviour react at all this frame.
fn reacts(g: &crate::game::Game, gd: GuardId) -> bool {
    g.tune.suppression && g.ai_enabled && !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].held.is_none() && g.gd[gd].script.is_none()
}

/// Once a frame's decay of one man's scalar and pulse.
fn decay(sup: &mut Sup, k: f64, kk: f64) {
    sup.v *= k;
    sup.kick *= kk;
    if sup.v < 1e-5 { sup.v = 0.0; }
    if sup.kick < 1e-5 { sup.kick = 0.0; }
}

/// His band, with hysteresis: a scalar hovering about a line under a steady volley is ONE episode.
fn band_of(v: f64, was: f64, pinned: f64, release: f64, flinch: f64) -> f64 {
    if v >= pinned || (was == 2.0 && v >= release) {
        2.0
    } else if v >= flinch || (was >= 1.0 && v >= flinch - 0.05) {
        1.0
    } else {
        0.0
    }
}

/// Where his start jolts him from: the muzzle of the last round, off his own feet.
fn start_from_round(g: &mut Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    let toward = match g.cond[g.gd[gd].ch].sup.src { Some(src) => Some(v3::sub(src, g.ch[c].pos)), None => None };
    let scale = crate::flinch::flinch_scale(g, Some(gd));
    crate::character::anim_flinch_from(g, c, toward, Some(scale), None);
}

/// Once a frame, ahead of the player and the guards: decay everybody's scalar and pulse; the guards' edges.
pub fn update_suppression(g: &mut crate::game::Game, dt: f64) {
    let k = m::pow(2.0, -dt / g.k.suppression.half_life);
    let kk = m::exp(-g.k.suppression.kick_decay * dt);
    for pl in crate::seats::men(g) {
        let down = crate::condition::down(g, g.pl[pl].ch);
        let p = &mut g.cond[g.pl[pl].ch].sup;
        if down { p.v = 0.0; p.kick = 0.0; } else { decay(p, k, kk); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        one_guards_edges(g, gd, k, kk);
    }
}

/// One guard's frame: his scalar decays, a band he has climbed into gets its start and its line, and a held-back line gets its chance.
fn one_guards_edges(g: &mut Game, gd: GuardId, k: f64, kk: f64) {
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() {
        let sup = &mut g.cond[g.gd[gd].ch].sup;
        sup.v = 0.0;
        sup.kick = 0.0;
        sup.band = 0.0;
        sup.was_pinned = false;
        sup_clear_reactions(sup);
        return;
    }
    decay(&mut g.cond[g.gd[gd].ch].sup, k, kk);
    if g.gd[gd].state != crate::game::GuardState::Alert || !reacts(g, gd) { sup_clear_reactions(&mut g.cond[g.gd[gd].ch].sup); }
    let v = g.cond[g.gd[gd].ch].sup.v;
    let was = g.cond[g.gd[gd].ch].sup.band;
    let band = band_of(v, was, g.k.suppression.pinned, g.k.suppression.release, g.k.suppression.flinch);
    if band > was && was < 1.0 && !reacts(g, gd)
        && g.gd[gd].script.as_ref().map_or(false, |s| s.flinch == Some(true))
        && g.tune.suppression
    {
        start_from_round(g, gd);
    }
    if band > was && reacts(g, gd) { climbs_a_band(g, gd, was, band); }
    if g.cond[g.gd[gd].ch].sup.was_pinned && g.cond[g.gd[gd].ch].sup.v < 0.02 {
        g.cond[g.gd[gd].ch].sup.was_pinned = false;
        if reacts(g, gd) && g.gd[gd].state == crate::game::GuardState::Alert
            && temperament_row(&g.gd[gd].callsign) == Some(crate::game::Temperament::Hard)
        {
            sup_bark(g, gd, SupMoment::Recover, None);
        }
    }
    if let Some(p) = g.cond[g.gd[gd].ch].sup.pending {
        let still = reacts(g, gd)
            && g.time - g.cond[g.gd[gd].ch].sup.pending_t < 2.0
            && match p {
                SupMoment::Displace => g.cond[g.gd[gd].ch].sup.cover.is_some(),
                SupMoment::Pinned => band == 2.0,
                _ => band >= 1.0,
            };
        if !still {
            g.cond[g.gd[gd].ch].sup.pending = None;
        } else {
            sup_bark(g, gd, p, Some(p == SupMoment::Displace));
        }
    }
    g.cond[g.gd[gd].ch].sup.band = band;
}

/// Into a higher band: the start from calm jolts him and holds his fire, and pinned he says so.
fn climbs_a_band(g: &mut Game, gd: GuardId, was: f64, band: f64) {
    let src_id = g.cond[g.gd[gd].ch].sup.src_id;
    let friendly = !crate::seats::is_man_id(g, src_id) && src_id >= 0.0 && {
        let team = g.team(gd);
        let theirs = crate::teams::team_of_id(g, src_id);
        !team.foe_of(theirs)
    };
    if was < 1.0 {
        start_from_round(g, gd);
        g.arms[g.gd[gd].ch].fire_cd = m::max(g.arms[g.gd[gd].ch].fire_cd, g.k.suppression.flinch_delay);
        if band < 2.0 {
            sup_bark(g, gd, if friendly { SupMoment::Friendly } else { SupMoment::Flinch }, None);
        }
    }
    if band == 2.0 { pinned_says_so(g, gd, friendly); }
}

/// Pinned: with another of his side just pinned too, he says it for the floor on the radio, now and then; else his own line.
fn pinned_says_so(g: &mut Game, gd: GuardId, friendly: bool) {
    g.cond[g.gd[gd].ch].sup.pinned_at = g.time;
    g.cond[g.gd[gd].ch].sup.was_pinned = true;
    let other = g.guards.iter().any(|&o| {
        o != gd
            && !crate::condition::down(g, g.gd[o].ch)
            && g.time - g.cond[g.gd[o].ch].sup.pinned_at < g.k.suppression.pair_secs
            && g.team(o) == g.team(gd)
            && !crate::runner::runner_mute(g, o)
    });
    if other && !crate::runner::runner_mute(g, gd)
        && g.time - g.m.suppression.floor_line_t.unwrap_or(-100.0) > g.k.suppression.floor_line_secs
    {
        let now = g.time;
        g.m.suppression.floor_line_t = Some(now);
        g.cond[g.gd[gd].ch].sup.bark_t = now;
        g.cond[g.gd[gd].ch].sup.bark_was = Some(SupMoment::Pinned);
        g.m.suppression.floor_bark_t = Some(now);
        let line = if g.team(gd) != crate::teams::Team::Guard { "they've got us pinned!" } else { "he's got us pinned!" };
        crate::game::game_say(g, gd, line, Some(true), None);
    } else {
        let force = g.cond[g.gd[gd].ch].sup.bark_was == Some(SupMoment::Flinch) && g.time - g.cond[g.gd[gd].ch].sup.bark_t > 1.5;
        sup_bark(g, gd, if friendly { SupMoment::Friendly } else { SupMoment::Pinned }, Some(force));
    }
}

/// `gd.path = []; gd.pathGoal = null; gd.stallRef = null`: a whole write of his path (Guard.path_serial), and a goal the standoff did not lay.
fn clear_walk(g: &mut Game, gd: GuardId) {
    let o = &mut g.gd[gd];
    o.drop_path();
    o.stall_ref = None;
}

/// The alert branch's question each frame: is this man's head down, or is he shifting to cover.
pub(super) fn head_down(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::suppression::HeadDown {
    if !reacts(g, gd) {
        sup_clear_reactions(&mut g.cond[g.gd[gd].ch].sup);
        return HeadDown::None;
    }
    if g.cond[g.gd[gd].ch].sup.cover.is_some() { return HeadDown::Displace; }
    if g.cond[g.gd[gd].ch].sup.v < g.k.suppression.flinch { g.cond[g.gd[gd].ch].sup.pinned_t = 0.0; }
    if g.cond[g.gd[gd].ch].sup.v >= g.k.suppression.pinned {
        if !g.cond[g.gd[gd].ch].sup.latched && g.cond[g.gd[gd].ch].sup.down_t <= 0.0 {
            let span = g.k.suppression.head_down;
            let drawn = span[0] + crate::dice::draw_of(g, "suppression::head_down#1", gd) * (span[1] - span[0]);
            g.cond[g.gd[gd].ch].sup.down_t = drawn;
        }
        g.cond[g.gd[gd].ch].sup.latched = true;
    } else if g.cond[g.gd[gd].ch].sup.v < g.k.suppression.release {
        g.cond[g.gd[gd].ch].sup.latched = false;
    }
    let down = g.cond[g.gd[gd].ch].sup.down_t > 0.0 || g.cond[g.gd[gd].ch].sup.latched;
    if !down {
        let release = g.k.suppression.release;
        let sup = &mut g.cond[g.gd[gd].ch].sup;
        if sup.down_t <= 0.0 && sup.pinned_t > 0.0 && sup.v < release {
            sup.pinned_t = m::max(0.0, sup.pinned_t - dt);
        }
        return HeadDown::None;
    }
    {
        let sup = &mut g.cond[g.gd[gd].ch].sup;
        sup.down_t = m::max(0.0, sup.down_t - dt);
        sup.pinned_t += dt;
    }
    let after = if ducking_hides(g, gd) { g.k.suppression.displace_after } else { OPEN_BEAT };
    if g.cond[g.gd[gd].ch].sup.pinned_t > after
        && !g.gd[gd].pinned
        && g.gd[gd].standoff.is_none()
        && g.time - g.cond[g.gd[gd].ch].sup.displaced_at > g.k.suppression.displace_cooldown
    {
        let threat = match g.cond[g.gd[gd].ch].sup.src {
            Some(src) if g.time - g.cond[g.gd[gd].ch].sup.src_t < g.k.suppression.src_memory_secs => Some(src),
            _ => g.gd[gd].last_known,
        };
        let p = match threat { Some(t) => cover_from(g, gd, t), None => None };
        if let Some(p) = p {
            let now = g.time;
            let sup = &mut g.cond[g.gd[gd].ch].sup;
            sup.cover = Some(p);
            sup.displaced_at = now;
            sup.displaces += 1.0;
            sup.pinned_t = 0.0;
            sup.down_t = 0.0;
            sup.latched = false;
            clear_walk(g, gd);
            sup_bark(g, gd, SupMoment::Displace, Some(true));
            return HeadDown::Displace;
        }
        g.cond[g.gd[gd].ch].sup.pinned_t = after - AGAIN;
    }
    HeadDown::Down
}

/// GETTING DOWN HIDES HIM where he stands, from everybody he knows of: else he goes for cover, a beat later, if there is any. Asked four times a second.
fn ducking_hides(g: &mut Game, gd: GuardId) -> bool {
    let (c, now) = (g.gd[gd].ch, g.time);
    let t = g.cond[c].sup.hides_t;
    if now < t || now - t >= 0.25 {
        let at = g.ch[c].pos;
        let hides = crate::picture::exposed_at(g, gd, at, Some(true)) <= OPEN;
        let sup = &mut g.cond[c].sup;
        (sup.hides, sup.hides_t) = (hides, now);
    }
    g.cond[c].sup.hides
}
/// how much of a line on him is a line · seconds he is down in the open before he goes · … before he looks for cover again where there was none
const OPEN: f64 = 0.15;
const OPEN_BEAT: f64 = 0.35;
const AGAIN: f64 = 1.5;

/// Rounds have a man at his post pinned and getting down hides him from nobody: where he could shift to, a beat later, if there is somewhere and he has not shifted of late.
pub fn caught_open(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let sup = &g.cond[g.gd[gd].ch].sup;
    if sup.pinned_t <= OPEN_BEAT || g.time - sup.displaced_at <= g.k.suppression.displace_cooldown || sup.hides || g.time - sup.hides_t > 0.5 { return None; }
    let threat = sup.src.filter(|_| g.time - sup.src_t < g.k.suppression.src_memory_secs).or(g.gd[gd].last_known)?;
    let to = cover_from(g, gd, threat);
    let now = g.time;
    let sup = &mut g.cond[g.gd[gd].ch].sup;
    if to.is_some() { sup.displaced_at = now; (sup.pinned_t, sup.down_t, sup.latched) = (0.0, 0.0, false); } else { sup.pinned_t = OPEN_BEAT - AGAIN; }
    to
}

/// The shift is over: hold the spot a few seconds, the first round from there a beat late.
pub(super) fn end_displace(g: &mut crate::game::Game, gd: GuardId) {
    let until = g.time + g.k.suppression.cover_hold;
    let delay = g.k.suppression.head_down_delay;
    let sup = &mut g.cond[g.gd[gd].ch].sup;
    sup.cover = None;
    sup.cover_until = until;
    sup.pinned_t = 0.0;
    sup.down_t = 0.0;
    sup.latched = false;
    clear_walk(g, gd);
    g.arms[g.gd[gd].ch].fire_cd = m::max(g.arms[g.gd[gd].ch].fire_cd, delay);
}

/// May the plain fight close on the player right now, or is he holding the cover he has just reached.
pub(super) fn holding_cover(g: &crate::game::Game, gd: GuardId) -> bool {
    g.tune.suppression && g.time < g.cond[g.gd[gd].ch].sup.cover_until
}

/// No place hidden from everybody: the kept one, the hard place in another man's line over the thin one.
fn cover_fall(kept_open: Option<V3>, as_was: Option<V3>) -> Option<V3> {
    kept_open.or(as_was)
}

/// A man under fire does not stop to open a door: floor behind a shut one is no cover to him.
fn through_a_shut_door(g: &crate::game::Game, from: V3, path: &[V3]) -> bool {
    let mut a = from;
    for &b in path {
        for i in 0..g.doors.list.len() {
            let d = g.doors.list[i];
            if g.dr[d].leaf.latched && crate::guards::crosses_door(g, a, b, d).is_some() { return true; }
        }
        a = b;
    }
    false
}

/// Where a pinned man shifts to: first fit in the preference order, a dozen path plans at most.
pub fn cover_from(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> {
    let all = crate::picture::ideas_of(g, gd).len() > 1;
    let mut as_was: Option<V3> = None;
    let mut was_open = false;
    let mut kept_thin: Option<V3> = None;
    let mut kept_open: Option<V3> = None;
    let c = g.gd[gd].ch;
    let from = g.ch[c].pos;
    let t: V3 = [threat[0], 1.15, threat[2]];
    let d0 = v3::dist_xz(from, threat);
    let away = v3::yaw_to(threat, from);
    let mut plans: i32 = 0;
    let (displace_min, displace_max, min_travel) = (g.k.suppression.displace_min, g.k.suppression.displace_max, g.k.suppression.min_travel);
    for da in [1.2, -1.2, 0.8, -0.8, 1.65, -1.65, 0.4, -0.4, 0.0, 2.1, -2.1] {
        for r in [2.6, 3.4, displace_min, displace_max] {
            let a = away + da;
            let p: V3 = [from[0] + m::sin(a) * r, 0.0, from[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
            if g.gd[gd].stays_in_rooms && !crate::enemysquads::in_squad_rooms(g, gd, p) { continue; }
            if v3::dist_xz(p, threat) < m::max(d0 - 1.5, min_travel + 0.5) { continue; }
            let chest: V3 = [p[0], 0.9, p[2]];
            if !crate::scene_collision::segment_blocked(g, t, chest, None, None, None) { continue; }
            let thin = {
                let energy = crate::wallpen::top_energy(g);
                crate::wallpen::thin_line(g, t, chest, energy, Some(0.0), None).is_some()
            };
            if thin && as_was.is_some() { continue; }
            let open = all && crate::picture::exposed_at(g, gd, p, Some(true)) > 0.15;
            if open && (kept_open.is_some() || (as_was.is_some() && !(kept_thin.is_some() && plans < 8))) { continue; }
            if crate::places::taken(g, gd, p, 0.9) { continue; }
            plans += 1;
            if plans > 12 { return cover_fall(kept_open, as_was); }
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
            if through_a_shut_door(g, from, &path) { continue; }
            let end = path[path.len() - 1];
            if v3::dist_xz(end, p) > 0.6 { continue; }
            if thin {
                let kept: V3 = [end[0], 0.0, end[2]];
                as_was = Some(kept);
                kept_thin = Some(kept);
                continue;
            }
            if open {
                was_open = true;
                if kept_thin.is_some() { kept_open = Some([end[0], 0.0, end[2]]); } else { as_was = Some([end[0], 0.0, end[2]]); }
                continue;
            }
            if was_open { g.stats.picture.moved += 1; }
            return Some([end[0], 0.0, end[2]]);
        }
    }
    cover_fall(kept_open, as_was)
}

/// What suppression does to a guard's next round: extra cone, and x the cadence he draws after it.
pub fn guard_fire_mods(g: &crate::game::Game, gd: GuardId) -> crate::suppression::GuardFireMods {
    if !g.tune.suppression { return GuardFireMods { spread: 0.0, gap: 1.0 }; }
    let v = g.cond[g.gd[gd].ch].sup.v;
    let spread = g.k.suppression.guard_spread * crate::condition::condition_sup(g, g.gd[gd].ch, v);
    GuardFireMods { spread, gap: 1.0 + g.k.suppression.gap_k * g.cond[g.gd[gd].ch].sup.v }
}

/// Is he rattled (over the flinch line, reactions on).
pub fn flinching(g: &crate::game::Game, gd: GuardId) -> bool {
    reacts(g, gd) && g.cond[g.gd[gd].ch].sup.v >= g.k.suppression.flinch
}

/// Extra cone on the player's own round for his own suppression.
pub fn sam_spread(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> f64 {
    if g.tune.suppression {
        let p = seat.player;
        let v = g.cond[g.pl[p].ch].sup.v;
        g.k.suppression.sam_spread * crate::condition::condition_sup(g, g.pl[p].ch, v)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_temperament_is_found_by_the_name() {
        assert_eq!(temperament_row("Reyes"), Some(crate::game::Temperament::Hard));
        assert_eq!(temperament_row("Novak"), Some(crate::game::Temperament::Nervy));
        assert_eq!(temperament_row("nobody"), None);
    }

    #[test]
    fn worse_news_outranks() {
        assert!(rank_of(SupMoment::Displace) > rank_of(SupMoment::Pinned));
        assert!(rank_of(SupMoment::Pinned) > rank_of(SupMoment::Flinch));
        assert_eq!(rank_of(SupMoment::Flinch), rank_of(SupMoment::Friendly));
        assert!(rank_of(SupMoment::Friendly) > rank_of(SupMoment::Recover));
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
