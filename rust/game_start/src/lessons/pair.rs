//! The tutorial's quiet mission for two: what each line asks, through the input only: walks to the marks the level shows, the cursor on a man, on the ground or on the fuse box, one-frame presses of the
//! squad's key and the word, a tap of the interaction key, the right button. Game state is read to time a press (the partner set, a man down).
use super::{Bot, Ended, Tutorial};
use crate::player::{at, flat, Walk, RIGHT, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use game::game::{GuardState, InteractableKind, MissionStage, Verb};
use game::ids::GuardId;
use game::orders::OrderTarget;
use game::view::{KeyCap, WaitWhy};

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-pair",
    idle_ok: &[],
    fight_ok: &[],
    ended: Ended::Stage,
    play: |b, id| Box::pin(deed(b, id)),
    finish: Some(|b| Box::pin(async move { b.to([39.5, 0.0, GATE[2]], 30.0).await; })),   // (on out by the far gate)
};

const REACT: f64 = 0.6;
// the level's marks
const FIRST: V3 = [9.4, 0.0, 20.0];
const GAP: V3 = [17.3, 0.0, 17.1];
const BEHIND_B: V3 = [19.6, 0.0, 17.75];
const ROUND: V3 = [26.6, 0.0, 20.2];
const PAST: V3 = [29.9, 0.0, 23.6];
const BOX: V3 = [32.6, 1.35, 25.7];
const PAD_AT: V3 = [32.3, 0.0, 10.35];
const TRUNK_FRONT: V3 = [35.6, 1.0, 6.8];
const DOOR: V3 = [33.25, 0.0, 9.6];
const GATE: V3 = [38.0, 0.0, 18.25];
/// the middle of the way out
const OUT_X: f64 = (38.1 + 38.75) / 2.0;

fn mate(b: &Bot) -> GuardId { b.squad().first().copied().unwrap_or_else(|| panic!("no partner up")) }
fn man(b: &Bot, route: &str) -> GuardId { b.on_route(route).unwrap_or_else(|| panic!("no man walks '{}'", route)) }
fn doing(b: &Bot, x: GuardId) -> String { b.seen(|g, s| game::orders::squad_orders_status_of(g, s, x)) }
fn chest(b: &Bot, x: GuardId) -> V3 { let p = b.man_at(x); [p[0], 1.2, p[2]] }
async fn calm(b: &Bot, n0: usize, place: &str) {
    let stirred: alloc::vec::Vec<String> = b.men().into_iter().filter(|&x| b.ask(|g| g.gd[x].unit.is_none() && g.gd[x].held.is_none()) && b.up(x) && b.state(x) != GuardState::Patrol).map(|x| format!("{} {:?}", b.called(x), b.state(x))).collect();
    if b.sightings() > n0 || !stirred.is_empty() { b.fail(format!("the pair was noticed {} ({})", place, if stirred.is_empty() { String::from("a sighting") } else { stirred.join(", ") })).await; }
}
/// the squad's key, tapped with the cursor on a man of theirs: the partner is given him
async fn on_man(b: &Bot, x: GuardId) {
    b.wait(0.4).await;
    b.aim(chest(b, x));
    b.ticks(2).await;
    b.aim(chest(b, x));
    if !matches!(b.under(), OrderTarget::Foe(_)) { b.fail(format!("the cursor on {} is not on him (is he in view from {}?)", b.called(x), at(b.pos()))).await; }
    b.press("KeyB", 4).await;
    let want = format!("foe:{}", b.called(x));
    match b.last_order() {
        Some((o, t)) if o == "pairMark" && t == want => b.say(format!("  the squad's key on {}: {}", b.called(x), o)),
        l => b.fail(format!("the squad's key on {} gave {:?}", b.called(x), l)).await,
    }
}
/// … with the cursor on bare ground beside him: the partner comes back
async fn on_ground(b: &Bot) {
    b.wait(0.3).await;
    let (me, n) = (b.pos(), b.orders_log());
    for r in [1.4, 2.0, 2.6] { for k in 0..8 {
        let a = k as f64 * game::m::PI / 4.0;
        b.aim([me[0] + r * game::m::cos(a), 0.05, me[2] + r * game::m::sin(a)]);
        b.tick().await;
        if !matches!(b.under(), OrderTarget::Floor(_)) { continue; }
        b.press("KeyB", 4).await;
        let l = b.last_order();
        if b.orders_log() == n || !l.as_ref().is_some_and(|l| l.0 == "fallIn") { b.fail(format!("the squad's key on the ground gave {:?}", if b.orders_log() == n { None } else { l })).await; }
        return;
    } }
    b.fail(format!("the cursor finds no bare ground beside the player at {}", at(me))).await;
}
/// the frame's line once an order is out: what the partner is doing, drawn as a thing to watch, never the ask a second time
async fn watch_line(b: &Bot, when: &str) {
    let Some(p) = b.prompt() else { b.fail(format!("no line {}", when)).await; return };
    if !p.watch || p.line.contains('[') { b.fail(format!("{} the line is still an ask: \"{}\"", when, p.line)).await; }
}
async fn together(b: &Bot, by: f64, secs: f64, what: &str) {
    let m = mate(b);
    if !b.until(|| flat(b.man_at(m), b.pos()) < by, secs).await { b.fail(format!("the partner did not {} ({}, {:.1} m off)", what, doing(b, m), flat(b.man_at(m), b.pos()))).await; }
}
async fn wait_set(b: &Bot, x: GuardId) {
    let (m, n0, t0) = (mate(b), b.sightings(), b.t());
    if !b.until(|| b.ask(|g| game::pairorders::pair_orders_job_of(g, m).is_some_and(|j| j.set)) || !b.up(x), 70.0).await { b.fail(format!("the partner never got set behind {} ({}, at {})", b.called(x), doing(b, m), at(b.man_at(m)))).await; }
    b.say(format!("  \"set\" {:.1} s on: the partner at {}, {:.2} m from {}", b.t() - t0, at(b.man_at(m)), flat(b.man_at(m), b.man_at(x)), b.called(x)));
    calm(b, n0, "while the partner worked round to his man").await;
}
/// the word's key as the floor plan marks it by a man of the pair who waits on it
fn pair_key(b: &Bot) -> alloc::vec::Vec<(KeyCap, bool)> { b.seen(|g, s| game::view::tick_view(g, s).plan.own.doors.iter().filter(|d| d.why == WaitWhy::Pair).map(|d| (d.cap, d.now)).collect()) }

async fn deed(b: &Bot, id: &str) {
    match id {
        "crouch" => { b.say(format!("  at the start: lit {:.2}; the partner {:.1} m off", b.lit(), flat(b.man_at(mate(b)), b.pos()))); b.crouch(true).await; }
        // a few strides in, the partner behind
        "lead" => {
            let n0 = b.sightings();
            b.to_near(FIRST, 20.0, 0.3).await;
            together(b, 4.2, 20.0, "follow to the first mark").await;
            b.wait(0.5).await;
            b.say(format!("  on the first mark: lit {:.2}; the partner at {} ({})", b.lit(), at(b.man_at(mate(b))), doing(b, mate(b))));
            if b.ask(|g| g.ch[g.gd[mate(b)].ch].anim.crouch_target < 0.5) { b.fail(String::from("the partner is on his feet behind a player who is low")).await; }
            calm(b, n0, "on the way to the first mark").await;
        }
        // (one step: the order, then the line is what he is doing until he says "set"; and until the word is taught nothing on the floor names its key)
        "mark" => {
            let a = man(b, "pickup");
            on_man(b, a).await;
            b.ticks(4).await;
            watch_line(b, "after the order on the first man").await;
            if b.locked("word") && !pair_key(b).is_empty() { b.fail(String::from("the word's key is marked by the partner while the lesson still holds the word shut")).await; }
            wait_set(b, a).await;
        }
        "word" => {
            let (a, n0) = (man(b, "pickup"), b.sightings());
            let k = pair_key(b);
            if k.len() != 1 || k[0].0 != KeyCap::Lit || !k[0].1 { b.fail(format!("the partner is set and the word is taught, and its key is not marked ready by him ({} mark(s))", k.len())).await; }
            b.wait(REACT).await;
            b.press("KeyF", 1).await;
            if !b.until(|| !b.up(a), 15.0).await { b.fail(format!("the word was given and {} is still up ({})", b.called(a), doing(b, mate(b)))).await; }
            b.wait(1.0).await;
            calm(b, n0, "when the first man went down").await;
        }
        // to the gap: both men of the truck park in view through it. He comes back by himself (no order asks for it)
        "gap" => {
            let n0 = b.sightings();
            b.crouch(true).await;
            b.to_near(GAP, 20.0, 0.3).await;
            together(b, 4.2, 30.0, "fall in again by himself after his man went down").await;
            b.wait(0.4).await;
            b.say(format!("  at the gap: lit {:.2}; the partner at {} ({})", b.lit(), at(b.man_at(mate(b))), doing(b, mate(b))));
            calm(b, n0, "at the gap").await;
        }
        "far" => { on_man(b, man(b, "far")).await; b.wait(1.5).await; }
        // through the gap to the near man's back, a stride off, and there until the partner is set behind the far one
        "near" => {
            let n0 = b.sightings();
            b.to_near(BEHIND_B, 15.0, 0.2).await;
            b.say(format!("  behind the near man: lit {:.2}, {:.2} m off", b.lit(), flat(b.pos(), b.man_at(man(b, "near")))));
            wait_set(b, man(b, "far")).await;
            calm(b, n0, "behind the near man").await;
        }
        // the grab is the signal: the partner's man drops on the same beat
        "hands" => {
            let (n, f) = (man(b, "near"), man(b, "far"));
            b.wait(REACT).await;
            if !b.close_in(n, None).await { b.fail(format!("could not get behind the near man ({:.2} m off, {:?})", flat(b.pos(), b.man_at(n)), b.state(n))).await; }
            let q = b.man_at(n);
            if !b.tap(InteractableKind::Guard, Verb::Grab, Some([q[0], 1.35, q[2]])).await { b.fail(String::from("no grab row behind the near man")).await; }
            let t0 = b.t();
            if !b.until(|| !b.up(f), 15.0).await { b.fail(format!("the grab did not send the partner ({})", doing(b, mate(b)))).await; }
            b.say(format!("  the far man down {:.1} s after the grab; the near man {}", b.t() - t0, if b.holding() { "in the arm" } else { "NOT held" }));
            if b.state(n) == GuardState::Alert { b.fail(String::from("the near man was roused by the double")).await; }
        }
        "sleep" => {
            let n = man(b, "near");
            b.click(RIGHT, None).await;
            if !b.until(|| !b.up(n) && !b.holding(), 15.0).await { b.fail(String::from("the held man did not go out")).await; }
            b.wait(0.6).await;
        }
        // the partner called in, round the second wall's end to the office yard's dark corner
        "past" => {
            let n0 = b.sightings();
            b.crouch(true).await;
            b.to(ROUND, 40.0).await;
            b.to_near(PAST, 30.0, 0.3).await;
            together(b, 4.2, 30.0, "come along past the wall").await;
            calm(b, n0, "coming into the office yard").await;
        }
        "hold" => {
            let (d, n0) = (man(b, "gate"), b.sightings());
            if !b.close_in(d, None).await { b.fail(format!("could not get behind the man at the far gate ({:.2} m off, he stands at {}, {:?})", flat(b.pos(), b.man_at(d)), at(b.man_at(d)), b.state(d))).await; }
            calm(b, n0, "coming up behind the man at the far gate").await;
            let q = b.man_at(d);
            if !b.tap(InteractableKind::Guard, Verb::Grab, Some([q[0], 1.35, q[2]])).await { b.fail(String::from("no grab row behind the man at the far gate")).await; }
            if !b.until(|| b.held_fast(), 4.0).await { b.fail(String::from("the grab did not take")).await; }
        }
        "ask" => {
            let mut n = 0;
            while n < 8 && !b.has_code("pad_office") {
                if !b.tap(InteractableKind::Held, Verb::Question, None).await { b.fail(String::from("no question row with a man held")).await; }
                b.until(|| b.has_code("pad_office"), 5.0).await;
                n += 1;
            }
            if !b.has_code("pad_office") { b.fail(String::from("eight questions and no code")).await; }
            b.say(format!("  the code after {} question(s)", n));
        }
        // the squad's key on the fuse box, with the man still in his arm
        "box" => {
            if !b.holding() { b.fail(String::from("nobody in his arm at the fuse box line")).await; }
            b.wait(0.3).await;
            b.aim(BOX);
            b.ticks(2).await;
            b.aim(BOX);
            b.press("KeyB", 4).await;
            if !b.last_order().is_some_and(|l| l.0 == "work") { b.fail(format!("the squad's key on the fuse box gave {:?}", b.last_order())).await; }
            b.ticks(4).await;
            watch_line(b, "after the order on the fuse box").await;
            let t0 = b.t();
            if !b.until(|| b.dark(), 40.0).await { b.fail(format!("the partner did not throw the mains ({}, at {})", doing(b, mate(b)), at(b.man_at(mate(b))))).await; }
            b.say(format!("  the mains went {:.1} s after the order; the man {}", b.t() - t0, if b.holding() { "still held" } else { "NOT held" }));
        }
        "drop" => {
            let d = man(b, "gate");
            b.wait(REACT).await;
            b.click(RIGHT, None).await;
            if !b.until(|| !b.up(d) && !b.holding(), 15.0).await { b.fail(String::from("the held man did not go out")).await; }
            b.wait(0.6).await;
            if !b.dark() { b.fail(String::from("the mains came back")).await; }
        }
        "nv" => b.press("KeyN", 20).await,
        "code" => {
            b.to_by(PAD_AT, 30.0, Walk::near(0.25), || false).await;
            if !b.tap(InteractableKind::Door, Verb::Code, Some([DOOR[0], 1.0, DOOR[2]])).await { b.fail(format!("no row to type the code (at {})", at(b.pos()))).await; }
            let d = b.door("office");
            if !b.until(|| !b.locked_door(d) || !b.closed(d), 10.0).await { b.fail(String::from("the office did not open to the code")).await; }
        }
        "take" => {
            b.to([DOOR[0], 0.0, DOOR[2] - 1.0], 20.0).await;
            b.to_near([TRUNK_FRONT[0] - 0.25, 0.0, TRUNK_FRONT[2]], 10.0, 0.2).await;
            if !b.tap(InteractableKind::Objective, Verb::Drive, Some(TRUNK_FRONT)).await { b.fail(String::from("no row at the trunk")).await; }
            b.until(|| b.stage() == MissionStage::Exfil, 20.0).await;
        }
        "out" => {
            on_ground(b).await;
            b.crouch(false).await;
            b.to([DOOR[0], 0.0, DOOR[2] + 1.2], 20.0).await;
            b.to([GATE[0] - 1.0, 0.0, GATE[2]], 30.0).await;
            together(b, 5.0, 30.0, "come to the far gate when called").await;
            b.to_near([OUT_X, 0.0, GATE[2]], 20.0, 0.2).await;
        }
        _ => panic!("tut-pair has no deed for the step '{}'", id),
    }
}
