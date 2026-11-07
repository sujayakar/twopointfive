//! HIS EARS (and his eye on a light going out): every event of the frame once, and what that kind of sound does to him. One function a kind, asked in
//! `hearing_check`'s order; each reads the sound as it reached him (`Sound`) and moves his awareness, his fix and his reaction.
//!
//! TO ADD A SOUND: its kind's function and its line in `heard`.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use crate::director::EventKind as K;
use crate::game::GuardState as S;

/// an event as it reached his ears
struct Sound {
    e: crate::game::GameEvent,
    /// its place in the frame's list (a line said of it is said once, by whoever is first)
    at_e: usize,
    /// how far it came, by the way it came, and straight
    d: f64,
    d_straight: f64,
    /// how much of it the way let through; whether that way is straight; where it seems to come from
    carry: f64,
    direct: bool,
    heard_at: V3,
    /// a blast or a mate's word has placed his belief since this was made: that report stands
    placed: bool,
    /// how keen he is, and that by his ears and by his ears for footfalls
    keen: f64,
    dull: f64,
    ear: f64,
    foot: f64,
    gunfire: bool,
    kick: bool,
}

/// Every event of the frame, once. `ear`, `foot`: how well he hears; `eyes`: whether he would see a light go out. (guards.ts hearingCheck)
pub fn hearing_check(g: &mut Game, gd: GuardId, ear: Option<f64>, eyes: Option<bool>, foot: Option<f64>) {
    let ear = ear.unwrap_or(1.0);
    let eyes = eyes.unwrap_or(true);
    let foot = foot.unwrap_or(ear);
    let keen = if escalation_of(g) >= 1.0 { KEEN } else { 1.0 };
    let dull = keen * ear;
    let c = g.gd[gd].ch;
    // By live index: a line said or a rule called on the way may file an event, and it is walked as the TypeScript's for … of walks it.
    let mut next = 0;
    while next < g.events.len() {
        let at_e = next;
        next += 1;
        if g.events[at_e].id.is_none() {
            g.event_seq += 1.0;
            g.events[at_e].id = Some(g.event_seq);
        }
        let e = g.events[at_e];
        let id = e.id.unwrap();
        if id <= g.gd[gd].heard_up_to { continue; }
        g.gd[gd].heard_up_to = m::max(g.gd[gd].heard_up_to, id);
        let placed = crate::belief::word_stands(g, gd, e.time);
        let d_straight = v3::dist(e.pos, g.ch[c].pos);
        let gunfire = (e.kind == K::Shot || e.kind == K::GuardShot) && e.level.unwrap_or(0.0) >= 1.0;
        let kick = e.kind == K::Kick;
        let mut s = Sound { e, at_e, d: d_straight, d_straight, carry: 1.0, direct: true, heard_at: e.pos, placed, keen, dull, ear, foot, gunfire, kick };
        if e.kind == K::LightOut {
            if eyes { light_out(g, gd, &s); }
            continue;
        }
        if e.kind == K::Whistle {
            if d_straight <= g.k.whistle.horizon { crate::whistle::hear_whistle(g, gd, e, keen, &mut |g: &mut Game, at: V3| small(g, gd, &e, at)); }
            continue;
        }
        let horizon = if e.kind == K::Bang || gunfire { 80.0 } else if kick { 40.0 } else if e.kind == K::Glass { g.k.glass.hear_m } else { 22.0 };
        if d_straight > horizon { continue; }
        let ear_at = v3::add(g.ch[c].pos, [0.0, 1.5, 0.0]);
        let prop = if e.kind == K::Glass { glass_heard(g, e, ear_at) } else { crate::weather::hear_path(g, v3::add(e.pos, [0.0, 1.0, 0.0]), ear_at, None) };
        crate::doorforce::door_force_hear(g, gd, e, crate::doorforce::DoorForceHearProp { direct: prop.direct, routed: prop.routed, path_len: prop.path_len, carry: prop.carry });
        s.d = prop.path_len;
        s.carry = prop.carry;
        s.direct = prop.direct;
        s.heard_at = [prop.via[0], 0.0, prop.via[2]];
        heard(g, gd, &s);
    }
}

/// what the sound does to him, by its kind
fn heard(g: &mut Game, gd: GuardId, s: &Sound) {
    odd_shot(g, gd, s);
    loud(g, gd, s);
    knock(g, gd, s);
    glass(g, gd, s);
    step(g, gd, s);
}

/// A LIGHT GOING OUT, seen: a glance at it, or a thing to look into.
fn light_out(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, at_e, d_straight, placed, .. } = s;
    let c = g.gd[gd].ch;
    if d_straight < 10.0 && g.gd[gd].awareness < 0.4 && light_out_seen(g, gd, LightOutSeenE { pos: e.pos, from: e.from }) {
        if e.cause == Some(crate::game::GameEventCause::Ocp) && g.gd[gd].awareness < 0.3 && g.tune.ocp_notice == crate::game::GameTuneOcpNotice::Glance {
            let src: V3 = match e.from {
                Some(f) => f,
                None => [e.pos[0], 2.4, e.pos[2]],
            };
            let pos = g.ch[c].pos;
            let eye = crate::guards::p_01::bone_or(g, c, crate::character::Bone::Head, v3::add(pos, [0.0, 1.65, 0.0]));
            let to = v3::sub(src, eye);
            let l = v3::len(to);
            let short_of: V3 = if l > 0.6 { v3::mad(eye, to, (l - 0.4) / l) } else { src };
            if d_straight < 8.0
                && (d_straight < 3.0
                    || point_in_view(g, gd, short_of, Some(10.0), Some(75.0))
                    || point_in_view(g, gd, [e.pos[0], 0.15, e.pos[2]], Some(10.0), Some(75.0)))
            {
                { let to = m::min(0.28, g.gd[gd].awareness + 0.18); crate::mind::set(g, gd, to, "guards"); }
                crate::whistle::glance_toward(g, gd, e.pos, Some(1.8));
                small(g, gd, &e, e.pos);
                crate::chronicle::chron_heard(g, gd, e, None);
                if unannounced(g, at_e) && g.gd[gd].state == S::Patrol {
                    g.events[at_e].announced = Some(true);
                    let lines = ocp_mutter();
                    let line = js::rem(g.ch[c].id + m::floor(g.time), lines.len() as f64) as usize;
                    crate::game::game_say(g, gd, &lines[line], Some(false), None, None);
                }
            }
        } else {
            let at = match e.sw_at {
                Some(s) => s,
                None => e.pos,
            };
            crate::mind::set(g, gd, 0.4, "guards");
            if !placed {
                crate::mind::fix(g, gd, Some(at), "guards");
            }
            g.gd[gd].react_t = 0.8;
            small(g, gd, &e, at);
            crate::chronicle::chron_heard(g, gd, e, None);
        }
    }
}

/// A SHOT THAT IS NOT GUNFIRE (a gun with a can on it), near enough to be odd.
fn odd_shot(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, d, d_straight, carry, direct, heard_at, placed, ear, gunfire, .. } = s;
    let odd = e.kind == K::Shot
        || (e.kind == K::GuardShot
            && crate::gunslots::quiet_heard(g)
            && d_straight >= 0.1
            && shot_relation(g, gd, crate::guards::p_01::shot_e(&e)) == ShotRelation::Threat);
    if odd && !gunfire && d < 7.0 * carry * m::max(0.3, e.level.unwrap_or(0.6) / 0.6) * ear + 0.5 {
        crate::mind::raise(g, gd, 0.5, "guards");
        let at = if direct { e.pos } else { heard_at };
        if !placed && (g.gd[gd].state != S::Alert || g.gd[gd].last_known.is_none() || g.time - g.gd[gd].last_seen_t > 1.0) {
            crate::mind::fix(g, gd, Some(at), "guards");
        }
        if g.gd[gd].state == S::Patrol {
            g.gd[gd].react_t = m::max(g.gd[gd].react_t, 0.6);
        }
        small(g, gd, &e, at);
        crate::chronicle::chron_heard(g, gd, e, None);
    }
}

/// GUNFIRE, A BLAST, A DOOR KICKED IN: he is alert, his fix is as good as his ears, and the first to hear it says what it was.
fn loud(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, at_e, d, d_straight, carry, direct, placed, gunfire, kick, .. } = s;
    if !(gunfire || e.kind == K::Bang || kick) { return; }
    let c = g.gd[gd].ch;
    if e.kind == K::GuardShot && d_straight < 0.1 {
        return;
    }
    let rel = if gunfire { shot_relation(g, gd, crate::guards::p_01::shot_e(&e)) } else { ShotRelation::Threat };
    if rel == ShotRelation::None
        || crate::frag::own_bang(g, gd, crate::frag::OwnBangE { frag: e.frag, who: e.who })
        || crate::doorforce::door_force_own_boot(g, gd, e)
    {
        return;
    }
    let ear_at = v3::add(g.ch[c].pos, [0.0, 1.5, 0.0]);
    let d_loud = crate::weather::loud_len(g, d, ear_at);
    let now = g.time;
    {
        let h = &mut g.gd[gd].witness.heard;
        if gunfire {
            h.shots += 1.0;
            h.last_shot_t = now;
        } else if kick {
            h.kick += 1.0;
        } else {
            h.bang += 1.0;
        }
        h.last_t = now;
    }
    let calm = was_calm(g, gd);
    crate::mind::raise(g, gd, 0.92, "guards");
    crate::chronicle::chron_heard(g, gd, e, None);
    if rel == ShotRelation::Own {
        let at = colleague_fire_pos(g, gd, ColleagueFirePosE { who: e.who, pos: e.pos, to: e.to, blind: e.blind });
        if let (Some(at), false) = (at, placed) {
            crate::mind::fix(g, gd, Some(at), "guards");
        }
    } else if g.gd[gd].state != S::Alert || g.gd[gd].last_known.is_none() || direct || g.time - g.gd[gd].last_seen_t > 1.0 {
        let err = if direct { 0.0 } else { m::min(4.0, 0.6 + d_loud * 0.08) * (1.3 - 0.3 * carry) };
        let a = crate::math_dice::sim_random(g) * m::PI * 2.0;
        let cand: V3 = [e.pos[0] + m::cos(a) * err, 0.0, e.pos[2] + m::sin(a) * err];
        if !placed {
            let reachable = !crate::scene_collision::nav_grid_is_blocked(g, cand[0], cand[2]) && {
                let from = g.ch[c].pos;
                crate::scene_collision::nav_grid_find_path(g, from, cand, None, None).is_some()
            };
            crate::mind::fix(g, gd, Some(if reachable { cand } else { e.pos }), "guards");
        }
    }
    g.gd[gd].witness.heard.loud_at = g.gd[gd].last_known;   // what the report came to in his head, written or not: the room fight's evidence reads it
    if calm {
        let late = 0.25 + m::min(1.2, d_loud / 25.0) + crate::math_dice::sim_random(g) * 0.4;
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, late);
    }
    if calm && unannounced(g, at_e) && !(kick && crate::frag::own_noise(g, gd, e.who)) {
        g.events[at_e].announced = Some(true);
        let line = if e.kind == K::Bang {
            if e.frag == Some(true) {
                "grenade! that was a grenade!"
            } else if e.charge == Some(true) {
                if crate::teams::defender_words(g, gd) { "that was a charge — they're breaching!" } else { "that was a charge!" }
            } else if g.gd[gd].hold || g.gd[gd].pinned {
                "flashbang!"
            } else {
                "flashbang! go, go!"
            }
        } else if kick {
            "that was a door going in — someone's inside!"
        } else if rel == ShotRelation::Own {
            "shots fired! on me!"
        } else if crate::teams::defender_words(g, gd) {
            "that was a shot — find them!"
        } else {
            "that was a shot — find him!"
        };
        crate::game::game_say(g, gd, line, Some(e.kind != K::Bang), None, None);
    }
}

/// A DOOR OR A THING KNOCKED.
fn knock(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, d, carry, heard_at, placed, dull, .. } = s;
    let c = g.gd[gd].ch;
    if (e.kind == K::Door || e.kind == K::Prop) && d < 12.0 && e.who != Some(g.ch[c].id) {
        let by_guard = e.who.unwrap_or(-1.0) >= 10.0 && !(crate::roomfight::fight_live(g) && step_is_threat(g, gd, StepIsThreatE { who: e.who }));
        let a = e.level.unwrap_or(0.2) * v3::clamp(1.0 - d / 12.0, 0.0, 1.0) * (if by_guard { 0.0 } else { 1.0 }) * carry * dull * crate::attention::attention_noise_k(g, gd);
        if a > 0.08 {
            let aw = g.gd[gd].awareness;
            crate::mind::set(g, gd, m::min(1.0, m::max(aw, m::min(if e.loud { 0.75 } else { 0.45 }, aw + a))), "guards");
            if !placed && (g.gd[gd].last_known.is_none() || g.gd[gd].state != S::Alert) {
                crate::mind::fix(g, gd, Some(heard_at), "guards");
            }
            if g.gd[gd].state == S::Patrol {
                g.gd[gd].react_t = 0.6;
            }
            small(g, gd, &e, heard_at);
            crate::chronicle::chron_heard(g, gd, e, None);
        }
    }
}

/// GLASS GOING.
fn glass(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, at_e, d, carry, heard_at, placed, dull, .. } = s;
    if e.kind == K::Glass && d < g.k.glass.hear_m {
        let a = e.level.unwrap_or(g.k.glass.hear_level) * v3::clamp(1.0 - d / g.k.glass.hear_m, 0.15, 1.0) * carry * dull;
        if a > 0.06 {
            let calm = was_calm(g, gd);
            let aw = g.gd[gd].awareness;
            crate::mind::set(g, gd, m::min(1.0, m::max(aw, m::min(0.85, 0.4 + a))), "guards");
            if !placed && (g.gd[gd].last_known.is_none() || g.gd[gd].state != S::Alert) {
                crate::mind::fix(g, gd, Some(heard_at), "guards");
            }
            if calm {
                g.gd[gd].react_t = m::max(g.gd[gd].react_t, 0.5);
            }
            small(g, gd, &e, heard_at);
            crate::chronicle::chron_heard(g, gd, e, None);
            if calm && unannounced(g, at_e) {
                g.events[at_e].announced = Some(true);
                let line = if d < 10.0 { "glass! — that was right here" } else { "that was glass going — somebody's out there" };
                crate::game::game_say(g, gd, line, Some(false), None, None);
            }
        }
    }
}

/// A FOOTFALL that is a threat's.
fn step(g: &mut Game, gd: GuardId, s: &Sound) {
    let &Sound { e, d, carry, heard_at, placed, keen, foot, .. } = s;
    if e.kind == K::Step
        && d < 9.0
        && !(g.player_invisible && crate::seats::is_man_id(g, e.who.unwrap_or(crate::consts::PLAYER_ID)))
        && step_is_threat(g, gd, StepIsThreatE { who: e.who })
    {
        let a = e.level.unwrap_or(0.3) * v3::clamp(1.0 - d / 9.0, 0.0, 1.0) * carry * keen * foot * crate::attention::attention_ear_k(g, gd);
        if a > 0.12 {
            let aw = g.gd[gd].awareness;
            crate::mind::set(g, gd, m::min(1.0, m::max(aw, m::min(0.6, aw + a * 0.5))), "guards");
            if !placed && (g.gd[gd].last_known.is_none() || g.gd[gd].state == S::Patrol) {
                crate::mind::fix(g, gd, Some(heard_at), "guards");
            }
            small(g, gd, &e, heard_at);
            crate::chronicle::chron_heard(g, gd, e, None);
            crate::attention::attention_step(g, gd, heard_at);
        }
    }
}

fn was_calm(g: &Game, gd: GuardId) -> bool {
    let st = g.gd[gd].state;
    st == crate::game::GuardState::Patrol || st == crate::game::GuardState::Suspicious
}

/// hearingCheck's `small`: what reached him and counted goes in his memory, and attention.ts is told which small thing it was. (guards.ts hearingCheck)
fn small(g: &mut Game, gd: GuardId, cur: &crate::game::GameEvent, where_: V3) {
    let now = g.time;
    let h = &mut g.gd[gd].witness.heard;
    h.small += 1.0;
    h.small_where = Some(where_);
    h.last_t = now;
    crate::attention::attention_heard(g, gd, *cur, where_);
}

/// Read live: a line once a noise, whoever got there first. (`!e.announced`)
fn unannounced(g: &Game, at: usize) -> bool {
    g.events[at].announced != Some(true)
}
