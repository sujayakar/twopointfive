use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the squad's row   (Q)
macro_rules! sq {
    ($g:expr, $q:expr) => {
        $g.tab.enemysquads_squad[$q]
    };
}
/// the squad's check, which is there   (Q.rc!)
macro_rules! rc {
    ($g:expr, $q:expr) => {
        $g.tab.enemysquads_squad[$q].rc.as_mut().unwrap()
    };
}
/// a man's post in the room fight   (D)
macro_rules! def {
    ($g:expr, $d:expr) => {
        $g.tab.roomfight_def[$d]
    };
}
/// the module's row of the game   (st, STATES.get(g))
macro_rules! st {
    ($g:expr) => {
        $g.m.enemysquads.states.as_mut().unwrap()
    };
}
/// the word of the team, which is there   (st.word)
macro_rules! wd {
    ($g:expr) => {
        $g.m.enemysquads.states.as_mut().unwrap().word.as_mut().unwrap()
    };
}

/// `COUNT[n] ?? n`
pub(super) fn count_word(n: usize) -> String {
    match count().get(n) {
        Some(w) => w.clone(),
        None => format!("{}", n),
    }
}
/// the row of Q.sent of the man `s`: a Sent is named by its man (home() keeps a man off a second row)
fn sent_of(g: &mut Game, q: EnemysquadsSquadId, s: GuardId) -> &mut crate::enemysquads::Sent {
    sq!(g, q).sent.iter_mut().find(|x| x.gd == s).unwrap()
}
/// drillFollow's and drillBack's ears: a holding man hears the team go on somewhere else, and nobody of the pair (his mates on that door within `reach`) has one of them in sight.
fn moved_on(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId, reach: f64) -> Option<crate::enemysquads::MovedOn> {
    let gd = s;
    let at = def!(g, d).noise_at?;
    let (aim, since) = {
        let row = sent_of(g, q, s);
        (row.aim, row.set_t.unwrap_or(row.t0))
    };
    if g.time - def!(g, d).noise_t > 1.0 || g.time - since < 4.0 || g.time - g.gd[gd].last_seen_t < 3.0 || !fit(g, gd) {
        return None;
    }
    let from = aim.unwrap_or(g.pos_of(gd));
    let follow_m = match squad_drill(g) {
        Some(k) => k.follow_m,
        None => g.k.squads.follow_m,
    };
    if dist_xz(at, from) < follow_m || lost_room_has(g, s, at, q) {
        return None;
    }
    let rows = sq!(g, q).sent.clone();
    let here = g.pos_of(gd);
    let mut pair: Vec<GuardId> = Vec::new();
    for x in rows.iter() {
        if x.role == SentRole::Flank
            && x.phase == SentPhase::Set
            && x.hold == Some(true)
            && alive(g, x.gd)
            && fit(g, x.gd)
            && (x.gd == s || dist_xz(g.pos_of(x.gd), here) < reach)
        {
            pair.push(x.gd);
        }
    }
    for x in pair.iter() {
        if g.time - g.gd[*x].last_seen_t < 3.0 {
            return None;
        }
    }
    Some(crate::enemysquads::MovedOn { at, pair })
}
/// A drilled garrison's pair holding a lost room's other door goes back to its own posts. `s` is his man: the row stays in Q.sent.
pub(super) fn back(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) {
    let gd = s;
    let k = match squad_drill(g) {
        Some(k) => k,
        None => g.k.squads,
    };
    let ally = sent_of(g, q, s).ally;
    let gone = match ally {
        Some(a) => sq!(g, a).state == SquadState::Down,
        None => false,
    };
    let at: V3;
    let pair: Vec<GuardId>;
    if gone {
        let now = g.time;
        let (gone_t, since) = {
            let row = sent_of(g, q, s);
            (*row.gone_t.get_or_insert(now), row.set_t.unwrap_or(row.t0))
        };
        if g.time - gone_t < k.back_beat || g.time - since < 4.0 || !fit(g, gd) {
            return;
        }
        let rows = sq!(g, q).sent.clone();
        let mut men = Vec::new();
        for x in rows.iter() {
            if x.role == SentRole::Flank
                && x.back != Some(true)
                && x.hold == Some(true)
                && (x.phase == SentPhase::Set || x.phase == SentPhase::Run)
                && alive(g, x.gd)
                && fit(g, x.gd)
            {
                men.push(x.gd);
            }
        }
        for x in men.iter() {
            if g.time - g.gd[*x].last_seen_t < 3.0 {
                return;
            }
        }
        let fix_at = sent_of(g, q, s).fix_at;
        at = def!(g, d).noise_at.unwrap_or(fix_at);
        pair = men;
    } else {
        let m = match moved_on(g, q, s, d, f64::INFINITY) {
            Some(m) => m,
            None => return,
        };
        at = m.at;
        pair = m.pair;
    }
    let mut names: Vec<GuardId> = Vec::new();
    for x in pair.iter() {
        let x = *x;
        let xd = match def_of(g, x) {
            Some(xd) => xd,
            None => continue,
        };
        if crate::roomfight::is_loose(g, xd) {
            continue;
        }
        let from = {
            let row = sent_of(g, q, x);
            row.phase = SentPhase::Run;
            row.back = Some(true);
            row.hold = Some(false);
            row.spot = Some(row.from);
            row.door = None;
            row.way.clear();
            row.from
        };
        for e in def!(g, xd).ev.iter_mut() {
            *e = 0.0;
        }
        // (`X.pt[0] = at[0]; X.pt[2] = at[2]` in place, enemysquads.ts:778: a fix or a cover that IS the point moves with it)
        { let was = def!(g, xd).pt; crate::roomfight::def_move_pt(g, xd, [at[0], was[1], at[2]]); }
        send_round(g, xd, from, None, Vec::new());
        names.push(x);
    }
    if names.is_empty() {
        return;
    }
    let q_name = sq!(g, q).name.clone();
    let line = if gone {
        let ally = sq!(g, ally.unwrap()).name.clone();
        format!("{}: {} isn't answering — back on our doors!", q_name, ally)
    } else {
        format!("{}: they've left it — back on our doors!", q_name)
    };
    crate::game::game_say(g, gd, &line, Some(true), None);
    event(g, q, SquadEventWhat::Back, Some(gd));
}

/// A drilled garrison goes and looks: once a tick the word of the team is kept, and gone quiet a pair is called and sent.
pub(super) fn look_out(g: &mut crate::game::Game) {
    let Some(k) = squad_drill(g).filter(|k| k.drill_look && g.k.pressure.sweep) else { return };
    hear_of_them(g);
    let squads = st!(g).squads.clone();
    pair_back(g, &squads);
    if wd!(g).lost { return; }
    let busy = squads.iter().any(|&q| has_its_hands_full(g, q));
    if let Some(c) = wd!(g).call.clone() { return pair_off(g, c, busy); }
    let (t, at, done, said) = {
        let w = wd!(g);
        (w.t, w.at, w.done, w.said_t)
    };
    if t == f64::NEG_INFINITY || done >= t || g.time - t < k.sweep_secs || g.time - said < k.sweep_secs || busy || somebody_sees_to(g, at) { return; }
    let Some((q, go)) = who_goes(g, &k, &squads, at) else { return };
    let now = g.time;
    let w = wd!(g);
    w.done = t;
    w.said_t = now;
    w.call = Some(StateWordCall { q, go: go.clone(), at, go_t: now + k.sweep_beat, w: t });
    let line = format!("{}: gone quiet — {} of us going to look!", sq!(g, q).name, count_word(go.len()));
    let first = def!(g, go[0]).gd;
    crate::game::game_say(g, first, &line, Some(true), None);
}
/// THE WORD OF THE TEAM: where and when anybody of theirs last said, saw or heard something of it.
fn hear_of_them(g: &mut Game) {
    let mut w = st!(g).word.take().unwrap_or_else(|| StateWord { t: f64::NEG_INFINITY, at: [0.0, 0.0, 0.0], done: f64::NEG_INFINITY, said_t: f64::NEG_INFINITY, lost: false, out: None, call: None });
    let mut heard = |t: f64, at: V3| if t > w.t { w.t = t; w.at = at; };
    if let Some(c) = st!(g).net.last() { heard(c.t, c.at); }
    for &d in &g.fight.defs {
        let r = &def!(g, d);
        if !alive(g, r.gd) { continue; }
        let man = &g.gd[r.gd];
        if man.held.is_none() && man.last_seen_t >= 0.0 { heard(man.last_seen_t, man.sight_pos); }
        if let Some(n) = r.noise_at { heard(r.noise_t, n); }
    }
    st!(g).word = Some(w);
}
/// The pair that went is back, or one of it is down: then nobody is sent again.
fn pair_back(g: &mut Game, squads: &[EnemysquadsSquadId]) {
    let Some(out) = wd!(g).out.clone() else { return };
    if out.iter().any(|&x| !alive(g, x)) {
        let w = wd!(g);
        w.lost = true;
        w.out = None;
    } else if !squads.iter().any(|&q| sq!(g, q).sent.iter().any(|s| s.role == SentRole::Look)) {
        wd!(g).out = None;
    }
}
/// In a fight, about to be, on a call, or with men out.
fn has_its_hands_full(g: &Game, q: EnemysquadsSquadId) -> bool {
    let r = &sq!(g, q);
    r.state == SquadState::Contact || r.state == SquadState::Alerted || !r.due.is_empty()
        || r.calls.as_ref().map_or(false, |c| !c.is_empty())
        || r.rc.as_ref().map_or(false, |c| !c.look.is_empty())
        || r.sent.iter().any(|s| alive(g, s.gd))
}
/// The pair that was called sets off when its moment has come; not if something has been heard since, or one of it is no longer at home.
fn pair_off(g: &mut Game, c: StateWordCall, busy: bool) {
    let h = home(g, c.q);
    let word_t = wd!(g).t;
    if word_t > c.w || busy || !c.go.iter().all(|x| h.contains(x)) {
        let w = wd!(g);
        w.call = None;
        if word_t <= c.w { w.done = f64::NEG_INFINITY; }
        return;
    }
    if g.time < c.go_t { return; }
    let men: Vec<GuardId> = c.go.iter().map(|x| def!(g, *x).gd).collect();
    let w = wd!(g);
    w.call = None;
    w.out = Some(men.clone());
    for &d in &c.go {
        let (gd, from) = (def!(g, d).gd, def!(g, d).station);
        let row = Sent { gd, from, role: SentRole::Look, phase: SentPhase::Loose, fix_at: c.at, t0: g.time, ..Sent::default() };
        sq!(g, c.q).sent.push(row);
        crate::roomfight::loose_at(g, d, c.at);
    }
    event(g, c.q, SquadEventWhat::Sweep, Some(men[0]));
}
/// A man of theirs is on the hunt already, or holds that very room.
fn somebody_sees_to(g: &mut Game, at: V3) -> bool {
    let up: Vec<RoomfightDefId> = g.fight.defs.iter().copied().filter(|&d| alive(g, def!(g, d).gd)).collect();
    if up.iter().any(|&d| crate::roomfight::is_loose(g, d)) { return true; }
    let room = crate::roomfight::room_fight_room_at(g, at);
    room.is_some() && up.iter().any(|&d| crate::roomfight::at_post(g, d) && def!(g, d).room == room)
}
/// The men of the squad nearest to that place that has two to spare from its rooms.
fn who_goes(g: &Game, k: &crate::knobs::SquadsK, squads: &[EnemysquadsSquadId], at: V3) -> Option<(EnemysquadsSquadId, Vec<RoomfightDefId>)> {
    let mut best: Option<(EnemysquadsSquadId, Vec<RoomfightDefId>, f64)> = None;
    for &q in squads {
        if sq!(g, q).state == SquadState::Down || (sq!(g, q).def.may_leave == Some(false) && !k.sweep_stayers) { continue; }
        let h = home(g, q);
        let spare = h.len() as f64 - rooms_held(g, &h) as f64;
        if spare < 2.0 { continue; }
        let mut free = Vec::new();
        for &d in &h { if may_look(g, d, at) { free.push(d); } }
        let go = goers(g, &free, at, m::min(m::max(2.0, k.sweep_men), spare));
        if go.len() < 2 { continue; }
        let far: Vec<f64> = go.iter().map(|&d| dist_xz(g.pos_of(def!(g, d).gd), at)).collect();
        let dist = m::min_n(&far);
        if best.as_ref().map_or(true, |b| dist < b.2) { best = Some((q, go, dist)); }
    }
    best.map(|b| (b.0, b.1))
}
/// not one who knows a covered lane lies that way
fn may_look(g: &Game, d: RoomfightDefId, at: V3) -> bool {
    let gd = def!(g, d).gd;
    !crate::lanes::lane_bars(g, gd, Some(at)) && !crate::lanes::lane_known_at(g, gd, Some(at))
}

/// Is that place in, or within three metres of, the room a holding man's door leads to.
fn lost_room_has(g: &mut crate::game::Game, s: GuardId, at: V3, q: EnemysquadsSquadId) -> bool {
    let door = match sent_of(g, q, s).door {
        Some(d) => d,
        None => return false,
    };
    let here = crate::roomfight::room_fight_room_at(g, g.pos_of(s));
    let mut p: V3 = [0.0, 0.0, 0.0];
    for r in [0.0, 1.5, 3.0] {
        let n = if js::truthy(r) { 8 } else { 1 };
        for k in 0..n {
            let a = k as f64 * core::f64::consts::PI / 4.0;
            p[0] = at[0] + m::cos(a) * r;
            p[2] = at[2] + m::sin(a) * r;
            let there = crate::roomfight::room_fight_room_at(g, p);
            if let Some(t) = there {
                if there != here && g.tab.roomfight_room[t].entries.iter().any(|e| e.door == door) {
                    return true;
                }
            }
        }
    }
    false
}

/// the word a check's die starts from: what checkDice works out, in integers
fn check_word(name: &str, salt: f64, phases: f64) -> u32 {
    let mut h: i32 = (0x811c9dc5u32 as i32) ^ js::to_i32(salt);
    for c in name.encode_utf16() {
        h = m::imul(h ^ (c as i32), 0x01000193);
    }
    (h ^ js::to_i32(m::floor(phases * 8388607.0))) as u32
}
/// The check's own dice, never the game's: seeded off the squad's name and its men's stride phases.
pub(super) fn check_dice(g: &crate::game::Game, q: EnemysquadsSquadId, salt: Option<f64>) -> crate::dice::Die {
    let salt = salt.unwrap_or(0.0);
    let mut phases = 0.0;
    for x in sq!(g, q).men.iter() {
        phases = phases + g.ch[g.gd[*x].ch].anim.phase;
    }
    let word = check_word(&sq!(g, q).name, salt, phases);
    crate::dice::Die(word as u32)
}
/// How long till the next check: a squad that has heard of the team counts its men oftener.
fn check_beat(g: &crate::game::Game, q: EnemysquadsSquadId, die: &mut crate::dice::Die) -> f64 {
    let k = g.k.squads;
    let alert = if uneasy(g, q) { k.check_alert_k } else { 1.0 };
    k.check_every * alert * (1.0 + (die.roll() * 2.0 - 1.0) * k.check_jitter)
}
/// checkBeat off the squad's own check's die: the die lives in g, so it comes out for the call and goes back with the draw
fn rc_beat(g: &mut Game, q: EnemysquadsSquadId) -> f64 {
    let mut die = rc!(g, q).die;
    let beat = check_beat(g, q, &mut die);
    rc!(g, q).die = die;
    beat
}
/// one draw of the squad's own check's die
fn rc_roll(g: &mut Game, q: EnemysquadsSquadId) -> f64 {
    rc!(g, q).die.roll()
}
/// The squad's check, made on the first asking: the row itself.
pub(super) fn rc_of<'g>(g: &'g mut crate::game::Game, q: EnemysquadsSquadId) -> &'g mut crate::enemysquads::Check {
    if sq!(g, q).rc.is_none() {
        let mut die = check_dice(g, q, None);
        let first = die.roll();
        let next_at = g.time + first * check_beat(g, q, &mut die);
        let now = g.time;
        sq!(g, q).rc = Some(crate::enemysquads::Check {
            next_at,
            stage: CheckStage::Idle,
            t0: 0.0,
            caller: None,
            who: None,
            silent: Vec::new(),
            known: JsSet::new(),
            looked: JsSet::new(),
            look: Vec::new(),
            n: 0.0,
            die,
            cut: None,
            beat: None,
            late: None,
            promised: None,
            afield: None,
            out: None,
            asked: None,
            net_out: None,
            by_ear: false,
            tick_t: now,
        });
    }
    rc!(g, q)
}
/// what each man's ears had counted by now
fn small0_of(g: &Game, q: EnemysquadsSquadId) -> JsMap<GuardId, f64> {
    let mut out = JsMap::new();
    for x in sq!(g, q).men.iter() {
        out.set(*x, g.gd[*x].witness.heard.small);
    }
    out
}
/// A man of the squad has gone quiet: where he stood, when, and what each mate's ears had counted by then. Nobody is told by it.
pub(super) fn went_silent(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: Option<V3>) {
    let at = match at {
        Some(a) => a,
        None => g.pos_of(m),
    };
    rc_of(g, q);
    let was = rc!(g, q).silent.iter().position(|r| r.gd == m);
    if let Some(ix) = was {
        let held = rc!(g, q).silent[ix].held;
        if held && !alive(g, m) {
            let small0 = small0_of(g, q);
            let now = g.time;
            let was_at = [at[0], 0.0, at[2]];
            {
                let r = &mut rc!(g, q).silent[ix];
                r.held = false;
                r.at = was_at;
                r.t = now;
                r.heard = false;
                r.small0 = small0;
            }
            rc!(g, q).looked.delete(m);
            let mut i = 0;
            while i < rc!(g, q).look.len() {
                let (who, back, man) = {
                    let x = &rc!(g, q).look[i];
                    (x.who, x.back, x.gd)
                };
                if who == m && !back {
                    {
                        let x = &mut rc!(g, q).look[i];
                        x.spot = was_at;
                        x.look_t = None;
                    }
                    if let Some(d) = def_of(g, man) {
                        if crate::roomfight::on_leg_of(g, d, crate::owner::by::SQUAD_LEG) {
                            walk(g, d, was_at);
                        }
                    }
                }
                i += 1;
            }
        }
        return;
    }
    let up = alive(g, m);
    let small0 = small0_of(g, q);
    let now = g.time;
    rc!(g, q).silent.push(crate::enemysquads::Quiet { gd: m, at: [at[0], 0.0, at[2]], t: now, heard: false, held: up, small0 });
}
/// A line of the check on the squad's net: the debrief's row and the strip's, never his bubble.
pub(super) fn on_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) {
    let cls = match cls {
        Some(c) => c,
        None => crate::cues::cue_class(text),
    };
    let chron = chron.unwrap_or(true);
    if g.gd[gd].held.is_some() {
        return;
    }
    crate::cues::cues_net(g, gd, text, cls);
    if chron {
        let _ = crate::chronicle::chron_radio(g, gd, text);
    }
}
/// A mate at home has gunfire in his ears with a drop.
pub(super) fn heard_fire(g: &mut crate::game::Game, q: EnemysquadsSquadId, up: &[GuardId]) -> bool {
    for x in up.iter() {
        let x = *x;
        let at = g.pos_of(x);
        if room_of(g, q, at, Some(1.0)).is_none() {
            continue;
        }
        let last = g.gd[x].witness.heard.last_shot_t;
        if (last >= 0.0 && g.time - last < 1.5) || g.time - bang_t(g, x) < 1.5 || crate::guards::gunfire_due(g, x, Some(true)) {
            return true;
        }
    }
    false
}
/// When a stun charge, or a door blown in, last reached this man's ears: his Witness counts them and this times them.
fn bang_t(g: &mut crate::game::Game, x: GuardId) -> f64 {
    let (bang, last_t) = {
        let h = &g.gd[x].witness.heard;
        (h.bang, h.last_t)
    };
    let now = g.time;
    let stale = match g.m.enemysquads.bangs.get(x) {
        Some(b) => now - b.read > 0.5 || now < b.read || bang < b.n,
        None => true,
    };
    if stale {
        g.m.enemysquads.bangs.set(x, EnemysquadsModBANGSValue { n: bang, t: -100.0, read: now });
        return -100.0;
    }
    let b = g.m.enemysquads.bangs.get_mut(x).unwrap();
    if bang > b.n {
        b.n = bang;
        b.t = last_t;
    }
    b.read = now;
    b.t
}
/// A looker's leg: pressure.ts's scripted leg, at the pace of a man going to look.
pub(super) fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    let pace = g.tune.guard_investigate;
    crate::roomfight::send(g, d, crate::roomfight::Leg { stays: true, ..crate::roomfight::Leg::errand(to, pace) }, crate::owner::by::SQUAD_LEG);
}

/// Is anything said or heard of the team that has the squad count its men oftener.
fn uneasy(g: &Game, q: EnemysquadsSquadId) -> bool {
    let r = &sq!(g, q);
    r.news_t > -100.0 || r.contact_t > -100.0 || r.wary == Some(true)
}
/// the man of longest silence nobody knows of or has looked for
fn missing(g: &mut Game, q: EnemysquadsSquadId) -> Option<GuardId> {
    let c = rc!(g, q);
    let mut best = None;
    let mut bt = f64::INFINITY;
    for r in c.silent.iter() {
        if !c.known.has(r.gd) && !c.looked.has(r.gd) && r.t < bt {
            bt = r.t;
            best = Some(r.gd);
        }
    }
    best
}
/// nobody is being asked for, and no check is on the air
fn off_air(c: &mut Check) {
    c.stage = CheckStage::Idle;
    c.who = None;
    c.by_ear = false;
    c.late = Some(false);
}
/// the same, and whoever was owed an asking is owed none
fn dropped(c: &mut Check) {
    off_air(c);
    c.cut = None;
    c.beat = None;
}
/// The check is over, the next one `after` from now; with `poll`, the routine check keeps its time. A man still owed an asking brings it forward.
fn idle(g: &mut Game, q: EnemysquadsSquadId, after: f64, poll: bool) {
    let now = g.time;
    let c = rc!(g, q);
    off_air(c);
    c.next_at = match (poll, c.beat) {
        (true, Some(b)) => m::max(b, now + after),
        _ => now + after,
    };
    c.beat = None;
    let Some(cut) = rc!(g, q).cut.take() else { return };
    let kept: Vec<CheckCutItem> = cut.into_iter().filter(|x| owed_of(g, x.who, q)).collect();
    if kept.is_empty() { return; }
    let first = m::min_n(&kept.iter().map(|x| x.at).collect::<Vec<f64>>());
    let c = rc!(g, q);
    c.beat = Some(c.next_at);
    c.next_at = m::min(c.next_at, m::max(now + m::min(after, 1.0), first));
    c.cut = Some(kept);
}
/// The check is over, the next one on the squad's own beat.
fn idle_a_beat(g: &mut Game, q: EnemysquadsSquadId) {
    let beat = rc_beat(g, q);
    idle(g, q, beat, false);
}
/// The next check in five seconds at the soonest: nobody can be asked for now.
fn idle_for_now(g: &mut Game, q: EnemysquadsSquadId) { idle(g, q, NOT_NOW, true); }
const NOT_NOW: f64 = 5.0;

/// Once a frame for a squad with the switch on: what its men's eyes and ears make of the men who went quiet, and the net's check. Some: where a man of theirs was found to be down.
pub(super) fn radio_check(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::RadioCheck> {
    begin(g, q);
    let men = sq!(g, q).men.clone();
    for &x in &men { if alive(g, x) { bang_t(g, x); } }
    let seen = drops(g, q, &men, busy);
    if busy {
        // (in a fight whoever has dropped is known to have)
        let quiet: Vec<GuardId> = rc!(g, q).silent.iter().map(|r| r.gd).collect();
        for x in quiet { if !alive(g, x) { rc!(g, q).known.add(x); } }
    }
    if busy || seen.is_some() {
        let now = g.time;
        let c = rc!(g, q);
        if c.promised.as_ref().is_some_and(|p| !p.is_empty()) { c.promised = None; }
        dropped(c);
        c.next_at = m::max(c.next_at, now + NOT_NOW);
        return if busy { None } else { seen };
    }
    if on_the_air(g, q, &men) {
        match rc!(g, q).stage {
            CheckStage::Idle => call(g, q, &men),
            CheckStage::Called => ask(g, q, &men),
            CheckStage::Asked => send_to_look(g, q),
        }
    }
    None
}

/// The check is made on first touch, and begun again when it was not run for a second.
fn begin(g: &mut Game, q: EnemysquadsSquadId) {
    let now = g.time;
    let c = rc_of(g, q);
    c.n += 1.0;
    let left = now - c.tick_t > 1.0;
    c.tick_t = now;
    if !left { return; }
    dropped(c);
    let beat = rc_beat(g, q);
    rc!(g, q).next_at = now + beat;
}

/// WHAT THEY MAKE OF THE MEN GONE QUIET, the last first. One answering again is off the list. A body in a mate's sight, or gunfire a mate at home heard as he dropped, and he is known to be down
/// (Some); a small thing heard since, where he stood, and that mate calls him.
fn drops(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId], busy: bool) -> Option<RadioCheck> {
    let mut seen = None;
    for i in (0..rc!(g, q).silent.len()).rev() {
        let (who, at, age) = {
            let r = &rc!(g, q).silent[i];
            (r.gd, r.at, g.time - r.t)
        };
        if !crate::guards::off_net(g, who) {
            let c = rc!(g, q);
            c.silent.remove(i);
            c.looked.delete(who);
            continue;
        }
        if rc!(g, q).known.has(who) || alive(g, who) { continue; }
        let looked_for = age < 1.0 || js::rem(rc!(g, q).n + i as f64, 6.0) == 0.0;
        let eyes = if looked_for { men.iter().copied().find(|&x| x != who && fit(g, x) && crate::guards::body_seen(g, x, who, None)) } else { None };
        if let Some(by) = eyes {
            let at = crate::guards::body_pos(g, who);
            let room = room_of(g, q, at, Some(1.0));
            if !busy && key_up(g, q, by, at, room.as_ref(), Some(who)) { continue; }
            seen = Some(found_down(g, q, who, by, at));
        } else if let Some(by) = heard_him_shot(g, q, men, i) {
            seen = Some(found_down(g, q, who, by, at));
        } else if let Some(by) = heard_a_thing(g, q, men, i) {
            rc!(g, q).silent[i].heard = true;
            called_by_ear(g, q, who, by, busy);
        }
    }
    seen
}
/// `who` is known to be down, by that man
fn found_down(g: &mut Game, q: EnemysquadsSquadId, who: GuardId, by: GuardId, at: V3) -> RadioCheck {
    rc!(g, q).known.add(who);
    event(g, q, SquadEventWhat::Found, Some(by));
    RadioCheck { at, room: room_of(g, q, at, Some(1.0)) }
}
/// A mate at home who had gunfire (or a charge going off) in his ears in the second after that man dropped.
fn heard_him_shot(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId], i: usize) -> Option<GuardId> {
    let (who, t, at) = {
        let r = &rc!(g, q).silent[i];
        (r.gd, r.t, r.at)
    };
    if g.time - t >= 1.5 || (cuts(g) && room_of(g, q, at, Some(1.0)).is_none()) { return None; }
    men.iter().copied().find(|&x| {
        x != who && fit(g, x) && (g.gd[x].witness.heard.last_shot_t >= t - 0.2 || bang_t(g, x) >= t - 0.2) && room_of(g, q, g.pos_of(x), Some(1.0)).is_some()
    })
}
/// A mate who has heard a small thing since that man dropped, where he stood. (Once, in the three seconds after)
fn heard_a_thing(g: &Game, q: EnemysquadsSquadId, men: &[GuardId], i: usize) -> Option<GuardId> {
    let r = &sq!(g, q).rc.as_ref()?.silent[i];
    if r.heard || g.time - r.t > 3.0 { return None; }
    men.iter().copied().find(|&x| {
        let e = &g.gd[x].witness.heard;
        x != r.gd && fit(g, x) && e.small > r.small0.get(x).copied().unwrap_or(f64::INFINITY) && e.small_where.is_some_and(|w| dist_xz(w, r.at) < g.k.squads.check_hear_m)
    })
}
/// The man who heard it calls him by name a moment later, unless something else is on.
fn called_by_ear(g: &mut Game, q: EnemysquadsSquadId, who: GuardId, by: GuardId, busy: bool) {
    let alerted = sq!(g, q).state == SquadState::Alerted;
    let c = rc!(g, q);
    if c.stage != CheckStage::Idle || busy || !c.look.is_empty() || alerted { return; }
    let k = g.k.squads;
    let t0 = g.time + k.check_heard[0] + rc_roll(g, q) * (k.check_heard[1] - k.check_heard[0]) - k.check_ask;
    let c = rc!(g, q);
    c.stage = CheckStage::Called;
    c.by_ear = true;
    c.caller = Some(by);
    c.who = Some(who);
    c.t0 = t0;
    event(g, q, SquadEventWhat::Check, Some(by));
}

/// Calls on the net have a length, and can be cut off.
fn cuts(g: &Game) -> bool { g.k.squads.call_cut_ask[1] > 0.0 }

/// Is there a check to go on with now: its time has come, nobody of the squad is on a call, and it has not heard of the team meanwhile.
fn on_the_air(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId]) -> bool {
    let k = g.k.squads;
    let now = g.time;
    if rc!(g, q).stage == CheckStage::Idle {
        let soon = uneasy(g, q);
        let c = rc!(g, q);
        if soon { c.next_at = m::min(c.next_at, now + k.check_every * k.check_alert_k * (1.0 + k.check_jitter)); }
        if now < c.next_at { return false; }
    }
    if sq!(g, q).calls.as_ref().map_or(false, |c| !c.is_empty()) {
        let c = rc!(g, q);
        if c.stage == CheckStage::Called && now - c.t0 >= k.check_ask { c.late = Some(true); }
        return false;
    }
    if rc!(g, q).cut.is_some() && !cuts(g) {
        let c = rc!(g, q);
        c.cut = None;
        c.promised = None;
        if let Some(b) = c.beat.take() { c.next_at = m::max(c.next_at, b); }
    }
    // a man in somebody's arm does not answer either
    for &x in men {
        if alive(g, x) && g.gd[x].held.is_some() && !rc!(g, q).looked.has(x) && !sq!(g, q).sent.iter().any(|s| s.gd == x) {
            let station = def_of(g, x).map(|d| def!(g, d).station);
            went_silent(g, q, x, station);
        }
    }
    // the squad has heard of the team since the check was called and its word is due: nobody is asked for over that
    if rc!(g, q).stage != CheckStage::Idle && sq!(g, q).state == SquadState::Alerted {
        let c = rc!(g, q);
        if let (CheckStage::Called, Some(who), false, true) = (c.stage, c.who, c.by_ear, cuts(g)) {
            rc!(g, q).cut.get_or_insert_with(Vec::new).insert(0, CheckCutItem { who, at: now });
        }
        idle_for_now(g, q);
        return false;
    }
    true
}

/// THE CHECK IS CALLED: first the man owed an asking, by the first man at home; else everybody, if there is anybody to miss or two to answer.
fn call(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId]) {
    let caller = home_first(g, q);
    let Some(caller) = caller.filter(|_| sq!(g, q).state != SquadState::Alerted && rc!(g, q).look.is_empty()) else { return idle_for_now(g, q) };
    let now = g.time;
    if rc!(g, q).cut.is_some() || rc!(g, q).beat.is_some() {
        let mut owed: Vec<CheckCutItem> = rc!(g, q).cut.take().unwrap_or_default().into_iter().filter(|x| owed_of(g, x.who, q)).collect();
        let beat = rc!(g, q).beat.take();
        if owed.is_empty() {
            if let Some(b) = beat.filter(|&b| now < b) { rc!(g, q).next_at = b; return; }
        } else {
            let x = owed.remove(0);
            let ask_t = g.k.squads.check_ask;
            let c = rc!(g, q);
            c.cut = if owed.is_empty() { None } else { Some(owed) };
            c.stage = CheckStage::Called;
            c.by_ear = false;
            c.caller = Some(caller);
            c.who = Some(x.who);
            c.t0 = m::max(now, x.at) - ask_t;
            event(g, q, SquadEventWhat::Check, Some(caller));
            return;
        }
    }
    if missing(g, q).is_none() && men.iter().filter(|&&x| !crate::guards::off_net(g, x)).count() < 2 { return idle_a_beat(g, q); }
    let c = rc!(g, q);
    c.stage = CheckStage::Called;
    c.t0 = now;
    c.caller = Some(caller);
    c.who = None;
    let line = format!("{}: radio check — sound off", sq!(g, q).name);
    on_net(g, caller, &line, Some(6.0), None);
    event(g, q, SquadEventWhat::Check, Some(caller));
}

/// A MAN IS ASKED FOR BY NAME: whoever it was called for, or the man of longest silence. By whoever heard something, or whoever called, or the first at home.
fn ask(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId]) {
    let k = g.k.squads;
    let now = g.time;
    let (by_ear, caller, named, t0) = {
        let c = rc!(g, q);
        (c.by_ear, c.caller, c.who, c.t0)
    };
    if by_ear && caller.map_or(true, |c| crate::guards::off_net(g, c)) {
        // (he who heard it has gone quiet himself: a mate may yet hear it)
        let c = rc!(g, q);
        if let Some(r) = named.and_then(|w| c.silent.iter_mut().find(|x| x.gd == w)) { r.heard = false; }
        off_air(c);
        return;
    }
    if now - t0 < k.check_ask { return; }
    let Some(who) = named.filter(|&w| owed_of(g, w, q)).or_else(|| missing(g, q)) else { return idle_a_beat(g, q) };
    if by_ear {
        if !fit(g, caller.unwrap()) { return; }
    } else if !caller.is_some_and(|c| fit(g, c) && !(now - g.gd[c].last_seen_t < 2.0)) {
        rc!(g, q).caller = home_first(g, q);
    }
    if by_ear || (rc!(g, q).late == Some(true) && rc!(g, q).caller.is_some() && cuts(g)) {
        let c = rc!(g, q);
        c.t0 = m::max(c.t0, now - k.check_ask);
    }
    let Some(caller) = rc!(g, q).caller else { return idle_for_now(g, q) };
    let routine = !by_ear && named.is_none();
    if net_asks(g) && routine {
        for &x in men {
            if crate::guards::off_net(g, x) && !rc!(g, q).known.has(x) { rc!(g, q).asked.get_or_insert_with(JsSet::new).add(x); }
        }
    }
    let c = rc!(g, q);
    c.late = Some(false);
    c.who = Some(who);
    c.stage = CheckStage::Asked;
    if let Some(p) = c.promised.as_mut() { p.delete(who); }
    let name = sq!(g, q).name.clone();
    let said = mate(g, who);
    on_net(g, caller, &format!("{}: {}? … {}, sound off", name, said, said), Some(2.0), None);
    event(g, q, SquadEventWhat::Asks, Some(caller));
    crate::netquietfeed::net_feed_asked(g, Some(caller), who, men);
}

/// NO ANSWER: the men at home nearest to his post go and look, and one at the least keeps the doors.
fn send_to_look(g: &mut Game, q: EnemysquadsSquadId) {
    let k = g.k.squads;
    let c = rc!(g, q);
    let of = c.who.filter(|&w| !c.known.has(w)).and_then(|w| c.silent.iter().find(|x| x.gd == w)).map(|r| (r.gd, r.at));
    let Some((who, at)) = of else { return idle_a_beat(g, q) };
    let waited = g.time - rc!(g, q).t0;
    let send = m::max(k.check_send, k.check_ask + 1.0);
    if waited < send { return; }
    let mut near: Vec<(RoomfightDefId, f64)> = home(g, q).iter().map(|d| (*d, dist_xz(g.pos_of(def!(g, *d).gd), at))).collect();
    js::sort_by(&mut near, |x, y| x.1 - y.1);
    let take = m::max(1.0, m::min(k.look_men, near.len() as f64 - 1.0)) as usize;
    let go: Vec<RoomfightDefId> = near.iter().take(take).map(|x| x.0).collect();
    if go.is_empty() {
        if waited > send + 20.0 { idle_for_now(g, q); }
        return;
    }
    let first = def!(g, go[0]).gd;
    if net_asks(g) {
        rc!(g, q).asked.get_or_insert_with(JsSet::new).add(who);
        for d in &go {
            let gd = def!(g, *d).gd;
            rc!(g, q).out.get_or_insert_with(JsSet::new).add(gd);
        }
    }
    for d in &go {
        let l = looker(g, q, *d, who, at);
        rc!(g, q).look.push(l);
        walk(g, *d, at);
    }
    let line = format!("{}: nothing from {} — {} moving to his post!", sq!(g, q).name.clone(), mate(g, who), count_word(go.len()));
    on_net(g, first, &line, None, None);
    event(g, q, SquadEventWhat::Looks, Some(first));
    idle_a_beat(g, q);
}
/// A man of squad Q set off to look for `who` at `at`: his record, with the post he leaves.
pub(super) fn looker(g: &crate::game::Game, q: EnemysquadsSquadId, d: RoomfightDefId, who: GuardId, at: V3) -> crate::enemysquads::Looker {
    let row = &def!(g, d);
    crate::enemysquads::Looker { gd: row.gd, who, spot: at, from: row.station, t0: g.time, news0: sq!(g, q).news_t, ..Default::default() }
}
/// Every looker out for `who` turns for the post he left.
fn turn_back(g: &mut crate::game::Game, q: EnemysquadsSquadId, who: GuardId) {
    let out: Vec<(GuardId, V3)> = rc!(g, q).look.iter_mut().filter(|x| x.who == who && !x.back).map(|x| { x.back = true; (x.gd, x.from) }).collect();
    for (gd, from) in out { if let Some(d) = on_his_way(g, gd) { walk(g, d, from); } }
}
/// the man's place in the fight, if he is on a leg of the squad's
fn on_his_way(g: &Game, gd: GuardId) -> Option<RoomfightDefId> { def_of(g, gd).filter(|&d| crate::roomfight::on_leg_of(g, d, crate::owner::by::SQUAD_LEG)) }
/// the row of the look list of the man `l`: a Looker is named by its man (home() keeps a man off a second row)
fn look_of(g: &mut Game, q: EnemysquadsSquadId, l: GuardId) -> &mut crate::enemysquads::Looker {
    rc!(g, q).look.iter_mut().find(|y| y.gd == l).unwrap()
}
/// what he finds: the mate he was sent for, or any man lying at another squad's posts; `around`: looking round him there, not walking by
fn lying(g: &mut Game, q: EnemysquadsSquadId, l: &Looker, around: bool) -> Option<GuardId> {
    let down = match l.about {
        Some(x) => sq!(g, x).men.clone(),
        None if around => vec![l.who],
        None => Vec::new(),
    };
    let near = if around { Some(LOOK_ROUND_M) } else { None };
    down.into_iter().find(|&b| crate::condition::down(g, g.gd[b].ch) && !rc!(g, q).known.has(b) && crate::guards::body_seen(g, l.gd, b, near))
}
/// he has the body: true = no longer a looker (the squad is in contact)
fn found(g: &mut Game, q: EnemysquadsSquadId, l: &Looker, body: GuardId) -> bool {
    let at = crate::guards::body_pos(g, body);
    let room = room_of(g, q, at, Some(1.0));
    if key_up(g, q, l.gd, at, room.as_ref(), Some(body)) { return false; }
    rc!(g, q).known.add(body);
    if let Some(x) = l.about { seen_to(g, x, body); }
    event(g, q, SquadEventWhat::Found, Some(l.gd));
    contact(g, q, at, room.as_ref(), Some(true), l.about.map(|_| l.gd), None);
    true
}
/// whom they were sent for, as it is said: a squad, or a mate
fn sent_for(g: &Game, l: &Looker) -> String {
    match l.about { Some(x) if l.all == Some(true) => sq!(g, x).name.clone(), _ => mate(g, l.who) }
}

/// A man out looking, once a frame; true = no longer a looker.
pub(super) fn looks(g: &mut crate::game::Game, q: EnemysquadsSquadId, gd: GuardId) -> bool {
    use crate::roomfight::LegEnd;
    let l = look_of(g, q, gd).clone();
    let Some(d) = still_up(g, q, gd) else { return true };
    if sq!(g, q).state == SquadState::Contact {
        // a fight at home: to whichever is nearer, at a run
        if on_his_way(g, gd).is_some() {
            let p = g.pos_of(gd);
            let to = if !l.back && dist_xz(p, l.spot) < dist_xz(p, l.from) { l.spot } else { l.from };
            crate::roomfight::run(g, d, to, None, 0.0, crate::owner::by::SQUAD_LEG);
        }
        return true;
    }
    let eyes = g.time - g.gd[gd].last_seen_t < 1.0 && roused(g, gd);
    let how = crate::roomfight::leg_ended(g, d, crate::owner::by::SQUAD_LEG);
    if eyes { saw_them(g, q, gd); }
    if eyes || matches!(how, Some(LegEnd::Contact | LegEnd::Taken)) { return true; }
    let dist = dist_xz(g.pos_of(gd), if l.back { l.from } else { l.spot });
    let (arrived, close) = (how == Some(LegEnd::Arrived), dist < LOOK_NEAR_M);
    if l.back {
        if how.is_none() { return false; }
        if arrived || close { crate::roomfight::back_on(g, d, true, "enemysquads"); } else { crate::roomfight::stand_here(g, d); }
        return true;
    }
    if called_off(g, q, &l) { return false; }
    if l.look_t.is_none() {
        if let Some(body) = passes_one(g, q, &l) { return found(g, q, &l, body); }
        if how.is_none() && dist >= LOOK_AT_M { return false; } // (on his way)
    }
    // he looks round him
    let now = g.time;
    let since = *look_of(g, q, gd).look_t.get_or_insert(now);
    if let Some(body) = lying(g, q, &l, true) { return found(g, q, &l, body); }
    if now - since < LOOK_SECS || said_on_air(g, q, &l) { return false; }
    if let Some(next) = next_stop(g, q, gd) { walk(g, d, next); return false; }
    if mate_still_out(g, q, &l) {
        look_of(g, q, gd).back = true;
        walk(g, d, l.from);
        return false;
    }
    nothing_there(g, q, &l, arrived || close);
    false
}
/// His place in the fight while he is up and nobody has him: down, his squad asks for him in his turn.
fn still_up(g: &mut Game, q: EnemysquadsSquadId, gd: GuardId) -> Option<RoomfightDefId> {
    if !alive(g, gd) {
        went_silent(g, q, gd, None);
        let soon = g.time + g.k.squads.check_ask;
        let c = rc!(g, q);
        c.next_at = m::min(c.next_at, soon);
        return None;
    }
    let d = def_of(g, gd)?;
    if g.gd[gd].held.is_some() { went_silent(g, q, gd, None); return None; }
    Some(d)
}
/// He has them in his eyes, away from his squad's rooms: he calls it.
fn saw_them(g: &mut Game, q: EnemysquadsSquadId, gd: GuardId) {
    let p = g.pos_of(gd);
    if room_of(g, q, p, Some(1.0)).is_some() { return; }
    let at = g.gd[gd].last_known.unwrap_or(p);
    if !key_up(g, q, gd, at, None, None) { contact(g, q, at, None, None, None, None); }
}
/// They turn back: he answers after all, or the squad has heard of the team since they set off.
fn called_off(g: &mut Game, q: EnemysquadsSquadId, l: &Looker) -> bool {
    let answers = match l.about {
        Some(x) if l.all == Some(true) => sq!(g, x).men.iter().any(|&y| !crate::guards::off_net(g, y)),
        _ => !crate::guards::off_net(g, l.who),
    };
    let news = sq!(g, q).state == SquadState::Alerted && sq!(g, q).news_t > l.news0;
    if !answers && !news { return false; }
    turn_back(g, q, l.who);
    if answers {
        let line = format!("{}: {}'s answering — back on our doors!", sq!(g, q).name, sent_for(g, l));
        on_net(g, l.gd, &line, None, None);
        event(g, q, SquadEventWhat::NoSign, Some(l.gd));
    }
    true
}
/// Somebody lying where he walks by, at another squad's posts: every sixth frame.
fn passes_one(g: &mut Game, q: EnemysquadsSquadId, l: &Looker) -> Option<GuardId> {
    l.about?;
    let n = l.n.unwrap_or(0.0) + 1.0;
    look_of(g, q, l.gd).n = Some(n);
    if js::rem(n, 6.0) != 0.0 { return None; }
    lying(g, q, l, false)
}
/// What he was sent about is being said on the air this moment.
fn said_on_air(g: &Game, q: EnemysquadsSquadId, l: &Looker) -> bool {
    let Some(x) = l.about else { return call_about(g, q, l.who) };
    let men = &sq!(g, x).men;
    sq!(g, q).calls.as_ref().map_or(false, |cs| cs.iter().any(|c| c.body.map_or(false, |b| men.contains(&b))))
}
/// The next place of his round, if there is one.
fn next_stop(g: &mut Game, q: EnemysquadsSquadId, gd: GuardId) -> Option<V3> {
    let r = look_of(g, q, gd);
    let stops = r.stops.as_mut().filter(|s| !s.is_empty())?;
    let next = stops.remove(0);
    r.spot = next;
    r.look_t = None;
    Some(next)
}
/// His mate, sent for the same man, is still on his way to look.
fn mate_still_out(g: &Game, q: EnemysquadsSquadId, l: &Looker) -> bool {
    sq!(g, q).rc.as_ref().is_some_and(|c| c.look.iter().any(|y| y.gd != l.gd && y.who == l.who && !y.back && alive(g, y.gd) && g.gd[y.gd].held.is_none() && on_his_way(g, y.gd).is_some()))
}
/// No body to see, or no way to his post: they go back, he is not asked for again, and the squad is ready.
fn nothing_there(g: &mut Game, q: EnemysquadsSquadId, l: &Looker, reached: bool) {
    rc!(g, q).looked.add(l.who);
    turn_back(g, q, l.who);
    let line = format!("{}: {} {} — back on our doors!", sq!(g, q).name, if reached { "no sign of" } else { "can't get to" }, sent_for(g, l));
    on_net(g, l.gd, &line, None, None);
    event(g, q, SquadEventWhat::NoSign, Some(l.gd));
    if let Some(x) = l.about { sq!(g, x).trouble = None; }
    if sq!(g, q).state == SquadState::Hold { sq!(g, q).ready = true; }
}

/// He has noticed the team by his own bar: 'alert' or 'search', not 'suspicious'.
pub(crate) fn roused(g: &crate::game::Game, m: GuardId) -> bool {
    g.gd[m].state == crate::game::GuardState::Alert || g.gd[m].state == crate::game::GuardState::Search
}
/// The trace's row for a sighting the squad does not call. Always false: nothing is called.
pub(super) fn glimpse(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> bool {
    let last = g.m.enemysquads.glimpsed.get(m).copied().unwrap_or(-100.0);
    let now = g.time;
    g.m.enemysquads.glimpsed.set(m, now);
    if now - last > 3.0 {
        event(g, q, SquadEventWhat::Glimpse, Some(m));
    }
    false
}
/// The floor is quiet as far as this squad knows: only then is a call a thing with a length.
pub(super) fn call_quiet(g: &crate::game::Game, q: EnemysquadsSquadId) -> bool {
    let r = &sq!(g, q);
    if r.state == SquadState::Contact || r.state == SquadState::Alerted || !(r.contact_t <= -100.0) || !(r.news_t <= -100.0) {
        return false;
    }
    let men = r.men.clone();
    for x in men.iter() {
        let up = alive(g, *x);
        let h = &g.gd[*x].witness.heard;
        if up && h.shots + h.kick + h.bang > 0.0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the word against the TypeScript's own (`(h ^ Math.floor(sum * 0x7fffff)) >>> 0`, run under bun)
    #[test]
    fn check_word_is_the_typescripts() {
        assert_eq!(check_word("Alpha", 0.0, 1.75), WORD_ALPHA);
        assert_eq!(check_word("Élan squad", 625341585.0, 0.123456789), WORD_ELAN);
        assert_eq!(check_word("", 0.0, 0.0), WORD_EMPTY);
    }
    const WORD_ALPHA: u32 = 60263861;
    const WORD_ELAN: u32 = 20381830;
    const WORD_EMPTY: u32 = 2166136261;
}
