//! src/game/player.ts lines 2300 to 2630
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- the one seam left: it waits on the next skeleton (notes.md, ## Skeleton)

/// Doors.USE_REACH, a class's static the skeleton does not have (doors.ts:1183)
fn seam_use_reach() -> f64 { todo!("needs: Doors.USE_REACH as a const of crate::doors") }

// ---- buildInteractables' closures, as private functions

/// a marker with nothing but its own fields set
fn mark(kind: crate::game::InteractableKind, pos: V3, line1: String, line2: String, in_reach: bool, off: bool) -> crate::game::Interactable {
    crate::game::Interactable { kind, pos, line1, line2, in_reach, off, ..Default::default() }
}
/// a verb row: the marker's identity, 'Space  <verb>' and any hint lines for the thing's other inputs   (player.ts row)
fn row(base: &crate::game::Interactable, verb: crate::game::Verb, text: &str, hints: &str) -> crate::game::Interactable {
    let mut r = base.clone();
    r.verb = Some(verb);
    r.running = Some(false);
    r.progress = None;
    r.line2 = if hints.is_empty() { format!("Space  {}", text) } else { format!("Space  {}  ·  {}", text, hints) };
    r
}
/// the one row of a running sustained verb: worded as running, its bar, and how it stops   (player.ts running)
fn running(base: &crate::game::Interactable, verb: crate::game::Verb, text: &str, progress: Option<f64>) -> crate::game::Interactable {
    let mut r = base.clone();
    r.verb = Some(verb);
    r.running = Some(true);
    r.line2 = String::from(text);
    r.progress = progress.map(|x| m::max(0.0, m::min(1.0, x)));
    r.in_reach = true;
    r
}
/// a row Space does nothing with: no verb   (player.ts status)
fn status(base: &crate::game::Interactable, text: &str) -> crate::game::Interactable {
    let mut r = base.clone();
    r.verb = None;
    r.running = Some(false);
    r.line2 = String::from(text);
    r.progress = None;
    r.in_reach = true;
    r
}
/// the marker of a thing that has verb rows: its line2 lists them. `things` holds the marker's place in `list`: the two are one object in the TypeScript   (player.ts offer)
fn offer(list: &mut Vec<crate::game::Interactable>, things: &mut Vec<(usize, Vec<crate::game::Interactable>)>, mut it: crate::game::Interactable, verbs: Vec<crate::game::Interactable>) {
    if !verbs.is_empty() {
        it.line2 = join_line2(&verbs);
        things.push((list.len(), verbs));
    }
    list.push(it);
}
/// rows' line2s joined the way a marker words them
fn join_line2(rows: &[crate::game::Interactable]) -> String {
    let mut s = String::new();
    for (i, r) in rows.iter().enumerate() {
        if i > 0 { s.push_str("  ·  "); }
        s.push_str(&r.line2);
    }
    s
}
/// the door's marker this frame, or a plain one if it has none   (player.ts marker)
fn marker(g: &mut Game, list: &[crate::game::Interactable], d: DoorId) -> crate::game::Interactable {
    if let Some(x) = list.iter().find(|x| x.door == Some(d)) { return x.clone(); }
    let pos = crate::doors::door_pos(g, d);
    let mut it = mark(crate::game::InteractableKind::Door, pos, String::from("door"), String::new(), true, false);
    it.door = Some(d);
    it
}
/// a kind's word in another module's copy of the union
fn kind_word(s: &str) -> crate::game::InteractableKind {
    type K = crate::game::InteractableKind;
    match s {
        "cam" => K::Cam,
        "guard" => K::Guard,
        "breaker" => K::Breaker,
        "door" => K::Door,
        "light" => K::Light,
        "body" => K::Body,
        "objective" => K::Objective,
        "held" => K::Held,
        "wall" => K::Wall,
        "switch" => K::Switch,
        "charge" => K::Charge,
        _ => K::Cancel,
    }
}
/// the `row` closure handed to another module, whose own type names the `over` patch
macro_rules! row_cb {
    ($over:ty) => {
        &mut |_g: &mut Game, base: crate::game::Interactable, verb: crate::game::Verb, text: String, hints: Option<String>, over: Option<$over>| -> crate::game::Interactable {
            let mut r = row(&base, verb, &text, hints.as_deref().unwrap_or(""));
            if let Some(o) = over {
                if let Some(x) = o.kind { r.kind = kind_word(x.as_str()); }
                if let Some(x) = o.pos { r.pos = x; }
                if let Some(x) = o.line1 { r.line1 = x; }
                if let Some(x) = o.line2 { r.line2 = x; }
                if let Some(x) = o.in_reach { r.in_reach = x; }
                if let Some(x) = o.off { r.off = x; }
                if let Some(x) = o.progress { r.progress = Some(x); }
                if let Some(x) = o.target { r.target = Some(x); }
                if let Some(x) = o.door { r.door = Some(x); }
                if let Some(x) = o.guard { r.guard = Some(x); }
                if let Some(x) = o.verb { r.verb = Some(x); }
                if let Some(x) = o.running { r.running = Some(x); }
                if let Some(x) = o.close { r.close = Some(x); }
                if let Some(x) = o.past { r.past = Some(x); }
                if let Some(x) = o.framed { r.framed = Some(x); }
            }
            r
        }
    };
}
/// the `offer` closure handed to another module
macro_rules! offer_cb {
    ($list:ident, $things:ident) => {
        &mut |_g: &mut Game, it: crate::game::Interactable, verbs: Vec<crate::game::Interactable>| offer(&mut $list, &mut $things, it, verbs)
    };
}
/// the `running` closure handed to another module
fn running_cb(_g: &mut Game, base: crate::game::Interactable, verb: crate::game::Verb, text: String, progress: f64) -> crate::game::Interactable {
    running(&base, verb, &text, Some(progress))
}
/// a number the TypeScript prints in a template, known to be whole
fn whole(x: f64) -> String { format!("{}", x as i64) }
/// `${Math.min(99, Math.round(x * 100))}`
fn pct(x: f64) -> String { whole(m::min(99.0, m::round(x * 100.0))) }
/// a civilian's possessive for 'gun to … head' with the row of WORDS.on
fn civ_whose(g: &mut Game, cv: CivId) -> String {
    if g.cv[cv].she == "he" { String::from("his") } else { g.cv[cv].her.clone() }
}

/// the closure interactOrder returns: in reach first, then by what matters (INTERACT_PRIORITY, lower = higher up), then nearest   (player.ts:2558)
fn interact_cmp(g: &mut Game, p: V3, priority: &JsMap<String, f64>, a: &crate::game::Interactable, b: &crate::game::Interactable) -> f64 {
    if a.in_reach != b.in_reach { return if a.in_reach { -1.0 } else { 1.0 }; }
    let rank = |k: crate::game::InteractableKind| *priority.get(String::from(k.as_str())).unwrap();
    let by_rank = rank(a.kind) - rank(b.kind);
    let by_dist = thing_dist(g, a, p) - thing_dist(g, b, p);
    js::or(by_rank, by_dist)
}

/// The markers Space concerns itself with, and the rows: what Space can do right now, one verb a row, the highlighted one.   (player.ts buildInteractables)
pub fn build_interactables(g: &mut crate::game::Game, mx: f64, my: f64, w: f64, h: f64) {
    let pl = g.player;
    let mut list: Vec<crate::game::Interactable> = Vec::new();
    let p = g.ch[g.pl[pl].ch].pos;
    let mut things: Vec<(usize, Vec<crate::game::Interactable>)> = Vec::new();
    let mut rows: Vec<crate::game::Interactable> = Vec::new();
    const NO_BOOT: &str = "kick it — it opens toward you, so that won't work";

    let mut ti = 0;
    while ti < g.targets.len() {
        let t = g.targets[ti];
        ti += 1;
        if crate::sim_lightshome::fixture_broken(g, t) || !g.tab.sim_lightshome_fixture[t].interactive { continue; }
        let (name, pos) = (g.tab.sim_lightshome_fixture[t].name.clone(), g.tab.sim_lightshome_fixture[t].pos);
        if crate::carlamps::dead_lamp(g, t) {
            let mut it = mark(crate::game::InteractableKind::Light, pos, name, String::from("click  shoot"), true, true);
            it.target = Some(t);
            list.push(it);
            continue;
        }
        let factor = crate::sim_lightshome::fixture_factor(g, t);
        let mut it = mark(crate::game::InteractableKind::Light, pos, format!("{}{}", name, if factor > 0.0 { "" } else { " (off)" }), String::from("right-click  OCP (the pistol's)  ·  click  shoot"), true, factor <= 0.0);
        it.target = Some(t);
        list.push(it);
    }

    let down = g.pl[pl].down;
    if !down && g.pl[pl].holding.is_some() {
        // a man in his arm: HIS rows are the only rows
        let hold = g.pl[pl].holding.clone().unwrap();
        let gd = hold.g;
        let gun = hold.variant == crate::character::HoldVariant::Gun;
        let k = g.k.hold.secs;
        let gc = g.gd[gd].ch;
        let head = match g.ch[gc].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[gc].pos, [0.0, 1.6, 0.0]) };
        let cv = g.gd[gd].civ;
        let callsign = g.gd[gd].callsign.clone();
        let line1 = match cv {
            Some(cv) => {
                let tail = if gun { format!("your pistol at {} head", civ_whose(g, cv)) } else { String::from("in your arm") };
                format!("{} · the {} · {}", callsign, g.cv[cv].label, tail)
            }
            None => format!("{} · {}", callsign, if gun { "your pistol at his head" } else { "in your arm" }),
        };
        let mut base = mark(crate::game::InteractableKind::Held, [head[0], head[1] + 0.3, head[2]], line1, String::new(), true, false);
        base.guard = Some(gd);
        if hold.phase == crate::character::HoldPhase::Grab {
            let text = match cv { Some(cv) => format!("getting hold of {}…", g.cv[cv].her), None => String::from("getting hold of him…") };
            rows.push(status(&base, &text));
        } else if hold.phase == crate::character::HoldPhase::Held && hold.swap_t < k.swap {
            rows.push(status(&base, if gun { "the pistol coming up to his head…" } else { "holstering it to get both hands on him again…" }));
        } else if hold.phase == crate::character::HoldPhase::Choke || (hold.wind_t >= 0.0 && !gun) {
            rows.push(running(&base, crate::game::Verb::Choke, "choking him out… — Space or a step and he breathes again", Some(if hold.phase == crate::character::HoldPhase::Choke { hold.t / k.choke } else { 0.0 })));
        } else if hold_killing(&hold) {
            rows.push(status(&base, "the knife…"));
        } else if hold.phase == crate::character::HoldPhase::Whip || (hold.wind_t >= 0.0 && gun) {
            rows.push(running(&base, crate::game::Verb::Whip, "pistol-whip… — Space or a step checks it", Some(if hold.phase == crate::character::HoldPhase::Whip { hold.t / k.whip } else { 0.0 })));
        } else if let (Some(cv), true) = (cv, gun) {
            let her = g.cv[cv].her.clone();
            rows.push(row(&base, crate::game::Verb::Comply, &format!("“stay down, stay quiet” — lets {} go, cowering", her), ""));
            rows.push(row(&base, crate::game::Verb::Whip, &format!("pistol-whip {} — knocks {} out, but a CIVILIAN costs −10 on the card", her, her), &format!("E  holster — to question {}", her)));
        } else if let Some(cv) = cv {
            let tk = held_talk(g, &hold);
            let (she, her) = (g.cv[cv].she.clone(), g.cv[cv].her.clone());
            let stage = g.tab.game_talk[g.gd[gd].talk].stage;
            let told = {
                let lines = crate::dialogue::civ_lines();
                match lines.intel.get(String::from(g.cv[cv].def.kind.as_str())) { Some(l) => l.len(), None => 0 }
            };
            let dry = stage >= 1.0 && g.cv[cv].intel_n >= told;
            let ask = if tk.asking {
                format!("you're asking — {} answers in a beat", she)
            } else if tk.sam_up {
                String::from("once you're done talking")
            } else if !tk.ready {
                format!("{}'s answering — tap to cut it short", she)
            } else if dry {
                format!("{}'s told you everything — ask anyway", she)
            } else if stage == 0.0 {
                format!("ask what {} knows — quietly", she)
            } else {
                format!("press {} — what else", her)
            };
            rows.push(row(&base, crate::game::Verb::Question, &ask, ""));
            rows.push(row(&base, crate::game::Verb::Comply, &format!("“stay down, stay quiet” — lets {} go, cowering; {} tells nobody", her, she), ""));
            rows.push(row(&base, crate::game::Verb::Choke, &format!("choke {} out — quiet, but a CIVILIAN costs −10 on the card", her), &format!("1  draw — gun to {} head", if she == "he" { "his" } else { "her" })));
        } else if gun {
            let ready = crate::weapons::firearm_rounds_ready(g, g.pl[pl].pistol) > 0.0;
            rows.push(row(&base, crate::game::Verb::Whip, "pistol-whip him — quick, a thud", &format!("LMB  {}  ·  E  holster — to question him", if ready { "fire past him" } else { "fire past him (empty)" })));
        } else {
            let q = crate::dialogue::prompt_for(g, gd);
            let tk = held_talk(g, &hold);
            let verb = if q.spent { String::from("he's said his piece — ask anyway") } else { format!("“{}”", q.text) };
            let ask = if tk.asking {
                String::from("you're asking — he answers in a beat")
            } else if tk.sam_up {
                format!("{} — once you're done talking", verb)
            } else if !tk.ready {
                String::from("he's answering — tap to cut it short")
            } else {
                format!("{}{}", verb, if tk.settled { "" } else { " — cuts in" })
            };
            rows.push(row(&base, crate::game::Verb::Question, &ask, ""));
            let clicks = if !g.touring { "RMB  choke him out  ·  " } else { "" };
            rows.push(row(&base, crate::game::Verb::Choke, "choke him out — quiet", &format!("{}1  draw — gun to his head", clicks)));
        }
        let mut held_marker = base.clone();
        held_marker.line2 = join_line2(&rows);
        held_marker.progress = rows.first().and_then(|r| r.progress);
        list.push(held_marker);
        let mut di = 0;
        while di < g.doors.list.len() {
            let d = g.doors.list[di];
            di += 1;
            let dp = crate::doors::door_pos(g, d);
            if crate::doors::door_off(g, d) || m::min(v3::dist_xz(dp, p), v3::dist_xz(g.dr[d].frame_centre, p)) > NEARBY_M { continue; }
            let locked_out = crate::doors::doors_locked_out(g, d, p, None);
            let closed = crate::doors::door_is_closed(g, d);
            let mut it = mark(
                crate::game::InteractableKind::Door,
                dp,
                format!("door · {}", if locked_out { "locked" } else if closed { "shut" } else { "open" }),
                String::from(if locked_out { "walk him into it — his key turns the lock" } else if closed { "walk him into it — it pushes open" } else { "open" }),
                false,
                false,
            );
            it.door = Some(d);
            list.push(it);
        }
    } else if !down {
        let busy = g.pl[pl].takedown.is_some()
            || g.pl[pl].kick.is_some()
            || g.pl[pl].picking.is_some()
            || g.pl[pl].wanding.is_some()
            || g.pl[pl].easing.is_some()
            || g.pl[pl].coding.is_some()
            || g.pl[pl].breaker_move.is_some()
            || g.pl[pl].door_move.is_some()
            || g.pl[pl].charging.is_some()
            || g.mission.drive_t >= 0.0;
        let keys = g.pl[pl].keys.clone();
        let (sprinting, crouch, dragging_any) = (g.pl[pl].sprinting, g.pl[pl].crouch, g.pl[pl].dragging.is_some());
        let mut di = 0;
        while di < g.doors.list.len() {
            let d = g.doors.list[di];
            di += 1;
            if crate::doors::door_off(g, d) { continue; }
            let dp = crate::doors::door_pos(g, d);
            let near = m::min(v3::dist_xz(dp, p), v3::dist_xz(g.dr[d].frame_centre, p)) < seam_use_reach();
            let reach = near && !sprinting;
            let shut = crate::doors::door_is_closed(g, d);
            let pick = crate::doors::door_pick(g, d);
            let pct_s = format!("{}%", pct(pick));
            let mut it = mark(crate::game::InteractableKind::Door, dp, String::new(), String::from(if reach { "" } else { "step closer" }), reach, false);
            it.door = Some(d);
            let mut verbs: Vec<crate::game::Interactable> = Vec::new();
            let pad = crate::devices::pad_state(g, d, p);
            if let Some(pad) = pad {
                it.line1 = if g.pl[pl].coding == Some(d) {
                    String::from("door · keypad · keying it in")
                } else if pad.known {
                    format!("door · keypad — you know the code ({})", crate::devices::spell(&pad.pad.def.code))
                } else {
                    String::from("door · keypad · locked")
                };
                let held = crate::doors::doors_boot_holds(g, d, p);
                if !reach {
                    it.line2 = String::from(if sprinting && near && !held { "run into it  kick it in (LOUD)" } else { "step closer" });
                } else if !busy {
                    if pad.known {
                        verbs.push(row(&it, crate::game::Verb::Code, &format!("enter the code — {}", crate::devices::spell(&pad.pad.def.code)), ""));
                    } else {
                        let mut r = row(&it, crate::game::Verb::Nocode, "keypad — no code", "");
                        r.off = true;
                        verbs.push(r);
                    }
                    if !dragging_any && (!crouch || can_stand(g)) {
                        if held {
                            let mut r = row(&it, crate::game::Verb::Kick, NO_BOOT, "");
                            r.off = true;
                            verbs.push(r);
                        } else {
                            verbs.push(row(&it, crate::game::Verb::Kick, "kick it in — LOUD", if crouch { "" } else { "or sprint into it" }));
                        }
                    }
                }
            } else if crate::doors::doors_locked_out(g, d, p, Some(&keys)) {
                let steel = g.dr[d].def.steel == Some(true);
                it.line1 = if crate::doors::door_rattle_t(g, d) > 0.0 {
                    String::from("door · locked — won't budge")
                } else if steel {
                    String::from("steel fire door · bolted from the stairs")
                } else if g.pl[pl].picking == Some(d) {
                    String::from("door · locked · picking it")
                } else if pick > 0.0 {
                    format!("door · locked · {} picked", pct_s)
                } else {
                    String::from("door · locked")
                };
                it.progress = if pick > 0.0 { Some(pick) } else { None };
                let held = !steel && crate::doors::doors_boot_holds(g, d, p);
                if !reach {
                    it.line2 = String::from(if sprinting && near && !held {
                        if steel { "run into it  (it won't give)" } else { "run into it  kick it in (LOUD)" }
                    } else {
                        "step closer"
                    });
                } else if !busy {
                    if !steel {
                        let text = if pick > 0.0 { format!("pick the lock — quiet, {} done", pct_s) } else { String::from("pick the lock — quiet, 4 s") };
                        verbs.push(row(&it, crate::game::Verb::Pick, &text, ""));
                    }
                    if crouch || dragging_any || steel {
                        verbs.push(row(&it, crate::game::Verb::Rattle, if steel { "try the bar — no keyhole this side" } else { "try the handle" }, ""));
                    }
                    if !dragging_any && (!crouch || can_stand(g)) {
                        if held {
                            let mut r = row(&it, crate::game::Verb::Kick, NO_BOOT, "");
                            r.off = true;
                            verbs.push(r);
                        } else {
                            verbs.push(row(&it, crate::game::Verb::Kick, if steel { "kick it — LOUD, and steel won't give" } else { "kick it in — LOUD" }, if crouch { "" } else { "or sprint into it" }));
                        }
                    }
                }
            } else if crate::doors::doors_key_opens(g, d, p, Some(&keys)) {
                it.line1 = String::from("door · locked — you have the key");
                if reach && !busy {
                    verbs.push(row(&it, crate::game::Verb::Unlock, "unlock — his key", ""));
                    verbs.push(row(&it, crate::game::Verb::Ease, "ease it open — silent", ""));
                }
            } else {
                let tail = if crate::doors::door_lock_broken(g, d) {
                    " · lock kicked in"
                } else if g.dr[d].def.locked == Some(true) && !crate::doors::door_locked(g, d) {
                    " · picked"
                } else {
                    ""
                };
                it.line1 = format!("door · {}{}", if shut { "shut" } else { "open" }, tail);
                if reach && !busy {
                    let toward = crate::doors::doors_pull_face(g, d, p);
                    if shut {
                        verbs.push(row(&it, crate::game::Verb::Push, if toward { "pull it open" } else { "push it open" }, ""));
                    } else {
                        verbs.push(row(&it, crate::game::Verb::Pull, if toward { "push it shut" } else { "pull it shut" }, ""));
                    }
                    if m::abs(crate::doors::door_angle(g, d)) < EASE_OFFER {
                        verbs.push(row(&it, crate::game::Verb::Ease, "ease it open — silent", ""));
                    }
                }
            }
            // the big slot's tool (bigkit.ts): in the boot's own row. True = the shotgun, false = the hammer
            {
                let can = !busy && !dragging_any && !sprinting && (!crouch || can_stand(g));
                let far = if can && !reach && g.pl[pl].holding.is_none() { crate::bigkit::sam_big_far(g, d, p) } else { None };
                let tool = match far {
                    Some(t) => Some(t == crate::bigkit::SamBigFar::Shotgun),
                    None => if reach && can { crate::bigkit::sam_big(g, d, Some(p)).map(|t| t == crate::bigkit::SamBig::Shotgun) } else { None },
                };
                if far.is_some() { it.in_reach = true; }
                if let Some(shotgun) = tool {
                    let lock = crate::doors::doors_locked_out(g, d, p, Some(&keys));
                    let mut r = if shotgun {
                        row(&it, crate::game::Verb::Breach, if lock { "shoot the lock off — LOUD" } else { "shoot it open — LOUD" }, "")
                    } else {
                        row(&it, crate::game::Verb::Hammer, "hammer it open — LOUD", "")
                    };
                    r.past = Some(!lock);
                    match verbs.iter().position(|v| v.verb == Some(crate::game::Verb::Kick)) {
                        Some(k) => verbs[k] = r,
                        None => verbs.push(r),
                    }
                }
            }
            // the optical wand (wand.ts): the last row of whatever the door offers; where the rows are already MAX_NEAR it takes the place of 'try the handle'
            if reach && !busy && crate::wand::wand_offered(g, pl, d) {
                if verbs.len() as f64 >= MAX_NEAR {
                    if let Some(k) = verbs.iter().position(|v| v.verb == Some(crate::game::Verb::Rattle)) { verbs.remove(k); }
                }
                verbs.push(row(&it, crate::game::Verb::Wand, "look under it — the wand, quiet", ""));
            }
            let free = reach && !busy && !crate::wallcharge::wall_charge_setting(g);
            crate::doortools::door_tools_sam_rows(g, &mut it, d, &mut verbs, free, &mut |_g: &mut Game, base: crate::game::Interactable, verb: crate::game::Verb, text: String, hints: Option<String>| -> crate::game::Interactable {
                row(&base, verb, &text, hints.as_deref().unwrap_or(""))
            });
            offer(&mut list, &mut things, it, verbs);
        }
        let mut gi = 0;
        while gi < g.guards.len() {
            let gd = g.guards[gi];
            gi += 1;
            if g.gd[gd].state == crate::game::GuardState::Dead || g.pl[pl].takedown.is_some() { continue; }
            if g.gd[gd].team == crate::teams::Team::Operative || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
            if crate::close::close_live(g) {
                crate::close::close_rows(g, gd, &p, busy, row_cb!(crate::close::CloseRowsRowOver), offer_cb!(list, things));
                continue;
            }
            let gc = g.gd[gd].ch;
            let gp = g.ch[gc].pos;
            let (dx, dz) = (p[0] - gp[0], p[2] - gp[2]);
            let dd = m::hyp2(dx, dz);
            if dd < 2.4 {
                let behind = m::cos(v3::wrap_angle(m::atan2(dx, dz) - g.ch[gc].body_yaw)) < -0.25;
                let alert = g.gd[gd].state == crate::game::GuardState::Alert;
                let could = dd < 1.05
                    && behind
                    && !alert
                    && !sprinting
                    && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]], None)
                    && !crate::scene_collision::segment_blocked_dynamic(g, [p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]], None);
                let headroom = !could || !crouch || can_stand(g);
                let can = could && headroom;
                let line1 = if alert { "guard · alert" } else if behind { "guard" } else { "guard · facing you" };
                let line2 = if alert {
                    "not while he's onto you"
                } else if !behind {
                    "get behind him"
                } else if sprinting {
                    "stop sprinting"
                } else if !could {
                    "get closer"
                } else {
                    "no room to stand up behind him here"
                };
                let mut it = mark(crate::game::InteractableKind::Guard, v3::add(gp, [0.0, 1.35, 0.0]), String::from(line1), String::from(line2), can, false);
                it.guard = Some(gd);
                let mut verbs: Vec<crate::game::Interactable> = Vec::new();
                if can && !busy {
                    verbs.push(if g.pl[pl].holstered {
                        row(&it, crate::game::Verb::Grab, "grab him", "LMB  takedown — silent")
                    } else if grab_variant(g) == crate::character::HoldVariant::Gun {
                        row(&it, crate::game::Verb::Grab, "grab him — pistol to his head", "E  holster for the LMB takedown")
                    } else if crate::game::player_rifle_up(g, pl) {
                        row(&it, crate::game::Verb::Grab, "grab him — the rifle goes on your back", "E  sling it for the LMB takedown")
                    } else {
                        row(&it, crate::game::Verb::Grab, "grab him", "E  holster for the LMB takedown")
                    });
                }
                offer(&mut list, &mut things, it, verbs);
            }
        }
        for gd in crate::civilians::civilian_bodies(g) {
            if g.pl[pl].takedown.is_some() || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
            let cv = g.gd[gd].civ.unwrap();
            if crate::rescue::rescue_is_hostage(g, gd) { continue; }
            if crate::snatchout::snatch_out_owns(g, gd) { continue; }
            let gc = g.gd[gd].ch;
            let gp = g.ch[gc].pos;
            let (dx, dz) = (p[0] - gp[0], p[2] - gp[2]);
            let dd = m::hyp2(dx, dz);
            if dd >= 2.4 { continue; }
            let cg = if crate::close::close_live(g) { Some(crate::close::close_civ_gate(g, gd, &p)) } else { None };
            let cst = g.cv[cv].state;
            let behind = match &cg {
                Some(c) => c.behind,
                None => cst == crate::civilians::CivState::Cower || m::cos(v3::wrap_angle(m::atan2(dx, dz) - g.ch[gc].body_yaw)) < -0.25,
            };
            let catchable = cst != crate::civilians::CivState::Flee && cst != crate::civilians::CivState::Evacuate && cst != crate::civilians::CivState::Report;
            let close_enough = match &cg { Some(c) => c.near, None => dd < 1.05 };
            let could = close_enough
                && behind
                && catchable
                && !sprinting
                && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]], None)
                && !crate::scene_collision::segment_blocked_dynamic(g, [p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]], None);
            let can = could && (!crouch || can_stand(g));
            let (name, label, she, her) = (g.cv[cv].name.clone(), g.cv[cv].label.clone(), g.cv[cv].she.clone(), g.cv[cv].her.clone());
            let state_word = if cst == crate::civilians::CivState::Cower {
                String::from(" · cowering")
            } else if cst == crate::civilians::CivState::Work {
                String::new()
            } else {
                format!(" · {}", cst.as_str())
            };
            let line2 = if !catchable {
                format!("not while {}'s {}", she, if cst == crate::civilians::CivState::Report { "with a guard" } else { "running" })
            } else if !behind {
                format!("get behind {}", her)
            } else if sprinting {
                String::from("stop sprinting")
            } else if !could {
                String::from("get closer")
            } else {
                format!("no room to stand up behind {} here", her)
            };
            let mut it = mark(crate::game::InteractableKind::Guard, v3::add(gp, [0.0, 1.35, 0.0]), format!("{} · the {}{}", name, label, state_word), line2, can, false);
            it.guard = Some(gd);
            let mut verbs: Vec<crate::game::Interactable> = Vec::new();
            if let Some(c) = &cg {
                it.close = Some(c.kind);
                if can && !busy { crate::close::close_civ_offer(g, gd, &p); }
            }
            if can && !busy {
                verbs.push(if g.pl[pl].holstered {
                    row(&it, crate::game::Verb::Grab, &format!("grab {} — a hand over the mouth", her), "LMB  takedown — quiet, and −10")
                } else if grab_variant(g) == crate::character::HoldVariant::Gun {
                    row(&it, crate::game::Verb::Grab, &format!("grab {} — pistol to {} head", her, her), "E  holster for the LMB takedown")
                } else {
                    row(&it, crate::game::Verb::Grab, &format!("grab {}", her), "E  holster for the LMB takedown")
                });
            }
            offer(&mut list, &mut things, it, verbs);
        }
        // the bodies: a civilian on the floor is a body like any
        let mut bodies: Vec<GuardId> = g.guards.clone();
        bodies.extend(crate::civilians::civilians_shown(g));
        for gd in bodies {
            if g.gd[gd].state != crate::game::GuardState::Dead || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
            let gc = g.gd[gd].ch;
            let hips = match g.ch[gc].bones.get(crate::character::Bone::Hips) { Some(b) => *b, None => g.ch[gc].pos };
            let mid: V3 = [hips[0], hips[1] + 0.12, hips[2]];
            let dragging = g.pl[pl].dragging == Some(gd);
            let reach = dragging || body_dist(g, gd, Some(1.3)) < 1.3;
            let callsign = g.gd[gd].callsign.clone();
            let nm = match g.gd[gd].civ { Some(cv) => format!("{} (the {})", callsign, g.cv[cv].label), None => callsign };
            let wound = g.gd[gd].wound.is_some();
            let line1 = if wound {
                format!("{}, wounded — out of the fight", nm)
            } else if crate::game::is_breathing(g, gd) {
                format!("body · {}, out cold — breathing", nm)
            } else {
                format!("body · {}, dead", nm)
            };
            let mut it = mark(crate::game::InteractableKind::Body, mid, line1, String::from(if reach { "" } else { "step closer" }), reach, false);
            it.guard = Some(gd);
            let mut verbs: Vec<crate::game::Interactable> = Vec::new();
            if reach && !busy {
                let lift = g.puppet.is_none();
                if dragging {
                    let carrying = g.pl[pl].haul.as_ref().map_or(false, |h| h.carry.is_some());
                    verbs.push(row(&it, crate::game::Verb::Drop, if carrying { "put him down" } else { "let go of him" }, "sprinting drops him"));
                } else if !dragging_any && !wound {
                    verbs.push(row(&it, crate::game::Verb::Drag, if lift { "pick him up" } else { "drag him off" }, ""));
                }
            }
            offer(&mut list, &mut things, it, verbs);
        }
        // the mission's rack while the drive is still in it
        let drive_t = g.mission.drive_t;
        if g.mission.stage == crate::game::MissionStage::Drive && (!g.quiet_utility || g.mission_in_tour) {
            let reach = crate::mission::rack_dist(g, p) < crate::consts::DRIVE_REACH && !dragging_any && !sprinting && g.pl[pl].takedown.is_none();
            let qw = crate::quiet::quiet_words(g);
            let line1 = match &qw {
                Some(q) => format!("{} · {}", q.holder, q.thing),
                None => format!("rack {} · the drive", whole(g.level.mission.rack.index + 1.0)),
            };
            let mut it = mark(crate::game::InteractableKind::Objective, g.level.mission.rack.front, line1, String::from(if reach { "" } else if dragging_any { "hands full" } else { "step closer" }), reach, false);
            it.progress = if drive_t >= 0.0 { Some(m::min(1.0, drive_t / crate::consts::DRIVE_SECS)) } else { None };
            let mut verbs: Vec<crate::game::Interactable> = Vec::new();
            if drive_t >= 0.0 {
                it.line2 = format!("{}… {}%  ·  stay on it", if qw.is_some() { "taking" } else { "pulling" }, pct(drive_t / crate::consts::DRIVE_SECS));
            } else if reach && !busy {
                let text = match &qw { Some(q) => format!("take {} — stay on it", q.thing), None => String::from("pull the drive — stay on it") };
                verbs.push(row(&it, crate::game::Verb::Drive, &text, ""));
            }
            offer(&mut list, &mut things, it, verbs);
        }
        {
            // the mains breaker panel, by hand: in SIGHT of its face, reach = within arm's length, hands free
            let bp = g.level.breaker.pos;
            let face: V3 = [bp[0], bp[1], bp[2] - 0.18];
            let eye: V3 = [p[0], if crouch { 0.9 } else { 1.3 }, p[2]];
            let active = crate::sim_lightshome::mains_active(g);
            let gone_for_good = active && crate::sim_lightshome::mains_permanent(g);
            if !gone_for_good && v3::dist_xz(bp, p) < NEARBY_M && (!crate::scene_collision::solid_blocked(g, eye, face, None) || match crate::workthings::sam_panel_point(g, p) { Some(at) => !crate::scene_collision::solid_blocked(g, eye, at, None), None => false }) {
                let reach = v3::dist_xz(bp, p) < 1.15 && !dragging_any && !sprinting && g.pl[pl].takedown.is_none() && g.pl[pl].holding.is_none();
                let it = mark(crate::game::InteractableKind::Breaker, [bp[0], bp[1] + 0.35, bp[2]], String::from(if active { "breaker · thrown" } else { "breaker · power on" }), String::from(if reach { "" } else { "step closer" }), reach, active);
                let mut verbs: Vec<crate::game::Interactable> = Vec::new();
                if reach && !busy {
                    verbs.push(if active { row(&it, crate::game::Verb::BreakerOn, "turn the power back on", "") } else { row(&it, crate::game::Verb::BreakerOff, "throw it — kill the lights", "") });
                }
                offer(&mut list, &mut things, it, verbs);
            }
        }
        crate::wallcharge::wall_charge_rows(g, p, busy, row_cb!(crate::wallcharge::WallChargeRowsRowOver), offer_cb!(list, things));
        crate::snatchout::snatch_out_rows_2(g, p, busy, row_cb!(crate::snatchout::SnatchOutRows2RowOver), offer_cb!(list, things));
        crate::rescueend::rescue_end_rows_2(g, p, busy, row_cb!(crate::rescueend::RescueEndRows2RowOver), offer_cb!(list, things));
        let setting = crate::wallcharge::wall_charge_setting(g);
        crate::defuse::defuse_rows(g, p, busy || setting, row_cb!(crate::defuse::DefuseRowsRowOver), offer_cb!(list, things));
        let working = setting || crate::defuse::defuse_working(g);
        crate::stickycam::sticky_rows(g, pl, busy || working, row_cb!(crate::stickycam::StickyRowsRowOver), offer_cb!(list, things));
        crate::lightswitch::switch_rows(g, pl, busy || working, row_cb!(crate::lightswitch::SwitchRowsRowOver), offer_cb!(list, things));
        // ---- the rows: the running verb alone (or a status while the feet are committed), else the top MAX_NEAR verbs of the things in reach by what matters
        if let Some(d) = g.pl[pl].picking {
            let pick = crate::doors::door_pick(g, d);
            let base = marker(g, &list, d);
            rows = vec![running(&base, crate::game::Verb::Pick, &format!("picking… {}%  ·  Space or a step breaks off (the lock keeps it a moment)", pct(pick)), Some(pick))];
        } else if let Some(wd) = g.pl[pl].wanding.as_ref().map(|w| (w.door, w.live)) {
            let base = marker(g, &list, wd.0);
            rows = vec![running(&base, crate::game::Verb::Wand, if wd.1 { "looking under it — the cursor turns the lens  ·  Space or a step pulls it out" } else { "feeding the wand under…  ·  Space or a step pulls it out" }, None)];
        } else if let Some(d) = g.pl[pl].coding {
            let base = marker(g, &list, d);
            let name = match crate::devices::devices_pad_of(g, d) { Some(k) => crate::devices::pad_door_name(g, &k), None => String::from("door") };
            let progress = g.pl[pl].code_t / crate::devices::CODE_SECS;
            rows = vec![running(&base, crate::game::Verb::Code, &format!("keying the {} code… — Space or a step breaks off", name), Some(progress))];
        } else if let Some((door, tool)) = g.pl[pl].kick.as_ref().map(|k| (k.door, k.tool.as_ref().map(|t| t.kind))) {
            let base = marker(g, &list, door);
            let text = match tool {
                Some(crate::game::PlayerKickToolKind::Hammer) => "hammering it…",
                Some(_) => "on the lock…",
                None => "kicking it in…",
            };
            rows = vec![status(&base, text)];
        } else if let Some(d) = g.pl[pl].easing {
            let base = marker(g, &list, d);
            let progress = m::abs(crate::doors::door_angle(g, d)) / EASE_MAX;
            rows = vec![running(&base, crate::game::Verb::Ease, "easing it open… — Space or a step stops it where it is", Some(progress))];
        } else if drive_t >= 0.0 {
            if let Some(it) = list.iter().find(|x| x.kind == crate::game::InteractableKind::Objective) {
                rows = vec![running(it, crate::game::Verb::Drive, &format!("pulling… {}% — stay on it (Space or a step lets go)", pct(drive_t / crate::consts::DRIVE_SECS)), Some(drive_t / crate::consts::DRIVE_SECS))];
            }
        } else if let Some(door) = g.pl[pl].charging.as_ref().map(|c| c.door) {
            let base = marker(g, &list, door);
            let progress = crate::doortools::door_tools_sam_progress(g);
            rows = vec![running(&base, crate::game::Verb::Charge, "setting the charge… — Space or a step breaks it off", Some(progress))];
        } else if crate::wallcharge::wall_charge_setting(g) {
            rows = vec![crate::wallcharge::wall_charge_running_row(g, &mut running_cb)];
        } else if crate::defuse::defuse_working(g) {
            rows = vec![crate::defuse::defuse_running_row(g, &mut running_cb)];
        } else if g.pl[pl].takedown.is_none() {
            let priority = interact_priority();
            js::sort_by(&mut things, |a, b| interact_cmp(g, p, &priority, &list[a.0], &list[b.0]));
            // (rails.ts: a tutorial mission offers only the rows its steps teach, and a thing left with none is marked)
            let mut flat0: Vec<crate::game::Interactable> = Vec::new();
            for (at, verbs) in things.iter() {
                let mut kept = 0;
                for r in verbs.iter() {
                    let offered = match r.verb { None => true, Some(v) => crate::rails::rails_offers(g, v.as_str()) };
                    if offered { flat0.push(r.clone()); kept += 1; }
                }
                if kept == 0 { list[*at].framed = Some(true); }
            }
            // (wand.ts: the look under a door is the one row that gives way: listed after every other thing's rows)
            let is_wand = |r: &crate::game::Interactable| r.verb == Some(crate::game::Verb::Wand);
            let flat: Vec<crate::game::Interactable> = if flat0.iter().any(is_wand) {
                let mut f: Vec<crate::game::Interactable> = flat0.iter().filter(|r| !is_wand(r)).cloned().collect();
                f.extend(flat0.iter().filter(|r| is_wand(r)).cloned());
                f
            } else {
                flat0
            };
            // (close.ts: a man's strike-only line has no verb: it sits under the rows Space can do)
            let hint: Vec<crate::game::Interactable> = if crate::close::close_live(g) { flat.iter().filter(|r| r.verb.is_none()).take(1).cloned().collect() } else { Vec::new() };
            // (owncharge.ts, bigkit.ts, stickycam.ts: one row each past the MAX_NEAR)
            let is_door = |r: &crate::game::Interactable| r.kind == crate::game::InteractableKind::Door;
            let is_charge = |r: &crate::game::Interactable| is_door(r) && r.verb == Some(crate::game::Verb::Charge);
            let is_past = |r: &crate::game::Interactable| is_door(r) && r.past == Some(true);
            let is_cam = |r: &crate::game::Interactable| r.kind == crate::game::InteractableKind::Cam;
            let is_own = |r: &crate::game::Interactable| is_cam(r) || is_charge(r) || is_past(r);
            let mut own: Vec<crate::game::Interactable> = Vec::new();
            own.extend(flat.iter().filter(|r| is_charge(r)).take(1).cloned());
            own.extend(flat.iter().filter(|r| is_past(r)).take(1).cloned());
            own.extend(flat.iter().filter(|r| is_cam(r)).take(1).cloned());
            let rest: Vec<crate::game::Interactable> = if !own.is_empty() { flat.iter().filter(|r| !is_own(r)).cloned().collect() } else { flat };
            // (ownblow.ts: a charge his own hands set: under the rows the things in reach had, ahead of the rows past the cap)
            let blow = if busy || g.pl[pl].sprinting || g.pl[pl].dragging.is_some() { None } else { crate::ownblow::own_blow_row(g) };
            let mine: Vec<crate::game::Interactable> = blow.into_iter().collect();
            let top: Vec<crate::game::Interactable> = if !hint.is_empty() {
                rest.iter().filter(|r| r.verb.is_some()).take((MAX_NEAR - 1.0) as usize).cloned().collect()
            } else {
                rest.iter().take(MAX_NEAR as usize).cloned().collect()
            };
            let under = top.iter().any(|r| r.verb.is_some());
            let mut out = top;
            if under { out.extend(mine.iter().cloned()); }
            out.extend(own);
            if !under { out.extend(mine); }
            out.extend(hint);
            rows = out;
        }
    }
    if crate::game::player_list_open(g, pl) && rows.iter().any(|r| r.verb.is_some() && r.running != Some(true)) {
        let c = cancel_row(g);
        rows.push(c);
    }
    g.interactables = list;
    // The highlight rides the list by key: on the same row if it is still there, else back to the top. A running verb's row is not carried over.
    let mut key = String::new();
    for (i, r) in rows.iter().enumerate() {
        if i > 0 { key.push('|'); }
        key.push_str(&row_key(g, r));
    }
    if key != g.pl[pl].near_key {
        g.pl[pl].near_key = key;
        let keep = match g.hover.clone() {
            Some(hv) if hv.running != Some(true) => {
                let want = row_key(g, &hv);
                let mut at = None;
                for (i, r) in rows.iter().enumerate() {
                    if row_key(g, r) == want { at = Some(i); break; }
                }
                at
            }
            _ => None,
        };
        let open = g.pl[pl].sel.open;
        g.pl[pl].near_sel = match keep {
            Some(k) if !(rows[k].kind == crate::game::InteractableKind::Charge && k > 0 && !open) => k as i32,
            _ => 0,
        };
    }
    g.nearby = rows;
    pick_hover(g);
    crate::close::close_tick(g);
}

/// identity of a row: the thing and the verb   (player.ts rowKey)
pub fn row_key(g: &mut crate::game::Game, r: &crate::game::Interactable) -> String {
    let who = match (r.door, r.guard) {
        (Some(d), _) => g.dr[d].def.name.clone(),
        (None, Some(gd)) => g.gd[gd].callsign.clone(),
        (None, None) => String::new(),
    };
    match r.verb {
        Some(v) => format!("{}:{}:{}", r.kind.as_str(), who, v.as_str()),
        None => format!("{}:{}:~{}", r.kind.as_str(), who, r.line2),
    }
}

/// Game.hover ← the highlighted row, Game.useDoor ← its door if it is a door row in reach   (player.ts pickHover)
pub fn pick_hover(g: &mut crate::game::Game) {
    let pl = g.player;
    let n = g.nearby.len() as i32;
    let sel = g.pl[pl].near_sel;
    g.pl[pl].near_sel = if n > 0 { ((sel % n) + n) % n } else { 0 };
    let best = g.nearby.get(g.pl[pl].near_sel as usize).cloned();
    g.use_door = match &best {
        Some(b) if b.kind == crate::game::InteractableKind::Door && b.in_reach => b.door,
        _ => None,
    };
    g.hover = best;
}

/// the selector's bottom row while it is open: letting go of Space on it does nothing   (player.ts cancelRow)
pub fn cancel_row(g: &mut crate::game::Game) -> crate::game::Interactable {
    let p = g.ch[g.pl[g.player].ch].pos;
    let mut it = mark(crate::game::InteractableKind::Cancel, [p[0], 1.3, p[2]], String::from("cancel"), String::from("let go here — nothing happens"), true, false);
    it.verb = Some(crate::game::Verb::Cancel);
    it.running = Some(false);
    it
}

/// how far a thing is from him, as its reach is measured: a door by its leaf or its doorway's centre, whichever is nearer   (player.ts thingDist)
pub fn thing_dist(g: &mut crate::game::Game, it: &crate::game::Interactable, p: V3) -> f64 {
    let d = v3::dist_xz(it.pos, p);
    match it.door {
        Some(door) => m::min(d, v3::dist_xz(g.dr[door].frame_centre, p)),
        None => d,
    }
}

/// The one ordering for 'what is offered near Sam'. The TypeScript only makes and returns a comparer; the skeleton dropped the return, so nothing is left to do here. The comparer is `interact_cmp`, which buildInteractables calls.   (player.ts interactOrder)
pub fn interact_order(g: &mut crate::game::Game, p: V3) {}

/// how many of a kind are on the belt   (player.ts throwCount)
pub fn throw_count(g: &mut crate::game::Game, kind: crate::game::ThrowKind) -> f64 {
    let pl = g.player;
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs,
        _ => g.pl[pl].canisters,
    }
}

/// a thrown thing not yet out of the hand goes back on the belt   (player.ts canBack)
pub fn can_back(g: &mut crate::game::Game, pl: PlayerId, kind: crate::game::ThrowKind) {
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags += 1.0,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs += 1.0,
        _ => g.pl[pl].canisters += 1.0,
    }
}

/// a wind-up called off with the can still in his fist: back on the belt   (player.ts throwOff)
pub fn throw_off(g: &mut crate::game::Game, pl: PlayerId) {
    if let Some(kind) = g.pl[pl].pending_throw.as_ref().map(|t| t.kind) {
        can_back(g, pl, kind);
        g.pl[pl].pending_throw = None;
    }
}

/// F: let go of the body you are dragging, or take hold of the nearest one within reach   (player.ts toggleDrag)
pub fn toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) {
    let pl = g.player;
    if g.pl[pl].dragging.is_some() {
        release_body(g, None);
        crate::game::game_msg(g, "body dropped");
        return;
    }
    if g.pl[pl].down || g.pl[pl].sprinting { return; }
    let mut best: Option<GuardId> = match which {
        Some(w) if g.gd[w].state == crate::game::GuardState::Dead => Some(w),
        _ => None,
    };
    if best.is_none() {
        let mut bd = 1.3;
        let mut all: Vec<GuardId> = g.guards.clone();
        all.extend(crate::civilians::civilians_shown(g));
        for gd in all {
            if g.gd[gd].state != crate::game::GuardState::Dead || g.gd[gd].wound.is_some() { continue; }
            let d = body_dist(g, gd, Some(1.3));
            if d < bd {
                bd = d;
                best = Some(gd);
            }
        }
    }
    let best = match best {
        Some(b) if g.ch[g.gd[b].ch].ragdoll.is_some() => b,
        _ => {
            crate::game::game_msg(g, "no body within reach");
            return;
        }
    };
    release_wall(g);
    g.pl[pl].dragging = Some(best);
    g.pl[pl].crouch = true;
    g.pl[pl].drag_noise_t = 0.3;
    g.pl[pl].drag_stuck_t = 0.0;
    let bc = g.gd[best].ch;
    if let Some(r) = g.ch[bc].ragdoll { crate::ragdoll::ragdoll_wake(g, r); }
    crate::hands::haul_begin(g, best);
    let (carry, head) = match g.pl[pl].haul.as_ref() {
        Some(h) => (h.carry.is_some(), h.end == crate::hands::HaulEnd::Head),
        None => (false, false),
    };
    crate::game::game_msg(g, if carry {
        "picking him up — Space puts him down, sprint drops him"
    } else if head {
        "got him under the arms — Space lets go, sprint drops him"
    } else {
        "dragging the body — Space lets go, sprint drops it"
    });
}

/// Let go of the body in tow: the ankle pins come off and its feet fall where they are   (player.ts releaseBody)
pub fn release_body(g: &mut crate::game::Game, laid: Option<bool>) {
    let laid = laid.unwrap_or(false);
    let pl = g.player;
    let b = match g.pl[pl].dragging {
        Some(gd) => g.gd[gd].ch,
        None => return,
    };
    if let Some(r) = g.ch[b].ragdoll { crate::ragdoll::ragdoll_clear_pins(g, r); }
    g.pl[pl].dragging = None;
    g.pl[pl].haul = None;
    let c = g.pl[pl].ch;
    g.ch[c].anim.haul = None;
    let at = match g.ch[b].bones.get(crate::character::Bone::FootL) { Some(f) => *f, None => g.ch[b].pos };
    crate::audio_iface::game_audio_like_play(g, "bodyFall", Some(at), Some(if laid { 0.12 } else { 0.3 }), None);
}

/// Break off whatever the hands and feet are in the middle of that belongs to the SPOT, and stand ready   (player.ts cancelActions)
pub fn cancel_actions(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    release_wall(g);
    if g.pl[pl].dragging.is_some() { release_body(g, None); }
    if g.pl[pl].holding.is_some() { free_held(g, crate::player::FreeHeldWhy::Cancel); }
    g.pl[pl].takedown = None;
    if g.pl[pl].kick.is_some() || g.pl[pl].picking.is_some() || g.pl[pl].wanding.is_some() || g.pl[pl].easing.is_some() || g.pl[pl].coding.is_some() || g.pl[pl].sel.down_t >= 0.0 {
        g.pl[pl].f_held = true;
    }
    g.pl[pl].kick = None;
    g.pl[pl].picking = None;
    crate::wand::wand_drop(g, pl);
    g.ch[c].anim.lockpick = false;
    if g.pl[pl].coding.is_some() { crate::devices::stop_coding(g); }
    crate::bigkit::sam_big_drop(g);
    g.pl[pl].easing = None;
    crate::hands::hands_cancel(g);
    g.pl[pl].sel.down_t = -1.0;
    g.pl[pl].sel.open = false;
    g.pl[pl].sel.moved = false;
    g.pl[pl].sel.ring = None;
    g.ch[c].anim.door_push = None;
    throw_off(g, pl);
    g.pl[pl].throw_held = false;
    g.pl[pl].throw_preview = None;
    g.pl[pl].sprinting = false;
    g.pl[pl].speed_sm = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
}

/// pushed out of whatever is there: did that converge (false: a gap the body cannot fit)   (player.ts teleportPlayer's settle)
fn settle(g: &mut Game, q: &mut V3) -> bool {
    crate::scene_collision::clamp_to_world(g, q, BODY_R);
    let r = body_r(g, *q);
    crate::scene_collision::collide_circle(g, q, r, 0.05, 2.2, Some(6.0));
    let mut t = v3::copy(*q);
    let r = body_r(g, t);
    !crate::scene_collision::collide_circle(g, &mut t, r, 0.05, 2.2, Some(1.0)) || v3::dist_xz(t, *q) < 0.005
}

/// Sandbox / tour / harness: stand the player at p, free of anything he was in the middle of   (player.ts teleportPlayer)
pub fn teleport_player(g: &mut crate::game::Game, p: V3) {
    let pl = g.player;
    if g.pl[pl].down { return; }
    if !p[0].is_finite() || !p[2].is_finite() { return; }
    cancel_actions(g);
    let mut q: V3 = [p[0], 0.0, p[2]];
    if !settle(g, &mut q) || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
        if let Some(c) = crate::scene_collision::nav_grid_nearest_free_point(g, q[0], q[2]) {
            q = c;
            settle(g, &mut q);
        }
    }
    crate::scene_collision::clamp_to_world(g, &mut q, BODY_R);
    let c = g.pl[pl].ch;
    g.ch[c].pos = q;
    g.pl[pl].safe = Some([q[0], 0.0, q[2]]);
    crate::character::character_update(g, c, 0.0);
    crate::game::game_note(g, "teleported");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door_marker() -> crate::game::Interactable {
        let mut it = mark(crate::game::InteractableKind::Door, [1.0, 2.0, 3.0], String::from("door · shut"), String::from("x"), false, false);
        it.progress = Some(0.5);
        it
    }

    #[test]
    fn a_row_words_its_verb_and_hint_and_clears_the_progress() {
        let r = row(&door_marker(), crate::game::Verb::Kick, "kick it in — LOUD", "or sprint into it");
        assert_eq!(r.line2, "Space  kick it in — LOUD  ·  or sprint into it");
        assert_eq!(r.running, Some(false));
        assert_eq!(r.progress, None);
        assert_eq!(row(&door_marker(), crate::game::Verb::Push, "push it open", "").line2, "Space  push it open");
    }

    #[test]
    fn a_running_row_clamps_its_progress_and_is_in_reach() {
        let r = running(&door_marker(), crate::game::Verb::Ease, "easing", Some(1.7));
        assert_eq!(r.progress, Some(1.0));
        assert!(r.in_reach);
        assert_eq!(r.running, Some(true));
        assert_eq!(running(&door_marker(), crate::game::Verb::Ease, "easing", None).progress, None);
    }

    #[test]
    fn a_thing_with_verbs_is_listed_and_named_by_its_place() {
        let (mut list, mut things) = (Vec::new(), Vec::new());
        let it = door_marker();
        let verbs = vec![row(&it, crate::game::Verb::Push, "push it open", ""), row(&it, crate::game::Verb::Ease, "ease it open — silent", "")];
        offer(&mut list, &mut things, it.clone(), verbs);
        offer(&mut list, &mut things, it, Vec::new());
        assert_eq!(list.len(), 2);
        assert_eq!(things.len(), 1);
        assert_eq!(things[0].0, 0);
        assert_eq!(list[0].line2, "Space  push it open  ·  Space  ease it open — silent");
        assert_eq!(list[1].line2, "x");
    }

    #[test]
    fn a_percentage_is_whole_and_stops_short_of_a_hundred() {
        assert_eq!(pct(0.999), "99");
        assert_eq!(pct(0.256), "26");
    }
}
