//! THE HANDS-OFF PLAYER: a staged room clear played from its first step to its end with nobody at the keys. He is a seat's player as any other is: what he does is the
//! hand the next tick is handed (the walk keys along the floor's way, the cursor on a door and the quick key pressed, then the word's key, his rifle on whoever
//! the picture shows him, and a hunt for whoever is left), written over the one the host sampled, through the host's own camera. Of the game he only asks.
//!
//! He knows two things a player does not: which rooms are held, and where the men left in them stand. He never reads how many are behind a door, and orders nothing but
//! through the keys. The beat between a man showing and the first round is rolled on a die of his own.
//!
//! He is a script of the one scripted player (`player.rs`).
use crate::player::{at, flat, Feet, Player, Script, Walk, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;
use core::future::Future;
use core::ops::Deref;
use core::pin::Pin;
use core::task::{Context, Waker};
use game::cmd::PlayerCmd;
use game::dice::Die;
use game::game::{Game, InteractableKind, Verb};
use game::ids::{DoorId, GuardId, RoomfightRoomId, SeatId};
use game::orders::OrderTarget;
use game::roomfight::Entry;
use game::teams::Team;

/// what the hands do beyond taking each held room by its nearest door
#[derive(Clone, Debug, Default)]
pub struct Plan {
    /// the die his beats are rolled on
    pub seed: u32,
    /// seconds of the run after which he stops
    pub cap: f64,
    /// doors by name: a held room that has one is taken first, by that door, in this order
    pub route: Vec<String>,
    /// before the rooms a teammate is sent to throw the breaker, and he waits for the dark
    pub breaker: bool,
}
/// no defender left · he is down · out of time · nothing left to try
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End { Taken, Sam, Cap, Spent }
impl End { pub fn as_str(self) -> &'static str { match self { End::Taken => "taken", End::Sam => "sam", End::Cap => "cap", End::Spent => "spent" } } }

/// seconds from a man showing to the first round
const REACT: f64 = 0.6;
/// he holds his round with a mate this near its line
const MATE_M: f64 = 0.6;

/// a man as he has been shown to him: first, last, and the beat he needs
#[derive(Clone, Copy)]
struct Shown { first: f64, last: f64, need: f64 }

/// the player, and what this script keeps beside him
struct Me {
    p: Player,
    target: Cell<Option<GuardId>>,
    given: Cell<u32>,
}
impl Deref for Me { type Target = Player; fn deref(&self) -> &Player { &self.p } }
impl Me {
    /// his teammates still up
    fn up(&self) -> Vec<GuardId> { self.seen(|g, s| game::orders::squad_orders_squad(g, s).into_iter().filter(|&m| !game::condition::down(g, g.gd[m].ch)).collect()) }
    /// defenders still up
    fn left(&self) -> Vec<GuardId> { self.ask(left) }
    fn orders(&self) -> usize { self.seen(|_, s| s.orders.log.len()) }
    fn order_given(&self, n: usize) -> String { self.seen(|_, s| if s.orders.log.len() > n { s.orders.log[s.orders.log.len() - 1].order.clone() } else { String::from("nothing given") }) }
    fn counted(&self, n: usize) -> bool { let given = self.orders() > n; if given { self.given.set(self.given.get() + 1); } given }
    fn ops_on(&self, d: DoorId) -> bool { self.seen(|g, s| s.door_tools.ops.iter().any(|&o| g.tab.doortools_door_op[o].door == d)) }

    async fn quick(&self) { let k = self.ask(|g| g.sets[g.seat].orders.knobs.quick_key.clone()); self.key(&k).await }

    async fn go(&self, p: V3, max: f64, stop: impl FnMut() -> bool) -> bool { self.p.go(p, max, stop, Walk::default()).await }

    // ---- the floor
    /// the rooms still held, and who is at home
    fn held(&self) -> Vec<Held> {
        self.ask(|g| {
            let mut out: Vec<Held> = Vec::new();
            for &d in &g.fight.defs {
                let def = &g.tab.roomfight_def[d];
                let Some(room) = def.room else { continue };
                let r = &g.tab.roomfight_room[room];
                if r.entries.is_empty() || r.cells.len() > 500 || game::condition::down(g, g.gd[def.gd].ch) { continue; }
                match out.iter_mut().find(|h| h.room == room) { Some(h) => h.men.push(def.gd), None => out.push(Held { room, entries: r.entries.clone(), men: vec![def.gd] }) }
            }
            out
        })
    }
    /// where a player stands to work a door: a few metres from it on its own side, off its line where the floor allows
    fn post(&self, e: &Entry) -> V3 {
        let (gate, me) = (out_pt(e, 0.7, 0.0), self.pos());
        // (beside the door's own wall, a few metres along it, where nobody in the room has a line to him through the opening)
        let inn = [out_pt(e, -1.5, 0.0), out_pt(e, -3.0, 1.0), out_pt(e, -3.0, -1.0)];
        let mut post: Option<(f64, V3)> = None;
        for (back, lat) in [(0.9, 4.2), (0.9, -4.2), (0.9, 5.4), (0.9, -5.4), (1.6, 4.8), (1.6, -4.8), (0.9, 3.2), (0.9, -3.2), (2.4, 5.5), (2.4, -5.5)] {
            let Some(p) = self.free(out_pt(e, back, lat)) else { continue };
            if flat(p, e.at) < 2.8 || flat(p, e.at) > 7.5 || !inn.iter().all(|&q| self.solid(p, q)) { continue; }
            if self.way_len(gate, p) > flat(gate, p) * 1.5 + 1.0 { continue; }
            let l = self.way_len(me, p);
            if post.is_none_or(|b| l < b.0) { post = Some((l, p)); }
        }
        if let Some(p) = post { return p.1; }
        for wide in [1.2, 0.0] {
            for (back, lat) in [(3.4f64, 1.6f64), (3.4, -1.6), (4.4, 1.2), (4.4, -1.2), (2.8, 2.2), (2.8, -2.2), (2.0, 2.8), (2.0, -2.8), (1.3, 3.2), (1.3, -3.2), (5.2, 0.6), (3.6, 0.5), (3.6, -0.5), (2.6, 0.0)] {
                if lat.abs() < wide { continue; }
                let Some(p) = self.free(out_pt(e, back, lat)) else { continue };
                if self.solid(p, gate) || flat(p, e.at) < 2.2 || self.way_len(gate, p) > flat(gate, p) * 1.4 + 0.8 { continue; }
                let l = self.way_len(me, p);
                if post.is_none_or(|b| l < b.0) { post = Some((l, p)); }
            }
            if let Some(p) = post { return p.1; }
        }
        self.free(out_pt(e, 2.5, 0.0)).unwrap_or(out_pt(e, 2.5, 0.0))
    }
    fn inside(&self, e: &Entry) -> Option<V3> { [2.5, 2.0, 1.5].into_iter().map(|b| out_pt(e, -b, 0.0)).find(|p| !self.ask(|g| game::scene_collision::nav_grid_is_blocked(g, p[0], p[2]))) }
    fn door_at(&self, d: DoorId, y: f64) -> V3 { self.ask(|g| { let c = g.dr[d].frame_centre; [c[0], y, c[2]] }) }
    fn sees(&self, d: DoorId) -> bool { [1.0, 0.1, 1.8].into_iter().any(|y| self.on_screen(self.door_at(d, y))) }
    /// the cursor on that door, as the keys read it
    async fn cursor_on(&self, d: DoorId) -> bool {
        for y in [1.0, 0.1, 1.8] {
            let p = self.door_at(d, y);
            if !self.on_screen(p) { continue; }
            self.aim(p);
            self.ticks(2).await;
            if let OrderTarget::Door(t) = self.under() { if t.door == d { return true; } }
        }
        false
    }

    // ---- the quick key
    async fn quick_on(&self, d: DoorId, why: &str) -> bool {
        if !self.cursor_on(d).await { self.say(format!("(quick · {}: the door is not under the cursor)", self.name(d))); return false; }
        let n = self.orders();
        self.quick().await;
        self.say(format!("quick · {} ({}) → {}", self.name(d), why, self.order_given(n)));
        self.aim_off();
        self.counted(n)
    }
    /// the men sent in through a door they are stacked on: the quick key, or the word's where the quick key is 'stack up' again on such a door
    async fn enter(&self, d: DoorId, why: &str) -> bool {
        let set = self.cursor_on(d).await && { let t = self.under(); self.seen(|g, s| game::autopilot::auto_pick(g, s, &t, Some(false), None).why == game::autopilot::AutoPickWhy::Set) };
        if !set { return self.quick_on(d, why).await; }
        let (n, word) = (self.orders(), self.ask(|g| g.k.doortools.blow_key.clone()));
        self.key(&word).await;
        self.say(format!("word · {} ({}) → {}", self.name(d), why, self.order_given(n)));
        self.aim_off();
        self.orders() > n || self.ops_on(d)
    }
    /// the cursor on his own feet with no teammate under it: what the quick key calls 'nothing', which is fall in
    async fn on_self(&self) -> bool {
        for (ox, oz) in [(0.0, 0.0), (0.25, 0.0), (-0.25, 0.0), (0.0, 0.25), (0.0, -0.25)] {
            let p = self.pos();
            self.aim([p[0] + ox, 0.05, p[2] + oz]);
            self.ticks(2).await;
            if let OrderTarget::None(t) = self.under() { if t.man.is_none() { return true; } }
        }
        false
    }
    async fn fall_in(&self) {
        if self.up().is_empty() { return; }
        self.on_self().await;
        let n = self.orders();
        self.quick().await;
        self.aim_off();
        self.say(format!("quick · on himself → {}", self.order_given(n)));
        self.counted(n);
    }
    /// one held room by the quick key alone: the men stack and go first, he walks to a post behind them, and in after them
    async fn squad_room(&self, h: &Held, e: &Entry) {
        let (d, post) = (e.door, self.post(e));
        self.say(format!("→ {} ({} at home), from {}", self.name(d), h.men.len(), at(post)));
        let mut sent = false;
        if !self.sees(d) { self.fall_in().await; self.wait(1.0).await; self.go(post, 60.0, || self.sees(d)).await; }
        for _ in 0..4 { if sent { break; } sent = self.quick_on(d, "stack").await; if !sent { self.go(post, 2.0, || false).await; } }
        self.wait(2.5).await;
        if !self.go(post, 60.0, || false).await { self.say(format!("(he did not reach his post: at {})", at(self.pos()))); }
        if !sent { self.quick_on(d, "stack").await; }
        self.until(|| { let up = self.up(); self.seen(|g, s| up.iter().all(|&gd| game::orders::squad_orders_of(s, gd).is_some_and(|a| g.tab.orders_assignment[a].arrived))) }, 25.0).await;
        for k in 0..4 {
            if !h.held(self) || self.up().is_empty() { break; }
            if !self.enter(d, if k > 0 { "again" } else { "go in" }).await { self.wait(1.0).await; continue; }
            self.until(|| self.ops_on(d), 3.0).await;
            self.until(|| !self.ops_on(d), 70.0).await;
            self.wait(1.5).await;
        }
        self.wait(1.0).await;
        if let Some(inn) = self.inside(e) { self.go(inn, 25.0, || false).await; }
        self.wait(1.0).await;
    }
    /// a held room with no teammate left to send: to the door, through it, and to each man in it
    async fn solo_room(&self, h: &Held, e: &Entry) {
        let d = e.door;
        self.say(format!("→ {} himself ({} at home)", self.name(d), h.men.len()));
        let face = out_pt(e, 0.9, 0.0);
        self.go(self.free(face).unwrap_or(face), 60.0, || false).await;
        let shut = || self.ask(|g| g.dr[d].leaf.locked && !g.dr[d].leaf.lock_broken && g.dr[d].leaf.angle.abs() < 0.3);
        if shut() {
            // (the highlight walked to the boot by its key, as a hand walks it)
            let end = self.t() + 2.0;
            while self.t() < end && shut() {
                let on = self.seen(|_, s| s.nearby.get(s.hand.near_sel.max(0) as usize).is_some_and(|r| r.kind == InteractableKind::Door && r.verb == Some(Verb::Kick)));
                self.down(if on { game::consts::INTERACT_KEY } else { "ArrowDown" });
                if on { self.until(|| !shut(), 2.5).await; break; }
                self.ticks(3).await;
            }
        }
        if let Some(inn) = self.inside(e) { self.go(inn, 20.0, || !h.held(self)).await; }
        for _ in 0..4 {
            let me = self.pos();
            let Some(m) = h.men.iter().copied().filter(|&m| !self.dead(m)).min_by(|&a, &b| flat(self.man_at(a), me).total_cmp(&flat(self.man_at(b), me))) else { break };
            self.go(self.man_at(m), 8.0, || self.dead(m) || self.target.get().is_some()).await;
            self.until(|| self.target.get().is_none(), 6.0).await;
            self.wait(0.3).await;
        }
    }
    /// the breaker thrown by a teammate: the quick key on its panel, then the wait for the dark
    async fn breaker(&self) {
        let dark = || self.ask(|g| g.light_home.mains.active);
        if dark() || self.up().is_empty() { return; }
        let p = self.ask(|g| g.level.breaker.pos);
        if p == [0.0; 3] { return; }
        let foot = self.free(p).unwrap_or(p);
        self.say(format!("→ the breaker at {}", at(p)));
        if !self.on_screen(p) {
            self.fall_in().await;
            self.wait(1.0).await;
            let near = slow(self, || self.on_screen(p) && self.way_len(self.pos(), foot) < 12.0);
            self.go(foot, 60.0, near).await;
        }
        let mut given = false;
        for q in [p, [foot[0], 0.05, foot[2]]] {
            if !self.on_screen(q) { continue; }
            self.aim(q);
            self.ticks(2).await;
            if !matches!(self.under(), OrderTarget::Floor(t) if t.thing.is_some()) { continue; }
            let n = self.orders();
            self.quick().await;
            self.say(format!("quick · the breaker → {}", self.order_given(n)));
            given = self.counted(n);
            if given { break; }
        }
        self.aim_off();
        if !given { self.say(String::from("(the breaker is not under the cursor: the rooms are taken lit)")); return; }
        if !self.until(dark, 45.0).await { self.say(String::from("(no dark in 45 s)")); }
        self.wait(1.5).await;
    }
    async fn play(&self, plan: &Plan) {
        self.ticks(20).await;
        if plan.breaker { self.breaker().await; }
        let mut tried: Vec<RoomfightRoomId> = Vec::new();
        let times = |tried: &[RoomfightRoomId], r: RoomfightRoomId| tried.iter().filter(|&&x| x == r).count();
        for _ in 0..24 {
            let held: Vec<Held> = self.held().into_iter().filter(|h| times(&tried, h.room) < 3).collect();
            if held.is_empty() { break; }
            let mut best: Option<(usize, Entry)> = None;
            'route: for name in &plan.route {
                for (i, h) in held.iter().enumerate() {
                    if times(&tried, h.room) > 0 { continue; }
                    if let Some(e) = h.entries.iter().find(|e| self.name(e.door) == *name) { best = Some((i, *e)); break 'route; }
                }
            }
            if best.is_none() {
                let (me, mut least) = (self.pos(), f64::INFINITY);
                for (i, h) in held.iter().enumerate() { for e in &h.entries {
                    let o = out_pt(e, 1.5, 0.0);
                    let l = self.way_len(me, self.free(o).unwrap_or(o));
                    if l < least { least = l; best = Some((i, *e)); }
                } }
            }
            let Some((i, e)) = best else { self.say(String::from("no way on foot to any held room")); break };
            tried.push(held[i].room);
            if self.up().is_empty() { self.solo_room(&held[i], &e).await } else { self.squad_room(&held[i], &e).await }
        }
        // ---- whoever is left is not at home
        for _ in 0..3 {
            for h in self.left() {
                let hp = self.man_at(h);
                let Some(way) = self.way(self.pos(), hp) else { continue };
                self.say(format!("hunt → {} {}", self.ask(|g| g.gd[h].callsign.clone()), at(hp)));
                if self.up().is_empty() {
                    self.go(hp, 30.0, || self.dead(h) || self.target.get().is_some()).await;
                    self.until(|| self.dead(h) || self.target.get().is_none(), 8.0).await;
                    continue;
                }
                self.fall_in().await;
                let me = self.pos();
                let path: Vec<V3> = core::iter::once([me[0], 0.0, me[2]]).chain(way).chain(core::iter::once([hp[0], 0.0, hp[2]])).collect();
                let (mut back, mut p) = (5.0, None);
                for w in path.windows(2).rev() {
                    let l = flat(w[0], w[1]);
                    if l >= back { let k = back / if l > 0.0 { l } else { 1.0 }; p = Some([w[1][0] + (w[0][0] - w[1][0]) * k, 0.0, w[1][2] + (w[0][2] - w[1][2]) * k]); break; }
                    back -= l;
                }
                // (… from where his floor first comes into the picture: a commander does not walk onto him)
                let mut near = slow(self, || { let q = self.man_at(h); self.on_screen([q[0], 0.05, q[2]]) && self.way_len(self.pos(), q) < 14.0 });
                if let Some(p) = p { self.go(self.free(p).unwrap_or(p), 40.0, || self.dead(h) || near()).await; }
                if !self.dead(h) && !self.up().is_empty() {
                    let q = self.man_at(h);
                    self.aim([q[0], 0.05, q[2]]);
                    self.ticks(2).await;
                    let n = self.orders();
                    self.quick().await;
                    self.say(format!("quick · the floor by {} → {}", self.ask(|g| g.gd[h].callsign.clone()), self.order_given(n)));
                    self.aim_off();
                    self.until(|| self.dead(h), 14.0).await;
                }
            }
        }
        self.wait(15.0).await;
    }
}

struct Held { room: RoomfightRoomId, entries: Vec<Entry>, men: Vec<GuardId> }
impl Held { fn held(&self, me: &Me) -> bool { self.men.iter().any(|&m| !me.dead(m)) } }

fn left(g: &Game) -> Vec<GuardId> { g.guards.iter().copied().filter(|&m| g.team(m) == Team::Hostile && !game::condition::down(g, g.gd[m].ch)).collect() }
fn out_pt(e: &Entry, back: f64, lat: f64) -> V3 { [e.at[0] + e.out[0] * back - e.out[1] * lat, 0.0, e.at[2] + e.out[1] * back + e.out[0] * lat] }
/// a question that costs a search of the floor, asked four times a second and not every tick
fn slow<'a>(me: &'a Me, mut f: impl FnMut() -> bool + 'a) -> impl FnMut() -> bool + 'a {
    let (mut next, mut v) = (-1.0, false);
    move || { if me.t() >= next { next = me.t() + 0.25; v = f(); } v }
}

pub struct HandsOff {
    me: Rc<Me>,
    script: Pin<Box<dyn Future<Output = ()>>>,
    cap: f64,
    die: Die,
    shown: Vec<(GuardId, Shown)>,
    last: Option<GuardId>,
    last_t: f64,
    busy: f64,
    end: Option<End>,
    /// when a round last left a gun
    shot_at: f64,
    heard: f64,
}
impl HandsOff {
    pub fn new(seat: SeatId, plan: Plan) -> HandsOff {
        let me = Rc::new(Me { p: Player::new(seat, Feet::default()), target: Cell::new(None), given: Cell::new(0) });
        let (cap, seed, own) = (plan.cap, plan.seed, me.clone());
        HandsOff {
            me, script: Box::pin(async move { own.play(&plan).await }), cap, die: Die(0xb07u32.wrapping_add(seed.wrapping_mul(131))), shown: Vec::new(), last: None, last_t: -9.0, busy: 0.0, end: None,
            shot_at: -9.0, heard: -1.0,
        }
    }
    pub fn end(&self) -> Option<End> { self.end }
    pub fn t(&self) -> f64 { self.me.t() }
    /// presses of the squad's keys that gave an order
    pub fn given(&self) -> u32 { self.me.given.get() }
    pub fn trace(&self) -> String { self.me.p.trace() }
    pub fn shot_at(&self) -> f64 { self.shot_at }
    pub fn stop(&mut self, end: End) {
        if self.end.is_some() { return; }
        self.end = Some(end);
        self.me.rest();
        self.me.target.set(None);
        self.me.say(format!("ended: {}", end.as_str()));
    }

    /// THE HAND OF THE TICK ABOUT TO RUN: what the host sampled, with his keys, his button and his cursor over it.
    pub fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64) {
        cmd.keys.retain(|k| !game::player::WALK_KEYS.contains(&k.as_str()));
        cmd.buttons = (cmd.buttons as u32 & !1) as f64;
        if self.end.is_some() { return; }
        for e in g.events.iter().filter(|e| e.time > self.heard) { if matches!(e.kind, game::director::EventKind::GuardShot) || (matches!(e.kind, game::director::EventKind::Shot) && e.loud) { self.shot_at = self.t(); } }
        self.heard = g.time;
        let me = self.me.clone();
        let pl = game::seats::seat(g, me.seat).player;
        if game::condition::down(g, g.pl[pl].ch) { return self.stop(End::Sam); }
        if g.room_clear.end == Some(game::roomclear::RoomClearEnd::Taken) || (self.t() > 0.0 && left(g).is_empty()) { return self.stop(End::Taken); }
        if self.t() >= self.cap { return self.stop(End::Cap); }
        me.open(g, cmd);
        let done = self.script.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_ready();
        if done { me.shut(); return self.stop(End::Spent); }
        self.aim(g, cmd);
        me.close(cmd, dt);
    }
    /// his hands, once a tick: the walk keys toward the goal, the cursor, the trigger
    fn aim(&mut self, g: &Game, cmd: &mut PlayerCmd) {
        let (me, t) = (self.me.clone(), self.t());
        let seat = game::seats::seat(g, me.seat);
        let pl = &g.pl[seat.player];
        let pos = g.ch[pl.ch].pos;
        let chest = |gd: GuardId| { let c = g.gd[gd].ch; g.ch[c].bones.get(game::character::Bone::Chest).copied().unwrap_or([g.ch[c].pos[0], 1.2, g.ch[c].pos[2]]) };
        // (he shoots only a man the fog shows, on the screen, with nothing solid between them and no whole pane, a beat after he shows)
        let eye = [pos[0], 1.45, pos[2]];
        let mut best: Option<(f64, GuardId)> = None;
        for h in g.guards.iter().copied().filter(|&h| g.team(h) == Team::Hostile) {
            if game::condition::down(g, g.gd[h].ch) { self.shown.retain(|s| s.0 != h); continue; }
            let (c, d) = (chest(h), flat(pos, g.ch[g.gd[h].ch].pos));
            if game::fogofwar::fog_of_war_hides(g, h) || !me.on_screen(c) || game::scene_collision::solid_blocked(g, eye, c, None) { continue; }
            let fresh = !self.shown.iter().any(|s| s.0 == h && t - s.1.last <= 1.0);
            if fresh { self.shown.retain(|s| s.0 != h); let need = REACT * (0.75 + 0.5 * self.die.roll()); self.shown.push((h, Shown { first: t, last: t, need })); }
            let s = &mut self.shown.iter_mut().find(|s| s.0 == h).unwrap().1;
            s.last = t;
            if t - s.first >= s.need && best.is_none_or(|b| d < b.0) { best = Some((d, h)); }
        }
        let mut best = best.map(|b| b.1);
        if best.is_some() && best != self.last && t - self.last_t < 0.5 { self.busy = t + REACT / 2.0; }
        if best.is_some() { self.last = best; self.last_t = t; }
        if t < self.busy { best = None; }
        me.target.set(best);
        me.set_busy(best.is_some());
        me.hands(g, cmd, best.map(chest));
        // (and not with one of his own in the line)
        let mate_in_line = best.is_some_and(|h| {
            let (a, b) = (pos, g.ch[g.gd[h].ch].pos);
            let l = flat(a, b);
            me.up().into_iter().any(|m| {
                let p = g.ch[g.gd[m].ch].pos;
                let along = ((p[0] - a[0]) * (b[0] - a[0]) + (p[2] - a[2]) * (b[2] - a[2])) / l.max(1e-6);
                along > 0.0 && along < l && flat(p, [a[0] + (b[0] - a[0]) * along / l, 0.0, a[2] + (b[2] - a[2]) * along / l]) < MATE_M
            })
        });
        if best.is_some() && !mate_in_line && seat.aim_guard == best && !pl.sprinting { cmd.clicked = (cmd.clicked as u32 | 1) as f64; }
        if game::weapons::firearm_rounds_ready(g, pl.rifle) == 0.0 && !game::weapons::firearm_reloading(g, pl.rifle) { me.down("KeyR"); }
    }
}
impl Script for HandsOff {
    fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64) { HandsOff::hand(self, g, cmd, dt) }
    fn over(&self) -> bool { self.end.is_some() }
}
