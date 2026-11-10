pub use ::game_types::close::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// What a new State holds: the TypeScript's own literal.
fn fresh_state() -> crate::close::State {
    crate::close::State { t: -1.0, best: None, mode: None, rmb_t: -1.0 }
}

/// The men sat down, made if none yet: the table itself.
fn seated_of(g: &mut crate::game::Game) -> &mut JsMap<GuardId, crate::close::Seat> {
    g.m.close.seated.get_or_insert_with(JsMap::new)
}

/// This seat's close-in state, made if none yet: the row itself.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::close::State {
    let p = g.player;
    g.m.close.state.get_or_insert_with(p, fresh_state)
}

/// One window, with its arc's cosine.
fn arc(id_2: crate::close::CloseId, kind: crate::close::CloseKind, mid: f64, half: f64, reach: f64, grab: bool, strike: bool) -> crate::close::CloseWindow {
    crate::close::CloseWindow { id: id_2, kind, mid, half, cos_half: m::cos(half), reach, grab, strike }
}

/// Has he a seat at all, down in it or on his way.
pub fn close_has_seat(g: &crate::game::Game, gd: GuardId) -> bool {
    g.m.close.seated.as_ref().map_or(false, |m| m.has(gd))
}

/// Is this man sitting.
pub fn close_seated(g: &crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].civ.is_some() { return crate::civilians::civilian_seated(g, gd); }
    g.m.close.seated.as_ref().and_then(|s| s.get(gd)).map_or(false, |st| st.down)
}

/// The windows round one man as he is now.
fn close_windows(g: &crate::game::Game, gd: GuardId) -> Vec<crate::close::CloseWindow> {
    use crate::close::{CloseId as I, CloseKind as K};
    let r = g.k.close.reach;
    let mut out = Vec::new();
    let rear_half = m::acos(-BEHIND_COS);
    if let Some(cv) = g.gd[gd].civ {
        if g.cv[cv].state == crate::civilians::CivState::Cower { out.push(arc(I::Behind, K::Behind, m::PI, m::PI, r.behind, true, true)); return out; }
        if crate::civilians::civilian_seated(g, gd) { out.push(arc(I::Behind, K::Seat, m::PI, rear_half, r.seat, true, true)); return out; }
        out.push(arc(I::Behind, K::Behind, m::PI, rear_half, r.behind, true, true));
        return out;
    }
    let rear = m::PI - rear_half;
    if close_seated(g, gd) {
        out.push(arc(I::Behind, K::Seat, m::PI, rear_half, r.seat, true, true));
        out.push(arc(I::Front, K::Front, 0.0, rear, r.seat_front, false, true));
        return out;
    }
    out.push(arc(I::Behind, K::Behind, m::PI, rear_half, r.behind, true, true));
    let front = v3::clamp(g.k.close.side_from_deg, 10.0, 100.0) * DEGR;
    if rear > front + 1e-3 {
        let mid = (front + rear) / 2.0;
        let half = (rear - front) / 2.0;
        out.push(arc(I::Left, K::Side, mid, half, r.side, false, true));
        out.push(arc(I::Right, K::Side, -mid, half, r.side, false, true));
    }
    out.push(arc(I::Front, K::Front, 0.0, front, r.front, false, true));
    out
}

/// The window a bearing falls in; the rear arc goes by the build's own cosine.
fn window_at(wins: &[crate::close::CloseWindow], b: f64) -> Option<crate::close::CloseWindow> {
    let cb = m::cos(b);
    for w in wins {
        if w.id == crate::close::CloseId::Behind && w.half < m::PI - 1e-6 {
            if cb < BEHIND_COS { return Some(*w); }
            continue;
        }
        if m::cos(b - w.mid) > w.cos_half - 1e-9 { return Some(*w); }
    }
    None
}

/// Nothing solid and no door leaf between two bodies at chest height.
pub(super) fn clear_between(g: &crate::game::Game, p: &[f64], q: &[f64]) -> bool {
    let a: V3 = [p[0], 1.0, p[2]];
    let b: V3 = [q[0], 1.0, q[2]];
    !crate::scene_collision::solid_blocked(g, a, b, None) && !crate::scene_collision::segment_blocked_dynamic(g, a, b, None)
}

/// May anything be done to him at all by Sam's hands.
pub fn close_subject(g: &crate::game::Game, gd: GuardId) -> bool {
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() { return false; }
    if g.team(gd).ours() || crate::fogofwar::fog_of_war_hidden(g, gd) { return false; }
    if g.gd[gd].civ.is_some() && g.staged.captives.contains(&gd) { return false; }
    true
}

/// What Sam could do to this man from `p`: the one test the rows, the click and the ready pose share.
pub fn close_at(g: &crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseOffer {
    use crate::civilians::CivState;
    let pl = g.player;
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let dx = p[0] - gp[0];
    let dz = p[2] - gp[2];
    let dist = m::hyp2(dx, dz);
    let bearing = v3::wrap_angle(m::atan2(dx, dz) - g.ch[c].body_yaw);
    let wins = close_windows(g, gd);
    let win = window_at(&wins, bearing);
    let mut o = crate::close::CloseOffer { guard: gd, win, dist, in_reach: false, clear: false, grab: false, strike: false, can: false, why: String::new() };
    let cv = g.gd[gd].civ;
    let him: String = match cv { Some(cv) => g.cv[cv].her.clone(), None => String::from("him") };
    let win = match win {
        Some(w) => w,
        None => { o.why = if cv.is_some() { format!("get behind {}", him) } else { String::from("get behind him") }; return o; }
    };
    let alert = g.gd[gd].state == crate::game::GuardState::Alert;
    let catchable = match cv { None => true, Some(cv) => { let s = g.cv[cv].state; s != CivState::Flee && s != CivState::Evacuate && s != CivState::Report } };
    let grab = win.grab && !alert && catchable;
    let strike = win.strike && catchable;
    if !grab && !strike {
        o.why = match cv {
            Some(cv) => format!("not while {}'s {}", g.cv[cv].she, if g.cv[cv].state == CivState::Report { "with a guard" } else { "running" }),
            None => String::from("not while he's onto you"),
        };
        return o;
    }
    if g.pl[pl].sprinting { o.why = String::from("stop sprinting"); return o; }
    o.in_reach = dist < win.reach;
    if !o.in_reach { o.why = String::from("get closer"); return o; }
    o.clear = clear_between(g, &gp, p);
    if !o.clear { o.why = String::from("get closer"); return o; }
    if g.pl[pl].crouch && !crate::player::can_stand(g) {
        let side = match win.kind { crate::close::CloseKind::Front => "in front of", crate::close::CloseKind::Side => "beside", _ => "behind" };
        o.why = format!("no room to stand up {} {} here", side, him);
        return o;
    }
    o.grab = grab;
    o.strike = strike;
    o.can = true;
    o
}

/// The right button, tapped or held, once a frame.
pub fn close_rmb(g: &mut crate::game::Game, on: bool, down: bool, hit: bool, dt: f64, near: bool, choke: bool, dropped: bool) -> Option<crate::close::CloseRmb> {
    use crate::close::CloseRmb as R;
    let tap_secs = g.k.close.rmb_tap_secs;
    let wait_secs = g.k.close.rmb_wait_secs;
    let s = state_of(g);
    if !on { s.rmb_t = -1.0; return if hit { Some(R::Press) } else { None }; }
    if hit {
        if near && !choke { s.rmb_t = -1.0; return Some(R::Press); }
        s.rmb_t = 0.0;
    }
    if s.rmb_t < 0.0 { return None; }
    if dropped { s.rmb_t = -1.0; return None; }
    if !down {
        let tap = s.rmb_t < tap_secs;
        s.rmb_t = -1.0;
        return if tap { Some(R::Tap) } else { None };
    }
    s.rmb_t += dt;
    if s.rmb_t < tap_secs { return None; }
    if choke { s.rmb_t = -1.0; return Some(R::Hold); }
    if near || s.rmb_t > tap_secs + wait_secs {
        s.rmb_t = -1.0;
        return if near { Some(R::Press) } else { None };
    }
    None
}

/// A blow landed: the tally.
/// The way in an offer was made by; none is from behind.
pub fn close_kind_of(o: Option<&crate::close::CloseOffer>) -> crate::close::CloseKind {
    match o.and_then(|o| o.win) { Some(w) => w.kind, None => crate::close::CloseKind::Behind }
}

/// One staff man's marker and rows.
pub fn close_rows(g: &mut crate::game::Game, gd: GuardId, p: &[f64], busy: bool, o: &mut crate::player::Offers) {
    use crate::close::CloseKind as K;
    use crate::game::{Interactable, InteractableKind};
    let pl = g.player;
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let far = m::hyp2(p[0] - gp[0], p[2] - gp[2]);
    if far >= 2.4 {
        if !busy && far < crate::stalk::stalk_begin_m(g) && crate::stalk::stalk_could(g, gd) {
            let line2 = sneak_hint();
            let it = Interactable { kind: InteractableKind::Guard, pos: v3::add(gp, [0.0, 1.35, 0.0]), line1: String::from("guard"), line2, in_reach: false, off: false, guard: Some(gd), ..Interactable::default() };
            o.add(it, Vec::new());
        }
        return;
    }
    let at = close_at(g, gd, p);
    let kind = close_kind_of(Some(&at));
    let seated = close_seated(g, gd);
    let alert = g.gd[gd].state == crate::game::GuardState::Alert;
    let where_ = match at.win { None => "", Some(w) => match w.kind { K::Front => " · facing you", K::Side => " · side on", _ => "" } };
    let line1 = format!("guard{}{}", if alert { " · alert" } else if seated { " · seated" } else { "" }, if alert { "" } else { where_ });
    let mut it = Interactable { kind: InteractableKind::Guard, pos: v3::add(gp, [0.0, if seated { 1.05 } else { 1.35 }, 0.0]), line1, line2: at.why.clone(), in_reach: at.can, off: false, guard: Some(gd), close: Some(kind), ..Interactable::default() };
    let sneak = !busy && crate::stalk::stalk_could(g, gd);
    if sneak && !at.can { it.line2 = sneak_hint(); }
    let mut verbs: Vec<Interactable> = Vec::new();
    if at.can && !busy {
        let is_civ = g.gd[gd].civ.is_some();
        let empty = g.pl[pl].holstered;
        let gun = crate::player::grab_variant(g, &crate::seats::here(g)) == crate::character::HoldVariant::Gun;
        let rmb = !(crate::rails::rails_hushes(g, "alt") && crate::rails::rails_hushes(g, "menu"));
        let away = !crate::rails::rails_hushes(g, "hands");
        let choke = if rmb && at.grab && !is_civ && crate::rails::rails_offers(g, "grab") { "  ·  RMB  hold to choke him" } else { "" };
        let rifle_up = crate::game::player_rifle_up(g, pl);
        let sling = if rifle_up { "sling it" } else { "holster" };
        let click: String = if !at.strike {
            String::new()
        } else if !empty {
            if away && (rmb) {
                let what: String = String::from("knockout");
                format!("E  {} for the {}", sling, what)
            } else { String::new() }
        } else if rmb {
            format!("RMB  knock him out{}", choke)
        } else {
            String::new()
        };
        let only = crate::stalk::stalk_click_only(g) && !is_civ;
        if at.grab && only {
            let line2 = if empty {
                if at.strike && rmb { format!("LMB  hold to grab him  ·  RMB  knock him out{}", choke) } else { String::from("LMB  hold to grab him") }
            } else {
                format!("E  {}, then hold LMB to grab him", sling)
            };
            verbs.push(crate::player::said(&it, &line2));
        } else if at.grab {
            let text = if kind == K::Seat {
                if gun { "haul him out of the chair — pistol to his head" } else { "haul him out of the chair" }
            } else if gun { "grab him — pistol to his head" } else if rifle_up { "grab him — the rifle goes on your back" } else { "grab him" };
            verbs.push(crate::player::row(&it, crate::game::Verb::Grab, text, &click));
        } else if at.strike {
            let why = if at.win.map_or(false, |w| w.grab) { "you can't grab him while he's onto you" } else { "the grab is from behind" };
            let line2 = if !click.is_empty() { format!("{}  ·  {}", click, why) } else { String::from(why) };
            verbs.push(crate::player::said(&it, &line2));
        }
        note_best(g, &at);
    }
    o.add(it, verbs);
}

/// The line that says what the held click does.
fn sneak_hint() -> String {
    String::from("LMB  hold to sneak up on him")
}

/// This frame's best offer: the nearest man Sam could do something to.
fn note_best(g: &mut crate::game::Game, o: &crate::close::CloseOffer) {
    let now = g.time;
    let s = state_of(g);
    if s.t != now { s.t = now; s.best = None; }
    if s.best.as_ref().map_or(true, |b| o.dist < b.dist) { s.best = Some(o.clone()); }
}

/// A civilian's gate with the windows on: is Sam in her window, and near enough.
pub fn close_civ_gate(g: &crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseCivGate {
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let b = v3::wrap_angle(m::atan2(p[0] - gp[0], p[2] - gp[2]) - g.ch[c].body_yaw);
    let wins = close_windows(g, gd);
    let w = window_at(&wins, b);
    crate::close::CloseCivGate {
        behind: w.is_some(),
        near: match w { Some(w) => m::hyp2(p[0] - gp[0], p[2] - gp[2]) < w.reach, None => false },
        kind: match w { Some(w) => w.kind, None => crate::close::CloseKind::Behind },
    }
}

/// A civilian Sam could take right now, for the ready pose.
pub fn close_civ_offer(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) {
    let o = close_at(g, gd, p);
    if o.can { note_best(g, &o); }
}

/// Sam's body says he is in reach: which pose, and what the hand is going for.
fn close_ready(g: &mut crate::game::Game) -> Option<crate::close::CloseReady> {
    use crate::character::{Bone, ReadyKind};
    use crate::close::{CloseKind as K, CloseReady};
    let pl = g.player;
    let now = g.time;
    let o = { let s = state_of(g); if s.t == now { s.best.clone() } else { None } };
    let o = match o { Some(o) if o.can => o, _ => return None };
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].tow.is_some() || g.pl[pl].planted()
        || crate::game::player_rifle_up(g, pl) || crate::player::can_in_hand(g, pl).is_some() { return None; }
    if crate::handwork::sam_work(g).is_some() { return None; }
    if g.pl[pl].wall.is_some() { return None; }
    let c = g.gd[o.guard].ch;
    let neck = g.ch[c].bones.get(Bone::Neck).or_else(|| g.ch[c].bones.get(Bone::Head)).copied();
    let pos = g.ch[c].pos;
    let at: V3 = match neck {
        Some(n) => [n[0], n[1], n[2]],
        None => [pos[0], pos[1] + (if o.win.map_or(false, |w| w.kind == K::Seat) { 1.1 } else { 1.45 }), pos[2]],
    };
    let kind = close_kind_of(Some(&o));
    if crate::game::player_pistol_up(g, pl) { return if o.grab { Some(CloseReady { kind: ReadyKind::GrabGun, at }) } else { None }; }
    if !o.grab { return Some(CloseReady { kind: ReadyKind::Strike, at }); }
    if g.pl[pl].crouch && kind != K::Seat && at[1] - pos[1] > g.k.close.hands_low_neck_m { return Some(CloseReady { kind: ReadyKind::Strike, at }); }
    Some(CloseReady { kind: if kind == K::Seat { ReadyKind::Seat } else { ReadyKind::Grab }, at })
}

/// Who holds the mode now: the nearest living man in range with nothing solid between.
fn mode_now(g: &crate::game::Game, pl: PlayerId) -> Option<GuardId> {
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].tow.is_some() || (g.pl[pl].planted() && g.pl[pl].takedown().is_none())
        || crate::game::player_rifle_up(g, pl) || g.pl[pl].sprinting || g.pl[pl].wall.is_some() { return None; }
    if crate::handwork::sam_work(g).is_some() { return None; }
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    let mut best: Option<GuardId> = None;
    let mut bd = m::INF;
    let mut men: Vec<GuardId> = g.guards.clone();
    men.extend(crate::civilians::civilians_shown(g));
    for gd in men {
        if !close_subject(g, gd) { continue; }
        let gc = g.gd[gd].ch;
        let gp = g.ch[gc].pos;
        let d = v3::dist_xz(p, gp);
        let held = g.m.close.state.get(pl).and_then(|s| s.mode);
        if d >= (if Some(gd) == held { g.k.close.mode_out_m } else { g.k.close.mode_m }) || d >= bd { continue; }
        if !clear_between(g, &gp, &p) { continue; }
        best = Some(gd);
        bd = d;
    }
    best
}

/// Once a frame after the rows are built: the poses and the mode of Sam's body.
pub fn close_tick(g: &mut crate::game::Game) {
    use crate::character::{Bone, HoldPhase, ReadyKind};
    let pl = g.player;
    state_of(g);
    let c = g.pl[pl].ch;
    let r = close_ready(g);
    let tell = if r.map_or(true, |r| r.kind != ReadyKind::Seat) && crate::handwork::sam_work(g).is_none() && !g.pl[pl].coding().is_some() { crate::stalk::stalk_tell(g) } else { None };
    let best: Option<GuardId> = { let now = g.time; let s = state_of(g); if s.t == now { s.best.as_ref().map(|b| b.guard) } else { None } };
    let on_tell = tell.filter(|t| r.is_none() || best.is_none() || best == Some(t.gd));
    if let Some(t) = on_tell {
        let k = (g.k.stalk.low_drop, g.k.stalk.low_back, g.k.stalk.low_lean);
        let an = &mut g.ch[c].anim;
        an.ready = Some(ReadyKind::Grab);
        an.ready_at = Some(t.at);
        an.ready_up = 1.0;
        an.ready_amt = t.reach;
        an.rise = t.rise;
        an.low = t.low;
        if t.low > 0.0 { an.low_shape[0] = k.0; an.low_shape[1] = k.1; an.low_shape[2] = k.2; }
    } else {
        let an = &mut g.ch[c].anim;
        an.ready = r.map(|r| r.kind);
        an.ready_at = r.map(|r| r.at);
        an.ready_up = if r.is_some() { 1.0 } else { 0.0 };
        an.ready_amt = 1.0;
        an.rise = 0.0;
        an.low = 0.0;
    }
    let cw = if r.is_none() { crate::stalk::stalk_wait(g) } else { None };
    if let Some(cw) = cw {
        let an = &mut g.ch[c].anim;
        an.ready = Some(ReadyKind::Grab);
        an.ready_at = Some(cw.at);
        an.ready_up = 0.0;
        an.ready_amt = cw.reach;
        an.coil = cw.coil;
    } else {
        let coil = g.ch[c].anim.coil;
        let rise_k = g.k.stalk.corner_rise;
        let mut next = 0.0;
        if let Some(h) = g.pl[pl].holding_mut() {
            if h.from.is_some() && h.phase == HoldPhase::Grab && coil > 0.0 {
                let coil0 = *h.coil0.get_or_insert(coil);
                next = if rise_k > 0.0 { coil0 * (1.0 - v3::smoothstep(0.0, rise_k, h.t)) } else { 0.0 };
            }
        }
        g.ch[c].anim.coil = next;
    }
    {
        let has_h = g.pl[pl].holding().is_some();
        g.ch[c].anim.hands_down = g.k.stalk.low_hands;
        let wide = if !has_h {
            m::min(1.0, g.ch[c].anim.low / m::max(0.05, g.k.stalk.wide_by))
        } else { 0.0 };
        g.ch[c].anim.wide = wide;
        if js::truthy(wide) {
            let ks = &g.k.stalk;
            let (ws, pole, hs) = ([ks.heft_out, ks.heft_down, ks.heft_fwd, ks.heft_lead], ks.heft_pole, [ks.heft_twist, ks.heft_swing, ks.heft_side, ks.heft_lean, ks.heft_dip, ks.heft_surge]);
            let an = &mut g.ch[c].anim;
            an.wide_shape = ws;
            an.wide_pole = Some(pole);
            an.heft_shape = hs;
        }
        g.ch[c].anim.heft = 1.0;
        let spring = match g.pl[pl].holding() {
            Some(h) if h.from.is_some() && h.phase == HoldPhase::Grab && h.coil0.is_none() => {
                let wind = g.k.stalk.low_wind;
                m::max(1.0, g.k.stalk.low_spring * (if wind > 0.0 { v3::smoothstep(0.0, wind, h.t) } else { 1.0 }))
            }
            _ => 0.0,
        };
        g.ch[c].anim.spring = spring;
    }
    let m_ = if g.pl[pl].takedown().is_some() { state_of(g).mode } else { mode_now(g, pl) };
    state_of(g).mode = m_;
    g.ch[c].anim.creep = m_.is_some();
    if let Some(mm) = m_ {
        let mc = g.gd[mm].ch;
        let ch = g.ch[mc].bone(Bone::Chest);
        let pos = g.ch[mc].pos;
        g.ch[c].anim.creep_at = Some(match ch { Some(ch) => [ch[0], ch[1], ch[2]], None => [pos[0], pos[1] + 1.2, pos[2]] });
    } else {
        g.ch[c].anim.creep_at = None;
    }
}

/// The dark: how much of the build's near-sight floor this man has for Sam right now.
pub fn close_near_k(g: &crate::game::Game, gd: GuardId, dist: f64) -> f64 {
    let st = g.gd[gd].state;
    if st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search || dist < g.k.close.touch_m { return 1.0; }
    let k = v3::smoothstep(g.k.close.dark_at, g.k.close.lit_at, g.shown[g.pl[g.player].ch].visibility);
    k
}

/// Where Sam stands to strike, on his own bearing off the man, and the way he faces.
pub fn close_station(g: &crate::game::Game, gd: GuardId, kind: crate::close::CloseKind, bearing: f64) -> crate::close::CloseStation {
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let a = g.ch[c].body_yaw + bearing;
    let r = if kind == crate::close::CloseKind::Seat { g.k.close.seat_dist } else { g.k.close.strike_dist };
    let pos: V3 = [gp[0] + m::sin(a) * r, 0.0, gp[2] + m::cos(a) * r];
    crate::close::CloseStation { pos }
}

/// The fall's hearing level for a way in, and his shout if he was onto Sam.
pub fn close_noise(g: &crate::game::Game, kind: crate::close::CloseKind, was_alert: bool) -> f64 {
    let n = g.k.close.noise;
    let base = match kind {
        crate::close::CloseKind::Behind => n.behind,
        crate::close::CloseKind::Side => n.side,
        crate::close::CloseKind::Front => n.front,
        crate::close::CloseKind::Seat => n.seat,
    };
    base + (if was_alert { g.k.close.alert_noise } else { 0.0 })
}

/// The tallies of what a take or a grab made.
pub fn close_took(what: crate::close::CloseTookWhat) {
    // the tallies' rows are in CLOSE_STATS's written order, which is CloseKind's
    match what { crate::close::CloseTookWhat::Take => {}, crate::close::CloseTookWhat::Grab => {}}
}

/// A man calm enough to sit.
fn calm(g: &crate::game::Game, gd: GuardId) -> bool {
    let m = &g.gd[gd];
    m.state == crate::game::GuardState::Patrol && m.awareness < 0.3 && m.held.is_none() && m.script.is_none()
}

/// Sit a calm staff man down on a mark; false if he cannot be sat.
pub fn close_seat(g: &mut crate::game::Game, gd: GuardId, at: Option<&[f64]>, face: Option<f64>) -> bool {
    if g.gd[gd].civ.is_some() || g.gd[gd].unit.is_some() || crate::condition::down(g, g.gd[gd].ch) || !calm(g, gd) { return false; }
    let prev = seated_of(g).get(gd).copied();
    if prev.is_some() && at.is_none() { return true; }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let st = crate::close::Seat {
        at: match at { Some(a) => [a[0], 0.0, a[2]], None => [pos[0], 0.0, pos[2]] },
        face: face.unwrap_or(g.ch[c].body_yaw),
        hold: match prev { Some(prev) => prev.hold, None => g.gd[gd].hold },
        down: false,
    };
    seated_of(g).set(gd, st);
    crate::control::set_hold(g, gd, true, "close");
    true
}

/// Up out of the chair, his hold flag as it was.
pub fn close_unseat(g: &mut crate::game::Game, gd: GuardId, back: Option<bool>) {
    let back = back.unwrap_or(false);
    let st = match seated_of(g).get(gd).copied() { Some(st) => st, None => return };
    seated_of(g).delete(gd);
    crate::control::set_hold(g, gd, st.hold, "close");
    let c = g.gd[gd].ch;
    if g.ch[c].anim.act.as_ref().map_or(false, |a| a.clip == sit_clip().clip) { g.ch[c].anim.act = None; }
    if back && st.down && !crate::condition::down(g, g.gd[gd].ch) {
        g.ch[c].pos[0] = st.at[0];
        g.ch[c].pos[2] = st.at[2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::close::{CloseId as I, CloseKind as K};
    use alloc::boxed::Box;

    fn a_man() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let c = g.ch.push(crate::character::Character::blank());
        let mut row = crate::game::Guard::blank();
        row.ch = c;
        let gd = g.gd.push(row);
        (g, gd)
    }
    fn id_at(w: &[crate::close::CloseWindow], b: f64) -> I { window_at(w, b).unwrap().id }

    /// Whichever way Sam comes at a standing man, he is in one of the four windows, the sides mirror each other, and the windows meet where the knobs say.
    #[test]
    fn a_standing_man_has_a_window_on_every_side() {
        let (g, gd) = a_man();
        let w = close_windows(&g, gd);
        let mut b = -3.2;
        while b < 3.2 {
            let at = window_at(&w, b).unwrap();
            let other = window_at(&w, -b).unwrap();
            assert_eq!(at.kind, other.kind, "bearing {} is not the mirror of {}", b, -b);
            b += 0.05;
        }
        let front = g.k.close.side_from_deg * DEGR;
        assert_eq!(id_at(&w, 0.0), I::Front);
        assert_eq!(id_at(&w, front - 0.01), I::Front);
        assert_eq!(id_at(&w, -(front - 0.01)), I::Front);
        assert_eq!(id_at(&w, front + 0.01), I::Left);
        assert_eq!(id_at(&w, -(front + 0.01)), I::Right);
        let rear = m::acos(BEHIND_COS);
        assert_eq!(id_at(&w, rear - 0.01), I::Left);
        assert_eq!(id_at(&w, -(rear - 0.01)), I::Right);
        assert_eq!(id_at(&w, rear + 0.01), I::Behind);
        assert_eq!(id_at(&w, -(rear + 0.01)), I::Behind);
        assert_eq!(id_at(&w, m::PI), I::Behind);
    }

    /// He is grabbed only from behind, and struck from anywhere, at each side's own reach.
    #[test]
    fn only_from_behind_is_he_grabbed() {
        let (g, gd) = a_man();
        let w = close_windows(&g, gd);
        let r = g.k.close.reach;
        for x in &w {
            assert!(x.strike);
            assert_eq!(x.grab, x.id == I::Behind, "{:?}", x.id);
            assert_eq!(x.reach, match x.kind { K::Behind => r.behind, K::Side => r.side, _ => r.front });
        }
    }

    /// A man sitting down has no sides: the rear is his seat's, the rest is his front.
    #[test]
    fn a_seated_man_has_a_seat_behind_and_a_front_all_the_way_round_to_it() {
        let (mut g, gd) = a_man();
        let mut seated = JsMap::new();
        seated.set(gd, crate::close::Seat { at: [0.0; 3], face: 0.0, hold: false, down: true });
        g.m.close.seated = Some(seated);
        let w = close_windows(&g, gd);
        assert_eq!(w.len(), 2);
        let rear = m::acos(BEHIND_COS);
        assert_eq!(window_at(&w, m::PI).unwrap().kind, K::Seat);
        assert_eq!(window_at(&w, rear + 0.01).unwrap().kind, K::Seat);
        assert_eq!(window_at(&w, rear - 0.01).unwrap().kind, K::Front);
        assert_eq!(window_at(&w, -(rear - 0.01)).unwrap().kind, K::Front);
    }

    #[test]
    fn no_offer_is_from_behind() {
        assert_eq!(close_kind_of(None), K::Behind);
        let o = crate::close::CloseOffer { win: Some(arc(I::Left, K::Side, 1.0, 0.5, 1.0, false, true)), ..crate::close::CloseOffer::default() };
        assert_eq!(close_kind_of(Some(&o)), K::Side);
        assert_eq!(close_kind_of(Some(&crate::close::CloseOffer::default())), K::Behind);
    }
}

/// Once a frame before the guards: the seated men kept on their marks while calm, stood up when not.
pub fn close_seat_tick(g: &mut crate::game::Game, dt: f64) {
    match &g.m.close.seated {
        Some(seated) if !seated.is_empty() => {}
        _ => return,
    }
    let mut at = 0;
    // A live walk: closeUnseat deletes the row the walk stands on, as under the TypeScript's `for … of` over a Map.
    while let Some(gd) = g.m.close.seated.as_ref().and_then(|seated| seated.next_key(&mut at)) {
        let Some(st) = g.m.close.seated.as_ref().and_then(|seated| seated.get(gd).copied()) else { continue };
        if !g.guards.contains(&gd) || !calm(g, gd) {
            close_unseat(g, gd, None);
            continue;
        }
        let c = g.gd[gd].ch;
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, st.face, 3.0 * dt);
        let settled = m::abs(v3::wrap_angle(g.ch[c].body_yaw - st.face)) < 0.35;
        let tx = st.at[0] + m::sin(st.face) * crate::civilians::SIT_ADVANCE;
        let tz = st.at[2] + m::cos(st.face) * crate::civilians::SIT_ADVANCE;
        let k = 1.0 - m::exp(-5.0 * dt);
        if settled {
            g.ch[c].pos[0] += (tx - g.ch[c].pos[0]) * k;
            g.ch[c].pos[2] += (tz - g.ch[c].pos[2]) * k;
            let clip = sit_clip();
            g.ch[c].anim.act = Some(crate::character::AnimAct { clip: clip.clip, loop_: Some(clip.loop_), rate: Some(clip.rate), in_: Some(clip.in_), out: Some(clip.out) });
        }
        g.gd[gd].speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
        g.gd[gd].path_goal = None;
        g.gd[gd].path_goal_orbit = false;
        let down = settled && crate::character::anim_act_shown(g, c) > 0.5;
        if let Some(row) = g.m.close.seated.as_mut().and_then(|seated| seated.get_mut(gd)) {
            row.down = down;
        }
    }
}

