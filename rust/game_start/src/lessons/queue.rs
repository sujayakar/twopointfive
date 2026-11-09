//! The tutorial's fourth mission. He is on the floor with his four and has his feet, his rifle, the command key and the word; what each step asks goes through the input only: a walk to where the thing
//! a line names is on the picture, the cursor on a door or a spot, the command key tapped or kept down with or without Shift held, a wedge's digit, the word. Game state is read to time a press and to
//! find where a thing is on the picture, never to do the deed.
use super::{Bot, Ended, Tutorial};
use crate::player::{at, Walk, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use game::ids::GuardId;
use game::orders::OrderTarget;

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-queue",
    // (`split` asks for the road gate again where it has been called off: with it still watched as `gateB` ends, it is done as it comes up)
    idle_ok: &["split"],
    fight_ok: &[],
    ended: Ended::Finished,
    play: |b, id| Box::pin(deed(b, id)),
    finish: None,
};

fn stacked_on(b: &Bot, m: GuardId) -> Option<String> { b.order_of(m).filter(|a| a.order == "stack").and_then(|a| a.door).map(|d| b.name(d)) }
fn on(b: &Bot, door: &str) -> Vec<GuardId> { b.squad().into_iter().filter(|&m| stacked_on(b, m).as_deref() == Some(door)).collect() }
fn set_on(b: &Bot, doors: &[&str]) -> bool { doors.iter().all(|d| !on(b, d).is_empty()) && b.squad().into_iter().filter(|&m| stacked_on(b, m).is_some()).all(|m| b.order_of(m).is_some_and(|a| a.arrived)) }
fn names(b: &Bot, l: &[GuardId]) -> String { if l.is_empty() { String::from("nobody") } else { l.iter().map(|&m| b.called(m)).collect::<Vec<_>>().join(", ") } }
/// a place on the picture, clear of its edges and of the squad's panel on the left
fn shown(b: &Bot, p: V3) -> bool { let (x, y, _) = b.screen(p); let [w, h] = b.client(); x > 240.0 && x < w - 60.0 && y > 70.0 && y < h - 110.0 }
fn mark_of(b: &Bot, id: &str) -> V3 { b.step(id).at.unwrap_or_else(|| panic!("the step '{}' has no mark", id)) }
/// HE WALKS UNTIL THE THING THE LINE NAMES IS ON HIS PICTURE (a hand follows the arrow at the picture's edge), and says how far that was
async fn see(b: &Bot, p: V3, what: &str) {
    if shown(b, p) { return; }
    b.to_by([p[0], 0.0, p[2]], 30.0, Walk::doors(), || shown(b, p)).await;
    b.wait(0.3).await;
    if !shown(b, p) { b.fail(format!("{} at {} never came onto the picture: he stands at {}", what, at(p), at(b.pos()))).await; }
}
/// the cursor on a step's own mark: the ring the line points at
async fn on_mark(b: &Bot, id: &str, what: &str) -> V3 { let p = mark_of(b, id); see(b, p, what).await; b.aim(p); b.ticks(2).await; p }
/// the command key tapped where the cursor is, Shift held or not, as a hand presses it
async fn key(b: &Bot, shift: bool) {
    if !shift { b.press("KeyB", 4).await; return; }
    b.hold(&["ShiftLeft"], 3.0 / 60.0, || false).await;
    b.down("KeyB");
    b.hold(&["ShiftLeft", "KeyB"], 2.0 / 60.0, || false).await;
    b.hold(&["ShiftLeft"], 3.0 / 60.0, || false).await;
    b.ticks(3).await
}
async fn door_under(b: &Bot, id: &str, door: &str) {
    on_mark(b, id, &format!("the door '{}'", door)).await;
    let d = b.door(door);
    let on_it = matches!(b.under(), OrderTarget::Door(t) if t.door == d);
    if !on_it { b.fail(format!("the cursor on the ring of step '{}' is not on the door '{}'", id, door)).await; }
}
/// the plain key on a door: everybody stacks on it
async fn all(b: &Bot, id: &str, door: &str) {
    door_under(b, id, door).await;
    key(b, false).await;
    b.ticks(6).await;
    if on(b, door).len() != b.squad().len() { b.fail(format!("the key on '{}' stacked {} of {} ({})", door, names(b, &on(b, door)), b.squad().len(), b.where_line())).await; }
}
/// Shift with the key on a second door: one pair is parted off to it and the other stays on the first
async fn part(b: &Bot, id: &str, door: &str, first: &str) {
    door_under(b, id, door).await;
    key(b, true).await;
    b.ticks(6).await;
    let (x, y) = (on(b, first), on(b, door));
    let one_pair = y.len() == 2 && b.ask(|g| game::orders::squad_orders_pair_of(g, y[0]) == game::orders::squad_orders_pair_of(g, y[1]));
    if x.len() != 2 || !one_pair { b.fail(format!("Shift with the key on '{}': on '{}' {}, on '{}' {}: not one whole pair on each", door, first, names(b, &x), door, names(b, &y))).await; }
    b.say(format!("  Shift with the key on '{}': {} go to it, {} stay on '{}'", door, names(b, &y), names(b, &x), first));
}
/// the word, once every stack stands on its door: ONE press, and men are through every door of it together
async fn word(b: &Bot, doors: &[&str], room: &str) {
    if !b.until(|| set_on(b, doors) || b.cleared(room), 70.0).await { b.say(String::from("  the stacks never all stood on their doors: the word anyway")); }
    if b.cleared(room) { b.fail(format!("{} fell before the word: a stack went in without it, and nothing of the word could be measured", room)).await; }
    let step = if room == "hall" { "word2" } else { "word" };
    if !b.step_is(step) { b.until(|| b.step_is(step), 12.0).await; }
    b.wait(0.4).await;
    let stacks: Vec<(&str, Vec<GuardId>)> = doors.iter().map(|&d| (d, on(b, d))).collect();
    b.press("KeyF", 3).await;
    b.ticks(6).await;
    let t0 = b.t();
    let r = b.ask(|g| g.staged.rooms.iter().find(|r| r.def.id == room).map(|r| [r.def.bounds.x0, r.def.bounds.x1, r.def.bounds.z0, r.def.bounds.z1]));
    let inside = |m: GuardId| r.is_some_and(|r| { let p = b.man_at(m); p[0] > r[0] + 0.2 && p[0] < r[1] - 0.2 && p[2] > r[2] + 0.2 && p[2] < r[3] - 0.2 });
    let daz0: Vec<(GuardId, f64)> = b.squad().into_iter().map(|m| (m, b.ask(|g| g.gd[m].dazzled_until))).collect();
    let (mut in_at, mut hit): (Vec<Option<f64>>, Vec<String>) = (doors.iter().map(|_| None).collect(), Vec::new());
    let watch = |in_at: &mut Vec<Option<f64>>, hit: &mut Vec<String>| {
        for (k, s) in stacks.iter().enumerate() { if in_at[k].is_none() && s.1.iter().any(|&m| inside(m)) { in_at[k] = Some(b.t() - t0); } }
        for &(m, u) in &daz0 { if b.ask(|g| g.gd[m].dazzled_until) > u + 1e-6 { let n = b.called(m); if !hit.contains(&n) { hit.push(n); } } }
    };
    let end = b.t() + 40.0;
    while b.t() < end - 1e-6 { watch(&mut in_at, &mut hit); if b.cleared(room) { break; } b.tick().await; }
    for _ in 0..180 { watch(&mut in_at, &mut hit); b.tick().await; }
    let out = in_at.iter().any(|t| t.is_none());
    let times: Vec<f64> = in_at.iter().flatten().copied().collect();
    let apart = if doors.len() > 1 && !out { times.iter().copied().fold(f64::MIN, f64::max) - times.iter().copied().fold(f64::MAX, f64::min) } else { 0.0 };
    let said = format!("the word at {}: {} · {:.1} s apart · teammates dazzled as they went in: {}{}", room, doors.iter().zip(&in_at).map(|(d, t)| match t { Some(t) => format!("{} first man in at {:.1} s", d, t), None => format!("{} NOBODY IN", d) }).collect::<Vec<_>>().join(" · "), apart, hit.len(), if hit.is_empty() { String::new() } else { format!(" ({})", hit.join(", ")) });
    if out || apart > 1.0 || !hit.is_empty() { b.fail(said.clone()).await; }
    b.say(format!("  {}", said));
    for _ in 0..2 { if b.cleared(room) { break; } b.wait(0.6).await; b.press("KeyF", 3).await; b.until(|| b.cleared(room), 40.0).await; }
}
/// who watches what: the men on a cover order, by the point each is turned to
fn watches(b: &Bot) -> String {
    b.squad().into_iter().map(|m| { let a = b.order_of(m); format!("{} -> {}", b.called(m), match a { Some(a) if a.order == "cover" && a.watch.is_some() => at(a.watch.unwrap()), Some(a) => a.order, None => String::from("no order") }) }).collect::<Vec<_>>().join(" · ")
}
/// he walks to a step's ring, and both things the next two lines name are on his picture from it with the view at rest (else the spot is the level's to move)
async fn spot(b: &Bot, id: &str, next: &[&str], via: &[V3]) {
    let p = mark_of(b, id);
    for &v in via { b.to_by(v, 30.0, Walk::near(0.6), || false).await; }
    b.to_by(p, 40.0, Walk::default(), || false).await;
    b.until(|| !b.step_is(id), 6.0).await;
    b.aim(b.pos());
    b.wait(0.5).await;
    let off: Vec<&&str> = next.iter().filter(|n| !shown(b, mark_of(b, n))).collect();
    if !off.is_empty() { b.fail(format!("from the ring of '{}' the mark of {:?} is off the picture", id, off)).await; }
}
fn note(b: &Bot) -> String { b.prompt().and_then(|p| p.note).unwrap_or_default() }
/// THE WHEEL FOR A PART, ON A STEP'S SPOT: Shift and the key kept down past the hold open the ring for the men a tap would have sent, and its second wedge turns them to the spot. `tap_first`: a Shift tap
/// first, which gives nothing on a step that teaches the key held and says to keep the keys down
async fn wheel(b: &Bot, id: &str, what: &str, tap_first: bool) {
    if tap_first {
        on_mark(b, id, what).await;
        let (before, n0) = (watches(b), b.orders_log());
        key(b, true).await;
        b.ticks(10).await;
        let n = note(b);
        if watches(b) != before || b.orders_log() != n0 || !n.contains("held down") { b.fail(format!("a Shift tap on the step that teaches the key kept down: {} · the note \"{}\"", watches(b), n)).await; }
    }
    on_mark(b, id, what).await;
    b.hold(&["ShiftLeft"], 3.0 / 60.0, || false).await;
    b.down("KeyB");
    b.hold(&["ShiftLeft", "KeyB"], 0.7, || false).await;
    let (part, up) = b.ask(|g| (g.orders.menu.as_ref().is_some_and(|m| m.part.is_some()), g.orders.menu.is_some()));
    if !part { b.fail(format!("Shift and the key kept down by {} opened {}", what, if up { "the ring for everyone" } else { "no ring" })).await; }
    let line = b.prompt().map_or(String::new(), |p| p.line);
    if !line.starts_with("Press") { b.fail(format!("the ring is up on the step's spot and the line is still \"{}\"", line)).await; }
    b.down("Digit2");
    b.hold(&["ShiftLeft", "KeyB", "Digit2"], 2.0 / 60.0, || false).await;
    b.hold(&["ShiftLeft", "KeyB"], 3.0 / 60.0, || false).await;
    b.ticks(10).await;
    b.say(format!("  its second wedge: {}", watches(b)));
}

async fn deed(b: &Bot, id: &str) {
    match id {
        "corner" => spot(b, "corner", &["door1", "door2"], &[]).await,
        "door1" => all(b, "door1", "guard").await,
        "door2" => { part(b, "door2", "guard_side", "guard").await; b.until(|| !b.step_is("door2"), 60.0).await; }
        "word" => { word(b, &["guard", "guard_side"], "guardroom").await; b.wait(1.5).await; }
        "front" => all(b, "front", "hall").await,
        // (round the hall's blind west wall: a body walked into the front door opens it, and the gun is on that doorway)
        "around" => spot(b, "around", &["back", "word2"], &[[23.3, 0.0, 13.4], [23.3, 0.0, 20.4]]).await,
        "back" => { part(b, "back", "hall_back", "hall").await; b.until(|| !b.step_is("back"), 70.0).await; }
        "word2" => { word(b, &["hall", "hall_back"], "hall").await; b.wait(1.5).await; }
        "call" => {
            b.say(format!("  as the hall falls: {}", watches(b)));
            b.aim(b.pos());
            b.ticks(2).await;
            key(b, false).await;
            b.ticks(6).await;
            b.say(format!("  the key on himself: {}", watches(b)));
        }
        "yard" => { spot(b, "yard", &["gateA", "gateB"], &[]).await; b.wait(2.0).await; }
        // (A SPOT IS WATCHED FROM THE WHEEL: a tap gives nothing on these steps and says to keep the keys down; the pair kept down past the hold is the ring, named for the men a tap would have sent, and its second wedge is theirs)
        "gateA" => wheel(b, "gateA", "the road gate", true).await,
        "gateB" => wheel(b, "gateB", "the back gate", true).await,
        "split" => { if b.step_is("split") { wheel(b, "split", "the road gate", false).await; } }
        "hold" => { b.until(|| (b.cleared("road") && b.cleared("lane")) || b.lesson().is_some_and(|v| v.finished), 120.0).await; b.wait(1.0).await; }
        _ => panic!("tut-queue has no deed for the step '{}'", id.to_string()),
    }
}
