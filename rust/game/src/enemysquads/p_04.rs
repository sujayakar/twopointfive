//! src/game/enemysquads.ts lines 1150 to 1564
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `COUNT[n] ?? n`
fn count_or_n(n: usize) -> String {
    match count().get(n) { Some(w) => w.clone(), None => format!("{}", n) }
}
/// `x.toFixed(d)`: Rust's format rounds an exact tie to even, the language away from zero
fn fixed(x: f64, d: usize) -> String {
    let long = format!("{:.*}", 1100, x);
    if let Some(dot) = long.find('.') {
        let tail = &long[dot + 1 + d..];
        if tail.starts_with('5') && tail[1..].bytes().all(|b| b == b'0') {
            return format!("{:.*}", d, f64::from_bits(x.to_bits() + 1));
        }
    }
    format!("{:.*}", d, x)
}
fn pos_of(g: &Game, m: GuardId) -> V3 { g.ch[g.gd[m].ch].pos }
fn sign_of(g: &Game, m: GuardId) -> String { g.gd[m].callsign.clone() }
fn name_of(g: &Game, q: EnemysquadsSquadId) -> String { g.tab.enemysquads_squad[q].name.clone() }
fn door_name(g: &Game, d: DoorId) -> String { g.dr[d].def.name.clone() }
fn def_gd(g: &Game, d: RoomfightDefId) -> GuardId { g.tab.roomfight_def[d].gd }
/// `X.men.some(m => !offNet(m))`: anybody of it answers   (netCheck's answers)
fn answers(g: &mut Game, x: EnemysquadsSquadId) -> bool {
    let men = g.tab.enemysquads_squad[x].men.clone();
    for m in men { if !crate::guards::off_net(g, m) { return true; } }
    false
}
/// `home(g, Q).find(D => D.gd !== m)?.gd`
fn home_but(g: &mut Game, q: EnemysquadsSquadId, m: GuardId) -> Option<GuardId> {
    let h = home(g, q);
    h.into_iter().map(|d| def_gd(g, d)).find(|&x| x != m)
}
/// `home(g, Q)[0]?.gd`
fn home_first(g: &mut Game, q: EnemysquadsSquadId) -> Option<GuardId> {
    let h = home(g, q);
    h.first().map(|&d| def_gd(g, d))
}
/// `Q.men.find(m => fit(g, m))`, with one man left out
fn fit_man(g: &mut Game, q: EnemysquadsSquadId, but: Option<GuardId>) -> Option<GuardId> {
    let men = g.tab.enemysquads_squad[q].men.clone();
    for x in men { if Some(x) != but && fit(g, x) { return Some(x); } }
    None
}

/// `STATES.get(g)!`, for a function the skeleton has reach it by its path: read, and let go before the next call that takes g
fn the_state(g: &Game) -> &State { g.m.enemysquads.states.as_ref().unwrap() }
/// `st.nc!` (netCheck's N)
fn net_of(g: &Game) -> &NetCheck { the_state(g).nc.as_ref().unwrap() }
fn net_mut(g: &mut Game) -> &mut NetCheck { g.m.enemysquads.states.as_mut().unwrap().nc.as_mut().unwrap() }

/// is a man of the squad on the air about this mate's body   (enemysquads.ts callAbout)
pub fn call_about(g: &mut crate::game::Game, q: EnemysquadsSquadId, body: GuardId) -> bool {
    match &g.tab.enemysquads_squad[q].calls {
        Some(l) => l.iter().any(|c| c.body == Some(body)),
        None => false,
    }
}

/// a caller dropped, taken, blinded or reeling is cut off: his call is gone   (enemysquads.ts callsCut)
pub fn calls_cut(g: &mut crate::game::Game, q: EnemysquadsSquadId) {
    let mut i = g.tab.enemysquads_squad[q].calls.as_ref().unwrap().len();
    while i > 0 {
        i -= 1;
        let c = g.tab.enemysquads_squad[q].calls.as_ref().unwrap()[i].clone();
        if fit(g, c.by) { continue; }
        g.tab.enemysquads_squad[q].calls.as_mut().unwrap().remove(i);
        let m = c.by;
        let sent_by = if net_asks(g) && crate::guards::off_net(g, m) && g.tab.enemysquads_squad[q].state != SquadState::Contact {
            g.tab.enemysquads_squad[q].rc.as_ref().and_then(|r| r.net_out.as_ref()).and_then(|o| o.get(m)).copied()
        } else { None };
        let st_is = sent_by.is_some() && g.m.enemysquads.states.is_some();
        let says = match sent_by {
            Some(s) if st_is && g.time - s.t < NET_OUT_SECS => home_but(g, q, m),
            _ => None,
        };
        if let (Some(s), true, Some(says)) = (sent_by, st_is, says) {
            let x = s.about;
            let (xn, qn, ms, ss, up) = (name_of(g, x), name_of(g, q), sign_of(g, m), sign_of(g, says), alive(g, m));
            let why = format!("{}'s call is cut off {} s in, {} s short ({}): he was sent to look at {}, and his breaking off is what he found — {} calls it", ms, fixed(g.time - c.t0, 1), fixed(c.end - g.time, 1), if !up { "down" } else { "taken" }, xn, ss);
            event(g, q, EventWhat::CutOff, &why, Some(m));
            let rc = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
            rc.net_out.as_mut().unwrap().delete(m);
            rc.known.add(m);
            if !alive(g, m) { g.tab.enemysquads_squad[q].fallen.add(m); }
            if !answers(g, x) {
                if let Some(nc) = g.m.enemysquads.states.as_mut().and_then(|st| st.nc.as_mut()) { nc.done.add(x); }
            }
            g.tab.enemysquads_squad[x].trouble = None;
            let said = mate(g, m);
            let words = ContactWords { say: format!("{}: {} cut off at {} — contact, {}!", qn, said, xn, xn), why: format!("{}'s posts: {}, sent there to look, keyed up and was cut off", xn, ms) };
            contact(g, q, s.at, None, Some(true), Some(says), Some(&words));
            continue;
        }
        let tail = cut_ask(g, q, m);
        if !crate::guards::off_net(g, m) && g.k.squads.call_cut_ask[1] > 0.0 && g.tab.enemysquads_squad[q].state != SquadState::Contact {
            let t = g.time;
            g.tab.enemysquads_squad[q].hung.get_or_insert_with(Vec::new).push(SquadHungItem { who: m, t });
        }
        let how = if !alive(g, m) { "down" } else if g.gd[m].held.is_some() { "taken" } else { "blinded or reeling" };
        let why = format!("{}'s call is cut off {} s in, {} s short ({}): the squad has been told nothing{}", sign_of(g, m), fixed(g.time - c.t0, 1), fixed(c.end - g.time, 1), how, tail);
        event(g, q, EventWhat::CutOff, &why, Some(m));
    }
}

/// the mates of a man whose call broke off, he being off the net: filed where he is and asked for by name. '' = nobody asks   (enemysquads.ts cutAsk)
pub fn cut_ask(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> String {
    let [lo, hi] = g.k.squads.call_cut_ask;
    let check_ask = g.k.squads.check_ask;
    let by = if hi > 0.0 && crate::guards::off_net(g, m) && g.tab.enemysquads_squad[q].state != SquadState::Contact {
        match home_but(g, q, m) { Some(x) => Some(x), None => fit_man(g, q, Some(m)) }
    } else { None };
    let by = match by { Some(x) => x, None => return String::new() };
    rc_of(g, q);   // (made here; each write below goes through Q.rc, since the calls that take g fall between them)
    if g.tab.enemysquads_squad[q].ask_die.is_none() {
        let d = check_dice(g, q, Some(0x3c6ef372u32 as f64));
        g.tab.enemysquads_squad[q].ask_die = Some(d);
    }
    let ask = lo + crate::math_dice::roll(g.tab.enemysquads_squad[q].ask_die.as_mut().unwrap()) * (hi - lo);
    let d0 = def_of(g, m);
    let p = pos_of(g, m);
    let from = g.tab.enemysquads_squad[q].rc.as_ref().unwrap().afield.as_ref().and_then(|a| a.get(m)).copied();
    if let Some(f) = from {
        if flat(p, f) <= 3.0 { g.tab.enemysquads_squad[q].rc.as_mut().unwrap().afield.as_mut().unwrap().delete(m); }
    }
    let out = {
        let s = &g.tab.enemysquads_squad[q];
        s.rc.as_ref().unwrap().look.iter().any(|x| x.gd == m) || s.sent.iter().any(|x| x.gd == m) || d0.is_none() || from.map_or(false, |f| flat(p, f) > 3.0)
    };
    let at = if alive(g, m) && !out { Some(g.tab.roomfight_def[d0.unwrap()].station) } else { None };
    went_silent(g, q, m, at, Some(true));
    g.tab.enemysquads_squad[q].rc.as_mut().unwrap().promised.get_or_insert_with(JsSet::new).add(m);
    let free = g.tab.enemysquads_squad[q].state != SquadState::Alerted;
    let now = g.time;
    {
        let c = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
        if free && c.stage == CheckStage::Idle && c.look.is_empty() {
            c.stage = CheckStage::Called; c.by_ear = false; c.caller = Some(by); c.who = Some(m); c.t0 = now + ask - check_ask;
            return format!(" — {} heard it break off and asks for him in {} s", sign_of(g, by), fixed(ask, 1));
        }
    }
    let (was, twin, stage, was_ear) = {
        let c = g.tab.enemysquads_squad[q].rc.as_ref().unwrap();
        let was = if c.who != Some(m) { c.who } else { None };
        let twin = was.is_some() && !c.by_ear && c.silent.iter().find(|x| Some(x.gd) == was).map(|x| x.t) == Some(now);
        (was, twin, c.stage, c.by_ear)
    };
    if free && stage == CheckStage::Called && !twin {
        if let (Some(w), true) = (was, was_ear) {
            let c = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
            if let Some(x) = c.silent.iter_mut().find(|x| x.gd == w) { x.heard = false; }
        } else if let Some(w) = was {
            let copy = g.tab.enemysquads_squad[q].rc.clone().unwrap();   // (read only: owedOf takes the check beside g)
            if owed_of(g, &copy, w) {
                g.tab.enemysquads_squad[q].rc.as_mut().unwrap().cut.get_or_insert_with(Vec::new).insert(0, CheckCutItem { who: w, at: now });
            }
        }
        let caller = g.tab.enemysquads_squad[q].rc.as_ref().unwrap().caller;
        let swap = was_ear || match caller { None => true, Some(x) => x == m || !fit(g, x) };
        let c = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
        if swap { c.caller = Some(by); }
        c.by_ear = false; c.who = Some(m); c.t0 = m::min(c.t0, now + ask - check_ask);
        let caller = c.caller;
        let who = match caller { Some(x) => sign_of(g, x), None => String::from("a mate") };
        let mid = match was { Some(w) => format!("was about to ask for {} with", sign_of(g, w)), None => String::from("has just called") };
        return format!(" — heard to break off: the check {} {} asks for him first, within {} s", who, mid, fixed(ask, 1));
    }
    let c = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
    c.cut.get_or_insert_with(Vec::new).push(CheckCutItem { who: m, at: now + ask });
    if c.beat.is_none() { c.beat = Some(c.next_at); }
    c.next_at = m::min(c.next_at, now + ask);
    let till = if !free { "the squad's word is out" } else if !c.look.is_empty() { "the men out looking are home" } else { "the check under way is done" };
    format!(" — heard to break off: he is asked for by name once {}", till)
}

/// off the net, and nobody knows or has given up on him   (enemysquads.ts owedOf)
pub fn owed_of(g: &mut crate::game::Game, c_2: &crate::enemysquads::Check, who: GuardId) -> bool {
    crate::guards::off_net(g, who) && !c_2.known.has(who) && !c_2.looked.has(who)
}

/// one whose call broke off while he reeled and who has gone off the net since is asked for after all   (enemysquads.ts callsHung)
pub fn calls_hung(g: &mut crate::game::Game, q: EnemysquadsSquadId) {
    let mut i = g.tab.enemysquads_squad[q].hung.as_ref().unwrap().len();
    while i > 0 {
        i -= 1;
        let h = g.tab.enemysquads_squad[q].hung.as_ref().unwrap()[i];
        let m = h.who;
        let on_air = g.tab.enemysquads_squad[q].calls.as_ref().map_or(false, |l| l.iter().any(|c| c.by == m));
        if g.time - h.t > HUNG_SECS || g.time < h.t || g.tab.enemysquads_squad[q].state == SquadState::Contact || on_air {
            g.tab.enemysquads_squad[q].hung.as_mut().unwrap().remove(i);
            continue;
        }
        if !crate::guards::off_net(g, m) { continue; }
        g.tab.enemysquads_squad[q].hung.as_mut().unwrap().remove(i);
        let tail = cut_ask(g, q, m);
        if tail.is_empty() { continue; }
        let what = format!("{}: {}, whose call broke off {} s ago, is {} and has said nothing since{}", name_of(g, q), sign_of(g, m), fixed(g.time - h.t, 1), if alive(g, m) { "taken" } else { "down" }, tail);
        note(g, Some(m), &what);
    }
}

/// the first call to end, or every call once the floor is loud, is the squad's: returned for the caller to make the contact   (enemysquads.ts callsTick)
pub fn calls_tick(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::Call> {
    if busy { g.tab.enemysquads_squad[q].calls.as_mut().unwrap().clear(); return None; }
    let loud = !call_quiet(g, q);
    let now = g.time;
    let mut done: Option<Call> = None;
    for c in g.tab.enemysquads_squad[q].calls.as_ref().unwrap().iter() {
        if (loud || now >= c.end) && done.as_ref().map_or(true, |d| c.end < d.end) { done = Some(c.clone()); }
    }
    let done = done?;
    g.tab.enemysquads_squad[q].calls.as_mut().unwrap().clear();
    let early = loud && now < done.end;
    if let (Some(body), true) = (done.body, g.tab.enemysquads_squad[q].rc.is_some()) {
        g.tab.enemysquads_squad[q].rc.as_mut().unwrap().known.add(body);
        if net_asks(g) {
            let x = g.m.enemysquads.states.as_ref().and_then(|st| st.of.get(body)).copied();
            if let Some(x) = x { if x != q { seen_to(g, x, body); } }
        }
        let why = format!("{} has said it: {} is down{}, {} s after he keyed up", sign_of(g, done.by), sign_of(g, body), if early { " (the floor went loud under his call)" } else { "" }, fixed(now - done.t0, 1));
        event(g, q, EventWhat::Found, &why, Some(done.by));
    } else {
        let why = format!("{}'s call is through{}, {} s after he keyed up", sign_of(g, done.by), if early { " at once (the floor went loud under it)" } else { "" }, fixed(now - done.t0, 1));
        event(g, q, EventWhat::Said, &why, Some(done.by));
    }
    Some(done)
}

/// tools and the panel: the calls on the air in this game right now   (enemysquads.ts squadCalls)
pub fn squad_calls(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadCallsItem> {
    let mut out = Vec::new();
    if let Some(st) = g.m.enemysquads.states.as_ref() {
        for &q in st.squads.iter() {
            let s = &g.tab.enemysquads_squad[q];
            if let Some(l) = &s.calls {
                for c in l.iter() { out.push(SquadCallsItem { squad: s.name.clone(), by: c.by, t0: c.t0, end: c.end, body: c.body }); }
            }
        }
    }
    out
}

/// every squad has heard that one of them is not answering: stood to, counting its own men oftener   (enemysquads.ts netWary)
pub fn net_wary(g: &mut crate::game::Game, about: &str) {
    let squads = the_state(g).squads.clone();
    for q in squads {
        if g.tab.enemysquads_squad[q].state == SquadState::Down { continue; }
        if g.tab.enemysquads_squad[q].wary != Some(true) {
            let what = format!("{} is wary ({} is not answering): stood to, its own men counted oftener", name_of(g, q), about);
            note(g, None, &what);
        }
        let s = &mut g.tab.enemysquads_squad[q];
        s.wary = Some(true);
        if s.state == SquadState::Hold { s.ready = true; }
    }
}

/// a man of squad X has been seen down by a man of another squad, and it is on the net   (enemysquads.ts seenTo)
pub fn seen_to(g: &mut crate::game::Game, x: EnemysquadsSquadId, body: GuardId) {
    if let Some(rc) = g.tab.enemysquads_squad[x].rc.as_mut() { rc.known.add(body); }
    if !answers(g, x) {
        if let Some(nc) = g.m.enemysquads.states.as_mut().and_then(|st| st.nc.as_mut()) { nc.done.add(x); }
    }
    g.tab.enemysquads_squad[x].trouble = None;
}

/// a line said a breath after the one it answers   (enemysquads.ts netSay)
pub fn net_say(g: &mut crate::game::Game, after: f64, gd: GuardId, text: &str, cls: Option<f64>) {
    let t = g.time + after;
    if let Some(nc) = g.m.enemysquads.states.as_mut().and_then(|st| st.nc.as_mut()) {
        nc.say.push(NetCheckSayItem { t, gd, text: String::from(text), cls });
    } else {
        on_net(g, gd, text, cls, None);
    }
}

/// men a squad has asked for or sent, with no answer and no end heard   (enemysquads.ts waitsOn)
pub fn waits_on(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<GuardId> {
    let mut out = Vec::new();
    if g.tab.enemysquads_squad[q].rc.is_none() { return out; }
    let men = g.tab.enemysquads_squad[q].men.clone();
    for m in men {
        let waited = {
            let c = g.tab.enemysquads_squad[q].rc.as_ref().unwrap();
            c.looked.has(m) || c.asked.as_ref().map_or(false, |a| a.has(m)) || c.out.as_ref().map_or(false, |o| o.has(m))
        };
        if waited && crate::guards::off_net(g, m) && !g.tab.enemysquads_squad[q].rc.as_ref().unwrap().known.has(m) { out.push(m); }
    }
    out
}

/// trouble of its own: a man asked for, men out, a call that broke off, or men it sent not heard of since (netSend's test)
fn own_trouble(g: &mut Game, q: EnemysquadsSquadId) -> bool {
    let s = &g.tab.enemysquads_squad[q];
    if let Some(c) = &s.rc {
        if !c.look.is_empty() || c.stage == CheckStage::Asked || c.cut.as_ref().map_or(false, |l| !l.is_empty()) { return true; }
    }
    if s.trouble.is_some() { return true; }
    let out = s.rc.as_ref().and_then(|c| c.out.as_ref()).map(|o| o.keys());
    if let Some(out) = out {
        for m in out {
            if crate::guards::off_net(g, m) && !g.tab.enemysquads_squad[q].rc.as_ref().unwrap().known.has(m) { return true; }
        }
    }
    false
}

/// men of the nearest squad that can spare them set off for squad X's posts; None = nobody anywhere can go   (enemysquads.ts netSend)
pub fn net_send(g: &mut crate::game::Game, x: EnemysquadsSquadId, who: GuardId, stops: &[V3], all: bool) -> Option<crate::enemysquads::NetSend> {
    let at = stops[0];
    let mut best: Option<(EnemysquadsSquadId, Vec<RoomfightDefId>, f64)> = None;
    let (squads, stay_ask) = { let st = the_state(g); (st.squads.clone(), st.stay_ask) };
    for q in squads {
        let stays = g.tab.enemysquads_squad[q].def.may_leave == Some(false);
        let n = if stays { g.k.stayask.men } else { g.k.squads.net_look_men };
        if q == x || n < 1.0 || g.tab.enemysquads_squad[q].state != SquadState::Hold || (stays && stay_ask != Some(true)) { continue; }
        let sent: Vec<GuardId> = g.tab.enemysquads_squad[q].sent.iter().map(|s| s.gd).collect();
        let mut one_out = false;
        for s in sent { if alive(g, s) { one_out = true; break; } }
        if one_out || g.tab.enemysquads_squad[q].calls.as_ref().map_or(false, |l| !l.is_empty()) { continue; }
        if own_trouble(g, q) { continue; }
        let h = home(g, q);
        let free: Vec<RoomfightDefId> = h.iter().copied().filter(|&d| { let m = def_gd(g, d); !g.tab.enemysquads_squad[q].went.iter().any(|s| s.gd == m) }).collect();
        let go = goers(g, &free, at, m::min(n, h.len() as f64 - 1.0));
        if (go.len() as f64) < n { continue; }
        let ds: Vec<f64> = go.iter().map(|&d| flat(pos_of(g, def_gd(g, d)), at)).collect();
        let d = m::min_n(&ds);
        if best.as_ref().map_or(true, |b| d < b.2) { best = Some((q, go, d)); }
    }
    let (bq, go, _) = best?;
    rc_of(g, bq);   // (made here; each write below goes through Q.rc, since looker and walk take g between them)
    for &d in go.iter() {
        let mut l = looker(g, bq, d, who, at);
        l.about = Some(x);
        l.all = Some(all);
        l.stops = Some(stops[1..].to_vec());
        let gd = def_gd(g, d);
        let t = g.time;
        let c = g.tab.enemysquads_squad[bq].rc.as_mut().unwrap();
        c.look.push(l);
        c.out.get_or_insert_with(JsSet::new).add(gd);
        c.net_out.get_or_insert_with(JsMap::new).set(gd, CheckNetOutValue { about: x, at, t });
        walk(g, d, at);
    }
    Some(NetSend { q: bq, go })
}

/// (netCheck's beat)
fn nc_beat(g: &mut Game) -> f64 {
    let any = the_state(g).squads.iter().any(|&q| { let s = &g.tab.enemysquads_squad[q]; s.wary == Some(true) || s.news_t > -100.0 || s.contact_t > -100.0 });
    let r = crate::math_dice::roll(&mut net_mut(g).die);
    let k = &g.k.squads;
    k.net_every * (if any { k.check_alert_k } else { 1.0 }) * (1.0 + (r * 2.0 - 1.0) * k.check_jitter)
}
/// (netCheck's idle)
fn nc_idle(g: &mut Game, after: f64) {
    let t = g.time;
    let n = net_mut(g);
    n.stage = NetCheckStage::Idle; n.about = None; n.caller = None; n.next_at = t + after;
}
/// `st.squads.filter(answers).length`
fn answering(g: &mut Game) -> usize {
    let squads = the_state(g).squads.clone();
    let mut n = 0;
    for q in squads { if answers(g, q) { n += 1; } }
    n
}
/// X was asked for by name and nobody went: once more at a later check, then the net lets it be   (netCheck's failed)
fn nc_failed(g: &mut Game, x: EnemysquadsSquadId, why: &str) {
    let n = net_of(g).tried.get(x).copied().unwrap_or(0.0) + 1.0;
    let cap = if answering(g) < 2 { 1.0 } else { NET_TRIES };
    net_mut(g).tried.set(x, n);
    g.tab.enemysquads_squad[x].trouble = None;
    if n >= cap { net_mut(g).done.add(x); }
    let caller = net_of(g).caller;
    let what = format!("{} {}: {}", why, name_of(g, x), if n >= cap { "the net does not ask for it again — the squads stay wary" } else { "asked for once more at a later check" });
    note(g, caller, &what);
}
/// men on their way to X's posts   (netCheck's outFor)
fn out_for(g: &Game, x: EnemysquadsSquadId) -> bool {
    the_state(g).squads.iter().any(|&q| g.tab.enemysquads_squad[q].rc.as_ref().map_or(false, |c| c.look.iter().any(|l| l.about == Some(x) && !l.back)))
}
/// not answering, and nobody knows why   (netCheck's silent)
fn nc_silent(g: &mut Game, x: EnemysquadsSquadId) -> bool {
    !answers(g, x) && g.tab.enemysquads_squad[x].called_t <= -100.0 && !net_of(g).done.has(x) && !out_for(g, x)
}
/// (netCheck's posts) the places of X's men, as its own side knows them
struct Post { at: V3, m: GuardId }
fn posts(g: &mut Game, x: EnemysquadsSquadId) -> Vec<Post> {
    let mut p: Vec<Post> = Vec::new();
    let men = g.tab.enemysquads_squad[x].men.clone();
    for m in men {
        if let Some(d) = def_of(g, m) {
            let s = g.tab.roomfight_def[d].station;
            if !p.iter().any(|a| flat(a.at, s) < 2.5) { p.push(Post { at: v3::copy(s), m }); }
        }
    }
    p
}
/// the man who asks: the man at home nearest X's posts, of any other squad   (netCheck's asker)
fn asker(g: &mut Game, x: EnemysquadsSquadId) -> Option<GuardId> {
    let p = posts(g, x);
    let (mut best, mut bd) = (None, f64::INFINITY);
    let squads = the_state(g).squads.clone();
    for q in squads {
        if q == x { continue; }
        for d in home(g, q) {
            let m = def_gd(g, d);
            let at = pos_of(g, m);
            let dist = if !p.is_empty() { let v: Vec<f64> = p.iter().map(|a| flat(a.at, at)).collect(); m::min_n(&v) } else { 0.0 };
            if dist < bd { bd = dist; best = Some(m); }
        }
    }
    best
}
/// `st.of.get(m)!` (netCheck's sq)
fn sq_of(g: &Game, m: GuardId) -> EnemysquadsSquadId { *the_state(g).of.get(m).unwrap() }
/// (netCheck's askFor)
fn ask_for(g: &mut Game, x: EnemysquadsSquadId, by: GuardId, why: &str) {
    let t = g.time;
    let n = net_mut(g);
    n.stage = NetCheckStage::Asked; n.about = Some(x); n.caller = Some(by); n.t0 = t;
    let q = sq_of(g, by);
    let (qn, xn) = (name_of(g, q), name_of(g, x));
    on_net(g, by, &format!("{}: {}? … {}, sound off", qn, xn, xn), Some(2.0), None);
    event(g, q, EventWhat::NetAsks, why, Some(by));
}

/// the net's roll call of the squads, the question after a squad's trouble, the ask by name, and the men who go   (enemysquads.ts netCheck)
pub fn net_check(g: &mut crate::game::Game) {
    if the_state(g).nc.is_none() {
        let first = the_state(g).squads[0];
        let mut die = check_dice(g, first, Some(0x1b873593u32 as f64));
        let next_at = g.time + (0.5 + 0.5 * crate::math_dice::roll(&mut die)) * g.k.squads.net_every;
        let tick_t = g.time;
        g.m.enemysquads.states.as_mut().unwrap().nc = Some(NetCheck { next_at, stage: NetCheckStage::Idle, t0: 0.0, caller: None, about: None, done: JsSet::new(), tried: JsMap::new(), queried: JsMap::new(), say: Vec::new(), die, tick_t });
    }
    if g.time - net_of(g).tick_t > 1.0 { let b = nc_beat(g); nc_idle(g, b); }
    let t = g.time;
    net_mut(g).tick_t = t;
    let mut i = 0;
    while i < net_of(g).say.len() {
        if g.time < net_of(g).say[i].t { i += 1; continue; }
        let l = net_mut(g).say.remove(i);
        if !crate::guards::off_net(g, l.gd) { on_net(g, l.gd, &l.text, l.cls, None); }
    }
    let squads = the_state(g).squads.clone();
    let settle_secs = g.k.squads.settle_secs;
    let last_call = match the_state(g).net.last() { Some(c) => c.t, None => -100.0 };
    let in_fight = squads.iter().any(|&q| { let s = g.tab.enemysquads_squad[q].state; s == SquadState::Contact || s == SquadState::Alerted });
    if in_fight || g.time - last_call < settle_secs {
        let (stage, about, caller) = { let n = net_of(g); (n.stage, n.about, n.caller) };
        if let (NetCheckStage::Asked, Some(a)) = (stage, about) {
            nc_failed(g, a, "the net has a fight on it, and nobody is sent to look for");
        } else if stage != NetCheckStage::Idle {
            note(g, caller, "the net's check is dropped: the net has a fight on it");
        }
        for &x in squads.iter() {
            if g.tab.enemysquads_squad[x].trouble.is_some() && answers(g, x) { g.tab.enemysquads_squad[x].trouble = None; }
        }
        let t = g.time;
        let n = net_mut(g);
        n.stage = NetCheckStage::Idle; n.about = None; n.caller = None; n.next_at = m::max(n.next_at, t + 5.0);
        return;
    }
    if squads.iter().any(|&q| g.tab.enemysquads_squad[q].calls.as_ref().map_or(false, |l| !l.is_empty())) { return; }
    match net_of(g).stage {
        NetCheckStage::Idle => nc_stage_idle(g, &squads),
        NetCheckStage::Called => nc_stage_called(g, &squads),
        NetCheckStage::Asked => nc_stage_asked(g),
    }
}

/// (netCheck, 'idle') a squad's trouble on the air and nothing after; then the net's own check
fn nc_stage_idle(g: &mut Game, squads: &[EnemysquadsSquadId]) {
    for &x in squads.iter() {
        let t = match g.tab.enemysquads_squad[x].trouble { Some(t) => t, None => continue };
        let done = net_of(g).done.has(x);
        let again_secs = g.k.squads.again_secs;
        let now = g.time;
        if g.tab.enemysquads_squad[x].called_t > -100.0 || done || (answers(g, x) && squads.iter().any(|&q| { let c = g.tab.enemysquads_squad[q].contact_t; c > -100.0 && now - c < again_secs })) {
            g.tab.enemysquads_squad[x].trouble = None;
            continue;
        }
        let [lo, hi] = g.k.squads.net_beat;
        let due = t + lo + js::rem(t * 7.31, 1.0) * (hi - lo);
        if now < due || out_for(g, x) { continue; }
        let by = match asker(g, x) { Some(b) => b, None => continue };
        let asked = net_of(g).queried.get(x).copied().unwrap_or(0.0);
        if answers(g, x) && asked >= NET_TRIES { g.tab.enemysquads_squad[x].trouble = None; continue; }
        let xn = name_of(g, x);
        if !answers(g, x) {
            let why = format!("nothing from {} since the trouble on its net {} s ago: {} asks for the squad by name", xn, fixed(now - t, 1), sign_of(g, by));
            ask_for(g, x, by, &why);
            return;
        }
        let ans = match home_first(g, x) { Some(a) => Some(a), None => fit_man(g, x, None) };
        let (busy, looking) = match &g.tab.enemysquads_squad[x].rc {
            Some(c) => (!c.look.is_empty() || c.stage != CheckStage::Idle || c.cut.as_ref().map_or(false, |l| !l.is_empty()), !c.look.is_empty()),
            None => (false, false),
        };
        let gone = waits_on(g, x);
        let n = asked + 1.0;
        let bq = sq_of(g, by);
        let bn = name_of(g, bq);
        on_net(g, by, &format!("{}: {} — what's going on?", bn, xn), Some(4.0), None);
        if let Some(a) = ans {
            let text = if busy {
                format!("{}: {} — stand by", xn, if looking { "still looking" } else { "checking our own" })
            } else if !gone.is_empty() {
                let who = if gone.len() > 1 { format!("{} of ours", count_or_n(gone.len())) } else { mate(g, gone[0]) };
                format!("{}: still nothing from {}", xn, who)
            } else {
                format!("{}: all up here — nothing more", xn)
            };
            net_say(g, NET_BREATH, a, &text, Some(4.0));
        }
        let names: Vec<String> = gone.iter().map(|&m| sign_of(g, m)).collect();
        let heard = match ans {
            Some(a) => format!("{} answers{}", sign_of(g, a), if busy { String::from(", still at it") } else if !gone.is_empty() { format!(": {} not heard of", names.join(", ")) } else { String::new() }),
            None => String::from("a man of it is on the net and cannot speak yet"),
        };
        let why = format!("{} asks {} what became of the trouble on its net: {}", sign_of(g, by), xn, heard);
        event(g, bq, EventWhat::NetQuery, &why, Some(by));
        net_mut(g).queried.set(x, n);
        g.tab.enemysquads_squad[x].trouble = if (busy || ans.is_none()) && n < NET_TRIES { Some(now + hi) } else { None };
        if (ans.is_some() && !busy && !gone.is_empty()) || ((busy || ans.is_none()) && n >= NET_TRIES) {
            let first = match gone.first() { Some(&m) => sign_of(g, m), None => String::from("a man") };
            net_wary(g, &format!("{} of {}", first, xn));
        }
    }
    if g.time < net_of(g).next_at { return; }
    for &q in squads.iter() {
        if answers(g, q) && g.tab.enemysquads_squad[q].rc.as_ref().map_or(false, |c| c.stage != CheckStage::Idle) { return; }
    }
    if answering(g) < 2 {
        let mut any_silent = false;
        for &q in squads.iter() { if nc_silent(g, q) { any_silent = true; break; } }
        if !any_silent { let b = nc_beat(g); nc_idle(g, b); return; }
    }
    let mut lead = None;
    for &q in squads.iter() {
        if g.tab.enemysquads_squad[q].trouble.is_none() { if let Some(m) = home_first(g, q) { lead = Some(m); break; } }
    }
    if lead.is_none() {
        for &q in squads.iter() { if let Some(m) = home_first(g, q) { lead = Some(m); break; } }
    }
    let lead = match lead {
        Some(m) => m,
        None => { let t = g.time; net_mut(g).next_at = t + 5.0; return; }
    };
    let t = g.time;
    let n = net_mut(g);
    n.stage = NetCheckStage::Called; n.t0 = t; n.caller = Some(lead);
    let q = sq_of(g, lead);
    let text = format!("{}: all squads — radio check", name_of(g, q));
    on_net(g, lead, &text, Some(6.0), Some(false));
    let why = format!("{} calls the net's check of the squads", sign_of(g, lead));
    event(g, q, EventWhat::NetCheck, &why, Some(lead));
}

/// (netCheck, 'called') a breath for the answers, then the silent squad not asked for yet is asked for by name
fn nc_stage_called(g: &mut Game, squads: &[EnemysquadsSquadId]) {
    if g.time - net_of(g).t0 < g.k.squads.check_ask { return; }
    let mut silent = Vec::new();
    for &q in squads.iter() { if nc_silent(g, q) { silent.push(q); } }
    {
        let n = net_of(g);
        js::sort_by(&mut silent, |&a, &b| n.tried.get(a).copied().unwrap_or(0.0) - n.tried.get(b).copied().unwrap_or(0.0));
    }
    let x = match silent.first() { Some(&x) => x, None => { let b = nc_beat(g); nc_idle(g, b); return; } };
    let by = match asker(g, x) { Some(b) => b, None => { nc_idle(g, 5.0); return; } };
    let why = format!("no answer from {} to the net's check: {} asks for the squad by name", name_of(g, x), sign_of(g, by));
    ask_for(g, x, by, &why);
}

/// (netCheck, 'asked') it answers, or men are sent to its posts, or nobody is free
fn nc_stage_asked(g: &mut Game) {
    let (x, caller, t0) = { let n = net_of(g); (n.about.unwrap(), n.caller, n.t0) };
    let xn = name_of(g, x);
    if answers(g, x) || g.tab.enemysquads_squad[x].called_t > -100.0 {
        note(g, caller, &format!("{} is on the net again: nobody goes", xn));
        let b = nc_beat(g); nc_idle(g, b);
        return;
    }
    if g.time - t0 < g.k.squads.check_ask { return; }
    let mut p = posts(g, x);
    let mut stops: Vec<V3> = Vec::new();
    let mut who: Option<GuardId> = None;
    let mut cur: V3 = match caller { Some(c) => pos_of(g, c), None => p.first().map_or([0.0; 3], |a| a.at) };
    while !p.is_empty() && (stops.len() as f64) < g.k.squads.net_stops {
        let from = cur;
        js::sort_by(&mut p, |a, b| flat(a.at, from) - flat(b.at, from));
        let a = p.remove(0);
        if who.is_none() { who = Some(a.m); }
        cur = a.at;
        stops.push(a.at);
    }
    let who = match who {
        Some(w) => w,
        None => { net_mut(g).done.add(x); let b = nc_beat(g); nc_idle(g, b); return; }
    };
    let sent = net_send(g, x, who, &stops, true);
    let sent = match sent {
        Some(s) => s,
        None => {
            if let Some(c) = caller {
                if !crate::guards::off_net(g, c) {
                    let text = format!("{}: nothing from {} — nobody to send, stay on your doors!", name_of(g, sq_of(g, c)), xn);
                    on_net(g, c, &text, None, None);
                }
            }
            net_wary(g, &xn);
            nc_failed(g, x, "nobody free to go and look for");
            let b = nc_beat(g); nc_idle(g, b);
            return;
        }
    };
    net_mut(g).done.add(x);
    g.tab.enemysquads_squad[x].trouble = None;
    let first = def_gd(g, sent.go[0]);
    let text = format!("{}: nothing from {} — {} moving to their rooms!", name_of(g, sent.q), xn, count_or_n(sent.go.len()));
    on_net(g, first, &text, None, None);
    let names: Vec<String> = sent.go.iter().map(|&d| sign_of(g, def_gd(g, d))).collect();
    let why = format!("{} walk{} to {}'s posts ({} of them), {} m off: nobody knows what became of it, and every squad is wary", names.join(" and "), if sent.go.len() > 1 { "" } else { "s" }, xn, stops.len(), fixed(flat(pos_of(g, first), stops[0]), 0));
    event(g, sent.q, EventWhat::NetLooks, &why, Some(first));
    net_wary(g, &xn);
    let b = nc_beat(g); nc_idle(g, b);
}

/// what a tally and the panel read   (enemysquads.ts squadsReport)
pub fn squads_report(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadReport> {
    let squads = match g.m.enemysquads.states.as_ref() { Some(st) => st.squads.clone(), None => return Vec::new() };
    let mut out = Vec::new();
    for q in squads {
        let (men, sent) = { let s = &g.tab.enemysquads_squad[q]; (s.men.clone(), s.sent.iter().map(|x| x.gd).collect::<Vec<GuardId>>()) };
        let up = men.iter().filter(|&&m| alive(g, m)).count();
        let n_out = sent.iter().filter(|&&m| alive(g, m)).count();
        let s = &g.tab.enemysquads_squad[q];
        let events = s.events.iter().map(|e| { let mut c = e.clone(); c.t = m::round(e.t * 10.0) / 10.0; c }).collect();
        out.push(SquadReport { id: s.def.id.clone(), name: s.name.clone(), men: men.len() as f64, up: up as f64, state: String::from(s.state.as_str()), out: n_out as f64, events });
    }
    out
}

/// the panel's line   (enemysquads.ts squadsSummary)
pub fn squads_summary(g: &mut crate::game::Game) -> String {
    let squads = match g.m.enemysquads.states.as_ref() { Some(st) => st.squads.clone(), None => return String::from("no squads on this floor") };
    let mut rows: Vec<String> = Vec::new();
    for q in squads {
        let (men, sent) = { let s = &g.tab.enemysquads_squad[q]; (s.men.clone(), s.sent.iter().map(|x| x.gd).collect::<Vec<GuardId>>()) };
        let up = men.iter().filter(|&&m| alive(g, m)).count();
        let n_out = sent.iter().filter(|&&m| alive(g, m)).count();
        let s = &g.tab.enemysquads_squad[q];
        let tail = if n_out > 0 { format!(" ({} out)", n_out) } else { String::new() };
        rows.push(format!("{} {}/{} {}{}", s.name, up, men.len(), s.state.as_str(), tail));
    }
    rows.join(" · ")
}

/// a squad's men by its name, the fallen among them   (enemysquads.ts squadMen)
pub fn squad_men(g: &mut crate::game::Game, name: &str) -> Vec<GuardId> {
    if let Some(st) = g.m.enemysquads.states.as_ref() {
        for &q in st.squads.iter() {
            let s = &g.tab.enemysquads_squad[q];
            if s.name == name { return s.men.clone(); }
        }
    }
    Vec::new()
}

/// (squadsTick's lostWhy) a man who went round is down: where and when, for the squad's timeline
fn lost_why(g: &mut Game, s: &Sent) -> String {
    let there = s.there.is_some();
    let spot_near = match s.spot { Some(p) if there => flat(pos_of(g, s.gd), p) < 4.0, _ => false };
    let how = if s.back == Some(true) { "on his way back to his post" }
        else if s.phase == SentPhase::Run { "on the way" }
        else if spot_near { "at his spot" }
        else if there { if s.hold == Some(true) { "after leaving his spot" } else { "after going in" } }
        else if s.phase == SentPhase::Set { "where he stopped" }
        else { "in the fight he ran into" };
    let door = match s.door { Some(d) => format!(" ({})", door_name(g, d)), None => String::new() };
    format!("{} down {}{}, {} s after the word", sign_of(g, s.gd), how, door, fixed(g.time - s.t0, 1))
}

/// (squadsTick's home, in the sent walk) off his leg, at home wherever that now is; a man who went round is still his squad's to say
fn sent_home(g: &mut Game, q: EnemysquadsSquadId, i: usize) {
    let s = &mut g.tab.enemysquads_squad[q];
    let row = s.sent.remove(i);
    if row.role == SentRole::Flank {
        s.went.push(row);
        if s.went.len() > 8 { s.went.remove(0); }
    }
}

/// every squad's contact, news, choice and men on the move, once a frame   (enemysquads.ts squadsTick)
pub fn squads_tick(g: &mut crate::game::Game) {
    let squads = match g.m.enemysquads.states.as_ref() { Some(st) => st.squads.clone(), None => return };
    if g.fight.defs.is_empty() { return; }
    for &q in squads.iter() { tick_contact(g, q); }
    for &q in squads.iter() { tick_news(g, q); }
    for &q in squads.iter() {
        if g.tab.enemysquads_squad[q].state == SquadState::Alerted && g.time >= g.tab.enemysquads_squad[q].decide_at { decide(g, q); }
    }
    if net_asks(g) { net_check(g); }
    look_out(g);
    for &q in squads.iter() { tick_out(g, q); }
}

/// squadsTick 1: contact, by its own men's senses; then its men in its other rooms, their beat up
fn tick_contact(g: &mut Game, q: EnemysquadsSquadId) {
    let men = g.tab.enemysquads_squad[q].men.clone();
    let mut up: Vec<GuardId> = Vec::new();
    for &m in men.iter() { if alive(g, m) { up.push(m); } }
    if g.tab.enemysquads_squad[q].calls.as_ref().map_or(false, |l| !l.is_empty()) { calls_cut(g, q); }
    if g.tab.enemysquads_squad[q].hung.as_ref().map_or(false, |l| !l.is_empty()) { calls_hung(g, q); }
    if up.is_empty() {
        if g.tab.enemysquads_squad[q].state != SquadState::Down {
            let t = g.time;
            let s = &mut g.tab.enemysquads_squad[q];
            s.state = SquadState::Down;
            s.due.clear();
            s.events.push(SquadEvent { t, what: SquadEventWhat::Down, n: 0.0, why: String::from("every man of it is down") });
            let what = format!("{} down", s.name);
            note(g, None, &what);
        }
        return;
    }
    let mut at: Option<V3> = None;
    let mut room: Option<crate::scene_scenarios::StructureRoom> = None;
    let mut by_drop = false;
    for &m in up.iter() {
        if g.tab.enemysquads_squad[q].sent.iter().any(|s| s.gd == m) { continue; }
        let p0 = pos_of(g, m);
        let d = def_of(g, m);
        let mine = room_of(g, q, p0, Some(1.0));
        let (d, mine) = match (d, mine) { (Some(d), Some(r)) => (d, r), _ => continue };
        if g.time - g.gd[m].last_seen_t < 1.0 && (noticed(g, m) || glimpse(g, q, m)) {
            let known = g.gd[m].last_known;
            let p = match known { Some(k) if !g.tab.roomfight_def[d].fix_is_pt => k, _ => pos_of(g, m) };
            let r = match room_of(g, q, p, Some(0.75)) { Some(r) => r, None => mine };
            if !(noticed(g, m) && key_up(g, q, m, p, Some(&r), None)) { at = Some(p); room = Some(r); break; }
        }
        let (noise, noise_t) = { let x = &g.tab.roomfight_def[d]; (x.noise_at, x.noise_t) };
        if let Some(n) = noise {
            if g.time - noise_t < 1.0 {
                if let Some(r) = room_of(g, q, n, None) { at = Some(n); room = Some(r); break; }
            }
        }
    }
    for &m in men.iter() {
        if alive(g, m) || g.tab.enemysquads_squad[q].fallen.has(m) { continue; }
        g.tab.enemysquads_squad[q].fallen.add(m);
        let p = pos_of(g, m);
        let r = if g.tab.enemysquads_squad[q].sent.iter().any(|s| s.gd == m) { None } else { room_of(g, q, p, Some(1.0)) };
        if r.is_some() && at.is_none() && g.tab.enemysquads_squad[q].state != SquadState::Contact && !heard_fire(g, q, &up) {
            went_silent(g, q, m, None, None);
            continue;
        }
        if r.is_some() && at.is_none() {
            let mut one_home = false;
            for &x in up.iter() { let xp = pos_of(g, x); if room_of(g, q, xp, Some(1.0)).is_some() { one_home = true; break; } }
            if one_home { at = Some(p); room = r; by_drop = true; }
        }
    }
    let busy = at.is_some() || g.tab.enemysquads_squad[q].state == SquadState::Contact;
    if let Some(f) = radio_check(g, q, busy) { at = Some(f.at); room = f.room; by_drop = true; }
    let mut by: Option<GuardId> = None;
    if g.tab.enemysquads_squad[q].calls.as_ref().map_or(false, |l| !l.is_empty()) {
        let busy = at.is_some() || g.tab.enemysquads_squad[q].state == SquadState::Contact;
        if let Some(c) = calls_tick(g, q, busy) { at = Some(c.at); room = c.room; by_drop = !c.eyes; by = Some(c.by); }
    }
    if let Some(a) = at {
        contact(g, q, a, room.as_ref(), Some(by_drop), by, None);
    } else if g.tab.enemysquads_squad[q].state == SquadState::Contact && g.time - g.tab.enemysquads_squad[q].contact_t > g.k.squads.quiet_secs {
        g.tab.enemysquads_squad[q].state = SquadState::Hold;
        g.tab.enemysquads_squad[q].fix = None;
        event(g, q, EventWhat::Quiet, "nothing of the team seen or heard in its rooms", None);
    }
    tick_due(g, q, &men);
}

/// squadsTick 1, its tail: its men in its other rooms, their beat up
fn tick_due(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId]) {
    let mut i = g.tab.enemysquads_squad[q].due.len();
    while i > 0 {
        i -= 1;
        let due = g.tab.enemysquads_squad[q].due[i].clone();
        if g.time < due.t { continue; }
        g.tab.enemysquads_squad[q].due.remove(i);
        let h = home(g, q);
        let d = match h.iter().copied().find(|&x| def_gd(g, x) == due.gd) { Some(d) => d, None => continue };
        if g.tab.enemysquads_squad[q].state != SquadState::Contact { continue; }
        let lost = match due.room.clone() { Some(r) => Some(r), None => g.tab.enemysquads_squad[q].hit_room.clone() };
        let mut up_in = 0usize;
        if let Some(l) = &lost {
            for &m in men.iter() {
                if !alive(g, m) { continue; }
                let p = pos_of(g, m);
                if room_of(g, q, p, Some(0.05)).map_or(false, |r| r.id == l.id) { up_in += 1; }   // (`===` of a room: its id, a room is one row of the structure)
            }
        }
        let drill = squad_drill(g);
        if let (Some(k), Some(l)) = (&drill, &lost) {
            if k.drill_own && (up_in as f64) < k.own_up {
                let said = g.tab.enemysquads_squad[q].own_said;
                if said.map_or(true, |t| !js::truthy(t) || g.time - t > 20.0) {
                    g.tab.enemysquads_squad[q].own_said = Some(g.time);
                    let text = format!("{}: {}'s {} — stay on your doors, make them come!", name_of(g, q), l.name, if up_in > 0 { "hit" } else { "gone" });
                    crate::game::game_say(g, due.gd, &text, Some(true), None, None);
                }
                let key = stay_key(g, due.gd, &StayKeyRoom { name: l.name.clone() });
                g.tab.enemysquads_squad[q].stayed.get_or_insert_with(JsSet::new).add(key);
                let how_many = if up_in == 1 { String::from("one man") } else if up_in > 0 { format!("{} men", count().get(up_in).map_or("undefined", |w| w.as_str())) } else { String::from("nobody") };
                let why = format!("{} does not go to {}: {} of the squad is up in it", sign_of(g, due.gd), l.name, how_many);
                event(g, q, EventWhat::Stays, &why, Some(due.gd));
                continue;
            }
        }
        let t = g.time;
        let from = v3::copy(g.tab.roomfight_def[d].station);
        g.tab.enemysquads_squad[q].sent.push(Sent { gd: due.gd, from, role: SentRole::Reinforce, phase: SentPhase::Loose, spot: None, door: None, fix_at: due.at, tries: 0.0, step_t: -100.0, t0: t, ..Sent::zeroed() });
        crate::pressure::loose(g, d, due.at);
        let (a, b) = (pos_of(g, due.gd), g.gd[due.gd].last_known.unwrap_or(due.at));
        let way = crate::dialogue::way_name(g, a, b, lost.as_ref().map(|l| l.name.as_str()));
        let way = match way { Some(w) if !w.is_empty() => format!(" {}", w), _ => String::new() };
        let text = format!("{}: coming to you{} — hold on!", name_of(g, q), way);
        crate::game::game_say(g, due.gd, &text, Some(true), None, None);
        let why = format!("{} goes to the squad's room under attack", sign_of(g, due.gd));
        event(g, q, EventWhat::Reinforce, &why, Some(due.gd));
    }
}

/// squadsTick 2: news, the net's word of another squad's contact or its own men's ears on a loud thing outside its rooms
fn tick_news(g: &mut Game, q: EnemysquadsSquadId) {
    let state = g.tab.enemysquads_squad[q].state;
    if state == SquadState::Down || state == SquadState::Contact || state == SquadState::Alerted { return; }
    let settled = g.time - g.tab.enemysquads_squad[q].decided_at >= g.k.squads.settle_secs;
    if !settled && !g.tab.enemysquads_squad[q].ready { return; }
    let mut fix: Option<Fix> = None;
    let nk = net_k(g);
    let beat = if nk != 1.0 && may_go(g, q) { nk } else { 1.0 };
    let waits = beat > 1.0;
    let now = g.time;
    let news_t = g.tab.enemysquads_squad[q].news_t;
    let net_lag = g.k.squads.net_lag;
    {
        let net = &g.m.enemysquads.states.as_ref().unwrap().net;
        let mut i = net.len();
        while i > 0 && fix.is_none() {
            i -= 1;
            let c = &net[i];
            if now - c.t > 10.0 { break; }
            if c.squad != q && c.t > news_t && now - c.t >= net_lag * beat { fix = Some(Fix { at: c.at, t: c.t, ally: Some(c.squad), by: FixBy::Net }); }
        }
    }
    if fix.is_none() && settled {
        for d in home(g, q) {
            let (noise, out, t) = { let x = &g.tab.roomfight_def[d]; (x.noise_at, x.noise_out, x.noise_t) };
            let n = match noise { Some(n) => n, None => continue };
            if !out || t <= news_t || now - t > 1.5 || room_of(g, q, n, None).is_some() { continue; }
            fix = Some(Fix { at: v3::copy(n), t, ally: None, by: FixBy::Ears });
            break;
        }
    }
    let fix = match fix { Some(f) => f, None => return };
    g.tab.enemysquads_squad[q].news_t = m::max(news_t, fix.t);
    let same = match &g.tab.enemysquads_squad[q].fix {
        Some(old) => if fix.by == FixBy::Net { old.ally == fix.ally } else { flat(old.at, fix.at) < 10.0 },
        None => false,
    };
    if same && g.tab.enemysquads_squad[q].state != SquadState::Hold && now - g.tab.enemysquads_squad[q].decided_at < g.k.squads.again_secs { return; }
    let wait = if g.tab.enemysquads_squad[q].ready && fix.by == FixBy::Net && !waits {
        g.k.squads.ready_beat
    } else {
        let think = g.k.squads.think;
        pick(g, think) * beat
    };
    let decide_at = now + wait;
    let s = &mut g.tab.enemysquads_squad[q];
    s.decide_at = decide_at;
    s.fix = Some(fix);
    s.state = SquadState::Alerted;
    let whose = if fix.by == FixBy::Net { format!("{}'s call", name_of(g, fix.ally.unwrap())) } else { String::from("its own ears") };
    let what = format!("{} hears of them ({}){}: its word in {} s", name_of(g, q), whose, if waits { ", holds" } else { "" }, fixed(decide_at - now, 1));
    note(g, None, &what);
}

/// squadsTick 3: the men who were sent
fn tick_out(g: &mut Game, q: EnemysquadsSquadId) {
    if g.tab.enemysquads_squad[q].rc.is_some() {
        let mut i = g.tab.enemysquads_squad[q].rc.as_ref().unwrap().look.len();
        while i > 0 {
            i -= 1;
            let l = g.tab.enemysquads_squad[q].rc.as_ref().unwrap().look[i].gd;
            if !looks(g, q, l) { continue; }
            let l = g.tab.enemysquads_squad[q].rc.as_ref().unwrap().look[i].clone();
            if g.k.squads.call_cut_ask[1] > 0.0 && !l.back && alive(g, l.gd) {
                g.tab.enemysquads_squad[q].rc.as_mut().unwrap().afield.get_or_insert_with(JsMap::new).set(l.gd, l.from);
            }
            if l.back && !crate::guards::off_net(g, l.gd) {
                let c = g.tab.enemysquads_squad[q].rc.as_mut().unwrap();
                if let Some(o) = c.net_out.as_mut() { o.delete(l.gd); }
                if let Some(o) = c.out.as_mut() { o.delete(l.gd); }
            }
            g.tab.enemysquads_squad[q].rc.as_mut().unwrap().look.remove(i);
        }
    }
    let mut i = g.tab.enemysquads_squad[q].went.len();
    while i > 0 {
        i -= 1;
        let w = g.tab.enemysquads_squad[q].went[i].clone();
        if alive(g, w.gd) { continue; }
        let why = lost_why(g, &w);
        event(g, q, EventWhat::Lost, &why, None);
        g.tab.enemysquads_squad[q].went.remove(i);
    }
    let mut i = g.tab.enemysquads_squad[q].sent.len();
    while i > 0 {
        i -= 1;
        tick_sent(g, q, i);
    }
}

/// squadsTick 3, one row of Q.sent (`s` is a copy read as the row stood; every write goes to the row by its index)
fn tick_sent(g: &mut Game, q: EnemysquadsSquadId, i: usize) {
    let s = g.tab.enemysquads_squad[q].sent[i].clone();
    let gd = s.gd;
    let d = def_of(g, gd);
    let d = match d {
        Some(d) if alive(g, gd) => d,
        _ => {
            if s.role == SentRole::Flank && !alive(g, gd) { let why = lost_why(g, &s); event(g, q, EventWhat::Lost, &why, None); }
            g.tab.enemysquads_squad[q].sent.remove(i);
            return;
        }
    };
    let mode = g.tab.roomfight_def[d].mode;
    if s.phase == SentPhase::Run { tick_run(g, q, i, s, d, mode); return; }
    if s.phase == SentPhase::Set {
        if mode != crate::roomfight::DefMode::Hold {
            g.tab.enemysquads_squad[q].sent[i].phase = SentPhase::Loose;
        } else if g.tab.enemysquads_squad[q].state == SquadState::Contact || (g.time - s.set_t.unwrap_or(s.t0) > g.k.squads.hold_secs && g.time - g.gd[gd].last_seen_t > 8.0) {
            sent_home(g, q, i);
        } else if s.hold == Some(true) && s.there.is_some() && squad_drill(g).map_or(false, |k| k.drill_back) {
            back(g, q, gd, d);
        }
        return;
    }
    if s.role == SentRole::Look && mode == crate::roomfight::DefMode::Hold && g.time - s.t0 > 6.0 && fit(g, gd) && g.time - g.gd[gd].last_seen_t > 8.0 && { let p = pos_of(g, gd); room_of(g, q, p, Some(1.0)).is_none() } {
        let mate_back = g.tab.enemysquads_squad[q].sent.iter().enumerate().any(|(j, x)| j != i && x.role == SentRole::Look && x.back == Some(true));
        if !mate_back {
            let text = format!("{}: no sign of them — back on our doors!", name_of(g, q));
            crate::game::game_say(g, gd, &text, Some(true), None, None);
            let why = format!("{} and his mate found nothing where the team was last put, {} s after the word", sign_of(g, gd), fixed(g.time - s.t0, 1));
            event(g, q, EventWhat::Back, &why, Some(gd));
        }
        let r = &mut g.tab.enemysquads_squad[q].sent[i];
        r.phase = SentPhase::Run; r.back = Some(true); r.spot = Some(s.from); r.door = None; r.way = None; r.tries = 0.0;
        crate::pressure::run(g, d, s.from, None, Some(0.3));
        return;
    }
    if mode == crate::roomfight::DefMode::Hold && g.time - s.t0 > 6.0 { sent_home(g, q, i); }
}

/// squadsTick 3, a man on a leg (`s.phase === 'run'`)
fn tick_run(g: &mut Game, q: EnemysquadsSquadId, i: usize, s: Sent, d: RoomfightDefId, mode: crate::roomfight::DefMode) {
    use crate::roomfight::DefMode;
    let gd = s.gd;
    let step = crate::pressure::boots(g, gd, s.step_t);
    g.tab.enemysquads_squad[q].sent[i].step_t = step;
    let at = pos_of(g, gd);
    if mode == DefMode::Move {
        if let Some(w) = &s.way {
            if w.len() > 1 && (flat(at, w[0]) < 1.4 || flat(at, w[1]) < flat(w[0], w[1])) {
                g.tab.enemysquads_squad[q].sent[i].way.as_mut().unwrap().remove(0);
                crate::pressure::run(g, d, w[1], s.door, Some(0.3));
            }
        }
        return;
    }
    let there = s.spot.map_or(false, |p| flat(at, p) < 2.0);
    let seen = g.time - g.gd[gd].last_seen_t < 1.0;
    let again = mode == DefMode::Hold && !there && !seen && s.tries < 3.0 && s.spot.is_some();
    if s.back == Some(true) {
        if again && fit(g, gd) {
            g.tab.enemysquads_squad[q].sent[i].tries = s.tries + 1.0;
            crate::pressure::run(g, d, s.spot.unwrap(), None, Some(0.3));
            return;
        }
        g.tab.enemysquads_squad[q].sent.remove(i);
        return;
    }
    if again && fit(g, gd) {
        g.tab.enemysquads_squad[q].sent[i].tries = s.tries + 1.0;
        let next = match s.way.as_ref().and_then(|w| w.first().copied()) { Some(p) => p, None => s.spot.unwrap() };
        crate::pressure::run(g, d, next, s.door, Some(0.3));
        return;
    }
    let holds = s.hold == Some(true);
    let dn = s.door.map(|x| door_name(g, x));
    let name = name_of(g, q);
    let sign = sign_of(g, gd);
    let since = fixed(g.time - s.t0, 1);
    if s.role == SentRole::Flank && mode == DefMode::Hold && (if holds { there || seen } else { seen && !there }) {
        let t = g.time;
        { let r = &mut g.tab.enemysquads_squad[q].sent[i]; r.phase = SentPhase::Set; r.set_t = Some(t); }
        if let (true, true, Some(door)) = (holds, there, s.door) {
            let room = g.tab.roomfight_def[d].room;
            let e = room.and_then(|r| g.tab.roomfight_room[r].entries.iter().position(|x| x.door == door));
            if let Some(e) = e {
                let eat = g.tab.roomfight_room[room.unwrap()].entries[e].at;
                let x = &mut g.tab.roomfight_def[d];
                x.ev[e] = m::max(x.ev[e], 1.5);
                x.want = e as f64; x.watch = e as f64;
                x.home = v3::copy(eat);
            } else if let Some(aim) = s.aim {
                let x = &mut g.tab.roomfight_def[d];
                x.want = -1.0; x.watch = -1.0;
                x.home = v3::copy(aim);
            }
            let x = &mut g.tab.roomfight_def[d];
            x.pt[0] = x.home[0]; x.pt[2] = x.home[2];
            let (pt, was_pt) = (x.pt, x.fix_is_pt);
            if !seen {
                // `gd.lastKnown = D.pt`: the same object again is no new fix (doze.ts:244), a first write of it is
                x.fix_is_pt = true;
                g.gd[gd].last_known = Some(pt);
                if !was_pt { g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1); }
            } else if was_pt {
                g.gd[gd].last_known = Some(pt);   // (his fix IS pt, and pt has just moved)
            }
        }
        if there && !seen {
            g.tab.enemysquads_squad[q].sent[i].there = Some(t);
            let got = crate::words::wg(g, &"we've got", &"we have");
            let text = format!("{}: in position — {} the door!", name, got);
            crate::game::game_say(g, gd, &text, Some(true), None, None);
            let why = format!("{}{}, {} s after the word", sign, dn.as_ref().map_or(String::new(), |n| format!(" holding {}", n)), since);
            event(g, q, EventWhat::InPosition, &why, Some(gd));
        } else {
            let why = format!("{} has them in sight {}{} and holds where he is, {} s after the word", sign, if there { "from his spot" } else { "on his way" }, dn.as_ref().map_or(String::new(), |n| format!("{}{}", if there { " at " } else { " to " }, n)), since);
            event(g, q, EventWhat::Met, &why, Some(gd));
        }
        return;
    }
    g.tab.enemysquads_squad[q].sent[i].phase = SentPhase::Loose;
    if mode != DefMode::Free { crate::pressure::loose(g, d, s.fix_at); }
    if there && !seen {
        g.tab.enemysquads_squad[q].sent[i].there = Some(g.time);
        let text = format!("{}: in position — on them!", name);
        crate::game::game_say(g, gd, &text, Some(true), None, None);
        let why = format!("{}{}, {} s after the word", sign, dn.as_ref().map_or(String::new(), |n| format!(" at {}", n)), since);
        event(g, q, EventWhat::InPosition, &why, Some(gd));
    } else if s.role == SentRole::Flank {
        let short = match s.spot { Some(p) => format!(" {} m short", fixed(flat(pos_of(g, gd), p), 0)), None => String::new() };
        let why = format!("{} {}{}{}, {} s after the word", sign, if seen { "has them in sight" } else { "is stopped" }, short, dn.as_ref().map_or(String::new(), |n| format!(" of {}", n)), since);
        event(g, q, EventWhat::Met, &why, Some(gd));
    }
}

/// `r3`: to three places; what is not a finite number is not given   (squadWaysRound)
fn r3(x: Option<f64>) -> Option<f64> {
    match x { Some(v) if v.is_finite() => Some(m::round(v * 1000.0) / 1000.0), _ => None }
}
/// the team heard through the walls anywhere in the room it has taken: a place every two metres of its floor, each a soft one as a noise is   (squadWaysRound's heard)
fn heard_in(g: &Game, heard_of: &mut JsMap<RoomfightRoomId, Vec<Known>>, t: RoomfightRoomId) -> Vec<Known> {
    if let Some(h) = heard_of.get(t) { return h.clone(); }
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let step = m::max(1.0, m::round(2.0 / cell));
    let mut h = Vec::new();
    for i in g.tab.roomfight_room[t].cells.iter() {
        let i = i.0;
        let ix = js::rem(i, w);
        let iz = (i - ix) / w;
        if js::rem(ix, step) == 0.0 && js::rem(iz, step) == 0.0 { h.push(Known { p: [(ix + 0.5) * cell, 0.0, (iz + 0.5) * cell], firm: false }); }
    }
    heard_of.set(t, h.clone());
    h
}

/// a static ask for a check: for every man at home of every squad that may leave its rooms, every other squad's room and door, would he find a way round. His position is moved and put back; so is the search's memo   (enemysquads.ts squadWaysRound)
pub fn squad_ways_round(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadWaysRoundItem> {
    let mut out = Vec::new();
    let squads = match g.m.enemysquads.states.as_ref() { Some(st) => st.squads.clone(), None => return out };
    let keep = g.m.enemysquads.ctx.take();
    let mut heard_of: JsMap<RoomfightRoomId, Vec<Known>> = JsMap::new();
    for &q in squads.iter() {
        if g.tab.enemysquads_squad[q].def.may_leave == Some(false) { continue; }
        for d in home(g, q) {
            let gd = def_gd(g, d);
            let c = g.gd[gd].ch;
            let post = g.ch[c].pos;
            let mut from: Vec<(String, V3)> = vec![(String::from("his post"), post)];
            let entries = match g.tab.roomfight_def[d].room { Some(r) => g.tab.roomfight_room[r].entries.clone(), None => Vec::new() };
            for en in entries.iter() {
                let stride: V3 = [en.at[0] - en.out[0], 0.0, en.at[2] - en.out[1]];
                if !crate::scene_collision::nav_grid_is_blocked(g, stride[0], stride[2]) { from.push((door_name(g, en.door), stride)); }
            }
            for (from_name, p) in from.iter() {
                g.ch[c].pos[0] = p[0];
                g.ch[c].pos[2] = p[2];
                for &x in squads.iter() {
                    if x == q { continue; }
                    let rooms = g.tab.enemysquads_squad[x].rooms.clone();
                    for room in rooms.iter() {
                        let at: V3 = [(room.bounds.x0 + room.bounds.x1) / 2.0, 0.0, (room.bounds.z0 + room.bounds.z1) / 2.0];
                        let t = crate::roomfight::room_fight_room_at(g, at);
                        let doors = match t { Some(t) => g.tab.roomfight_room[t].entries.clone(), None => Vec::new() };
                        for e in doors.iter() {
                            for team in [false, true] {
                                for hold in [false, true] {
                                    let mut says = WayRound2Says { limit: None, best: None };
                                    let known: Vec<Known> = if team {
                                        let mut k = vec![
                                            Known { p: [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]], firm: true },
                                            Known { p: [e.at[0] - e.out[0] * 3.0, 0.0, e.at[2] - e.out[1] * 3.0], firm: true },
                                        ];
                                        k.extend(heard_in(g, &mut heard_of, t.unwrap()));
                                        k
                                    } else { Vec::new() };
                                    g.m.enemysquads.ctx = None;   // (the memo's key counts the known places, it does not tell them apart)
                                    let back = if hold { g.k.squads.hold_back_m } else { 0.0 };
                                    let w = way_round2(g, d, at, &known, &[e.door], &[], &[], Some(back), Some(&mut says));
                                    let way = w.as_ref().map(|w| match w.door { Some(x) => door_name(g, x), None => String::from("open floor") });
                                    // (`r3(w.L)!` of a way with no finite length has no number in the TypeScript: NaN here)
                                    let l = match &w { Some(w) => r3(Some(w.l)).unwrap_or(f64::NAN), None => 0.0 };
                                    out.push(SquadWaysRoundItem { squad: name_of(g, q), man: sign_of(g, gd), from: from_name.clone(), room: room.name.clone(), door: door_name(g, e.door), hold, team, way, l, limit: r3(says.limit), best: r3(says.best) });
                                }
                            }
                        }
                    }
                }
            }
            g.ch[c].pos[0] = post[0];
            g.ch[c].pos[2] = post[2];
        }
    }
    g.m.enemysquads.ctx = keep;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the language's own answers (bun: `x.toFixed(d)`), ties and near-ties
    #[test]
    fn fixed_rounds_as_to_fixed_does() {
        for &(x, d, want) in &[(0.25, 1, "0.3"), (0.35, 1, "0.3"), (2.5, 0, "3"), (0.5, 0, "1"), (1.45, 1, "1.4"), (-0.25, 1, "-0.3"), (12.0, 0, "12"), (3.14159, 1, "3.1"), (1.0, 1, "1.0")] {
            assert_eq!(fixed(x, d), want);
        }
    }
    #[test]
    fn a_count_past_the_words_is_its_number() {
        assert_eq!(count_or_n(2), "two");
        assert_eq!(count_or_n(7), "7");
    }
}
