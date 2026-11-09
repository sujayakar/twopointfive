//! The squad's tutorial mission on the ground. He is a man on the floor: his own feet, the view's zoom, the squad's key on a point under the cursor (the ground, a door, himself) and the go key. He never
//! fires: what the squad does with one order and one word is what the run is about. He is as mortal as a player, which is what the run is for.
use super::{Bot, Ended, Tutorial};
use crate::player::{at, V3};
use alloc::boxed::Box;
use alloc::format;

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-team",
    // (the roadblock can be down before he is through the gate: then its step is done as it comes up)
    idle_ok: &["gate"],
    fight_ok: &[],
    ended: Ended::Finished,
    play: |b, id| Box::pin(deed(b, id)),
    finish: None,
};

const YARD: V3 = [11.4, 0.02, 16.4];
const TEA_DOOR: V3 = [9.0, 1.0, 11.7];
const KITCHEN_DOOR: V3 = [13.8, 1.0, 6.25];
const BUNK_DOOR: V3 = [28.8, 1.0, 16.25];
const TRUCKS: V3 = [6.1, 0.0, 23.6];
/// inside the yard's wall beside the gate, out of the gateway's line · inside the tea room, in sight of the kitchen's door · beside the bunkhouse, short of its door
const BY_GATE: V3 = [10.4, 0.0, 18.6];
const IN_TEA: V3 = [10.4, 0.0, 8.6];
const BY_BUNK: V3 = [26.2, 0.0, 17.5];
const BLOCK: [&str; 3] = ["Tango 6", "Tango 7", "Tango 8"];

fn near(b: &Bot, p: V3, r: f64) -> bool { b.squad().into_iter().all(|m| { let q = b.man_at(m); game::m::hyp2(q[0] - p[0], q[2] - p[2]) < r }) }
/// the squad's key on himself: they fall in. Asked of the order book: a tap that gave anything else is a line that lies
async fn call_in(b: &Bot) {
    for _ in 0..3 {
        if b.following() { break; }
        let p = b.pos();
        b.command([p[0], 1.0, p[2]]).await;
        b.ticks(4).await;
        // (beside a doorway the cursor on himself can be on the door as drawn, and the tap is then that door's: what a hand does is step off it and tap again)
        if !b.following() && b.last_order().is_some_and(|l| l.0 == "stack") {
            b.say(format!("  the squad's key on himself at {} gave the door beside him: he steps off it and taps again", at(p)));
            b.to([p[0] - 2.2, 0.0, p[2] + 0.6], 12.0).await;
        }
    }
    if !b.following() { b.fail(format!("the squad's key on himself gave {:?}: {}", b.last_order(), b.where_line())).await; }
}
/// stack on a door, then the go key once they stand at it. The squad's key is tapped on the door only while they are NOT stacked on it, and the room has to fall to the go key
async fn breach(b: &Bot, door: V3, room: &str) {
    for _ in 0..3 { if b.stacked() { break; } b.command(door).await; b.until(|| b.stacked(), 25.0).await; }
    if !b.stacked() { b.fail(format!("the squad's key on the door of '{}' did not stack them: {}", room, b.where_line())).await; }
    b.wait(1.5).await;
    for _ in 0..2 { if b.taken(room) { break; } b.press("KeyF", 2).await; b.until(|| b.taken(room), 30.0).await; }
    if !b.taken(room) { b.fail(format!("the go key did not take '{}': {}", room, b.where_line())).await; }
    b.wait(2.0).await;
}
async fn reach(b: &Bot, p: V3, max: f64, what: &str) { if !b.to(p, max).await { b.fail(format!("he did not reach {}: he is at {}", what, at(b.pos()))).await; } }

async fn deed(b: &Bot, id: &str) {
    match id {
        "zoom" => { for _ in 0..20 { if !b.locked("zoom") { b.zoom_view(60.0); } b.tick().await; } b.wait(1.0).await; }
        // nothing to press: the two cans are called, thrown and burning before the line goes by
        "smoke" => b.wait(4.2).await,
        "move" => {
            b.command(YARD).await;
            if !b.until(|| b.squad().into_iter().all(|m| { let q = b.man_at(m); q[2] < 19.7 && q[0] > 4.4 }), 45.0).await {
                b.command(YARD).await;
                b.until(|| b.squad().into_iter().all(|m| b.man_at(m)[2] < 19.7), 30.0).await;
            }
            b.wait(1.0).await;
        }
        "with" => reach(b, BY_GATE, 40.0, "the yard from the road").await,
        "gate" => { let men = b.named(&BLOCK); b.until(|| !men.iter().any(|&x| b.up(x)), 70.0).await; b.wait(0.9).await; }
        "stack" => {
            for _ in 0..3 {
                b.command(TEA_DOOR).await;
                if b.until(|| b.stacked() || near(b, TEA_DOOR, 3.2), 30.0).await { break; }
                b.say(format!("  not at the door: {}", b.where_line()));
            }
            b.until(|| b.stacked(), 30.0).await;
            b.wait(0.5).await;
        }
        "go" => {
            for _ in 0..2 { if b.taken("tea") { break; } b.press("KeyF", 2).await; b.until(|| b.taken("tea"), 30.0).await; }
            if !b.taken("tea") { b.fail(format!("the go key did not take the tea room: {}", b.where_line())).await; }
            b.wait(2.0).await;
        }
        // (the kitchen's door is inside the tea room: he walks in to point at it, as a player does)
        "kitchen" => { reach(b, IN_TEA, 40.0, "the tea room").await; breach(b, KITCHEN_DOOR, "kitchen").await; }
        "fallin" => { call_in(b).await; b.wait(1.0).await; }
        "lead" => {
            reach(b, BY_BUNK, 60.0, "the bunkhouse").await;
            if !b.until(|| b.men_in([19.0, 28.9, 11.7, 19.8]) >= 3.min(b.squad().len()), 40.0).await { b.say(format!("  not with him: {}", b.where_line())); }
        }
        "bunk" => breach(b, BUNK_DOOR, "bunk").await,
        "out" => {
            call_in(b).await;
            reach(b, TRUCKS, 80.0, "the trucks").await;
            if !b.until(|| b.squad().into_iter().filter(|&m| { let q = b.man_at(m); q[0] < 12.5 && q[2] > 20.1 }).count() >= 3.min(b.squad().len()), 60.0).await { b.say(format!("  not back with him: {}", b.where_line())); }
            b.wait(1.2).await;
        }
        _ => panic!("tut-team has no deed for the step '{}'", id),
    }
}
