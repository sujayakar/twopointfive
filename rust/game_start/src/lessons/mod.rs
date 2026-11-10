//! THE TUTORIALS, PLAYED: a scripted player (`player.rs`) through each lesson of the mission list, with his hands only, as the page stands the mission. Per step of the level's block: half a second
//! of doing nothing, and the step must still be up (a line moves on when the thing is done, never on a timer); what a player would do for it (`road.rs` and the rest: one file a tutorial); and the
//! step must have gone by. After the last: no row of the lesson's on the objective, and the run ends. A script reads the game to time a press and find a thing on the picture, and never reads
//! which step is up to act beyond what a person reads off the screen. He is as mortal as a player.
use crate::player::{at, flat, Feet, Player, Script, Scripted, Walk, LEFT, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::ops::Deref;
use core::pin::Pin;
use game::cmd::PlayerCmd;
use game::game::{Game, GuardState, InteractableKind, MissionStage, SlotKind, Verb};
use game::ids::{DoorId, FirearmId, GuardId, OrdersAssignmentId, SeatId};
use game::m;
use game::orders::Assignment;
use game::scene_scenarios::LessonStep;
use game::teams::Team;
use game::throwables::ItemKind;
use game::view::{LessonView, RailPrompt};

pub mod kit;
pub mod pair;
pub mod queue;
pub mod quiet;
pub mod road;
pub mod team;

/// a step's deed, a tutorial's own
pub type Deed<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;
/// what says the run is over
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ended { Stage, Finished }

/// a tutorial as its player has it
pub struct Tutorial {
    pub id: &'static str,
    /// steps that may already be true as they come up (each file says why). Their half second and their deed are run all the same
    pub idle_ok: &'static [&'static str],
    /// steps a fight on the floor may finish before he has done them. Such a step is logged as gone by with the fight
    pub fight_ok: &'static [&'static str],
    pub ended: Ended,
    pub play: for<'a> fn(&'a Bot, &'a str) -> Deed<'a>,
    /// after the last step: on to the mission's end
    pub finish: Option<for<'a> fn(&'a Bot) -> Deed<'a>>,
}
/// one such step lost to a fight is the fight; two in a row are lessons nobody played
const FIGHT_RUN: u32 = 1;
/// seconds a run may take
const CAP: f64 = 600.0;

pub fn tutorials() -> [&'static Tutorial; 6] { [&road::TUTORIAL, &team::TUTORIAL, &kit::TUTORIAL, &queue::TUTORIAL, &quiet::TUTORIAL, &pair::TUTORIAL] }

/// how a run went
#[derive(Clone, Debug, Default)]
pub struct Out {
    /// each step that went by, and when
    pub steps: Vec<(String, f64)>,
    /// the steps that went by with the fight before he got to them, with how long each was up
    pub by_fight: Vec<(String, f64)>,
    /// every step went by on its deed and the run ended
    pub ended: bool,
}

/// THE PLAYER OF A LESSON, as a host hands a seat over to him
pub struct Lesson {
    script: Scripted,
    pub me: Rc<Bot>,
    pub out: Rc<RefCell<Out>>,
}
impl Lesson {
    /// `pace`: seconds a reader spends on each line before he acts, on top of the half second of nothing
    pub fn new(seat: SeatId, t: &'static Tutorial, pace: f64) -> Lesson {
        let hands = Rc::new(Player::new(seat, Feet { search: 8000, corner: 0.55, stops: 0.1, moved: 0.15, stuck: 1.5, tries: 5, eyes_on_way: true }));
        let me = Rc::new(Bot { p: hands.clone(), kept: Cell::new(-1.0) });
        let out = Rc::new(RefCell::new(Out::default()));
        let up_at: Rc<RefCell<Vec<Option<f64>>>> = Rc::new(RefCell::new(Vec::new()));
        let (own, wrote, ups) = (me.clone(), out.clone(), up_at.clone());
        let mut guards: Vec<(GuardId, GuardState)> = Vec::new();
        let mut down_for = 0.0;
        let ups_w = up_at;
        let script = Scripted::new(hands, async move { run(&own, t, pace, &wrote, &ups).await }).watched(move |p| {
            // WHAT THE WHOLE RUN OWES, whichever line it is on (and `owes`, which the tests hold it to): he is not down for long, the run has an end
            let i = p.ask(|g| g.lesson.i) as usize;
            { let mut u = ups_w.borrow_mut(); if u.len() <= i { u.resize(i + 1, None); } if u[i].is_none() { u[i] = Some(p.t()); } }
            let moved: Vec<String> = p.ask(|g| g.guards.iter().filter_map(|&x| {
                let st = g.gd[x].state;
                match guards.iter_mut().find(|w| w.0 == x) {
                    Some(w) if w.1 != st => { let s = format!("   {}: {:?} -> {:?} at {}", g.gd[x].callsign, w.1, st, at(g.ch[g.gd[x].ch].pos)); w.1 = st; Some(s) }
                    Some(_) => None,
                    None => { guards.push((x, st)); None }
                }
            }).collect());
            for s in moved { p.say(s); }
            if p.seen(|g, s| game::condition::down(g, g.pl[s.player].ch)) {
                if down_for == 0.0 {
                    let by = p.ask(|g| g.chronicle.counts.down_by.clone());
                    let shooter = p.ask(|g| g.guards.iter().find(|&&x| by == g.gd[x].callsign).map(|&x| g.ch[g.gd[x].ch].pos));
                    p.say(format!("   !! the player is down at {}, by {} at {:?}; {}", at(p.pos()), by, shooter.map(at), p.ask(|g| g.guards.iter().filter(|&&x| !g.team(x).ours() && !game::condition::down(g, g.gd[x].ch)).map(|&x| format!("{} {:?} {} {}", g.gd[x].callsign, g.gd[x].state, at(g.ch[g.gd[x].ch].pos), if g.gd[x].script.is_some() { "script" } else { "own" })).collect::<Vec<_>>().join(", "))));
                }
                down_for += 1.0 / 60.0;
                if down_for > 2.0 { p.failure(format!("the player went down at {} (mission stage {})", at(p.pos()), p.ask(|g| g.mission.stage.as_str()))); }
            }
            if p.t() >= CAP { p.failure(format!("time cap ({} s) at {}, mission stage {}", CAP, at(p.pos()), p.ask(|g| g.mission.stage.as_str()))); }
        });
        Lesson { script, me, out }
    }
    /// why it failed (None: it has not)
    pub fn why(&self) -> Option<String> { self.script.me.failed() }
    pub fn trace(&self) -> String { self.script.me.trace() }
}
impl Script for Lesson {
    fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64) { self.script.hand(g, cmd, dt) }
    fn over(&self) -> bool { self.script.over() }
    fn lens(&mut self) -> [f64; 2] { self.script.lens() }
}

async fn run(b: &Bot, t: &'static Tutorial, pace: f64, out: &RefCell<Out>, up_at: &RefCell<Vec<Option<f64>>>) {
    b.ticks(20).await;   // (the start's first frames: poses, the deal's lines)
    let (ids, of): (Vec<String>, usize) = { let s = b.steps(); (s.iter().map(|x| x.id.clone()).collect(), s.len()) };
    let card_max = b.ask(|g| g.k.rails.card_max);
    let mut unplayed = 0;
    for (i, id) in ids.iter().enumerate() {
        let idle_ok = t.idle_ok.contains(&id.as_str());
        let fight_ok = t.fight_ok.contains(&id.as_str());
        let went = |wrote: &str| { b.say(format!("step {}/{} '{}' {}", i + 1, of, id, wrote)); out.borrow_mut().steps.push((id.clone(), b.t())); };
        if b.lesson_i() > i && fight_ok {
            let ups = up_at.borrow();
            let up = (ups.get(i + 1).copied().flatten().unwrap_or(b.t()) - ups.get(i).copied().flatten().unwrap_or(b.t())).max(0.0);
            drop(ups);
            out.borrow_mut().by_fight.push((id.clone(), up));
            unplayed += 1;
            went(&format!("went by with the fight before its turn (it was up {:.1} s)", up));
            if unplayed > FIGHT_RUN { b.fail(format!("{} steps in a row went by with the fight before the script reached them: lessons nobody played (the last: '{}', up {:.1} s)", unplayed, id, up)).await; }
            continue;
        }
        unplayed = 0;
        // (a step whose honest alternative an earlier deed already made true may have gone by when its turn comes: its half second, its reader's pause and its deed are run all the same)
        let gone = b.lesson_i() > i && idle_ok;
        if !gone && b.lesson_i() != i { let up = b.lesson().and_then(|v| v.id); b.fail(format!("step {} '{}' is not the step that is up (the lesson is on {}: {:?})", i + 1, id, b.lesson_i() + 1, up)).await; }
        b.ticks(30).await;   // half a second of nothing: the line must not move on by itself
        if (b.lesson_i() != i || b.settling()) && fight_ok { b.until(|| b.lesson_i() > i, card_max + 3.0).await; went("went by with the fight"); continue; }
        if (b.lesson_i() != i || b.settling()) && !idle_ok { b.fail(format!("step {} '{}' went by with nothing done (its test was already true when it came up)", i + 1, id)).await; }
        if pace > 0.0 { b.wait(pace).await; }   // (a step that only waits for the floor may well go by while he reads: its deed then finds its wait already over)
        b.say(format!("step {}/{} '{}'{}", i + 1, of, id, if gone { " (it had already gone by)" } else { "" }));
        (t.play)(b, id).await;
        b.ticks(30).await;
        // (a step before a card settles: its deed done, it stays up for the card's beat)
        if b.lesson_i() <= i && b.settling() { b.until(|| b.lesson_i() > i, card_max + 3.0).await; }
        if b.lesson_i() <= i { b.fail(format!("step {} '{}' did not go by after its script (player at {}, stage {})", i + 1, id, at(b.pos()), b.ask(|g| g.mission.stage.as_str()))).await; }
        out.borrow_mut().steps.push((id.clone(), b.t()));
        if b.is_down() { b.fail(format!("the player went down in step '{}'", id)).await; }
    }
    if !b.lesson().is_some_and(|v| v.finished) { b.fail(String::from("the lesson does not say it is finished after its last step")).await; }
    if b.seen(|g, s| game::lesson::lesson_objective(g, s).is_some()) { b.fail(String::from("the lesson still holds the objective row after its last step")).await; }
    b.say(String::from("every step done; playing the mission out"));
    let ended = || match t.ended { Ended::Finished => b.lesson().is_some_and(|v| v.finished), Ended::Stage => b.ask(|g| g.mission.stage == MissionStage::Done) };
    if !ended() { if let Some(f) = t.finish { f(b).await; } }
    if !b.until(ended, 5.0).await { b.fail(format!("the run did not end after the script's finish (stage {}, player at {}{})", b.ask(|g| g.mission.stage.as_str()), at(b.pos()), if b.is_down() { ", DOWN" } else { "" })).await; }
    b.say(String::from("ended"));
    out.borrow_mut().ended = true;
}

/// THE PLAYER, and what the tutorials' deeds ask of the game in common
pub struct Bot {
    p: Rc<Player>,
    /// a time a step keeps for a later one (the whistle's, for the man it brought)
    pub kept: Cell<f64>,
}
impl Deref for Bot { type Target = Player; fn deref(&self) -> &Player { &self.p } }

impl Bot {
    // ---- the lesson, as a HUD is told it
    pub fn steps(&self) -> Vec<LessonStep> { self.ask(|g| game::lesson::lesson_data(g).map_or(Vec::new(), |l| l.steps.clone())) }
    pub fn step(&self, id: &str) -> LessonStep { self.steps().into_iter().find(|t| t.id == id).unwrap_or_else(|| panic!("the block has no step '{}'", id)) }
    pub fn lesson_i(&self) -> usize { self.ask(|g| g.lesson.i) as usize }
    pub fn lesson(&self) -> Option<LessonView> { self.seen(|g, s| game::lesson::lesson_view(g, s)) }
    pub fn settling(&self) -> bool { self.lesson().is_some_and(|v| v.settling) }
    pub fn step_is(&self, id: &str) -> bool { self.lesson().is_some_and(|v| v.id.as_deref() == Some(id)) }
    pub fn prompt(&self) -> Option<RailPrompt> { self.lesson().and_then(|v| v.rails).and_then(|r| r.prompt) }
    /// the frame holds this action shut
    pub fn locked(&self, action: &str) -> bool { self.lesson().and_then(|v| v.rails).is_some_and(|r| !r.open.iter().any(|x| x == action)) }

    // ---- him
    pub fn me<R>(&self, f: impl FnOnce(&game::game::Player) -> R) -> R { self.seen(|g, s| f(&g.pl[s.player])) }
    pub fn is_down(&self) -> bool { self.seen(|g, s| g.cond[g.pl[s.player].ch].down) }
    pub fn on_wall(&self) -> bool { self.me(|p| p.wall.is_some()) }
    pub fn lit(&self) -> f64 { self.seen(|g, s| g.shown[g.pl[s.player].ch].visibility) }
    pub fn rifle(&self) -> FirearmId { self.me(|p| p.rifle) }
    /// the gun in his hands, or the rifle where none is
    pub fn gun(&self) -> FirearmId { self.me(|p| if p.slot == SlotKind::Pistol { p.pistol } else { p.rifle }) }
    pub fn rounds(&self) -> f64 { let w = self.gun(); self.ask(|g| game::weapons::firearm_rounds_ready(g, w)) }
    pub fn reloading(&self) -> bool { let w = self.gun(); self.ask(|g| game::weapons::firearm_reloading(g, w)) }
    pub fn holding(&self) -> bool { self.me(|p| p.holding.is_some()) }
    pub fn held_fast(&self) -> bool { self.me(|p| p.holding.as_ref().is_some_and(|h| h.phase == game::character::HoldPhase::Held)) }
    pub fn dark(&self) -> bool { self.ask(|g| g.light_home.mains.active) }
    pub fn has_code(&self, code: &str) -> bool { self.me(|p| p.codes.has(String::from(code))) }
    pub fn stage(&self) -> MissionStage { self.ask(|g| g.mission.stage) }

    // ---- men
    pub fn men(&self) -> Vec<GuardId> { self.ask(|g| g.guards.clone()) }
    pub fn up(&self, x: GuardId) -> bool { self.ask(|g| !game::condition::down(g, g.gd[x].ch)) }
    pub fn state(&self, x: GuardId) -> GuardState { self.ask(|g| g.gd[x].state) }
    pub fn awareness(&self, x: GuardId) -> f64 { self.ask(|g| g.gd[x].awareness) }
    pub fn called(&self, x: GuardId) -> String { self.ask(|g| g.gd[x].callsign.clone()) }
    pub fn yaw(&self, x: GuardId) -> f64 { self.ask(|g| g.ch[g.gd[x].ch].body_yaw) }
    pub fn scripted(&self, x: GuardId) -> bool { self.ask(|g| g.gd[x].script.is_some()) }
    /// the enemy, up or down
    pub fn foes(&self) -> Vec<GuardId> { self.ask(|g| g.guards.iter().copied().filter(|&x| !g.team(x).ours()).collect()) }
    pub fn downed(&self) -> usize { self.foes().into_iter().filter(|&x| !self.up(x)).count() }
    /// the man who walks the route of this name
    pub fn on_route(&self, name: &str) -> Option<GuardId> { self.ask(|g| g.guards.iter().copied().find(|&x| g.gd[x].unit.is_none() && g.tab.route[g.gd[x].route].name == name)) }
    pub fn named(&self, names: &[&str]) -> Vec<GuardId> { names.iter().filter_map(|n| self.ask(|g| g.guards.iter().copied().find(|&x| g.gd[x].callsign == *n))).collect() }
    /// his teammates still up
    pub fn squad(&self) -> Vec<GuardId> { self.ask(|g| g.guards.iter().copied().filter(|&x| g.team(x).ours() && !game::condition::down(g, g.gd[x].ch)).collect()) }
    pub fn chest(&self, x: GuardId) -> V3 { self.ask(|g| { let c = g.gd[x].ch; g.ch[c].bones.get(game::character::Bone::Chest).copied().unwrap_or([g.ch[c].pos[0], 1.25, g.ch[c].pos[2]]) }) }
    pub fn head(&self, x: GuardId) -> V3 { self.ask(|g| { let c = g.gd[x].ch; g.ch[c].bones.get(game::character::Bone::Head).copied().unwrap_or([g.ch[c].pos[0], 1.6, g.ch[c].pos[2]]) }) }
    /// he has a clear line from his head to the man's chest
    pub fn line(&self, x: GuardId) -> bool {
        let eye = self.seen(|g, s| { let c = g.pl[s.player].ch; g.ch[c].bones.get(game::character::Bone::Head).copied().unwrap_or([g.ch[c].pos[0], 1.5, g.ch[c].pos[2]]) });
        let to = self.chest(x);
        !self.world(|w| w.line(eye, to, game::ask::Line::Solid))
    }
    /// the spot `d` metres straight behind a man, as he faces now
    pub fn behind(&self, x: GuardId, d: f64) -> V3 { let (p, y) = (self.man_at(x), self.yaw(x)); [p[0] - m::sin(y) * d, 0.0, p[2] - m::cos(y) * d] }
    /// how many times the enemy has had eyes on him so far (the chronicle's own rows)
    pub fn sightings(&self) -> usize { self.ask(|g| g.chronicle.events.iter().filter(|e| matches!(e.kind, game::chronicle::ChronKind::Spotted | game::chronicle::ChronKind::Contact)).count()) }

    // ---- orders
    pub fn order_of(&self, x: GuardId) -> Option<Assignment> { self.seen(|g, s| game::orders::squad_orders_of(s, x).map(|a: OrdersAssignmentId| g.tab.orders_assignment[a].clone())) }
    pub fn ordered(&self, x: GuardId) -> String { self.order_of(x).map_or(String::from("-"), |a| a.order) }
    pub fn orders_log(&self) -> usize { self.seen(|_, s| s.orders.log.len()) }
    /// the last order on the book, and what it was given on
    pub fn last_order(&self) -> Option<(String, String)> { self.seen(|_, s| s.orders.log.last().map(|l| (l.order.clone(), l.target.clone()))) }
    pub fn orders_line(&self) -> String { self.squad().into_iter().map(|x| format!("{} {}", self.called(x), self.ordered(x))).collect::<Vec<_>>().join(" · ") }
    pub fn where_line(&self) -> String { self.squad().into_iter().map(|x| format!("{} {} {}", self.called(x), at(self.man_at(x)), self.ordered(x))).collect::<Vec<_>>().join(" · ") }
    pub fn stacked(&self) -> bool { self.squad().into_iter().all(|x| self.order_of(x).is_some_and(|a| a.order == "stack" && a.arrived)) }
    pub fn following(&self) -> bool { self.squad().into_iter().all(|x| self.ordered(x) == "fallIn") }
    pub fn men_in(&self, r: [f64; 4]) -> usize { self.squad().into_iter().filter(|&x| { let p = self.man_at(x); p[0] > r[0] && p[0] < r[1] && p[2] > r[2] && p[2] < r[3] }).count() }
    pub fn cleared(&self, room: &str) -> bool { self.ask(|g| g.staged.rooms.iter().any(|r| r.def.id == room && r.cleared)) }
    /// a room is theirs: cleared, or three of them are in it
    pub fn taken(&self, room: &str) -> bool {
        let b = self.ask(|g| g.staged.rooms.iter().find(|r| r.def.id == room).map(|r| (r.cleared, [r.def.bounds.x0, r.def.bounds.x1, r.def.bounds.z0, r.def.bounds.z1])));
        let Some((cleared, b)) = b else { return false };
        cleared || self.men_in(b) >= 3
    }
    /// the squad's key tapped on a point
    pub async fn command(&self, p: V3) {
        self.aim(p);
        self.ticks(2).await;
        self.aim(p);
        if !self.on_screen(p) { self.say(format!("  (the squad's key on {}, off the picture: the cursor stops at its edge)", at(p))); }
        self.press("KeyB", 2).await;
    }

    // ---- doors, things
    pub fn door(&self, name: &str) -> DoorId { self.ask(|g| g.doors.list.iter().copied().find(|&d| g.dr[d].def.name == name)).unwrap_or_else(|| panic!("the script names a door '{}' the level does not have", name)) }
    pub fn closed(&self, d: DoorId) -> bool { self.ask(|g| game::doors::door_is_closed(g, d)) }
    pub fn locked_door(&self, d: DoorId) -> bool { self.ask(|g| game::doors::door_locked(g, d)) }
    pub fn angle(&self, d: DoorId) -> f64 { self.ask(|g| game::doors::door_angle(g, d)) }
    pub fn thrown(&self, kind: ItemKind) -> bool { self.ask(|g| g.items.items.iter().any(|&i| g.tab.item[i].kind == kind && !g.tab.item[i].fired)) }
    /// the can in the air, until it has gone off (or `secs`)
    pub async fn landed(&self, kind: ItemKind, secs: f64) {
        let mut seen = false;
        for _ in 0..(secs * 60.0) as u32 { let it = self.thrown(kind); if seen && !it { break; } seen = seen || it; self.tick().await; }
        self.ticks(20).await
    }

    // ---- his feet, as the lessons walk: a shut door on the way is pushed
    pub async fn to(&self, p: V3, max: f64) -> bool { self.to_near(p, max, 0.3).await }
    pub async fn to_near(&self, p: V3, max: f64, near: f64) -> bool { self.to_by(p, max, Walk { near, ..Walk::doors() }, || false).await }
    pub async fn to_by(&self, p: V3, max: f64, o: Walk, stop: impl FnMut() -> bool) -> bool { let q = self.free(p).unwrap_or([p[0], 0.0, p[2]]); self.go(q, max, stop, o).await }

    // ---- his hands, as the lessons use them
    /// the trigger held on a place, with a fresh press every quarter second (a pistol fires a round a press), for `secs` or until `stop`
    pub async fn trigger(&self, mut on: impl FnMut() -> V3, secs: f64, mut stop: impl FnMut() -> bool) {
        self.aim(on());
        self.tick().await;
        self.click_now(LEFT);
        for i in 0..(secs * 60.0) as u32 {
            self.aim(on());
            self.button(LEFT, true);
            if i % 14 == 13 { self.click_now(LEFT); }
            self.tick().await;
            if stop() { break; }
        }
        self.button(LEFT, false);
        self.tick().await
    }
    /// shoot every man he has a line to, of `who`, until `n` are down, reloading as the gun runs dry, for `secs` at most; with no man to shoot, `idle`
    pub async fn fight(&self, n: usize, secs: f64, mut idle: impl AsyncFnMut(), who: Option<&[GuardId]>) -> bool {
        let t0 = self.t();
        while self.downed() < n && self.t() - t0 < secs && !self.is_down() {
            if self.rounds() == 0.0 && !self.reloading() { self.press("KeyR", 1).await; continue; }
            let me = self.pos();
            let list = who.map_or_else(|| self.foes(), |w| w.to_vec());
            let x = list.into_iter().filter(|&y| self.up(y) && self.line(y)).min_by(|&a, &b| flat(self.man_at(a), me).total_cmp(&flat(self.man_at(b), me)));
            let Some(x) = x else { idle().await; continue };
            if self.me(|p| p.slot) == SlotKind::Pistol {
                // (a round a press, once the gun has come round to the cursor: the aim has weight)
                for _ in 0..16 { if !self.up(x) { break; } self.aim(self.chest(x)); self.tick().await; }
                self.click(LEFT, Some(self.chest(x))).await;
                self.ticks(6).await;
            } else {
                self.trigger(|| self.chest(x), 1.2, || !self.up(x) || !self.line(x)).await;
            }
        }
        self.downed() >= n
    }
    /// after whoever is still up until `n` are down: toward the nearest until there is a line, then the fight's own aim and trigger
    pub async fn hunt(&self, n: usize, secs: f64) -> bool {
        self.fight(n, secs, async || {
            let Some(x) = self.foes().into_iter().find(|&y| self.up(y)) else { self.ticks(10).await; return };
            let p = self.man_at(x);
            self.to_by([p[0], 0.0, p[2]], 1.5, Walk::doors(), || self.line(x)).await;
        }, None).await
    }
    /// a slot's key, and the half second the thing takes to come into the hand (a click before that is not a throw)
    pub async fn take(&self, code: &str) {
        self.press(code, 5).await;
        self.until(|| self.me(|p| p.equip_t <= 0.0), 3.0).await;
        self.ticks(3).await
    }
    /// the row of that kind and verb worked; a grab where the only way to a man is the left button held on him: that
    pub async fn verb(&self, kind: InteractableKind, verb: Verb, aim: Option<V3>) -> bool {
        if self.row(kind, verb, aim).await { self.say(format!("  the row: {} > {}", kind.as_str(), verb.as_str())); return true; }
        if kind == InteractableKind::Guard && verb == Verb::Grab && !self.holding() {
            let me = self.pos();
            let near = self.men().into_iter().filter(|&x| self.up(x) && self.ask(|g| g.gd[x].civ.is_none()) && flat(me, self.man_at(x)) < 4.0).min_by(|&a, &b| flat(self.man_at(a), me).total_cmp(&flat(self.man_at(b), me)));
            if let Some(x) = near {
                for i in 0..300 {
                    if self.holding() { break; }
                    let c = self.man_at(x);
                    self.aim([c[0], 1.2, c[2]]);
                    self.button(LEFT, true);
                    if i == 0 { self.click_now(LEFT); }
                    self.tick().await;
                }
                self.button(LEFT, false);
                if self.holding() { self.say(String::from("  left button held: the stalk's grab")); return true; }
            }
        }
        let rows = self.seen(|_, s| s.nearby.iter().map(|r| format!("{}:{}", r.kind.as_str(), r.verb.map_or("-", |v| v.as_str()))).collect::<Vec<_>>().join(" "));
        self.say(format!("  no '{} > {}' row (rows: {})", kind.as_str(), verb.as_str(), rows));
        false
    }
    /// the interaction key TAPPED: the row the line means has to be the lit one (a line says the key, never 'hold it and choose')
    pub async fn tap(&self, kind: InteractableKind, verb: Verb, aim: Option<V3>) -> bool {
        let has = || self.seen(|_, s| s.nearby.iter().any(|r| r.kind == kind && r.verb == Some(verb) && r.in_reach));
        for _ in 0..60 { if has() { break; } if let Some(p) = aim { self.aim(p); } self.tick().await; }
        let lit = self.seen(|g, s| s.nearby.get(g.pl[s.player].near_sel.max(0) as usize).map(|r| (r.kind, r.verb)));
        if let (true, Some(l)) = (has(), lit) { if l != (kind, Some(verb)) { self.fail(format!("a tap of the interaction key here does '{}', and the line means '{}'", l.1.map_or("-", |v| v.as_str()), verb.as_str())).await; } }
        self.verb(kind, verb, aim).await
    }
    /// crouched up to a standing man's back, the last strides straight at him; true once in reach behind him
    pub async fn close_in(&self, x: GuardId, via: Option<V3>) -> bool {
        self.crouch(true).await;
        if let Some(v) = via { self.to(v, 40.0).await; }
        self.to_near(self.behind(x, 1.6), 20.0, 0.25).await;
        self.nudge(self.behind(x, 0.8), 6.0, || flat(self.pos(), self.man_at(x)) < 0.92).await;
        self.ticks(6).await;
        flat(self.pos(), self.man_at(x)) < 1.05
    }
    /// the enemy has not caught him: said by the chronicle and the man's own state
    pub async fn unseen(&self, n0: usize, x: GuardId, place: &str) {
        if self.sightings() > n0 || self.state(x) == GuardState::Alert { self.fail(format!("{} noticed the player {} ({:?}, aware {:.2})", self.called(x), place, self.state(x), self.awareness(x))).await; }
    }
    /// "[3] on it": the digits in brackets a line names, in order
    pub fn digits(line: &str) -> Vec<char> { let b: Vec<char> = line.chars().collect(); b.windows(3).filter(|w| w[0] == '[' && w[1].is_ascii_digit() && w[2] == ']').map(|w| w[1]).collect() }
    pub fn at_str(p: V3) -> String { at(p).to_string() }
}
