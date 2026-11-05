//! src/game/enemysquads.ts lines 751 to 1150
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
fn count_word(n: usize) -> String {
    match count().get(n) {
        Some(w) => w.clone(),
        None => format!("{}", n),
    }
}
/// `m.char.pos`
fn pos(g: &Game, m: GuardId) -> V3 {
    g.ch[g.gd[m].ch].pos
}
/// `names.join(' and ')` of the men's callsigns
fn callsigns(g: &Game, men: &[GuardId]) -> String {
    let mut out = String::new();
    for (i, x) in men.iter().enumerate() {
        if i > 0 {
            out.push_str(" and ");
        }
        out.push_str(&g.gd[*x].callsign);
    }
    out
}

/// the row of Q.sent of the man `s`: a Sent is named by its man (home() keeps a man off a second row)
fn sent_of(g: &mut Game, q: EnemysquadsSquadId, s: GuardId) -> &mut crate::enemysquads::Sent {
    sq!(g, q).sent.iter_mut().find(|x| x.gd == s).unwrap()
}
/// A drilled garrison's pair holding a lost room's other door goes back to its own posts. `s` is his man: the row stays in Q.sent.   (enemysquads.ts back)
pub fn back(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) {
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
    let door = match sent_of(g, q, s).door {
        Some(dr) => g.dr[dr].def.name.clone(),
        None => String::from("the door"),
    };
    for x in pair.iter() {
        let x = *x;
        let xd = match def_of(g, x) {
            Some(xd) => xd,
            None => continue,
        };
        if def!(g, xd).mode == crate::roomfight::DefMode::Free {
            continue;
        }
        let from = {
            let row = sent_of(g, q, x);
            row.phase = SentPhase::Run;
            row.back = Some(true);
            row.hold = Some(false);
            row.spot = Some(row.from);
            row.door = None;
            row.way = None;
            row.tries = 0.0;
            row.from
        };
        for e in def!(g, xd).ev.iter_mut() {
            *e = 0.0;
        }
        def!(g, xd).pt[0] = at[0];
        def!(g, xd).pt[2] = at[2];
        crate::pressure::run(g, xd, from, None, Some(0.3));
        names.push(x);
    }
    if names.is_empty() {
        return;
    }
    let q_name = sq!(g, q).name.clone();
    let line = if gone {
        let ally = sq!(g, ally.unwrap()).name.clone();
        let verb = crate::words::wg(g, &"isn't", &"is not");
        format!("{}: {} {} answering — back on our doors!", q_name, ally, verb)
    } else {
        format!("{}: they've left it — back on our doors!", q_name)
    };
    crate::game::game_say(g, gd, &line, Some(true), None, None);
    let (s_aim, since) = {
        let row = sent_of(g, q, s);
        (row.aim, row.set_t.unwrap_or(row.t0))
    };
    let why = if gone {
        format!("{} is not answering", sq!(g, ally.unwrap()).name)
    } else {
        let from = s_aim.unwrap_or(pos(g, gd));
        format!("the team was heard {:.0} m off, going on", flat(at, from))
    };
    let text = format!(
        "{} leave{} {}: {}, {:.1} s after setting",
        callsigns(g, &names),
        if names.len() > 1 { "" } else { "s" },
        door,
        why,
        g.time - since
    );
    event(g, q, EventWhat::Back, &text, Some(gd));
}


/// A drilled garrison goes and looks: once a tick the word of the team is kept, and gone quiet a pair is called and sent.   (enemysquads.ts lookOut)
pub fn look_out(g: &mut crate::game::Game) {
    let k = match squad_drill(g) {
        Some(k) => k,
        None => return,
    };
    if !k.drill_look || !g.k.pressure.sweep {
        return;
    }
    st!(g).word.get_or_insert_with(|| StateWord {
        t: f64::NEG_INFINITY,
        at: [0.0, 0.0, 0.0],
        done: f64::NEG_INFINITY,
        said_t: f64::NEG_INFINITY,
        lost: false,
        out: None,
        call: None,
    });
    let last = st!(g).net.last().map(|c| (c.t, c.at));
    if let Some((t, at)) = last {
        let w = wd!(g);
        if t > w.t {
            w.t = t;
            w.at = at;
        }
    }
    let defs = g.fight.defs.clone();
    for dd in defs.iter() {
        let dd = *dd;
        let man = def!(g, dd).gd;
        if !alive(g, man) {
            continue;
        }
        let (held, seen_t, sight) = {
            let row = &g.gd[man];
            (row.held.is_some(), row.last_seen_t, row.sight_pos)
        };
        let (noise, noise_t) = {
            let r = &def!(g, dd);
            (r.noise_at, r.noise_t)
        };
        let w = wd!(g);
        if !held && seen_t >= 0.0 && seen_t > w.t {
            w.t = seen_t;
            w.at = sight;
        }
        if let Some(n) = noise {
            if noise_t > w.t {
                w.t = noise_t;
                w.at = n;
            }
        }
    }
    let squads = st!(g).squads.clone();
    if let Some(out) = wd!(g).out.clone() {
        let mut one_down = false;
        for x in out.iter() {
            if !alive(g, *x) {
                one_down = true;
                break;
            }
        }
        if one_down {
            let w = wd!(g);
            w.lost = true;
            w.out = None;
            note(g, None, "go and look: a man of the look is down — no squad sends another");
        } else {
            let any = squads.iter().any(|sid| sq!(g, *sid).sent.iter().any(|s| s.role == SentRole::Look));
            if !any {
                wd!(g).out = None;
            }
        }
    }
    if wd!(g).lost {
        return;
    }
    let mut busy = false;
    for sid in squads.iter() {
        let sid = *sid;
        let r = &sq!(g, sid);
        if r.state == SquadState::Contact
            || r.state == SquadState::Alerted
            || !r.due.is_empty()
            || r.calls.as_ref().map_or(false, |c| !c.is_empty())
            || r.rc.as_ref().map_or(false, |c| !c.look.is_empty())
        {
            busy = true;
            break;
        }
        let sent: Vec<GuardId> = r.sent.iter().map(|s| s.gd).collect();
        let mut out = false;
        for x in sent.iter() {
            if alive(g, *x) {
                out = true;
                break;
            }
        }
        if out {
            busy = true;
            break;
        }
    }
    if let Some(c) = wd!(g).call.clone() {
        let h = home(g, c.q);
        let word_t = wd!(g).t;
        if word_t > c.w || busy || !c.go.iter().all(|x| h.contains(x)) {
            let w = wd!(g);
            w.call = None;
            if word_t <= c.w {
                w.done = f64::NEG_INFINITY;
            }
            return;
        }
        if g.time < c.go_t {
            return;
        }
        let men: Vec<GuardId> = c.go.iter().map(|x| def!(g, *x).gd).collect();
        {
            let w = wd!(g);
            w.call = None;
            w.out = Some(men.clone());
        }
        for x in c.go.iter() {
            let x = *x;
            let row = crate::enemysquads::Sent {
                gd: def!(g, x).gd,
                from: def!(g, x).station,
                role: SentRole::Look,
                phase: SentPhase::Loose,
                hold: None,
                spot: None,
                door: None,
                fix_at: c.at,
                tries: 0.0,
                step_t: -100.0,
                t0: g.time,
                there: None,
                way: None,
                aim: None,
                set_t: None,
                ally: None,
                gone_t: None,
                back: None,
            };
            sq!(g, c.q).sent.push(row);
            crate::pressure::loose(g, x, c.at);
        }
        let text = format!("{} go to where the team was last seen or heard, {:.0} s after the last word of them", callsigns(g, &men), g.time - c.w);
        event(g, c.q, EventWhat::Sweep, &text, Some(men[0]));
        return;
    }
    let (w_t, w_at, w_done, w_said) = {
        let w = wd!(g);
        (w.t, w.at, w.done, w.said_t)
    };
    if w_t == f64::NEG_INFINITY || w_done >= w_t || g.time - w_t < k.sweep_secs || g.time - w_said < k.sweep_secs || busy {
        return;
    }
    let defs = g.fight.defs.clone();
    for dd in defs.iter() {
        let man = def!(g, *dd).gd;
        if alive(g, man) && def!(g, *dd).mode == crate::roomfight::DefMode::Free {
            return;
        }
    }
    let r = crate::roomfight::room_fight_room_at(g, w_at);
    if r.is_some() {
        for dd in defs.iter() {
            let man = def!(g, *dd).gd;
            if alive(g, man) && def!(g, *dd).mode == crate::roomfight::DefMode::Hold && def!(g, *dd).room == r {
                return;
            }
        }
    }
    let mut best: Option<(EnemysquadsSquadId, Vec<RoomfightDefId>, f64)> = None;
    for sid in squads.iter() {
        let sid = *sid;
        if sq!(g, sid).state == SquadState::Down || (sq!(g, sid).def.may_leave == Some(false) && !k.sweep_stayers) {
            continue;
        }
        let h = home(g, sid);
        let mut rooms: Vec<Option<RoomfightRoomId>> = Vec::new();
        for x in h.iter() {
            let room = def!(g, *x).room;
            if !rooms.contains(&room) {
                rooms.push(room);
            }
        }
        let spare = h.len() as f64 - rooms.len() as f64;
        let go = if spare >= 2.0 {
            let mut free = Vec::new();
            for x in h.iter() {
                if may_look(g, *x, w_at) {
                    free.push(*x);
                }
            }
            goers(g, &free, w_at, m::min(m::max(2.0, k.sweep_men), spare))
        } else {
            Vec::new()
        };
        if go.len() < 2 {
            continue;
        }
        let far: Vec<f64> = go.iter().map(|x| flat(pos(g, def!(g, *x).gd), w_at)).collect();
        let dist = m::min_n(&far);
        let better = match &best {
            Some(b) => dist < b.2,
            None => true,
        };
        if better {
            best = Some((sid, go, dist));
        }
    }
    let (bq, bgo, _) = match best {
        Some(b) => b,
        None => return,
    };
    let now = g.time;
    {
        let w = wd!(g);
        w.done = w_t;
        w.said_t = now;
        w.call = Some(StateWordCall { q: bq, go: bgo.clone(), at: w_at, go_t: now + k.sweep_beat, w: w_t });
    }
    let name = sq!(g, bq).name.clone();
    let n = count_word(bgo.len());
    let now_line = format!("{}: gone quiet — {} of us going to look!", name, n);
    let was_line = format!("{}: gone quiet — {} of us go and look!", name, n);
    let line = crate::words::wg(g, &now_line, &was_line);
    let first = def!(g, bgo[0]).gd;
    crate::game::game_say(g, first, &line, Some(true), None, None);
}
/// not a man on his way to cover or wedging a door; not one who knows a covered lane lies that way   (enemysquads.ts lookOut's `may`)
fn may_look(g: &mut Game, d: RoomfightDefId, at: V3) -> bool {
    let gd = def!(g, d).gd;
    let ph = crate::digin::dig_in_phase(g, gd);
    (ph == "idle" || ph == "set") && !crate::lanes::lane_holds(g, gd, Some(at)) && !crate::lanes::lane_warns(g, gd, Some(at))
}

/// Is that place in, or within three metres of, the room a holding man's door leads to.   (enemysquads.ts lostRoomHas)
pub fn lost_room_has(g: &mut crate::game::Game, s: &crate::enemysquads::Sent, at: V3) -> bool {
    let door = match s.door {
        Some(d) => d,
        None => return false,
    };
    let here = crate::roomfight::room_fight_room_at(g, pos(g, s.gd));
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

/// the word a check's die starts from: what checkDice works out, in integers   (enemysquads.ts checkDice)
fn check_word(name: &str, salt: f64, phases: f64) -> u32 {
    let mut h: i32 = (0x811c9dc5u32 as i32) ^ js::to_i32(salt);
    for c in name.encode_utf16() {
        h = m::imul(h ^ (c as i32), 0x01000193);
    }
    (h ^ js::to_i32(m::floor(phases * 8388607.0))) as u32
}
/// The check's own dice, never the game's: seeded off the squad's name and its men's stride phases.   (enemysquads.ts checkDice)
pub fn check_dice(g: &mut crate::game::Game, q: EnemysquadsSquadId, salt: Option<f64>) -> crate::math_dice::Die {
    let salt = salt.unwrap_or(0.0);
    let mut phases = 0.0;
    for x in sq!(g, q).men.iter() {
        phases = phases + g.ch[g.gd[*x].ch].anim.phase;
    }
    let word = check_word(&sq!(g, q).name, salt, phases);
    crate::math_dice::die_of(word as f64)
}
/// How long till the next check: a squad that has heard of the team counts its men oftener.   (enemysquads.ts checkBeat)
pub fn check_beat(g: &mut crate::game::Game, q: EnemysquadsSquadId, die: &mut crate::math_dice::Die) -> f64 {
    let k = g.k.squads;
    let r = &sq!(g, q);
    let alert = if r.news_t > -100.0 || r.contact_t > -100.0 || r.wary == Some(true) { k.check_alert_k } else { 1.0 };
    k.check_every * alert * (1.0 + (crate::math_dice::roll(die) * 2.0 - 1.0) * k.check_jitter)
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
    crate::math_dice::roll(&mut rc!(g, q).die)
}
/// The squad's check, made on the first asking: the row itself.   (enemysquads.ts rcOf)
pub fn rc_of<'g>(g: &'g mut crate::game::Game, q: EnemysquadsSquadId) -> &'g mut crate::enemysquads::Check {
    if sq!(g, q).rc.is_none() {
        let mut die = check_dice(g, q, None);
        let first = crate::math_dice::roll(&mut die);
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
/// what each man's ears had counted by now   (enemysquads.ts wentSilent's `small0`)
fn small0_of(g: &Game, q: EnemysquadsSquadId) -> JsMap<GuardId, f64> {
    let mut out = JsMap::new();
    for x in sq!(g, q).men.iter() {
        out.set(*x, g.gd[*x].witness.heard.small);
    }
    out
}
/// A man of the squad has gone quiet: where he stood, when, and what each mate's ears had counted by then. Nobody is told by it.   (enemysquads.ts wentSilent)
pub fn went_silent(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: Option<V3>, broke: Option<bool>) {
    let at = match at {
        Some(a) => a,
        None => pos(g, m),
    };
    let broke = broke.unwrap_or(false);
    rc_of(g, q);
    let was = rc!(g, q).silent.iter().position(|r| r.gd == m);
    let name = sq!(g, q).name.clone();
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
            let text = format!("{}: {}, not answering since he was taken, is down in its rooms: a new silence, looked for where he fell", name, g.gd[m].callsign);
            note(g, Some(m), &text);
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
                        x.tries = 0.0;
                        x.look_t = None;
                    }
                    if let Some(d) = def_of(g, man) {
                        if def!(g, d).mode == crate::roomfight::DefMode::Move {
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
    let up = alive(g, m);
    let text = format!(
        "{}: {} goes quiet ({}) {}",
        name,
        g.gd[m].callsign,
        if up { "taken" } else { "down" },
        if broke { "with his call broken off: his mates heard that much, and no more" } else { "and nobody of the squad knows it" }
    );
    note(g, Some(m), &text);
}
/// A line of the check on the squad's net: the log's row, the debrief's and the strip's, never his bubble.   (enemysquads.ts onNet)
pub fn on_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) {
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
    let line = format!("{} (radio): {}", g.gd[gd].callsign, text);
    crate::game::game_msg(g, &line);
}
/// A mate at home has gunfire in his ears with a drop.   (enemysquads.ts heardFire)
pub fn heard_fire(g: &mut crate::game::Game, q: EnemysquadsSquadId, up: &[GuardId]) -> bool {
    for x in up.iter() {
        let x = *x;
        let at = pos(g, x);
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
/// When a stun charge, or a door blown in, last reached this man's ears: his Witness counts them and this times them.   (enemysquads.ts bangT)
pub fn bang_t(g: &mut crate::game::Game, x: GuardId) -> f64 {
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
/// A looker's leg: pressure.ts's scripted leg, at the pace of a man going to look.   (enemysquads.ts walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    crate::pressure::run(g, d, to, None, None);
    let gd = def!(g, d).gd;
    let speed = g.tune.guard_investigate;
    crate::squad::order(g, gd, crate::squad::OrderPatch { speed: Some(Some(speed)), ..Default::default() });
}

/// the memo of radioCheck's `atHome`
fn at_home(g: &mut Game, q: EnemysquadsSquadId, h: &mut Option<Vec<RoomfightDefId>>) -> Vec<RoomfightDefId> {
    if h.is_none() {
        *h = Some(home(g, q));
    }
    h.clone().unwrap()
}
/// the man of longest silence nobody knows of or has looked for   (radioCheck's `missing`)
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
/// is he still to be asked for   (radioCheck's `owed`)
fn owed(g: &mut Game, q: EnemysquadsSquadId, who: GuardId) -> bool {
    let c = rc!(g, q).clone();
    owed_of(g, &c, who)
}
/// the check goes idle, the next one `after` from now; with `poll`, the routine check keeps its time   (radioCheck's `idle`)
fn idle(g: &mut Game, q: EnemysquadsSquadId, after: f64, poll: bool) -> Option<crate::enemysquads::RadioCheck> {
    let now = g.time;
    {
        let c = rc!(g, q);
        c.stage = CheckStage::Idle;
        c.who = None;
        c.by_ear = false;
        c.late = Some(false);
        c.next_at = match (poll, c.beat) {
            (true, Some(b)) => m::max(b, now + after),
            _ => now + after,
        };
        c.beat = None;
    }
    if let Some(cut) = rc!(g, q).cut.clone() {
        let mut kept = Vec::new();
        for x in cut.iter() {
            if owed(g, q, x.who) {
                kept.push(*x);
            }
        }
        rc!(g, q).cut = if kept.is_empty() { None } else { Some(kept) };
    }
    let c = rc!(g, q);
    if let Some(cut) = c.cut.as_ref() {
        let ats: Vec<f64> = cut.iter().map(|x| x.at).collect();
        let first = m::min_n(&ats);
        let next = c.next_at;
        c.beat = Some(next);
        c.next_at = m::min(next, m::max(now + m::min(after, 1.0), first));
    }
    None
}
/// Once a frame for a squad with the switch on: what its men's eyes and ears make of the men who went quiet, and the net's check.   (enemysquads.ts radioCheck)
pub fn radio_check(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::RadioCheck> {
    let k = g.k.squads;
    rc_of(g, q);
    rc!(g, q).n += 1.0;
    if g.time - rc!(g, q).tick_t > 1.0 {
        {
            let c = rc!(g, q);
            c.stage = CheckStage::Idle;
            c.who = None;
            c.by_ear = false;
            c.cut = None;
            c.beat = None;
            c.late = Some(false);
        }
        let beat = rc_beat(g, q);
        let now = g.time;
        rc!(g, q).next_at = now + beat;
    }
    let now = g.time;
    rc!(g, q).tick_t = now;
    let men = sq!(g, q).men.clone();
    let name = sq!(g, q).name.clone();
    for x in men.iter() {
        if alive(g, *x) {
            bang_t(g, *x);
        }
    }
    let mut h: Option<Vec<RoomfightDefId>> = None;
    let mut seen: Option<crate::enemysquads::RadioCheck> = None;
    let cut_on = k.call_cut_ask[1] > 0.0;
    let n_silent = rc!(g, q).silent.len();
    for i in (0..n_silent).rev() {
        let (who, r_t, r_at, r_heard) = {
            let r = &rc!(g, q).silent[i];
            (r.gd, r.t, r.at, r.heard)
        };
        let age = g.time - r_t;
        if !crate::guards::off_net(g, who) {
            let c = rc!(g, q);
            c.silent.remove(i);
            c.looked.delete(who);
            continue;
        }
        if rc!(g, q).known.has(who) || alive(g, who) {
            continue;
        }
        // (a) a mate's eyes
        let mut by = None;
        if age < 1.0 || js::rem(rc!(g, q).n + i as f64, 6.0) == 0.0 {
            for x in men.iter() {
                let x = *x;
                if x != who && fit(g, x) && crate::guards::body_seen(g, x, who, None) {
                    by = Some(x);
                    break;
                }
            }
        }
        if let Some(by) = by {
            let at = crate::guards::body_pos(g, who);
            if !busy {
                let room = room_of(g, q, at, Some(1.0));
                if key_up(g, q, by, at, room.as_ref(), Some(who)) {
                    continue;
                }
            }
            rc!(g, q).known.add(who);
            let text = format!("{} has {} in sight, down {:.1} m from him, {:.1} s after he dropped", g.gd[by].callsign, g.gd[who].callsign, flat(pos(g, by), at), age);
            event(g, q, EventWhat::Found, &text, Some(by));
            seen = Some(crate::enemysquads::RadioCheck { at, room: room_of(g, q, at, Some(1.0)) });
            continue;
        }
        if age > 3.0 {
            continue;
        }
        // gunfire (or a charge going off) a mate at home heard in the second after he dropped
        let mut shot = None;
        if age < 1.5 && (!cut_on || room_of(g, q, r_at, Some(1.0)).is_some()) {
            for x in men.iter() {
                let x = *x;
                if x != who && fit(g, x) && (g.gd[x].witness.heard.last_shot_t >= r_t - 0.2 || bang_t(g, x) >= r_t - 0.2) {
                    let at = pos(g, x);
                    if room_of(g, q, at, Some(1.0)).is_some() {
                        shot = Some(x);
                        break;
                    }
                }
            }
        }
        if let Some(shot) = shot {
            rc!(g, q).known.add(who);
            let text = format!(
                "{} heard the {} that dropped {}",
                g.gd[shot].callsign,
                if g.gd[shot].witness.heard.last_shot_t >= r_t - 0.2 { "shot" } else { "blast" },
                g.gd[who].callsign
            );
            event(g, q, EventWhat::Found, &text, Some(shot));
            seen = Some(crate::enemysquads::RadioCheck { at: r_at, room: room_of(g, q, r_at, Some(1.0)) });
            continue;
        }
        if r_heard {
            continue;
        }
        // (c) a mate's ears: a small thing heard SINCE he dropped, where he stood
        let mut ear = None;
        for x in men.iter() {
            let x = *x;
            if x == who || !fit(g, x) {
                continue;
            }
            let before = rc!(g, q).silent[i].small0.get(x).copied().unwrap_or(f64::INFINITY);
            let e = &g.gd[x].witness.heard;
            if e.small > before {
                if let Some(w) = e.small_where {
                    if flat(w, r_at) < k.check_hear_m {
                        ear = Some(x);
                        break;
                    }
                }
            }
        }
        let ear = match ear {
            Some(e) => e,
            None => continue,
        };
        rc!(g, q).silent[i].heard = true;
        let free = {
            let state = sq!(g, q).state;
            let c = rc!(g, q);
            c.stage == CheckStage::Idle && !busy && c.look.is_empty() && state != SquadState::Alerted
        };
        if free {
            let draw = rc_roll(g, q);
            let now = g.time;
            let t0 = now + k.check_heard[0] + draw * (k.check_heard[1] - k.check_heard[0]) - k.check_ask;
            {
                let c = rc!(g, q);
                c.stage = CheckStage::Called;
                c.by_ear = true;
                c.caller = Some(ear);
                c.who = Some(who);
                c.t0 = t0;
            }
            let text = format!("{} heard something where {} stands: he asks for him in {:.1} s", g.gd[ear].callsign, g.gd[who].callsign, t0 + k.check_ask - now);
            event(g, q, EventWhat::Check, &text, Some(ear));
        }
    }
    if busy {
        let rows: Vec<GuardId> = rc!(g, q).silent.iter().map(|r| r.gd).collect();
        for x in rows.iter() {
            if !alive(g, *x) {
                rc!(g, q).known.add(*x);
            }
        }
    }
    if busy || seen.is_some() {
        let promised = rc!(g, q).promised.as_ref().map(|p| p.keys());
        if let Some(promised) = promised {
            if !promised.is_empty() {
                for x in promised.iter() {
                    let text = format!(
                        "{}: nobody asks for {} after all — {} first, and counts its men as a squad in a fight does",
                        name,
                        g.gd[*x].callsign,
                        if busy { "the squad is in contact" } else { "a man of it is seen down" }
                    );
                    note(g, Some(*x), &text);
                }
                rc!(g, q).promised = None;
            }
        }
        let now = g.time;
        let c = rc!(g, q);
        c.stage = CheckStage::Idle;
        c.who = None;
        c.by_ear = false;
        c.cut = None;
        c.beat = None;
        c.late = Some(false);
        c.next_at = m::max(c.next_at, now + 5.0);
        return if busy { None } else { seen };
    }
    let now = g.time;
    let news = {
        let r = &sq!(g, q);
        r.news_t > -100.0 || r.contact_t > -100.0 || r.wary == Some(true)
    };
    if rc!(g, q).stage == CheckStage::Idle && news {
        let c = rc!(g, q);
        c.next_at = m::min(c.next_at, now + k.check_every * k.check_alert_k * (1.0 + k.check_jitter));
    }
    if rc!(g, q).stage == CheckStage::Idle && now < rc!(g, q).next_at {
        return None;
    }
    if sq!(g, q).calls.as_ref().map_or(false, |c| !c.is_empty()) {
        let c = rc!(g, q);
        if c.stage == CheckStage::Called && now - c.t0 >= k.check_ask {
            c.late = Some(true);
        }
        return None;
    }
    if rc!(g, q).cut.is_some() && !cut_on {
        let c = rc!(g, q);
        c.cut = None;
        c.promised = None;
        if let Some(b) = c.beat {
            c.next_at = m::max(c.next_at, b);
            c.beat = None;
        }
    }
    // (b) the net's check
    for x in men.iter() {
        let x = *x;
        if alive(g, x) && g.gd[x].held.is_some() && !rc!(g, q).looked.has(x) && !sq!(g, q).sent.iter().any(|s| s.gd == x) {
            let station = def_of(g, x).map(|d| def!(g, d).station);
            went_silent(g, q, x, station, None);
        }
    }
    // the squad has heard of the team since the check was called and its word is due: nobody is asked for over that
    if rc!(g, q).stage != CheckStage::Idle && sq!(g, q).state == SquadState::Alerted {
        let (caller, who, stage, by_ear) = {
            let c = rc!(g, q);
            (c.caller, c.who, c.stage, c.by_ear)
        };
        let about = match who {
            Some(w) => format!(", {} {}", g.gd[w].callsign, if stage == CheckStage::Asked { "asked for and nobody sent" } else { "not asked for" }),
            None => String::new(),
        };
        let text = format!("{} check dropped{}: the squad has heard of the team since it was called", name, about);
        note(g, caller, &text);
        if stage == CheckStage::Called && who.is_some() && !by_ear && cut_on {
            let now = g.time;
            rc!(g, q).cut.get_or_insert_with(Vec::new).insert(0, CheckCutItem { who: who.unwrap(), at: now });
        }
        return idle(g, q, 5.0, true);
    }
    match rc!(g, q).stage {
        CheckStage::Idle => {
            if sq!(g, q).state == SquadState::Alerted || at_home(g, q, &mut h).is_empty() || !rc!(g, q).look.is_empty() {
                return idle(g, q, 5.0, true);
            }
            if rc!(g, q).cut.is_some() || rc!(g, q).beat.is_some() {
                let mut x: Option<CheckCutItem> = None;
                let mut rest: Option<Vec<CheckCutItem>> = None;
                if let Some(cut) = rc!(g, q).cut.clone() {
                    let mut out = Vec::new();
                    for y in cut.iter() {
                        if owed(g, q, y.who) {
                            if x.is_none() {
                                x = Some(*y);
                            } else {
                                out.push(*y);
                            }
                        }
                    }
                    rest = Some(out);
                }
                let b = rc!(g, q).beat;
                {
                    let c = rc!(g, q);
                    c.cut = match rest {
                        Some(v) if !v.is_empty() => Some(v),
                        _ => None,
                    };
                    c.beat = None;
                }
                if x.is_none() {
                    if let Some(b) = b {
                        if g.time < b {
                            rc!(g, q).next_at = b;
                            return None;
                        }
                    }
                }
                if let Some(x) = x {
                    let first = at_home(g, q, &mut h)[0];
                    let caller = def!(g, first).gd;
                    let now = g.time;
                    {
                        let c = rc!(g, q);
                        c.stage = CheckStage::Called;
                        c.by_ear = false;
                        c.caller = Some(caller);
                        c.who = Some(x.who);
                        c.t0 = m::max(now, x.at) - k.check_ask;
                    }
                    let when = if x.at > now { format!(" in {:.1} s", x.at - now) } else { String::new() };
                    let text = format!("{} has not forgotten the call that broke off: he asks for {}{}", g.gd[caller].callsign, g.gd[x.who].callsign, when);
                    event(g, q, EventWhat::Check, &text, Some(caller));
                    return None;
                }
            }
            if missing(g, q).is_none() {
                let mut on_net_men = 0;
                for x in men.iter() {
                    if !crate::guards::off_net(g, *x) {
                        on_net_men += 1;
                    }
                }
                if on_net_men < 2 {
                    let beat = rc_beat(g, q);
                    return idle(g, q, beat, false);
                }
            }
            let first = at_home(g, q, &mut h)[0];
            let caller = def!(g, first).gd;
            let now = g.time;
            {
                let c = rc!(g, q);
                c.stage = CheckStage::Called;
                c.t0 = now;
                c.caller = Some(caller);
                c.who = None;
            }
            on_net(g, caller, &format!("{}: radio check — sound off", name), Some(6.0), None);
            let text = format!("{} calls the squad's radio check", g.gd[caller].callsign);
            event(g, q, EventWhat::Check, &text, Some(caller));
            None
        }
        CheckStage::Called => {
            let (by_ear, caller0) = {
                let c = rc!(g, q);
                (c.by_ear, c.caller)
            };
            if by_ear
                && match caller0 {
                    Some(c) => crate::guards::off_net(g, c),
                    None => true,
                }
            {
                let who0 = rc!(g, q).who;
                let text = format!(
                    "{} check dropped: {} who heard something where {} stands is off the net before he asked",
                    name,
                    match caller0 {
                        Some(c) => g.gd[c].callsign.clone(),
                        None => String::from("the man"),
                    },
                    match who0 {
                        Some(w) => g.gd[w].callsign.clone(),
                        None => String::from("a mate"),
                    }
                );
                note(g, caller0, &text);
                let c = rc!(g, q);
                if let Some(w) = who0 {
                    if let Some(r) = c.silent.iter_mut().find(|x| x.gd == w) {
                        r.heard = false;
                    }
                }
                c.stage = CheckStage::Idle;
                c.who = None;
                c.by_ear = false;
                c.late = Some(false);
                return None;
            }
            if g.time - rc!(g, q).t0 < k.check_ask {
                return None;
            }
            let named = rc!(g, q).who;
            let keep = match named {
                Some(w) => crate::guards::off_net(g, w) && !rc!(g, q).known.has(w) && !rc!(g, q).looked.has(w),
                None => false,
            };
            let who = if keep { named } else { missing(g, q) };
            let who = match who {
                Some(w) => w,
                None => {
                    let beat = rc_beat(g, q);
                    return idle(g, q, beat, false);
                }
            };
            if by_ear {
                if !fit(g, caller0.unwrap()) {
                    return None;
                }
                let now = g.time;
                let c = rc!(g, q);
                c.t0 = m::max(c.t0, now - k.check_ask);
            } else {
                let fine = match caller0 {
                    Some(c) => fit(g, c) && !(g.time - g.gd[c].last_seen_t < 2.0),
                    None => false,
                };
                if !fine {
                    let first = at_home(g, q, &mut h).first().map(|d| def!(g, *d).gd);
                    rc!(g, q).caller = first;
                }
            }
            let caller = match rc!(g, q).caller {
                Some(c) => c,
                None => return idle(g, q, 5.0, true),
            };
            if rc!(g, q).late == Some(true) && cut_on && !by_ear {
                let now = g.time;
                let c = rc!(g, q);
                c.t0 = m::max(c.t0, now - k.check_ask);
            }
            rc!(g, q).late = Some(false);
            if net_asks(g) && !by_ear && rc!(g, q).who.is_none() {
                for x in men.iter() {
                    if crate::guards::off_net(g, *x) && !rc!(g, q).known.has(*x) {
                        rc!(g, q).asked.get_or_insert_with(JsSet::new).add(*x);
                    }
                }
            }
            {
                let c = rc!(g, q);
                c.who = Some(who);
                c.stage = CheckStage::Asked;
                if let Some(p) = c.promised.as_mut() {
                    p.delete(who);
                }
            }
            let said = mate(g, who);
            let said2 = mate(g, who);
            on_net(g, caller, &format!("{}: {}? … {}, sound off", name, said, said2), Some(2.0), None);
            let text = format!("no answer from {}: {} asks for him by name", g.gd[who].callsign, g.gd[caller].callsign);
            event(g, q, EventWhat::Asks, &text, Some(caller));
            crate::netquietfeed::net_feed_asked(g, Some(caller), who, &men);
            None
        }
        CheckStage::Asked => {
            let who = rc!(g, q).who;
            let row = match who {
                Some(w) => rc!(g, q).silent.iter().find(|x| x.gd == w).map(|r| (r.at, r.t)),
                None => None,
            };
            let (who, r_at, r_t) = match (who, row) {
                (Some(w), Some((at, t))) if !rc!(g, q).known.has(w) => (w, at, t),
                _ => {
                    let beat = rc_beat(g, q);
                    return idle(g, q, beat, false);
                }
            };
            let send = m::max(k.check_send, k.check_ask + 1.0);
            if g.time - rc!(g, q).t0 < send {
                return None;
            }
            let a = at_home(g, q, &mut h);
            let mut by: Vec<(RoomfightDefId, f64)> = a.iter().map(|d| (*d, flat(pos(g, def!(g, *d).gd), r_at))).collect();
            js::sort_by(&mut by, |x, y| x.1 - y.1);
            let take = m::max(1.0, m::min(k.look_men, a.len() as f64 - 1.0)) as usize;
            let go: Vec<RoomfightDefId> = by.iter().take(take).map(|x| x.0).collect();
            if go.is_empty() {
                return if g.time - rc!(g, q).t0 > send + 20.0 { idle(g, q, 5.0, true) } else { None };
            }
            let men_go: Vec<GuardId> = go.iter().map(|d| def!(g, *d).gd).collect();
            if net_asks(g) {
                let c = rc!(g, q);
                c.asked.get_or_insert_with(JsSet::new).add(who);
                for x in men_go.iter() {
                    c.out.get_or_insert_with(JsSet::new).add(*x);
                }
            }
            for d in go.iter() {
                let l = looker(g, q, *d, who, r_at);
                rc!(g, q).look.push(l);
                walk(g, *d, r_at);
            }
            let line = format!("{}: nothing from {} — {} moving to his post!", name, mate(g, who), count_word(go.len()));
            on_net(g, men_go[0], &line, None, None);
            let text = format!(
                "{} walk{} to where {} stood, {:.0} m off, {:.1} s after he went quiet: nothing is said to the other squads",
                callsigns(g, &men_go),
                if go.len() > 1 { "" } else { "s" },
                g.gd[who].callsign,
                flat(pos(g, men_go[0]), r_at),
                g.time - r_t
            );
            event(g, q, EventWhat::Looks, &text, Some(men_go[0]));
            let beat = rc_beat(g, q);
            idle(g, q, beat, false)
        }
    }
}
/// A man of squad Q set off to look for `who` at `at`: his record, with the post he leaves and how he held it.   (enemysquads.ts looker)
pub fn looker(g: &mut crate::game::Game, q: EnemysquadsSquadId, d: RoomfightDefId, who: GuardId, at: V3) -> crate::enemysquads::Looker {
    let row = &def!(g, d);
    let gd = row.gd;
    let man = &g.gd[gd];
    crate::enemysquads::Looker {
        gd,
        who,
        spot: at,
        from: row.station,
        back: false,
        tries: 0.0,
        step_t: -100.0,
        t0: g.time,
        news0: sq!(g, q).news_t,
        look_t: None,
        calm: crate::enemysquads::Calm {
            state: man.state,
            awareness: man.awareness,
            hold: man.hold,
            pinned: man.pinned,
            station: row.station,
            home: row.home,
            room: row.room,
            ev: row.ev.clone(),
            watch: row.watch,
            want: row.want,
            pt: row.pt,
        },
        about: None,
        all: None,
        stops: None,
        n: None,
    }
}
/// Every looker out for `who` turns for the post he left.   (enemysquads.ts turnBack)
pub fn turn_back(g: &mut crate::game::Game, q: EnemysquadsSquadId, who: GuardId) {
    let mut i = 0;
    while i < rc!(g, q).look.len() {
        let (x_who, x_back, man) = {
            let x = &rc!(g, q).look[i];
            (x.who, x.back, x.gd)
        };
        if x_who == who && !x_back {
            let from = {
                let x = &mut rc!(g, q).look[i];
                x.back = true;
                x.tries = 0.0;
                x.from
            };
            if let Some(d) = def_of(g, man) {
                if def!(g, d).mode == crate::roomfight::DefMode::Move {
                    walk(g, d, from);
                }
            }
        }
        i += 1;
    }
}
/// the row of the look list of the man `l`: a Looker is named by its man (home() keeps a man off a second row)
fn look_of(g: &mut Game, q: EnemysquadsSquadId, l: GuardId) -> &mut crate::enemysquads::Looker {
    rc!(g, q).look.iter_mut().find(|y| y.gd == l).unwrap()
}
/// Back on the post he left, as he held it: wary and no more.   (enemysquads.ts settle)
pub fn settle(g: &mut crate::game::Game, l: &crate::enemysquads::Looker, d: RoomfightDefId) {
    let gd = def!(g, d).gd;
    let c = &l.calm;
    let roused = c.state == crate::game::GuardState::Alert || c.state == crate::game::GuardState::Search;
    let heard = g.gd[gd].witness.heard.last_shot_t > l.t0 || g.gd[gd].awareness >= 0.85;
    if g.gd[gd].script.is_some() {
        let to = if !roused {
            crate::squad::EndGuardScriptState::Suspicious
        } else if c.state == crate::game::GuardState::Alert {
            crate::squad::EndGuardScriptState::Alert
        } else {
            crate::squad::EndGuardScriptState::Search
        };
        crate::squad::end_guard_script(g, gd, Some(to));
    }
    let aw = g.gd[gd].awareness;
    g.gd[gd].awareness = if roused {
        m::max(aw, c.awareness)
    } else if heard {
        m::max_n(&[aw, c.awareness, 0.35])
    } else {
        m::min(0.45, m::max_n(&[aw, c.awareness, 0.35]))
    };
    g.gd[gd].hold = c.hold;
    g.gd[gd].pinned = c.pinned;
    let (state_now, seen_t) = (String::from(g.gd[gd].state.as_str()), g.gd[gd].last_seen_t);
    let row = &mut def!(g, d);
    row.mode = crate::roomfight::DefMode::Hold;
    row.station = c.station;
    row.home = c.home;
    row.room = c.room;
    row.ev = c.ev.clone();
    row.watch = c.watch;
    row.want = c.want;
    row.pt[0] = c.pt[0];
    row.pt[1] = c.pt[1];
    row.pt[2] = c.pt[2];
    row.glance_until = -1.0;
    row.fix_keep = -1.0;
    row.rally_at = f64::NAN;
    row.last_state = state_now;
    row.last_seen = seen_t;
    let pt = row.pt;
    g.gd[gd].last_known = Some(pt);
}
/// what he finds: the mate he was sent for, or any man lying at another squad's posts   (looks' `lying`)
fn lying(g: &mut Game, q: EnemysquadsSquadId, gd: GuardId, x: Option<EnemysquadsSquadId>, who: GuardId, around: bool) -> Option<GuardId> {
    if let Some(x) = x {
        let men = sq!(g, x).men.clone();
        for b in men.iter() {
            let b = *b;
            if g.gd[b].state == crate::game::GuardState::Dead && !rc!(g, q).known.has(b) && crate::guards::body_seen(g, gd, b, if around { Some(LOOK_ROUND_M) } else { None }) {
                return Some(b);
            }
        }
        return None;
    }
    if g.gd[who].state == crate::game::GuardState::Dead && !rc!(g, q).known.has(who) && around && crate::guards::body_seen(g, gd, who, Some(LOOK_ROUND_M)) {
        Some(who)
    } else {
        None
    }
}
/// he has the body: true = no longer a looker (the squad is in contact)   (looks' `found`)
fn found(g: &mut Game, q: EnemysquadsSquadId, gd: GuardId, x: Option<EnemysquadsSquadId>, t0: f64, body: GuardId) -> bool {
    let at = crate::guards::body_pos(g, body);
    let room = room_of(g, q, at, Some(1.0));
    if key_up(g, q, gd, at, room.as_ref(), Some(body)) {
        return false;
    }
    rc!(g, q).known.add(body);
    if let Some(x) = x {
        seen_to(g, x, body);
    }
    let posts = match x {
        Some(x) => format!(" at {}'s posts", sq!(g, x).name),
        None => String::new(),
    };
    let text = format!(
        "{}, sent to look{}, has {} down {:.1} m from him, {:.1} s after he set off",
        g.gd[gd].callsign,
        posts,
        g.gd[body].callsign,
        flat(pos(g, gd), at),
        g.time - t0
    );
    event(g, q, EventWhat::Found, &text, Some(gd));
    let room = room_of(g, q, at, Some(1.0));
    contact(g, q, at, room.as_ref(), Some(true), if x.is_some() { Some(gd) } else { None }, None);
    true
}
/// A man out looking, once a frame; true = no longer a looker. `l` is his man: his row stays in the look list, written where it lies.   (enemysquads.ts looks)
pub fn looks(g: &mut crate::game::Game, q: EnemysquadsSquadId, l: GuardId) -> bool {
    let gd = l;
    let who = look_of(g, q, l).who;
    let dd = def_of(g, gd);
    if !alive(g, gd) {
        went_silent(g, q, gd, None, None);
        let (now, ask) = (g.time, g.k.squads.check_ask);
        let c = rc!(g, q);
        c.next_at = m::min(c.next_at, now + ask);
        return true;
    }
    let d = match dd {
        Some(d) => d,
        None => return true,
    };
    if g.gd[gd].held.is_some() {
        went_silent(g, q, gd, None, None);
        return true;
    }
    let step_t = look_of(g, q, l).step_t;
    let step_t = crate::pressure::boots(g, gd, step_t);
    look_of(g, q, l).step_t = step_t;
    // (back, spot and from move only just before a return below: these are his row as it stands all the way down)
    let (l_back, l_spot, l_from, l_t0, l_news0, x, all) = {
        let r = look_of(g, q, l);
        (r.back, r.spot, r.from, r.t0, r.news0, r.about, r.all == Some(true))
    };
    if sq!(g, q).state == SquadState::Contact {
        if def!(g, d).mode == crate::roomfight::DefMode::Move {
            let p = pos(g, gd);
            let to = if !l_back && flat(p, l_spot) < flat(p, l_from) { l_spot } else { l_from };
            crate::pressure::run(g, d, to, None, None);
        }
        return true;
    }
    let eyes = g.time - g.gd[gd].last_seen_t < 1.0 && noticed(g, gd);
    if eyes || def!(g, d).mode != crate::roomfight::DefMode::Move {
        if eyes {
            let p = pos(g, gd);
            if room_of(g, q, p, Some(1.0)).is_none() {
                let at = g.gd[gd].last_known.unwrap_or(p);
                if !(noticed(g, gd) && key_up(g, q, gd, at, None, None)) {
                    let at = g.gd[gd].last_known.unwrap_or(pos(g, gd));
                    contact(g, q, at, None, None, None, None);
                }
            }
        }
        return true;
    }
    let name = sq!(g, q).name.clone();
    if !l_back {
        let answering = if all {
            let men = sq!(g, x.unwrap()).men.clone();
            let mut any = false;
            for y in men.iter() {
                if !crate::guards::off_net(g, *y) {
                    any = true;
                    break;
                }
            }
            any
        } else {
            !crate::guards::off_net(g, who)
        };
        if answering {
            turn_back(g, q, who);
            let said = if all { sq!(g, x.unwrap()).name.clone() } else { mate(g, who) };
            on_net(g, gd, &format!("{}: {}'s answering — back on our doors!", name, said), None, None);
            let text = if all {
                format!("somebody of {} is on the net again: the men who went to its posts go back", sq!(g, x.unwrap()).name)
            } else {
                format!("{} is on the net again: the men who went for him go back", g.gd[who].callsign)
            };
            event(g, q, EventWhat::NoSign, &text, Some(gd));
            return false;
        }
    }
    if !l_back && sq!(g, q).state == SquadState::Alerted && sq!(g, q).news_t > l_news0 {
        turn_back(g, q, who);
        return false;
    }
    let goal = if l_back { l_from } else { l_spot };
    let dist = flat(pos(g, gd), goal);
    let arrived = g.gd[gd].script.as_ref().map_or(false, |s| s.arrived == Some(true));
    let there = arrived || dist < (if l_back { 0.5 } else { LOOK_AT_M });
    let stuck = def!(g, d).stall_t > 1.3;
    let close = dist < LOOK_NEAR_M;
    let looking = !l_back && look_of(g, q, l).look_t.is_some();
    if x.is_some() && !l_back && !looking {
        let r = look_of(g, q, l);
        let n = r.n.unwrap_or(0.0) + 1.0;
        r.n = Some(n);
        if js::rem(n, 6.0) == 0.0 {
            if let Some(body) = lying(g, q, gd, x, who, false) {
                return found(g, q, gd, x, l_t0, body);
            }
        }
    }
    if !there && !looking {
        if !stuck && def!(g, d).move_t <= 8.0 {
            return false;
        }
        if !(stuck && close) && look_of(g, q, l).tries < 8.0 {
            look_of(g, q, l).tries += 1.0;
            walk(g, d, goal);
            return false;
        }
    }
    let reached = there || close;
    if l_back {
        if reached {
            let row = look_of(g, q, l).clone();
            settle(g, &row, d);
        }
        return true;
    }
    let now = g.time;
    let look_t = *look_of(g, q, l).look_t.get_or_insert(now);
    if let Some(body) = lying(g, q, gd, x, who, true) {
        return found(g, q, gd, x, l_t0, body);
    }
    if g.time - look_t < LOOK_SECS {
        let arrived = g.gd[gd].script.as_ref().map_or(false, |s| s.arrived == Some(true));
        if arrived && flat(pos(g, gd), l_spot) < 0.5 {
            g.gd[gd].script.as_mut().unwrap().arrived = Some(false);
            def!(g, d).stall_t = 0.0;
            def!(g, d).move_t = 0.0;
        } else if arrived || def!(g, d).stall_t > 1.0 || def!(g, d).move_t > 8.0 {
            walk(g, d, l_spot);
        }
        return false;
    }
    let on_air = match x {
        Some(x) => {
            let men = &sq!(g, x).men;
            sq!(g, q).calls.as_ref().map_or(false, |cs| cs.iter().any(|c| c.body.map_or(false, |b| men.contains(&b))))
        }
        None => call_about(g, q, who),
    };
    if on_air {
        return false;
    }
    let next_stop = {
        let r = look_of(g, q, l);
        match r.stops.as_mut() {
            Some(stops) if !stops.is_empty() => {
                let next = stops.remove(0);
                r.spot = next;
                r.look_t = None;
                r.tries = 0.0;
                Some(next)
            }
            _ => None,
        }
    };
    if let Some(next) = next_stop {
        walk(g, d, next);
        return false;
    }
    // his mate is still on his way to look: he goes back to his post
    let rows: Vec<(GuardId, GuardId, bool)> = rc!(g, q).look.iter().map(|y| (y.gd, y.who, y.back)).collect();
    let mut mate_out = false;
    for (y_gd, y_who, y_back) in rows.iter() {
        let y_gd = *y_gd;
        if y_gd != gd && *y_who == who && !*y_back && alive(g, y_gd) && g.gd[y_gd].held.is_none() {
            if let Some(yd) = def_of(g, y_gd) {
                if def!(g, yd).mode == crate::roomfight::DefMode::Move {
                    mate_out = true;
                    break;
                }
            }
        }
    }
    if mate_out {
        {
            let r = look_of(g, q, l);
            r.back = true;
            r.tries = 0.0;
        }
        walk(g, d, l_from);
        let text = format!(
            "{}: {} {} {} and goes back; his mate is still on his way to look",
            name,
            g.gd[gd].callsign,
            if reached { "sees nothing of" } else { "finds no way to" },
            g.gd[who].callsign
        );
        note(g, Some(gd), &text);
        return false;
    }
    // no body to see, or no way to his post: they go back, he is not asked for again, and the squad is ready
    rc!(g, q).looked.add(who);
    turn_back(g, q, who);
    let said = if all { sq!(g, x.unwrap()).name.clone() } else { mate(g, who) };
    on_net(g, gd, &format!("{}: {} {} — back on our doors!", name, if reached { "no sign of" } else { "can't get to" }, said), None, None);
    let what = if all {
        let xn = &sq!(g, x.unwrap()).name;
        if reached {
            format!("nobody of {} is at its posts", xn)
        } else {
            format!("no way to {}'s posts", xn)
        }
    } else if reached {
        format!("{} is not where he stood", g.gd[who].callsign)
    } else {
        format!("no way to {}'s post", g.gd[who].callsign)
    };
    let text = format!("{}: {} and whoever came with him go back to their posts, the squad ready for a call", what, g.gd[gd].callsign);
    event(g, q, EventWhat::NoSign, &text, Some(gd));
    if let Some(x) = x {
        sq!(g, x).trouble = None;
    }
    if sq!(g, q).state == SquadState::Hold {
        sq!(g, q).ready = true;
    }
    false
}

/// He has noticed the team by his own bar: 'alert' or 'search', not 'suspicious'.   (enemysquads.ts noticed)
pub fn noticed(g: &mut crate::game::Game, m: GuardId) -> bool {
    g.gd[m].state == crate::game::GuardState::Alert || g.gd[m].state == crate::game::GuardState::Search
}
/// The trace's row for a sighting the squad does not call. Always false: nothing is called.   (enemysquads.ts glimpse)
pub fn glimpse(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> bool {
    let last = g.m.enemysquads.glimpsed.get(m).copied().unwrap_or(-100.0);
    let now = g.time;
    g.m.enemysquads.glimpsed.set(m, now);
    if now - last > 3.0 {
        let text = format!(
            "{} has one of them in his eye and has not made him out (awareness {:.2}, {}): nothing is called",
            g.gd[m].callsign,
            g.gd[m].awareness,
            g.gd[m].state.as_str()
        );
        event(g, q, EventWhat::Glimpse, &text, Some(m));
    }
    false
}
/// The floor is quiet as far as this squad knows: only then is a call a thing with a length.   (enemysquads.ts callQuiet)
pub fn call_quiet(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool {
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
/// A man has something only he knows: on a quiet floor he keys up. true = the squad is not told this frame; false = the floor is not quiet.   (enemysquads.ts keyUp)
pub fn key_up(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, body: Option<GuardId>) -> bool {
    if !call_quiet(g, q) {
        return false;
    }
    if !fit(g, m) {
        return true;
    }
    let calls = sq!(g, q).calls.get_or_insert_with(Vec::new);
    if let Some(c) = calls.iter_mut().find(|x| x.by == m) {
        if body.is_none() {
            c.eyes = true;
            c.at = at;
            c.room = room.cloned();
        }
        return true;
    }
    if sq!(g, q).call_die.is_none() {
        let die = check_dice(g, q, Some(625341585.0));
        sq!(g, q).call_die = Some(die);
    }
    let [lo, hi] = g.k.squads.call_secs;
    let secs = lo + crate::math_dice::roll(sq!(g, q).call_die.as_mut().unwrap()) * (hi - lo);
    let now = g.time;
    sq!(g, q).calls.as_mut().unwrap().push(crate::enemysquads::Call { by: m, t0: now, end: now + secs, at, room: room.cloned(), body, eyes: body.is_none() });
    let said = mate(g, m);
    let line = format!("{}: {} — I've got—", sq!(g, q).name, said);
    on_net(g, m, &line, Some(2.0), None);
    let what = match body {
        Some(b) => format!("{} down in his sight", g.gd[b].callsign),
        None => format!("one of them in his sight, {}", g.gd[m].state.as_str()),
    };
    let text = format!("{} keys up ({}): the squad has it in {:.2} s unless he is cut off", g.gd[m].callsign, what, secs);
    event(g, q, EventWhat::Calls, &text, Some(m));
    true
}
/// Is a man of the squad on the air about this mate's body.   (enemysquads.ts callAbout)
pub fn call_about(g: &mut crate::game::Game, q: EnemysquadsSquadId, body: GuardId) -> bool {
    sq!(g, q).calls.as_ref().map_or(false, |cs| cs.iter().any(|c| c.body == Some(body)))
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
