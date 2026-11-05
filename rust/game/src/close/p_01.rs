//! src/game/close.ts lines 1 to 463
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A seated man's row. (close.ts:148 SEATED holds close::Seat; the skeleton's table still holds SeatId, cmd.ts's Seat)
fn seam_seat_get(g: &mut Game, gd: GuardId) -> Option<crate::close::Seat> { if !close_has_seat(g, gd) { return None; } todo!("needs: g.m.close.seated as JsMap<GuardId, close::Seat>, not SeatId (close.ts:148)") }
/// Sets a seated man's row. (close.ts SEATED, as `seam_seat_get`)
fn seam_seat_set(g: &mut Game, gd: GuardId, st: crate::close::Seat) { todo!("needs: g.m.close.seated as JsMap<GuardId, close::Seat>, not SeatId (close.ts:148)") }
/// Deletes a seated man's row. (close.ts SEATED, as `seam_seat_get`)
fn seam_seat_delete(g: &mut Game, gd: GuardId) { todo!("needs: g.m.close.seated as JsMap<GuardId, close::Seat>, not SeatId (close.ts:148)") }

/// What a new State holds: the TypeScript's own literal. (close.ts stateOf)
fn fresh_state() -> crate::close::State {
    crate::close::State { t: -1.0, best: None, mode: None, mode_d: m::INF, rmb_t: -1.0 }
}

/// The log of what was taken, the debugger's. (close.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: crate::close::NoteWhat, kind: Option<crate::close::CloseKind>, bearing: Option<f64>, dist: Option<f64>) {
    let what = match what {
        crate::close::NoteWhat::Take => crate::close::CloseModCLOSELOGItemWhat::Take,
        crate::close::NoteWhat::Grab => crate::close::CloseModCLOSELOGItemWhat::Grab,
        crate::close::NoteWhat::Seat => crate::close::CloseModCLOSELOGItemWhat::Seat,
        crate::close::NoteWhat::Unseat => crate::close::CloseModCLOSELOGItemWhat::Unseat,
    };
    let item = crate::close::CloseModCLOSELOGItem { t: g.time, who: g.gd[gd].callsign.clone(), what, kind, bearing: bearing.unwrap_or(0.0), dist: dist.unwrap_or(0.0) };
    g.m.close.close_log.push(item);
    if g.m.close.close_log.len() > 200 { g.m.close.close_log.remove(0); }
}

/// The men sat down, made if none yet; a copy: writes go through `g.m.close.seated`. (close.ts seatedOf)
pub fn seated_of(g: &mut crate::game::Game) -> JsMap<GuardId, SeatId> {
    g.m.close.seated.get_or_insert_with(JsMap::new).clone()
}

/// This seat's close-in state, made if none yet: the row itself. (close.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::close::State {
    let p = g.player;
    g.m.close.state.get_or_insert_with(p, fresh_state)
}

/// One window, with its arc's cosine. (close.ts arc)
pub fn arc(id_2: crate::close::CloseId, kind: crate::close::CloseKind, mid: f64, half: f64, reach: f64, grab: bool, strike: bool) -> crate::close::CloseWindow {
    crate::close::CloseWindow { id: id_2, kind, mid, half, cos_half: m::cos(half), reach, grab, strike }
}

/// Has he a seat at all, down in it or on his way. (close.ts closeHasSeat)
pub fn close_has_seat(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.close.seated.as_ref().map_or(false, |m| m.has(gd))
}

/// Is this man sitting. (close.ts closeSeated)
pub fn close_seated(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].civ.is_some() { return crate::civilians::civilian_seated(g, gd); }
    seam_seat_get(g, gd).map_or(false, |st| st.down)
}

/// The windows round one man as he is now. (close.ts closeWindows)
pub fn close_windows(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::close::CloseWindow> {
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

/// The window a bearing falls in; the rear arc goes by the build's own cosine. (close.ts windowAt)
pub fn window_at(wins: &[crate::close::CloseWindow], b: f64) -> Option<crate::close::CloseWindow> {
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

/// Nothing solid and no door leaf between two bodies at chest height. (close.ts clearBetween)
pub fn clear_between(g: &mut crate::game::Game, p: &[f64], q: &[f64]) -> bool {
    let a: V3 = [p[0], 1.0, p[2]];
    let b: V3 = [q[0], 1.0, q[2]];
    !crate::scene_collision::solid_blocked(g, a, b, None) && !crate::scene_collision::segment_blocked_dynamic(g, a, b, None)
}

/// May anything be done to him at all by Sam's hands. (close.ts closeSubject)
pub fn close_subject(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() { return false; }
    if g.gd[gd].team == crate::teams::Team::Operative || crate::fogofwar::fog_of_war_hidden(g, gd) { return false; }
    if g.gd[gd].civ.is_some() && crate::rescue::rescue_is_hostage(g, gd) { return false; }
    true
}

/// What Sam could do to this man from `p`: the one test the rows, the click and the ready pose share. (close.ts closeAt)
pub fn close_at(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseOffer {
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
    let mut o = crate::close::CloseOffer { guard: gd, win, dist, bearing, in_reach: false, clear: false, grab: false, strike: false, can: false, why: String::new() };
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

/// The two-button blow is live: no show running. (close.ts closeLethal)
pub fn close_lethal(g: &mut crate::game::Game) -> bool { !g.touring }

/// The right button, tapped or held, once a frame. (close.ts closeRmb)
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

/// There is no knife: no show running. (close.ts closeBare)
pub fn close_bare(g: &mut crate::game::Game) -> bool { !g.touring }

/// The windows are live: no show running. (close.ts closeLive)
pub fn close_live(g: &mut crate::game::Game) -> bool { !g.touring }

/// A blow landed: the tally. (close.ts closeBlow)
pub fn close_blow(g: &mut crate::game::Game, lethal: bool) { g.stats.close.blows[if lethal { 0 } else { 1 }] += 1; }

/// The way in an offer was made by; none is from behind. (close.ts closeKindOf)
pub fn close_kind_of(o: Option<&crate::close::CloseOffer>) -> crate::close::CloseKind {
    match o.and_then(|o| o.win) { Some(w) => w.kind, None => crate::close::CloseKind::Behind }
}

/// One staff man's marker and rows. (close.ts closeRows)
pub fn close_rows(g: &mut crate::game::Game, gd: GuardId, p: &[f64], busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::close::CloseRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    use crate::close::CloseKind as K;
    use crate::game::{Interactable, InteractableKind};
    let pl = g.player;
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let far = m::hyp2(p[0] - gp[0], p[2] - gp[2]);
    if far >= 2.4 {
        if !busy && far < crate::stalk::stalk_begin_m(g) && crate::stalk::stalk_could(g, gd) {
            let line2 = sneak_hint(g);
            let it = Interactable { kind: InteractableKind::Guard, pos: v3::add(gp, [0.0, 1.35, 0.0]), line1: String::from("guard"), line2, in_reach: false, off: false, guard: Some(gd), ..Interactable::zeroed() };
            offer(g, it, Vec::new());
        }
        return;
    }
    let o = close_at(g, gd, p);
    let kind = close_kind_of(Some(&o));
    let seated = close_seated(g, gd);
    let alert = g.gd[gd].state == crate::game::GuardState::Alert;
    let where_ = match o.win { None => "", Some(w) => match w.kind { K::Front => " · facing you", K::Side => " · side on", _ => "" } };
    let line1 = format!("guard{}{}", if alert { " · alert" } else if seated { " · seated" } else { "" }, if alert { "" } else { where_ });
    let mut it = Interactable { kind: InteractableKind::Guard, pos: v3::add(gp, [0.0, if seated { 1.05 } else { 1.35 }, 0.0]), line1, line2: o.why.clone(), in_reach: o.can, off: false, guard: Some(gd), close: Some(kind), ..Interactable::zeroed() };
    let sneak = !busy && crate::stalk::stalk_could(g, gd);
    if sneak && !o.can { it.line2 = sneak_hint(g); }
    let mut verbs: Vec<Interactable> = Vec::new();
    if o.can && !busy {
        let is_civ = g.gd[gd].civ.is_some();
        let empty = g.pl[pl].holstered;
        let gun = crate::player::grab_variant(g) == crate::character::HoldVariant::Gun;
        let bare = close_bare(g);
        let two = close_lethal(g) || bare;
        let rmb = !(crate::rails::rails_hushes(g, "alt") && crate::rails::rails_hushes(g, "menu"));
        let away = !crate::rails::rails_hushes(g, "hands");
        let choke = if rmb && o.grab && !is_civ && (bare || close_lethal(g)) && crate::rails::rails_offers(g, "grab") { "  ·  RMB  hold to choke him" } else { "" };
        let rifle_up = crate::game::player_rifle_up(g, pl);
        let sling = if rifle_up { "sling it" } else { "holster" };
        let click: String = if !o.strike {
            String::new()
        } else if !empty {
            if away && (rmb || !bare) {
                let what: String = if bare { String::from("knockout") } else if two { String::from("knife / the knockout") } else { format!("LMB {}", if kind == K::Behind || kind == K::Seat { "takedown" } else { "strike" }) };
                format!("E  {} for the {}", sling, what)
            } else { String::new() }
        } else if (two && sneak) || bare {
            if rmb { format!("RMB  knock him out{}", choke) } else { String::new() }
        } else if two {
            if rmb { format!("LMB  knife — lethal  ·  RMB  knock him out{}", choke) } else { String::from("LMB  knife — lethal") }
        } else if kind == K::Behind {
            String::from("LMB  takedown — silent")
        } else if kind == K::Seat {
            String::from("LMB  put him out where he sits — silent")
        } else if kind == K::Side {
            String::from("LMB  strike — quiet")
        } else {
            String::from("LMB  strike — he gets a sound out")
        };
        let only = crate::stalk::stalk_click_only(g) && !is_civ;
        if o.grab && only {
            let line2 = if empty {
                if two && o.strike && rmb { format!("LMB  hold to grab him  ·  RMB  knock him out{}", choke) } else { String::from("LMB  hold to grab him") }
            } else {
                format!("E  {}, then hold LMB to grab him", sling)
            };
            verbs.push(Interactable { verb: None, running: Some(false), progress: None, in_reach: true, line2, ..it.clone() });
        } else if o.grab {
            let text = if kind == K::Seat {
                if gun { "haul him out of the chair — pistol to his head" } else { "haul him out of the chair" }
            } else if gun { "grab him — pistol to his head" } else if rifle_up { "grab him — the rifle goes on your back" } else { "grab him" };
            let v = row(g, it.clone(), crate::game::Verb::Grab, String::from(text), Some(click), None);
            verbs.push(v);
        } else if o.strike {
            let why = if o.win.map_or(false, |w| w.grab) { "you can't grab him while he's onto you" } else { "the grab is from behind" };
            let line2 = if !click.is_empty() { format!("{}  ·  {}", click, why) } else { String::from(why) };
            verbs.push(Interactable { verb: None, running: Some(false), progress: None, line2, in_reach: true, ..it.clone() });
        }
        note_best(g, &o);
    }
    offer(g, it, verbs);
}

/// The line that says what the held click does. (close.ts sneakHint)
pub fn sneak_hint(g: &mut crate::game::Game) -> String {
    String::from(if g.k.stalk.drag { "LMB  press and drag to sneak up on him" } else { "LMB  hold to sneak up on him" })
}

/// This frame's best offer: the nearest man Sam could do something to. (close.ts noteBest)
pub fn note_best(g: &mut crate::game::Game, o: &crate::close::CloseOffer) {
    let now = g.time;
    let s = state_of(g);
    if s.t != now { s.t = now; s.best = None; }
    if s.best.as_ref().map_or(true, |b| o.dist < b.dist) { s.best = Some(o.clone()); }
}

/// A civilian's gate with the windows on: is Sam in her window, and near enough. (close.ts closeCivGate)
pub fn close_civ_gate(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseCivGate {
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

/// A civilian Sam could take right now, for the ready pose. (close.ts closeCivOffer)
pub fn close_civ_offer(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) {
    let o = close_at(g, gd, p);
    if o.can { note_best(g, &o); }
}

/// Sam's body says he is in reach: which pose, and what the hand is going for. (close.ts closeReady)
pub fn close_ready(g: &mut crate::game::Game) -> Option<crate::close::CloseReady> {
    use crate::character::{Bone, ReadyKind};
    use crate::close::{CloseKind as K, CloseReady};
    let pl = g.player;
    if g.touring { return None; }
    let now = g.time;
    let o = { let s = state_of(g); if s.t == now { s.best.clone() } else { None } };
    let o = match o { Some(o) if o.can => o, _ => return None };
    if g.pl[pl].down || g.pl[pl].holding.is_some() || g.pl[pl].takedown.is_some() || g.pl[pl].dragging.is_some() || g.pl[pl].kick.is_some() || g.pl[pl].picking.is_some() || g.pl[pl].wanding.is_some()
        || crate::game::player_rifle_up(g, pl) || (crate::game::player_canister_sel(g, pl).is_some() && !g.pl[pl].holstered) { return None; }
    if crate::handwork::sam_work(g).is_some() || (g.pl[pl].coding.is_some() && crate::handwork::live(g, crate::handwork::LiveHalf::Work)) { return None; }
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

/// The man Sam is close to, as `close_tick` left it this frame. (close.ts closeMode)
pub fn close_mode(g: &mut crate::game::Game) -> Option<GuardId> {
    g.m.close.state.get(g.player).and_then(|s| s.mode)
}

/// Who holds the mode now: the nearest living man in range with nothing solid between. (close.ts modeNow)
pub fn mode_now(g: &mut crate::game::Game, pl: PlayerId) -> Option<GuardId> {
    g.m.close.state.get_or_insert_with(pl, fresh_state).mode_d = m::INF;
    if g.touring || g.pl[pl].down || g.pl[pl].holding.is_some() || g.pl[pl].dragging.is_some() || g.pl[pl].kick.is_some() || g.pl[pl].picking.is_some() || g.pl[pl].wanding.is_some()
        || crate::game::player_rifle_up(g, pl) || g.pl[pl].sprinting || g.pl[pl].wall.is_some() { return None; }
    if crate::handwork::sam_work(g).is_some() || (g.pl[pl].coding.is_some() && crate::handwork::live(g, crate::handwork::LiveHalf::Work)) { return None; }
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
        let d = m::hyp2(p[0] - gp[0], p[2] - gp[2]);
        let held = g.m.close.state.get(pl).and_then(|s| s.mode);
        if d >= (if Some(gd) == held { g.k.close.mode_out_m } else { g.k.close.mode_m }) || d >= bd { continue; }
        if !clear_between(g, &gp, &p) { continue; }
        best = Some(gd);
        bd = d;
    }
    g.m.close.state.get_or_insert_with(pl, fresh_state).mode_d = bd;
    best
}

/// Once a frame after the rows are built: the poses and the mode of Sam's body. (close.ts closeTick)
pub fn close_tick(g: &mut crate::game::Game) {
    use crate::character::{Bone, HoldPhase, ReadyKind};
    let pl = g.player;
    state_of(g);
    let c = g.pl[pl].ch;
    let r = close_ready(g);
    let tell = if r.map_or(true, |r| r.kind != ReadyKind::Seat) && crate::handwork::sam_work(g).is_none() && !(g.pl[pl].coding.is_some() && crate::handwork::live(g, crate::handwork::LiveHalf::Work)) { crate::stalk::stalk_tell(g) } else { None };
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
    let cw = if r.is_none() && close_bare(g) { crate::stalk::stalk_wait(g) } else { None };
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
        if let Some(h) = g.pl[pl].holding.as_mut() {
            if h.from.is_some() && h.phase == HoldPhase::Grab && coil > 0.0 {
                let coil0 = *h.coil0.get_or_insert(coil);
                next = if rise_k > 0.0 { coil0 * (1.0 - v3::smoothstep(0.0, rise_k, h.t)) } else { 0.0 };
            }
        }
        g.ch[c].anim.coil = next;
    }
    {
        let has_h = g.pl[pl].holding.is_some();
        let bare = close_bare(g);
        g.ch[c].anim.hands_down = if bare { g.k.stalk.low_hands } else { 0.0 };
        let bare = close_bare(g);
        let wide = if bare && !has_h {
            if g.k.tapstep.on && g.pl[pl].crouch { 1.0 } else { m::min(1.0, g.ch[c].anim.low / m::max(0.05, g.k.stalk.wide_by)) }
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
        let spring = match g.pl[pl].holding.as_ref() {
            Some(h) if h.from.is_some() && h.phase == HoldPhase::Grab && h.coil0.is_none() => {
                let wind = g.k.stalk.low_wind;
                m::max(1.0, g.k.stalk.low_spring * (if wind > 0.0 { v3::smoothstep(0.0, wind, h.t) } else { 1.0 }))
            }
            _ => 0.0,
        };
        g.ch[c].anim.spring = spring;
    }
    if r.is_some() { g.stats.close.ready_frames += 1; }
    let m_ = if g.pl[pl].takedown.is_some() { state_of(g).mode } else { mode_now(g, pl) };
    state_of(g).mode = m_;
    g.ch[c].anim.creep = m_.is_some() || crate::tapstep::tap_held(g);
    if !crate::tapstep::tap_held(g) { g.ch[c].anim.tap_u = -1.0; }
    if let Some(mm) = m_ {
        let mc = g.gd[mm].ch;
        let ch = g.ch[mc].bones.get(Bone::Chest).copied();
        let pos = g.ch[mc].pos;
        g.ch[c].anim.creep_at = Some(match ch { Some(ch) => [ch[0], ch[1], ch[2]], None => [pos[0], pos[1] + 1.2, pos[2]] });
        g.stats.close.mode_frames += 1;
    } else {
        g.ch[c].anim.creep_at = None;
    }
}

/// The dark: how much of the build's near-sight floor this man has for Sam right now. (close.ts closeNearK)
pub fn close_near_k(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 {
    if g.touring { return 1.0; }
    let st = g.gd[gd].state;
    if st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search || dist < g.k.close.touch_m { return 1.0; }
    let k = v3::smoothstep(g.k.close.dark_at, g.k.close.lit_at, g.pl[g.player].visibility);
    if k < 1.0 { g.stats.close.dark_passes += 1; }
    k
}

/// Where Sam stands to strike, on his own bearing off the man, and the way he faces. (close.ts closeStation)
pub fn close_station(g: &mut crate::game::Game, gd: GuardId, kind: crate::close::CloseKind, bearing: f64) -> crate::close::CloseStation {
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let a = g.ch[c].body_yaw + bearing;
    let r = if kind == crate::close::CloseKind::Seat { g.k.close.seat_dist } else { g.k.close.strike_dist };
    let pos: V3 = [gp[0] + m::sin(a) * r, 0.0, gp[2] + m::cos(a) * r];
    crate::close::CloseStation { pos, yaw: m::atan2(gp[0] - pos[0], gp[2] - pos[2]) }
}

/// The fall's hearing level for a way in, and his shout if he was onto Sam. (close.ts closeNoise)
pub fn close_noise(g: &mut crate::game::Game, kind: crate::close::CloseKind, was_alert: bool) -> f64 {
    let n = g.k.close.noise;
    let base = match kind {
        crate::close::CloseKind::Behind => n.behind,
        crate::close::CloseKind::Side => n.side,
        crate::close::CloseKind::Front => n.front,
        crate::close::CloseKind::Seat => n.seat,
    };
    base + (if was_alert { g.k.close.alert_noise } else { 0.0 })
}

/// The tallies of what a take or a grab made. (close.ts closeTook)
pub fn close_took(g: &mut crate::game::Game, gd: GuardId, what: crate::close::CloseTookWhat, kind: crate::close::CloseKind, bearing: f64, dist: f64) {
    // the tallies' rows are in CLOSE_STATS's written order, which is CloseKind's
    match what { crate::close::CloseTookWhat::Take => g.stats.close.takes[kind as usize] += 1, crate::close::CloseTookWhat::Grab => g.stats.close.grabs[kind as usize] += 1 }
    let what = match what { crate::close::CloseTookWhat::Take => crate::close::NoteWhat::Take, crate::close::CloseTookWhat::Grab => crate::close::NoteWhat::Grab };
    note(g, gd, what, Some(kind), Some(bearing), Some(dist));
}

/// How far out along a bearing the window runs before a wall or a leaf ends it, bisected. (close.ts clipReach)
pub fn clip_reach(g: &mut crate::game::Game, gd: GuardId, a: f64, r0: f64, r1: f64) -> f64 {
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let (s, co) = (m::sin(a), m::cos(a));
    let at = |r: f64| -> V3 { [gp[0] + s * r, 0.0, gp[2] + co * r] };
    if clear_between(g, &gp, &at(r1)) { return r1; }
    if !clear_between(g, &gp, &at(r0)) { return r0; }
    let (mut lo, mut hi) = (r0, r1);
    for _ in 0..5 {
        let mid = (lo + hi) / 2.0;
        if clear_between(g, &gp, &at(mid)) { lo = mid; } else { hi = mid; }
    }
    lo
}

/// What a window says on the floor, by what Sam's hands hold. (close.ts labelOf)
pub fn label_of(g: &mut crate::game::Game, w: crate::close::CloseWindow) -> String {
    use crate::close::CloseKind as K;
    let empty = g.pl[g.player].holstered;
    String::from(match w.kind {
        K::Front | K::Side => if empty { "STRIKE" } else { "STRIKE · HOLSTER FIRST" },
        K::Seat => if empty { "HAUL OUT · TAKEDOWN" } else { "HAUL OUT" },
        K::Behind => if empty { "GRAB · TAKEDOWN" } else { "GRAB" },
    })
}

/// The windows to draw: none on the page, CLOSE.marks being off. (close.ts closeMarks)
pub fn close_marks(g: &mut crate::game::Game) -> Vec<crate::close::CloseMark> { Vec::new() }

/// A man calm enough to sit. (close.ts calm)
pub fn calm(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let m = &g.gd[gd];
    m.state == crate::game::GuardState::Patrol && m.awareness < 0.3 && m.held.is_none() && m.script.is_none()
}

/// Sit a calm staff man down on a mark; false if he cannot be sat. (close.ts closeSeat)
pub fn close_seat(g: &mut crate::game::Game, gd: GuardId, at: Option<&[f64]>, face: Option<f64>) -> bool {
    if g.gd[gd].civ.is_some() || g.gd[gd].unit.is_some() || g.gd[gd].state == crate::game::GuardState::Dead || !calm(g, gd) { return false; }
    g.m.close.seated.get_or_insert_with(JsMap::new);
    let prev = seam_seat_get(g, gd);
    if prev.is_some() && at.is_none() { return true; }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let st = crate::close::Seat {
        at: match at { Some(a) => [a[0], 0.0, a[2]], None => [pos[0], 0.0, pos[2]] },
        face: face.unwrap_or(g.ch[c].body_yaw),
        hold: match prev { Some(prev) => prev.hold, None => g.gd[gd].hold },
        down: false,
    };
    seam_seat_set(g, gd, st);
    g.gd[gd].hold = true;
    g.stats.close.seats += 1;
    note(g, gd, crate::close::NoteWhat::Seat, None, None, None);
    true
}

/// Up out of the chair, his hold flag as it was. (close.ts closeUnseat)
pub fn close_unseat(g: &mut crate::game::Game, gd: GuardId, back: Option<bool>) {
    let back = back.unwrap_or(false);
    g.m.close.seated.get_or_insert_with(JsMap::new);
    let st = match seam_seat_get(g, gd) { Some(st) => st, None => return };
    seam_seat_delete(g, gd);
    g.gd[gd].hold = st.hold;
    let c = g.gd[gd].ch;
    if g.ch[c].anim.act.as_ref().map_or(false, |a| a.clip == sit_clip().clip) { g.ch[c].anim.act = None; }
    if back && st.down && g.gd[gd].state != crate::game::GuardState::Dead {
        g.ch[c].pos[0] = st.at[0];
        g.ch[c].pos[2] = st.at[2];
    }
    g.stats.close.unseats += 1;
    note(g, gd, crate::close::NoteWhat::Unseat, None, None, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::close::{CloseId as I, CloseKind as K};

    /// The standing staff man's table of close_windows, at the page's numbers.
    fn standing() -> Vec<crate::close::CloseWindow> {
        let rear_half = m::acos(-BEHIND_COS);
        let rear = m::PI - rear_half;
        let front = v3::clamp(50.0, 10.0, 100.0) * DEGR;
        let mid = (front + rear) / 2.0;
        let half = (rear - front) / 2.0;
        vec![
            arc(I::Behind, K::Behind, m::PI, rear_half, 1.05, true, true),
            arc(I::Left, K::Side, mid, half, 0.95, false, true),
            arc(I::Right, K::Side, -mid, half, 0.95, false, true),
            arc(I::Front, K::Front, 0.0, front, 0.85, false, true),
        ]
    }

    #[test]
    fn the_arcs_bits_are_the_typescripts() {
        let w = standing();
        // bun, EXACT.on, close.ts's own expressions
        let want: [(u64, u64, u64); 4] = BITS_ARCS;
        for (i, (mid, half, cos_half)) in want.iter().enumerate() {
            assert_eq!(w[i].mid.to_bits(), *mid, "mid {}", i);
            assert_eq!(w[i].half.to_bits(), *half, "half {}", i);
            assert_eq!(w[i].cos_half.to_bits(), *cos_half, "cosHalf {}", i);
        }
    }

    #[test]
    fn every_bearing_falls_in_the_typescripts_window() {
        let w = standing();
        // one letter per bearing from -3.2 by 0.05: windowAt under bun, EXACT.on
        let want = WINDOWS_SWEEP.as_bytes();
        for (i, c) in want.iter().enumerate() {
            let b = -3.2 + 0.05 * (i as f64);
            let got = match window_at(&w, b) { Some(x) => x.id.as_str().as_bytes()[0], None => b'-' };
            assert_eq!(got, *c, "bearing {}", b);
        }
    }

    #[test]
    fn no_offer_is_from_behind() {
        assert_eq!(close_kind_of(None), K::Behind);
        let o = crate::close::CloseOffer { win: Some(arc(I::Left, K::Side, 1.0, 0.5, 1.0, false, true)), ..crate::close::CloseOffer::zeroed() };
        assert_eq!(close_kind_of(Some(&o)), K::Side);
        assert_eq!(close_kind_of(Some(&crate::close::CloseOffer::zeroed())), K::Behind);
    }

    const BITS_ARCS: [(u64, u64, u64); 4] = [
        (0x400921fb54442d18, 0x3ff51700e0c14b25, 0x3fd0000000000000),
        (0x3ff591b27b4bccf0, 0x3fde6d0d31ed086d, 0x3fec73907aaf881c),
        (0xbff591b27b4bccf0, 0x3fde6d0d31ed086d, 0x3fec73907aaf881c),
        (0x0, 0x3febecde5da115a9, 0x3fe491b7523c161d),
    ];
    const WINDOWS_SWEEP: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbrrrrrrrrrrrrrrrrrrrffffffffffffffffffffffffffffffffffflllllllllllllllllllbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
}
