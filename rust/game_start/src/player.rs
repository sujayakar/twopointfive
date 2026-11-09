//! A PLAYER OF A SEAT, AS A SCRIPT PLAYS ONE: what he does is the hand the next tick is handed (keys held and pressed, buttons, the wheel, the cursor), written over the one the host sampled, through
//! the host's own camera. Of the game he only asks. The hands-off player (`hands_off.rs`) and the lessons' players (`lessons/`) are scripts of this one player.
//!
//! A SCRIPT IS ASYNC CODE WITH NO EXECUTOR: an await is 'one tick passes', polled once a tick. It asks the game through `Player::ask`, whose borrow cannot outlive the question. A script that
//! fails unwinds nothing: `fail` leaves the words on the player and never comes back, and whoever polls him drops the script there.
use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use game::ask::{Line, WorldAsk};
use game::cmd::PlayerCmd;
use game::game::{Game, InteractableKind, Verb};
use game::ids::{DoorId, GuardId, SeatId};
use game::m;
use game::orders::OrderTarget;
use game::view::HostNow;

pub type V3 = [f64; 3];

pub const MOVE: [&str; 4] = ["KeyW", "KeyA", "KeyS", "KeyD"];
/// the mouse's bits in a hand
pub const LEFT: u32 = 1;
pub const RIGHT: u32 = 4;

pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }
pub fn at(p: V3) -> String { format!("({:.1}, {:.1})", p[0], p[2]) }

/// HOW A SCRIPT'S FEET WALK: the numbers its author played it with
#[derive(Clone, Copy, Debug)]
pub struct Feet {
    /// how many cells a search of the floor may open
    pub search: u32,
    /// how near a corner of the way counts as reached
    pub corner: f64,
    /// nearer to where he is sent than this, no key goes down
    pub stops: f64,
    /// a man who has not moved this far in `stuck` seconds is stuck, and takes a step aside and a fresh way, `tries` times
    pub moved: f64,
    pub stuck: f64,
    pub tries: u32,
    /// the cursor goes along the way, on each corner of it (else it stays where it is left, or a few metres ahead of him)
    pub eyes_on_way: bool,
}
impl Default for Feet { fn default() -> Feet { Feet { search: 6000, corner: 0.6, stops: 0.2, moved: 0.25, stuck: 1.2, tries: 8, eyes_on_way: false } } }

/// HOW A WALK GOES beside where to and for how long
#[derive(Clone, Copy, Debug)]
pub struct Walk {
    /// how near the end of it counts as there
    pub near: f64,
    pub sprint: bool,
    /// a shut door on the way is opened, by this row (else he walks at it)
    pub door: Option<Verb>,
}
impl Default for Walk { fn default() -> Walk { Walk { near: 0.3, sprint: false, door: None } } }
impl Walk {
    pub fn near(near: f64) -> Walk { Walk { near, ..Walk::default() } }
    pub fn doors() -> Walk { Walk { door: Some(Verb::Push), ..Walk::default() } }
}

/// what the script and the hand share
pub struct Player {
    game: Cell<*const Game>,
    pub seat: SeatId,
    feet: Feet,
    /// the lens and the canvas of the hand being made
    now: RefCell<HostNow>,
    ahead: Cell<[f64; 2]>,
    t: Cell<f64>,
    goal: Cell<Option<V3>>,
    look: Cell<Option<V3>>,
    /// where he has put the cursor
    cursor: Cell<Option<[f64; 2]>>,
    press: RefCell<Vec<String>>,
    held: RefCell<Vec<String>>,
    buttons: Cell<u32>,
    clicks: Cell<u32>,
    wheel: Cell<f64>,
    sprint: Cell<bool>,
    /// his hands are on something else and his feet are stopped for it: a walk that is not moving is not stuck
    busy: Cell<bool>,
    quick_at: Cell<f64>,
    /// the view turned and pulled back, for the host to do before the next hand is sampled
    lens: Cell<[f64; 2]>,
    trace: RefCell<Vec<String>>,
    failed: RefCell<Option<String>>,
}

impl Player {
    pub fn new(seat: SeatId, feet: Feet) -> Player {
        Player {
            game: Cell::new(core::ptr::null()), seat, feet, now: RefCell::new(HostNow::default()), ahead: Cell::new([0.0, 1.0]), t: Cell::new(0.0), goal: Cell::new(None), look: Cell::new(None), cursor: Cell::new(None),
            press: RefCell::new(Vec::new()), held: RefCell::new(Vec::new()), buttons: Cell::new(0), clicks: Cell::new(0), wheel: Cell::new(0.0), sprint: Cell::new(false), busy: Cell::new(false), quick_at: Cell::new(-9.0),
            lens: Cell::new([0.0, 0.0]), trace: RefCell::new(Vec::new()), failed: RefCell::new(None),
        }
    }

    // ---- the hand: `open`, the script polled, `hands`, `close`, once a tick
    /// the hand is being made on this game and this sampled hand: it stands for as long as the script is polled
    pub fn open(&self, g: &Game, cmd: &mut PlayerCmd) {
        if let Some(c) = self.cursor.get() { (cmd.mx, cmd.my) = (c[0], c[1]); }
        let m16 = |v: &[f64]| -> [f32; 16] { core::array::from_fn(|i| v[i] as f32) };
        *self.now.borrow_mut() = HostNow { mouse: [cmd.mx, cmd.my], client: [cmd.w, cmd.h], view_proj: m16(&cmd.view.vp), inv_view_proj: m16(&cmd.view.ivp) };
        { let f = cmd.view.forward; let l = m::hyp2(f[0], f[2]); if l > 0.0 { self.ahead.set([f[0] / l, f[2] / l]); } }
        self.game.set(g);
    }
    /// what he has asked of his hands, on the hand of this tick. `over`: a point his eyes are on whatever the script says, and his feet stop for it
    pub fn hands(&self, g: &Game, cmd: &mut PlayerCmd, over: Option<V3>) {
        let pos = g.ch[g.pl[game::seats::seat(g, self.seat).player].ch].pos;
        let mut ahead = None;
        if let Some(goal) = self.goal.get() {
            let (dx, dz) = (goal[0] - pos[0], goal[2] - pos[2]);
            let d = m::hyp2(dx, dz);
            if d > self.feet.stops {
                ahead = Some([pos[0] + dx / d * 6.0, 1.1, pos[2] + dz / d * 6.0]);
                if over.is_none() {
                    let f = self.ahead.get();
                    let (a, b) = ((dx * f[0] + dz * f[1]) / d, (dx * -f[1] + dz * f[0]) / d);
                    for (on, k) in [(a > 0.3827, 0), (b < -0.3827, 1), (a < -0.3827, 2), (b > 0.3827, 3)] { if on { cmd.keys.push(String::from(MOVE[k])); } }
                }
            }
            if self.sprint.get() { cmd.keys.push(String::from("ShiftLeft")); }
        }
        let look = over.or(self.look.get()).or(ahead);
        if let Some(p) = look {
            let (x, y, front) = game::camera::project(&self.now.borrow().view_proj, p, cmd.w, cmd.h, None, None);
            if front { cmd.mx = x.clamp(10.0, (cmd.w - 10.0).max(10.0)); cmd.my = y.clamp(10.0, (cmd.h - 10.0).max(10.0)); self.cursor.set(Some([cmd.mx, cmd.my])); }
        }
        cmd.keys.extend(self.held.borrow().iter().cloned());
        cmd.buttons = (cmd.buttons as u32 | self.buttons.get()) as f64;
        cmd.clicked = (cmd.clicked as u32 | self.clicks.replace(0)) as f64;
        cmd.wheel += self.wheel.replace(0.0);
    }
    /// the hand is made: the game is out of his reach until the next, and the tick's time passes
    pub fn close(&self, cmd: &mut PlayerCmd, dt: f64) {
        self.shut();
        cmd.pressed.append(&mut self.press.borrow_mut());
        self.t.set(self.t.get() + dt);
    }
    /// the game out of his reach
    pub fn shut(&self) { self.game.set(core::ptr::null()); }
    /// he stops where he stands, with the cursor let be
    pub fn rest(&self) { self.goal.set(None); self.look.set(None); }
    pub fn take_lens(&self) -> [f64; 2] { self.lens.replace([0.0, 0.0]) }

    // ---- what he asks
    /// a question of the game, as the last tick left it
    pub fn ask<R>(&self, f: impl FnOnce(&Game) -> R) -> R {
        // (set for as long as a tick's hand is being made, and only polled inside that: `open` and `close`)
        let g = self.game.get();
        assert!(!g.is_null(), "a scripted player is asked between two hands");
        f(unsafe { &*g })
    }
    pub fn world<R>(&self, f: impl FnOnce(&WorldAsk) -> R) -> R { self.ask(|g| f(&WorldAsk::of(g))) }
    pub fn seen<R>(&self, f: impl FnOnce(&Game, &game::seats::SeatRef) -> R) -> R { self.ask(|g| f(g, &game::seats::seat(g, self.seat))) }
    pub fn t(&self) -> f64 { self.t.get() }
    pub fn pos(&self) -> V3 { self.seen(|g, s| g.ch[g.pl[s.player].ch].pos) }
    pub fn man_at(&self, gd: GuardId) -> V3 { self.ask(|g| g.ch[g.gd[gd].ch].pos) }
    pub fn dead(&self, gd: GuardId) -> bool { self.ask(|g| g.gd[gd].state == game::game::GuardState::Dead) }
    /// a place of the world on the picture, and is it inside the margins of the picture a hand can point at
    pub fn screen(&self, p: V3) -> (f64, f64, bool) {
        let n = self.now.borrow();
        let (w, h) = (n.client[0], n.client[1]);
        let (x, y, front) = game::camera::project(&n.view_proj, p, w, h, None, None);
        (x, y, front && x > 30.0 && x < w - 30.0 && y > 30.0 && y < h - 30.0)
    }
    pub fn on_screen(&self, p: V3) -> bool { self.screen(p).2 }
    /// the picture's size
    pub fn client(&self) -> [f64; 2] { self.now.borrow().client }
    /// what the cursor is on, as the keys read it
    pub fn under(&self) -> OrderTarget { let n = self.now.borrow().clone(); self.seen(|g, s| game::orders::target_under(g, s, &n, &mut Vec::new())) }
    pub fn name(&self, d: DoorId) -> String { self.ask(|g| g.dr[d].def.name.clone()) }
    pub fn say(&self, s: String) {
        let mut t = self.trace.borrow_mut();
        t.push(format!("{:6.1}  {}", self.t.get(), s));
        if t.len() > 400 { t.remove(0); }
    }
    pub fn trace(&self) -> String { self.trace.borrow().join("\n") }
    pub fn set_busy(&self, busy: bool) { self.busy.set(busy); }
    /// the script ends here, with these words
    pub async fn fail(&self, why: String) {
        self.failure(why);
        core::future::pending::<()>().await
    }
    /// … or from outside it, at the next hand
    pub fn failure(&self, why: String) { let mut f = self.failed.borrow_mut(); if f.is_none() { *f = Some(why); } }
    pub fn failed(&self) -> Option<String> { self.failed.borrow().clone() }

    // ---- the script's own clock
    pub async fn tick(&self) { Once(false).await }
    pub async fn ticks(&self, n: u32) { for _ in 0..n { self.tick().await } }
    pub async fn wait(&self, secs: f64) { let end = self.t.get() + secs; while self.t.get() < end - 1e-6 { self.tick().await } }
    pub async fn until(&self, mut done: impl FnMut() -> bool, secs: f64) -> bool {
        let end = self.t.get() + secs;
        while self.t.get() < end - 1e-6 && !done() { self.tick().await }
        done()
    }

    // ---- his hands
    /// a key goes down for a tick. (The squad's is never pressed twice inside the window of a double press: his second is a second try at an order, never 'fall in')
    pub async fn key(&self, code: &str) {
        let (quick, twice, secs) = self.ask(|g| (code == g.orders.knobs.quick_key, g.k.cmdring.twice, g.k.cmdring.twice_secs));
        if quick && twice {
            while self.t.get() < self.quick_at.get() + secs + 0.05 { self.tick().await }
            self.quick_at.set(self.t.get());
        }
        self.down(code);
        self.tick().await
    }
    /// … and `after` ticks more
    pub async fn press(&self, code: &str, after: u32) { self.key(code).await; self.ticks(after).await }
    /// the press alone, on the hand being made (a key that stays down: then `hold`)
    pub fn down(&self, code: &str) { self.press.borrow_mut().push(String::from(code)); }
    /// keys kept down for `secs`, or until `stop`
    pub async fn hold(&self, codes: &[&str], secs: f64, mut stop: impl FnMut() -> bool) {
        let n = self.held.borrow().len();
        self.held.borrow_mut().extend(codes.iter().map(|c| String::from(*c)));
        let end = self.t.get() + secs;
        while self.t.get() < end - 1e-6 { self.tick().await; if stop() { break; } }
        self.held.borrow_mut().truncate(n);
    }
    /// the cursor on a place of the world, held there as the view moves, until it is put elsewhere
    pub fn aim(&self, p: V3) { self.look.set(Some(p)); }
    pub fn aim_off(&self) { self.look.set(None); }
    /// a click, on the hand being made (`button`: it stays down)
    pub fn click_now(&self, bits: u32) { self.clicks.set(self.clicks.get() | bits); }
    pub fn button(&self, bits: u32, down: bool) { self.buttons.set(if down { self.buttons.get() | bits } else { self.buttons.get() & !bits }); }
    /// a click and two ticks, the cursor on the place first where one is named
    pub async fn click(&self, bits: u32, p: Option<V3>) {
        if let Some(p) = p { self.aim(p); self.tick().await; }
        self.click_now(bits);
        self.ticks(2).await
    }
    /// a button pressed, kept down on a point for `secs` and let go
    pub async fn hold_button(&self, bits: u32, p: V3, secs: f64) {
        self.aim(p);
        self.tick().await;
        self.click_now(bits);
        self.button(bits, true);
        self.wait(secs).await;
        self.button(bits, false);
        self.ticks(3).await
    }
    pub async fn wheel(&self, notches: i32) { for _ in 0..notches.abs() { self.wheel.set(self.wheel.get() + 100.0 * notches.signum() as f64); self.ticks(2).await; } }
    /// the host's view turned, and pulled back
    pub fn turn_view(&self, yaw: f64) { let l = self.lens.get(); self.lens.set([l[0] + yaw, l[1]]); }
    pub fn zoom_view(&self, delta: f64) { let l = self.lens.get(); self.lens.set([l[0], l[1] + delta]); }
    pub async fn crouch(&self, on: bool) { for _ in 0..3 { if self.seen(|g, s| g.pl[s.player].crouch) == on { break; } self.press("KeyC", 3).await; } }

    // ---- his feet
    /// the nearest place a man may stand, on the asked point's own side of a wall where there is one (scene_collision nav_grid_nearest_free_point, asked of the game shared)
    pub fn free(&self, p: V3) -> Option<V3> {
        self.world(|w| {
            let g = w.shared();
            let n = &g.col.nav;
            if !game::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return Some([p[0], 0.0, p[2]]); }
            let mid = |x: f64, z: f64| [(x + 0.5) * n.cell, 0.0, (z + 0.5) * n.cell];
            let (gx, gz) = (m::floor(p[0] / n.cell).clamp(0.0, n.w - 1.0), m::floor(p[2] / n.cell).clamp(0.0, n.h - 1.0));
            if n.blocked[(gz * n.w + gx) as usize] == 0 { return Some(mid(gx, gz)); }
            let (mut fallback, mut extra) = (None, 1);
            for r in 1..8i32 {
                for dz in -r..=r { for dx in -r..=r {
                    if dx.abs().max(dz.abs()) != r { continue; }
                    let (x, z) = (gx + dx as f64, gz + dz as f64);
                    if x < 0.0 || z < 0.0 || x >= n.w || z >= n.h || n.blocked[(z * n.w + x) as usize] != 0 { continue; }
                    let q = mid(x, z);
                    fallback = fallback.or(Some(q));
                    if !w.line([p[0], 1.0, p[2]], [q[0], 1.0, q[2]], Line::Solid) { return Some(q); }
                } }
                if fallback.is_some() { if extra <= 0 { return fallback; } extra -= 1; }
            }
            fallback
        })
    }
    pub fn way(&self, a: V3, b: V3) -> Option<Vec<V3>> { self.world(|w| w.way(a, b, self.feet.search)) }
    pub fn way_len(&self, a: V3, b: V3) -> f64 {
        let Some(w) = self.way(a, b) else { return f64::INFINITY };
        let (mut l, mut q) = (0.0, a);
        for p in w { l += flat(q, p); q = p; }
        l + flat(q, b)
    }
    pub fn solid(&self, a: V3, b: V3) -> bool { self.world(|w| w.line([a[0], 1.1, a[2]], [b[0], 1.1, b[2]], Line::Solid)) }
    /// walk to `p`; false: stopped early (`stop`, `max` seconds, or not getting anywhere after fresh ways)
    pub async fn go(&self, p: V3, max: f64, mut stop: impl FnMut() -> bool, o: Walk) -> bool {
        let (mut left, mut again) = (max, 0u32);
        let feet = self.feet;
        let gone = |r: bool| { self.goal.set(None); self.sprint.set(false); if feet.eyes_on_way { self.look.set(None); } r };
        'afresh: loop {
            let mut path = self.way(self.pos(), p).unwrap_or_default();
            if path.last().is_none_or(|&l| flat(l, p) > 0.05) { path.push([p[0], 0.0, p[2]]); }
            if let Some(verb) = o.door {
                if let Some(d) = self.door_on_way(&path) {
                    if flat(self.pos(), self.ask(|g| g.dr[d].frame_centre)) < 40.0 {
                        let name = self.name(d);
                        let opening: Pin<Box<dyn Future<Output = bool> + '_>> = Box::pin(self.open_door(d, verb));
                        if !opening.await { self.say(format!("  the door {} would not open", name)); return gone(false); }
                        continue 'afresh;
                    }
                }
            }
            let (t0, last) = (self.t.get(), path.len() - 1);
            let (mut mark, mut mark_t) = (self.pos(), self.t.get());
            self.sprint.set(o.sprint);
            for (i, &wp) in path.iter().enumerate() {
                self.goal.set(Some([wp[0], 0.0, wp[2]]));
                while flat(self.pos(), wp) > if i == last { o.near } else { feet.corner } {
                    if feet.eyes_on_way { self.look.set(Some([wp[0], 1.1, wp[2]])); }
                    self.tick().await;
                    if stop() || self.t.get() - t0 > left { return gone(false); }
                    let me = self.pos();
                    if flat(me, mark) > feet.moved || self.busy.get() { mark = me; mark_t = self.t.get(); }
                    else if self.t.get() - mark_t > feet.stuck {
                        // (a mate in a doorway, a chair, the tip of a leaf: a step to one side, then the other, and a fresh way)
                        if again >= feet.tries { return gone(false); }
                        let (dx, dz) = (wp[0] - me[0], wp[2] - me[2]);
                        let l = { let l = m::hyp2(dx, dz); if l > 0.0 { l } else { 1.0 } };
                        let (side, k) = (if again % 2 == 1 { -1.0 } else { 1.0 }, 0.9 + 0.3 * (again >> 1) as f64);
                        self.goal.set(Some([me[0] - dz / l * side * k - dx / l * 0.3, 0.0, me[2] + dx / l * side * k - dz / l * 0.3]));
                        let end = self.t.get() + 0.6;
                        while self.t.get() < end { self.tick().await; if stop() { return gone(false); } }
                        left -= self.t.get() - t0;
                        again += 1;
                        continue 'afresh;
                    }
                }
            }
            return gone(true);
        }
    }
    /// straight at `p` for up to `secs`, or until `stop`, with no search of the floor
    pub async fn nudge(&self, p: V3, secs: f64, mut stop: impl FnMut() -> bool) {
        self.goal.set(Some([p[0], 0.0, p[2]]));
        let end = self.t.get() + secs;
        while self.t.get() < end - 1e-6 {
            self.aim([p[0], 1.1, p[2]]);
            self.tick().await;
            if stop() { break; }
        }
        self.goal.set(None);
    }
    /// the first shut door the way crosses: a leg of it over the leaf's line, jamb to jamb
    fn door_on_way(&self, path: &[V3]) -> Option<DoorId> {
        self.ask(|g| {
            let mut a = self.pos();
            for &b in path {
                let (mut best, mut near) = (None, 2.0);
                for &d in &g.doors.list {
                    if !game::doors::door_is_closed(g, d) { continue; }
                    let def = &g.dr[d].def;
                    let (th, w) = (def.closed_dir, def.width + 0.15);
                    let (hx, hz) = (def.hinge[0] - m::cos(th) * 0.15, def.hinge[1] - m::sin(th) * 0.15);
                    let (ux, uz) = (m::cos(th) * (w + 0.15), m::sin(th) * (w + 0.15));
                    let (rx, rz) = (b[0] - a[0], b[2] - a[2]);
                    let den = rx * uz - rz * ux;
                    if den.abs() < 1e-9 { continue; }
                    let (k, l) = (((hx - a[0]) * uz - (hz - a[2]) * ux) / den, ((hx - a[0]) * rz - (hz - a[2]) * rx) / den);
                    if (0.0..=1.0).contains(&k) && (0.0..=1.0).contains(&l) && k < near { near = k; best = Some(d); }
                }
                if best.is_some() { return best; }
                a = b;
            }
            None
        })
    }
    /// a shut door opened from his side of it: he walks up to it and works its row
    pub async fn open_door(&self, d: DoorId, verb: Verb) -> bool {
        let closed = || self.ask(|g| game::doors::door_is_closed(g, d));
        if !closed() { return true; }
        let (c, th) = self.ask(|g| (g.dr[d].frame_centre, g.dr[d].def.closed_dir));
        let (nx, nz) = (-m::sin(th), m::cos(th));
        let me = self.pos();
        let s = if (me[0] - c[0]) * nx + (me[2] - c[2]) * nz >= 0.0 { 1.0 } else { -1.0 };
        let face = [c[0] + nx * s * 0.95, 0.0, c[2] + nz * s * 0.95];
        self.go(self.free(face).unwrap_or(face), 25.0, || !closed(), Walk::default()).await;
        if !closed() { return true; }
        if !self.row(InteractableKind::Door, verb, Some([c[0], 1.0, c[2]])).await { return false; }
        self.until(|| !closed(), 6.0).await;
        self.ticks(20).await;
        !closed()
    }
    /// A ROW OF THE LIST BESIDE HIM: the highlight walked to it by its key and the interaction key tapped, within a second, with the cursor kept on a place where one is named. False: no such row came up.
    pub async fn row(&self, kind: InteractableKind, verb: Verb, aim: Option<V3>) -> bool {
        for _ in 0..60 {
            if let Some(p) = aim { self.aim(p); }
            self.tick().await;
            let (lit, there) = self.seen(|g, s| {
                let rows = &s.nearby;
                (rows.get(g.pl[s.player].near_sel.max(0) as usize).is_some_and(|r| r.kind == kind && r.verb == Some(verb)), rows.iter().any(|r| r.kind == kind && r.verb == Some(verb)))
            });
            if lit { self.down(game::consts::INTERACT_KEY); self.ticks(2).await; return true; }
            if there { self.down("ArrowDown"); }
        }
        false
    }
}

/// one tick passes
struct Once(bool);
impl Future for Once {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> { if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending } }
}

/// WHAT A HOST HANDS A SEAT OVER TO: he writes the hand of the tick about to run
pub trait Script {
    /// the hand of the tick about to run: what the host sampled, with his over it. The box world's copy stands (`scene_collision::stands`).
    fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64);
    /// he has nothing more to do
    fn over(&self) -> bool;
    /// the view turned and pulled back since the last hand, for the host to do before it samples the next
    fn lens(&mut self) -> [f64; 2] { [0.0, 0.0] }
}

/// A PLAYER AND THE SCRIPT HE PLAYS. `watch` looks on once a tick before the script does: what a whole run owes, whichever line of the script it is on (it fails him through `Player::failure`).
pub struct Scripted {
    pub me: Rc<Player>,
    script: Option<Pin<Box<dyn Future<Output = ()>>>>,
    watch: Option<Box<dyn FnMut(&Player)>>,
}
impl Scripted {
    pub fn new(me: Rc<Player>, script: impl Future<Output = ()> + 'static) -> Scripted { Scripted { me, script: Some(Box::pin(script)), watch: None } }
    pub fn watched(mut self, watch: impl FnMut(&Player) + 'static) -> Scripted { self.watch = Some(Box::new(watch)); self }
}
impl Script for Scripted {
    fn hand(&mut self, g: &Game, cmd: &mut PlayerCmd, dt: f64) {
        // (the host's Input keeps what the last hand held: what is held and pressed now is his to say)
        cmd.keys.clear();
        cmd.pressed.clear();
        (cmd.buttons, cmd.clicked, cmd.wheel) = (0.0, 0.0, 0.0);
        let Some(script) = self.script.as_mut() else { return };
        let me = self.me.clone();
        me.open(g, cmd);
        if let Some(w) = self.watch.as_mut() { w(&me); }
        let done = me.failed().is_some() || script.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_ready();
        me.hands(g, cmd, None);
        me.close(cmd, dt);
        if done || me.failed().is_some() { self.script = None; me.rest(); }
    }
    fn over(&self) -> bool { self.script.is_none() }
    fn lens(&mut self) -> [f64; 2] { self.me.take_lens() }
}
