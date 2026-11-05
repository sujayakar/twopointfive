//! src/game/picture.ts lines 1 to 452
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the module's state, known to be there (never held across a call that takes `g`)
macro_rules! st { ($g:expr) => { $g.m.picture.states.as_mut().unwrap() } }
/// one man's picture in the table, known to be there
macro_rules! pic { ($g:expr, $gd:expr) => { st!($g).men.get_mut($gd).unwrap() } }
/// a picture by its owner: None = Sam's
macro_rules! row { ($g:expr, $o:expr) => { match $o { None => &mut st!($g).sam, Some(x) => st!($g).men.get_mut(x).unwrap() } } }

// ---------------------------------------------------------------- the seats
/// run in the game as this seat has it: as it stands when that seat is on Game already (viewOfBody's "a view handed in is kept"), else with_seat
fn in_seat(g: &mut Game, seat: PlayerId, run: &mut dyn FnMut(&mut Game)) {
    if seat == g.player { run(g); } else { crate::seats::with_seat(g, seat, run); }
}
/// eachSeat(g, f) over one view: seats::each_seat's closure takes two `&mut Game` and cannot be lent (seats.ts eachSeat)
fn on_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    if g.seats.len() == 1 { f(g); return; }
    let mut i = 0;
    while i < g.seats.len() {
        let s = g.seats[i];
        i += 1;
        let man = crate::cmd::seat_man(g, s);
        in_seat(g, man, &mut *f);
    }
}

// ---------------------------------------------------------------- the state
/// S.men.get or a new one, in `s`: picOf's literal
fn ensure_pic(s: &mut State, gd: GuardId, now: f64, look_secs: f64) {
    if !s.men.has(gd) {
        let look_t = now + (s.men.len() % 8) as f64 * look_secs / 8.0;
        s.men.set(gd, Pic { owner: Some(gd), look_t, ..Pic::zeroed() });
    }
}
/// a man's ideas, copied in their order; None = he has no picture
fn ideas_copy(g: &Game, gd: GuardId) -> Option<Vec<Idea>> {
    let p = g.m.picture.states.as_ref()?.men.get(gd)?;
    Some(p.ideas.values().copied().collect())
}

/// the module's state, made if need be: stateOf's literal (picture.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::picture::State {
    if g.m.picture.states.is_none() {
        g.m.picture.states = Some(State {
            men: JsMap::new(),
            sam: Pic { owner: None, look_t: -1e9, ..Pic::zeroed() },
            tell_t: -1e9,
            done: JsSet::new(),
            view: None,
            view_t: -1e9,
        });
    }
    g.m.picture.states.as_mut().unwrap()
}

/// a man's picture, made if need be (picture.ts picOf)
pub fn pic_of<'g>(g: &'g mut crate::game::Game, gd: GuardId, now: f64) -> &'g mut crate::picture::Pic {
    let look_secs = g.k.picture.look_secs;
    let s = state_of(g);
    ensure_pic(s, gd, now, look_secs);
    s.men.get_mut(gd).unwrap()
}

/// (picture.ts pictureReset)
pub fn picture_reset(g: &mut crate::game::Game) {
    g.m.picture.states = None;
}

/// (picture.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// (picture.ts floor)
pub fn floor(p: V3) -> V3 {
    [p[0], 0.0, p[2]]
}

/// (picture.ts nameOf)
pub fn name_of(g: &mut crate::game::Game, w: crate::picture::Who) -> String {
    match w {
        Who::Player => String::from("Player"),
        Who::Guard(o) => g.gd[o].callsign.clone(),
    }
}

/// (picture.ts up)
pub fn up(g: &mut crate::game::Game, w: crate::picture::Who) -> bool {
    match w {
        Who::Player => !g.pl[g.player].down,
        Who::Guard(o) => g.gd[o].state != crate::game::GuardState::Dead && g.ch[g.gd[o].ch].alive,
    }
}

/// (picture.ts posOf)
pub fn pos_of(g: &mut crate::game::Game, w: crate::picture::Who) -> V3 {
    match w {
        Who::Player => g.ch[g.pl[g.player].ch].pos,
        Who::Guard(o) => g.ch[g.gd[o].ch].pos,
    }
}

/// no longer anybody's enemy to place: a unit off the floor, a man who dropped his gun and ran (picture.ts gone)
pub fn gone(g: &mut crate::game::Game, w: crate::picture::Who) -> bool {
    match w {
        Who::Player => false,
        Who::Guard(o) => !g.guards.contains(&o) || crate::runner::runner_fled(g, o),
    }
}

/// can his eye make anything out AT a place it has a line to: near enough, or lit (picture.ts madeOut)
pub fn made_out(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    if flat(g.ch[g.gd[gd].ch].pos, p) <= g.k.picture.made_out_m {
        return true;
    }
    let e = crate::guards::light_estimate_at(g, [p[0], 1.1, p[2]], None, None, None, None);
    crate::game::game_visibility_of(g, e) >= g.k.picture.made_out
}

/// a man who keeps a picture: a living armed man of an AI side (picture.ts fighter)
pub fn fighter(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead && g.ch[g.gd[gd].ch].alive && g.gd[gd].civ.is_none() && g.gd[gd].team != crate::teams::Team::Civilian
}

/// the enemies there are for a side, as a list of men and Sam (picture.ts foesOf)
pub fn foes_of(g: &mut crate::game::Game, team: crate::teams::Team) -> Vec<crate::picture::Who> {
    let mut out = Vec::new();
    if team != crate::teams::Team::Player && crate::teams::is_hostile(g, team, crate::teams::Team::Player) {
        out.push(Who::Player);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if g.gd[o].civ.is_none() && crate::teams::is_hostile(g, team, ot) {
            out.push(Who::Guard(o));
        }
    }
    out
}

// ---------------------------------------------------------------- what reaches him
/// learn with g.time in hand, for a row that lives in `g`
fn learn_now(time: f64, p: &mut Pic, who: Who, at: V3, t: f64, how: IdeaHow, by: Option<GuardId>) {
    if let Some(i) = p.ideas.get_mut(who) {
        i.at = floor(at);
        i.t = t;
        i.how = how;
        i.by = by;
        i.lost = false;
    } else {
        p.ideas.set(who, Idea { who, at: floor(at), t, since: time, how, by, lost: false });
    }
    p.downs.delete(who);
}

/// `p` is the picture's owner: None = Sam's (picture.ts learn)
pub fn learn(g: &mut crate::game::Game, p: Option<GuardId>, who: crate::picture::Who, at: V3, t: f64, how: crate::picture::IdeaHow, by: Option<GuardId>) {
    let time = g.time;
    learn_now(time, row!(g, p), who, at, t, how, by);
}

/// one man's look about him, on his row of the table: every enemy his eye has, the place his senses work from, the believed places he sees empty, the men he knows are down (picture.ts look)
pub fn look(g: &mut crate::game::Game, gd: GuardId) {
    let now = g.time;
    g.stats.picture.looks += 1;
    if g.gd[gd].held.is_some() {
        return;
    }
    let team = g.gd[gd].team;
    for who in foes_of(g, team) {
        if !up(g, who) || gone(g, who) {
            continue;
        }
        let rate = match who {
            Who::Player => {
                let vis = crate::guards::can_see(g, gd);
                if !vis.visible {
                    continue;
                }
                crate::guards::sight_gain(g, gd, vis.dist)
            }
            Who::Guard(o) => {
                let c = g.gd[o].ch;
                let vis = crate::guards::can_see_char(g, gd, c, false);
                if !vis.visible {
                    continue;
                }
                crate::guards::sight_gain_on(g, gd, o, vis.dist)
            }
        };
        if rate > g.k.picture.glimpse {
            let at = pos_of(g, who);
            learn(g, Some(gd), who, at, now, IdeaHow::Seen, None);
            g.stats.picture.seen += 1;
        }
    }
    let forget_secs = g.k.picture.forget_secs;
    let foe = g.gd[gd].foe.map(|f| match f {
        crate::game::Foe::Guard(o) => Who::Guard(o),
        crate::game::Foe::Player => Who::Player,
    });
    if let (Some(f), true, Some(known)) = (foe, g.gd[gd].script.is_none(), g.gd[gd].last_known) {
        let fix_t = g.gd[gd].fix_t;
        if up(g, f) && !gone(g, f) && now - fix_t < forget_secs {
            let i = pic!(g, gd).ideas.get(f).copied();
            let (heard_m, walk_mps) = (g.k.picture.heard_m, g.k.picture.walk_mps);
            let take = match i {
                None => true,
                Some(i) => {
                    let own = i.how == IdeaHow::Seen || i.how == IdeaHow::Heard;
                    fix_t > i.t + 0.05 && !(i.how == IdeaHow::Seen && now - i.t < 0.5) && (!own || flat(known, i.at) <= heard_m + 2.0 + (fix_t - i.t) * walk_mps * 2.0)
                }
            };
            if take {
                learn(g, Some(gd), f, known, m::min(now, fix_t), IdeaHow::Fix, None);
                g.stats.picture.fixes += 1;
            }
        }
    }
    let blind = now < g.gd[gd].dazzled_until;
    let mut at = 0;
    while let Some(who) = pic!(g, gd).ideas.next_key(&mut at) {
        let i = *pic!(g, gd).ideas.get(who).unwrap();
        if now - i.t > forget_secs || gone(g, who) {
            pic!(g, gd).ideas.delete(who);
            g.stats.picture.forgot += 1;
            continue;
        }
        if !up(g, who) {
            let (fell, body) = match who {
                Who::Player => (now, floor(g.ch[g.pl[g.player].ch].pos)),
                Who::Guard(o) => (g.gd[o].died_at, crate::guards::body_pos(g, o)),
            };
            if (i.how == IdeaHow::Seen && fell - i.t < 1.0)
                || (!blind
                    && (crate::guards::point_in_view(g, gd, [body[0], 0.3, body[2]], Some(16.0), Some(75.0)) || crate::guards::point_in_view(g, gd, [body[0], 0.9, body[2]], Some(16.0), Some(75.0)))
                    && made_out(g, gd, body))
            {
                let p = pic!(g, gd);
                p.ideas.delete(who);
                p.downs.set(who, now);
                g.stats.picture.downs += 1;
            }
            continue;
        }
        if !i.lost
            && now - i.t > 1.0
            && !blind
            && flat(pos_of(g, who), i.at) > 1.2
            && crate::guards::point_in_view(g, gd, [i.at[0], 1.1, i.at[2]], Some(14.0), Some(58.0))
            && crate::guards::point_in_view(g, gd, [i.at[0], 0.4, i.at[2]], Some(14.0), Some(58.0))
            && made_out(g, gd, i.at)
        {
            pic!(g, gd).ideas.get_mut(who).unwrap().lost = true;
            g.stats.picture.lost += 1;
        }
    }
}

/// Sam's own eyes, for the squad's net, on S.sam: a hostile the fog shows him with a line from his head (picture.ts lookSam)
pub fn look_sam(g: &mut crate::game::Game) {
    let now = g.time;
    let c = g.pl[g.player].ch;
    if g.pl[g.player].down {
        return;
    }
    let cpos = g.ch[c].pos;
    let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(cpos, [0.0, 1.65, 0.0]),
    };
    let fwd = crate::character::character_forward(g, c);
    let thr = crate::guards::team_through(g);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if g.gd[o].civ.is_some() || !crate::teams::is_hostile(g, crate::teams::Team::Player, ot) || !crate::teams::is_hostile(g, ot, crate::teams::Team::Operative) {
            continue;
        }
        let who = Who::Guard(o);
        let q = g.ch[g.gd[o].ch].pos;
        if !up(g, who) {
            if st!(g).sam.ideas.has(who) && !crate::scene_collision::segment_blocked(g, eye, [q[0], 0.3, q[2]], None, None, None) {
                let sam = &mut st!(g).sam;
                sam.ideas.delete(who);
                sam.downs.set(who, now);
            }
            continue;
        }
        if crate::runner::runner_fled(g, o) {
            continue;
        }
        let d = flat(q, cpos);
        if d > g.k.picture.reach_m {
            continue;
        }
        let shown = if crate::fogofwar::fog_of_war_active(g) {
            !crate::fogofwar::fog_of_war_hidden(g, o)
        } else {
            d < g.k.picture.made_out_m || ((q[0] - cpos[0]) * fwd[0] + (q[2] - cpos[2]) * fwd[2] > 0.0 && g.gd[o].visibility >= g.k.picture.made_out)
        };
        if shown && (!crate::scene_collision::segment_blocked(g, eye, [q[0], 1.1, q[2]], Some(0.02), Some(thr), None) || !crate::scene_collision::segment_blocked(g, eye, [q[0], 1.6, q[2]], Some(0.02), Some(thr), None)) {
            learn(g, None, who, q, now, IdeaHow::Seen, None);
        }
    }
    let forget_secs = g.k.picture.forget_secs;
    let mut at = 0;
    while let Some(who) = st!(g).sam.ideas.next_key(&mut at) {
        let t = st!(g).sam.ideas.get(who).unwrap().t;
        if now - t > forget_secs || gone(g, who) {
            st!(g).sam.ideas.delete(who);
        }
    }
}

/// this frame's gunfire, placed by ear (picture.ts hear)
pub fn hear(g: &mut crate::game::Game) {
    let now = g.time;
    let mut ei = 0;
    while ei < g.events.len() {
        let e = g.events[ei];
        ei += 1;
        let id = match e.who {
            Some(id) if e.kind == crate::director::EventKind::Shot || e.kind == crate::director::EventKind::GuardShot => id,
            _ => continue,
        };
        if state_of(g).done.has(e.serial) {
            continue;
        }
        state_of(g).done.add(e.serial);
        let who = if crate::seats::is_man_id(g, id) {
            Who::Player
        } else {
            let mut found = None;
            for &x in g.guards.iter() {
                if g.ch[g.gd[x].ch].id == id {
                    found = Some(Who::Guard(x));
                    break;
                }
            }
            match found {
                Some(w) => w,
                None => continue,
            }
        };
        let team = match who {
            Who::Player => crate::teams::Team::Player,
            Who::Guard(o) => g.gd[o].team,
        };
        let loud = e.level.unwrap_or(0.0) >= 1.0;
        let mut gi = 0;
        while gi < g.guards.len() {
            let gd = g.guards[gi];
            gi += 1;
            let gt = g.gd[gd].team;
            if who == Who::Guard(gd) || !fighter(g, gd) || g.gd[gd].held.is_some() || !crate::teams::is_hostile(g, gt, team) {
                continue;
            }
            let p = pic_of(g, gd, now);
            if let Some(i) = p.ideas.get(who) {
                if i.how == IdeaHow::Seen && now - i.t < 1.0 {
                    continue;
                }
            }
            if now - p.heard.get(who).copied().unwrap_or(-1e9) < 0.8 {
                continue;
            }
            p.heard.set(who, now);
            let gpos = g.ch[g.gd[gd].ch].pos;
            if flat(e.pos, gpos) > (if loud { 80.0 } else { 22.0 }) {
                continue;
            }
            let ear = v3::add(gpos, [0.0, 1.5, 0.0]);
            let prop = crate::scene_collision::sound_paths_propagate(g, v3::add(e.pos, [0.0, 1.0, 0.0]), ear, None);
            if !loud {
                let stretched = prop.path_len * crate::weather::weather_hear_mask(g, ear);
                if stretched >= g.k.picture.pop_m * prop.carry * m::max(0.3, e.level.unwrap_or(0.6) / 0.6) + 0.5 {
                    continue;
                }
            }
            if !prop.direct && !prop.routed {
                continue;
            }
            let mut at = if prop.direct { e.pos } else { prop.via };
            if !prop.direct && loud {
                let q: V3 = [m::round(e.pos[0] / 2.0) * 2.0, 0.0, m::round(e.pos[2] / 2.0) * 2.0];
                if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
                    at = q;
                }
            }
            learn_now(now, pic_of(g, gd, now), who, at, now, IdeaHow::Heard, None);
            g.stats.picture.heard += 1;
        }
    }
}

/// are two men on one net: a's call reaches b. Each is his picture's owner, None = Sam's: one picture a man, so `a === b` is owner against owner (picture.ts onNet)
pub fn on_net(g: &mut crate::game::Game, a: Option<GuardId>, b: Option<GuardId>) -> bool {
    let bo = match b {
        Some(bo) if a != b => bo,
        _ => return false,
    };
    if !fighter(g, bo) || g.gd[bo].held.is_some() {
        return false;
    }
    if crate::runner::runner_owns(g, bo) {
        return false;
    }
    if let Some(ao) = a {
        if crate::runner::runner_owns(g, ao) {
            return false;
        }
    }
    let ao = match a {
        None => return g.gd[bo].team == crate::teams::Team::Operative,
        Some(ao) => ao,
    };
    if !fighter(g, ao) || g.gd[ao].held.is_some() || g.gd[ao].team != g.gd[bo].team {
        return false;
    }
    if g.gd[ao].team != crate::teams::Team::Hostile {
        return true;
    }
    let qa = crate::enemysquads::squad_of(g, ao);
    let qb = crate::enemysquads::squad_of(g, bo);
    if let (Some(qa), Some(qb)) = (qa, qb) {
        if qa.def.id == qb.def.id {
            return true;
        }
    }
    flat(g.ch[g.gd[ao].ch].pos, g.ch[g.gd[bo].ch].pos) <= g.k.picture.shout_m
}

/// the calls: what a man has seen or heard for tellSecs reaches his net, and the men he knows are down (picture.ts tell)
pub fn tell(g: &mut crate::game::Game) {
    let now = g.time;
    let tell_secs = g.k.picture.tell_secs;
    let mut all: Vec<Option<GuardId>> = vec![None];
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if st!(g).men.has(gd) {
            all.push(Some(gd));
        }
    }
    for &a in all.iter() {
        if let Some(ao) = a {
            if !fighter(g, ao) || g.gd[ao].held.is_some() {
                continue;
            }
        }
        // (a's own lists cannot change in this walk: nobody calls himself)
        let (a_ideas, a_downs): (Vec<(Who, Idea)>, Vec<(Who, f64)>) = {
            let ap: &mut Pic = row!(g, a);
            (ap.ideas.iter().map(|(k, v)| (k, *v)).collect(), ap.downs.iter().map(|(k, v)| (k, *v)).collect())
        };
        for &b in all.iter() {
            if !on_net(g, a, b) {
                continue;
            }
            for &(who, i) in a_ideas.iter() {
                let bp: &mut Pic = row!(g, b);
                let j = bp.ideas.get(who).copied();
                if i.lost {
                    if let Some(j) = j {
                        if !j.lost && j.t <= i.t + 1e-6 {
                            bp.ideas.get_mut(who).unwrap().lost = true;
                        }
                    }
                    continue;
                }
                if i.how == IdeaHow::Told || i.how == IdeaHow::Fix || now - i.since < tell_secs || bp.downs.has(who) {
                    continue;
                }
                if let Some(j) = j {
                    if j.t >= i.t - tell_secs {
                        continue;
                    }
                }
                learn_now(now, bp, who, i.at, i.t, IdeaHow::Told, a);
                g.stats.picture.told += 1;
            }
            for &(who, t) in a_downs.iter() {
                let bp: &mut Pic = row!(g, b);
                if now - t >= tell_secs && !bp.downs.has(who) {
                    bp.ideas.delete(who);
                    bp.downs.set(who, now);
                }
            }
        }
    }
}

/// an idea handed to one man from outside his senses, told, `age` seconds old (picture.ts pictureTell)
pub fn picture_tell(g: &mut crate::game::Game, gd: GuardId, who: crate::picture::Who, at: V3, age: Option<f64>) {
    let age = age.unwrap_or(0.0);
    if !fighter(g, gd) {
        return;
    }
    let now = g.time;
    learn_now(now, pic_of(g, gd, now), who, at, now - age, IdeaHow::Told, None);
}

/// once a frame, after everybody has moved (picture.ts pictureTick)
pub fn picture_tick(g: &mut crate::game::Game, _dt: f64) {
    if !g.ai_enabled || g.touring {
        return;
    }
    state_of(g);
    let now = g.time;
    let look_secs = g.k.picture.look_secs;
    for gd in st!(g).men.keys() {
        if !g.guards.contains(&gd) {
            st!(g).men.delete(gd);
        }
    }
    let mut ops = false;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !fighter(g, gd) {
            continue;
        }
        if g.gd[gd].team == crate::teams::Team::Operative {
            ops = true;
        }
        let look_t = pic_of(g, gd, now).look_t;
        if now >= look_t {
            pic!(g, gd).look_t = m::max(look_t + look_secs, now - look_secs);
            let id = g.ch[g.gd[gd].ch].id;
            let view = crate::seats::view_of_body(g, crate::seats::ViewOfBodyC { id });
            in_seat(g, view, &mut |g: &mut Game| look(g, gd));
        }
        let p = pic!(g, gd);
        if (p.step.is_some() || p.step_bad.is_some()) && now - p.step_ask.unwrap_or(-1e9) > 0.25 {
            p.step = None;
            p.step_bad = None;
            p.step_in = Some(-1.0);
        }
    }
    if ops && now >= st!(g).sam.look_t {
        st!(g).sam.look_t = now + look_secs;
        on_each_seat(g, &mut |g: &mut Game| look_sam(g));
    }
    if !g.events.is_empty() {
        hear(g);
    }
    if now - st!(g).tell_t >= 0.5 {
        st!(g).tell_t = now;
        tell(g);
    }
}

// ---------------------------------------------------------------- what the rules ask
/// a man's ideas (picture.ts ideasOf)
pub fn ideas_of(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::picture::Idea> {
    ideas_copy(g, gd).unwrap_or_default()
}

/// what an idea counts for now: whole while fresh, less as it ages, less again once its place was seen empty; 0 = forgotten (picture.ts ideaWeight)
pub fn idea_weight(g: &mut crate::game::Game, i: crate::picture::Idea) -> f64 {
    weight_at(&g.k.picture, g.time, i)
}
fn weight_at(k: &crate::knobs::PictureK, now: f64, i: Idea) -> f64 {
    let age = now - i.t;
    if age >= k.forget_secs {
        return 0.0;
    }
    let w = if age <= k.fresh_secs {
        1.0
    } else if age <= k.stale_secs {
        1.0 - (1.0 - k.stale_k) * (age - k.fresh_secs) / (k.stale_secs - k.fresh_secs)
    } else {
        k.stale_k * (1.0 - (age - k.stale_secs) / (k.forget_secs - k.stale_secs))
    };
    w * (if i.lost { k.lost_k } else { 1.0 })
}

/// how far the man may have walked since, in metres (picture.ts ideaReach)
pub fn idea_reach(g: &mut crate::game::Game, i: crate::picture::Idea) -> f64 {
    reach_at(&g.k.picture, g.time, i)
}
fn reach_at(k: &crate::knobs::PictureK, now: f64, i: Idea) -> f64 {
    let age = now - i.t;
    m::min(k.walk_max, (if i.how == IdeaHow::Heard || i.how == IdeaHow::Fix { k.heard_m } else { 0.0 }) + m::max(0.0, age - k.fresh_secs) * k.walk_mps)
}

/// the places he may be by now: six ways off the idea at its reach in whole metres, each a straight walk from it (picture.ts maybeOf)
pub fn maybe_of(g: &mut crate::game::Game, i: crate::picture::Idea) -> Vec<V3> {
    let r = m::floor(idea_reach(g, i));
    let mut out = Vec::new();
    if r < 1.0 {
        return out;
    }
    for n in 0..6 {
        let a = n as f64 * m::PI / 3.0;
        let q: V3 = [i.at[0] + m::sin(a) * r, 0.0, i.at[2] + m::cos(a) * r];
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && crate::scene_collision::nav_grid_walkable(g, i.at, q) {
            out.push(q);
        }
    }
    out
}

/// hidden only if chest AND head are (picture.ts lineTo)
pub fn line_to(g: &mut crate::game::Game, from: V3, p: V3, h: f64) -> bool {
    if !g.m.picture.viewing {
        g.stats.picture.lines += 1;
    }
    if flat(from, p) > g.k.picture.reach_m {
        return false;
    }
    let e: V3 = [from[0], g.k.picture.eye_h, from[2]];
    if !crate::scene_collision::segment_blocked(g, e, [p[0], h, p[2]], None, None, None) {
        return true;
    }
    let head = h + g.k.picture.head_up;
    !crate::scene_collision::segment_blocked(g, e, [p[0], head, p[2]], None, None, None)
}

/// (picture.ts exposedTo)
pub fn exposed_to(g: &mut crate::game::Game, i: crate::picture::Idea, p: V3, h: f64) -> f64 {
    let w = idea_weight(g, i);
    if w <= 0.0 {
        return 0.0;
    }
    if !i.lost && line_to(g, i.at, p, h) {
        return w;
    }
    let maybe_k = g.k.picture.maybe_k;
    for q in maybe_of(g, i) {
        if line_to(g, q, p, h) {
            return w * maybe_k;
        }
    }
    if i.lost && line_to(g, i.at, p, h) { w * maybe_k } else { 0.0 }
}

/// how exposed is this spot as far as he knows: the heaviest idea with a line to a man standing (or kneeling) there (picture.ts exposedAt)
pub fn exposed_at(g: &mut crate::game::Game, gd: GuardId, p: V3, low: Option<bool>) -> f64 {
    let low = low.unwrap_or(false);
    let ideas = match ideas_copy(g, gd) {
        Some(l) if !l.is_empty() => l,
        _ => return 0.0,
    };
    if !g.m.picture.viewing {
        g.stats.picture.asks += 1;
    }
    let mut e = 0.0;
    let h = if low { g.k.picture.low_h } else { g.k.picture.chest_h };
    for i in ideas {
        if e >= 1.0 {
            break;
        }
        e = m::max(e, exposed_to(g, i, p, h));
    }
    e
}

/// the function exposureAsker returns, asked of one spot: exposedAt's own answer, each idea's places dealt once, the heaviest asked first (picture.ts:314)
fn exposure_ask(g: &mut Game, ask: &mut ExposureAsker, p: V3) -> f64 {
    g.stats.picture.asks += 1;
    let ExposureAsker::Ideas { h, rows } = ask;
    let h = *h;
    let mut e = 0.0;
    for x in rows.iter_mut() {
        if e >= x.1 {
            break;
        }
        let i = x.0;
        if !i.lost && line_to(g, i.at, p, h) {
            e = x.1;
            continue;
        }
        let k = x.1 * g.k.picture.maybe_k;
        if e >= k {
            continue;
        }
        if x.2.is_none() {
            x.2 = Some(maybe_of(g, i));
        }
        let mut hit = false;
        for &q in x.2.as_ref().unwrap().iter() {
            if line_to(g, q, p, h) {
                hit = true;
                break;
            }
        }
        if hit || (i.lost && line_to(g, i.at, p, h)) {
            e = k;
        }
    }
    e
}

/// the same question asked of many spots at one moment; None = nothing known (picture.ts exposureAsker)
pub fn exposure_asker(g: &mut crate::game::Game, gd: GuardId, low: Option<bool>) -> Option<crate::picture::ExposureAsker> {
    let low = low.unwrap_or(false);
    let ideas = match ideas_copy(g, gd) {
        Some(l) if !l.is_empty() => l,
        _ => return None,
    };
    let h = if low { g.k.picture.low_h } else { g.k.picture.chest_h };
    let mut l: Vec<(Idea, f64, Option<Vec<V3>>)> = Vec::new();
    for i in ideas {
        let w = idea_weight(g, i);
        if w > 0.0 {
            l.push((i, w, None));
        }
    }
    if l.is_empty() {
        return None;
    }
    js::sort_by(&mut l, |a, b| b.1 - a.1);
    Some(ExposureAsker::Ideas { h, rows: l })
}

/// where he believes his enemies to be, the ideas firm enough to steer a walk by (picture.ts believedOf)
pub fn believed_of(g: &mut crate::game::Game, gd: GuardId) -> Vec<V3> {
    let mut out = Vec::new();
    for i in ideas_of(g, gd) {
        if !i.lost && g.time - i.t <= g.k.picture.stale_secs {
            out.push(i.at);
        }
    }
    out
}

/// a rule's dry asks: no count ticked (picture.ts pictureQuiet)
pub fn picture_quiet<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = g.m.picture.viewing;
    g.m.picture.viewing = true;
    let r = fn_(g);
    g.m.picture.viewing = was;
    r
}

/// the worst any of these men knows of the spot (picture.ts exposedFor)
pub fn exposed_for(g: &mut crate::game::Game, men: &[GuardId], p: V3, low: Option<bool>) -> f64 {
    let low = low.unwrap_or(false);
    let mut e = 0.0;
    for &gd in men {
        if e >= 1.0 {
            break;
        }
        e = m::max(e, exposed_at(g, gd, p, Some(low)));
    }
    e
}

/// knownPlaces's `put`: the freshest word of each man; a same-age word that he is not there holds
fn known_put(best: &mut JsMap<Who, (V3, f64, bool)>, who: Who, at: V3, t: f64, lost: bool) {
    match best.get_mut(who) {
        Some(j) if !(t > j.1 + 1e-6) => {
            if lost && t >= j.1 - 1e-6 {
                j.2 = true;
            }
        }
        _ => {
            best.set(who, (at, t, lost));
        }
    }
}
/// knownPlaces's `of`, over a picture's ideas copied
fn known_of(g: &mut Game, best: &mut JsMap<Who, (V3, f64, bool)>, ideas: Vec<Idea>, now: f64, max_age: f64) {
    for i in ideas {
        if now - i.t <= max_age && !gone(g, i.who) {
            known_put(best, i.who, i.at, i.t, i.lost);
        }
    }
}

/// where these men, between them, believe their enemies to be: each enemy once, at the freshest word no older than max_age (picture.ts knownPlaces)
pub fn known_places(g: &mut crate::game::Game, men: &[GuardId], max_age: f64) -> Vec<crate::picture::KnownPlacesItem> {
    state_of(g);
    let now = g.time;
    let squad = men.iter().any(|&gd| g.gd[gd].team == crate::teams::Team::Operative);
    let mut best: JsMap<Who, (V3, f64, bool)> = JsMap::new();
    for &gd in men {
        if fighter(g, gd) {
            if let Some(ideas) = ideas_copy(g, gd) {
                known_of(g, &mut best, ideas, now, max_age);
            }
        }
    }
    if squad {
        let sam: Vec<Idea> = st!(g).sam.ideas.values().copied().collect();
        known_of(g, &mut best, sam, now, max_age);
        if crate::fogofwar::fog_of_war_active(g) {
            let mut i = 0;
            while i < g.guards.len() {
                let h = g.guards[i];
                i += 1;
                let ht = g.gd[h].team;
                if g.gd[h].civ.is_some() || !crate::teams::is_hostile(g, crate::teams::Team::Operative, ht) || gone(g, Who::Guard(h)) {
                    continue;
                }
                if let Some(s) = crate::fogofwar::fog_of_war_sighting(g, h) {
                    if s.age >= 0.0 && s.age <= max_age {
                        known_put(&mut best, Who::Guard(h), s.at, now - s.age, false);
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for (who, j) in best.iter() {
        if !j.2 {
            out.push(KnownPlacesItem { who, at: v3::copy(j.0), age: now - j.1 });
        }
    }
    out
}

/// do these men know of any enemy at all (picture.ts anyIdea)
pub fn any_idea(g: &mut crate::game::Game, men: &[GuardId]) -> bool {
    let s = match g.m.picture.states.as_ref() {
        Some(s) => s,
        None => return false,
    };
    men.iter().any(|&gd| s.men.get(gd).map_or(0, |p| p.ideas.len()) > 0)
}

/// several whole deals of marks, safest first, by the worst any of the men knows of a deal's worst mark, in tenths (picture.ts safestSets)
pub fn safest_sets(g: &mut crate::game::Game, men: &[GuardId], sets: &[Vec<crate::picture::SafestSetsSetsItemItem>]) -> Vec<Vec<crate::picture::SafestSetsSetsItemItem>> {
    if sets.len() < 2 || !any_idea(g, men) {
        return sets.to_vec();
    }
    let mut ranked: Vec<(usize, f64)> = Vec::new();
    for (i, s) in sets.iter().enumerate() {
        let mut e = 0.0;
        for q in s.iter() {
            e = m::max(e, exposed_for(g, men, q.pos, None));
        }
        ranked.push((i, m::round(e * 10.0)));
    }
    js::sort_by(&mut ranked, |a, b| js::or(a.1 - b.1, a.0 as f64 - b.0 as f64));
    ranked.iter().map(|x| sets[x.0].clone()).collect()
}

const N4: [[f64; 2]; 4] = [[1.0, 0.0], [-1.0, 0.0], [0.0, 1.0], [0.0, -1.0]];

/// where good cover is as far as he knows: floor inside r, beside something solid, no known enemy's line to it; best first (picture.ts coverNear)
pub fn cover_near(g: &mut crate::game::Game, gd: GuardId, around: V3, r: f64, n: Option<f64>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>) -> Vec<crate::picture::CoverSpot> {
    let n = n.unwrap_or(6.0);
    let mut ok = ok;
    if ideas_of(g, gd).is_empty() {
        return Vec::new();
    }
    let cell = g.col.nav.cell;
    let mut out: Vec<CoverSpot> = Vec::new();
    let x0 = m::round(around[0] / cell) * cell + cell / 2.0;
    let z0 = m::round(around[2] / cell) * cell + cell / 2.0;
    let span = m::ceil(r) as i32;
    for ix in -span..=span {
        for iz in -span..=span {
            let q: V3 = [x0 + ix as f64, 0.0, z0 + iz as f64];
            let d = flat(q, around);
            if d > r || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
                continue;
            }
            if !N4.iter().any(|o| crate::scene_collision::nav_grid_is_blocked(g, q[0] + o[0] * 0.75, q[2] + o[1] * 0.75)) {
                continue;
            }
            if let Some(f) = ok.as_mut() {
                if !f(g, q) {
                    continue;
                }
            }
            let stand = exposed_at(g, gd, q, None);
            let low = if stand > 0.0 { exposed_at(g, gd, q, Some(true)) } else { 0.0 };
            if stand > 0.0 && low > 0.0 {
                continue;
            }
            let mut edge = false;
            for o in N4.iter() {
                let s: V3 = [q[0] + o[0], 0.0, q[2] + o[1]];
                if !crate::scene_collision::nav_grid_is_blocked(g, s[0], s[2]) && crate::scene_collision::nav_grid_walkable(g, q, s) && exposed_at(g, gd, s, None) > 0.0 {
                    edge = true;
                    break;
                }
            }
            out.push(CoverSpot { at: q, stand, low, edge, score: (if stand > 0.0 { 0.7 } else { 1.0 }) + (if edge { 0.4 } else { 0.0 }) - 0.3 * d / m::max(1.0, r) });
        }
    }
    js::sort_by(&mut out, |a, b| b.score - a.score);
    out.truncate(n as usize);
    out
}

/// pictureStep's `free`: a straight walk off the place, a stride nearer a believed man at most, a body's room from his own side, the asker's word
fn step_free(g: &mut Game, q: V3, gd: GuardId, mark: V3, sam: V3, bad: Option<V3>, firm: &[V3], ok: &mut Option<&mut dyn FnMut(&mut Game, V3) -> bool>) -> bool {
    if flat(q, mark) > 0.05 && !crate::scene_collision::nav_grid_walkable(g, mark, q) {
        return false;
    }
    if flat(q, sam) < 0.9 {
        return false;
    }
    if let Some(b) = bad {
        if flat(q, b) < 0.6 {
            return false;
        }
    }
    for &f in firm {
        let d = flat(q, f);
        if d < 3.0 || d < flat(mark, f) - 1.0 {
            return false;
        }
    }
    let team = g.gd[gd].team;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead || crate::teams::is_hostile(g, team, ot) {
            continue;
        }
        if flat(g.ch[g.gd[o].ch].pos, q) < 0.8 {
            return false;
        }
        if let Some(os) = st!(g).men.get(o).and_then(|p| p.step) {
            if flat(os.at, q) < 0.9 {
                return false;
            }
        }
        if let Some(og) = g.gd[o].script.as_ref().and_then(|s| s.goal) {
            if flat(og, q) < 0.8 {
                return false;
            }
        }
    }
    match ok {
        Some(f) => f(g, q),
        None => true,
    }
}
/// pictureStep's `holds`: the cover holds a foot either way
fn step_holds(g: &mut Game, gd: GuardId, at: V3, low: bool) -> bool {
    for o in [[0.3, 0.0], [-0.3, 0.0], [0.0, 0.3], [0.0, -0.3]] {
        if !(crate::scene_collision::nav_grid_is_blocked(g, at[0] + o[0], at[2] + o[1]) || exposed_at(g, gd, [at[0] + o[0], 0.0, at[2] + o[1]], Some(low)) <= 0.0) {
            return false;
        }
    }
    true
}

/// a step off a place in a known enemy's line, asked every frame of a man on a mark the squad chose; None: his place is his (picture.ts pictureStep)
pub fn picture_step(g: &mut crate::game::Game, gd: GuardId, key: u32, mark: V3, ok: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, busy: Option<bool>) -> Option<crate::picture::PictureStep> {
    let mut ok = ok;
    let mut mark = mark;
    let busy = busy.unwrap_or(false);
    match g.m.picture.states.as_ref() {
        Some(s) if s.men.has(gd) => {}
        _ => return None,
    }
    let now = g.time;
    let (step_at, step_free_at, step_rest, step_after, step_secs, step_m) = {
        let k = &g.k.picture;
        (k.step_at, k.step_free, k.step_rest, k.step_after, k.step_secs, k.step_m)
    };
    let p = pic!(g, gd);
    let asked = p.step_ask.unwrap_or(-1e9);
    let gap = now - asked > 0.25 || now < asked;
    p.step_ask = Some(now);
    if let Some(s) = p.step.as_mut() {
        if s.key.is_none() {
            s.key = Some(key);
        }
    }
    if let Some(s) = p.step {
        if s.key != Some(key) || gap {
            p.step = None;
        }
    }
    if gap {
        p.step_in = Some(-1.0);
        p.step_bad = None;
    }
    if p.ideas.is_empty() {
        p.step = None;
        p.step_in = Some(-1.0);
        return None;
    }
    let x = p.step;
    if let Some(xs) = x {
        if now >= xs.look_t {
            pic!(g, gd).step.as_mut().unwrap().look_t = now + 0.5;
            if exposed_at(g, gd, xs.mark, None) < step_free_at {
                let p = pic!(g, gd);
                p.step = None;
                p.step_in = Some(-1.0);
                p.step_next = Some(now + step_rest);
                return None;
            }
            let off = flat(g.ch[g.gd[gd].ch].pos, xs.at) > 0.6 && now - xs.t0 > step_secs;
            if off {
                pic!(g, gd).step_bad = Some(xs.at);
            }
            if off || exposed_at(g, gd, xs.at, Some(xs.low)) >= step_at {
                let p = pic!(g, gd);
                p.step = None;
                p.step_in = Some(now - step_after);
                p.step_next = Some(now);
                mark = xs.mark;
            }
        }
        if pic!(g, gd).step.is_some() {
            return Some(PictureStep { at: xs.at, low: xs.low });
        }
    }
    let p = pic!(g, gd);
    if busy {
        p.step_in = Some(-1.0);
        return None;
    }
    if p.step_for.map_or(true, |f| flat(f, mark) > 0.4) {
        p.step_for = Some(floor(mark));
        // (X === P.step here only when he came in with no step: one he had was just cleared, and the count it set stands)
        if x.is_none() {
            p.step_in = Some(-1.0);
        }
    }
    if now < p.step_next.unwrap_or(-1e9) {
        return None;
    }
    p.step_next = Some(now + 0.25);
    if exposed_at(g, gd, mark, None) < step_at {
        pic!(g, gd).step_in = Some(-1.0);
        return None;
    }
    let p = pic!(g, gd);
    let si = p.step_in.unwrap_or(-1.0);
    if si < 0.0 || now < si {
        p.step_in = Some(now);
    }
    if now - p.step_in.unwrap() < step_after {
        return None;
    }
    let bad = p.step_bad;
    let sam = g.ch[g.pl[g.player].ch].pos;
    let mut firm: Vec<V3> = Vec::new();
    for i in ideas_copy(g, gd).unwrap_or_default() {
        if !i.lost && idea_weight(g, i) >= step_at {
            firm.push(i.at);
        }
    }
    let spots = {
        let mut free = |g: &mut Game, q: V3| step_free(g, q, gd, mark, sam, bad, &firm, &mut ok);
        cover_near(g, gd, mark, step_m, Some(8.0), Some(&mut free))
    };
    let mut best: Option<CoverSpot> = None;
    for c in spots.iter() {
        if c.stand <= 0.0 && step_holds(g, gd, c.at, false) {
            best = Some(*c);
            break;
        }
    }
    if best.is_none() {
        for c in spots.iter() {
            if c.low <= 0.0 && step_holds(g, gd, c.at, true) {
                best = Some(*c);
                break;
            }
        }
    }
    let best = match best {
        Some(b) => b,
        None => {
            pic!(g, gd).step_next = Some(now + step_rest);
            return None;
        }
    };
    let low = !(best.stand <= 0.0 && step_holds(g, gd, best.at, false));
    let at = floor(best.at);
    pic!(g, gd).step = Some(Step { key: Some(key), mark: floor(mark), at, low, look_t: now + 0.5, t0: now });
    g.stats.picture.stepped += 1;
    Some(PictureStep { at, low })
}

/// the step he is on, if any (picture.ts pictureStepOf)
pub fn picture_step_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::picture::PictureStepOf> {
    let x = g.m.picture.states.as_ref()?.men.get(gd)?.step?;
    Some(PictureStepOf { at: x.at, mark: x.mark, low: x.low })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn knobs() -> crate::knobs::PictureK {
        let mut k = crate::knobs::PictureK::zeroed();
        k.fresh_secs = 2.0;
        k.stale_secs = 20.0;
        k.stale_k = 0.4;
        k.forget_secs = 90.0;
        k.lost_k = 0.5;
        k.walk_mps = 1.2;
        k.walk_max = 6.0;
        k.heard_m = 2.0;
        k
    }
    fn idea(t: f64, how: IdeaHow, lost: bool) -> Idea {
        Idea { who: Who::Player, at: [0.0; 3], t, since: t, how, by: None, lost }
    }

    /// the TypeScript's own bits (ideaWeight, ideaReach under EXACT, the page's numbers, g.time 37.3)
    #[test]
    fn weight_and_reach_are_the_typescripts_bits() {
        let rows: [(f64, IdeaHow, bool, u64, u64); 6] = [
            (37.1, IdeaHow::Seen, false, 0x3ff0000000000000, 0x0000000000000000),
            (30.7, IdeaHow::Seen, false, 0x3feb17e4b17e4b19, 0x4016147ae147ae11),
            (9.9, IdeaHow::Heard, false, 0x3fd6e4ca759d1c62, 0x4018000000000000),
            (9.9, IdeaHow::Told, true, 0x3fc6e4ca759d1c62, 0x4018000000000000),
            (33.3, IdeaHow::Fix, false, 0x3fedddddddddddde, 0x401199999999999a),
            (-60.0, IdeaHow::Seen, false, 0x0000000000000000, 0x4018000000000000),
        ];
        let k = knobs();
        for (t, how, lost, w, r) in rows {
            let i = idea(t, how, lost);
            assert_eq!(weight_at(&k, 37.3, i).to_bits(), w, "weight at {t}");
            assert_eq!(reach_at(&k, 37.3, i).to_bits(), r, "reach at {t}");
        }
    }

    #[test]
    fn learning_again_keeps_since_and_takes_him_off_the_downs() {
        let mut p = Pic::zeroed();
        p.downs.set(Who::Player, 1.0);
        learn_now(5.0, &mut p, Who::Player, [1.0, 2.0, 3.0], 4.0, IdeaHow::Seen, None);
        assert!(!p.downs.has(Who::Player));
        let i = *p.ideas.get(Who::Player).unwrap();
        assert_eq!((i.at, i.t, i.since), ([1.0, 0.0, 3.0], 4.0, 5.0));
        p.ideas.get_mut(Who::Player).unwrap().lost = true;
        learn_now(9.0, &mut p, Who::Player, [7.0, 2.0, 8.0], 8.0, IdeaHow::Told, None);
        let i = *p.ideas.get(Who::Player).unwrap();
        assert_eq!((i.at, i.t, i.since, i.how, i.lost), ([7.0, 0.0, 8.0], 8.0, 5.0, IdeaHow::Told, false));
    }

    /// learn's `p` names the row by its owner, None = Sam's; the idea lands in that row, stamped with g.time
    #[test]
    fn learn_writes_the_row_its_owner_names() {
        let mut g = Game::blank();
        g.time = 5.0;
        let gd = GuardId(2);
        pic_of(&mut g, gd, 5.0);
        learn(&mut g, Some(gd), Who::Player, [1.0, 2.0, 3.0], 4.0, IdeaHow::Seen, None);
        learn(&mut g, None, Who::Guard(gd), [7.0, 2.0, 8.0], 3.0, IdeaHow::Told, Some(gd));
        let man = *pic_of(&mut g, gd, 5.0).ideas.get(Who::Player).unwrap();
        assert_eq!((man.at, man.t, man.since), ([1.0, 0.0, 3.0], 4.0, 5.0));
        assert!(pic_of(&mut g, gd, 5.0).ideas.get(Who::Guard(gd)).is_none());
        let sam = *state_of(&mut g).sam.ideas.get(Who::Guard(gd)).unwrap();
        assert_eq!((sam.at, sam.how, sam.by), ([7.0, 0.0, 8.0], IdeaHow::Told, Some(gd)));
        assert!(state_of(&mut g).sam.ideas.get(Who::Player).is_none());
    }

    #[test]
    fn known_places_keeps_the_freshest_word_and_a_same_age_not_there() {
        let mut best: JsMap<Who, (V3, f64, bool)> = JsMap::new();
        known_put(&mut best, Who::Player, [1.0, 0.0, 0.0], 5.0, false);
        known_put(&mut best, Who::Player, [2.0, 0.0, 0.0], 4.0, true);
        assert_eq!(*best.get(Who::Player).unwrap(), ([1.0, 0.0, 0.0], 5.0, false));
        known_put(&mut best, Who::Player, [3.0, 0.0, 0.0], 5.0, true);
        assert_eq!(*best.get(Who::Player).unwrap(), ([1.0, 0.0, 0.0], 5.0, true));
        known_put(&mut best, Who::Player, [4.0, 0.0, 0.0], 6.0, false);
        assert_eq!(*best.get(Who::Player).unwrap(), ([4.0, 0.0, 0.0], 6.0, false));
    }

    /// a write through the row lands, and a row is made once: the lost write of skeleton 1
    #[test]
    fn state_of_and_pic_of_hand_back_the_row() {
        let mut g = Game::blank();
        assert_eq!(state_of(&mut g).tell_t, -1e9);
        assert_eq!(state_of(&mut g).sam.look_t, -1e9);
        assert_eq!(state_of(&mut g).sam.owner, None);
        state_of(&mut g).tell_t = 3.0;
        assert_eq!(state_of(&mut g).tell_t, 3.0);
        pic_of(&mut g, GuardId(4), 10.0).heard.set(Who::Player, 2.0);
        let p = pic_of(&mut g, GuardId(4), 99.0);
        assert_eq!(p.heard.get(Who::Player).copied(), Some(2.0));
        assert_eq!(p.owner, Some(GuardId(4)));
        assert_eq!(p.look_t, 10.0);
    }

    /// picture.ts:377-378,389,391: the asker's key by value; a kept step answers with no line asked while its next look is ahead
    #[test]
    fn a_step_is_kept_for_its_own_key_and_a_loaded_one_takes_the_first_asker() {
        let mut g = Game::blank();
        g.time = 10.0;
        let gd = GuardId(1);
        let step = |key| Step { key, mark: [1.0, 0.0, 1.0], at: [2.0, 0.0, 2.0], low: true, look_t: 50.0, t0: 9.0 };
        {
            let p = pic_of(&mut g, gd, 10.0);
            p.ideas.set(Who::Player, idea(9.0, IdeaHow::Seen, false));
            p.step = Some(step(Some(7)));
            p.step_ask = Some(10.0);
        }
        let got = picture_step(&mut g, gd, 7, [1.0, 0.0, 1.0], None, Some(true)).unwrap();
        assert_eq!((got.at, got.low), ([2.0, 0.0, 2.0], true));
        assert!(picture_step(&mut g, gd, 8, [1.0, 0.0, 1.0], None, Some(true)).is_none());
        assert!(pic_of(&mut g, gd, 10.0).step.is_none());
        {
            let p = pic_of(&mut g, gd, 10.0);
            p.step = Some(step(None));
        }
        assert!(picture_step(&mut g, gd, 8, [1.0, 0.0, 1.0], None, Some(true)).is_some());
        assert_eq!(pic_of(&mut g, gd, 10.0).step.unwrap().key, Some(8));
    }

    #[test]
    fn a_new_picture_takes_its_turn_by_the_table_size() {
        let mut s = State::zeroed();
        for n in 0..9 {
            ensure_pic(&mut s, GuardId(n), 10.0, 0.2);
        }
        assert_eq!(s.men.get(GuardId(3)).unwrap().look_t.to_bits(), (10.0f64 + 3.0 * 0.2 / 8.0).to_bits());
        assert_eq!(s.men.get(GuardId(8)).unwrap().look_t, 10.0);
        ensure_pic(&mut s, GuardId(3), 99.0, 0.2);
        assert_eq!(s.men.get(GuardId(3)).unwrap().owner, Some(GuardId(3)));
        assert!(s.men.get(GuardId(3)).unwrap().look_t < 11.0);
    }
}
