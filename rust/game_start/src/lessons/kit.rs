//! The tutorial's third mission: what each line asks, through the input only, in the order a new player would do it. He is mortal in his own half: the fights are the level's script's and none of
//! them is aimed at a man who does what the line says. In the squad's half he gives the orders the lines ask for (the command key tapped on a thing, held for its ring and a digit, the word) and
//! shoots only on the last line.
use super::{Bot, Ended, Tutorial};
use crate::player::{at, flat, Walk, LEFT, RIGHT, V3};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use game::game::{InteractableKind, Verb};
use game::ids::GuardId;
use game::throwables::ItemKind;

pub static TUTORIAL: Tutorial = Tutorial {
    id: "tut-kit",
    // (the grenade took both of the launcher's two, the frag both of the yard's; a man who picked the charge with the ring's first showing has done the two lines after it; his one frag away, the
    //  pistol is in his hands already)
    idle_ok: &["pistol", "last", "tab", "charge", "clear", "lights"],
    // (a line inside the squad's own fight, which the room's fall may finish before he has pressed its key)
    fight_ok: &["breach", "clear"],
    ended: Ended::Finished,
    play: |b, id| Box::pin(deed(b, id)),
    finish: None,
};

// the level's marks (tools/levels/tut-kit.ts)
const AXIS: f64 = 14.25;
const JAMB: V3 = [15.1, 0.0, 11.7];
const INNER_X: f64 = 12.5;
const INNER_Z: f64 = 9.25;
const INNER_FROM: V3 = [11.55, 0.0, INNER_Z];
const BAGS_END: V3 = [20.2, 0.0, 13.6];
const GAP: V3 = [21.9, 0.1, 12.7];
const LAMP: V3 = [24.4, 3.2, 17.7];
const BARRIER: V3 = [23.3, 0.0, 13.9];
const BACK: V3 = [24.45, 0.0, 16.5];
const NEST: V3 = [22.6, 0.4, 7.0];
const SHOP_DOOR: V3 = [25.0, 1.0, AXIS];
const STORE_DOORWAY: V3 = [31.6, 0.2, AXIS];
const STUN_AT: V3 = [33.4, 0.1, AXIS];
const ARCH_MOUTH: V3 = [16.5, 0.0, AXIS];
const LAUNCH_FROM: V3 = [16.1, 0.0, AXIS + 1.0];
const LAST_MEN: [V3; 2] = [[25.7, 0.0, 13.98], [26.5, 0.0, 14.5]];
/// (x0, x1, z0, z1)
const INSIDE: [f64; 4] = [12.9, 24.7, 5.3, 22.7];
const BREAKER: V3 = [19.4, 1.35, 5.165];
const BUNK_DOOR: V3 = [27.75, 1.0, 10.8];
const WALL_MARK: V3 = [30.95, 1.1, 6.6];
const FRAG_AT: V3 = [29.6, 0.05, 19.8];
const YARD_FROM: V3 = [23.8, 0.02, 19.75];

fn own_ring(b: &Bot) -> bool { b.me(|p| p.sel.open && p.sel.ring.is_some()) }
fn last_two(b: &Bot) -> Vec<GuardId> { b.named(&["Brandt", "Marsh"]) }
/// the command key HELD on a point until the squad's ring is up, then the digits the line that is up names, in order
async fn ring(b: &Bot, p: V3) -> bool {
    b.aim(p);
    b.ticks(2).await;
    b.aim(p);
    b.down("KeyB");
    b.hold(&["KeyB"], 0.45, || false).await;
    b.ticks(2).await;
    let Some(flat_ring) = b.ask(|g| g.sets[g.seat].orders.menu.as_ref().map(|m| m.flat == Some(true))) else { b.say(String::from("  the squad's ring did not come up")); return false };
    let said = b.ask(|g| game::lesson::lesson_step(g).and_then(|t| if flat_ring { t.flat_line } else { t.menu_line })).unwrap_or_default();
    let rows = || b.ask(|g| g.sets[g.seat].orders.menu.as_ref().map_or(String::from("no ring"), |m| m.rows.iter().map(|&r| g.tab.orders_order_def[r].id.clone()).collect::<Vec<_>>().join(" · ")));
    let digits = Bot::digits(&said);
    b.say(format!("  the ring is up: '{}' -> {:?} · {}", said, digits, rows()));
    for d in digits { b.press(&format!("Digit{}", d), 4).await; b.say(format!("    [{}] -> {}", d, rows())); }
    true
}

async fn deed(b: &Bot, id: &str) {
    match id {
        // ---- his own kit, alone ----
        // the four stay at the truck: he walks in by the gate to the wooden door
        "gate" => {
            b.to(INNER_FROM, 30.0).await;
            let far: Vec<GuardId> = b.squad().into_iter().filter(|&m| b.man_at(m)[0] > 9.2).collect();
            if !far.is_empty() { b.fail(format!("{} followed him in through the gate: the four wait at the truck until he calls them ({})", far.len(), b.orders_line())).await; }
        }
        // a TAP of the interaction key, and the tap's row is the door's
        "open" => {
            let c = [INNER_X, 1.0, INNER_Z];
            b.aim(c);
            b.ticks(20).await;
            let top = b.seen(|_, s| s.nearby.iter().find_map(|r| r.verb));
            if top != Some(Verb::Push) { b.fail(format!("a tap at the wooden door gives {:?}, not the door opened", top)).await; }
            b.press(game::consts::INTERACT_KEY, 3).await;
            let d = b.door("inner");
            if !b.until(|| !b.closed(d), 5.0).await { b.fail(String::from("a tap of the interaction key did not open the wooden door")).await; }
        }
        // through the doorway and up beside the blue gate; then down, as the gun starts along the sandbags
        "arch" => { b.to([INNER_X + 1.0, 0.0, INNER_Z], 20.0).await; b.to(JAMB, 20.0).await; }
        "low" => { b.wait(1.2).await; b.crouch(true).await; b.to([16.9, 0.0, 13.3], 12.0).await; b.to(BAGS_END, 15.0).await; }
        "smoke" => { b.take("Digit3").await; b.click(LEFT, Some(GAP)).await; b.landed(ItemKind::Smoke, 4.0).await; b.wait(2.8).await; }
        // when the plume is up: across the gap, behind the barrier
        "cross" => { b.to_near(BARRIER, 10.0, 0.5).await; }
        // over the barrier and into the nest; a second one if the first fell short
        "frag" => {
            b.crouch(false).await;
            for _ in 0..2 {
                if b.downed() >= 1 { break; }
                b.take("Digit5").await;
                b.click(LEFT, Some(NEST)).await;
                b.landed(ItemKind::Frag, 4.5).await;
                b.until(|| b.downed() >= 1, 4.0).await;   // (the fuse runs on after it lands)
            }
            if b.downed() < 1 {
                let x = b.named(&["Novak"]).first().copied();
                b.take("Digit2").await;
                b.to_by([24.9, 0.0, 11.6], 10.0, Walk::doors(), || x.is_some_and(|x| b.line(x))).await;
                b.fight(1, 20.0, async || b.tick().await, None).await;
            }
        }
        "pistol" => b.take("Digit1").await,
        // the lamp on its pole by the shop's front, from behind the barrier: a round a press until it is dark
        "lamp" => {
            let t = b.ask(|g| g.targets.iter().copied().find(|&f| g.tab.fixture[f].name == "lamp_shop"));
            let Some(t) = t else { b.fail(String::from("no lamp_shop among the light targets")).await; return };
            let lit = || b.ask(|g| g.tab.fixture[t].on && !g.tab.fixture[t].broken && g.tab.fixture[t].factor > 0.05);
            for _ in 0..10 {
                if !lit() { break; }
                b.aim(LAMP);
                b.ticks(20).await;
                b.click(LEFT, Some(LAMP)).await;
                b.ticks(12).await;
                if b.rounds() == 0.0 { b.press("KeyR", 1).await; b.until(|| !b.reloading(), 5.0).await; }
            }
            if lit() { b.fail(String::from("ten pistol rounds at the lamp and it is still lit")).await; }
        }
        // the interaction key HELD on the locked door: his ring, with the one row the level lets this door offer
        "ring" => {
            b.to([24.2, 0.0, AXIS + 0.2], 10.0).await;
            b.aim(SHOP_DOOR);
            b.ticks(30).await;
            let rows: Vec<&str> = b.seen(|_, s| s.nearby.iter().filter(|r| r.kind == InteractableKind::Door).map(|r| r.verb.map_or("-", |v| v.as_str())).collect());
            b.say(format!("  the door's rows: {}", rows.join(" ")));
            if rows.join(" ") != "charge" { b.fail(format!("the shop's door offers '{}', not the charge alone", rows.join(" "))).await; }
            b.down(game::consts::INTERACT_KEY);
            b.hold(&[game::consts::INTERACT_KEY], 0.5, || own_ring(b)).await;
            b.ticks(3).await;
            if !own_ring(b) { b.fail(String::from("the interaction key held on the locked door put no ring up")).await; }
        }
        "tab" => {
            if !own_ring(b) { b.fail(String::from("his ring came down by itself before the line that asks for Tab")).await; }
            b.press("Tab", 6).await;
            if own_ring(b) { b.fail(String::from("Tab did not put his ring away")).await; }
            if b.seen(|_, s| !s.door_tools.set.is_empty()) { b.fail(String::from("Tab set the charge")).await; }
        }
        "charge" => {
            if !b.verb(InteractableKind::Door, Verb::Charge, Some(SHOP_DOOR)).await { b.fail(String::from("no 'charge' row on the shop's door")).await; }
            b.until(|| b.me(|p| p.charging.is_none() && p.kick.is_none()), 12.0).await;
            b.ticks(20).await;
        }
        "back" => { b.to_near(BACK, 10.0, 0.5).await; b.ticks(20).await; }
        // (his own charge is a row of his own list, and stood clear it is the only one, so a tap gives it)
        "blow" => {
            let warned = b.seen(|g, s| game::view::tick_view(g, s).corner.iter().any(|n| n.text.find("TOO CLOSE:").is_some_and(|i| n.text[i..].split('·').next().unwrap_or("").split(|c: char| !c.is_alphanumeric()).any(|w| w == "you"))));
            if warned { b.fail(String::from("the mark that says stand clear is inside his own charge's backblast")).await; }
            b.press(game::consts::INTERACT_KEY, 30).await;
        }
        // out from beside the door, square to the doorway (from its edge the rounds go into the jamb), and the pistol
        "shop" => {
            b.to([23.5, 0.0, AXIS + 0.05], 6.0).await;
            let who = b.named(&["Tango 1"]);
            if !b.fight(2, 25.0, async || b.ticks(6).await, Some(&who)).await { b.fail(String::from("the man in the shop did not drop")).await; }
        }
        // from the shop's floor, the stun grenade through the doorway to the store, and a step aside
        "stun" => {
            b.to([26.8, 0.0, AXIS], 10.0).await;
            b.take("Digit4").await;
            b.click(LEFT, Some(STUN_AT)).await;
            b.until(|| b.thrown(ItemKind::Flash), 2.0).await;   // (once it has left his hand: the cursor is the throw's aim until it does)
            b.to([26.8, 0.0, AXIS + 1.5], 4.0).await;
            b.landed(ItemKind::Flash, 3.4).await;
            if b.seen(|g, s| g.cond[g.pl[s.player].ch].blind_until > g.time) { b.fail(String::from("he stepped aside as the line says and his own stun grenade still blinded him")).await; }
        }
        // the pistol, and INTO the room (a gun in a doorway has no room to come up across the jamb)
        "store" => {
            b.take("Digit1").await;
            b.to([STORE_DOORWAY[0], 0.0, AXIS], 6.0).await;
            if !b.fight(4, 30.0, async || b.nudge(STORE_DOORWAY, 0.3, || false).await, None).await { b.fail(String::from("the two in the store did not drop")).await; }
        }
        "rifle" => b.take("Digit2").await,
        // back out through the shop and the gun court to the arch's mouth. (He stops as the line changes, as a person does: the ring's near edge, the closest the line lets him be to the doorway)
        "leave" => {
            let d = b.door("store");
            if !b.locked_door(d) || b.angle(d).abs() > 0.05 { b.fail(String::from("the store's back door is open or unlocked before the two for the launcher come by it")).await; }
            b.to_by(LAUNCH_FROM, 40.0, Walk::doors(), || !b.step_is("leave")).await;
        }
        // the line that only has him look: it goes by once the two stand in the shop's doorway, and not before
        "behind" => {
            b.until(|| !b.step_is("behind"), 9.0).await;
            if b.step_is("behind") { return; }
            let men: Vec<GuardId> = last_two(b).into_iter().filter(|&x| b.up(x)).collect();
            let off = men.iter().enumerate().filter(|(k, &x)| flat(b.man_at(x), LAST_MEN[*k]) > 0.8).count();
            if men.len() < 2 || off > 0 { b.fail(format!("the shot is asked for before the two stand in the doorway ({} up, {} still on the way)", men.len(), off)).await; }
        }
        // the two in the shop's doorway behind him: the grenade into it, once both stand there (8.5 m and more: armed)
        "launch" => {
            b.until(|| { let m = last_two(b); m.len() >= 2 && m.iter().enumerate().all(|(k, &x)| !b.up(x) || flat(b.man_at(x), LAST_MEN[k]) < 0.8) }, 12.0).await;
            let Some(x) = last_two(b).into_iter().find(|&y| b.up(y)) else { b.fail(String::from("nobody came out through the shop")).await; return };
            if b.angle(b.door("store")).abs() <= 1.0 { b.fail(String::from("the two are in the doorway and the store's back door is not open: they did not come by it")).await; }
            let on = b.ask(|g| game::weapons::rifle_attachment(g, g.pl[g.player].rifle));
            if on != Some(game::weapons::RifleAttachment::He40) { b.fail(format!("the launcher is on {:?}: the lesson has no line that swaps it", on)).await; }
            b.wait(0.5).await;
            b.hold_button(RIGHT, b.chest(x), 0.5).await;
            b.wait(1.5).await;
            if b.ask(|g| g.stats.he40.duds > 0) { b.fail(String::from("the launcher's grenade was a dud from where the line before stood him: the doorway is inside the 8 m it needs to arm")).await; }
        }
        // whoever the grenade left: the rifle, from the arch, and after him if he will not come
        "last" => {
            let who = last_two(b);
            b.fight(6, 60.0, async || {
                let Some(x) = b.foes().into_iter().find(|&y| b.up(y) && !b.scripted(y)) else { b.ticks(10).await; return };
                let p = b.man_at(x);
                if flat(p, b.pos()) > 14.0 { b.to_by([p[0], 0.0, p[2]], 1.5, Walk::doors(), || b.line(x)).await; } else { b.ticks(10).await; }
            }, Some(&who)).await;
        }

        // ---- the squad's kit, on his word ----
        // a tap of the command key on himself: the four come up from the truck, at a run
        "call" => {
            let (home, all) = (b.squad().into_iter().filter(|&m| b.man_at(m)[0] < 9.2).count(), b.squad().len());
            if home < all { b.fail(format!("the four did not wait at the truck ({})", b.orders_line())).await; }
            b.to([ARCH_MOUTH[0] + 1.6, 0.0, AXIS], 10.0).await;
            let me = b.pos();
            b.command([me[0], 1.0, me[2]]).await;
            if !b.until(|| b.men_in(INSIDE) >= 3.min(b.squad().len()), 60.0).await { b.fail(format!("the four did not come up to him ({}) · he is at {}", b.where_line(), at(b.pos()))).await; }
            b.until(|| !b.step_is("call"), 6.0).await;   // (the line holds a moment once they are in)
        }
        // (the line says a buddy team goes: one man works the box and the other of his pair covers him from behind)
        "breaker" => {
            for _ in 0..3 {
                if b.dark() { break; }
                // (the box is off the top of the picture from where the last line stood him: he walks until it is on it, as a hand follows the arrow at the picture's edge)
                b.to_by([BREAKER[0], 0.0, BREAKER[2] + 0.6], 30.0, Walk::doors(), || b.on_screen(BREAKER)).await;
                b.ticks(10).await;
                b.command(BREAKER).await;
                let worker = || b.squad().into_iter().find(|&m| b.ordered(m) == "work");
                b.until(|| worker().is_some() || b.dark(), 1.0).await;
                b.say(format!("  the key on the box from {} gave {:?}", at(b.pos()), b.last_order()));
                if let Some(w) = worker() { b.say(format!("  the breaker box: {} works it", b.called(w))); }
                b.until(|| b.dark(), 30.0).await;
            }
            if !b.dark() { b.fail(format!("a tap of the command key on the breaker box did not put the lights out ({})", b.orders_line())).await; }
        }
        "nv" => b.press("KeyN", 10).await,
        // the command key HELD on the bunk room's door, and the wedge the line names; then the word, if they wait on it
        "door" => {
            b.to([27.2, 0.0, AXIS], 30.0).await;
            for _ in 0..3 {
                if b.taken("bunk") { break; }
                if !ring(b, BUNK_DOOR).await { continue; }
                b.say(format!("  after the ring: {}", b.orders_line()));
                b.until(|| b.taken("bunk") || b.squad().into_iter().all(|m| b.order_of(m).is_some_and(|a| a.arrived)), 25.0).await;
                b.wait(1.0).await;
                if !b.taken("bunk") { b.press("KeyF", 2).await; }
                b.until(|| b.taken("bunk"), 30.0).await;
                b.wait(2.0).await;
            }
            if !b.taken("bunk") { b.fail(format!("the bunk room is not theirs ({})", b.orders_line())).await; }
        }
        // the ring on the wall between the two rooms: its second ring has the charge. A charge already on the wall is left to the word, never asked for again
        "wall" => {
            let set = || b.seen(|_, s| !s.wall_charge.set.is_empty());
            let clear = || set() && b.seen(|g, s| game::wallcharge::wall_charge_too_close_of(g, s, s.wall_charge.set.first().copied()).is_empty());
            b.to([28.0, 0.0, 8.6], 30.0).await;
            for _ in 0..3 {
                if set() || b.taken("fuel") { break; }
                ring(b, WALL_MARK).await;
                b.until(set, 20.0).await;
            }
            if !set() && !b.taken("fuel") { b.fail(format!("no charge went on the wall ({})", b.orders_line())).await; }
            b.to([27.4, 0.0, 12.4], 20.0).await;   // (he stands off too: out through the door, into the shop)
            b.until(clear, 20.0).await;
            b.wait(0.6).await;
        }
        "breach" => {
            b.wait(1.0).await;
            b.press("KeyF", 2).await;
            if !b.until(|| b.taken("fuel"), 40.0).await { b.fail(format!("the fuel room is not theirs ({})", b.orders_line())).await; }
            b.wait(2.0).await;
        }
        // a tap on himself, and out through the shop to the court outside the wash yard's gateway
        "lead" => {
            let me = b.pos();
            b.command([me[0], 1.0, me[2]]).await;
            b.to(YARD_FROM, 40.0).await;
            b.until(|| !b.step_is("lead"), 30.0).await;
        }
        "lights" => { b.until(|| !b.step_is("lights"), 14.0).await; }
        // the ring on the floor behind the sandbags: its second ring has the frag
        "throw" => {
            let men = || b.named(&["Voss", "Kerr"]);
            let up: Vec<GuardId> = men().into_iter().filter(|&x| b.up(x)).collect();
            let dark = up.iter().filter(|&&x| b.ask(|g| !g.tab.light[g.gd[x].flashlight].enabled)).count();
            if !up.is_empty() && dark == up.len() { b.fail(String::from("the line says he can see their flashlights and they burn none")).await; }
            for _ in 0..2 {
                if !men().into_iter().any(|x| b.up(x)) { break; }
                if !ring(b, FRAG_AT).await { continue; }
                b.say(format!("  after the ring: {}", b.orders_line()));
                b.landed(ItemKind::Frag, 8.0).await;
                b.until(|| !men().into_iter().any(|x| b.up(x)), 5.0).await;
            }
        }
        "clear" => { b.hunt(11, 60.0).await; }
        _ => panic!("tut-kit has no deed for the step '{}'", id),
    }
}
