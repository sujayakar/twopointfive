//! The room worked from the threshold: both jambs' diagonals held, each man's cuts walked, and the hold's own clock.
use super::*;

pub(super) fn step_thr(g: &mut Game, op: DoorOpId, cx: &Cx) {
    keep_open(g, cx.d);
    door_tools_thr_men(g, op);
    door_tools_thr_watch(g, op);
    if thr!(g, op).step_owed.unwrap_or(false) && !door_tools_word_owed(g, op) { return; }
    let jamb = thr!(g, op).jamb.clone();
    jamb_lights(g, op, &jamb, cx.t);
    let sr = o!(g, op).search.flatten();
    if let Some(s) = sr { jamb_looks(g, op, s, &jamb, cx.t); }
    let pies = jamb.iter().any(|&m_| o!(g, op).slice.get(m_).is_some_and(|s| s.pie.is_some()));
    let walkers = if pies { jamb.clone() } else { vec![o!(g, op).lead] };
    let walk = Walk { op, t: cx.t, jamb, sr, going: format!("slicing · {}", cx.name), holding: format!("holding the threshold · {}", cx.name) };
    let mut done = true;
    for gd in walkers { if !walked(g, &walk, gd) { done = false; } }
    if done && pies && !thr!(g, op).sliced {
        thr!(g, op).sliced = true;
        let w = format!("sliced{}", unseen_of(g, sr));
        door_tools_beat(g, op, &w);
    }
    if !thr!(g, op).hold && door_tools_last_call(g, op) { last_warning_hold(g, op); return; }
    hold_over(g, op, cx, done);
}

/// What one tick of the cuts is: the order, its jamb men, its search, and what the board reads of them on the way and on the mark.
struct Walk { op: DoorOpId, t: f64, jamb: Vec<GuardId>, sr: Option<DoortoolsRoomSearchId>, going: String, holding: String }

/// One man on his cuts: the job he came into the tick with, where he is, whether he leans and kneels his cuts or only stands them, and which cut.
struct Man { gd: GuardId, a: OrdersAssignmentId, p: V3, search: Option<bool>, pie: bool, i: usize }

/// A man's next mark: where, what his muzzle is on, how he stands at it, and what the board reads.
struct Mark<'a> { post: V3, at: V3, search: Option<bool>, lean: (Option<f64>, Option<bool>), going: &'a str, there: &'a str }
fn to_mark(g: &mut Game, op: DoorOpId, gd: GuardId, x: Mark) {
    let k = &g.k.doortools;
    let a = DoorToolsAssignA {
        post: Some(x.post), face: Some(v3::yaw_to(x.post, x.at)), watch: Some(x.at), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
        light: Some(light_of(&crate::seats::here(g))), search: x.search, pace: if k.slice_pace > 0.0 { Some(k.slice_pace) } else { None },
        lean: x.lean.0, crouch: x.lean.1, going: x.going.into(), there: x.there.into(), ..Default::default()
    };
    door_tools_assign(g, op, gd, &a);
}
/// How he stands at the cut he is on: his lean and his knee, where he leans it at all.
fn lean_of(g: &mut Game, op: DoorOpId, gd: GuardId) -> (Option<f64>, Option<bool>) {
    let s = sl!(g, op, gd);
    match &s.pie { Some(p) if js::truthy(p.lean) => (Some(p.lean), Some(p.knee[s.i as usize])), _ => (None, None) }
}
/// The end of a beat: what of the floor nobody has had a muzzle across.
fn unseen_of(g: &Game, sr: Option<DoortoolsRoomSearchId>) -> String {
    sr.map_or(String::new(), |s| format!(" · {} of {} cells unseen", sr!(g, s).left, sr!(g, s).pts.len()))
}

/// A light at the jambs: one of them has his lit, and a man in the black with no goggles that show it lights his.
fn jamb_lights(g: &mut Game, op: DoorOpId, jamb: &[GuardId], t: f64) {
    let light = g.k.doortools.thr_light;
    let job = |g: &Game, m_: GuardId| o!(g, op).mine.get(m_).copied();
    if light == 1.0 && !jamb.is_empty() && !jamb.iter().any(|&m_| job(g, m_).is_some_and(|a| asg!(g, a).search.unwrap_or(false))) {
        if let Some(a) = job(g, jamb[0]) { if !crate::nvg::nvg_worn(g, jamb[0]) { asg!(g, a).search = Some(true); } }
    }
    // (the one of the two who walks no cuts beside one who does is left dark)
    let slice = &o!(g, op).slice;
    let lit = match jamb.get(1).copied() { Some(b) if !(slice.get(jamb[0]).is_some_and(|s| s.pie.is_some()) && slice.get(b).is_none()) => Some(b), _ => jamb.first().copied() };
    for &m_ in jamb {
        let Some(a) = job(g, m_) else { continue };
        if light > 0.0 && !asg!(g, a).search.unwrap_or(false) && Some(m_) == lit && crate::nvg::nvg_black(g, m_) { asg!(g, a).search = Some(true); }
        if asg!(g, a).search.unwrap_or(false) { asg!(g, a).search_t = Some(t); }
    }
}

/// The floor the jamb men's muzzles are across, where it is lit or their goggles show it; their lines are asked again twice a second.
fn jamb_looks(g: &mut Game, op: DoorOpId, s: DoortoolsRoomSearchId, jamb: &[GuardId], t: f64) {
    if t >= thr!(g, op).los_t && !thr!(g, op).told {
        thr!(g, op).los_t = t + 0.5;
        for &m_ in jamb { sr!(g, s).los.delete(m_); }
    }
    let mut shows = |g: &mut Game, m_: GuardId, i: f64| -> bool {
        g.tab.light[g.gd[m_].flashlight].enabled || door_tools_thr_lit_cell(g, op, s, i)
            || (crate::nvg::nvg_down(g, m_) && (door_tools_thr_tube_cell(g, op, s, i) || { let q = sr!(g, s).pts[i as usize]; crate::irlight::ir_shows_at(g, m_, q) }))
    };
    door_tools_look(g, op, s, Some(jamb), Some(&mut shows));
}

/// One man's cuts, a tick. True: he owes none.
fn walked(g: &mut Game, w: &Walk, gd: GuardId) -> bool {
    let (op, t) = (w.op, w.t);
    if o!(g, op).slice.get(gd).is_none() { return true; }
    let Some(a) = o!(g, op).mine.get(gd).copied() else { return true };
    let pie = sl!(g, op, gd).pie.is_some();
    if pie && pie!(g, op, gd).fin { return true; }
    let m_ = Man { gd, a, p: g.pos_of(gd), search: asg!(g, a).search, pie, i: sl!(g, op, gd).i as usize };
    if pie {
        pie!(g, op, gd).begun = true;
        if v3::dist_xz(m_.p, sl!(g, op, gd).pts[m_.i]) < 0.12 { pie!(g, op, gd).on = true; }
    }
    let fights = |g: &Game| crate::roomfight::room_fight_contact(g, gd) || crate::roomfight::room_fight_engaged(g, gd);
    if pie && m_.i > 0 && !pie!(g, op, gd).on && pie!(g, op, gd).to.is_none() && pie!(g, op, gd).gun == 0.0 && fights(g) { fight_short(g, w, &m_); }
    if !on_mark(g, w, &m_) { sl!(g, op, gd).t = f64::NAN; return false; }
    if sl!(g, op, gd).t.is_nan() || fights(g) { sl!(g, op, gd).t = t; }
    if no_shot(g, w, &m_) { return true; }
    let rested = t - sl!(g, op, gd).t;
    let look = g.k.doortools.slice_look;
    if pie && pie!(g, op, gd).to.is_some() && rested >= look { on_to_mark(g, w, &m_); return false; }
    let end_on = pie && pie!(g, op, gd).end_on;
    if rested < (if end_on { g.k.doortools.thr_lean_look } else { look }) { return false; }
    if next_look(g, w, &m_) { return false; }
    if let Some(next) = next_cut(g, w, &m_, end_on) { to_cut(g, w, &m_, next); return false; }
    if pie && js::truthy(pie!(g, op, gd).end) && !end_on && m_.i > 0 { end_lean(g, w, &m_); return false; }
    cuts_done(g, w, &m_);
    true
}

/// Met on the way to his cut: he fights from where he is, and walks on to it after.
fn fight_short(g: &mut Game, w: &Walk, m_: &Man) {
    let (op, gd, i) = (w.op, m_.gd, m_.i);
    let here: V3 = [m_.p[0], 0.0, m_.p[2]];
    let at = asg!(g, m_.a).watch.unwrap_or(sl!(g, op, gd).looks[i][0]);
    pie!(g, op, gd).to = Some(sl!(g, op, gd).pts[i]);
    sl!(g, op, gd).pts[i] = here;
    pie!(g, op, gd).on = true;
    let lean = lean_of(g, op, gd);
    to_mark(g, op, gd, Mark { post: here, at, search: m_.search, lean, going: &w.going, there: &w.going });
}
/// Done there: he goes on to the cut he was on the way to.
fn on_to_mark(g: &mut Game, w: &Walk, m_: &Man) {
    let (op, gd, i) = (w.op, m_.gd, m_.i);
    let look = sl!(g, op, gd).looks[i][0];
    let to = pie!(g, op, gd).to.unwrap();
    sl!(g, op, gd).pts[i] = to;
    { let r = pie!(g, op, gd); r.to = None; r.on = false; r.gun = 0.0; r.at = w.t; }
    { let s = sl!(g, op, gd); s.j = 0.0; s.t = f64::NAN; }
    let lean = lean_of(g, op, gd);
    to_mark(g, op, gd, Mark { post: to, at: look, search: m_.search, lean, going: &w.going, there: &w.going });
}

/// Is he on the mark of his cut, and down on the knee it is taken from (or has he had long enough to be).
fn on_mark(g: &mut Game, w: &Walk, m_: &Man) -> bool {
    let (op, gd) = (w.op, m_.gd);
    let (off, arrived) = (v3::dist_xz(m_.p, sl!(g, op, gd).pts[m_.i]), asg!(g, m_.a).arrived);
    if !m_.pie { return arrived || off <= 0.5; }
    let r = pie!(g, op, gd);
    let (knee, long) = (r.knee[m_.i], !(w.t - r.at <= PIE_WAIT));
    long || ((arrived || off < 0.12) && !(knee && g.ch[g.gd[gd].ch].anim.crouch < 0.8))
}

/// He has a man to shoot and no shot from the cut: a step in, as often as there is floor for one, then a cut back and done. True: he went back.
fn no_shot(g: &mut Game, w: &Walk, m_: &Man) -> bool {
    let (op, gd, i) = (w.op, m_.gd, m_.i);
    if !m_.pie || crate::roomfight::room_fight_no_shot(g, gd) <= pie_gun().after || w.t - pie!(g, op, gd).gun_at <= pie_gun().every { return false; }
    let step = step_in(g, w, m_);
    pie!(g, op, gd).gun_at = w.t;
    if let Some(q) = step {
        pie!(g, op, gd).gun += 1.0;
        sl!(g, op, gd).pts[i] = q;
        let at = asg!(g, m_.a).watch.unwrap_or(sl!(g, op, gd).looks[i][0]);
        let lean = if i > 0 { lean_of(g, op, gd) } else { (None, None) };
        to_mark(g, op, gd, Mark { post: q, at, search: m_.search, lean, going: &w.going, there: &w.going });
        return false;
    }
    if i == 0 { return false; }
    { let r = pie!(g, op, gd); r.gun = 0.0; r.fin = true; r.to = None; }
    let s = sl!(g, op, gd);
    s.i -= 1.0;
    let (post, at) = (s.pts[i - 1], middle(&s.looks[i - 1]));
    to_mark(g, op, gd, Mark { post, at, search: m_.search, lean: (None, None), going: &w.holding, there: &w.holding });
    let said = format!("no shot from the cut · {} back", g.gd[gd].callsign);
    door_tools_beat(g, op, &said);
    true
}
/// The floor a step further in from his cut, where he may have it: not into a wall, nor onto the man at the other jamb or his cut.
fn step_in(g: &mut Game, w: &Walk, m_: &Man) -> Option<V3> {
    let (op, gd) = (w.op, m_.gd);
    let at = sl!(g, op, gd).pts[m_.i];
    let (inw, steps) = { let r = pie!(g, op, gd); (r.inw, r.gun) };
    let q: V3 = [at[0] + inw[0] * pie_gun().step, 0.0, at[2] + inw[2] * pie_gun().step];
    if steps >= pie_gun().most || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [q[0], 0.45, q[2]], None) { return None; }
    let apart = g.k.doortools.thr_pie_apart;
    let Some(x) = w.jamb.iter().copied().find(|&x| x != gd && !crate::condition::down(g, g.gd[x].ch)) else { return Some(q) };
    let his = o!(g, op).slice.get(x).map(|s| s.pts[s.i as usize]);
    if v3::dist_xz(g.pos_of(x), q) < apart || his.is_some_and(|x| v3::dist_xz(x, q) < apart) { return None; }
    Some(q)
}
fn middle(looks: &[V3]) -> V3 { looks[looks.len() >> 1] }

/// The next angle of the cut he is on.
fn next_look(g: &mut Game, w: &Walk, m_: &Man) -> bool {
    let s = sl!(g, w.op, m_.gd);
    let j = s.j as usize + 1;
    if j >= s.looks[m_.i].len() { return false; }
    s.j += 1.0;
    s.t = w.t;
    let at = s.looks[m_.i][j];
    asg!(g, m_.a).watch = Some(at);
    asg!(g, m_.a).face = Some(v3::yaw_to(m_.p, at));
    true
}

/// The cut he owes after this one: the next, or of cuts he leans, the next that shows floor nobody has seen.
fn next_cut(g: &mut Game, w: &Walk, m_: &Man, end_on: bool) -> Option<usize> {
    if !m_.pie { return Some(m_.i + 1).filter(|&n| n < sl!(g, w.op, m_.gd).pts.len()); }
    if end_on { return None; }
    let s = { let s = sl!(g, w.op, m_.gd); DoorToolsPieNextS { pts: s.pts.clone(), i: s.i, pie: s.pie.as_ref().map(|r| DoorToolsPieNextSPie { eyes: r.eyes.clone() }) } };
    let next = door_tools_pie_next(g, w.op, m_.gd, &s);
    if next >= 0.0 { Some(next as usize) } else { None }
}
fn to_cut(g: &mut Game, w: &Walk, m_: &Man, next: usize) {
    let (op, gd) = (w.op, m_.gd);
    let s = sl!(g, op, gd);
    s.i = next as f64;
    s.j = 0.0;
    s.t = f64::NAN;
    if let Some(r) = s.pie.as_mut() { r.at = w.t; r.gun = 0.0; r.on = false; r.to = None; }
    let (post, at, last) = (s.pts[next], s.looks[next][0], next + 1 >= s.pts.len());
    let light = g.k.doortools.thr_light;
    let search = if m_.pie { m_.search } else { Some(light >= 2.0 || (light == 1.0 && w.jamb.len() < 2)) };
    let lean = lean_of(g, op, gd);
    to_mark(g, op, gd, Mark { post, at, search, lean, going: &w.going, there: if last { &w.holding } else { &w.going } });
}

/// The last of a leaned cut: a lean past the frame's end, for what the standing cuts cannot show.
fn end_lean(g: &mut Game, w: &Walk, m_: &Man) {
    let (op, gd) = (w.op, m_.gd);
    pie!(g, op, gd).end_on = true;
    let end = pie!(g, op, gd).end;
    let x = thr!(g, op);
    x.los_t = m::min(x.los_t, w.t + 0.3);
    let s = sl!(g, op, gd);
    s.t = w.t;
    let (post, at) = (s.pts[m_.i], s.looks[m_.i][0]);
    to_mark(g, op, gd, Mark { post, at, search: m_.search, lean: (Some(end), None), going: &w.going, there: &w.going });
}

/// No more cuts: he holds the middle of what the last one showed.
fn cuts_done(g: &mut Game, w: &Walk, m_: &Man) {
    let (op, gd) = (w.op, m_.gd);
    let (post, at) = { let s = sl!(g, op, gd); (s.pts[m_.i], middle(&s.looks[m_.i])) };
    if m_.pie {
        pie!(g, op, gd).fin = true;
        to_mark(g, op, gd, Mark { post, at, search: m_.search, lean: (None, None), going: &w.holding, there: &w.holding });
        return;
    }
    if thr!(g, op).sliced { return; }
    thr!(g, op).sliced = true;
    asg!(g, m_.a).watch = Some(at);
    asg!(g, m_.a).face = Some(v3::yaw_to(m_.p, at));
    let said = format!("set{}", unseen_of(g, w.sr));
    door_tools_beat(g, op, &said);
}

/// Somebody in there is about to be shot: no hold, they go.
fn last_warning_hold(g: &mut Game, op: DoorOpId) {
    if !o!(g, op).last_hold.unwrap_or(false) {
        o!(g, op).last_hold = Some(true);
        o!(g, op).last_tossed = Some(thr!(g, op).tossed);
        door_tools_beat(g, op, "last warning · no hold");
    }
    thr!(g, op).force = true;
    door_tools_to_pockets(g, op);
}

/// The hold is over once the cuts are walked and it has been quiet, or at its cap: on to the pockets, or told to wait for Sam's word.
fn hold_over(g: &mut Game, op: DoorOpId, cx: &Cx, done: bool) {
    let k = &g.k.doortools;
    let (first_min, min, quiet, cap) = (k.thr_first_min, k.thr_min, k.thr_quiet, k.thr_cap);
    let x = thr!(g, op);
    let min = if x.first == Some(ThresholdFirst::Done) { first_min } else { min };
    if !((done && cx.el >= min && cx.t - x.quiet_t >= quiet) || cx.el >= cap) { return; }
    if !x.hold {
        if !x.step_owed.unwrap_or(false) { door_tools_to_pockets(g, op); }
        return;
    }
    if x.told || x.cut_owed.unwrap_or(false) { return; }
    x.told = true;
    door_tools_beat(g, op, "holding · on your word");
    crate::game::game_say(g, cx.lead, "holding on the door.", None, None);
    for m_ in thr!(g, op).jamb.clone() {
        if let Some(j) = o!(g, op).mine.get(m_).copied() { asg!(g, j).there = format!("holding the threshold on your word · {}", cx.name).into(); }
    }
}
