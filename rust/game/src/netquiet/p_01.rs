//! src/game/netquiet.ts lines 1 to 429
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- this file's data and private helpers

/// `Mind.say` and `Mind.last_kind` are indices of THIS list (netquiet.ts KINDS), not of the enum's own order.
static KINDS: [LineKind; 6] = [LineKind::Name, LineKind::Why, LineKind::Last, LineKind::Body, LineKind::Alone, LineKind::Ask];

/// (netquiet.ts LINES)
fn lines_of(kind: LineKind) -> &'static [&'static str] {
    match kind {
        LineKind::Name => &["{name}. … {name}.", "{name}? … {name}.", "{name}. {name}, answer up."],
        LineKind::Why => &["why's nobody answering?", "where is everybody? answer me.", "that's another of you quiet. come on."],
        LineKind::Last => &["somebody talk to me!", "anybody. anybody at all, talk to me!", "is anyone left on this net?"],
        LineKind::Body => &["he's down. nobody heard a thing?", "man down here. not a sound out of him.", "he's down, and nobody heard it?"],
        LineKind::Alone => &["that's my whole squad quiet. it's just me in here.", "nobody left in here but me.", "it's gone quiet in here. i'm on my own."],
        LineKind::Ask => &["all posts, answer up.", "anyone on this net, come in.", "who's still up? sound off."],
    }
}

/// (netquiet.ts LINES_NOW) a line as it is said now, by the line it stands for
static LINES_NOW: [(&str, &str); 2] = [
    ("that's another of you quiet. come on.", "that's another one of you not answering. come on."),
    ("that's my whole squad quiet. it's just me in here.", "my whole squad's quiet. it's just me in here."),
];

fn line_now(t: &str) -> Option<&'static str> {
    LINES_NOW.iter().find(|(was, _)| *was == t).map(|(_, now)| *now)
}

/// `KINDS.indexOf(kind)`
fn kind_index(kind: LineKind) -> f64 {
    KINDS.iter().position(|k| *k == kind).map_or(-1.0, |i| i as f64)
}

fn pos_of(g: &mut Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].pos
}

fn id_of(g: &mut Game, gd: GuardId) -> f64 {
    let c = g.gd[gd].ch;
    g.ch[c].id
}

// ---------------------------------------------------------------- the small questions

/// this file's lines, every kind's or one kind's, with a line's wording of now beside it (netquiet.ts netQuietLines)
pub fn net_quiet_lines(g: &mut crate::game::Game, kind: Option<crate::netquiet::LineKind>) -> Vec<String> {
    let mut l: Vec<&str> = Vec::new();
    match kind {
        Some(k) => l.extend_from_slice(lines_of(k)),
        None => {
            for k in KINDS.iter() {
                l.extend_from_slice(lines_of(*k));
            }
        }
    }
    let now: Vec<&str> = l.iter().filter_map(|t| line_now(t)).collect();
    l.extend(now);
    l.iter().map(|t| String::from(*t)).collect()
}

/// distance in plan (netquiet.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// both switches: settled on the page (netquiet.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    true
}

/// live, and the check's breakage is never set by the game (netquiet.ts fed)
pub fn fed(g: &mut crate::game::Game) -> bool {
    true
}

/// the loud noises his ears have counted (netquiet.ts loudOf)
pub fn loud_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let h = &g.gd[gd].witness.heard;
    h.shots + h.kick + h.bang
}

/// a man this file is about at all: a defender or night staff, not the hostage rescue's cast (netquiet.ts side)
pub fn side(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let team = g.gd[gd].team;
    g.gd[gd].civ.is_none()
        && (team == crate::teams::Team::Hostile || team == crate::teams::Team::Guard)
        && !(g.rescue.active && g.rescue.hostiles.contains(&gd))
}

/// …and alive (netquiet.ts ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead && side(g, gd)
}

/// …on the net: alive, not in Sam's arm, not a man who has run (netquiet.ts onNet)
pub fn on_net(g: &mut crate::game::Game, gd: GuardId) -> bool {
    ours(g, gd) && g.gd[gd].held.is_none() && !crate::runner::runner_owns(g, gd)
}

/// gone to his mates, whether or not they know it yet (netquiet.ts lostTo)
pub fn lost_to(g: &mut crate::game::Game, m: GuardId) -> bool {
    g.gd[m].state == crate::game::GuardState::Dead
        || g.gd[m].held.is_some()
        || crate::runner::runner_owns(g, m)
        || !g.guards.contains(&m)
}

/// his row, made on first meeting: the row itself, for a few lines (netquiet.ts mind)
pub fn mind<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::netquiet::Mind {
    if !g.m.netquiet.state_of.has(gd) {
        let (heard_t, cs) = (g.gd[gd].witness.heard.last_t, g.gd[gd].witness.heard.small);
        let cl = loud_of(g, gd);
        let n = Mind {
            quiet: 0.0,
            asked: JsSet::new(),
            known: JsSet::new(),
            paid: JsMap::new(),
            say: -1.0,
            say_t: 0.0,
            say_name: String::new(),
            bark: g.k.netquiet.bark_secs,
            last_kind: -1.0,
            body_t: 60.0,
            roused: 0.0,
            heard_t,
            cs,
            cl,
            fx: f64::NAN,
            fz: f64::NAN,
            whip_at: None,
            whip_in: 0.0,
            whip_t: 0.0,
            whip_held: 0.0,
            last: NONE,
            t: 0.0,
            mate_t: 0.0,
            news: 0.0,
            news_t: 60.0,
            eye: 0.0,
            eye_half: g.k.netquiet.eye_deg * v3::DEG,
            spot: None,
            yaw: 0.0,
            arc: 0.0,
            look: None,
            low: false,
        };
        g.m.netquiet.state_of.set(gd, n);
    }
    g.m.netquiet.state_of.get_mut(gd).unwrap()
}

/// a shock of this file's on one man; a steady man takes `steadyK` of it (netquiet.ts shock)
pub fn shock(g: &mut crate::game::Game, gd: GuardId, amount: f64) {
    if amount > 0.0 {
        let k = if matches!(crate::morale::training_of(g, gd), crate::morale::TrainingOf::Steady) { g.k.netquiet.steady_k } else { 1.0 };
        crate::morale::morale_shock(g, gd, amount * k);
    }
}

/// …for one dead man: what this adds over the most he has already taken for that man (netquiet.ts pay)
pub fn pay(g: &mut crate::game::Game, gd: GuardId, id_2: f64, amount: f64) {
    let n = mind(g, gd);
    let had = n.paid.get(crate::Num(id_2)).copied().unwrap_or(0.0);
    if amount > had {
        n.paid.set(crate::Num(id_2), amount);
        shock(g, gd, amount - had);
    }
}

/// a line owed `after` seconds from now; a later one replaces one not yet said (netquiet.ts owe)
pub fn owe(n: &mut crate::netquiet::Mind, kind: crate::netquiet::LineKind, name: &str, after: f64) {
    n.say = kind_index(kind);
    n.say_t = after;
    n.say_name = String::from(name);
}

/// he says a line of the kind's pool (netquiet.ts speak)
pub fn speak(g: &mut crate::game::Game, gd: GuardId, kind: crate::netquiet::LineKind, name: &str, radio: bool) {
    let pool: Vec<&str> = lines_of(kind).iter().map(|t| line_now(t).unwrap_or(*t)).collect();
    let len = pool.len() as f64;
    let at = js::rem(m::floor(crate::math_dice::sim_random(g) * len), len) as usize;
    let text = pool[at].replace("{name}", name);
    crate::game::game_say(g, gd, &text, Some(radio), None, None);
    let n = mind(g, gd);
    n.bark = 0.0;
    n.last_kind = kind_index(kind);
    g.stats.netquiet.lines += 1;
    let (id, t) = (id_of(g, gd), g.time);
    g.m.netquiet.netquiet_log.push(NetquietModNETQUIETLOGItem { id, t, kind, text, radio });
    if g.m.netquiet.netquiet_log.len() > 64 {
        g.m.netquiet.netquiet_log.remove(0);
    }
}

/// is anybody of his side up on the whole net, the men in `but` aside (netquiet.ts netUp)
pub fn net_up(g: &mut crate::game::Game, gd: GuardId, but: Option<&[GuardId]>) -> bool {
    let but = but.unwrap_or(&[]);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o != gd && g.gd[o].team == g.gd[gd].team && on_net(g, o) && !but.contains(&o) {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------- the feeds

/// the men of `silent` who do not answer `asker`, and the line he owes for it (netquiet.ts quietFor)
pub fn quiet_for(g: &mut crate::game::Game, asker: GuardId, silent: &[GuardId], hearers: &[GuardId]) {
    let (unanswered, heard_it, ask_beat) = (g.k.netquiet.unanswered, g.k.netquiet.heard_it, g.k.netquiet.ask_beat);
    mind(g, asker);
    let mut fresh: Option<GuardId> = None;
    for &m in silent {
        let id = id_of(g, m);
        let n = mind(g, asker);
        if !n.asked.has(crate::Num(id)) {
            n.asked.add(crate::Num(id));
            n.quiet += 1.0;
            g.stats.netquiet.quiet += 1;
            pay(g, asker, id, unanswered);
            fresh = Some(m);
        }
        for &h in hearers {
            let nh = mind(g, h);
            if nh.asked.has(crate::Num(id)) {
                continue;
            }
            nh.asked.add(crate::Num(id));
            nh.quiet += 1.0;
            pay(g, h, id, heard_it);
        }
    }
    g.stats.netquiet.asks += 1;
    let mut rung = mind(g, asker).quiet;
    for &h in hearers {
        rung = m::max(rung, mind(g, h).quiet);
    }
    if let Some(fresh) = fresh {
        let kind = if !net_up(g, asker, Some(silent)) {
            LineKind::Last
        } else if rung >= 2.0 {
            LineKind::Why
        } else {
            LineKind::Name
        };
        let name = crate::enemysquads::mate(g, fresh);
        owe(mind(g, asker), kind, &name, ask_beat);
    }
}

/// a radio check's silence: `men` = the net he asked on (netquiet.ts netQuietAsked)
pub fn net_quiet_asked(g: &mut crate::game::Game, asker: Option<GuardId>, who: GuardId, men: &[GuardId]) {
    let Some(asker) = asker else { return };
    if !on_net(g, asker) || asker == who {
        return;
    }
    let team = g.gd[asker].team;
    if team == crate::teams::Team::Guard {
        g.stats.netquiet.staff_asks += 1;
    }
    let mut hearers = Vec::new();
    for &m in men {
        if m != asker && m != who && g.gd[m].team == team && on_net(g, m) {
            hearers.push(m);
        }
    }
    quiet_for(g, asker, &[who], &hearers);
}

/// the last defenders' check closes: `silent` did not answer `asker`, `heard` did (netquiet.ts netQuietClosed)
pub fn net_quiet_closed(g: &mut crate::game::Game, asker: GuardId, silent: &[GuardId], heard: &[GuardId]) {
    if !on_net(g, asker) {
        return;
    }
    let team = g.gd[asker].team;
    let mut quiet = Vec::new();
    for &m in silent {
        if m != asker && g.gd[m].team == team && side(g, m) {
            quiet.push(m);
        }
    }
    if quiet.is_empty() {
        return;
    }
    g.stats.netquiet.stand_checks += 1;
    let mut hearers = Vec::new();
    for &m in heard {
        if m != asker && on_net(g, m) {
            hearers.push(m);
        }
    }
    quiet_for(g, asker, &quiet, &hearers);
}

/// the man he has found: the nearest in view he does not know of, else the nearest he does not know of (netquiet.ts bodyOf)
pub fn body_of(g: &mut crate::game::Game, finder: GuardId, men: &[GuardId]) -> Option<GuardId> {
    let at = pos_of(g, finder);
    let eye: V3 = [at[0], 1.5, at[2]];
    let team = g.gd[finder].team;
    let body_m = g.k.netquiet.body_m;
    let mut best: Option<GuardId> = None;
    let mut bs = m::INF;
    for &o in men {
        if o == finder || g.gd[o].state != crate::game::GuardState::Dead || g.gd[o].team != team || !side(g, o) {
            continue;
        }
        let c = g.gd[o].ch;
        let b = g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[c].pos);
        let d = flat(b, at);
        let in_view = d < body_m && !crate::scene_collision::segment_blocked(g, eye, [b[0], 0.3, b[2]], None, None, None);
        let id = g.ch[c].id;
        let knows = mind(g, finder).known.has(crate::Num(id));
        let s = d + if in_view { 0.0 } else { 100.0 } + if knows { 1000.0 } else { 0.0 };
        if s < bs {
            bs = s;
            best = Some(o);
        }
    }
    best
}

/// a body found: the finder, and the men of `men` he tells (netquiet.ts netQuietBody)
pub fn net_quiet_body(g: &mut crate::game::Game, finder: Option<GuardId>, men: &[GuardId]) {
    let Some(finder) = finder else { return };
    if !ours(g, finder) || crate::runner::runner_owns(g, finder) {
        return;
    }
    let (k_body, k_told, calm_secs, body_beat) = (g.k.netquiet.body, g.k.netquiet.body_told, g.k.netquiet.calm_secs, g.k.netquiet.body_beat);
    let again = mind(g, finder).body_t < 5.0;
    let body = body_of(g, finder, men);
    let id = match body {
        Some(b) => id_of(g, b),
        None => -1.0,
    };
    let team = g.gd[finder].team;
    let mut told = Vec::new();
    for &o in men {
        if o != finder && g.gd[o].team == team && on_net(g, o) {
            told.push(o);
        }
    }
    if again {
        if body.is_some() {
            // (two men lying together are one find: nothing is paid, but he and the men told know both are gone)
            let mut each = vec![finder];
            each.extend_from_slice(&told);
            for o in each {
                let nm = mind(g, o);
                nm.known.add(crate::Num(id));
                let had = nm.paid.get(crate::Num(id)).copied().unwrap_or(0.0);
                nm.paid.set(crate::Num(id), m::max(had, if o == finder { k_body } else { k_told }));
            }
        }
        return;
    }
    mind(g, finder).body_t = 0.0;
    g.stats.netquiet.bodies += 1;
    if team == crate::teams::Team::Guard {
        g.stats.netquiet.staff_bodies += 1;
    }
    if body.is_some() {
        mind(g, finder).known.add(crate::Num(id));
        pay(g, finder, id, k_body);
    } else {
        shock(g, finder, k_body);
    }
    for &o in &told {
        let nm = mind(g, o);
        if body.is_some() {
            nm.known.add(crate::Num(id));
            pay(g, o, id, k_told);
        } else {
            shock(g, o, k_told);
        }
    }
    let shot_t = g.gd[finder].witness.heard.last_shot_t;
    if !(shot_t >= 0.0 && g.time - shot_t < 3.0) && g.time - g.gd[finder].last_seen_t > calm_secs {
        owe(mind(g, finder), LineKind::Body, "", body_beat);
    }
}

/// has he an enemy in his eyes now, or had one inside `shooterSecs` (netquiet.ts foeInView)
pub fn foe_in_view(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.time - g.gd[gd].last_seen_t < g.k.netquiet.shooter_secs {
        return true;
    }
    let team = g.gd[gd].team;
    let player = g.player;
    if crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[player].down && !g.player_invisible && crate::guards::can_see(g, gd).visible {
        return true;
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let (o_team, o_ch) = (g.gd[o].team, g.gd[o].ch);
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead && crate::teams::is_hostile(g, team, o_team) && crate::guards::can_see_char(g, gd, o_ch, false).visible {
            return true;
        }
    }
    false
}

/// a man goes down: every mate with the fall in his eyes takes it, and knows he is gone (netquiet.ts netQuietFall)
pub fn net_quiet_fall(g: &mut crate::game::Game, victim: GuardId) {
    if !ours(g, victim) {
        return;
    }
    let (fall_seen, fall_unseen, leader_shock) = (g.k.netquiet.fall_seen, g.k.netquiet.fall_unseen, g.k.morale.leader_shock);
    let id = id_of(g, victim);
    let (v_team, v_ch) = (g.gd[victim].team, g.gd[victim].ch);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == victim || !on_net(g, o) || g.gd[o].team != v_team || g.time < g.gd[o].dazzled_until {
            continue;
        }
        if !crate::guards::can_see_char(g, o, v_ch, false).visible {
            continue;
        }
        mind(g, o);
        let seen = foe_in_view(g, o);
        mind(g, o).known.add(crate::Num(id));
        if crate::morale::saw_leader_fall(g, o) == 0.0 {
            let n = mind(g, o);
            let had = n.paid.get(crate::Num(id)).copied().unwrap_or(0.0);
            n.paid.set(crate::Num(id), m::max(had, leader_shock));
        }
        pay(g, o, id, if seen { fall_seen } else { fall_unseen });
        g.stats.netquiet.falls += 1;
        if !seen {
            g.stats.netquiet.unseen += 1;
        }
    }
}

// ---------------------------------------------------------------- the whip

/// where he whips to, for what his ears have just counted; never truer than his hearing (netquiet.ts noiseOf)
pub fn noise_of(g: &mut crate::game::Game, gd: GuardId, d: Option<RoomfightDefId>, small: bool, loud: bool) -> Option<V3> {
    let (fx, fz) = {
        let n = mind(g, gd);
        (n.fx, n.fz)
    };
    let whip_m = g.k.netquiet.whip_m;
    let mut bd = g.k.netquiet.whip_near_m;
    let (small_where, heard_last_t) = (g.gd[gd].witness.heard.small_where, g.gd[gd].witness.heard.last_t);
    let p = pos_of(g, gd);
    let me = id_of(g, gd);
    let team = g.gd[gd].team;
    let lk = g.gd[gd].last_known;
    let rf: Option<V3> = if small && small_where.is_some() {
        small_where
    } else if !loud {
        None
    } else if let Some(d) = d {
        let def = &g.tab.roomfight_def[d];
        if def.noise_at.is_some() && def.noise_t >= g.time - 1e-6 { def.noise_at } else { None }
    } else {
        match lk {
            Some(l) if l[0] != fx || l[2] != fz => Some(l),
            _ => None,
        }
    };
    let rf = rf?;
    if flat(rf, p) > whip_m {
        return None;
    }
    let eye: V3 = [p[0], 1.5, p[2]];
    let mut any = false;
    let mut best: Option<V3> = None;
    let mut i = 0;
    while i < g.events.len() {
        let (kind, time, who, level, pos) = {
            let e = &g.events[i];
            (e.kind, e.time, e.who, e.level, e.pos)
        };
        i += 1;
        if kind == crate::director::EventKind::LightOut || time < heard_last_t - 0.05 || who == Some(me) {
            continue;
        }
        if let Some(w) = who {
            let w_team = crate::teams::team_of_id(g, w);
            if !crate::teams::is_hostile(g, team, w_team) {
                continue;
            }
        }
        let big = ((kind == crate::director::EventKind::Shot || kind == crate::director::EventKind::GuardShot) && level.unwrap_or(0.0) >= 1.0) || kind == crate::director::EventKind::Bang || kind == crate::director::EventKind::Kick;
        if if big { !loud } else { !small || m::hyp2(pos[0] - p[0], pos[2] - p[2]) > 22.0 } {
            continue;
        }
        any = true;
        let d = flat(pos, rf);
        if d < bd && !crate::scene_collision::segment_blocked(g, eye, [pos[0], 1.0, pos[2]], None, None, None) {
            bd = d;
            best = Some(pos);
        }
    }
    if !any {
        return None;
    }
    let at = best.unwrap_or(rf);
    Some([at[0], 0.0, at[2]])
}

/// on the noise now: the swing made and time left of it, and not low on his spot (netquiet.ts whipping)
pub fn whipping(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(n) = g.m.netquiet.state_of.get(gd) else { return false };
    n.whip_at.is_some() && n.whip_in <= 0.0 && n.whip_t > 0.0 && !n.low
}

// ---------------------------------------------------------------- the last man's spot

/// the entries as seen from `p` by a crouched man: how many he has a line to, the arc they sit in, its middle (netquiet.ts arcAt)
pub fn arc_at(g: &mut crate::game::Game, e: &[crate::roomfight::Entry], p: V3) -> crate::netquiet::ArcAt {
    let eye: V3 = [p[0], g.k.netquiet.eye_m, p[2]];
    let mut seen = 0.0;
    let mut near = m::INF;
    let mut yaws: Vec<f64> = Vec::new();
    for en in e {
        let q: V3 = [en.at[0] - en.out[0] * 0.5, 1.2, en.at[2] - en.out[1] * 0.5];
        if !crate::scene_collision::segment_blocked(g, eye, q, None, None, None) {
            seen += 1.0;
        }
        yaws.push(m::atan2(q[0] - p[0], q[2] - p[2]));
        near = m::min(near, flat(q, p));
    }
    if yaws.is_empty() {
        return ArcAt { seen: 0.0, arc: 0.0, yaw: 0.0, near: 0.0 };
    }
    js::sort_by(&mut yaws, |a, b| *a - *b);
    let mut gap = -1.0;
    let mut at = 0;
    for i in 0..yaws.len() {
        let nx = if i + 1 < yaws.len() { yaws[i + 1] } else { yaws[0] + m::PI * 2.0 };
        let d = nx - yaws[i];
        if d > gap {
            gap = d;
            at = i;
        }
    }
    if yaws.len() == 1 {
        return ArcAt { seen, arc: 0.0, yaw: yaws[0], near };
    }
    let arc = m::PI * 2.0 - gap;
    let from = if at + 1 < yaws.len() { yaws[at + 1] } else { yaws[0] };
    ArcAt { seen, arc, yaw: v3::wrap_angle(from + arc / 2.0), near }
}

/// a spot's marks (netquiet.ts lowSpot `rate`)
#[derive(Clone, Copy, Debug)]
struct Rate {
    p: V3,
    seen: f64,
    arc: f64,
    near: f64,
    score: f64,
}

/// (netquiet.ts lowSpot `rate`)
fn rate(g: &mut Game, e: &[crate::roomfight::Entry], from: V3, lim: f64, p: V3) -> Rate {
    let a = arc_at(g, e, p);
    let back: V3 = [p[0] - m::sin(a.yaw) * 1.1, 1.0, p[2] - m::cos(a.yaw) * 1.1];
    let score = m::min(a.near, 8.0) - 0.35 * flat(p, from) + if crate::scene_collision::segment_blocked(g, [p[0], 1.0, p[2]], back, None, None, None) { 1.5 } else { 0.0 };
    Rate { p, seen: a.seen, arc: m::max(a.arc, lim), near: a.near, score }
}

/// (netquiet.ts lowSpot `better`)
fn better(a: &Rate, b: &Rate, by: f64) -> bool {
    if a.seen != b.seen {
        a.seen > b.seen
    } else if m::abs(a.arc - b.arc) > 1e-6 {
        a.arc < b.arc
    } else {
        a.score > b.score + by
    }
}

/// (netquiet.ts lowSpot `order`)
fn order_of(a: &Rate, b: &Rate) -> f64 {
    if better(a, b, 0.0) {
        -1.0
    } else if better(b, a, 0.0) {
        1.0
    } else {
        0.0
    }
}

/// `Array.sort` for lowSpot's comparators, which are NOT total orders (a tolerance makes them non-transitive; V8 takes that, Rust's `sort_by`,
/// so `js::sort_by`, may panic on it). Stable: binary insertion, what V8 does on a short run.
fn sort_spots(v: &mut [Rate], cmp: impl Fn(&Rate, &Rate) -> f64) {
    for start in 1..v.len() {
        let pivot = v[start];
        let (mut left, mut right) = (0, start);
        while left < right {
            let mid = left + (right - left) / 2;
            if cmp(&pivot, &v[mid]) < 0.0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        v.copy_within(left..start, left + 1);
        v[left] = pivot;
    }
}

/// where he goes: of his room's free floor, the spot with the most entries in view inside the narrowest arc; None = stay. No die. (netquiet.ts lowSpot)
pub fn low_spot(g: &mut crate::game::Game, gd: GuardId, r: RoomfightRoomId) -> Option<V3> {
    let (door_m, stay_m, max_m, lim) = (g.k.netquiet.door_m, g.k.netquiet.stay_m, g.k.netquiet.max_m, g.k.netquiet.arc_deg * v3::DEG);
    let (cs, w) = (g.col.nav.cell, g.col.nav.w);
    let e = g.tab.roomfight_room[r].entries.clone();
    let cells = g.tab.roomfight_room[r].cells.keys();
    let from = pos_of(g, gd);
    let here = rate(g, &e, from, lim, [from[0], 0.0, from[2]]);
    let coarse = cells.len() > 400;
    let mouth = !(here.near >= door_m);
    let mut all: Vec<Rate> = Vec::new();
    for i in cells {
        let i = i.0;
        let ix = js::rem(i, w);
        let iz = js::to_i32(i / w) as f64;
        if coarse && ((js::to_i32(ix) & 1) != 0 || (js::to_i32(iz) & 1) != 0) {
            continue;
        }
        let p: V3 = [(ix + 0.5) * cs, 0.0, (iz + 0.5) * cs];
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || flat(p, from) > max_m || flat(p, from) < 0.75 {
            continue;
        }
        all.push(rate(g, &e, from, lim, p));
    }
    let mut c: Vec<Rate> = all.iter().copied().filter(|r| r.near >= door_m).collect();
    if !c.is_empty() {
        if !mouth {
            c.retain(|r| better(r, &here, stay_m));
        }
        sort_spots(&mut c, order_of);
    } else {
        c = all.iter().copied().filter(|r| r.near > here.near + 0.25).collect();
        sort_spots(&mut c, |a, b| if m::abs(a.near - b.near) > 0.25 { b.near - a.near } else { order_of(a, b) });
    }
    for r in c.iter().take(6) {
        if !mouth && r.arc > m::PI {
            break;
        }
        if crate::scene_collision::nav_grid_find_path(g, from, r.p, None, None).is_some() {
            return Some(r.p);
        }
    }
    None
}

// ---------------------------------------------------------------- the tick

/// may this file act on him this frame (netquiet.ts free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.ai_enabled
        && !g.touring
        && !g.quiet_utility
        && g.gd[gd].held.is_none()
        && g.time >= g.gd[gd].dazzled_until
        && g.time >= g.gd[gd].stagger_until
        && !crate::morale::morale_owns(g, gd)
        && !crate::runner::runner_owns(g, gd)
}

/// is he the last: 2 = he had mates and none of his side is up anywhere · 1 = his squad is down and he knows it, nobody at his side · 0 = neither (netquiet.ts lastKind)
pub fn last_kind(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let mut had = crate::enemysquads::squad_of(g, gd).map_or(0, |q| q.men.len()) > 1;
    let mut up = false;
    let mut near = false;
    let team = g.gd[gd].team;
    let mate_m = g.k.netquiet.mate_m;
    let p = pos_of(g, gd);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].team != team || !side(g, o) {
            continue;
        }
        had = true;
        if !lost_to(g, o) {
            up = true;
            let q = pos_of(g, o);
            if flat(q, p) < mate_m && !crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], [q[0], 1.5, q[2]], None, None, None) {
                near = true;
            }
        }
    }
    if had && !up {
        return 2.0;
    }
    if !near && knows_squad(g, gd) { 1.0 } else { 0.0 }
}

/// every man of his squad is down or taken, and he knows it of each (netquiet.ts knowsSquad)
pub fn knows_squad(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(q) = crate::enemysquads::squad_of(g, gd) else { return false };
    if q.men.len() < 2 {
        return false;
    }
    for &o in &q.men {
        if o == gd {
            continue;
        }
        let known = lost_to(g, o) && {
            let id = id_of(g, o);
            let n = mind(g, gd);
            n.asked.has(crate::Num(id)) || n.known.has(crate::Num(id))
        };
        if !known {
            return false;
        }
    }
    true
}

/// the last man at his act, and this file's to keep where he is (netquiet.ts mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let (last_beat, give_up) = (g.k.netquiet.last_beat, g.k.netquiet.give_up);
    let Some(n) = g.m.netquiet.state_of.get(gd) else { return false };
    n.last >= SAID && n.last <= LOW && !(n.last == SAID && n.t > last_beat + give_up)
}

/// at his post: holding his room, on nobody's scripted leg (netquiet.ts posted)
pub fn posted(g: &mut crate::game::Game, gd: GuardId, d: RoomfightDefId) -> bool {
    g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold && g.gd[gd].hold && g.gd[gd].script.is_none()
}

/// once a frame, after morale's tick: the lines owed, the whip, a step of the last man's act (netquiet.ts netQuietTick)
pub fn net_quiet_tick(g: &mut crate::game::Game, dt: f64) {
    g.m.netquietfeed.netfeed.live = true;
    for f in crate::netquietfeed::net_feed_take(g) {
        if let Some(by) = f.by {
            if !g.guards.contains(&by) {
                continue;
            }
        }
        match (f.what, f.who) {
            (crate::netquietfeed::NetFeedWhat::Asked, Some(who)) => {
                g.stats.netquiet.squad_asks += 1;
                net_quiet_asked(g, f.by, who, &f.men);
            }
            (crate::netquietfeed::NetFeedWhat::Body, _) => {
                g.stats.netquiet.squad_bodies += 1;
                net_quiet_body(g, f.by, &f.men);
            }
            _ => {}
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !ours(g, gd) {
            g.m.netquiet.state_of.delete(gd);
            crate::netquietfeed::net_feed_hold(g, gd, false);
            continue;
        }
        if crate::runner::runner_owns(g, gd) {
            if let Some(n) = g.m.netquiet.state_of.get_mut(gd) {
                n.last = NONE;
                n.t = 0.0;
                n.mate_t = 0.0;
                n.spot = None;
                n.look = None;
                n.whip_at = None;
                n.whip_t = 0.0;
                n.say = -1.0;
                n.low = false;
            }
            crate::netquietfeed::net_feed_hold(g, gd, false);
            continue;
        }
        tick_man(g, gd, dt);
    }
}

/// one man's frame of the tick; a `return` here is the TypeScript loop's `continue` (netquiet.ts netQuietTick)
fn tick_man(g: &mut Game, gd: GuardId, dt: f64) {
    let k = g.k.netquiet.clone();
    mind(g, gd);
    let ok = free(g, gd);
    let eyes = g.time - g.gd[gd].last_seen_t < k.calm_secs;
    let mut d: Option<RoomfightDefId> = None;
    for &o in &g.fight.defs {
        if g.tab.roomfight_def[o].gd == gd {
            d = Some(o);
            break;
        }
    }
    let patrol = g.gd[gd].state == crate::game::GuardState::Patrol;
    let n = mind(g, gd);
    n.bark = m::min(n.bark + dt, 60.0);
    n.body_t = m::min(n.body_t + dt, 60.0);
    n.low = false;
    let news = n.quiet + n.asked.len() as f64 + n.known.len() as f64;
    if news != n.news {
        n.news = news;
        n.news_t = 0.0;
    } else {
        n.news_t = m::min(n.news_t + dt, 60.0);
    }
    n.roused = if patrol { 0.0 } else { m::min(n.roused + dt, 60.0) };
    let held = mine(g, gd);
    crate::netquietfeed::net_feed_hold(g, gd, held);
    // the line owed
    let n = mind(g, gd);
    if n.say >= 0.0 {
        n.say_t -= dt;
        if n.say_t <= 0.0 {
            let kind = KINDS.get(n.say as usize).copied().unwrap_or(LineKind::Name);
            if ok && !eyes && (kind == LineKind::Last || n.bark >= k.bark_secs) {
                n.say = -1.0;
                let name = n.say_name.clone();
                let radio = kind != LineKind::Body || net_up(g, gd, None);
                speak(g, gd, kind, &name, radio);
            } else if n.say_t < -k.say_hold {
                n.say = -1.0;
            }
        }
    }
    // the whip: only to a noise HE heard, at the place his ears put it
    let heard_t = g.gd[gd].witness.heard.last_t;
    if heard_t > mind(g, gd).heard_t {
        let heard_small = g.gd[gd].witness.heard.small;
        let small = heard_small > mind(g, gd).cs;
        let heard_loud = loud_of(g, gd);
        let loud = heard_loud > mind(g, gd).cl;
        let on = whipping(g, gd);
        if (small || loud) && ok && !eyes && crate::morale::rung_of(g, gd) >= 1.0 && mind(g, gd).last != BACK && !(on && mind(g, gd).whip_held < k.whip_hold) {
            if let Some(at) = noise_of(g, gd, d, small, loud) {
                if flat(at, pos_of(g, gd)) > 0.5 {
                    let n = mind(g, gd);
                    let swing = !on && !(n.whip_at.is_some() && n.whip_in > 0.0);
                    if swing {
                        n.whip_in = k.whip_shift;
                    }
                    let moved = match n.whip_at {
                        Some(w) => flat(at, w) > 1.0,
                        None => true,
                    };
                    if moved {
                        n.whip_at = Some(at);
                        n.whip_held = 0.0;
                    }
                    n.whip_t = k.whip_secs;
                    if swing {
                        g.stats.netquiet.whips += 1;
                    }
                }
            }
        }
        if ok && mind(g, gd).last == LOW && (small || loud) {
            rouse(g, gd);
        }
        let (now_t, now_small) = (g.gd[gd].witness.heard.last_t, g.gd[gd].witness.heard.small);
        let now_loud = loud_of(g, gd);
        let n = mind(g, gd);
        n.heard_t = now_t;
        n.cs = now_small;
        n.cl = now_loud;
    }
    let lk = g.gd[gd].last_known;
    let n = mind(g, gd);
    n.fx = lk.map_or(f64::NAN, |l| l[0]);
    n.fz = lk.map_or(f64::NAN, |l| l[2]);
    if n.whip_at.is_some() {
        if n.whip_in > 0.0 {
            n.whip_in -= dt;
        } else {
            n.whip_held += dt;
            n.whip_t -= dt;
            if n.whip_t <= 0.0 || eyes || !ok {
                n.whip_at = None;
                n.whip_t = 0.0;
            }
        }
    }
    // the last man
    if n.last == NONE {
        if !ok || eyes || n.say >= 0.0 {
            return;
        }
        let quiet_none = !js::truthy(n.quiet) && n.known.is_empty();
        if g.gd[gd].state == crate::game::GuardState::Patrol && quiet_none {
            return;
        }
        let l = last_kind(g, gd);
        if !js::truthy(l) {
            return;
        }
        if l == 2.0 && !js::truthy(mind(g, gd).quiet) && !knows_squad(g, gd) {
            // nobody is left, but he cannot know it: roused `calmSecs`, he asks the net himself, and it is the silence after that has him act
            let n = mind(g, gd);
            if n.roused >= k.calm_secs && n.bark >= k.bark_secs {
                speak(g, gd, LineKind::Ask, "", true);
                mind(g, gd).quiet += 1.0;
                g.stats.netquiet.quiet += 1;
                g.stats.netquiet.asks += 1;
                g.stats.netquiet.self_asks += 1;
                shock(g, gd, k.unanswered);
                owe(mind(g, gd), LineKind::Last, "", k.ask_beat);
            }
            return;
        }
        let n = mind(g, gd);
        if n.news_t < k.ask_beat {
            return;
        }
        let kind = if l == 2.0 { LineKind::Last } else { LineKind::Alone };
        let said = n.last_kind == kind_index(kind) && n.bark < 10.0;
        if !said && n.bark < k.bark_secs {
            return;
        }
        n.last = SAID;
        n.t = 0.0;
        g.stats.netquiet.last += 1;
        let held = mine(g, gd);
        crate::netquietfeed::net_feed_hold(g, gd, held);
        if !said {
            speak(g, gd, kind, "", true);
        }
        return;
    }
    n.t += dt;
    let lk = last_kind(g, gd);
    let n = mind(g, gd);
    n.mate_t = if js::truthy(lk) { 0.0 } else { n.mate_t + dt };
    if n.mate_t >= k.calm_secs {
        // a living mate has been with him `calmSecs`: up, and his owner's again
        n.last = NONE;
        n.t = 0.0;
        n.mate_t = 0.0;
        n.spot = None;
        n.look = None;
        crate::netquietfeed::net_feed_hold(g, gd, false);
        g.stats.netquiet.released += 1;
        return;
    }
    if n.last == SAID {
        if n.t < k.last_beat || !ok || eyes {
            return;
        }
        let Some(d) = d else {
            n.last = DONE;
            crate::netquietfeed::net_feed_hold(g, gd, false);
            return;
        };
        let out = g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Free && g.gd[gd].script.is_none();
        if !posted(g, gd, d) && !out {
            return;
        }
        let room = g.tab.roomfight_def[d].room;
        let to = match room {
            Some(r) if !g.tab.roomfight_room[r].entries.is_empty() => match low_spot(g, gd, r) {
                Some(s) => Some(s),
                None if out => Some(v3::copy(g.tab.roomfight_def[d].station)),
                None => None,
            },
            _ => None,
        };
        if let (Some(to), Some(r)) = (to, room) {
            let e = g.tab.roomfight_room[r].entries.clone();
            let a = arc_at(g, &e, to);
            let n = mind(g, gd);
            n.spot = Some(v3::copy(to));
            n.yaw = a.yaw;
            n.arc = a.arc;
            n.last = BACK;
            n.t = 0.0;
            n.whip_at = None;
            g.stats.netquiet.backs += 1;
            // the room fight's own scripted leg, walked backwards, his gun on the doors
            let here = pos_of(g, gd);
            let def = &mut g.tab.roomfight_def[d];
            def.mode = crate::roomfight::DefMode::Move;
            def.move_kind = crate::roomfight::DefMoveKind::Fallback;
            def.came_by = None;
            def.move_t = 0.0;
            def.stall_at = v3::copy(here);
            def.stall_t = 0.0;
            def.rally_at = f64::NAN;
            g.gd[gd].pinned = false;
            crate::squad::order(
                g,
                gd,
                crate::squad::OrderPatch {
                    goal: Some(Some(v3::copy(to))),
                    speed: Some(Some(k.back_speed)),
                    face: Some(Some(a.yaw)),
                    strafe: Some(Some(true)),
                    aim_at: Some(Some([to[0] + m::sin(a.yaw) * 6.0, 1.1, to[2] + m::cos(a.yaw) * 6.0])),
                    upper: Some(Some(crate::character::UpperMode::Aim)),
                    stance: Some(Some(crate::character::Stance::None)),
                    crouch: Some(Some(false)),
                    ..Default::default()
                },
            );
        } else {
            settle(g, gd, d);
        }
        return;
    }
    if n.last == BACK {
        let Some(d) = d else {
            n.last = DONE;
            crate::netquietfeed::net_feed_hold(g, gd, false);
            return;
        };
        if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Move {
            settle(g, gd, d);
        }
        return;
    }
    if n.last != LOW {
        return;
    }
    let Some(spot) = n.spot else { return };
    let Some(d) = d else { return };
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Hold {
        return;
    }
    if flat(pos_of(g, gd), spot) > 1.0 {
        settle(g, gd, d);
    }
    mind(g, gd).low = ok;
    // where his eyes are, low on his gun: read off his own pose once he is down and still, eased
    let c = g.gd[gd].ch;
    let hd = crate::character::character_head_dir(g, c);
    let chest = g.ch[c].chest_dir;
    let (lx, ly, lz) = (chest[0] + hd[0], chest[1] + hd[1], chest[2] + hd[2]);
    let len = m::hyp3(lx, ly, lz);
    if g.ch[c].anim.crouch > k.crouch - 0.15 && g.gd[gd].speed < 0.3 && len > 1e-3 {
        let e = m::min(1.0, dt * 2.0);
        let flat_k = m::hyp2(lx, lz) / len;
        let aim_yaw = g.ch[c].aim_yaw;
        let n = mind(g, gd);
        n.eye += (v3::wrap_angle(m::atan2(lx, lz) - aim_yaw) - n.eye) * e;
        n.eye_half += (m::acos(v3::clamp(m::cos(k.eye_deg * v3::DEG) / m::max(flat_k, 1e-3), -1.0, 1.0)) - n.eye_half) * e;
    }
    watch(g, gd);
    // nothing seen, nothing heard for `alertSecs`: he is no longer 'alert'; the spot, the crouch and the arc stay
    let alert = g.gd[gd].state == crate::game::GuardState::Alert;
    let long_enough = mind(g, gd).t > k.alert_secs;
    if alert && ok && long_enough && g.time - m::max(g.gd[gd].last_seen_t, g.gd[gd].witness.heard.last_t) > k.alert_secs {
        ease(g, gd);
    } else if !alert && ok {
        wary(g, gd);
    }
}

/// on his gun: roused, at the floor's own alert line (netquiet.ts rouse)
pub fn rouse(g: &mut crate::game::Game, gd: GuardId) {
    if g.gd[gd].state != crate::game::GuardState::Alert {
        g.gd[gd].state = crate::game::GuardState::Alert;
    }
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.92);
}

/// …and off it again: wary, not alert (netquiet.ts ease)
pub fn ease(g: &mut crate::game::Game, gd: GuardId) {
    g.gd[gd].state = crate::game::GuardState::Suspicious;
    g.stats.netquiet.eased += 1;
}

/// the awareness he does not fall under, low on his spot (netquiet.ts wary)
pub fn wary(g: &mut crate::game::Game, gd: GuardId) {
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, g.k.netquiet.wary);
}

/// down where he stands, on his gun: the arc as it reads from here (netquiet.ts settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, d: RoomfightDefId) {
    let at = pos_of(g, gd);
    let p: V3 = [at[0], 0.0, at[2]];
    let entries = g.tab.roomfight_def[d].room.map(|r| g.tab.roomfight_room[r].entries.clone()).filter(|e| !e.is_empty());
    let (arc, yaw) = match entries {
        Some(e) => {
            let a = arc_at(g, &e, p);
            (a.arc, a.yaw)
        }
        None => {
            let c = g.gd[gd].ch;
            (m::PI * 2.0, g.ch[c].body_yaw)
        }
    };
    let n = mind(g, gd);
    n.spot = Some(p);
    n.yaw = yaw;
    n.arc = arc;
    n.last = LOW;
    n.t = 0.0;
    n.low = true;
    g.stats.netquiet.low += 1;
    watch(g, gd);
    rouse(g, gd);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot(seen: f64, arc: f64, near: f64, score: f64) -> Rate {
        Rate { p: [0.0, 0.0, 0.0], seen, arc, near, score }
    }

    #[test]
    fn owed_lines_index_the_files_own_list_not_the_enums_order() {
        let mut n = Mind::zeroed();
        owe(&mut n, LineKind::Name, "x", 1.0);
        assert_eq!(n.say, 0.0);
        owe(&mut n, LineKind::Why, "", 1.0);
        assert_eq!(n.say, 1.0);
        owe(&mut n, LineKind::Body, "", 2.0);
        assert_eq!(n.say, 3.0);
        assert_eq!(n.say_t, 2.0);
        owe(&mut n, LineKind::Ask, "", 1.0);
        assert_eq!(n.say, 5.0);
    }

    #[test]
    fn a_write_through_the_row_lands_and_asking_makes_no_row() {
        let mut g = Game::blank();
        let gd = GuardId(3);
        assert!(!mine(&mut g, gd));
        assert!(!whipping(&mut g, gd));
        assert!(!g.m.netquiet.state_of.has(gd));
        g.m.netquiet.state_of.set(gd, Mind::zeroed());
        owe(mind(&mut g, gd), LineKind::Why, "x", 1.4);
        assert_eq!(g.m.netquiet.state_of.get(gd).unwrap().say, 1.0);
        mind(&mut g, gd).last = SAID;
        assert!(mine(&mut g, gd));
        mind(&mut g, gd).t = g.k.netquiet.last_beat + g.k.netquiet.give_up + 1.0;
        assert!(!mine(&mut g, gd));
        let n = mind(&mut g, gd);
        n.whip_at = Some([1.0, 0.0, 1.0]);
        n.whip_t = 1.0;
        assert!(whipping(&mut g, gd));
    }

    #[test]
    fn every_reworded_line_stands_for_a_line_of_a_pool() {
        for (was, _) in LINES_NOW.iter() {
            assert!(KINDS.iter().any(|k| lines_of(*k).contains(was)), "{}", was);
        }
    }

    #[test]
    fn a_spot_wins_by_what_it_sees_then_its_arc_then_its_score() {
        assert!(better(&spot(2.0, 3.0, 0.0, 0.0), &spot(1.0, 0.1, 0.0, 9.0), 0.0));
        assert!(better(&spot(1.0, 0.5, 0.0, 0.0), &spot(1.0, 1.0, 0.0, 9.0), 0.0));
        assert!(!better(&spot(1.0, 1.0, 0.0, 1.5), &spot(1.0, 1.0, 0.0, 1.0), 1.0));
        assert!(better(&spot(1.0, 1.0, 0.0, 2.5), &spot(1.0, 1.0, 0.0, 1.0), 1.0));
    }

    #[test]
    fn the_spots_sort_is_stable_and_takes_a_comparator_with_a_tolerance() {
        // near-ties chain (each within 0.25 of the next, the ends further apart): not transitive; the score breaks the ties, here all equal
        let mut v: Vec<Rate> = (0..40).map(|i| spot(1.0, 1.0, i as f64 * 0.2, 0.0)).collect();
        sort_spots(&mut v, |a, b| if m::abs(a.near - b.near) > 0.25 { b.near - a.near } else { order_of(a, b) });
        assert_eq!(v.len(), 40);
        let mut w = vec![spot(1.0, 1.0, 0.0, 1.0), spot(1.0, 1.0, 1.0, 2.0), spot(1.0, 1.0, 2.0, 1.0), spot(2.0, 1.0, 3.0, 0.0)];
        sort_spots(&mut w, order_of);
        assert_eq!([w[0].near, w[1].near, w[2].near, w[3].near], [3.0, 1.0, 0.0, 2.0]);
    }
}
