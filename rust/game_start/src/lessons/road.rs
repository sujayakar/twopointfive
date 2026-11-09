//! The tutorial's first mission: what each line asks, through the input only, in the order a new player would do it. He is mortal: the fights are the level's script's and none of them is aimed at
//! a man who does what the line says.
use super::{Bot, Ended, Tutorial};
use crate::player::{at, flat, Walk, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use game::game::{InteractableKind, MissionStage, SlotKind, Verb};
use game::ids::GuardId;
use game::player::WallHoldKind;

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-road",
    // (the last men already down, a gun no reload would help, a man who stepped off the gateway's wall to shoot the sentry)
    idle_ok: &["last", "door", "reload", "off"],
    fight_ok: &[],
    ended: Ended::Stage,
    play: |b, id| Box::pin(settled(b, id)),
    finish: None,
};

// the level's marks
const POST_X0: f64 = 11.0;
const GATEWAY_A: f64 = 13.65;
const HUT_X0: f64 = 17.0;
const HUT_DOOR_Z: f64 = 12.75;
const COVER_Z: f64 = 15.5;
const RB_WALL: (f64, f64) = (27.4, 29.8);
const VAN: V3 = [3.1, 0.0, 12.6];
const WALL: V3 = [10.35, 0.0, 11.9];
const DOOR_FROM: V3 = [15.2, 0.0, HUT_DOOR_Z];
const PLANKS_X: f64 = 20.2;
const HUT_MAN: V3 = [20.7, 0.0, 11.45];
const TRUNK: V3 = [32.7, 0.0, 10.45];
const JAMB: V3 = [21.5, 0.0, 14.75];
const LANE_Z: f64 = 16.7;
const BAGS_END: V3 = [26.7, 0.0, LANE_Z];
const COVER: V3 = [28.6, 0.0, 16.2];
const EXIT: (f64, f64) = (37.8, 14.25);
const LAST_MEN: [V3; 3] = [[36.1, 0.0, 13.3], [35.4, 0.0, 15.3], [36.1, 0.0, 16.1]];

fn man(b: &Bot, route: &str) -> GuardId { b.on_route(route).unwrap_or_else(|| panic!("no man walks '{}'", route)) }
/// the men down: on this road there is nobody else
fn down(b: &Bot) -> usize { b.downed() }
fn stack(b: &Bot) -> bool { b.me(|p| p.wall.as_ref().is_some_and(|w| w.kind == Some(WallHoldKind::Stack))) }
async fn off_wall(b: &Bot) { if b.on_wall() { b.press("KeyQ", 10).await; } }
fn sprinting() -> Walk { Walk { sprint: true, ..Walk::doors() } }
fn last_in(b: &Bot) -> bool { let m = b.men(); m.iter().skip(3).enumerate().all(|(k, &x)| !b.up(x) || flat(b.man_at(x), LAST_MEN[k]) < 0.9) }
/// the lane behind the sandbags, from the side door's lee to its far end
async fn lane(b: &Bot) { b.to([JAMB[0] + 0.1, 0.0, LANE_Z], 12.0).await; b.to(BAGS_END, 15.0).await; }

/// a line stays up a beat after it is done where the level says so (a step's `settle`: the man falls, the door goes, the reload ends): each step waits that out, as a player does
async fn settled(b: &Bot, id: &str) {
    deed(b, id).await;
    if b.step_is(id) && b.settling() { b.until(|| !b.step_is(id), 6.0).await; }
}

async fn deed(b: &Bot, id: &str) {
    match id {
        // THE ROAD: off the start mark, up the truck's near side and behind its cab, with the sentry's rounds going into its far side
        "move" => {
            if !b.locked("camera") { b.fail(String::from("the view can be turned before the line that teaches it")).await; }
            b.to(VAN, 20.0).await;
        }
        // the view, turned: it was locked until this line, and it is put back as it was once the line has gone by
        "turn" => {
            let mut n = 0;
            while n < 60 && b.step_is("turn") { b.turn_view(0.025); b.tick().await; n += 1; }
            if b.step_is("turn") { b.fail(String::from("the view was turned most of a quarter turn and the line is still up")).await; }
            for _ in 0..n { b.turn_view(-0.025); b.tick().await; }
        }
        "ready" => b.take("Digit2").await,
        // round the truck's nose and along under the rock to the post's wall, running: never in the gateway's line
        "sprint" => {
            b.to([3.2, 0.0, 11.1], 10.0).await;
            b.to_by(WALL, 12.0, Walk { near: 0.6, ..sprinting() }, || false).await;
        }
        // walk into the wall as the page takes one
        "wall" => {
            if b.on_wall() { b.fail(String::from("he is on a wall before the line that teaches it")).await; }
            b.nudge([POST_X0 + 0.5, 0.0, WALL[2]], 1.5, || b.on_wall()).await;
            if !b.on_wall() { b.aim([POST_X0 + 0.6, 1.2, WALL[2]]); b.press("KeyQ", 20).await; }
            if !b.on_wall() { b.fail(String::from("no wall taken beside the gateway, by walking into it or by its key")).await; }
        }
        // along the wall to the gateway's corner, and the cursor on the man past it: the page steps him out with the rifle in the shoulder
        "peek" => {
            let s = man(b, "sentry");
            b.nudge([WALL[0], 0.0, GATEWAY_A + 0.4], 4.0, || !b.on_wall() || b.line(s) || b.me(|p| p.wall.as_ref().is_some_and(|w| if w.kind == Some(WallHoldKind::Stack) { w.at_edge != 0.0 } else { w.peek > 0.5 }))).await;
            if stack(b) && !b.step_is("peek") { b.fail(String::from("the lean's line went by on the slide alone, before the cursor was past the corner")).await; }
            for _ in 0..150 { if !b.step_is("peek") { break; } b.aim(b.chest(s)); b.tick().await; }
            b.say(format!("  at the corner: {} · a line to the sentry {} · body {}", b.me(|p| p.wall.as_ref().map_or(String::from("off the wall"), |w| format!("{:?} lean {:.2} peek {:.2} edge {}", w.kind, w.lean.unwrap_or(0.0), w.peek, w.at_edge))), b.line(s), at(b.pos())));
            if b.step_is("peek") { b.fail(String::from("at the gateway's corner with the cursor on the sentry he did not lean out")).await; }
        }
        // … and shoot the man in the gateway's line from there
        "fire" => {
            let s = man(b, "sentry");
            for _ in 0..90 { if b.line(s) { break; } b.aim(b.chest(s)); b.tick().await; }
            if !b.line(s) { b.fail(String::from("leaning round the gateway's corner he has no line to the sentry")).await; }
            let r0 = b.rounds();
            if !b.fight(1, 20.0, async || b.tick().await, Some(&[s])).await { b.fail(format!("the sentry did not drop (line {}, he is {:?}, {} rounds left the gun)", b.line(s), b.state(s), r0 - b.rounds())).await; }
            b.say(format!("  the sentry is down from the corner: {} round(s)", r0 - b.rounds()));
            b.ticks(45).await;
            if !b.step_is("fire") { b.fail(String::from("the next line came up within a second of the sentry going down: the moment did not settle")).await; }
        }
        // the rifle's selector: the setup's first row (the rifle's digit again, then 1)
        "auto" => {
            b.press("Digit2", 12).await;
            if b.me(|p| p.setup.is_none()) { b.fail(String::from("the rifle's key pressed again did not open its setup")).await; }
            b.press("Digit1", 12).await;
            if b.step_is("auto") { b.fail(format!("the selector did not move (the hands hold {:?}, its setup is {})", b.me(|p| p.slot), if b.me(|p| p.setup.is_some()) { "up" } else { "shut" })).await; }
            if b.me(|p| p.slot) != SlotKind::Rifle { b.fail(format!("the selector's digit took the {:?} into his hands", b.me(|p| p.slot))).await; }
        }
        // off the wall the way the line says: the move keys, square away from it
        "off" => {
            let Some(n) = b.me(|p| p.wall.as_ref().map(|w| w.n)) else { return };
            let p = b.pos();
            b.nudge([p[0] + n[0] * 1.5, 0.0, p[2] + n[2] * 1.5], 1.5, || !b.on_wall()).await;
            if b.on_wall() { b.fail(String::from("a second and a half of walking square away from the wall and he is still on it")).await; }
        }
        "inner" => { b.to([POST_X0 + 1.2, 0.0, 15.2], 10.0).await; b.to_near(DOOR_FROM, 20.0, 0.6).await; }
        // THE HUT'S DOOR: in by the gateway, square on to the leaf, and the trigger held until it is off its hinges (twelve rifle rounds)
        "door" => {
            let (d, on, r0) = (b.door("hut"), [HUT_X0, 1.1, HUT_DOOR_Z], b.rounds());
            if !b.locked_door(d) || !b.closed(d) { b.fail(String::from("the hut's door is open or unlocked before the line that shoots it out")).await; }
            b.to_near(DOOR_FROM, 6.0, 0.3).await;
            b.trigger(|| on, 6.0, || !b.closed(d)).await;
            let n = r0 - b.rounds();
            b.say(format!("  the hut's door: {} after {} round(s)", if b.closed(d) { "still standing" } else { "out" }, n));
            if b.closed(d) || n > 14.0 { b.fail(format!("the hut's door took {} rounds square on and is {} (it stands twelve)", n, if b.closed(d) { "still standing" } else { "out" })).await; }
            // (the rounds for the door go on through it, through the boards and into the back room: the man there stands a long pace off their line)
            if down(b) != 1 { b.fail(format!("{} men are down after the door: a round through the leaf found the man behind the boards", down(b))).await; }
            b.ticks(40).await;
            if !b.step_is("door") { b.fail(String::from("the reload's line came up within a second of the door going: the moment did not settle")).await; }
        }
        // the reload, and the line stays up until it has run to its end
        "reload" => {
            if !b.step_is("reload") { return; }   // (a gun with nothing to gain from a reload: the line has gone by)
            let hut = b.door("hut");
            b.until(|| !b.closed(hut), 2.0).await;
            if b.closed(hut) { b.fail(String::from("the reload's line is up and the hut's door still stands: the script did not open it")).await; }
            let w = b.gun();
            let full = || !b.reloading() && b.ask(|g| game::weapons::firearm_rounds_ready(g, w) >= g.m.weapons.weapons.get(g.fa[w].def).unwrap().mag_capacity);
            if !full() && !b.reloading() { b.press("KeyR", 6).await; }
            if !b.reloading() && !full() { b.fail(format!("R did not start a reload ({} ready)", b.rounds())).await; }
            while b.reloading() { b.tick().await; if !b.step_is("reload") { b.fail(String::from("the next line came up while the rifle was still being reloaded")).await; } }
            b.ticks(30).await;
            if !b.step_is("reload") { b.fail(String::from("the next line came up within half a second of the reload: the moment did not settle")).await; }
            b.say(format!("  reloaded: {} ready", b.rounds()));
        }
        // THE PLANK WALL: from outside the doorway, the cursor on the man behind the boards and the trigger held
        "thin" => {
            let x = man(b, "hut");
            if !b.up(x) { b.fail(String::from("the man behind the boards is down before his line")).await; }
            b.to_near([HUT_X0 - 0.7, 0.0, HUT_DOOR_Z + 0.15], 6.0, 0.25).await;
            b.wait(2.2).await;
            if b.is_down() { b.fail(String::from("he stood at the hut's doorway as the man behind the boards fired, and was hit")).await; }
            if !b.until(|| !b.up(x) || flat(b.man_at(x), HUT_MAN) < 0.5, 5.0).await { b.fail(format!("the man in the back room did not come out of his nook to the boards (he is at {})", Bot::at_str(b.man_at(x)))).await; }
            // (a burst at the door can chew the boards' near end: a line through a gap his own rounds made is his to have. What is asked is that the boards, where they still stand on the
            //  line between the two, hide the man)
            let (seen, me, c) = (b.line(x), b.pos(), b.chest(x));
            let zc = c[2] + (me[2] - c[2]) * (c[0] - PLANKS_X) / (c[0] - me[0]);
            let stand = b.world(|w| w.ray([PLANKS_X - 0.5, 1.2, zc], [1.0, 0.0, 0.0], 1.0).is_some());
            if seen && stand { b.fail(format!("he has a clear line to the man behind the boards from the doorway with the boards standing between them (he is at {}, the player at {})", Bot::at_str(c), Bot::at_str(me))).await; }
            if seen { b.say(String::from("  the boards on the line to him are down (his own rounds at the door): he is shot through the gap")); }
            let r0 = b.rounds();
            for _ in 0..3 {
                if !b.up(x) { break; }
                if b.rounds() == 0.0 { b.press("KeyR", 1).await; b.until(|| !b.reloading(), 5.0).await; }
                b.trigger(|| b.chest(x), 2.5, || !b.up(x)).await;
            }
            if b.up(x) { b.fail(format!("{} rounds through the boards and the man behind them is still up", r0 - b.rounds())).await; }
            b.say(format!("  the man behind the boards is down through them: {} round(s)", r0 - b.rounds()));
        }
        // round the boards' end, out of the side door and into the wind wall's lee; then down, as the gun starts along the sandbags
        "arch" => { b.to([HUT_X0 + 1.0, 0.0, HUT_DOOR_Z], 10.0).await; b.to([PLANKS_X + 0.6, 0.0, HUT_DOOR_Z + 0.1], 15.0).await; b.to(JAMB, 15.0).await; }
        "crouch" => { b.wait(1.2).await; b.crouch(true).await; }
        // crouched along behind the sandbags
        "low" => lane(b).await,
        // on behind the roadblock's wall, up, and into it
        "cover" => {
            b.to([RB_WALL.0 + 0.5, 0.0, COVER[2] + 0.2], 8.0).await;
            b.crouch(false).await;
            b.to_near(COVER, 6.0, 0.3).await;
            b.nudge([COVER[0], 0.0, COVER_Z], 1.5, || b.on_wall()).await;
            if !b.on_wall() { b.aim([COVER[0], 1.2, COVER_Z - 0.2]); b.press("KeyQ", 20).await; }
            if !b.on_wall() { b.fail(String::from("no wall taken at the roadblock, by walking into it or by its key")).await; }
        }
        // along it to its bare end, the cursor on the gunner: the lean again, with nobody saying how
        "gunner" => {
            let x = man(b, "gunner");
            b.nudge([RB_WALL.1 + 0.4, 0.0, COVER[2] - 0.3], 4.0, || !b.on_wall() || b.me(|p| p.wall.as_ref().is_some_and(|w| if w.kind == Some(WallHoldKind::Stack) { w.at_edge != 0.0 } else { w.peek > 0.5 }))).await;
            for _ in 0..150 {
                if b.line(x) && (!stack(b) || b.me(|p| p.wall.as_ref().and_then(|w| w.lean).unwrap_or(0.0)) > 0.4) { break; }
                b.aim(b.chest(x));
                b.tick().await;
            }
            if !b.line(x) { b.fail(String::from("leaning round the end of the roadblock's wall he has no line to the gunner")).await; }
            if !b.scripted(x) { b.fail(String::from("the gunner was let go while the player did as the lines say (he is the script's until a man walks out in front of him)")).await; }
            let r0 = b.rounds();
            if !b.fight(3, 25.0, async || b.tick().await, Some(&[x])).await { b.fail(format!("the gunner did not drop (line {}, he is {:?}, {} rounds left the gun)", b.line(x), b.state(x), r0 - b.rounds())).await; }
            b.say(format!("  the gunner is down from the wall's end: {} round(s)", r0 - b.rounds()));
        }
        // the line that only has him look: it goes by once the three stand inside the gate, and not before
        "watch" => {
            b.until(|| !b.step_is("watch"), 10.0).await;
            if b.step_is("watch") { return; }
            let men: Vec<GuardId> = b.men().into_iter().skip(3).filter(|&x| b.up(x)).collect();
            if men.len() < 3 || !last_in(b) { b.fail(format!("the trigger is asked for before the three stand inside the gate ({} up)", men.len())).await; }
        }
        // the three in the gate's lamp, from the wall's end
        "last" => {
            b.fight(6, 60.0, async || {
                let Some(x) = b.men().into_iter().find(|&y| b.up(y)) else { return };
                let p = b.man_at(x);
                if !b.on_wall() && flat(p, b.pos()) > 12.0 { b.to_by([p[0], 0.0, p[2]], 1.5, Walk::doors(), || b.line(x)).await; } else { b.ticks(10).await; }
            }, None).await;
        }
        // THE RADIO: the trunk in the gun's nest, round the end of its sandbags. The line asks for a TAP, and a tap does the top row: it must be the radio's (the gunner lies a pace from it)
        "radio" => { off_wall(b).await; b.to_near([TRUNK[0], 0.0, TRUNK[2] + 0.95], 30.0, 0.3).await; }
        "take" => {
            b.to_near([TRUNK[0], 0.0, TRUNK[2] + 0.95], 6.0, 0.3).await;
            b.ticks(2).await;
            let top = b.seen(|_, s| s.nearby.iter().find_map(|r| r.verb.map(|v| (r.kind, v))));
            if let Some((k, v)) = top { if v != Verb::Drive { b.fail(format!("a tap at the trunk gives '{} > {}', not the radio", k.as_str(), v.as_str())).await; } }
            if !b.verb(InteractableKind::Objective, Verb::Drive, Some([TRUNK[0], 0.5, TRUNK[2] + 0.4])).await { b.fail(String::from("no row to take the radio")).await; }
            b.until(|| b.stage() == MissionStage::Exfil, 20.0).await;
        }
        // on up the road: through the gate
        "out" => {
            off_wall(b).await;
            b.to_by([EXIT.0 - 1.6, 0.0, EXIT.1], 30.0, sprinting(), || false).await;
            b.to([EXIT.0 + 1.2, 0.0, EXIT.1], 10.0).await;
        }
        _ => panic!("tut-road has no deed for the step '{}'", id),
    }
}
