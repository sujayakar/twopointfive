//! The tutorial's quiet mission: what each line asks, through the input only. The mission teaches the page's buttons (left the knife, right the knockout).
use super::{Bot, Ended, Tutorial};
use game::v3::dist_xz;
use crate::player::{at, Walk, LEFT, RIGHT, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use game::game::{GuardState, InteractableKind, MissionStage, Verb};
use game::ids::GuardId;
use game::weapons::RifleAttachment;

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-quiet",
    idle_ok: &[],
    fight_ok: &[],
    ended: Ended::Stage,
    play: |b, id| Box::pin(deed(b, id)),
    finish: Some(|b| Box::pin(async move { b.to([6.0, 0.0, 14.0], 30.0).await; })),   // (out by the track gate)
};

// the level's marks (tools/levels/tut-quiet.ts TQ)
const DARK: V3 = [15.2, 0.0, 17.6];
const WHISTLE: V3 = [21.5, 0.0, 17.8];
const HIDE: V3 = [23.9, 0.0, 19.05];
const LOOK: V3 = [26.4, 0.0, 17.75];
const SMOKE: V3 = [30.6, 0.0, 17.7];
const BACK_DOOR: V3 = [27.9, 0.0, 17.2];
const NOOK: V3 = [26.1, 0.0, 14.15];
const UNDER: V3 = [23.2, 0.0, 12.4];
const BOX: V3 = [22.1, 1.35, 16.2];
const BOX_AT: V3 = [21.65, 0.0, 15.3];
const WAIT: V3 = [20.45, 0.0, 13.85];
const SAFE_FRONT: V3 = [14.5, 1.0, 9.48];
const SHOT: V3 = [26.6, 0.0, 12.7];
const BEAM_HIGH: f64 = 24.45;
/// a person's beat between seeing a man come through and moving
const REACT: f64 = 0.6;
const ROUTES: [&str; 3] = ["hall", "court", "table"];

fn man(b: &Bot, route: &str) -> GuardId { b.on_route(route).unwrap_or_else(|| b.men()[ROUTES.iter().position(|r| *r == route).unwrap_or(0)]) }
fn near(p: f64) -> Walk { Walk::near(p) }
/// how many times a beam has been broken this run
fn trips(b: &Bot) -> f64 { b.ask(|g| g.devices.lasers.iter().map(|&l| g.tab.devices_laser_rt[l].trips as f64).sum()) }
fn rifle_load(b: &Bot) -> Option<RifleAttachment> { let w = b.rifle(); b.ask(|g| game::weapons::rifle_attachment(g, w)) }
fn tube(b: &Bot) -> (f64, f64) { let w = b.rifle(); b.ask(|g| (g.fa[w].shells, g.fa[w].rings)) }
/// a clear way from a point a metre up to his head: neither wall nor furniture
fn clear_to(b: &Bot, from: V3, h: GuardId) -> bool {
    let (to, p) = (b.head(h), [from[0], 1.0, from[2]]);
    let d = [to[0] - p[0], to[1] - p[1], to[2] - p[2]];
    let len = game::m::hyp3(d[0], d[1], d[2]);
    let dir = [d[0] / len, d[1] / len, d[2] / len];
    b.ask(|g| game::scene_collision::raycast(g, p, dir, len - 0.3).is_none() && game::props::prop_system_raycast(g, p, dir, len - 0.3).is_none())
}
/// he has stood still for `secs`
async fn stood_still(b: &Bot, x: GuardId, secs: f64, max: f64) -> bool {
    let (mut p, mut since) = (b.man_at(x), b.t());
    b.until(|| { if dist_xz(b.man_at(x), p) > 0.05 { p = b.man_at(x); since = b.t(); } b.t() - since > secs }, max).await
}

async fn deed(b: &Bot, id: &str) {
    match id {
        "crouch" => { b.say(format!("  at the start: lit {:.2}", b.lit())); b.crouch(true).await; }
        // out of the porch lamp's pool, round the house's corner into its shadow
        "dark" => { b.to([11.4, 0.0, 16.6], 20.0).await; b.to_near(DARK, 20.0, 0.3).await; b.wait(0.6).await; b.say(format!("  round the corner: lit {:.2}", b.lit())); }
        // along the house's wall, round the first buttress, to the whistle's mark: never out of its shadow
        "creep" => {
            let (a, n0) = (man(b, "court"), b.sightings());
            let worst = core::cell::Cell::new(0.0f64);
            let look = || { worst.set(worst.get().max(b.lit())); false };
            b.to_by([19.0, 0.0, 17.4], 40.0, Walk::doors(), look).await;
            b.to_by(WHISTLE, 10.0, Walk { near: 0.2, ..Walk::doors() }, look).await;
            b.say(format!("  along the wall: the most light on him {:.2}; {} {:?}, aware {:.2}, {:.1} m off", worst.get(), b.called(a), b.state(a), b.awareness(a), dist_xz(b.pos(), b.man_at(a))));
            b.unseen(n0, a, "in the lane").await;
        }
        // the whistle at the mark, a person's beat, then round the near end of the long wall to the lane's side of it, while he comes the long way round on the house's side.
        // (The interaction key kept down puts the ring up, on open floor its one wedge is the whistle, and its digit does it)
        "whistle" => {
            let (a, n0) = (man(b, "court"), b.sightings());
            let secs = b.ask(|g| g.k.handring.hold_secs);
            b.down(game::consts::INTERACT_KEY);
            b.hold(&[game::consts::INTERACT_KEY], secs + 0.1, || false).await;
            if b.seen(|_, s| s.hand.sel.ring.is_none()) { b.fail(String::from("the interaction key kept down put no ring up")).await; }
            b.press("Digit5", 0).await;
            let t0 = b.t();
            b.wait(REACT).await;
            b.to_near([22.7, 0.0, 19.1], 8.0, 0.25).await;
            b.to_near(HIDE, 8.0, 0.15).await;
            b.say(format!("  behind the wall {:.1} s after the whistle: lit {:.2}; {} {:?} at {}, {}", b.t() - t0, b.lit(), b.called(a), b.state(a), at(b.man_at(a)), if b.scripted(a) { "the script has him" } else { "his own man" }));
            if b.state(a) == GuardState::Patrol { b.fail(format!("the whistle moved nobody ({} at {})", b.called(a), at(b.man_at(a)))).await; }
            if !b.scripted(a) { b.fail(format!("the level's script did not take the man who heard the whistle ({} {:?})", b.called(a), b.state(a))).await; }
            b.unseen(n0, a, "going behind the wall").await;
            b.kept.set(t0);
        }
        // nothing pressed while he comes round, looks through the gap and turns away; once he stands by the crate with his back to the gap, a person's beat, round the wall's end, up the walk, through the gap, to his back
        "grab" => {
            let (a, n0, t0) = (man(b, "court"), b.sightings(), b.kept.get());
            // (what his light does while he looks: the most it puts on the place the whistle came from, and on the player behind the wall)
            let (mut on_spot, mut on_me, mut looked): (f64, f64, Option<V3>) = (0.0, 0.0, None);
            let lit_up = |x: GuardId| b.ask(|g| g.tab.light[g.gd[x].flashlight].enabled);
            let there = b.until(|| {
                let (me, d) = (b.man_at(a), b.ask(|g| g.gd[a].beam_dir));
                if lit_up(a) {
                    let to = [WHISTLE[0] - me[0], WHISTLE[2] - me[2]];
                    let n = { let n = game::m::hyp2(to[0], to[1]); if n > 0.0 { n } else { 1.0 } };
                    let cos = (to[0] * d[0] + to[1] * d[2]) / n;
                    if dist_xz(me, LOOK) < 0.4 && cos > on_spot { on_spot = cos; looked = Some(me); }
                }
                on_me = on_me.max(b.lit());
                let y = b.yaw(a) - game::m::PI / 2.0;
                (b.scripted(a) && dist_xz(me, SMOKE) < 0.35 && game::m::abs(game::m::atan2(game::m::sin(y), game::m::cos(y))) < 0.3) || b.state(a) == GuardState::Alert || !b.scripted(a)
            }, 45.0).await;
            b.say(format!("  {} looked from {:?}, his light's cos {:.2}; the most light on the player behind the wall {:.2}", b.called(a), looked.map(at), on_spot, on_me));
            b.say(format!("  {} at his smoke {:.1} s after the whistle ({}, facing {:.2}, {:?})", if there && b.scripted(a) { "he is" } else { "he never got" }, b.t() - t0, at(b.man_at(a)), b.yaw(a), b.state(a)));
            b.unseen(n0, a, "behind the wall").await;
            match looked {
                Some(from) if on_spot >= 0.96 => { if dist_xz(from, WHISTLE) < 4.0 { b.fail(format!("{} looked from {:.1} m: close enough to read the prints", b.called(a), dist_xz(from, WHISTLE))).await; } }
                _ => b.fail(format!("{} did not put his light on the place the whistle came from", b.called(a))).await,
            }
            if !there || !b.scripted(a) { b.fail(format!("{} was not there to be taken ({:?}, at {})", b.called(a), b.state(a), at(b.man_at(a)))).await; }
            b.wait(REACT).await;
            b.to_near([22.75, 0.0, 19.0], 8.0, 0.25).await;
            b.to_near([23.6, 0.0, 17.8], 8.0, 0.25).await;
            b.say(format!("  on the walk: {} at {}, facing {:.2}, {:?}, {:.1} m off; lit {:.2}", b.called(a), at(b.man_at(a)), b.yaw(a), b.state(a), dist_xz(b.pos(), b.man_at(a)), b.lit()));
            // his torch has come down for the smoke and he has clicked it off: still the hand's, under his shoulder, and dark
            let low = b.ask(|g| { let c = g.gd[a].ch; g.ch[c].bones.get(game::character::Bone::UpperArmR).map_or(-1.0, |u| u[1] - g.tab.light[g.gd[a].flashlight].pos[1]) });
            let carry = b.ask(|g| game::guards::hand_carry(g, a));
            if b.scripted(a) && (carry != game::guards::InHand::Torch || lit_up(a) || low < 0.2) { b.fail(format!("{} does not smoke with his torch low and switched off ({:?}, {}, {:.2} m under the shoulder)", b.called(a), carry, if lit_up(a) { "lit" } else { "out" }, low)).await; }
            if !b.close_in(a, Some([25.5, 0.0, 17.75])).await { b.fail(format!("could not get behind the first man ({:.2} m off, he stands at {}, {:?})", dist_xz(b.pos(), b.man_at(a)), at(b.man_at(a)), b.state(a))).await; }
            b.unseen(n0, a, "coming up behind him").await;
            if !b.scripted(a) { b.fail(format!("{} turned from his smoke as the player came up crouched ({:?})", b.called(a), b.state(a))).await; }
            let q = b.man_at(a);
            if !b.tap(InteractableKind::Guard, Verb::Grab, Some([q[0], 1.35, q[2]])).await { b.fail(String::from("no grab row behind the first man")).await; }
            if !b.until(|| b.held_fast(), 4.0).await { b.fail(format!("the grab did not take ({:?}; {} {:?}, {:.2} m off)", b.me(|p| p.holding().map(|h| h.phase)), b.called(a), b.state(a), dist_xz(b.pos(), b.man_at(a)))).await; }
            b.say(format!("  held {:.1} s after the whistle", b.t() - t0));
        }
        "ask" => {
            for _ in 0..8 {
                if b.has_code("pad_strong") { break; }
                if !b.tap(InteractableKind::Held, Verb::Question, None).await { b.fail(String::from("no question row with a man held")).await; }
                b.until(|| b.has_code("pad_strong"), 5.0).await;
            }
            if !b.has_code("pad_strong") { b.fail(String::from("eight questions and no code")).await; }
        }
        "drop" => {
            let a = man(b, "court");
            b.click(RIGHT, None).await;
            if !b.until(|| !b.up(a), 15.0).await { b.fail(String::from("the choke did not put him out")).await; }
            b.wait(1.0).await;
        }
        // the back door is three strides on from where the man was taken, past him
        "door" => { b.crouch(true).await; b.to_near(BACK_DOOR, 15.0, 0.2).await; b.say(format!("  at the back door: lit {:.2}", b.lit())); }
        // the key at the lock, then nothing until the cylinder goes over
        "pick" => {
            // a stray left click here, nobody in an arm and nobody in reach: the button is on since the last step, and it must do nothing a man could hear
            let shots = || b.seen(|g, s| g.fa[g.pl[s.player].rifle].shots_fired + g.fa[g.pl[s.player].pistol].shots_fired);
            let n = shots();
            b.click(LEFT, Some([26.0, 0.0, 18.6])).await;
            b.ticks(20).await;
            if shots() > n || !b.me(|p| p.holstered) { b.fail(format!("a stray left click at the lock {}", if shots() > n { "fired a round" } else { "drew the gun" })).await; }
            if !b.tap(InteractableKind::Door, Verb::Pick, Some([27.9, 1.0, 16.5])).await { b.fail(String::from("no row to pick the back door")).await; }
            let d = b.door("back");
            if !b.until(|| !b.locked_door(d), 14.0).await { b.fail(String::from("the back door did not pick")).await; }
        }
        "nook" => {
            let d = b.door("back");
            if !b.open_door(d, Verb::Push).await { b.fail(String::from("the back door did not open")).await; }
            b.until(|| !b.closed(d), 4.0).await;
            b.to([27.9, 0.0, 15.7], 15.0).await;
        }
        // the first line inside the door names the laser: the dark corner, on a knee
        "corner" => {
            let (h, n0) = (man(b, "hall"), b.sightings());
            // (the interaction key is his on this line: the back door, open behind him, has its row from inside and the row moves the leaf. The first build took the key here to shut the
            //  stalk's click, and a tap at this door did nothing and said nothing)
            let (d, on_door) = (b.door("back"), [27.9, 1.0, 16.5]);
            let a0 = b.angle(d);
            b.aim(on_door);
            b.ticks(20).await;
            let row = b.seen(|_, s| s.nearby.iter().find(|r| r.kind == InteractableKind::Door && r.door == Some(d) && r.verb.is_some() && r.in_reach).and_then(|r| r.verb));
            let Some(verb) = row else { b.fail(format!("no row for the open back door from inside it: the interaction key is dead on this line")).await; return };
            b.verb(InteractableKind::Door, verb, Some(on_door)).await;
            b.until(|| (b.angle(d) - a0).abs() > 0.1, 4.0).await;
            b.ticks(60).await;
            if (b.angle(d) - a0).abs() < 0.1 { b.fail(format!("the back door's row '{}' did not move the leaf from inside ({:.2} -> {:.2})", verb.as_str(), a0, b.angle(d))).await; }
            b.to_near(NOOK, 10.0, 0.12).await;
            b.wait(0.4).await;
            // (the whistle is not his in the store: it would bring the man round to him. Its own code does nothing, and where his actions are a ring, the interaction key held on open floor puts none up, with no wedge to pick, and a click whistles nobody)
            let blown = || b.ask(|g| g.stats.whistle.blown);
            let w0 = blown();
            b.press("KeyV", 6).await;
            b.hold(&[game::consts::INTERACT_KEY], 0.6, || false).await;
            let ring = b.seen(|_, s| s.hand.sel.open);
            b.click(LEFT, Some(b.chest(h))).await;
            b.ticks(20).await;
            if ring || blown() > w0 { b.fail(format!("in the store {} (a whistle would bring the hall's man round to him)", if blown() > w0 { "he whistled" } else { "the interaction key held put the hand's ring up" })).await; }
            if b.seen(|_, s| s.hand.sel.open) { b.press(game::backkey::BACK_KEY, 3).await; }
            b.say(format!("  in the corner: lit {:.2}; {} {:?}, aware {:.2}, {:.1} m off", b.lit(), b.called(h), b.state(h), b.awareness(h), dist_xz(b.pos(), b.man_at(h))));
            b.unseen(n0, h, "coming into the store").await;
        }
        // THE RIFLE'S LAUNCHER. The rifle off his back by its own key
        "rifle" => {
            let (sh, ri) = tube(b);
            if rifle_load(b) != Some(RifleAttachment::He40) || sh != 1.0 || ri != 5.0 { b.fail(format!("the mission's card is not on him: the launcher is up on {:?} with {} grenade and {} sponge rounds (one grenade, five sponge rounds, the grenade up)", rifle_load(b), sh, ri)).await; }
            // (his hands are empty, the interaction key is his and the man is in reach of a held left click: the frame gives that click to the stalk unless a step denies it, and this one does. Held from the doorway's line, where a stalk would have a way to him)
            let (h, t0) = (man(b, "hall"), trips(b));
            b.crouch(true).await;
            b.to_near(SHOT, 10.0, 0.15).await;
            let from = b.pos();
            let q = b.man_at(h);
            b.hold_button(LEFT, [q[0], 1.2, q[2]], 1.5).await;
            if b.holding() || trips(b) > t0 || dist_xz(b.pos(), from) > 0.1 { b.fail(format!("a left click held on the hall's man took him or walked the player at him (held {}, moved {:.2} m, the beam {})", b.holding(), dist_xz(b.pos(), from), if trips(b) > t0 { "broken" } else { "whole" })).await; }
            b.to_near(NOOK, 10.0, 0.15).await;
            b.take("Digit2").await;
            if !b.ask(|g| game::game::player_rifle_up(g, g.player)) { b.fail(String::from("the rifle's key did not bring it into his hands")).await; }
        }
        // the swap: 2 with the rifle up steps what is on the right button. The right button is not his yet: held on the man before the swap, nothing leaves the tube. (Tried from the doorway's
        // line, where the launcher HAS a line to him: from the corner no round reaches him whatever the frame says. Then back to the corner for the swap)
        "load" => {
            let (h, had) = (man(b, "hall"), tube(b));
            b.crouch(true).await;
            b.to_near(SHOT, 10.0, 0.15).await;
            b.hold_button(RIGHT, b.head(h), 0.5).await;
            b.wait(0.5).await;
            if tube(b) != had || b.state(h) != GuardState::Patrol { b.fail(format!("the right button fired on the line that swaps the load ({:?} -> {:?}; {} {:?})", had, tube(b), b.called(h), b.state(h))).await; }
            b.to_near(NOOK, 10.0, 0.15).await;
            b.press("Digit2", 20).await;
            if rifle_load(b) != Some(RifleAttachment::Ring) { b.press("Digit2", 10).await; }
            if rifle_load(b) != Some(RifleAttachment::Ring) { b.fail(format!("the rifle's key did not put the sponge round up ({:?})", rifle_load(b))).await; }
        }
        // out of the corner to the mark on the doorway's line, crouched: the corner itself has no line to him. (The reason this line exists is asked of the level itself, with no round spent)
        "behind" => {
            let (h, n0) = (man(b, "hall"), b.sightings());
            if clear_to(b, NOOK, h) || !clear_to(b, SHOT, h) { b.fail(format!("the corner {} line to his head and the mark {}: the line that sends him out of the corner rests on the first having none and the second having one", if clear_to(b, NOOK, h) { "has" } else { "has no" }, if clear_to(b, SHOT, h) { "has one" } else { "has none" })).await; }
            b.say(format!("  {} {:?} at {}, facing {:.2}, the chest beam {:.2} m behind him", b.called(h), b.state(h), at(b.man_at(h)), b.yaw(h), BEAM_HIGH - b.man_at(h)[0]));
            b.crouch(true).await;
            b.to_near(SHOT, 10.0, 0.15).await;
            b.ticks(10).await;
            b.say(format!("  on the doorway's line: lit {:.2}, {:.1} m behind him; he {:?}, aware {:.2}", b.lit(), dist_xz(b.pos(), b.man_at(h)), b.state(h), b.awareness(h)));
            b.unseen(n0, h, "stepping out to the doorway's line").await;
        }
        // the ring let go at the back of his head
        "sponge" => {
            let (h, n0, t0, had) = (man(b, "hall"), b.sightings(), trips(b), tube(b).1);
            b.hold_button(RIGHT, b.head(h), 0.5).await;
            if tube(b).1 != had - 1.0 { b.fail(format!("the right button let no sponge round go ({} left of {})", tube(b).1, had)).await; }
            if !b.until(|| !b.up(h), 3.0).await { b.fail(format!("the sponge round did not put the hall's man out ({:?}, aware {:.2})", b.state(h), b.awareness(h))).await; }
            b.wait(1.5).await;
            if trips(b) > t0 { b.fail(String::from("the ring broke the chest beam")).await; }
            let ko = b.ask(|g| g.chronicle.events.iter().any(|e| e.kind == game::chronicle::ChronKind::Ko && matches!(e.data.of("method"), Some(Some(game::chronicle::ChronEventDataValue::Text(m))) if m == "ring")));
            if !ko { b.fail(String::from("the hall's man is down and the night's record has no man put out by a ring")).await; }
            if b.sightings() > n0 { b.fail(String::from("the shift had eyes on him at the shot")).await; }
            let stirred: Vec<String> = b.men().into_iter().filter(|&x| b.up(x) && b.state(x) != GuardState::Patrol).map(|x| format!("{} {:?}", b.called(x), b.state(x))).collect();
            if !stirred.is_empty() { b.fail(format!("{} after the hall's man went down", stirred.join(", "))).await; }
        }
        // the rifle away again: the hands' own key
        "sling" => {
            b.press(game::handslot::HANDSLOT_KEY, 10).await;
            b.until(|| b.me(|p| p.holstered), 3.0).await;
            if !b.me(|p| p.holstered) { b.fail(String::from("the hands' key did not put the rifle away")).await; }
        }
        // through the doorway into the lit hall and under the chest beam on a knee
        "under" => {
            let alarms = || b.ask(|g| g.chronicle.events.iter().filter(|e| e.kind == game::chronicle::ChronKind::Alarm).count());
            let (a0, t0) = (alarms(), trips(b));
            b.crouch(true).await;
            b.to([26.2, 0.0, 12.6], 10.0).await;
            b.to([24.4, 0.0, 13.0], 10.0).await;
            b.to_near(UNDER, 10.0, 0.2).await;
            if alarms() > a0 || trips(b) > t0 { b.fail(String::from("the chest beam was tripped on a knee")).await; }
        }
        "nv" => b.press("KeyN", 20).await,
        // along the hall to the box on its wall, a stride inside the counting room's doorway
        "box" => {
            b.to_near(BOX_AT, 12.0, 0.2).await;
            if !b.tap(InteractableKind::Breaker, Verb::BreakerOff, Some(BOX)).await { b.fail(String::from("no row at the mains box")).await; }
            if !b.until(|| b.dark(), 2.0).await { b.fail(String::from("the mains did not go")).await; }
            // a person's beat, then round the end of the niche's wall and in behind it
            b.wait(REACT + 0.5).await;
            b.crouch(true).await;
            b.to([21.0, 0.0, 14.6], 10.0).await;
            b.to([21.0, 0.0, 14.1], 10.0).await;
            b.to_near(WAIT, 10.0, 0.15).await;
            b.say(format!("  in the niche: lit {:.2}", b.lit()));
        }
        // whoever is sent comes through the doorway to the box, and is taken at it from behind
        "boxman" => {
            let n0 = b.sightings();
            let sent = b.men().into_iter().find(|&x| b.up(x) && b.ask(|g| matches!(&g.gd[x].task, Some(game::game::GuardTask::Arm0(t)) if t.kind == game::game::GuardTaskArm0Kind::Breaker)));
            let c = sent.unwrap_or_else(|| man(b, "table"));
            let came = b.until(|| { let q = b.man_at(c); (q[0] > 20.2 && dist_xz(q, [BOX[0], 0.0, BOX[2] - 0.6]) < 1.3) || b.state(c) == GuardState::Alert || !b.up(c) || !b.dark() }, 40.0).await;
            stood_still(b, c, 0.25, 4.0).await;   // he stops at the box, his back to the room: only then does anything move
            let t0 = b.t();
            if !came || !b.dark() { b.fail(format!("{} {} ({:?}, at {})", b.called(c), if came { "had the mains back in before he could be reached" } else { "did not come to the box" }, b.state(c), at(b.man_at(c)))).await; }
            b.unseen(n0, c, "in the niche").await;
            b.wait(REACT).await;
            b.to_near([21.0, 0.0, 14.15], 8.0, 0.2).await;   // round the end of the niche's wall: he stands a stride on, his back to it
            for _ in 0..8 {   // (he may still be stepping up to the lever)
                if dist_xz(b.pos(), b.man_at(c)) < 0.8 { break; }
                let q = b.man_at(c);
                b.nudge([q[0], 0.0, q[2]], 0.4, || dist_xz(b.pos(), b.man_at(c)) < 0.8).await;
            }
            b.ticks(4).await;
            b.say(format!("  in reach {:.2} m off, {:.1} s after he got to the box; he {:?}", dist_xz(b.pos(), b.man_at(c)), b.t() - t0, b.state(c)));
            b.click(RIGHT, None).await;
            if !b.until(|| !b.up(c), 4.0).await { b.fail(format!("the right button did not put the man at the box down ({:?}, {:.2} m off)", b.state(c), dist_xz(b.pos(), b.man_at(c)))).await; }
            if !b.dark() { b.fail(String::from("he had the mains back in before he went down")).await; }
            b.wait(1.0).await;
        }
        "code" => {
            b.to([13.25, 0.0, 14.6], 20.0).await;
            b.to([13.25, 0.0, 12.85], 10.0).await;
            b.to([16.9, 0.0, 12.85], 10.0).await;
            b.to_near([17.3, 0.0, 10.5], 20.0, 0.2).await;
            if !b.tap(InteractableKind::Door, Verb::Code, Some([16.5, 1.0, 10.3])).await { b.fail(String::from("no row to type the code")).await; }
            let d = b.door("strong");
            if !b.until(|| !b.locked_door(d) || !b.closed(d), 10.0).await { b.fail(String::from("the strongroom did not open to the code")).await; }
        }
        "safe" => {
            b.to([15.4, 0.0, 10.3], 20.0).await;
            b.to_near([14.5, 0.0, 9.9], 10.0, 0.2).await;
            if !b.tap(InteractableKind::Objective, Verb::Drive, Some(SAFE_FRONT)).await { b.fail(String::from("no row at the safe")).await; }
            b.until(|| b.stage() == MissionStage::Exfil, 20.0).await;
        }
        "out" => { b.crouch(false).await; b.to([16.9, 0.0, 12.85], 20.0).await; b.to([13.25, 0.0, 12.85], 10.0).await; b.to([13.4, 0.0, 15.0], 20.0).await; b.to([11.4, 0.0, 15.1], 20.0).await; }
        "gate" => { b.to([9.0, 0.0, 14.0], 20.0).await; b.to_by([7.5, 0.0, 14.0], 30.0, near(0.25), || false).await; }
        _ => panic!("tut-quiet has no deed for the step '{}'", id),
    }
}
